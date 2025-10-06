use crate::{fn_execute, fn_query};

use tx_wasm_sdk::types::cosmos::authz::v1beta1::{
    MsgExec, MsgExecResponse, MsgGrant, MsgGrantResponse, MsgRevoke, MsgRevokeResponse,
    QueryGranteeGrantsRequest, QueryGranteeGrantsResponse, QueryGranterGrantsRequest,
    QueryGranterGrantsResponse, QueryGrantsRequest, QueryGrantsResponse,
};
use test_tube_tx::module::Module;
use test_tube_tx::runner::Runner;

pub struct Authz<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Authz<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Authz<'a, R>
where
    R: Runner<'a>,
{
    fn_execute! {
        pub grant: MsgGrant["/cosmos.authz.v1beta1.MsgGrant"] => MsgGrantResponse
    }

    fn_execute! {
        pub revoke: MsgRevoke["/cosmos.authz.v1beta1.MsgRevoke"] => MsgRevokeResponse
    }

    fn_execute! {
        pub exec: MsgExec["/cosmos.authz.v1beta1.MsgExec"] => MsgExecResponse
    }

    fn_query! {
        pub query_grants ["/cosmos.authz.v1beta1.Query/Grants"]: QueryGrantsRequest => QueryGrantsResponse
    }

    fn_query! {
        pub query_granter_grants ["/cosmos.authz.v1beta1.Query/GranterGrants"]: QueryGranterGrantsRequest => QueryGranterGrantsResponse
    }

    fn_query! {
        pub query_grantee_grants ["/cosmos.authz.v1beta1.Query/GranteeGrants"]: QueryGranteeGrantsRequest => QueryGranteeGrantsResponse
    }
}

#[cfg(test)]
mod tests {
    use tx_wasm_sdk::shim::Any;
    use tx_wasm_sdk::types::coreum::asset::nft::v1::{
        MsgIssueClass, MsgMint, NftIdentifier, SendAuthorization,
    };
    use tx_wasm_sdk::types::cosmos::authz::v1beta1::{
        Grant, MsgExec, MsgGrant, QueryGrantsRequest,
    };
    use tx_wasm_sdk::types::cosmos::nft::v1beta1::{MsgSend, QueryOwnerRequest};
    use cosmwasm_std::Coin;

    use crate::runner::app::FEE_DENOM;
    use crate::{Account, AssetNFT, Authz, TXTestApp, Module, NFT};

    #[test]
    fn authz_integration() {
        let app = TXTestApp::new();
        let signer = app
            .init_account(&[Coin::new(100_000_000_000_000_000_000u128, FEE_DENOM)])
            .unwrap();
        let grantee = app
            .init_account(&[Coin::new(100_000_000_000_000_000_000u128, FEE_DENOM)])
            .unwrap();
        let authz = Authz::new(&app);
        let assetnft = AssetNFT::new(&app);
        let nft = NFT::new(&app);

        assetnft
            .issue(
                MsgIssueClass {
                    issuer: signer.address(),
                    symbol: "TEST".to_string(),
                    name: "TEST_NAME".to_string(),
                    description: "test_description".to_string(),
                    uri: "".to_string(),
                    uri_hash: "".to_string(),
                    data: None,
                    features: vec![],
                    royalty_rate: "0".to_string(),
                },
                &signer,
            )
            .unwrap();

        let class_id = format!("{}-{}", "TEST", signer.address()).to_lowercase();

        assetnft
            .mint(
                MsgMint {
                    sender: signer.address(),
                    class_id: class_id.clone(),
                    id: "test1".to_string(),
                    uri: "".to_string(),
                    uri_hash: "".to_string(),
                    data: None,
                    recipient: signer.address(),
                },
                &signer,
            )
            .unwrap();

        let owner_response = nft
            .query_owner(&QueryOwnerRequest {
                class_id: class_id.clone(),
                id: "test1".to_string(),
            })
            .unwrap();

        assert_eq!(owner_response.owner, signer.address());

        let send_authorization = SendAuthorization {
            nfts: vec![NftIdentifier {
                class_id: class_id.clone(),
                id: "test1".to_string(),
            }],
        }
        .to_any();

        authz
            .grant(
                MsgGrant {
                    granter: signer.address(),
                    grantee: grantee.address(),
                    grant: Some(Grant {
                        authorization: Some(Any {
                            type_url: send_authorization.type_url,
                            value: send_authorization.value.to_vec(),
                        }),
                        expiration: None,
                    }),
                },
                &signer,
            )
            .unwrap();

        let grants = authz
            .query_grants(&QueryGrantsRequest {
                msg_type_url: "".to_string(),
                pagination: None,
                granter: signer.address(),
                grantee: grantee.address(),
            })
            .unwrap();

        assert_eq!(grants.grants.len(), 1);

        let send_msg = MsgSend {
            class_id: class_id.clone(),
            id: "test1".to_string(),
            sender: signer.address(),
            receiver: grantee.address(),
        };

        authz
            .exec(
                MsgExec {
                    grantee: grantee.address(),
                    msgs: vec![Any {
                        type_url: send_msg.to_any().type_url,
                        value: send_msg.to_any().value.to_vec(),
                    }],
                },
                &grantee,
            )
            .unwrap();

        let owner_response = nft
            .query_owner(&QueryOwnerRequest {
                class_id,
                id: "test1".to_string(),
            })
            .unwrap();

        assert_eq!(owner_response.owner, grantee.address());
    }
}
