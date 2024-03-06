use osmosis_std::types::cosmos::authz::v1beta1::{
    MsgExec, MsgExecResponse, MsgGrant, MsgGrantResponse, QueryGranteeGrantsRequest,
    QueryGranteeGrantsResponse, QueryGranterGrantsRequest, QueryGranterGrantsResponse,
    QueryGrantsRequest, QueryGrantsResponse,
};
use test_tube::{fn_execute, fn_query};

use test_tube::module::Module;
use test_tube::runner::Runner;

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
        pub exec: MsgExec["/cosmos.authz.v1beta1.MsgExec"] => MsgExecResponse
    }

    fn_execute! {
        pub grant: MsgGrant["/cosmos.authz.v1beta1.MsgGrant"] => MsgGrantResponse
    }

    fn_query! {
        pub query_grantee_grants ["/cosmos.authz.v1beta1.Query/GranteeGrants"]: QueryGranteeGrantsRequest => QueryGranteeGrantsResponse
    }

    fn_query! {
        pub query_granter_grants ["/cosmos.authz.v1beta1.Query/GranterGrants"]: QueryGranterGrantsRequest => QueryGranterGrantsResponse
    }

    fn_query! {
        pub query_grants ["/cosmos.authz.v1beta1.Query/Grants"]: QueryGrantsRequest => QueryGrantsResponse
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::Coin;
    use osmosis_std::shim::{Any, Timestamp};
    use osmosis_std::types::{
        cosmos::authz::v1beta1::{
            GenericAuthorization, Grant, GrantAuthorization, MsgExec, MsgGrant,
            QueryGranteeGrantsRequest, QueryGranterGrantsRequest,
        },
        cosmos::bank::v1beta1::{MsgSend, QueryBalanceRequest, SendAuthorization},
        cosmos::base::v1beta1::Coin as BaseCoin,
    };
    use prost::Message;

    use crate::{Account, Authz, Bank, OsmosisTestApp};
    use test_tube::Module;

    const TWO_WEEKS_SECS: i64 = 14 * 24 * 60 * 60;

    #[test]
    fn authz_integration() {
        let app = OsmosisTestApp::new();
        let signer = app
            .init_account(&[
                Coin::new(100_000_000_000_000_000_000u128, "uosmo"),
                Coin::new(10u128, "usdc"),
            ])
            .unwrap();
        let receiver = app
            .init_account(&[Coin::new(1_000_000_000_000u128, "uosmo")])
            .unwrap();
        let authz = Authz::new(&app);
        let bank = Bank::new(&app);

        let response = authz
            .query_grantee_grants(&QueryGranteeGrantsRequest {
                grantee: receiver.address(),
                pagination: None,
            })
            .unwrap();
        assert_eq!(response.grants, vec![]);

        let mut buf = vec![];
        SendAuthorization::encode(
            &SendAuthorization {
                spend_limit: vec![BaseCoin {
                    amount: 10u128.to_string(),
                    denom: "usdc".to_string(),
                }],
            },
            &mut buf,
        )
        .unwrap();

        let timestamp = app.get_block_timestamp().seconds() as i64;
        let ts: i64 = timestamp + TWO_WEEKS_SECS;

        let expiration = Timestamp {
            seconds: ts,
            nanos: 0_i32,
        };

        authz
            .grant(
                MsgGrant {
                    granter: signer.address(),
                    grantee: receiver.address(),
                    grant: Some(Grant {
                        authorization: Some(Any {
                            type_url: "/cosmos.bank.v1beta1.SendAuthorization".to_string(),
                            value: buf.clone(),
                        }),
                        expiration: Some(expiration.clone()),
                    }),
                },
                &signer,
            )
            .unwrap();

        let response = authz
            .query_grantee_grants(&QueryGranteeGrantsRequest {
                grantee: receiver.address(),
                pagination: None,
            })
            .unwrap();
        assert_eq!(
            response.grants,
            vec![GrantAuthorization {
                granter: signer.address(),
                grantee: receiver.address(),
                authorization: Some(Any {
                    type_url: "/cosmos.bank.v1beta1.SendAuthorization".to_string(),
                    value: buf.clone(),
                }),
                expiration: Some(expiration.clone()),
            }]
        );

        let mut buf_2 = vec![];
        GenericAuthorization::encode(
            &GenericAuthorization {
                msg: "/cosmos.gov.v1beta1.MsgVote".to_string(),
            },
            &mut buf_2,
        )
        .unwrap();

        authz
            .grant(
                MsgGrant {
                    granter: signer.address(),
                    grantee: receiver.address(),
                    grant: Some(Grant {
                        authorization: Some(Any {
                            type_url: "/cosmos.authz.v1beta1.GenericAuthorization".to_string(),
                            value: buf_2.clone(),
                        }),
                        expiration: Some(expiration.clone()),
                    }),
                },
                &signer,
            )
            .unwrap();

        let response = authz
            .query_grantee_grants(&QueryGranteeGrantsRequest {
                grantee: receiver.address(),
                pagination: None,
            })
            .unwrap();
        assert_eq!(
            response.grants,
            vec![
                GrantAuthorization {
                    granter: signer.address(),
                    grantee: receiver.address(),
                    authorization: Some(Any {
                        type_url: "/cosmos.bank.v1beta1.SendAuthorization".to_string(),
                        value: buf.clone(),
                    }),
                    expiration: Some(expiration.clone()),
                },
                GrantAuthorization {
                    granter: signer.address(),
                    grantee: receiver.address(),
                    authorization: Some(Any {
                        type_url: "/cosmos.authz.v1beta1.GenericAuthorization".to_string(),
                        value: buf_2.clone(),
                    }),
                    expiration: Some(expiration.clone()),
                }
            ]
        );

        let response = authz
            .query_granter_grants(&QueryGranterGrantsRequest {
                granter: signer.address(),
                pagination: None,
            })
            .unwrap();
        assert_eq!(
            response.grants,
            vec![
                GrantAuthorization {
                    granter: signer.address(),
                    grantee: receiver.address(),
                    authorization: Some(Any {
                        type_url: "/cosmos.bank.v1beta1.SendAuthorization".to_string(),
                        value: buf,
                    }),
                    expiration: Some(expiration.clone()),
                },
                GrantAuthorization {
                    granter: signer.address(),
                    grantee: receiver.address(),
                    authorization: Some(Any {
                        type_url: "/cosmos.authz.v1beta1.GenericAuthorization".to_string(),
                        value: buf_2,
                    }),
                    expiration: Some(expiration.clone()),
                }
            ]
        );

        let response = bank
            .query_balance(&QueryBalanceRequest {
                address: receiver.address(),
                denom: "usdc".to_string(),
            })
            .unwrap();
        assert_eq!(
            response.balance.unwrap(),
            BaseCoin {
                amount: 0u128.to_string(),
                denom: "usdc".to_string(),
            }
        );

        let mut buf_3 = vec![];
        MsgSend::encode(
            &MsgSend {
                from_address: signer.address(),
                to_address: receiver.address(),
                amount: vec![BaseCoin {
                    amount: 10u128.to_string(),
                    denom: "usdc".to_string(),
                }],
            },
            &mut buf_3,
        )
        .unwrap();

        authz
            .exec(
                MsgExec {
                    grantee: receiver.address(),
                    msgs: vec![Any {
                        type_url: "/cosmos.bank.v1beta1.MsgSend".to_string(),
                        value: buf_3.clone(),
                    }],
                },
                &receiver,
            )
            .unwrap();

        let response = bank
            .query_balance(&QueryBalanceRequest {
                address: receiver.address(),
                denom: "usdc".to_string(),
            })
            .unwrap();
        assert_eq!(
            response.balance.unwrap(),
            BaseCoin {
                amount: 10u128.to_string(),
                denom: "usdc".to_string(),
            }
        );
    }
}
