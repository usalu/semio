//! 📦️ Package glue — wiring only. Domain lives at owner `🦀️component.rs`. The `wasm_bindgen`
//! `KernelHost` wrapper lives in THIS file, behind `#[cfg(target_arch = "wasm32")]`, so the pure
//! crate core never sees `wasm_bindgen`/`web_sys`. It passes only byte buffers (pack-encoded
//! `Envelope`/`TurnResult`/`Decision`) — no typed value ever crosses the wasm boundary.

#[path = "../../🦀️component.rs"]
mod component;
pub use component::*;

//#region 🔖️KernelHost
#[cfg(target_arch = "wasm32")]
mod kernel_host {
    use wasm_bindgen::prelude::*;

    use super::component::{pack, ActivationEvent, ActorId, Decision, Envelope, Kernel, ShardKind, TurnResult};

    /// 🌉️ wasm-bindgen wrapper around [`Kernel`] for the React-web / wgpu-web hosts (see design
    /// §1's three-host list). Every method takes/returns pack-encoded bytes only — this type owns
    /// no logic of its own beyond (de)serializing at the boundary and delegating to `Kernel`.
    #[wasm_bindgen]
    pub struct KernelHost {
        inner: Kernel,
    }

    #[wasm_bindgen]
    impl KernelHost {
        #[wasm_bindgen(constructor)]
        pub async fn new(shard_count: u16, exclusive_reserve: u16, grants_per_tick: u32) -> KernelHost {
            KernelHost { inner: Kernel::new(ShardKind::WebWorker, shard_count, exclusive_reserve, grants_per_tick) }
        }

        /// ▶️ `activation_bytes` is a pack-encoded `(PackageId, u16 plugin_ordinal, ActorKind, Lane,
        /// Option<WindowId>, ActivationEvent)` tuple; returns the pack-encoded fresh `ActorId`.
        #[wasm_bindgen]
        pub async fn activate(&mut self, activation_bytes: &[u8]) -> Result<Vec<u8>, JsError> {
            let mut pos = 0usize;
            let package = super::component::PackageId::pack_decode(activation_bytes, &mut pos).map_err(to_js_error)?;
            let plugin_ordinal = pack::read_u16(activation_bytes, &mut pos, "activate::plugin_ordinal").map_err(to_js_error)?;
            let kind = super::component::ActorKind::pack_decode(activation_bytes, &mut pos).map_err(to_js_error)?;
            let lane = super::component::Lane::pack_decode(activation_bytes, &mut pos).map_err(to_js_error)?;
            let window = pack::read_opt(activation_bytes, &mut pos, "activate::window", super::component::WindowId::pack_decode).map_err(to_js_error)?;
            let event = ActivationEvent::Manual;
            let id = self.inner.activate(package, plugin_ordinal, kind, lane, window, event);
            let mut out = Vec::new();
            id.pack_encode(&mut out);
            Ok(out)
        }

        #[wasm_bindgen]
        pub async fn submit(&mut self, envelope_bytes: &[u8]) -> Result<Vec<u8>, JsError> {
            let mut pos = 0usize;
            let envelope = Envelope::pack_decode(envelope_bytes, &mut pos).map_err(to_js_error)?;
            let backpressure = self.inner.submit(&envelope);
            let mut out = Vec::new();
            backpressure.pack_encode(&mut out);
            Ok(out)
        }

        #[wasm_bindgen]
        pub async fn tick(&mut self, now_ms: u64) -> Vec<u8> {
            let decision: Decision = self.inner.tick(now_ms);
            let mut out = Vec::new();
            decision.pack_encode(&mut out);
            out
        }

        #[wasm_bindgen]
        pub async fn complete(&mut self, actor_bytes: &[u8], turn_result_bytes: &[u8], now_ms: u64) -> Result<(), JsError> {
            let mut actor_pos = 0usize;
            let actor = ActorId::pack_decode(actor_bytes, &mut actor_pos).map_err(to_js_error)?;
            let mut result_pos = 0usize;
            let result = TurnResult::pack_decode(turn_result_bytes, &mut result_pos).map_err(to_js_error)?;
            self.inner.complete(actor, &result, now_ms).map_err(to_js_error_kernel)?;
            Ok(())
        }

        #[wasm_bindgen]
        pub async fn metrics(&self) -> Vec<u8> {
            let mut out = Vec::new();
            self.inner.metrics().pack_encode(&mut out);
            out
        }
    }

    async fn to_js_error(err: pack::PackError) -> JsError {
        JsError::new(&err.to_string())
    }

    async fn to_js_error_kernel(err: super::component::KernelError) -> JsError {
        JsError::new(&err.to_string())
    }
}
#[cfg(target_arch = "wasm32")]
pub use kernel_host::KernelHost;
//#endregion 🔖️KernelHost
