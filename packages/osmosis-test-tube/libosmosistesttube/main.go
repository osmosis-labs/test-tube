package main

import "C"

import (
	// std
	"encoding/base64"
	"encoding/json"
	"fmt"
	"os"
	"sync"
	"time"

	// helpers
	proto "github.com/cosmos/gogoproto/proto"
	"github.com/pkg/errors"

	// tendermint
	abci "github.com/cometbft/cometbft/abci/types"

	// cosmos sdk
	coreheader "cosmossdk.io/core/header"
	codectypes "github.com/cosmos/cosmos-sdk/codec/types"
	"github.com/cosmos/cosmos-sdk/crypto/keys/secp256k1"
	sdk "github.com/cosmos/cosmos-sdk/types"

	banktestutil "github.com/cosmos/cosmos-sdk/x/bank/testutil"
	banktypes "github.com/cosmos/cosmos-sdk/x/bank/types"

	// wasmd
	wasmtypes "github.com/CosmWasm/wasmd/x/wasm/types"

	// cosmwasm-testing

	"github.com/osmosis-labs/test-tube/osmosis-test-tube/result"
	"github.com/osmosis-labs/test-tube/osmosis-test-tube/testenv"
	// osmosis
	// lockuptypes "github.com/osmosis-labs/osmosis/v16/x/lockup/types"
)

var (
	envCounter  uint64 = 0
	envRegister        = sync.Map{}
	mu          sync.Mutex
	blockTime   uint64 = 5
)

//export InitTestEnv
func InitTestEnv() uint64 {
	// Temp fix for concurrency issue
	mu.Lock()
	defer mu.Unlock()

	// temp: suppress noise from stdout
	os.Stdout = nil

	envCounter += 1
	id := envCounter

	nodeHome, err := os.MkdirTemp("", ".osmosis-test-tube-temp-")
	if err != nil {
		panic(err)
	}

	env := new(testenv.TestEnv)
	env.App = testenv.NewOsmosisApp(nodeHome)
	env.NodeHome = nodeHome
	env.ParamTypesRegistry = *testenv.NewParamTypeRegistry()

	ctx, valPriv := testenv.InitChain(env.App)

	env.Ctx = ctx
	env.ValPrivs = []*secp256k1.PrivKey{&valPriv}

	env.SetupParamTypes()

	// Allow testing unoptimized contract
	wasmtypes.MaxWasmSize = 1024 * 1024 * 1024 * 1024 * 1024

	env.FundValidators()

	err = produceEmptyBlock(env)
	if err != nil {
		panic(err)
	}

	envRegister.Store(id, *env)

	return id
}

//export CleanUp
func CleanUp(envId uint64) {
	env := loadEnv(envId)
	err := os.RemoveAll(env.NodeHome)
	if err != nil {
		panic(err)
	}
	envRegister.Delete(envId)
}

//export InitAccount
func InitAccount(envId uint64, coinsJson string) *C.char {
	env := loadEnv(envId)
	var coins sdk.Coins

	if err := json.Unmarshal([]byte(coinsJson), &coins); err != nil {
		panic(err)
	}

	priv := secp256k1.GenPrivKey()
	accAddr := sdk.AccAddress(priv.PubKey().Address())

	for _, coin := range coins {
		// create denom if not exist
		_, hasDenomMetaData := env.App.BankKeeper.GetDenomMetaData(env.Ctx, coin.Denom)
		if !hasDenomMetaData {
			denomMetaData := banktypes.Metadata{
				DenomUnits: []*banktypes.DenomUnit{{
					Denom:    coin.Denom,
					Exponent: 0,
				}},
				Base: coin.Denom,
			}

			env.App.BankKeeper.SetDenomMetaData(env.Ctx, denomMetaData)
		}

	}

	err := banktestutil.FundAccount(env.Ctx, env.App.BankKeeper, accAddr, coins)
	if err != nil {
		panic(errors.Wrapf(err, "Failed to fund account"))
	}

	base64Priv := base64.StdEncoding.EncodeToString(priv.Bytes())

	// make sure block is produced so that updated state is on chain
	err = produceEmptyBlock(&env)
	if err != nil {
		panic(err)
	}
	envRegister.Store(envId, env)

	return C.CString(base64Priv)
}

func produceEmptyBlock(env *testenv.TestEnv) error {
	_, err := finalizeBlock(env, [][]byte{}, blockTime)
	if err != nil {
		return err
	}
	_, err = commitWithCustomIncBlockTime(env)
	if err != nil {
		return err
	}
	return nil
}

