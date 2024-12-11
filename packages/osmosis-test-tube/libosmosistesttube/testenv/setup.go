package testenv

import (
	"encoding/json"
	"fmt"
	"strings"
	"time"

	// helpers

	// tendermint
	"cosmossdk.io/errors"
	log "cosmossdk.io/log"
	abci "github.com/cometbft/cometbft/abci/types"
	dbm "github.com/cosmos/cosmos-db"

	// cosmos-sdk
	cmtproto "github.com/cometbft/cometbft/proto/tendermint/types"
	"github.com/cosmos/cosmos-sdk/baseapp"
	"github.com/cosmos/cosmos-sdk/crypto/keys/secp256k1"
	"github.com/cosmos/cosmos-sdk/server"
	simtestutil "github.com/cosmos/cosmos-sdk/testutil/sims"
	sdk "github.com/cosmos/cosmos-sdk/types"
	banktestutil "github.com/cosmos/cosmos-sdk/x/bank/testutil"
	slashingtypes "github.com/cosmos/cosmos-sdk/x/slashing/types"
	stakingkeeper "github.com/cosmos/cosmos-sdk/x/staking/keeper"
	stakingtypes "github.com/cosmos/cosmos-sdk/x/staking/types"

	// wasmd

	wasmtypes "github.com/CosmWasm/wasmd/x/wasm/types"

	// osmosis
	"github.com/osmosis-labs/osmosis/v27/app"
	concentrateliquiditytypes "github.com/osmosis-labs/osmosis/v27/x/concentrated-liquidity/types"
	gammtypes "github.com/osmosis-labs/osmosis/v27/x/gamm/types"
	ibcratelimittypes "github.com/osmosis-labs/osmosis/v27/x/ibc-rate-limit/types"
	incentivetypes "github.com/osmosis-labs/osmosis/v27/x/incentives/types"
	lockuptypes "github.com/osmosis-labs/osmosis/v27/x/lockup/types"
	minttypes "github.com/osmosis-labs/osmosis/v27/x/mint/types"
	poolincentivetypes "github.com/osmosis-labs/osmosis/v27/x/pool-incentives/types"
	poolmanagertypes "github.com/osmosis-labs/osmosis/v27/x/poolmanager/types"
	protorevtypes "github.com/osmosis-labs/osmosis/v27/x/protorev/types"
	smartaccounttypes "github.com/osmosis-labs/osmosis/v27/x/smart-account/types"
	superfluidtypes "github.com/osmosis-labs/osmosis/v27/x/superfluid/types"
	tokenfactorytypes "github.com/osmosis-labs/osmosis/v27/x/tokenfactory/types"
	twaptypes "github.com/osmosis-labs/osmosis/v27/x/twap/types"

	sdkmath "cosmossdk.io/math"

	tmtypes "github.com/cometbft/cometbft/types"

	codectypes "github.com/cosmos/cosmos-sdk/codec/types"

	cryptocodec "github.com/cosmos/cosmos-sdk/crypto/codec"

	authtypes "github.com/cosmos/cosmos-sdk/x/auth/types"

	banktypes "github.com/cosmos/cosmos-sdk/x/bank/types"
)

