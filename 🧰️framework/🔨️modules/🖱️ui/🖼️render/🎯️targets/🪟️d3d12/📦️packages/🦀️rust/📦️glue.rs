//! @emoji 🪟️ Hand-written Direct3D 12 backend for Windows.
//!
//! Implements [`ui_render::GraphicsBackend`] for this platform. Everything above it — the element
//! pipeline, layout, `Scene::finish`, the `RenderPacket` — is platform-neutral and shared, so this
//! crate's whole job is to replay batches a device can execute. It makes **no** ordering, batching or
//! clipping decisions of its own; all of that already happened in `Scene::finish`, which is precisely
//! what lets four independently hand-written backends agree pixel-for-pixel.
//!
//! ⚠️ SCAFFOLD — owned by packet `backend-d3d12`. Replace this placeholder wholesale.

#[cfg(not(target_os = "windows"))]
compile_error!("semio-framework-ui-backend-d3d12 builds only on Windows.");

//#region 🔖️Backend
//#endregion 🔖️Backend