//export IncreaseTime
func IncreaseTime(envId uint64, seconds uint64) int64 {
	env := loadEnv(envId)
	_, err := finalizeBlock(&env, [][]byte{}, seconds)
	if err != nil {
		panic(err)
	}
	_, err = commitWithCustomIncBlockTime(&env)
	if err != nil {
		panic(err)
	}
	envRegister.Store(envId, env)

	return env.Ctx.BlockTime().UnixNano()
}

//export FinalizeBlock
func FinalizeBlock(envId uint64, tx string) *C.char {
	env := loadEnv(envId)

	txBytes, err := base64.StdEncoding.DecodeString(tx)
	if err != nil {
		return encodeErrToResultBytes(result.ExecuteError, err)
	}

	res, err := finalizeBlock(&env, [][]byte{txBytes}, blockTime)

	if err != nil {
		return encodeErrToResultBytes(result.ExecuteError, err)
	}

	envRegister.Store(envId, env)

	bz, err := proto.Marshal(res)
	if err != nil {
		return encodeErrToResultBytes(result.ExecuteError, err)
	}

	return encodeBytesResultBytes(bz)
}

func finalizeBlock(env *testenv.TestEnv, txs [][]byte, seconds uint64) (*abci.ResponseFinalizeBlock, error) {
	newBlockTime := env.Ctx.BlockTime().Add(time.Duration(seconds) * time.Second)

	header := env.Ctx.BlockHeader()
	header.Time = newBlockTime
	header.Height++

	env.Ctx = env.App.BaseApp.NewUncachedContext(false, header).WithHeaderInfo(coreheader.Info{
		Height: header.Height,
		Time:   header.Time,
	})

	res, err := env.App.FinalizeBlock(&abci.RequestFinalizeBlock{
		Txs:    txs,
		Height: env.Ctx.BlockHeight(),
		Time:   env.Ctx.BlockTime(),
	})

	return res, err
}

//export Commit
func Commit(envId uint64) *C.char {
	env := loadEnv(envId)
	res, err := commitWithCustomIncBlockTime(&env)
	if err != nil {
		return encodeErrToResultBytes(result.ExecuteError, err)
	}

	envRegister.Store(envId, env)

	bz, err := proto.Marshal(res)
	if err != nil {
		return encodeErrToResultBytes(result.ExecuteError, err)
	}

	return encodeBytesResultBytes(bz)
}

func commitWithCustomIncBlockTime(env *testenv.TestEnv) (*abci.ResponseCommit, error) {
	res, err := env.App.Commit()
	if err != nil {
		return nil, err
	}

	return res, nil
}

//export WasmSudo
func WasmSudo(envId uint64, bech32Address, msgJson string) *C.char {
	env := loadEnv(envId)
	// Temp fix for concurrency issue
	mu.Lock()
	defer mu.Unlock()

	accAddr, err := sdk.AccAddressFromBech32(bech32Address)
	if err != nil {
		panic(err)
	}

	msgBytes := []byte(msgJson)

	res, err := env.App.WasmKeeper.Sudo(env.Ctx, accAddr, msgBytes)
	if err != nil {
		return encodeErrToResultBytes(result.ExecuteError, err)
	}

	envRegister.Store(envId, env)

	return encodeBytesResultBytes(res)
}

//export Query
func Query(envId uint64, path, base64QueryMsgBytes string) *C.char {
	env := loadEnv(envId)
	queryMsgBytes, err := base64.StdEncoding.DecodeString(base64QueryMsgBytes)
	if err != nil {
		panic(err)
	}

	req := abci.RequestQuery{}
	req.Data = queryMsgBytes

	route := env.App.GRPCQueryRouter().Route(path)
	if route == nil {
		err := errors.New("No route found for `" + path + "`")
		return encodeErrToResultBytes(result.QueryError, err)
	}

	res, err := route(env.Ctx, &req)

	if err != nil {
		return encodeErrToResultBytes(result.QueryError, err)
	}

	return encodeBytesResultBytes(res.Value)
}

//export GetBlockTime
func GetBlockTime(envId uint64) int64 {
	env := loadEnv(envId)
	return env.Ctx.BlockTime().UnixNano()
}

//export GetBlockHeight
func GetBlockHeight(envId uint64) int64 {
	env := loadEnv(envId)
	return env.Ctx.BlockHeight()
}

//export AccountSequence
func AccountSequence(envId uint64, bech32Address string) uint64 {
	env := loadEnv(envId)

	addr, err := sdk.AccAddressFromBech32(bech32Address)

	if err != nil {
		panic(err)
	}

	seq, err := env.App.AppKeepers.AccountKeeper.GetSequence(env.Ctx, addr)

	if err != nil {
		panic(err)
	}

	return seq
}

