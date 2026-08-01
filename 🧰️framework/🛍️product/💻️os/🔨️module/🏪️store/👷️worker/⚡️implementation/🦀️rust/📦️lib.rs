//! 🧵️ WASM backbone worker — browser-side `DocumentHost` actor relaying the same protocol as
//! `framework/product/os/core/js/🟦️backbone-worker.ts`, without materializing projections.

use store_sync::{
    DocumentActorConfig, DocumentActorMsg, DocumentEvent, DocumentHost, PersistenceBinding,
};
use serde::Deserialize;
use wasm_bindgen::prelude::*;

//#region 🔖️Protocol
#[derive(Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
enum WorkerRequest {
    Open {
        document_id: String,
        schema: String,
        bindings: Vec<PersistenceBinding>,
        watch_external: Option<bool>,
        actor: String,
    },
    Close { document_id: String },
    Send {
        document_id: String,
        message: DocumentActorMsg,
    }
}

#[derive(Debug, serde::Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
enum WorkerResponse {
    Event { document_id: String, event: DocumentEvent },
    Ready,
}
//#endregion 🔖️Protocol

//#region 🔖️Worker
struct DocumentEntry {
    cmd_tx: tokio::sync::mpsc::UnboundedSender<DocumentActorMsg>,
}

#[wasm_bindgen]
pub struct BackboneWorkerHost {
    host: DocumentHost,
    documents: std::collections::HashMap<String, DocumentEntry>,
}

#[wasm_bindgen]
impl BackboneWorkerHost {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            host: DocumentHost::new(),
            documents: std::collections::HashMap::new(),
        }
    }

    #[wasm_bindgen(js_name = handleRequestBytes)]
    pub fn handle_request_bytes(&mut self, bytes: &[u8]) -> Result<(), JsValue> {
        let json = std::str::from_utf8(bytes).map_err(|error| JsValue::from_str(&error.to_string()))?;
        self.handle_request_json(json)
    }

    #[wasm_bindgen(js_name = handleRequestJson)]
    pub fn handle_request_json(&mut self, json: &str) -> Result<(), JsValue> {
        let request: WorkerRequest =
            serde_json::from_str(json).map_err(|error| JsValue::from_str(&error.to_string()))?;
        match request {
            WorkerRequest::Open {
                document_id,
                schema,
                bindings,
                watch_external,
                actor,
            } => {
                self.host.close(&document_id);
                let channels = self.host.open(DocumentActorConfig {
                    document_id: document_id.clone(),
                    schema,
                    bindings,
                    watch_external: watch_external.unwrap_or(true),
                    actor,
                });
                let mut events = self.host.subscribe(&document_id);
                let cmd_tx = channels.cmd_tx.clone();
                self.documents.insert(document_id.clone(), DocumentEntry { cmd_tx });
                wasm_bindgen_futures::spawn_local(async move {
                    loop {
                        match events.recv().await {
                            Ok(event) => {
                                let response = WorkerResponse::Event {
                                    document_id: document_id.clone(),
                                    event,
                                };
                                if let Ok(json) = serde_json::to_string(&response) {
                                    post_worker_message_bytes(json.as_bytes());
                                }
                            }
                            Err(_) => break,
                        }
                    }
                });
            }
            WorkerRequest::Close { document_id } => {
                self.host.send(&document_id, DocumentActorMsg::Detach);
                self.host.close(&document_id);
                self.documents.remove(&document_id);
            }
            WorkerRequest::Send { document_id, message } => {
                if let Some(entry) = self.documents.get(&document_id) {
                    let _ = entry.cmd_tx.send(message);
                }
            }
        }
        Ok(())
    }

    #[wasm_bindgen(js_name = postReady)]
    pub fn post_ready() {
        if let Ok(json) = serde_json::to_string(&WorkerResponse::Ready) {
            post_worker_message_bytes(json.as_bytes());
        }
    }
}

fn post_worker_message_bytes(bytes: &[u8]) {
    let global = js_sys::global();
    if let Ok(post_message) = js_sys::Reflect::get(&global, &JsValue::from_str("postMessage")) {
        if let Some(function) = post_message.dyn_ref::<js_sys::Function>() {
            let wire = js_sys::Object::new();
            let _ = js_sys::Reflect::set(&wire, &JsValue::from_str("wire"), &js_sys::Uint8Array::from(bytes));
            let _ = function.call1(&global, &wire);
        }
    }
}

#[allow(dead_code)]
fn post_worker_message(json: &str) {
    post_worker_message_bytes(json.as_bytes());
}
//#endregion 🔖️Worker
