# Design Notes

## Preserved Source Rationale

<a id="source-notes-1"></a>

### 🦀️.rs:1

🎭️ Pooled actor/plugin runtime kernel — domain-neutral, pure: no I/O, no clock (callers pass
`now_ms`), no `wasm_bindgen`/`web_sys`/`winit`/`tokio`/`std::thread` in this file. Transports are
injected via the [`ShardTransport`] trait; the one exception, [`ThreadTransport`], lives behind
`#[cfg(not(target_arch = "wasm32"))]` because it is built on `std::sync::mpsc`.

🪡 **Opaque seam**: `Effect`, `Event` and `UiPatch` are owned by a parallel packet in
`🎠️kernel` and are NOT defined here. This crate carries them as pack-encoded `Vec<u8>` inside
[`TurnResult`] (`ui_patches`, `effects`) and [`Payload::Event`] — the kernel crate encodes/decodes
the concrete typed collections; this crate only ever moves already-encoded bytes. The same seam
applies to `CapabilityGrant`: `kernel::CapabilityGrant` (`🛂️manifest/🦀️.rs`) is an
application-layer type this framework-tier crate must not depend on (would invert the
framework → os-product layering), so [`CapabilityGrant`] here is a minimal, local, pack-codeable
stand-in reconciled by the `B1-host-native` packet at integration time.

See `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📓️design-runtime.md` §1.
