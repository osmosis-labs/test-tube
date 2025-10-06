package testenv

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"
	"time"

	"cosmossdk.io/log"
	"cosmossdk.io/math"
	abci "github.com/cometbft/cometbft/abci/types"
	tmproto "github.com/cometbft/cometbft/proto/tendermint/types"
	tmtypes "github.com/cometbft/cometbft/types"
	dbm "github.com/cosmos/cosmos-db"
	"github.com/cosmos/cosmos-sdk/baseapp"
	cosmosclient "github.com/cosmos/cosmos-sdk/client"
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
	"github.com/pkg/errors"

	txapp "github.com/tokenize-x/tx-chain/v6/app"
	txconfig "github.com/tokenize-x/tx-chain/v6/pkg/config"
	txconstant "github.com/tokenize-x/tx-chain/v6/pkg/config/constant"
	assetfttypes "github.com/tokenize-x/tx-chain/v6/x/asset/ft/types"
	assetnfttypes "github.com/tokenize-x/tx-chain/v6/x/asset/nft/types"
)

var NetworkConfig txconfig.NetworkConfig

func init() {
	NetworkConfig = newNetworkConfig()
	NetworkConfig.SetSDKConfig()
	txapp.ChosenNetwork = NetworkConfig
}

type TestEnv struct {
	App                *txapp.App
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

func SetupApp(nodeHome string) (*txapp.App, []byte) {
	db := dbm.NewMemDB()
	appInstance := txapp.New(
		log.NewNopLogger(),
		db,
		nil,
		true,
		simtestutil.NewAppOptionsWithFlagHome(nodeHome),
		baseapp.SetChainID(string(NetworkConfig.ChainID())),
	)

	networkProvider, ok := NetworkConfig.Provider.(txconfig.DynamicConfigProvider)
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

	clientCtx := cosmosclient.Context{}.
		WithCodec(appInstance.AppCodec()).
		WithInterfaceRegistry(appInstance.InterfaceRegistry()).
		WithTxConfig(appInstance.TxConfig())

	// generate network state
	genesisState, err := networkProvider.AppState(context.Background(), clientCtx, appInstance.BasicModuleManager)
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

	_, err = appInstance.InitChain(
		&abci.RequestInitChain{
			ChainId:         string(NetworkConfig.ChainID()),
			Validators:      []abci.ValidatorUpdate{},
			ConsensusParams: consensusParams,
			AppStateBytes:   stateBytes,
		},
	)
	if err != nil {
		panic(errors.Errorf("can't init chain: %s", err))
	}

	return appInstance, validatorKey.Bytes()
}

func (env *TestEnv) BeginNewBlock(timeIncreaseSeconds uint64) {
	var valAddr []byte

	validators, err := env.App.StakingKeeper.GetAllValidators(env.Ctx)
	if err != nil {
		panic(errors.Errorf("can't begin new block: %s", err))
	}
	if len(validators) >= 1 {
		valAddrFancy, err := validators[0].GetConsAddr()
		requireNoErr(err)
		valAddr = valAddrFancy
	} else {
		valAddrFancy := env.setupValidator(stakingtypes.Bonded)
		validator, _ := env.App.StakingKeeper.GetValidator(env.Ctx, valAddrFancy)
		valAddr2, _ := validator.GetConsAddr()
		valAddr = valAddr2
	}

	env.beginNewBlockWithProposer(valAddr, timeIncreaseSeconds)
}

func (env *TestEnv) GetValidatorAddresses() []string {
	validators, err := env.App.StakingKeeper.GetAllValidators(env.Ctx)
	if err != nil {
		panic(errors.Errorf("can't get validator addresses: %s", err))
	}
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
	validator, err := env.App.StakingKeeper.GetValidatorByConsAddr(env.Ctx, proposer)

	if err != nil {
		panic(errors.Errorf("can't begin a new block: %s", err))
	}

	valAddr, err := validator.GetConsAddr()
	requireNoErr(err)

	newBlockTime := env.Ctx.BlockTime().Add(time.Duration(timeIncreaseSeconds) * time.Second)
	header := tmproto.Header{ChainID: string(NetworkConfig.ChainID()), Height: env.Ctx.BlockHeight() + 1, Time: newBlockTime}
	newCtx := env.Ctx.WithBlockTime(newBlockTime).WithBlockHeight(env.Ctx.BlockHeight() + 1)
	env.Ctx = newCtx
	requestFinalizeBlock := &abci.RequestFinalizeBlock{
		Txs: nil,
		DecidedLastCommit: abci.CommitInfo{
			Votes: []abci.VoteInfo{{
				Validator: abci.Validator{Address: valAddr, Power: 1000},
			}},
		},
		Height:             newCtx.BlockHeight(),
		Time:               newCtx.BlockTime(),
		NextValidatorsHash: nil,
		ProposerAddress:    valAddr,
	}
	_, err = env.App.FinalizeBlock(requestFinalizeBlock)
	if err != nil {
		panic(errors.Errorf("can't begin a new block: %s", err))
	}
	env.Ctx = env.App.NewContextLegacy(false, header)
}

func (env *TestEnv) setupValidator(bondStatus stakingtypes.BondStatus) sdk.ValAddress {
	valPk := ed25519.GenPrivKey()
	valPub := valPk.PubKey()
	valAddr := sdk.ValAddress(valPub.Address())

	params, err := env.App.StakingKeeper.GetParams(env.Ctx)
	requireNoErr(err)
	requireNoErr(err)
	bondDenom := params.BondDenom
	selfBond := sdk.NewCoins(sdk.Coin{Amount: math.NewInt(100), Denom: bondDenom})

	err = testutil.FundAccount(env.Ctx, env.App.BankKeeper, sdk.AccAddress(valPub.Address()), selfBond)
	requireNoErr(err)

	stakingHandler := stakingkeeper.NewMsgServerImpl(env.App.StakingKeeper)
	stakingCoin := sdk.NewCoin(bondDenom, selfBond[0].Amount)

	Commission := stakingtypes.NewCommissionRates(math.LegacyMustNewDecFromStr("0.05"), math.LegacyMustNewDecFromStr("0.05"), math.LegacyMustNewDecFromStr("0.05"))
	msg, err := stakingtypes.NewMsgCreateValidator(valAddr.String(), valPub, stakingCoin, stakingtypes.Description{}, Commission, math.OneInt())
	requireNoErr(err)

	res, err := stakingHandler.CreateValidator(env.Ctx, msg)
	requireNoErr(err)

	requireNoNil("staking handler", res)

	err = env.App.BankKeeper.SendCoinsFromModuleToModule(env.Ctx, stakingtypes.NotBondedPoolName, stakingtypes.BondedPoolName, sdk.NewCoins(stakingCoin))
	requireNoErr(err)

	val, err := env.App.StakingKeeper.GetValidator(env.Ctx, valAddr)
	requireNoErr(err)

	val = val.UpdateStatus(bondStatus)
	err = env.App.StakingKeeper.SetValidator(env.Ctx, val)
	requireNoErr(err)

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
	err = env.App.SlashingKeeper.SetValidatorSigningInfo(env.Ctx, consAddr, signingInfo)
	requireNoErr(err)

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

func newNetworkConfig() txconfig.NetworkConfig {
	networkConfig := txconfig.NetworkConfig{
		Provider: txconfig.DynamicConfigProvider{
			GenesisInitConfig: txconfig.GenesisInitConfig{
				AddressPrefix: txconstant.AddressPrefixMain,
				ChainID:       txconstant.ChainIDMain,
				GenesisTime:   time.Now(),
				Denom:         txconstant.DenomMain,
				GovConfig: txconfig.GenesisInitGovConfig{
					MinDeposit:   sdk.Coins{sdk.NewCoin(txconstant.DenomMain, math.NewInt(1000))},
					VotingPeriod: time.Second * 10,
				},
				CustomParamsConfig: txconfig.GenesisInitCustomParamsConfig{
					MinSelfDelegation: math.NewInt(10_000_000), // 10 core
				},
			},
		},
	}

	return networkConfig
}
