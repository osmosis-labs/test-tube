pub mod app;

#[cfg(test)]
mod tests {
    use std::ffi::CString;

    use super::app::OsmosisTestApp;
    use osmosis_std::types::osmosis::gamm::poolmodels::balancer::v1beta1::{
        MsgCreateBalancerPool, MsgCreateBalancerPoolResponse,
    };
    use osmosis_std::types::osmosis::gamm::v1beta1::PoolParams;
    use osmosis_std::types::osmosis::poolmanager::v1beta1::{PoolRequest, PoolResponse};
    use test_tube::account::Account;
    use test_tube::runner::error::RunnerError::{ExecuteError, QueryError};
    use test_tube::runner::result::RawResult;
    use test_tube::runner::Runner;
    use test_tube::RunnerExecuteResult;

    #[derive(::prost::Message)]
    struct AdhocRandomQueryRequest {
        #[prost(uint64, tag = "1")]
        id: u64,
    }

    #[derive(::prost::Message)]
    struct AdhocRandomQueryResponse {
        #[prost(string, tag = "1")]
        msg: String,
    }

    #[test]
    fn test_query_error_no_route() {
        let app = OsmosisTestApp::default();
        let res = app.query::<AdhocRandomQueryRequest, AdhocRandomQueryResponse>(
            "/osmosis.random.v1beta1.Query/AdhocRandom",
            &AdhocRandomQueryRequest { id: 1 },
        );

        let err = res.unwrap_err();
        assert_eq!(
            err,
            QueryError {
                msg: "No route found for `/osmosis.random.v1beta1.Query/AdhocRandom`".to_string()
            }
        );
    }

    #[test]
    fn test_query_error_failed_query() {
        let app = OsmosisTestApp::default();
        let res = app.query::<PoolRequest, PoolResponse>(
            "/osmosis.poolmanager.v1beta1.Query/Pool",
            &PoolRequest { pool_id: 1 },
        );

        let err = res.unwrap_err();
        assert_eq!(
            err,
            QueryError {
                msg: "rpc error: code = Internal desc = failed to find route for pool id (1)"
                    .to_string()
            }
        );
    }

    #[test]
    fn test_execute_error() {
        let app = OsmosisTestApp::default();
        let signer = app.init_account(&[]).unwrap();
        let res: RunnerExecuteResult<MsgCreateBalancerPoolResponse> = app.execute(
            MsgCreateBalancerPool {
                sender: signer.address(),
                pool_params: Some(PoolParams {
                    swap_fee: "10000000000000000".to_string(),
                    exit_fee: "10000000000000000".to_string(),
                    smooth_weight_change_params: None,
                }),
                pool_assets: vec![],
                future_pool_governor: "".to_string(),
            },
            MsgCreateBalancerPool::TYPE_URL,
            &signer,
        );

        let err = res.unwrap_err();
        assert_eq!(
            err,
            ExecuteError {
                msg: String::from("pool should have at least 2 assets, as they must be swapping between at least two assets")
            }
        )
    }

    #[test]
    fn test_raw_result_ptr_with_0_bytes_in_content_should_not_error() {
        let base64_string = base64::encode(vec![vec![0u8], vec![0u8]].concat());
        let res = unsafe {
            RawResult::from_ptr(
                CString::new(base64_string.as_bytes().to_vec())
                    .unwrap()
                    .into_raw(),
            )
        }
        .unwrap()
        .into_result()
        .unwrap();

        assert_eq!(res, vec![0u8]);
    }
}