func GenesisStateWithValSet(appInstance *app.OsmosisApp) (app.GenesisState, secp256k1.PrivKey) {
	privVal := NewPV()
	pubKey, _ := privVal.GetPubKey()
	validator := tmtypes.NewValidator(pubKey, 1)
	valSet := tmtypes.NewValidatorSet([]*tmtypes.Validator{validator})

	// generate genesis account
	senderPrivKey := secp256k1.GenPrivKey()
	senderPrivKey.PubKey().Address()
	acc := authtypes.NewBaseAccountWithAddress(senderPrivKey.PubKey().Address().Bytes())

	//////////////////////
	balances := []banktypes.Balance{}
	genesisState := app.NewDefaultGenesisState()
	genAccs := []authtypes.GenesisAccount{acc}
	authGenesis := authtypes.NewGenesisState(authtypes.DefaultParams(), genAccs)
	genesisState[authtypes.ModuleName] = appInstance.AppCodec().MustMarshalJSON(authGenesis)

	validators := make([]stakingtypes.Validator, 0, len(valSet.Validators))
	delegations := make([]stakingtypes.Delegation, 0, len(valSet.Validators))

	bondAmt := sdk.DefaultPowerReduction
	initValPowers := []abci.ValidatorUpdate{}

	for _, val := range valSet.Validators {
		pk, _ := cryptocodec.FromCmtPubKeyInterface(val.PubKey)
		pkAny, _ := codectypes.NewAnyWithValue(pk)
		validator := stakingtypes.Validator{
			OperatorAddress:   sdk.ValAddress(val.Address).String(),
			ConsensusPubkey:   pkAny,
			Jailed:            false,
			Status:            stakingtypes.Bonded,
			Tokens:            bondAmt,
			DelegatorShares:   sdkmath.LegacyOneDec(),
			Description:       stakingtypes.Description{},
			UnbondingHeight:   int64(0),
			UnbondingTime:     time.Unix(0, 0).UTC(),
			Commission:        stakingtypes.NewCommission(sdkmath.LegacyZeroDec(), sdkmath.LegacyZeroDec(), sdkmath.LegacyZeroDec()),
			MinSelfDelegation: sdkmath.ZeroInt(),
		}

		valAddr, err := sdk.ValAddressFromHex(val.Address.String())
		requireNoErr(err)
		validators = append(validators, validator)
		delegations = append(delegations, stakingtypes.NewDelegation(genAccs[0].GetAddress().String(), valAddr.String(), sdkmath.LegacyOneDec()))

		// add initial validator powers so consumer InitGenesis runs correctly
		pub, _ := val.ToProto()
		initValPowers = append(initValPowers, abci.ValidatorUpdate{
			Power:  val.VotingPower,
			PubKey: pub.PubKey,
		})
	}
	// set validators and delegations
	stakingGenesis := stakingtypes.NewGenesisState(stakingtypes.DefaultParams(), validators, delegations)
	genesisState[stakingtypes.ModuleName] = appInstance.AppCodec().MustMarshalJSON(stakingGenesis)

	totalSupply := sdk.NewCoins()
	for _, b := range balances {
		// add genesis acc tokens to total supply
		totalSupply = totalSupply.Add(b.Coins...)
	}

	for range delegations {
		// add delegated tokens to total supply
		totalSupply = totalSupply.Add(sdk.NewCoin(sdk.DefaultBondDenom, bondAmt))
	}

	// add bonded amount to bonded pool module account
	balances = append(balances, banktypes.Balance{
		Address: authtypes.NewModuleAddress(stakingtypes.BondedPoolName).String(),
		Coins:   sdk.Coins{sdk.NewCoin(sdk.DefaultBondDenom, bondAmt)},
	})

	// update total supply
	bankGenesis := banktypes.NewGenesisState(
		banktypes.DefaultGenesisState().Params,
		balances,
		totalSupply,
		[]banktypes.Metadata{},
		[]banktypes.SendEnabled{},
	)
	genesisState[banktypes.ModuleName] = appInstance.AppCodec().MustMarshalJSON(bankGenesis)

	_, err := tmtypes.PB2TM.ValidatorUpdates(initValPowers)
	if err != nil {
		panic("failed to get vals")
	}

	return genesisState, secp256k1.PrivKey{Key: privVal.PrivKey.Bytes()}
}

type TestEnv struct {
	App                *app.OsmosisApp
	Ctx                sdk.Context
	ParamTypesRegistry ParamTypeRegistry
	ValPrivs           []*secp256k1.PrivKey
	NodeHome           string
}

// TestAppOptions is a stub implementing AppOptions
type TestAppOptions struct{}

// Get implements AppOptions
func (ao TestAppOptions) Get(o string) interface{} {
	if o == server.FlagTrace {
		return true
	}

	if o == "wasm.simulation_gas_limit" {
		return ^uint64(0) // max uint64
	}
	return nil
}

func NewOsmosisApp(nodeHome string) *app.OsmosisApp {
	db := dbm.NewMemDB()

	return app.NewOsmosisApp(
		log.NewNopLogger(),
		db,
		nil,
		true,
		map[int64]bool{},
		nodeHome,
		5,
		TestAppOptions{},
		app.EmptyWasmOpts,
		baseapp.SetChainID("osmosis-1"),
	)

}

