//! 🧩️ The browser module of `semio-s-artifact-trinity-jack-lsp`: links the library and owns its `JackLspSession` wasm-bindgen export.

extern crate semio_s_artifact_trinity_jack_lsp as _;

/// 🌐️ `JackLspSession` over one Jack language server session. Browser-only: `target_arch = "wasm32"` is also true for `wasm32-wasip2`.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
mod browser {
    use semio_s_artifact_trinity_jack_lsp::JackLanguageSession;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct JackLspSession {
        session: JackLanguageSession,
    }

    #[wasm_bindgen]
    impl JackLspSession {
        #[wasm_bindgen(constructor)]
        pub fn new() -> JackLspSession {
            JackLspSession { session: JackLanguageSession::new() }
        }

        #[wasm_bindgen(js_name = loadSnapshotJson)]
        pub fn load_snapshot_json(&mut self, json: &str) -> Result<(), JsValue> {
            self.session.load_snapshot(json).map_err(|message| JsValue::from_str(&message))
        }

        #[wasm_bindgen(js_name = handleMessageJson)]
        pub fn handle_message_json(&mut self, message: &str) -> String {
            self.session.handle_message_json(message)
        }

        #[wasm_bindgen(js_name = hasExited)]
        pub fn has_exited(&self) -> bool {
            self.session.has_exited()
        }
    }

    impl Default for JackLspSession {
        fn default() -> Self {
            Self::new()
        }
    }
}
