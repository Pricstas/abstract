/// Macro to update the ownership of an Abstract contract.
///
/// ```rustignore
/// pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> ContractResult {
///     match msg {
///         ...
///         ExecuteMsg::UpdateOwnership(action) => {
///             execute_update_ownership!(ContractResponse, deps, env, info, action)
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! execute_update_ownership {
    ($response_type:ident, $deps:expr, $env:expr, $info:expr, $action:expr) => {{
        let ownership = cw_ownable::update_ownership($deps, &$env.block, &$info.sender, $action)?;
        Ok($response_type::new(
            "update_ownership",
            ownership.into_attributes(),
        ))
    }};
}

/// Macro to query the ownership of a contract.
///
/// ```rustignore
/// pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
///     match msg {
///         ...
///         QueryMsg::Ownership {} => query_ownership!(deps),
///     }
/// }
/// ```
#[macro_export]
macro_rules! query_ownership {
    ($deps:expr) => {{
        cosmwasm_std::to_json_binary(&cw_ownable::get_ownership($deps.storage)?)
    }};
}

#[cfg(test)]
mod tests {
    use abstract_macros::abstract_response;
    use abstract_testing::mock_env_validated;
    use cosmwasm_schema::{cw_serde, QueryResponses};
    use cosmwasm_std::{
        from_json,
        testing::{message_info, mock_dependencies},
        Addr, Binary, StdError, StdResult,
    };
    use cw_ownable::{cw_ownable_execute, cw_ownable_query, Action, OwnershipError};
    use thiserror::Error;

    const MOCK_CONTRACT: &str = "contract";

    #[abstract_response(MOCK_CONTRACT)]
    pub struct MockResponse;

    #[cw_ownable_execute]
    #[cw_serde]
    enum ExecuteMsg {}

    #[cw_ownable_query]
    #[cw_serde]
    #[derive(QueryResponses)]
    enum QueryMsg {}

    #[derive(Error, Debug, PartialEq)]
    pub enum MockError {
        #[error(transparent)]
        Std(#[from] StdError),
        #[error(transparent)]
        Ownership(#[from] cw_ownable::OwnershipError),
    }

    const NEW_OWNER: &str = "new_owner";
    const OLD_OWNER: &str = "old_owner";

    #[coverage_helper::test]
    fn test_update_ownership_macro() -> Result<(), MockError> {
        let mut deps = mock_dependencies();

        let env = mock_env_validated(deps.api);
        let old_owner = deps.api.addr_make(OLD_OWNER);
        let new_owner = deps.api.addr_make(NEW_OWNER);
        let info = message_info(&old_owner, &[]);

        cw_ownable::initialize_owner(&mut deps.storage, &deps.api, Some(old_owner.as_str()))?;

        let mut_deps = deps.as_mut();

        // ExecuteMsg for testing the macro
        let transfer_ownership_action = Action::TransferOwnership {
            new_owner: new_owner.to_string(),
            expiry: None,
        };

        let ownership_msg = ExecuteMsg::UpdateOwnership(transfer_ownership_action);

        let result: Result<_, OwnershipError> = match ownership_msg {
            ExecuteMsg::UpdateOwnership(action) => {
                execute_update_ownership!(MockResponse, mut_deps, env, info, action)
            }
        };

        let expected_response = MockResponse::new(
            "update_ownership",
            vec![
                ("owner", old_owner.as_str()),
                ("pending_owner", new_owner.as_str()),
                ("pending_expiry", "none"),
            ],
        );

        assert_eq!(result.unwrap(), expected_response);

        Ok(())
    }

    #[coverage_helper::test]
    fn test_query_ownership_macro() -> Result<(), MockError> {
        let mut deps = mock_dependencies();
        let _env = mock_env_validated(deps.api);

        let old_owner = deps.api.addr_make("owner1");

        cw_ownable::initialize_owner(&mut deps.storage, &deps.api, Some(old_owner.as_str()))?;

        // Ownership query message for testing the macro
        let ownership_query_msg = QueryMsg::Ownership {};

        let result: StdResult<Binary> = match ownership_query_msg {
            QueryMsg::Ownership {} => query_ownership!(deps.as_ref()),
        };

        let expected = cw_ownable::Ownership {
            owner: Some(old_owner),
            pending_owner: None,
            pending_expiry: None,
        };

        // Deserialize the query response
        let actual: cw_ownable::Ownership<Addr> = from_json(result.unwrap())?;

        assert_eq!(actual, expected);

        Ok(())
    }
}