func InitChain(appInstance *app.OsmosisApp) (sdk.Context, secp256k1.PrivKey) {
	sdk.DefaultBondDenom = "uosmo"
	genesisState, valPriv := GenesisStateWithValSet(appInstance)

	encCfg := app.MakeEncodingConfig()

	// Set up Wasm genesis state
	wasmGen := wasmtypes.GenesisState{
		Params: wasmtypes.Params{
			// Allow store code without gov
			CodeUploadAccess:             wasmtypes.AllowEverybody,
			InstantiateDefaultPermission: wasmtypes.AccessTypeEverybody,
		},
	}
	genesisState[wasmtypes.ModuleName] = encCfg.Marshaler.MustMarshalJSON(&wasmGen)

	// set staking genesis state
	stakingGenesisState := stakingtypes.GenesisState{}
	appInstance.AppCodec().UnmarshalJSON(genesisState[stakingtypes.ModuleName], &stakingGenesisState)

	stateBytes, err := json.MarshalIndent(genesisState, "", " ")

	requireNoErr(err)

	concensusParams := simtestutil.DefaultConsensusParams
	concensusParams.Block = &cmtproto.BlockParams{
		MaxBytes: 22020096,
		MaxGas:   -1,
	}

	// replace sdk.DefaultDenom with "uosmo", a bit of a hack, needs improvement
	stateBytes = []byte(strings.Replace(string(stateBytes), "\"stake\"", "\"uosmo\"", -1))

	appInstance.InitChain(
		&abci.RequestInitChain{
			Validators:      []abci.ValidatorUpdate{},
			ConsensusParams: concensusParams,
			AppStateBytes:   stateBytes,
			ChainId:         "osmosis-1",
		},
	)

	ctx := appInstance.NewContextLegacy(false, cmtproto.Header{Height: 0, ChainID: "osmosis-1", Time: time.Now().UTC()})

	// Manually set validator signing info, otherwise we panic
	vals, err := appInstance.StakingKeeper.GetAllValidators(ctx)
	if err != nil {
		panic(err)
	}
	for _, val := range vals {
		consAddr, _ := val.GetConsAddr()
		signingInfo := slashingtypes.NewValidatorSigningInfo(
			consAddr,
			ctx.BlockHeight(),
			time.Unix(0, 0),
			false,
			0,
		)
		err := appInstance.SlashingKeeper.SetValidatorSigningInfo(ctx, consAddr, signingInfo)
		if err != nil {
			panic(err)
		}
	}
	return ctx, valPriv
}

func (env *TestEnv) BeginNewBlock(executeNextEpoch bool, timeIncreaseSeconds uint64) {
	validators, err := env.App.StakingKeeper.GetAllValidators(env.Ctx)
	requireNoErr(err)
	valAddr, err := validators[0].GetConsAddr()
	requireNoErr(err)

	env.beginNewBlockWithProposer(executeNextEpoch, valAddr, timeIncreaseSeconds)
}

func (env *TestEnv) FundValidators() {
	for _, valPriv := range env.ValPrivs {
		valAddr := sdk.AccAddress(valPriv.PubKey().Address())
		err := banktestutil.FundAccount(env.Ctx, env.App.BankKeeper, valAddr, sdk.NewCoins(sdk.NewInt64Coin("uosmo", 9223372036854775807)))
		if err != nil {
			panic(errors.Wrapf(err, "Failed to fund account"))
		}
	}
}

func (env *TestEnv) InitValidator() []byte {
	valPriv, valAddrFancy := env.setupValidator(stakingtypes.Bonded)
	validator, _ := env.App.StakingKeeper.GetValidator(env.Ctx, valAddrFancy)
	valAddr, _ := validator.GetConsAddr()

	env.ValPrivs = append(env.ValPrivs, valPriv)
	err := banktestutil.FundAccount(env.Ctx, env.App.BankKeeper, sdk.AccAddress(valAddr), sdk.NewCoins(sdk.NewInt64Coin("uosmo", 9223372036854775807)))
	if err != nil {
		panic(errors.Wrapf(err, "Failed to fund account"))
	}

	return valAddr
}

func (env *TestEnv) GetValidatorAddresses() []string {
	validators, err := env.App.StakingKeeper.GetAllValidators(env.Ctx)
	requireNoErr(err)
	var addresses []string
	for _, validator := range validators {
		addresses = append(addresses, validator.OperatorAddress)
	}

	return addresses
}

// beginNewBlockWithProposer begins a new block with a proposer.
func (env *TestEnv) beginNewBlockWithProposer(executeNextEpoch bool, proposer sdk.ValAddress, timeIncreaseSeconds uint64) {
	validator, err := env.App.StakingKeeper.GetValidator(env.Ctx, proposer)
	requireNoErr(err)

	valConsAddr, err := validator.GetConsAddr()
	requireNoErr(err)

	valAddr := valConsAddr

	epochIdentifier := env.App.SuperfluidKeeper.GetEpochIdentifier(env.Ctx)
	epoch := env.App.EpochsKeeper.GetEpochInfo(env.Ctx, epochIdentifier)
	newBlockTime := env.Ctx.BlockTime().Add(time.Duration(timeIncreaseSeconds) * time.Second)
	if executeNextEpoch {
		newBlockTime = env.Ctx.BlockTime().Add(epoch.Duration).Add(time.Second)
	}

	header := cmtproto.Header{Height: env.Ctx.BlockHeight() + 1, Time: newBlockTime}
	env.Ctx = env.Ctx.WithBlockTime(newBlockTime).WithBlockHeight(env.Ctx.BlockHeight() + 1)
	voteInfos := []abci.VoteInfo{{
		Validator:   abci.Validator{Address: valAddr, Power: 1000},
		BlockIdFlag: cmtproto.BlockIDFlagCommit,
	}}
	env.Ctx = env.Ctx.WithVoteInfos(voteInfos)

	_, err = env.App.BeginBlocker(env.Ctx)
	requireNoErr(err)

	env.Ctx = env.App.NewContextLegacy(false, header)
}

