//! 🌉️ Raster retained document-load bridge.

#[cfg(target_arch = "wasm32")]
mod document_vcs {
    //#region 🔖️ArtifactVcs
    use std::cell::RefCell;

    use wasm_bindgen::prelude::*;

    use semio_framework_plugin::{ArtifactEnvelopeDecodeOperationHandle, ArtifactEnvelopeDecodeOperationPoll, EditorApp, PluginApp, VcsArtifactApp};

    use crate::editor::raster::RasterPlayApp;

    type RasterApp = VcsArtifactApp<EditorApp<RasterPlayApp>>;

    const RASTER_ENVELOPE_MAXIMUM_PAGES: usize = store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_PAGES;
    const RASTER_ENVELOPE_MAXIMUM_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES;

    fn js_fault(error: impl ToString) -> JsValue {
        JsValue::from_str(&error.to_string())
    }

    #[wasm_bindgen]
    pub struct RasterEnvelopeLoadHandle {
        operation: u64,
        generation: u64,
    }

    impl RasterEnvelopeLoadHandle {
        fn runtime_handle(&self) -> ArtifactEnvelopeDecodeOperationHandle {
            ArtifactEnvelopeDecodeOperationHandle { operation: semio_framework_job::OperationId(self.operation), generation: semio_framework_job::Generation(self.generation) }
        }
    }

    #[wasm_bindgen]
    impl RasterEnvelopeLoadHandle {
        #[wasm_bindgen(getter)]
        pub fn operation(&self) -> u64 {
            self.operation
        }

        #[wasm_bindgen(getter)]
        pub fn generation(&self) -> u64 {
            self.generation
        }
    }

    #[wasm_bindgen]
    pub struct RasterArtifactVcs {
        app: RefCell<RasterApp>,
    }

    #[wasm_bindgen]
    impl RasterArtifactVcs {
        #[wasm_bindgen(constructor)]
        pub async fn new() -> Result<RasterArtifactVcs, JsValue> {
            let app = VcsArtifactApp::new(EditorApp::<RasterPlayApp>::default()).await;
            Ok(Self { app: RefCell::new(app) })
        }

        #[wasm_bindgen(js_name = beginEnvelopeLoad)]
        pub fn begin_envelope_load(&self, maximum_pages: usize, maximum_bytes: usize) -> Result<RasterEnvelopeLoadHandle, JsValue> {
            if maximum_pages == 0 || maximum_pages > RASTER_ENVELOPE_MAXIMUM_PAGES || maximum_bytes == 0 || maximum_bytes > RASTER_ENVELOPE_MAXIMUM_BYTES {
                return Err(js_fault("raster-envelope.invalid-credits"));
            }
            let handle = self.app.borrow_mut().begin_artifact_envelope_ingress(maximum_pages, maximum_bytes).map_err(js_fault)?;
            Ok(RasterEnvelopeLoadHandle { operation: handle.operation.0, generation: handle.generation.0 })
        }

        #[wasm_bindgen(js_name = admitEnvelopePage)]
        pub fn admit_envelope_page(&self, handle: &RasterEnvelopeLoadHandle, source: &js_sys::Uint8Array) -> Result<(), JsValue> {
            let len = usize::try_from(source.length()).map_err(js_fault)?;
            if len > store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES {
                return Err(js_fault("raster-envelope.page-too-large"));
            }
            let mut app = self.app.borrow_mut();
            app.preflight_artifact_envelope_ingress_page(handle.runtime_handle(), len).map_err(js_fault)?;
            app.construct_and_admit_artifact_envelope_ingress_page(handle.runtime_handle(), len, || {
                let mut bytes = [0; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
                source.copy_to(&mut bytes[..len]);
                store::ArtifactEnvelopeDecodePage::try_from_array(bytes, len).expect("preflighted Raster page length is fixed")
            })
            .map_err(js_fault)
        }

        #[wasm_bindgen(js_name = sealEnvelopeLoad)]
        pub fn seal_envelope_load(&self, handle: &RasterEnvelopeLoadHandle) -> Result<bool, JsValue> {
            self.app.borrow_mut().seal_artifact_envelope_ingress(handle.runtime_handle()).map_err(js_fault)
        }

        #[wasm_bindgen(js_name = pollEnvelopeLoad)]
        pub fn poll_envelope_load(&self, handle: &RasterEnvelopeLoadHandle) -> Result<u8, JsValue> {
            let mut app = self.app.borrow_mut();
            app.maintenance_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).map_err(js_fault)?;
            match app.advance_artifact_envelope_load(handle.runtime_handle()).map_err(js_fault)? {
                ArtifactEnvelopeDecodeOperationPoll::Pending => Ok(0),
                ArtifactEnvelopeDecodeOperationPoll::Progress => Ok(1),
                ArtifactEnvelopeDecodeOperationPoll::Ready => {
                    if !app.acknowledge_artifact_store_replacement(handle.runtime_handle()).map_err(js_fault)? {
                        return Ok(1);
                    }
                    Ok(2)
                }
                ArtifactEnvelopeDecodeOperationPoll::Cancelled => {
                    let _ = app.acknowledge_artifact_store_replacement(handle.runtime_handle()).map_err(js_fault)?;
                    Ok(3)
                }
                ArtifactEnvelopeDecodeOperationPoll::Fault => {
                    let _ = app.acknowledge_artifact_store_replacement(handle.runtime_handle()).map_err(js_fault)?;
                    Ok(4)
                }
            }
        }

        #[wasm_bindgen(js_name = cancelEnvelopeLoad)]
        pub fn cancel_envelope_load(&self, handle: &RasterEnvelopeLoadHandle) -> Result<(), JsValue> {
            self.app.borrow_mut().cancel_artifact_envelope_load(handle.runtime_handle()).map_err(js_fault)
        }

        #[wasm_bindgen(js_name = closeStep)]
        pub fn close_step(&self) -> Result<bool, JsValue> {
            match self.app.borrow_mut().close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).map_err(js_fault)? {
                semio_framework_plugin::PluginCloseStep::Complete => Ok(true),
                semio_framework_plugin::PluginCloseStep::Pending { .. } | semio_framework_plugin::PluginCloseStep::Blocked { .. } => Ok(false),
            }
        }
    }
    //#endregion 🔖️ArtifactVcs
}

#[cfg(target_arch = "wasm32")]
pub use document_vcs::*;
