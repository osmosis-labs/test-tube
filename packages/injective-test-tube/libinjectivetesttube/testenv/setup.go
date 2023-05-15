package testenv

import (
	"encoding/json"
	"fmt"
	"strings"
	"time"

	// helpers

	// tendermint

	dbm "github.com/cometbft/cometbft-db"
	abci "github.com/cometbft/cometbft/abci/types"
	"github.com/cometbft/cometbft/libs/log"
	tmproto "github.com/cometbft/cometbft/proto/tendermint/types"
	tmtypes "github.com/cometbft/cometbft/types"
	"github.com/cosmos/cosmos-sdk/baseapp"
	"github.com/cosmos/cosmos-sdk/x/bank/testutil"

	"github.com/cosmos/cosmos-sdk/crypto/keys/ed25519"
	"github.com/cosmos/cosmos-sdk/crypto/keys/secp256k1"
	"github.com/cosmos/cosmos-sdk/server"

	"github.com/cosmos/cosmos-sdk/testutil/mock"
	simtestutil "github.com/cosmos/cosmos-sdk/testutil/sims"
	sdk "github.com/cosmos/cosmos-sdk/types"
	authtypes "github.com/cosmos/cosmos-sdk/x/auth/types"
	banktypes "github.com/cosmos/cosmos-sdk/x/bank/types"
	govtypes "github.com/cosmos/cosmos-sdk/x/gov/types"
	govv1types "github.com/cosmos/cosmos-sdk/x/gov/types/v1"

	slashingtypes "github.com/cosmos/cosmos-sdk/x/slashing/types"
	stakingkeeper "github.com/cosmos/cosmos-sdk/x/staking/keeper"
	stakingtypes "github.com/cosmos/cosmos-sdk/x/staking/types"

	// wasmd
	"github.com/CosmWasm/wasmd/x/wasm"
	wasmtypes "github.com/CosmWasm/wasmd/x/wasm/types"

	// injective
	"github.com/InjectiveLabs/injective-core/injective-chain/app"
	exchangetypes "github.com/InjectiveLabs/injective-core/injective-chain/modules/exchange/types"
	tokenfactorytypes "github.com/InjectiveLabs/injective-core/injective-chain/modules/tokenfactory/types"
)

type TestEnv struct {
	App                *app.InjectiveApp
	Ctx                sdk.Context
	ParamTypesRegistry ParamTypeRegistry
	Validator          []byte
}

// DebugAppOptions is a stub implementing AppOptions
type DebugAppOptions struct{}

// Get implements AppOptions
func (ao DebugAppOptions) Get(o string) interface{} {
	if o == server.FlagTrace {
		return true
	}
	return nil
}

