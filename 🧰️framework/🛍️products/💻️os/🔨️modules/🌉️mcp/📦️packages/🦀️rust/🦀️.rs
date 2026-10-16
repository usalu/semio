//! 🌉️ Gateway runtime, entrypoint and facade.

extern crate semio_framework_os_kernel as store;

#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATION_OBSERVER: semio_framework_os_mcp_foundation::test_allocation::RequestedAllocator = semio_framework_os_mcp_foundation::test_allocation::RequestedAllocator;

#[path = "../../🏗️bootstrap/🦀️.rs"]
pub mod entrypoint;

#[cfg(test)]
#[path = "../../🧪️tests/🔬️bin-quick/🦀️.rs"]
mod entrypoint_quick;

#[cfg(test)]
#[path = "../../💡️inference/🧪️tests/🔬️inference-jobs/🦀️.rs"]
mod inference_inference_jobs;

#[cfg(test)]
#[path = "../../📣️notify/🧪️tests/🔬️quick/🦀️.rs"]
mod notify_quick;

#[path = "../../🦀️.rs"]
mod root;

#[cfg(test)]
#[path = "../../🧪️tests/🔬️quick/🦀️.rs"]
mod root_quick;

#[cfg(test)]
#[path = "../../🏠️workspace/🧪️tests/🔬️quick/🦀️.rs"]
mod workspace_quick;

pub mod agent_credential {
    pub use semio_framework_os_mcp_foundation::agent_credential::{CredentialExchangeProtocolV1, CredentialExchangeRequestV1, DelegatedCredentialV1, DelegatedSessionGrantV1, read_delegated_credential_file_v1};
}
pub mod catalog {
    pub use semio_framework_os_mcp_catalog::catalog::{CatalogSource, compile};
}
pub mod protocol {
    pub use semio_framework_os_mcp_wire::protocol::{InMemoryToolRegistry, ToolRegistry};
}
pub mod inference {
    pub use semio_framework_os_mcp_inference_hub::hub_wire::{HubInferenceApprovalRequestV1, HubInferenceEventPageV1, HubInferenceSubmitRequestV1, InferenceHubMethodV1, InferenceHubRequestV1, InferenceHubResponseV1, InferenceHubTransport, InferenceHubTransportErrorV1, InferenceRouteErrorV1, approve_hub_inference_job, cancel_hub_inference_job, read_hub_inference_job_events, submit_hub_inference_job, undo_hub_inference_approval};
    pub use semio_framework_os_mcp_inference_hub::service::{RemoteInferenceProtocolV1, install_remote_inference_protocols_v1};
}
pub mod workspace {
    pub use semio_framework_os_mcp_workspace::headless_workspace_state::{HeadlessWorkspace};
    pub use semio_framework_os_mcp_catalog::plugin_paths_and_descriptor::{load_package_descriptor};
    pub mod remote {
        pub use semio_framework_os_mcp_hub_remote::remote::{exchange_delegated_session_v1};
    }
}
pub use semio_framework_os_mcp_foundation::errors::{GatewayError, GatewayErrorCode};
pub use semio_framework_os_mcp_workspace::headless_workspace_state::{HeadlessWorkspace};
pub use crate::entrypoint::run_mcp_entrypoint;
