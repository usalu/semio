
use super::*;
use ui_render::{FinishParams, ResourceRegistry, Scene, SceneBuilder};

/// 🧵️ Drives an `async fn` that structurally never suspends (Metal's device/queue/layer creation
/// is synchronous — see `MetalBackend::new`'s docstring) to completion without pulling in an
/// executor crate. Panics if the future ever actually returns `Pending`, which would mean this
/// backend grew a real suspension point somewhere and this helper is no longer valid.
fn block_on<F: std::future::Future>(future: F) -> F::Output {
    use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
    const VTABLE: RawWakerVTable = RawWakerVTable::new(|_| RAW_WAKER, |_| {}, |_| {}, |_| {});
    const RAW_WAKER: RawWaker = RawWaker::new(std::ptr::null(), &VTABLE);
    let waker = unsafe { Waker::from_raw(RAW_WAKER) };
    let mut context = Context::from_waker(&waker);
    let mut future = Box::pin(future);
    match future.as_mut().poll(&mut context) {
        Poll::Ready(value) => value,
        Poll::Pending => panic!("metal backend: a construction future that should never suspend returned Pending"),
    }
}

fn finish_params(viewport: [f32; 2]) -> FinishParams {
    FinishParams { viewport, dpr: 1.0, time_seconds_origin: 0.0, resource_ops: Vec::new() }
}

#[test]
fn constructing_a_headless_backend_succeeds_or_skips_cleanly() {
    let Ok(mut backend) = block_on(MetalBackend::new_headless(PhysicalSize::new(64, 64), 1.0)) else {
        eprintln!("skipping: no Metal device available on this machine");
        return;
    };
    assert_eq!(GraphicsBackend::name(&backend), "metal");
    assert!(GraphicsBackend::capabilities(&backend).max_texture_dimension > 0);
    assert_eq!(GraphicsBackend::device_status(&backend), DeviceStatus::Healthy);
    let _ = GraphicsBackend::resize(&mut backend, PhysicalSize::new(64, 64), 1.0);
}

#[test]
fn zero_size_resize_parks_and_restores() {
    let Ok(mut backend) = block_on(MetalBackend::new_headless(PhysicalSize::new(64, 64), 1.0)) else {
        eprintln!("skipping: no Metal device available on this machine");
        return;
    };
    GraphicsBackend::resize(&mut backend, PhysicalSize::ZERO, 1.0).expect("resize to zero");
    let packet = Scene::finish(SceneBuilder::default(), finish_params([0.0, 0.0])).expect("finish");
    assert!(matches!(GraphicsBackend::render(&mut backend, &packet, 0.0), Ok(RenderReport::SkippedZeroSize)));

    GraphicsBackend::resize(&mut backend, PhysicalSize::new(64, 64), 1.0).expect("resize back");
    let mut builder = SceneBuilder::default();
    builder.push_solid([0.0, 0.0, 10.0, 10.0], [1.0, 0.0, 0.0, 1.0]);
    let packet = Scene::finish(builder, finish_params([64.0, 64.0])).expect("finish");
    let report = GraphicsBackend::render(&mut backend, &packet, 0.0).expect("render after restoring a nonzero size");
    assert!(matches!(report, RenderReport::Presented { .. } | RenderReport::SkippedOutOfDate));
}

#[test]
fn apply_resources_before_render_succeeds_and_an_unapplied_id_errors_cleanly() {
    let Ok(mut backend) = block_on(MetalBackend::new_headless(PhysicalSize::new(64, 64), 1.0)) else {
        eprintln!("skipping: no Metal device available on this machine");
        return;
    };
    let mut registry = ResourceRegistry::default();
    registry.request_texture_upload("known", 4, 4, vec![0; 64]);
    let mut applied_builder = SceneBuilder::default();
    applied_builder.push_raster_quad(&mut registry, "known", [0.0, 0.0, 10.0, 10.0], [0.0, 0.0, 1.0, 1.0], 1.0);
    GraphicsBackend::apply_resources(&mut backend, &registry.drain_ops()).expect("apply_resources");
    let applied_packet = Scene::finish(applied_builder, finish_params([64.0, 64.0])).expect("finish");
    let report = GraphicsBackend::render(&mut backend, &applied_packet, 0.0).expect("render with an applied texture");
    assert!(matches!(report, RenderReport::Presented { .. } | RenderReport::SkippedOutOfDate));

    let mut unapplied_builder = SceneBuilder::default();
    unapplied_builder.push_raster_quad(&mut registry, "unknown", [0.0, 0.0, 10.0, 10.0], [0.0, 0.0, 1.0, 1.0], 1.0);
    let unapplied_packet = Scene::finish(unapplied_builder, finish_params([64.0, 64.0])).expect("finish");
    let result = GraphicsBackend::render(&mut backend, &unapplied_packet, 0.0);
    assert!(matches!(result, Err(BackendError::UnknownResource(ResourceKind::Texture))));
}

#[test]
fn forced_device_loss_reports_lost_and_recover_names_the_dead_generation() {
    let Ok(mut backend) = block_on(MetalBackend::new_headless(PhysicalSize::new(64, 64), 1.0)) else {
        eprintln!("skipping: no Metal device available on this machine");
        return;
    };
    let mut registry = ResourceRegistry::default();
    let texture = registry.request_texture_upload("icon", 4, 4, vec![0; 64]);
    GraphicsBackend::apply_resources(&mut backend, &registry.drain_ops()).expect("apply_resources");

    GraphicsBackend::debug_force_device_loss(&mut backend);
    assert!(matches!(GraphicsBackend::device_status(&backend), DeviceStatus::Lost(_)));
    let packet = Scene::finish(SceneBuilder::default(), finish_params([64.0, 64.0])).expect("finish");
    assert!(matches!(GraphicsBackend::render(&mut backend, &packet, 0.0), Err(BackendError::DeviceLost(_))));

    let recovered = GraphicsBackend::recover(&mut backend).expect("recover");
    assert_eq!(recovered.lost_textures, vec![texture]);
    assert_eq!(GraphicsBackend::device_status(&backend), DeviceStatus::Healthy);
}

#[test]
fn read_back_reports_zero_size_cleanly_before_any_frame_is_presented() {
    let Ok(mut backend) = block_on(MetalBackend::new_headless(PhysicalSize::ZERO, 1.0)) else {
        eprintln!("skipping: no Metal device available on this machine");
        return;
    };
    assert!(matches!(GraphicsBackend::read_back(&mut backend), Err(BackendError::ZeroSizeSurface)));
}
