//! 🌐️ Browser-only DAG canvas session bridging `DagHost` to a wgpu canvas.

// #region 🔖️WasmSession
use super::*;
use semio_framework_async::browser::future_to_promise;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

struct DagSessionInner {
    host: DagHost,
    gpu: canvas::gpu_session::CanvasGpuSession,
    width: u32,
    height: u32,
    dpr: f64,
}

#[wasm_bindgen]
pub struct DagSession {
    state: Rc<RefCell<DagSessionInner>>,
}

#[wasm_bindgen]
impl DagSession {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { state: Rc::new(RefCell::new(DagSessionInner { host: DagHost::default_demo(), gpu: canvas::gpu_session::CanvasGpuSession::default(), width: 1, height: 1, dpr: 1.0 })) }
    }

    #[wasm_bindgen(js_name = loadHostSnapshotJson)]
    pub fn load_host_snapshot_json(&self, json: &str) -> Result<(), JsValue> {
        let host = DagHost::load_host_snapshot_json(json).map_err(|e| JsValue::from_str(&e.to_string()))?;
        self.state.borrow_mut().host = host;
        Ok(())
    }

    #[wasm_bindgen(js_name = hostSnapshotJson)]
    pub fn host_snapshot_json(&self) -> Result<String, JsValue> {
        self.state.borrow().host.host_snapshot_json().map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = nodeOverlaysJson)]
    pub fn node_overlays_json(&self) -> Result<String, JsValue> {
        self.state.borrow().host.node_overlays_json().map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = labelOverlayPaintStateJson)]
    pub fn label_overlay_paint_state_json(&self) -> Result<String, JsValue> {
        self.state.borrow().host.label_overlay_paint_state_json().map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = attachCanvas)]
    pub fn attach_canvas(&mut self, canvas: HtmlCanvasElement, logical_w: u32, logical_h: u32, dpr: f64) -> js_sys::Promise {
        let inner = self.state.clone();
        let lw = logical_w.max(1);
        let lh = logical_h.max(1);
        let dpr = dpr.max(1.0);
        let pw = ((lw as f64 * dpr).round() as u32).max(1);
        let ph = ((lh as f64 * dpr).round() as u32).max(1);
        future_to_promise(async move {
            let admission = canvas::gpu_session::CanvasGpuSession::create_canvas_surface(canvas.clone(), pw, ph).await.map_err(|err| JsValue::from_str(&err))?;
            let mut g = inner.borrow_mut();
            g.width = lw;
            g.height = lh;
            g.dpr = dpr;
            g.host.set_viewport(lw, lh, dpr);
            g.gpu.finish_attach(admission);
            Ok(JsValue::UNDEFINED)
        })
    }

    #[wasm_bindgen(js_name = gpuReady)]
    pub fn gpu_ready(&self) -> bool {
        self.state.borrow().gpu.gpu_ready()
    }

    #[wasm_bindgen(js_name = setSize)]
    pub fn set_size(&mut self, width: u32, height: u32, dpr: f64) {
        let mut inner = self.state.borrow_mut();
        inner.width = width.max(1);
        inner.height = height.max(1);
        inner.dpr = dpr.max(1.0);
        let (w, h, d) = (inner.width, inner.height, inner.dpr);
        inner.host.set_viewport(w, h, d);
        let pw = ((w as f64 * d).round() as u32).max(1);
        let ph = ((h as f64 * d).round() as u32).max(1);
        inner.gpu.resize_surface(pw, ph);
    }

    #[wasm_bindgen(js_name = setCamera)]
    pub fn set_camera(&self, x: f64, y: f64, zoom: f64) {
        self.state.borrow_mut().host.set_camera(x, y, zoom);
    }

    #[wasm_bindgen(js_name = setWheelZoomActive)]
    pub fn set_wheel_zoom_active(&self, active: bool) {
        self.state.borrow_mut().host.set_wheel_zoom_active(active);
    }

    #[wasm_bindgen(js_name = lodScaleJson)]
    pub fn lod_scale_json(&self) -> String {
        dag_lod_scale_json()
    }

    #[wasm_bindgen(js_name = setAutomaticLod)]
    pub fn set_automatic_lod(&self, enabled: bool) {
        self.state.borrow_mut().host.set_automatic_lod(enabled);
    }

    #[wasm_bindgen(js_name = setForcedDrawLodLabel)]
    pub fn set_forced_draw_lod_label(&self, label: &str) {
        self.state.borrow_mut().host.set_forced_draw_lod_label(label);
    }

    #[wasm_bindgen(js_name = drawLodLabel)]
    pub fn draw_lod_label(&self) -> String {
        self.state.borrow().host.draw_lod_label().to_string()
    }

    #[wasm_bindgen(js_name = pointerDown)]
    pub fn pointer_down(&self, x: f64, y: f64, extend: bool) {
        self.state.borrow_mut().host.pointer_down(x, y, extend);
    }

    #[wasm_bindgen(js_name = pointerMove)]
    pub fn pointer_move(&self, x: f64, y: f64) {
        self.state.borrow_mut().host.pointer_move(x, y);
    }

    #[wasm_bindgen(js_name = pointerUp)]
    pub fn pointer_up(&self, x: f64, y: f64) {
        self.state.borrow_mut().host.pointer_up(x, y);
    }

    #[wasm_bindgen(js_name = takePendingOpenInstanceId)]
    pub fn take_pending_open_instance_id(&self) -> Option<String> {
        dag_take_pending_open_instance_id(&mut self.state.borrow_mut().host)
    }

    #[wasm_bindgen(js_name = screenToWorld)]
    pub fn screen_to_world(&self, x: f64, y: f64) -> js_sys::Array {
        let (wx, wy) = dag_screen_to_world(&self.state.borrow().host, x, y);
        let out = js_sys::Array::new();
        out.push(&JsValue::from_f64(wx));
        out.push(&JsValue::from_f64(wy));
        out
    }

    #[wasm_bindgen(js_name = reorganize)]
    pub fn reorganize(&self,options_json:&str,maximum_bytes:u32,maximum_work:u32,progress:js_sys::Function)->Result<(),JsValue>{
        let report=|phase:&str,completed:u64|progress.call2(&JsValue::NULL,&JsValue::from_str(phase),&JsValue::from_f64(completed as f64)).map(|v|v.as_bool().unwrap_or(false)).unwrap_or(false);
        let mut decoding=|p:semio_framework_value::native_decoding::NativeDecodeProgress|report("decode",p.completed as u64);let mut decode=semio_framework_value::NativeDecodeControl::new(maximum_bytes as usize,&mut decoding);
        let opts=crate::infinite::board::io::text::layout::decode_dag_options_json(options_json,&mut decode).map_err(|e|JsValue::from_str(&e.to_string()))?;
        let mut working=|p:crate::infinite::board::schema::layout::LayoutProgress|report("layout",p.completed);let mut work=LayoutControl::new(maximum_work as u64,&mut working);
        self.state.borrow_mut().host.reorganize(&opts,&mut work).map_err(|e|JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = setCanvasThemeJson)]
    pub fn set_canvas_theme_json(&mut self, json: &str) {
        let mut accepted=|_|true;
        let mut control=semio_framework_value::NativeDecodeControl::new(64*1024,&mut accepted);
        if let Ok(overlay)=crate::infinite::board::io::text::palette::decode_board_palette_overlay_json(json,&mut control){self.state.borrow_mut().host.set_canvas_palette(&overlay);}
    }

    #[wasm_bindgen(js_name = renderFrame)]
    pub fn render_frame(&self) -> Result<(), JsValue> {
        let mut inner = self.state.borrow_mut();
        let mut scene = canvas::Scene::new();
        let clear = inner.host.canvas_theme.raster_clear;
        inner.host.paint_scene(&mut scene, inner.width, inner.height, inner.dpr);
        let scene = canvas::render::scale_scene_for_device_pixel_ratio(scene, inner.dpr);
        inner.gpu.render_frame(&scene, clear)
    }
}
// #endregion 🔖️WasmSession
