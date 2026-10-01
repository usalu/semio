//! 🖥 Native application with its explicit installed-service inventory.
fn main() {
    semio_framework_os_renderer_wgpu::run_native_entrypoint(semio_s_dev_services::service_contributions_v1());
}
