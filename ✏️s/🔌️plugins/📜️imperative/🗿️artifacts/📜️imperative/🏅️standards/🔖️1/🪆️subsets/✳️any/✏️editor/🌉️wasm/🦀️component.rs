//! 🌐️ Imperative play app — browser-facing session wrapper, the ONLY wasm-bindgen surface for this app;
//! kept in its own component (not the artifact `⚙️engine`) so the engine stays pure headless compute.

#[cfg(target_arch = "wasm32")]
mod wasm_session {
    use crate::artifacts::imperative::PathRef;
    use crate::editor::imperative::engine::{ImperativeCoreError, ImperativeHost};
    use std::cell::RefCell;
    use std::rc::Rc;
    use wasm_bindgen::prelude::*;

    struct ImperativeSessionInner {
        host: ImperativeHost,
    }

    #[wasm_bindgen]
    pub struct ImperativeSession {
        state: Rc<RefCell<ImperativeSessionInner>>,
    }

    #[wasm_bindgen]
    impl ImperativeSession {
        #[wasm_bindgen(constructor)]
        pub async fn new() -> Self {
            Self { state: Rc::new(RefCell::new(ImperativeSessionInner { host: ImperativeHost::default() })) }
        }

        #[wasm_bindgen(js_name = loadPathJson)]
        pub async fn load_path_json(&self, json: &str) -> Result<(), JsValue> {
            let host = ImperativeHost::load_json(json).map_err(|err: ImperativeCoreError| JsValue::from_str(&err.to_string()))?;
            self.state.borrow_mut().host = host;
            Ok(())
        }

        #[wasm_bindgen(js_name = pathJson)]
        pub async fn path_json(&self) -> Result<String, JsValue> {
            self.state.borrow().host.to_json().map_err(|err| JsValue::from_str(&err.to_string()))
        }

        #[wasm_bindgen(js_name = catalogueJson)]
        pub async fn catalogue_json(&self) -> String {
            self.state.borrow().host.catalogue_json()
        }

        #[wasm_bindgen(js_name = addStep)]
        pub async fn add_step(&self, kind: &str, index: Option<usize>) -> String {
            self.state.borrow_mut().host.add_step(kind, index)
        }

        #[wasm_bindgen(js_name = addStepAt)]
        pub async fn add_step_at(&self, path_ref_json: &str, kind: &str, index: Option<usize>) -> Result<String, JsValue> {
            let path_ref: PathRef = serde_json::from_str(path_ref_json).map_err(|err| JsValue::from_str(&err.to_string()))?;
            self.state.borrow_mut().host.add_step_at(&path_ref, kind, index).map_err(|err| JsValue::from_str(&err.to_string()))
        }

        #[wasm_bindgen(js_name = removeStep)]
        pub async fn remove_step(&self, id: &str) -> bool {
            self.state.borrow_mut().host.remove_step(id)
        }

        #[wasm_bindgen(js_name = removeStepAt)]
        pub async fn remove_step_at(&self, path_ref_json: &str, id: &str) -> Result<bool, JsValue> {
            let path_ref: PathRef = serde_json::from_str(path_ref_json).map_err(|err| JsValue::from_str(&err.to_string()))?;
            Ok(self.state.borrow_mut().host.remove_step_at(&path_ref, id))
        }

        #[wasm_bindgen(js_name = moveStep)]
        pub async fn move_step(&self, id: &str, new_index: usize) -> bool {
            self.state.borrow_mut().host.move_step(id, new_index)
        }

        #[wasm_bindgen(js_name = moveStepAt)]
        pub async fn move_step_at(&self, path_ref_json: &str, id: &str, new_index: usize) -> Result<bool, JsValue> {
            let path_ref: PathRef = serde_json::from_str(path_ref_json).map_err(|err| JsValue::from_str(&err.to_string()))?;
            Ok(self.state.borrow_mut().host.move_step_at(&path_ref, id, new_index))
        }

        #[wasm_bindgen(js_name = setStepParamsJson)]
        pub async fn set_step_params_json(&self, id: &str, json: &str) -> Result<(), JsValue> {
            self.state.borrow_mut().host.set_step_params_json(id, json).map_err(|err| JsValue::from_str(&err.to_string()))
        }

        #[wasm_bindgen(js_name = setStepParamsAt)]
        pub async fn set_step_params_at(&self, path_ref_json: &str, id: &str, json: &str) -> Result<(), JsValue> {
            let path_ref: PathRef = serde_json::from_str(path_ref_json).map_err(|err| JsValue::from_str(&err.to_string()))?;
            self.state.borrow_mut().host.set_step_params_at(&path_ref, id, json).map_err(|err| JsValue::from_str(&err.to_string()))
        }

        #[wasm_bindgen]
        pub async fn run(&self) -> Result<String, JsValue> {
            let result = self.state.borrow().host.run();
            serde_json::to_string(&result).map_err(|err| JsValue::from_str(&err.to_string()))
        }

        #[wasm_bindgen(js_name = compileText)]
        pub async fn compile_text(&self) -> String {
            self.state.borrow().host.compile_text()
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm_session::ImperativeSession;
