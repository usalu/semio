
use super::*;
use crate::resource::ResourceRegistry;
use crate::scene::{FinishParams, Scene, SceneBuilder};

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn finish_params(viewport: [f32; 2]) -> FinishParams {
    FinishParams { viewport, dpr: 1.0, time_seconds_origin: 0.0, resource_ops: Vec::new() }
}

/// 🧬️ Exercised only through `<B: GraphicsBackend>` — the same shape every generic frame driver
/// uses — never through `dyn GraphicsBackend`, proving the trait needs no vtable.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn drive_backend<B: GraphicsBackend>(backend: &mut B) -> DeviceCapabilities {
    backend.capabilities()
}

#[test]
fn null_backend_satisfies_the_graphics_backend_trait() {
    let mut backend = NullBackend::new();
    let caps = drive_backend(&mut backend);
    assert!(caps.max_texture_dimension > 0);
    assert_eq!(backend.name(), "null");
}

#[test]
fn zero_size_surface_is_skipped_not_errored() {
    let mut backend = NullBackend::new();
    backend.resize(PhysicalSize::ZERO, 1.0).expect("resize to zero");
    let packet = Scene::finish(SceneBuilder::default(), finish_params([0.0, 0.0])).expect("finish");
    assert!(matches!(backend.render(&packet, 0.0), Ok(RenderReport::SkippedZeroSize)));
}

#[test]
fn resize_to_zero_and_back_restores_a_working_surface() {
    let mut backend = NullBackend::new();
    backend.resize(PhysicalSize::new(200, 100), 1.0).expect("initial resize");
    backend.resize(PhysicalSize::ZERO, 1.0).expect("resize to zero");
    backend.resize(PhysicalSize::new(200, 100), 1.0).expect("resize back");
    assert_eq!(backend.device_status(), DeviceStatus::Healthy);

    let mut builder = SceneBuilder::default();
    builder.push_solid([0.0, 0.0, 10.0, 10.0], [1.0, 0.0, 0.0, 1.0]);
    let packet = Scene::finish(builder, finish_params([200.0, 100.0])).expect("finish");
    assert!(matches!(backend.render(&packet, 0.0), Ok(RenderReport::Presented { .. })));
}

#[test]
fn apply_resources_before_render_succeeds_and_an_unapplied_id_errors_cleanly() {
    let mut backend = NullBackend::new();
    backend.resize(PhysicalSize::new(100, 100), 1.0).expect("resize");
    let mut registry = ResourceRegistry::default();

    // 🖼️ Drawing only *interns* a texture — `push_raster_quad` references it by id. Getting the
    // pixels to the device is a separate request, which is what makes a raster quad cheap to emit
    // every frame; so the upload has to be asked for explicitly before the backend can honour it.
    registry.request_texture_upload("known", 4, 4, vec![0; 64]);
    let mut applied_builder = SceneBuilder::default();
    applied_builder.push_raster_quad(&mut registry, "known", [0.0, 0.0, 10.0, 10.0], [0.0, 0.0, 1.0, 1.0], 1.0);
    backend.apply_resources(&registry.drain_ops()).expect("apply_resources");
    let applied_packet = Scene::finish(applied_builder, finish_params([100.0, 100.0])).expect("finish");
    assert!(matches!(backend.render(&applied_packet, 0.0), Ok(RenderReport::Presented { .. })));

    let mut unapplied_builder = SceneBuilder::default();
    unapplied_builder.push_raster_quad(&mut registry, "unknown", [0.0, 0.0, 10.0, 10.0], [0.0, 0.0, 1.0, 1.0], 1.0);
    let unapplied_packet = Scene::finish(unapplied_builder, finish_params([100.0, 100.0])).expect("finish");
    let result = backend.render(&unapplied_packet, 0.0);
    assert!(matches!(result, Err(BackendError::UnknownResource(ResourceKind::Texture))));
}

#[cfg(feature = "backend-testing")]
#[test]
fn forced_device_loss_reports_lost_status_and_recover_returns_dead_generations() {
    let mut backend = NullBackend::new();
    backend.resize(PhysicalSize::new(100, 100), 1.0).expect("resize");
    let mut registry = ResourceRegistry::default();
    let texture = registry.request_texture_upload("icon", 4, 4, vec![0; 64]);
    backend.apply_resources(&registry.drain_ops()).expect("apply_resources");

    backend.debug_force_device_loss();
    assert!(matches!(backend.device_status(), DeviceStatus::Lost(_)));

    let recovered = backend.recover().expect("recover");
    assert_eq!(recovered.lost_textures, vec![texture]);
    assert_eq!(backend.device_status(), DeviceStatus::Healthy);
}
