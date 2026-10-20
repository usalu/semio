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
        pub fn next_close_copy_byte_demand(&self)->Result<usize,JsValue> {self.session.next_close_copy_byte_demand().map_err(|error|JsValue::from_str(&error.to_string()))}
        pub fn next_close_capacity_byte_demand(&self,copy:usize)->Result<usize,JsValue> {self.session.next_close_capacity_byte_demand(copy).map_err(|error|JsValue::from_str(&error.to_string()))}
        pub fn next_close_release_byte_demand(&self)->Result<usize,JsValue> {self.session.next_close_release_byte_demand().map_err(|error|JsValue::from_str(&error.to_string()))}
        pub fn next_close_depth_demand(&self)->Result<usize,JsValue> {self.session.next_close_depth_demand().map_err(|error|JsValue::from_str(&error.to_string()))}
        pub fn close_step(&self,maximum_items:usize,maximum_copy_bytes:usize,maximum_capacity_bytes:usize,maximum_release_bytes:usize,maximum_depth:usize)->String {
            match self.session.close_step(semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items,maximum_copy_bytes,maximum_capacity_bytes,maximum_release_bytes,maximum_depth}) {
                Ok(step)=>{let progress=step.progress();let phase=if matches!(step,semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)) {"complete"}else {"pending"};format!("{{\"phase\":\"{phase}\",\"items\":{},\"copyBytes\":{},\"capacityBytes\":{},\"releaseBytes\":{}}}",progress.copied_items,progress.copied_bytes,progress.retained_capacity_bytes,progress.released_bytes)},
                Err(error)=>semio_framework_pack_json::to_string(&semio_framework_pack_json::object([("error".into(),semio_framework_pack_json::Value::String(error.to_string()))])),
            }
        }
    }
}
