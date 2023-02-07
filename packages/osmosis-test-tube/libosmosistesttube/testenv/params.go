package testenv

import (
	"fmt"

	codectypes "github.com/cosmos/cosmos-sdk/codec/types"
	paramstypes "github.com/cosmos/cosmos-sdk/x/params/types"
	proto "github.com/gogo/protobuf/proto"
)

type ProtoParamSet interface {
	proto.Message
	paramstypes.ParamSet
}

type ParamTypeRegistry struct {
	m map[string]ProtoParamSet
}

func NewParamTypeRegistry() *ParamTypeRegistry {
	return &ParamTypeRegistry{
		m: make(map[string]ProtoParamSet),
	}
}

func (r *ParamTypeRegistry) RegisterParamSet(p ProtoParamSet) {
	r.m["/"+proto.MessageName(p)] = p
}
func (r *ParamTypeRegistry) GetEmptyParamsSet(typeUrl string) (ProtoParamSet, bool) {
	_pset, ok := r.m[typeUrl]
	pset := proto.Clone(_pset)
	pset.Reset()
	return pset.(ProtoParamSet), ok
}

func (r *ParamTypeRegistry) UnpackAny(any *codectypes.Any) (ProtoParamSet, error) {
	msg, ok := r.GetEmptyParamsSet(any.GetTypeUrl())

	if !ok {
		return nil, fmt.Errorf("type %s is not registered", any.GetTypeUrl())
	}

	err := proto.Unmarshal(any.Value, msg)

	if err != nil {
		return nil, err
	}

	return msg, nil
}