//export AccountNumber
func AccountNumber(envId uint64, bech32Address string) uint64 {
	env := loadEnv(envId)

	addr, err := sdk.AccAddressFromBech32(bech32Address)

	if err != nil {
		panic(err)
	}

	acc := env.App.AppKeepers.AccountKeeper.GetAccount(env.Ctx, addr)
	return acc.GetAccountNumber()
}

//export Simulate
func Simulate(envId uint64, base64TxBytes string) *C.char { // => base64GasInfo
	env := loadEnv(envId)
	// Temp fix for concurrency issue
	mu.Lock()
	defer mu.Unlock()

	txBytes, err := base64.StdEncoding.DecodeString(base64TxBytes)
	if err != nil {
		panic(err)
	}

	gasInfo, _, err := env.App.Simulate(txBytes)
	if err != nil {
		return encodeErrToResultBytes(result.ExecuteError, err)
	}

	bz, err := proto.Marshal(&gasInfo)
	if err != nil {
		panic(err)
	}

	return encodeBytesResultBytes(bz)
}

//export SetParamSet
func SetParamSet(envId uint64, subspaceName, base64ParamSetBytes string) *C.char {
	env := loadEnv(envId)

	// Temp fix for concurrency issue
	mu.Lock()
	defer mu.Unlock()

	paramSetBytes, err := base64.StdEncoding.DecodeString(base64ParamSetBytes)
	if err != nil {
		panic(err)
	}

	subspace, ok := env.App.ParamsKeeper.GetSubspace(subspaceName)
	if !ok {
		err := errors.New("No subspace found for `" + subspaceName + "`")
		return encodeErrToResultBytes(result.ExecuteError, err)
	}

	pReg := env.ParamTypesRegistry

	any := codectypes.Any{}
	err = proto.Unmarshal(paramSetBytes, &any)

	if err != nil {
		return encodeErrToResultBytes(result.ExecuteError, err)
	}

	pset, err := pReg.UnpackAny(&any)

	if err != nil {
		return encodeErrToResultBytes(result.ExecuteError, err)
	}

	subspace.SetParamSet(env.Ctx, pset)

	// make sure block is produced so that updated state is on chain
	err = produceEmptyBlock(&env)
	if err != nil {
		panic(err)
	}

	envRegister.Store(envId, env)

	// return empty bytes if no error
	return encodeBytesResultBytes([]byte{})
}

//export GetParamSet
func GetParamSet(envId uint64, subspaceName, typeUrl string) *C.char {
	env := loadEnv(envId)

	subspace, ok := env.App.ParamsKeeper.GetSubspace(subspaceName)
	if !ok {
		err := errors.New("No subspace found for `" + subspaceName + "`")
		return encodeErrToResultBytes(result.ExecuteError, err)
	}

	pReg := env.ParamTypesRegistry
	pset, ok := pReg.GetEmptyParamsSet(typeUrl)

	if !ok {
		err := errors.New("No param set found for `" + typeUrl + "`")
		return encodeErrToResultBytes(result.ExecuteError, err)
	}
	subspace.GetParamSet(env.Ctx, pset)

	bz, err := proto.Marshal(pset)

	if err != nil {
		panic(err)
	}

	return encodeBytesResultBytes(bz)
}

//export GetValidatorAddress
func GetValidatorAddress(envId uint64, n int32) *C.char {
	env := loadEnv(envId)
	return C.CString(env.GetValidatorAddresses()[n])
}

//export GetValidatorPrivateKey
func GetValidatorPrivateKey(envId uint64, n int32) *C.char {
	env := loadEnv(envId)

	priv := env.ValPrivs[n].Key
	base64Priv := base64.StdEncoding.EncodeToString(priv)
	return C.CString(base64Priv)
}

// ========= utils =========

func loadEnv(envId uint64) testenv.TestEnv {
	item, ok := envRegister.Load(envId)
	env := testenv.TestEnv(item.(testenv.TestEnv))
	if !ok {
		panic(fmt.Sprintf("env not found: %d", envId))
	}
	return env
}

func encodeErrToResultBytes(code byte, err error) *C.char {
	return C.CString(result.EncodeResultFromError(code, err))
}

func encodeBytesResultBytes(bytes []byte) *C.char {
	return C.CString(result.EncodeResultFromOk(bytes))
}

// must define main for ffi build
func main() {}
