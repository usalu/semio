//! 🧵️ WASM backbone worker — browser-side `ArtifactHost` actor relaying the same protocol as
//! `framework/product/os/core/js/🟦️backbone-worker.ts`, without materializing snapshots.

use crate::os_store::sync::{
    backbone_worker_wire::{self, BackboneWorkerRequest, BackboneWorkerResponse},
    ArtifactActorMsg, ArtifactHost,
};
use wasm_bindgen::prelude::*;

//#region 🔖️Worker
struct DocumentEntry {
    cmd_tx: tokio::sync::mpsc::UnboundedSender<ArtifactActorMsg>,
}

#[wasm_bindgen]
pub struct BackboneWorkerHost {
    host: ArtifactHost,
    pool: std::sync::Arc<semio_framework_async::WorkerPool>,
    documents: std::collections::HashMap<String, DocumentEntry>,
}

#[wasm_bindgen]
impl BackboneWorkerHost {
    #[wasm_bindgen(constructor)]
    pub async fn new() -> Self {
        console_error_panic_hook::set_once();
        let pool = std::sync::Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
        Self { host: ArtifactHost::new(pool.clone()), pool, documents: std::collections::HashMap::new() }
    }

    #[wasm_bindgen(js_name = handleRequestBytes)]
    pub async fn handle_request_bytes(&mut self, bytes: &[u8]) -> Result<(), JsValue> {
        let _ = self.pool.pump(0);
        let request = backbone_worker_wire::decode_request(bytes).map_err(|error| JsValue::from_str(&error))?;
        match request {
            BackboneWorkerRequest::Open { document_id, schema, bindings, watch_external, actor } => {
                let config = crate::os_store::sync::ArtifactActorConfig { document_id: document_id.clone(), schema, bindings, watch_external: watch_external.unwrap_or(true), actor };
                self.host.close(&document_id);
                let channels = self.host.open(config);
                let mut events = self.host.subscribe(&document_id);
                let cmd_tx = channels.cmd_tx.clone();
                self.documents.insert(document_id.clone(), DocumentEntry { cmd_tx });
                wasm_bindgen_futures::spawn_local(async move {
                    loop {
                        match events.recv().await {
                            Ok(event) => {
                                let response = BackboneWorkerResponse::Event { document_id: document_id.clone(), event };
                                if let Ok(bytes) = backbone_worker_wire::encode_response(&response) {
                                    post_worker_message_bytes(&bytes);
                                }
                            }
                            Err(_) => break,
                        }
                    }
                });
            }
            BackboneWorkerRequest::Close { document_id } => {
                self.host.send(&document_id, ArtifactActorMsg::Detach);
                self.host.close(&document_id);
                self.documents.remove(&document_id);
            }
            BackboneWorkerRequest::Send { document_id, message } => {
                if let Some(entry) = self.documents.get(&document_id) {
                    let _ = entry.cmd_tx.send(message);
                }
            }
        }
        Ok(())
    }

    #[wasm_bindgen(js_name = postReady)]
    pub async fn post_ready() {
        if let Ok(bytes) = backbone_worker_wire::encode_response(&BackboneWorkerResponse::Ready) {
            post_worker_message_bytes(&bytes);
        }
    }
}
//#endregion 🔖️Worker

async fn post_worker_message_bytes(bytes: &[u8]) {
    let global = js_sys::global();
    if let Ok(post_message) = js_sys::Reflect::get(&global, &JsValue::from_str("postMessage")) {
        if let Some(function) = post_message.dyn_ref::<js_sys::Function>() {
            let wire = js_sys::Object::new();
            let _ = js_sys::Reflect::set(&wire, &JsValue::from_str("wire"), &js_sys::Uint8Array::from(bytes));
            let _ = function.call1(&global, &wire);
        }
    }
}
