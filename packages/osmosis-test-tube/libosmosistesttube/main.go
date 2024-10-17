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
)

//export InitTestEnv
func InitTestEnv() uint64 {
	// Temp fix for concurrency issue
	mu.Lock()
	defer mu.Unlock()

	// temp: suppress noise from stdout
	// os.Stdout = nil

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

	env.BeginNewBlock(false, 5)

	env.FundValidators()

	env.App.EndBlocker(env.Ctx)

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

	envRegister.Store(envId, env)

	return C.CString(base64Priv)
}

//export IncreaseTime
func IncreaseTime(envId uint64, seconds uint64) int64 {
	env := loadEnv(envId)
	_, err := finalizeBlock(&env, [][]byte{})
	if err != nil {
		panic(err)
	}
	_, err = commitWithCustomIncBlockTime(&env, seconds)
	if err != nil {
		panic(err)
	}
	envRegister.Store(envId, env)

	return env.Ctx.BlockTime().UnixNano()
}

//export FinalizeBlock
func FinalizeBlock(envId uint64, txs string) *C.char {
	env := loadEnv(envId)

	txsBytes, err := base64.StdEncoding.DecodeString(txs)
	if err != nil {
		return encodeErrToResultBytes(result.ExecuteError, err)
	}

	res, err := finalizeBlock(&env, [][]byte{txsBytes})

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

func finalizeBlock(env *testenv.TestEnv, txs [][]byte) (*abci.ResponseFinalizeBlock, error) {
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
	res, err := commitWithCustomIncBlockTime(&env, 5)
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

func commitWithCustomIncBlockTime(env *testenv.TestEnv, seconds uint64) (*abci.ResponseCommit, error) {
	res, err := env.App.Commit()
	if err != nil {
		return nil, err
	}

	newBlockTime := env.Ctx.BlockTime().Add(time.Duration(seconds) * time.Second)

	header := env.Ctx.BlockHeader()
	header.Time = newBlockTime
	header.Height++

	env.Ctx = env.App.BaseApp.NewUncachedContext(false, header).WithHeaderInfo(coreheader.Info{
		Height: header.Height,
		Time:   header.Time,
	})

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

	fmt.Println("ctx height", env.Ctx.BlockHeight())

	res, err := route(env.Ctx, &req)

	fmt.Println("query height", res.Height)

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
func main() {
	// envId := InitTestEnv()
	// fmt.Println("envId", envId)
	// blockTime := GetBlockTime(envId)
	// fmt.Println("[Block Time] Current block time:", time.Unix(0, blockTime))

	// coinsJson := `[{"denom": "uosmo", "amount": "100000000"}]`

	// // NOTE: there is a but in ctx construction (ie. NewContext variants function)
	// // Somehow using legacy constructor causes a nil keeper error when querying
	// // but using non legacy constructor resulted in 0 validators

	// // following what's done here prop make sense: https://github.com/osmosis-labs/osmosis/blob/0711797bf19cce7a1952bee68620376f10c577d9/app/apptesting/test_suite.go#L463
	// // see how this is used in context

	// account1 := InitAccount(envId, coinsJson)
	// account2 := InitAccount(envId, coinsJson)

	// base64PrivKey1 := C.GoString(account1)
	// base64PrivKey2 := C.GoString(account2)
	// privKeyBytes1, err := base64.StdEncoding.DecodeString(base64PrivKey1)
	// if err != nil {
	// 	panic(err)
	// }
	// privKeyBytes2, err := base64.StdEncoding.DecodeString(base64PrivKey2)
	// if err != nil {
	// 	panic(err)
	// }

	// privKey1 := secp256k1.PrivKey{Key: privKeyBytes1}
	// privKey2 := secp256k1.PrivKey{Key: privKeyBytes2}

	// acc1 := sdk.AccAddress(privKey1.PubKey().Address())
	// acc2 := sdk.AccAddress(privKey2.PubKey().Address())

	// fmt.Println("Account 1 Address:", acc1.String())
	// fmt.Println("Account 2 Address:", acc2.String())

	// queryPath := "/cosmos.bank.v1beta1.Query/Balance"
	// queryMsg := banktypes.QueryBalanceRequest{Address: acc1.String(), Denom: "uosmo"}

	// queryMsgBytes, err := proto.Marshal(&queryMsg)
	// if err != nil {
	// 	panic(err)
	// }

	// base64QueryMsg := base64.StdEncoding.EncodeToString(queryMsgBytes)
	// balanceResult := Query(envId, queryPath, base64QueryMsg)
	// base64BalanceResult := C.GoString(balanceResult)
	// balanceResultBytes, err := base64.StdEncoding.DecodeString(base64BalanceResult)
	// if err != nil {
	// 	panic(err)
	// }

	// if balanceResultBytes[0] != 0 {
	// 	panic("Query failed")
	// }

	// balanceResponseBytes := balanceResultBytes[1:]

	// balanceResponse := banktypes.QueryBalanceResponse{}
	// err = proto.Unmarshal(balanceResponseBytes, &balanceResponse)
	// if err != nil {
	// 	panic(err)
	// }

	// fmt.Println("Account 1 OSMO balance:", balanceResponse)

	// // Create a bank send transaction
	// fromAddr := acc1
	// toAddr := acc2

	// amount := sdk.NewCoins(sdk.NewCoin("uosmo", sdkmath.NewInt(1000000)))
	// msg := banktypes.NewMsgSend(fromAddr, toAddr, amount)

	// txBuilder := app.GetEncodingConfig().TxConfig.NewTxBuilder()
	// err = txBuilder.SetMsgs(msg)
	// if err != nil {
	// 	panic(err)
	// }

	// env := loadEnv(envId)

	// accSeq := AccountSequence(envId, fromAddr.String())
	// accNum := AccountNumber(envId, fromAddr.String())

	// txBuilder.SetGasLimit(200000)
	// txBuilder.SetFeeAmount(sdk.NewCoins(sdk.NewCoin("uosmo", sdkmath.NewInt(5000)))) // 0.005 OSMO as fee

	// sigV2 := signing.SignatureV2{
	// 	PubKey: privKey1.PubKey(),
	// 	Data: &signing.SingleSignatureData{
	// 		SignMode: signing.SignMode_SIGN_MODE_DIRECT,
	// 	},
	// 	Sequence: accSeq,
	// }
	// txBuilder.SetSignatures(sigV2)

	// signerData := authsigning.SignerData{ChainID: "osmosis-1",
	// 	AccountNumber: accNum,
	// 	Sequence:      accSeq,
	// }

	// signBytes, err := authsigning.GetSignBytesAdapter(
	// 	env.Ctx, app.GetEncodingConfig().TxConfig.SignModeHandler(), signing.SignMode_SIGN_MODE_DIRECT, signerData, txBuilder.GetTx())
	// if err != nil {
	// 	panic(err)
	// }
	// sig, err := privKey1.Sign(signBytes)
	// if err != nil {
	// 	panic(err)
	// }
	// sigV2.Data.(*signing.SingleSignatureData).Signature = sig

	// txBuilder.SetSignatures(sigV2)

	// txBytes, err := app.GetEncodingConfig().TxConfig.TxEncoder()(txBuilder.GetTx())
	// if err != nil {
	// 	panic(err)
	// }

	// base64Tx := base64.StdEncoding.EncodeToString(txBytes)
	// FinalizeBlock(envId, base64Tx)
	// Commit(envId)
	// blockTime = GetBlockTime(envId)
	// fmt.Println("[Block Time] Current block time:", time.Unix(0, blockTime))

	// balance := env.App.BankKeeper.GetBalance(env.Ctx, fromAddr, "uosmo")
	// fmt.Println("[Keeper] Account 1 OSMO balance:", balance)

	// balanceResult = Query(envId, queryPath, base64QueryMsg)
	// base64BalanceResult = C.GoString(balanceResult)
	// balanceResultBytes, err = base64.StdEncoding.DecodeString(base64BalanceResult)
	// if err != nil {
	// 	panic(err)
	// }

	// if balanceResultBytes[0] != 0 {
	// 	panic("Query failed")
	// }

	// balanceResponseBytes = balanceResultBytes[1:]
	// balanceResponse = banktypes.QueryBalanceResponse{}
	// err = proto.Unmarshal(balanceResponseBytes, &balanceResponse)
	// if err != nil {
	// 	panic(err)
	// }

	// IncreaseTime(envId, 200)

	// blockTime = GetBlockTime(envId)
	// fmt.Println("[Block Time] Current block time:", time.Unix(0, blockTime))

	// fmt.Println("[Query] Account 1 OSMO balance:", balanceResponse)
}
