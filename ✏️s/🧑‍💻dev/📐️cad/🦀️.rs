//! 📐️ Outward CAD browser assembly with explicitly supplied STEP geometry operations.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
mod browser {
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen]
    pub struct BrowserSession { session: semio_s_spatial_kernel_semio_session::Session }
    #[wasm_bindgen]
    impl BrowserSession {
        #[wasm_bindgen(constructor)]
        pub fn new() -> Self { Self { session: semio_s_artifact_stdio_step::geometry::session::geometry_session() } }
        pub fn brep_invoke(&self, method: &str, arguments: &str) -> String { self.session.brep_invoke_json(method, arguments) }
        pub fn tessellate(&self, handle: &str, tolerance: f64) -> String { self.session.tessellate_geometry_json_for_wasm(handle, tolerance) }
        pub fn dispose(&self, handle: &str) -> Result<(), JsValue> { self.session.dispose_geometry(handle).map_err(|error| JsValue::from_str(&error)) }
        pub fn begin_close(&self) { self.session.begin_close(); }
        pub fn cancel_close(&self) { self.session.cancel_close(); }
        pub fn resume_close(&self) { self.session.resume_close(); }
        pub fn terminal_is_empty(&self) -> bool { self.session.terminal_is_empty() }
        pub fn close_step(&self, maximum_items: usize, maximum_bytes: usize) -> String {
            match self.session.close_step(maximum_items,maximum_bytes) {
                Ok(neural_engine::ValueRetirementStep::Blocked) => "{\"phase\":\"blocked\",\"items\":0,\"bytes\":0}".into(),
                Ok(neural_engine::ValueRetirementStep::Complete) => "{\"phase\":\"complete\",\"items\":0,\"bytes\":0}".into(),
                Ok(neural_engine::ValueRetirementStep::Pending { released_items,released_bytes }) => format!("{{\"phase\":\"pending\",\"items\":{released_items},\"bytes\":{released_bytes}}}"),
                Err(error) => semio_framework_os_flow::os_pack::json::to_string(&semio_framework_os_flow::os_pack::json::object([("error".into(),semio_framework_os_flow::os_pack::json::Value::String(error))])),
            }
        }
    }
}
