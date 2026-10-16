//! 🖼️ Per-surface WebGPU presentation of the browser Flow session.

use super::*;

//#region 🖼️SurfacePresentation

/// 🖥️ Per-surface WebGPU presentation for the browser flow session.
///
/// The flow session speaks a linear-memory byte ABI (`flow_bridge_send`/`flow_bridge_poll`), which
/// cannot carry an `HtmlCanvasElement` — that is the whole reason the node-graph canvas had no GPU
/// exit and the host drew its own boxes instead. These two exports bind the canvas out of band,
/// keyed by the SAME `surface` id the ABI's `attachSurface` already issues, so no session state is
/// duplicated and nothing about the message protocol changes. The session itself is
/// `CanvasGpuSession` — the one browser vello/wgpu presenter every other board surface in this
/// repo uses (`DagSession`, `BoardSession`, paint/map/editor), reused, not forked.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub(super) mod surface_canvas {
    use semio_framework_canvas::{self as canvas, gpu_session::CanvasGpuSession, Color, Scene};
    use semio_framework_async::browser::future_to_promise;
    use std::cell::{Cell,RefCell};
    use std::collections::BTreeMap;
    use wasm_bindgen::prelude::*;
    use web_sys::HtmlCanvasElement;

    thread_local! {
        static SURFACE_CANVASES: RefCell<BTreeMap<u32, CanvasGpuSession>> = const { RefCell::new(BTreeMap::new()) };
    }

    fn physical(logical: u32, dpr: f64) -> u32 {
        ((f64::from(logical.max(1)) * dpr.max(1.0)).round() as u32).max(1)
    }

    /// 🔌️ Binds one flow surface's canvas to a WebGPU device. Resolves `true` when the canvas will
    /// present on the GPU and `false` when no adapter/device/surface could be had — never rejects,
    /// because a host without WebGPU must still reach a painted canvas through the 2D replay path.
    #[wasm_bindgen(js_name = flowAttachSurfaceCanvas)]
    pub fn flow_attach_surface_canvas(surface: u32, canvas: HtmlCanvasElement, logical_width: u32, logical_height: u32, dpr: f64) -> js_sys::Promise {
        let width = physical(logical_width, dpr);
        let height = physical(logical_height, dpr);
        future_to_promise(async move {
            match CanvasGpuSession::create_canvas_surface(canvas.clone(), width, height).await {
                Ok(admission) => {
                    SURFACE_CANVASES.with(|map| map.borrow_mut().entry(surface).or_default().finish_attach(admission));
                    Ok(JsValue::TRUE)
                }
                Err(_) => Ok(JsValue::FALSE),
            }
        })
    }

    /// 📐️ Resizes an attached surface's swapchain; a no-op for a surface that presents in 2D.
    #[wasm_bindgen(js_name = flowResizeSurfaceCanvas)]
    pub fn flow_resize_surface_canvas(surface: u32, logical_width: u32, logical_height: u32, dpr: f64) {
        let width = physical(logical_width, dpr);
        let height = physical(logical_height, dpr);
        SURFACE_CANVASES.with(|map| {
            if let Some(session) = map.borrow_mut().get_mut(&surface) {
                session.resize_surface(width, height);
            }
        });
    }

    /// 🔎️ Whether this surface presents on the GPU — the host reads it to decide whether to keep a
    /// 2D context off the canvas (a canvas admits exactly one context kind, for its whole life).
    #[wasm_bindgen(js_name = flowSurfaceCanvasPresentsOnGpu)]
    pub fn flow_surface_canvas_presents_on_gpu(surface: u32) -> bool {
        SURFACE_CANVASES.with(|map| map.borrow().get(&surface).is_some_and(CanvasGpuSession::gpu_ready))
    }

    /// 🧹️ Releases a surface's device, swapchain and canvas retention.
    #[wasm_bindgen(js_name = flowDetachSurfaceCanvas)]
    pub fn flow_detach_surface_canvas(surface: u32) {
        SURFACE_CANVASES.with(|map| {
            if let Some(mut session) = map.borrow_mut().remove(&surface) {
                session.detach();
            }
        });
    }

    /// 🎬️ Rasterizes one painted scene onto the surface's canvas. `false` means nothing presented,
    /// which is the host's signal to replay the encoded draw list into a 2D context instead.
    pub fn present_surface_scene(surface: u32, scene: &Scene, clear: Color, dpr: f64) -> bool {
        SURFACE_CANVASES.with(|map| {
            let mut map = map.borrow_mut();
            let Some(session) = map.get_mut(&surface) else { return false };
            if !session.gpu_ready() {
                return false;
            }
            if (dpr.max(1.0) - 1.0).abs() < f64::EPSILON {
                return session.render_frame(scene, clear).is_ok();
            }
            let scaled = canvas::render::scale_scene_for_device_pixel_ratio(scene.clone(), dpr);
            session.render_frame(&scaled, clear).is_ok()
        })
    }
}

/// 🚫️ Non-browser arm: there is no `HtmlCanvasElement` to present onto (native runs this component
/// only under `#[cfg(test)]`, and the WASI guest target does not compile this file at all), so every
/// frame takes the encoded-draw-list exit.
#[cfg(not(all(target_arch = "wasm32", not(target_env = "p2"))))]
pub(super) mod surface_canvas {
    use semio_framework_canvas::{Color, Scene};

    pub fn present_surface_scene(_surface: u32, _scene: &Scene, _clear: Color, _dpr: f64) -> bool {
        false
    }

    pub fn flow_resize_surface_canvas(_surface: u32, _logical_width: u32, _logical_height: u32, _dpr: f64) {}

    pub fn flow_detach_surface_canvas(_surface: u32) {}
}

//#endregion 🖼️SurfacePresentation
