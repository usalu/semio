//! 🕸️ Browser execution of the actual child DAG store.
    use crate::{create_dag_store,empty_dag_document,ActorId,DagStore};
    use std::cell::RefCell;
    use wasm_bindgen::prelude::*;
    use semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::{EntityIdentityAuthority, Observer};
    use semio_framework_value::native_encoding::NativeEncodeProgress;

    /// 🪪️ Fixed identity-authoring ceiling every browser dispatch admits before it owns any authored bytes.
    const DAG_VCS_IDENTITY_CEILING_BYTES: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;

    #[wasm_bindgen]
    pub struct DagSnapshotVcs {
        store: RefCell<DagStore>,
    }

    #[wasm_bindgen]
    impl DagSnapshotVcs {
        /// 🌐️ Constructs the VCS bridge without synchronously blocking the browser host callback.
        #[wasm_bindgen(js_name = create)]
        pub async fn create(actor: String) -> Result<DagSnapshotVcs, JsValue> {
            let store = create_dag_store("dag", empty_dag_document(), ActorId(actor.into())).await.map_err(|e| JsValue::from_str(&e.to_string()))?;
            Ok(Self { store: RefCell::new(store) })
        }

        #[wasm_bindgen(js_name = dispatchText)]
        pub async fn dispatch_text(&self, command_text: &str) -> Result<(), JsValue> {
            let mut store = self.store.try_borrow_mut().map_err(|_| JsValue::from_str("DAG VCS operation already in progress"))?;
            let mut observer = |_: NativeEncodeProgress| true;
            let mut identity = EntityIdentityAuthority::new(DAG_VCS_IDENTITY_CEILING_BYTES, &mut observer as &mut Observer<'_>).map_err(|e| JsValue::from_str(&e.to_string()))?;
            store.dispatch_text(command_text, &mut identity).await.map(|_| ()).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = dispatchBinary)]
        pub async fn dispatch_binary(&self, command_bytes: &[u8]) -> Result<(), JsValue> {
            let mut store = self.store.try_borrow_mut().map_err(|_| JsValue::from_str("DAG VCS operation already in progress"))?;
            let mut observer = |_: NativeEncodeProgress| true;
            let mut identity = EntityIdentityAuthority::new(DAG_VCS_IDENTITY_CEILING_BYTES, &mut observer as &mut Observer<'_>).map_err(|e| JsValue::from_str(&e.to_string()))?;
            store.dispatch_binary(command_bytes, &mut identity).await.map(|_| ()).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = snapshotJson)]
        pub async fn snapshot_json(&self) -> Result<String, JsValue> {
            let store = self.store.try_borrow().map_err(|_| JsValue::from_str("DAG VCS operation already in progress"))?;
            store.snapshot_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = envelopeJson)]
        pub async fn envelope_json(&self) -> Result<String, JsValue> {
            let store = self.store.try_borrow().map_err(|_| JsValue::from_str("DAG VCS operation already in progress"))?;
            store.envelope_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = generation)]
        pub async fn generation(&self) -> Result<u32, JsValue> {
            let store = self.store.try_borrow().map_err(|_| JsValue::from_str("DAG VCS operation already in progress"))?;
            Ok(store.generation() as u32)
        }
    }
