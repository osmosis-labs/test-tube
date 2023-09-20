package testenv

import (
	"encoding/json"
	"fmt"
	"strings"
	"time"

	dbm "github.com/cometbft/cometbft-db"
	abci "github.com/cometbft/cometbft/abci/types"
	"github.com/cometbft/cometbft/libs/log"
	tmproto "github.com/cometbft/cometbft/proto/tendermint/types"
	tmtypes "github.com/cometbft/cometbft/types"
	"github.com/cosmos/cosmos-sdk/baseapp"
	"github.com/cosmos/cosmos-sdk/crypto/keys/ed25519"
	"github.com/cosmos/cosmos-sdk/crypto/keys/secp256k1"
	"github.com/cosmos/cosmos-sdk/server"
	"github.com/cosmos/cosmos-sdk/testutil/mock"
	simtestutil "github.com/cosmos/cosmos-sdk/testutil/sims"
	sdk "github.com/cosmos/cosmos-sdk/types"
	authtypes "github.com/cosmos/cosmos-sdk/x/auth/types"
	"github.com/cosmos/cosmos-sdk/x/bank/testutil"
	slashingtypes "github.com/cosmos/cosmos-sdk/x/slashing/types"
	stakingkeeper "github.com/cosmos/cosmos-sdk/x/staking/keeper"
	stakingtypes "github.com/cosmos/cosmos-sdk/x/staking/types"

	coreumapp "github.com/CoreumFoundation/coreum/v3/app"
	coreumconfig "github.com/CoreumFoundation/coreum/v3/pkg/config"
	coreumconstant "github.com/CoreumFoundation/coreum/v3/pkg/config/constant"
	assetfttypes "github.com/CoreumFoundation/coreum/v3/x/asset/ft/types"
	assetnfttypes "github.com/CoreumFoundation/coreum/v3/x/asset/nft/types"
)

var NetworkConfig coreumconfig.NetworkConfig

func init() {
	NetworkConfig = newNetworkConfig()
	NetworkConfig.SetSDKConfig()
	coreumapp.ChosenNetwork = NetworkConfig
}

type TestEnv struct {
	App                *coreumapp.App
	Ctx                sdk.Context
	ParamTypesRegistry ParamTypeRegistry
	Validator          []byte
	NodeHome           string
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

func SetupApp(nodeHome string) (*coreumapp.App, []byte) {
	db := dbm.NewMemDB()
	appInstance := coreumapp.New(
		log.NewNopLogger(),
		db,
		nil,
		true,
		simtestutil.NewAppOptionsWithFlagHome(nodeHome),
		baseapp.SetChainID(string(NetworkConfig.ChainID())),
	)

	networkProvider, ok := NetworkConfig.Provider.(coreumconfig.DynamicConfigProvider)
	if !ok {
		panic("failed to cast network config provider to DynamicConfigProvider")
	}

	// create validator set with single validator
	validatorKey := secp256k1.GenPrivKey()
	pval := mock.PV{PrivKey: validatorKey}
	conval := mock.PV{PrivKey: ed25519.GenPrivKey()}
	pubKey, err := pval.GetPubKey()
	requireNoErr(err)
	validator := tmtypes.NewValidator(pubKey, 1)
	conPubKey, err := conval.GetPubKey()
	validator.PubKey = conPubKey
	valSet := tmtypes.NewValidatorSet([]*tmtypes.Validator{validator})

	// generate at least one account
	senderPrivKey := ed25519.GenPrivKey()
	acc := authtypes.NewBaseAccount(senderPrivKey.PubKey().Address().Bytes(), senderPrivKey.PubKey(), 0, 0)

	// generate network state
	genesisState, err := networkProvider.AppState()
	requireNoErr(err)

	// register the validator and account in the genesis
	genesisState, err = simtestutil.GenesisStateWithValSet(appInstance.AppCodec(), genesisState, valSet, []authtypes.GenesisAccount{acc})

	stateBytes, err := json.MarshalIndent(genesisState, "", " ")
	requireNoErr(err)

	// the `GenesisStateWithValSet` uses the `sdk.DefaultBondDenom` as denom for the balances, replace with correct
	stateBytes = []byte(
		strings.ReplaceAll(string(stateBytes),
			fmt.Sprintf("\"%s\"", sdk.DefaultBondDenom),
			fmt.Sprintf("\"%s\"", NetworkConfig.Denom()),
		))

	consensusParams := simtestutil.DefaultConsensusParams
	// same setting as on mainnet
	consensusParams.Block.MaxBytes = 22020096
	consensusParams.Block.MaxGas = 50000000

	appInstance.InitChain(
		abci.RequestInitChain{
			ChainId:         string(NetworkConfig.ChainID()),
			Validators:      []abci.ValidatorUpdate{},
			ConsensusParams: consensusParams,
			AppStateBytes:   stateBytes,
		},
	)

	return appInstance, validatorKey.Bytes()
}

func (env *TestEnv) BeginNewBlock(timeIncreaseSeconds uint64) {
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

	env.beginNewBlockWithProposer(valAddr, timeIncreaseSeconds)
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
func (env *TestEnv) beginNewBlockWithProposer(proposer sdk.ConsAddress, timeIncreaseSeconds uint64) {
	validator, found := env.App.StakingKeeper.GetValidatorByConsAddr(env.Ctx, proposer)

	if !found {
		panic("validator not found")
	}

	valConsAddr, err := validator.GetConsAddr()
	requireNoErr(err)

	valAddr := valConsAddr.Bytes()

	newBlockTime := env.Ctx.BlockTime().Add(time.Duration(timeIncreaseSeconds) * time.Second)
	header := tmproto.Header{ChainID: string(NetworkConfig.ChainID()), Height: env.Ctx.BlockHeight() + 1, Time: newBlockTime}
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

func (env *TestEnv) setupValidator(bondStatus stakingtypes.BondStatus) sdk.ValAddress {
	valPk := ed25519.GenPrivKey()
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
	requireTrue("validator found", found)

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

	pReg.RegisterParamSet(&assetfttypes.Params{})
	pReg.RegisterParamSet(&assetnfttypes.Params{})
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

func requireTrue(name string, b bool) {
	if !b {
		panic(fmt.Sprintf("%s must be true", name))
	}
}

func newNetworkConfig() coreumconfig.NetworkConfig {
	networkConfig := coreumconfig.NetworkConfig{
		Provider: coreumconfig.DynamicConfigProvider{
			AddressPrefix:   coreumconstant.AddressPrefixMain,
			GenesisTemplate: coreumconfig.GenesisV3Template,
			ChainID:         coreumconstant.ChainIDMain,
			GenesisTime:     time.Now(),
			BlockTimeIota:   time.Second,
			Denom:           coreumconstant.DenomMain,
			GovConfig: coreumconfig.GovConfig{
				ProposalConfig: coreumconfig.GovProposalConfig{
					MinDepositAmount: "1000",
					VotingPeriod:     (time.Second * 10).String(),
				},
			},
			CustomParamsConfig: coreumconfig.CustomParamsConfig{
				Staking: coreumconfig.CustomParamsStakingConfig{
					MinSelfDelegation: sdk.NewInt(10_000_000), // 10 core
				},
			},
		},
	}

	return networkConfig
}
