use abstract_sdk::{
    base::{Handler, InstantiateEndpoint},
    feature_objects::{AnsHost, VersionControlContract},
};
use abstract_std::{
    adapter::{AdapterState, InstantiateMsg},
    objects::module_version::set_module_data,
};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;
use schemars::JsonSchema;
use serde::Serialize;

use crate::state::{AdapterContract, ContractError};

impl<
        Error: ContractError,
        CustomInitMsg: Serialize + JsonSchema,
        CustomExecMsg,
        CustomQueryMsg,
        SudoMsg,
    > InstantiateEndpoint
    for AdapterContract<Error, CustomInitMsg, CustomExecMsg, CustomQueryMsg, SudoMsg>
{
    type InstantiateMsg = InstantiateMsg<CustomInitMsg>;
    /// Instantiate the api
    fn instantiate(
        self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: Self::InstantiateMsg,
    ) -> Result<Response, Error> {
        let ans_host = AnsHost {
            address: deps.api.addr_validate(&msg.base.ans_host_address)?,
        };

        let version_control = VersionControlContract {
            address: deps.api.addr_validate(&msg.base.version_control_address)?,
        };

        // Base state
        let state = AdapterState {
            version_control,
            ans_host,
        };
        let (name, version, metadata) = self.info();
        set_module_data(deps.storage, name, version, self.dependencies(), metadata)?;
        set_contract_version(deps.storage, name, version)?;
        self.base_state.save(deps.storage, &state)?;

        let Some(handler) = self.maybe_instantiate_handler() else {
            return Ok(Response::new());
        };
        handler(deps, env, info, self, msg.module)
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::needless_borrows_for_generic_args)]
    use abstract_sdk::{
        base::InstantiateEndpoint,
        feature_objects::{AnsHost, VersionControlContract},
    };
    use abstract_std::{
        adapter::{AdapterState, BaseInstantiateMsg, InstantiateMsg},
        objects::module_version::{ModuleData, MODULE},
    };
    use abstract_testing::prelude::*;
    use cosmwasm_std::{testing::*, StdError};
    use cw2::{ContractVersion, CONTRACT};
    use speculoos::prelude::*;

    use crate::mock::{AdapterMockResult, MockInitMsg, MOCK_ADAPTER, MOCK_DEP, TEST_METADATA};

    #[test]
    fn successful() -> AdapterMockResult {
        let api = MOCK_ADAPTER.with_dependencies(&[MOCK_DEP]);
        let env = mock_env();
        let mut deps = mock_dependencies();
        let abstr = AbstractMockAddrs::new(deps.api);

        let info = message_info(&abstr.account.manager, &[]);
        deps.querier = abstract_testing::mock_querier(deps.api);
        let init_msg = InstantiateMsg {
            base: BaseInstantiateMsg {
                ans_host_address: abstr.ans_host.to_string(),
                version_control_address: abstr.version_control.to_string(),
            },
            module: MockInitMsg {},
        };
        let res = api.instantiate(deps.as_mut(), env, info, init_msg)?;
        assert_that!(&res.messages.len()).is_equal_to(0);
        // confirm mock init handler executed
        assert_that!(&res.data).is_equal_to(Some("mock_init".as_bytes().into()));

        let module_data = MODULE.load(&deps.storage)?;
        assert_that!(module_data).is_equal_to(ModuleData {
            module: TEST_MODULE_ID.into(),
            version: TEST_VERSION.into(),
            dependencies: vec![(&crate::mock::MOCK_DEP).into()],
            metadata: Some(TEST_METADATA.into()),
        });

        let contract_version = CONTRACT.load(&deps.storage)?;
        assert_that!(contract_version).is_equal_to(ContractVersion {
            contract: TEST_MODULE_ID.into(),
            version: TEST_VERSION.into(),
        });

        let api = MOCK_ADAPTER;
        let none_authorized = api.authorized_addresses.is_empty(&deps.storage);
        assert!(none_authorized);

        let state = api.base_state.load(&deps.storage)?;
        assert_that!(state).is_equal_to(AdapterState {
            version_control: VersionControlContract {
                address: abstr.version_control,
            },
            ans_host: AnsHost {
                address: abstr.ans_host,
            },
        });
        Ok(())
    }
}
