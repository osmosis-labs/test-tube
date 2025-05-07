use cosmwasm_std::Coin;
use osmosis_std::types::osmosis::{
    incentives::{
        GaugeByIdRequest, GaugeByIdResponse, MsgCreateGauge, MsgCreateGaugeResponse,
        QueryLockableDurationsRequest, QueryLockableDurationsResponse,
    },
    lockup::{LockQueryType, QueryCondition},
    poolincentives::{
        self,
        v1beta1::{QueryGaugeIdsRequest, QueryGaugeIdsResponse},
    },
};
use test_tube::{fn_execute, fn_query, Account, Module, Runner};

use crate::{Gamm, OsmosisTestApp};

pub struct Incentives<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Incentives<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Incentives<'a, R>
where
    R: Runner<'a>,
{
    fn_execute! {
        pub create_gauge: MsgCreateGauge => MsgCreateGaugeResponse
    }

    fn_query! {
        pub query_lockable_durations ["/osmosis.incentives.Query/LockableDurations"]: QueryLockableDurationsRequest => QueryLockableDurationsResponse
    }

    fn_query! {
        pub query_gauge_by_id ["/osmosis.incentives.Query/GaugeByID"]: GaugeByIdRequest => GaugeByIdResponse
    }
}
pub struct Poolincentives<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Poolincentives<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Poolincentives<'a, R>
where
    R: Runner<'a>,
{
    fn_query! {
        pub query_gauge_ids ["/osmosis.poolincentives.v1beta1.Query/GaugeIds"]: QueryGaugeIdsRequest => QueryGaugeIdsResponse
    }
}

#[test]
fn gauge_gets_lost() {
    let app = OsmosisTestApp::default();
    let gamm = Gamm::new(&app);
    let incentives = Incentives::new(&app);
    let poolincentives = Poolincentives::new(&app);

    let denom0 = "uatom";
    let denom1 = "uosmo";
    let alice = app
        .init_account(&[
            Coin::new(2_000_000_000_000, denom0),
            Coin::new(1_000_000_000_000, denom1),
        ])
        .unwrap();

    let pool_id = gamm
        .create_basic_pool(
            &[
                Coin::new(100_000_000_000, denom0),
                Coin::new(100_000_000_000, denom1),
            ],
            &alice,
        )
        .unwrap()
        .data
        .pool_id;

    // Saving next gauge_id, because `MsgCreateGaugeResponse` does not return gauge id, but that's another topic
    let gauge_ids = poolincentives
        .query_gauge_ids(&{ QueryGaugeIdsRequest { pool_id } })
        .unwrap();
    let next_gauge_id = gauge_ids
        .gauge_ids_with_duration
        .iter()
        .last()
        .map(|gauge| gauge.gauge_id + 1)
        .unwrap();

    let lockable_durations = incentives
        .query_lockable_durations(&QueryLockableDurationsRequest {})
        .unwrap();

    let seconds = app.get_block_time_seconds();
    incentives
        .create_gauge(
            MsgCreateGauge {
                pool_id: 0, // Can't set it with LockQueryType::ByDuration
                is_perpetual: false,
                owner: alice.address(),
                distribute_to: Some(QueryCondition {
                    lock_query_type: LockQueryType::ByDuration.into(),
                    denom: format!("gamm/pool/{pool_id}"),
                    duration: Some(lockable_durations.lockable_durations[0].clone()),
                    timestamp: None,
                }),
                coins: vec![osmosis_std::types::cosmos::base::v1beta1::Coin {
                    denom: denom0.to_owned(),
                    amount: "100000000000".to_owned(),
                }],
                start_time: Some(osmosis_std::shim::Timestamp { seconds, nanos: 0 }),
                num_epochs_paid_over: 10,
            },
            &alice,
        )
        .unwrap();

    // Not added to the gauge_ids for this pool id
    let gauge_ids = poolincentives
        .query_gauge_ids(&{ QueryGaugeIdsRequest { pool_id } })
        .unwrap();
    assert!(gauge_ids
        .gauge_ids_with_duration
        .iter()
        .find(|gauge| gauge.gauge_id == next_gauge_id)
        .is_none());

    // But it exists
    let new_gauge = incentives.query_gauge_by_id(&GaugeByIdRequest { id: next_gauge_id });
    assert!(new_gauge.is_ok());
}