func SetupInjectiveApp() (*app.InjectiveApp, []byte) {
	db := dbm.NewMemDB()
	encCfg := app.MakeEncodingConfig()

	appInstance := app.NewInjectiveApp(
		log.NewNopLogger(),
		db,
		nil,
		true,
		map[int64]bool{},
		app.DefaultNodeHome,
		5,
		encCfg,
		app.TestAppOptions{},
		baseapp.SetChainID("injective-777"),
	)

	// validator keys
	validatorKey := secp256k1.GenPrivKey()
	pval := mock.PV{PrivKey: validatorKey}
	conval := mock.PV{PrivKey: ed25519.GenPrivKey()}
	pubKey, err := pval.GetPubKey()
	requireNoErr(err)

	// create validator set with single validator
	validator := tmtypes.NewValidator(pubKey, 1)
	conPubKey, err := conval.GetPubKey()
	validator.PubKey = conPubKey

	valSet := tmtypes.NewValidatorSet([]*tmtypes.Validator{validator})

	// generate genesis account
	senderPrivKey := ed25519.GenPrivKey()
	acc := authtypes.NewBaseAccount(senderPrivKey.PubKey().Address().Bytes(), senderPrivKey.PubKey(), 0, 0)
	balance := banktypes.Balance{
		Address: acc.GetAddress().String(),
		Coins:   sdk.NewCoins(sdk.NewCoin(sdk.DefaultBondDenom, sdk.NewInt(100000000000000))),
	}

	genesisState := app.NewDefaultGenesisState()
	genesisState, err = simtestutil.GenesisStateWithValSet(appInstance.AppCodec(), genesisState, valSet, []authtypes.GenesisAccount{acc}, balance)
	requireNoErr(err)

	// Set up Wasm genesis state
	wasmGen := wasm.GenesisState{
		Params: wasmtypes.Params{
			// Allow store code without gov
			CodeUploadAccess:             wasmtypes.AllowEverybody,
			InstantiateDefaultPermission: wasmtypes.AccessTypeEverybody,
		},
	}
	genesisState[wasm.ModuleName] = encCfg.Marshaler.MustMarshalJSON(&wasmGen)

	// Set up governance genesis state
	govParams := govv1types.DefaultParams()
	votingPeriod := time.Second * 10 // 10 second
	govParams.VotingPeriod = &votingPeriod
	govGen := govv1types.GenesisState{
		StartingProposalId: govv1types.DefaultStartingProposalID,
		Deposits:           govv1types.Deposits{},
		Votes:              govv1types.Votes{},
		Proposals:          govv1types.Proposals{},
		Params:             &govParams,
	}

	genesisState[govtypes.ModuleName] = encCfg.Marshaler.MustMarshalJSON(&govGen)

	// Set up exchange genesis state
	exchangeParams := exchangetypes.DefaultParams()
	exchangeParams.IsInstantDerivativeMarketLaunchEnabled = true
	exchangeGen := exchangetypes.GenesisState{
		Params: exchangeParams,
	}
	genesisState[exchangetypes.ModuleName] = encCfg.Marshaler.MustMarshalJSON(&exchangeGen)

	stateBytes, err := json.MarshalIndent(genesisState, "", " ")

	requireNoErr(err)

	consensusParams := app.DefaultConsensusParams

	// replace sdk.DefaultDenom with "inj", a bit of a hack, needs improvement
	stateBytes = []byte(strings.Replace(string(stateBytes), "\"stake\"", "\"inj\"", -1))

	appInstance.InitChain(
		abci.RequestInitChain{
			ChainId:         "injective-777",
			Validators:      []abci.ValidatorUpdate{},
			ConsensusParams: consensusParams,
			AppStateBytes:   stateBytes,
		},
	)

	return appInstance, validatorKey.Bytes()
}

func (env *TestEnv) BeginNewBlock(executeNextEpoch bool, timeIncreaseSeconds uint64) {
	var valAddr []byte

	validators := env.App.StakingKeeper.GetAllValidators(env.Ctx)

	if len(validators) >= 1 {
		valAddrFancy, err := validators[0].GetConsAddr()
		requireNoErr(err)
		valAddr = valAddrFancy.Bytes()

	} else {
		valAddrFancy := env.setupValidator(stakingtypes.Bonded)
		validator, _ := env.App.StakingKeeper.GetValidator(env.Ctx, valAddrFancy)
		valAddr2, _ := validator.GetConsAddr()
		valAddr = valAddr2.Bytes()
	}

	env.beginNewBlockWithProposer(executeNextEpoch, valAddr, timeIncreaseSeconds)
}

func (env *TestEnv) GetValidatorAddresses() []string {
	validators := env.App.StakingKeeper.GetAllValidators(env.Ctx)
	var addresses []string
	for _, validator := range validators {
		addresses = append(addresses, validator.OperatorAddress)
	}

	return addresses
}

func (env *TestEnv) GetValidatorPrivateKey() []byte {
	return env.Validator
}

