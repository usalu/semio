/// ⌨️ Native renderer executable with an empty document-service inventory.

#[cfg(target_arch = "wasm32")]
fn main() {}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    semio_renderer_wgpu::run_native_entrypoint(Vec::new(),semio_os_kernel::native_catalog::configured_native_catalog_v1().expect("Native catalog refused"));
}
