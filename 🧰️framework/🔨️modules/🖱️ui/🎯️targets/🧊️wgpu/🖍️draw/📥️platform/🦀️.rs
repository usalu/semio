//! 🪟️ Genuine platform carrier capture; backend GPU objects never cross this boundary.
#[cfg(all(not(target_arch="wasm32"),not(target_os="wasi")))]
mod native {
 use std::sync::Arc;
 use crate::GpuContext;
 pub use semio_framework_ui_wgpu_device::platform::NativeWindow;
 pub struct NativeSurfaceCapture {capture:semio_framework_ui_wgpu_device::platform::NativeSurfaceCapture}
 impl NativeSurfaceCapture {
  pub fn capture(window:Arc<NativeWindow>)->Result<Self,String>{Ok(Self{capture:semio_framework_ui_wgpu_device::platform::NativeSurfaceCapture::capture(window)?})}
  pub fn prepare(self)->impl std::future::Future<Output=Result<GpuContext,String>>+Send{async move{Ok(GpuContext::from_prepared_device(self.capture.prepare().await?))}}
 }
}
#[cfg(all(not(target_arch="wasm32"),not(target_os="wasi")))]
pub use native::NativeSurfaceCapture;
#[cfg(all(target_arch="wasm32",not(target_env="p2")))]
mod browser {
 use crate::GpuContext;
 pub use semio_framework_ui_wgpu_device::platform::TransferredCanvas;
 pub struct BrowserSurfaceCapture{canvas:TransferredCanvas,width:f32,height:f32,dpr:f32}
 impl BrowserSurfaceCapture{
  pub fn capture(canvas:TransferredCanvas,width:f32,height:f32,dpr:f32)->Self{Self{canvas,width,height,dpr}}
  pub async fn prepare(self)->Result<GpuContext,String>{Ok(GpuContext::from_prepared_device(semio_framework_ui_wgpu_device::platform::prepare_canvas(self.canvas,self.width,self.height,self.dpr).await?))}
 }
}
#[cfg(all(target_arch="wasm32",not(target_env="p2")))]
pub use browser::{BrowserSurfaceCapture,TransferredCanvas};
#[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
pub fn schedule_frame(window: &NativeWindow, callback: impl FnMut() + 'static) {
    use wasm_bindgen::closure::Closure;
    use wasm_bindgen::JsCast;

    let mut callback = callback;
    let closure = Closure::wrap(Box::new(move || {
        callback();
    }) as Box<dyn FnMut()>);
    web_sys::window().and_then(|w| w.request_animation_frame(closure.as_ref().unchecked_ref()).ok());
    closure.forget();
    let _ = window;
}

#[cfg(all(not(target_arch = "wasm32"), not(target_os = "wasi")))]
pub fn schedule_frame(window: &NativeWindow, _callback: impl FnMut() + 'static) {
    window.request_redraw();
}

#[cfg(not(target_os="wasi"))]
pub use semio_framework_ui_wgpu_device::platform::NativeWindow;