// beginNewBlockWithProposer begins a new block with a proposer.
func (env *TestEnv) beginNewBlockWithProposer(executeNextEpoch bool, proposer sdk.ConsAddress, timeIncreaseSeconds uint64) {
	// validator, found := env.App.StakingKeeper.GetValidator(env.Ctx, proposer)
	validator, found := env.App.StakingKeeper.GetValidatorByConsAddr(env.Ctx, proposer)

	if !found {
		panic("validator not found")
	}

	valConsAddr, err := validator.GetConsAddr()

	requireNoErr(err)

	valAddr := valConsAddr.Bytes()

	newBlockTime := env.Ctx.BlockTime().Add(time.Duration(timeIncreaseSeconds) * time.Second)

	header := tmproto.Header{ChainID: "injective-777", Height: env.Ctx.BlockHeight() + 1, Time: newBlockTime}
	newCtx := env.Ctx.WithBlockTime(newBlockTime).WithBlockHeight(env.Ctx.BlockHeight() + 1)
	env.Ctx = newCtx
	lastCommitInfo := abci.CommitInfo{
		Votes: []abci.VoteInfo{{
			Validator:       abci.Validator{Address: valAddr, Power: 1000},
			SignedLastBlock: true,
		}},
	}
	reqBeginBlock := abci.RequestBeginBlock{Header: header, LastCommitInfo: lastCommitInfo}

	env.App.BeginBlock(reqBeginBlock)
	env.Ctx = env.App.NewContext(false, reqBeginBlock.Header)
}

func (env *TestEnv) SetDefaultValidator(consAddr sdk.ConsAddress) {
	signingInfo := slashingtypes.NewValidatorSigningInfo(
		consAddr,
		env.Ctx.BlockHeight(),
		0,
		time.Unix(0, 0),
		false,
		0,
	)
	env.App.SlashingKeeper.SetValidatorSigningInfo(env.Ctx, consAddr, signingInfo)
}

func (env *TestEnv) setupValidator(bondStatus stakingtypes.BondStatus) sdk.ValAddress {
	// valPk := secp256k1.GenPrivKey()
	// set up the validator
	valPk := ed25519.GenPrivKey()

	// env.Validator = valPk.Bytes()

	valPub := valPk.PubKey()

	valAddr := sdk.ValAddress(valPub.Address())
	bondDenom := env.App.StakingKeeper.GetParams(env.Ctx).BondDenom
	selfBond := sdk.NewCoins(sdk.Coin{Amount: sdk.NewInt(100), Denom: bondDenom})

	err := testutil.FundAccount(env.App.BankKeeper, env.Ctx, sdk.AccAddress(valPub.Address()), selfBond)
	requireNoErr(err)

	stakingHandler := stakingkeeper.NewMsgServerImpl(env.App.StakingKeeper)
	stakingCoin := sdk.NewCoin(bondDenom, selfBond[0].Amount)

	Commission := stakingtypes.NewCommissionRates(sdk.MustNewDecFromStr("0.05"), sdk.MustNewDecFromStr("0.05"), sdk.MustNewDecFromStr("0.05"))
	msg, err := stakingtypes.NewMsgCreateValidator(valAddr, valPub, stakingCoin, stakingtypes.Description{}, Commission, sdk.OneInt())
	requireNoErr(err)

	res, err := stakingHandler.CreateValidator(env.Ctx, msg)
	requireNoErr(err)

	requireNoNil("staking handler", res)

	env.App.BankKeeper.SendCoinsFromModuleToModule(env.Ctx, stakingtypes.NotBondedPoolName, stakingtypes.BondedPoolName, sdk.NewCoins(stakingCoin))

	val, found := env.App.StakingKeeper.GetValidator(env.Ctx, valAddr)
	requierTrue("validator found", found)

	val = val.UpdateStatus(bondStatus)
	env.App.StakingKeeper.SetValidator(env.Ctx, val)

	consAddr, err := val.GetConsAddr()
	requireNoErr(err)

	signingInfo := slashingtypes.NewValidatorSigningInfo(
		consAddr,
		env.Ctx.BlockHeight(),
		0,
		time.Unix(0, 0),
		false,
		0,
	)
	env.App.SlashingKeeper.SetValidatorSigningInfo(env.Ctx, consAddr, signingInfo)

	return valAddr
}

func (env *TestEnv) SetupParamTypes() {
	pReg := env.ParamTypesRegistry

	pReg.RegisterParamSet(&tokenfactorytypes.Params{})
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
