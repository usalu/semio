//! 🧩 Native and MCP hosts receive their defining caller's installed inventory.
pub use semio_framework_os_kernel::os_directory::client::InstalledServiceContributionV1;

/// 🖥 Runs the native host with exactly the supplied installed services.
#[cfg(feature = "native-renderer")]
pub fn run_native_v1(services: Vec<InstalledServiceContributionV1>) {
    semio_framework_os_renderer_wgpu::run_native_entrypoint(services);
}

/// 🌉 Runs the MCP host with exactly the supplied service and credential protocols.
#[cfg(feature = "mcp-service")]
pub fn run_mcp_v1(services: Vec<semio_framework_os_mcp::inference::RemoteInferenceProtocolV1>, credentials: Vec<semio_framework_os_mcp::agent_credential::CredentialExchangeProtocolV1>) {
    semio_framework_os_mcp::run_mcp_entrypoint(services, credentials);
}