func (env *TestEnv) setupValidator(bondStatus stakingtypes.BondStatus) (*secp256k1.PrivKey, sdk.ValAddress) {
	valPriv := secp256k1.GenPrivKey()
	valPub := valPriv.PubKey()
	valAddr := sdk.ValAddress(valPub.Address())
	params, err := env.App.StakingKeeper.GetParams(env.Ctx)
	requireNoErr(err)
	bondDenom := params.BondDenom
	selfBond := sdk.NewCoins(sdk.Coin{Amount: sdkmath.NewInt(100), Denom: bondDenom})

	err = banktestutil.FundAccount(env.Ctx, env.App.BankKeeper, sdk.AccAddress(valPub.Address()), selfBond)
	requireNoErr(err)

	stakingMsgServer := stakingkeeper.NewMsgServerImpl(env.App.StakingKeeper)
	stakingCoin := sdk.NewCoin(bondDenom, selfBond[0].Amount)
	ZeroCommission := stakingtypes.NewCommissionRates(sdkmath.LegacyZeroDec(), sdkmath.LegacyZeroDec(), sdkmath.LegacyZeroDec())
	msg, err := stakingtypes.NewMsgCreateValidator(valAddr.String(), valPub, stakingCoin, stakingtypes.Description{}, ZeroCommission, sdkmath.OneInt())
	requireNoErr(err)

	res, err := stakingMsgServer.CreateValidator(env.Ctx, msg)
	requireNoErr(err)
	requireNoNil("staking handler", res)

	env.App.BankKeeper.SendCoinsFromModuleToModule(env.Ctx, stakingtypes.NotBondedPoolName, stakingtypes.BondedPoolName, sdk.NewCoins(stakingCoin))

	val, err := env.App.StakingKeeper.GetValidator(env.Ctx, valAddr)
	requireNoErr(err)

	val = val.UpdateStatus(bondStatus)
	env.App.StakingKeeper.SetValidator(env.Ctx, val)

	consAddr, err := val.GetConsAddr()
	requireNoErr(err)
	env.setupDefaultValidatorSigningInfo(consAddr)

	return valPriv, valAddr
}

func (env *TestEnv) SetupDefaultValidator() {
	validators, err := env.App.StakingKeeper.GetAllValidators(env.Ctx)
	requireNoErr(err)
	valAddrFancy, err := validators[0].GetConsAddr()
	requireNoErr(err)
	env.setupDefaultValidatorSigningInfo(valAddrFancy)
}

func (env *TestEnv) setupDefaultValidatorSigningInfo(consAddr sdk.ConsAddress) {
	signingInfo := slashingtypes.NewValidatorSigningInfo(
		consAddr,
		env.Ctx.BlockHeight(),
		time.Unix(0, 0),
		false,
		0,
	)
	env.App.SlashingKeeper.SetValidatorSigningInfo(env.Ctx, consAddr, signingInfo)
}

func (env *TestEnv) SetupParamTypes() {
	pReg := env.ParamTypesRegistry
	pReg.RegisterParamSet(&lockuptypes.Params{})
	pReg.RegisterParamSet(&incentivetypes.Params{})
	pReg.RegisterParamSet(&minttypes.Params{})
	pReg.RegisterParamSet(&twaptypes.Params{})
	pReg.RegisterParamSet(&gammtypes.Params{})
	pReg.RegisterParamSet(&ibcratelimittypes.Params{})
	pReg.RegisterParamSet(&tokenfactorytypes.Params{})
	pReg.RegisterParamSet(&superfluidtypes.Params{})
	pReg.RegisterParamSet(&smartaccounttypes.Params{})
	pReg.RegisterParamSet(&poolincentivetypes.Params{})
	pReg.RegisterParamSet(&protorevtypes.Params{})
	pReg.RegisterParamSet(&poolmanagertypes.Params{})
	pReg.RegisterParamSet(&concentrateliquiditytypes.Params{})
}

func requireNoErr(err error) {
	if err != nil {
		panic(err)
	}
}

func requireNoNil(name string, nilable any) {
	if nilable == nil {
		panic(fmt.Sprintf("%s must not be nil", name))
	}
}

func requierTrue(name string, b bool) {
	if !b {
		panic(fmt.Sprintf("%s must be true", name))
	}
}
