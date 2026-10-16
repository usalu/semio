use super::*;

fn actual_gpu() -> Option<GpuContext> {
    match semio_framework_async::block_on(GpuContext::headless(1, 1)) {
        Ok(gpu) => Some(gpu),
        Err(fault) => { eprintln!("[DEBUG] native engine raster ownership law unavailable: {fault}"); None }
    }
}
fn reserved(gpu: &mut GpuContext, witness: RasterTextureWitness) -> EngineRasterBuild {
    let identity = RasterContentIdentity::revision(1, 1, [1; 6]).unwrap();
    gpu.begin_raster_ownership(witness).unwrap();
    gpu.publish_raster_ownership(witness, "engine").unwrap();
    gpu.seal_raster_ownership(witness).unwrap();
    let mut admitted = false;
    for _ in 0..64 { admitted = gpu.prepare_raster_admission_step("engine", 1, 1, identity, witness).unwrap(); if admitted { break } }
    assert!(admitted);
    gpu.begin_engine_raster("engine", 1, 1, identity, witness, witness).unwrap()
}
fn close_build(gpu: &mut GpuContext, build: &mut EngineRasterBuild) {
    build.begin_close();
    for _ in 0..32 { if gpu.close_engine_raster_step(build).unwrap() { break } }
    assert!(build.terminal_is_empty());
}
fn close_gpu(gpu: &mut GpuContext) {
    for _ in 0..4096 { if gpu.close_raster_table_step().unwrap() { break } }
    assert!(gpu.raster_table_terminal_is_empty());
}

/// 🧷️ A session sharing a real device and queue cannot adopt another session's original admission.
#[test]
fn another_actual_session_returns_the_exact_owned_build_before_first_allocation() {
    let Some(mut gpu) = actual_gpu() else { return };
    let mut foreign = gpu.offscreen(1, 1).unwrap();
    let witness = RasterTextureWitness { scene_revision: 1, preview_generation: 1, operation: 1 };
    let original = Arc::clone(&gpu.roots);
    let build = reserved(&mut gpu, witness);
    assert!(build.admission.is_some());
    assert!(build.texture.is_none());
    let result = foreign.engine_raster_step(build, &Scene::default(), Color::new([0.0; 4]), witness);
    let Err(EngineRasterStageFault::Returned { fault, mut build }) = result else { panic!("foreign actual session must return the original build") };
    assert_eq!(fault, "engine GPU build belongs to another actual session");
    assert!(Arc::ptr_eq(build.roots.as_ref().unwrap(), &original));
    assert!(build.admission.is_some());
    assert!(build.texture.is_none());
    assert!(foreign.close_engine_raster_step(&mut build).is_err());
    assert!(Arc::ptr_eq(build.roots.as_ref().unwrap(), &original));
    close_build(&mut gpu, &mut build);
    close_gpu(&mut gpu);
    close_gpu(&mut foreign);
    eprintln!("[DEBUG] actual sibling session refused original build; issuer retained roots and closed exact reservation");
}

/// ⏳️ Changed live freshness refuses texture birth and retains the issuer's complete original build.
#[test]
fn stale_exact_reservation_returns_original_roots_before_allocating_texture() {
    let Some(mut gpu) = actual_gpu() else { return };
    let witness = RasterTextureWitness { scene_revision: 2, preview_generation: 3, operation: 4 };
    let original = Arc::clone(&gpu.roots);
    let build = reserved(&mut gpu, witness);
    let expected = RasterTextureWitness { operation: 5, ..witness };
    let result = gpu.engine_raster_step(build, &Scene::default(), Color::new([0.0; 4]), expected);
    let Err(EngineRasterStageFault::Returned { mut build, .. }) = result else { panic!("stale claim must return original build") };
    assert!(Arc::ptr_eq(build.roots.as_ref().unwrap(), &original));
    assert!(build.admission.is_some());
    assert!(build.texture.is_none());
    assert!(build.view.is_none());
    assert!(build.renderer.is_none());
    assert!(!build.terminal_is_empty());
    close_build(&mut gpu, &mut build);
    close_gpu(&mut gpu);
    eprintln!("[DEBUG] stale exact tuple allocated no texture; actual original build closed before terminal");
}

/// 📬️ A full actual close slot returns a second issuer's exact admission instead of consuming it.
#[test]
fn occupied_retirement_slot_returns_the_other_actual_issued_admission() {
    let Some(mut gpu) = actual_gpu() else { return };
    let mut other = gpu.offscreen(1, 1).unwrap();
    let first = RasterTextureWitness { scene_revision: 7, preview_generation: 7, operation: 7 };
    let second = RasterTextureWitness { scene_revision: 8, preview_generation: 8, operation: 8 };
    let mut first_build = reserved(&mut gpu, first);
    let mut second_build = reserved(&mut other, second);
    let original = Arc::clone(second_build.roots.as_ref().unwrap());
    let first_admission = first_build.admission.take().unwrap();
    assert!(gpu.cancel_engine_texture_admission(first_admission).is_ok());
    let second_admission = second_build.admission.take().unwrap();
    match gpu.cancel_engine_texture_admission(second_admission) {
        Err(RasterTextureCancelFault::Returned { fault, admission }) => {
            assert_eq!(fault, "raster rejected admission retirement capacity exhausted");
            second_build.admission = Some(admission);
        }
        _ => panic!("occupied actual slot must return the other original admission"),
    }
    assert!(Arc::ptr_eq(second_build.roots.as_ref().unwrap(), &original));
    assert!(second_build.admission.is_some());
    close_build(&mut other, &mut second_build);
    close_build(&mut gpu, &mut first_build);
    close_gpu(&mut other);
    close_gpu(&mut gpu);
    eprintln!("[DEBUG] actual occupied retirement returned second issuer admission; both actual tables/builds retired");
}

/// 🧺️ A stale actual admission transferred into an available rejection close slot is reported as retained.
#[test]
fn rejected_actual_admission_is_retained_only_after_the_close_cursor_owns_it() {
    let Some(mut gpu) = actual_gpu() else { return };
    let mut issuer = gpu.offscreen(1, 1).unwrap();
    let witness = RasterTextureWitness { scene_revision: 9, preview_generation: 9, operation: 9 };
    let mut build = reserved(&mut issuer, witness);
    let admission = build.admission.take().unwrap();
    assert!(matches!(gpu.cancel_engine_texture_admission(admission), Err(RasterTextureCancelFault::Retained("raster admission was stale and retained for bounded retirement"))));
    close_build(&mut issuer, &mut build);
    close_gpu(&mut issuer);
    close_gpu(&mut gpu);
    eprintln!("[DEBUG] actual stale admission moved into rejected close owner before Retained; both actual tables/builds retired");
}
