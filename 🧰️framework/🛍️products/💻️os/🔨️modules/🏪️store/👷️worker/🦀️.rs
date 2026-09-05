//! 🧵️ WASM backbone worker — browser-side `ArtifactHost` actor relaying the same protocol as
//! `🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts`, without materializing snapshots.

use crate::os_store::sync::{
    backbone_worker_wire::{self, BackboneWorkerRequest, BackboneWorkerResponse},
    ArtifactActorMsg, ArtifactHost, ArtifactMailboxSender,
};
use wasm_bindgen::prelude::*;

//#region 🔖️Worker
struct DocumentEntry {
    cmd_tx: ArtifactMailboxSender,
    document_key: crate::os_store::sync::ArtifactDocumentKey,
}

fn install_worker_panic_hook() {
    static INSTALLED: std::sync::Once = std::sync::Once::new();
    INSTALLED.call_once(|| {
        std::panic::set_hook(Box::new(|panic| eprintln!("[semio-worker panic] {panic}")));
    });
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
        install_worker_panic_hook();
        let pool = std::sync::Arc::new(semio_framework_async::process_worker_pool(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
        Self { host: ArtifactHost::new(pool.clone()), pool, documents: std::collections::HashMap::new() }
    }

    #[wasm_bindgen(js_name = handleRequestBytes)]
    pub async fn handle_request_bytes(&mut self, bytes: &[u8]) -> Result<(), JsValue> {
        let _ = self.pool.pump(0);
        let request = backbone_worker_wire::decode_request(bytes).map_err(|error| JsValue::from_str(&error))?;
        match request {
            BackboneWorkerRequest::Open { document_id, schema, bindings, watch_external, actor } => {
                let config = crate::os_store::sync::ArtifactActorConfig { document_id: document_id.clone(), schema, bindings, watch_external: watch_external.unwrap_or(true), actor };
                if let Some(previous) = self.documents.remove(&document_id) {
                    self.host.close_key(&previous.document_key);
                }
                let channels = self.host.open(config).await;
                let mut events = self.host.subscribe_key(&channels.document_key).await;
                let cmd_tx = channels.cmd_tx.clone();
                self.documents.insert(document_id.clone(), DocumentEntry { cmd_tx, document_key: channels.document_key });
                semio_framework_async::browser::spawn_local(async move {
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
                if let Some(entry) = self.documents.remove(&document_id) {
                    self.host.send_key(&entry.document_key, ArtifactActorMsg::Detach).await;
                    self.host.close_key(&entry.document_key);
                }
            }
            BackboneWorkerRequest::Send { document_id, message } => {
                if let Some(entry) = self.documents.get(&document_id) {
                    let _ = entry.cmd_tx.send(*message);
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
