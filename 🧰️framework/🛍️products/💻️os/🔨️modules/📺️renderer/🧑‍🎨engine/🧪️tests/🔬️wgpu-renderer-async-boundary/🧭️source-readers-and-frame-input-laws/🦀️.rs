/// 🧭️ Source readers and frame input laws: the unbounded step grant, the library, binary, manifest, host, draw and prepared source readers, the refused frame preparation, the presented input seal, the pre-submit epoch change and the executor bridge laws.

/// 🎟️ Unbounded retained grant: the pre-migration step budget carried no retained limit.
const UNBOUNDED_STEP_GRANT: semio_framework_job::RetainedCloneGrant = semio_framework_job::RetainedCloneGrant { maximum_items: usize::MAX, maximum_copy_bytes: usize::MAX, maximum_capacity_bytes: usize::MAX, maximum_release_bytes: usize::MAX, maximum_depth: usize::MAX };

#[allow(non_snake_case)]
fn LIBRARY_SOURCE() -> &'static str {
    semio_framework_os_renderer_wgpu_test_support::renderer_source()
}
#[allow(non_snake_case)]
fn BINARY_SOURCE() -> &'static str {
    semio_framework_os_renderer_wgpu_test_support::repo_text("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🦀️.rs")
}
#[allow(non_snake_case)]
fn MANIFEST_SOURCE() -> &'static str {
    semio_framework_os_renderer_wgpu_test_support::repo_text("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Cargo.toml")
}
#[allow(non_snake_case)]
fn WINT_APP_SOURCE() -> &'static str {
    semio_framework_os_renderer_wgpu_test_support::repo_text("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs")
}
#[allow(non_snake_case)]
fn OS_HOST_SOURCE() -> &'static str {
    semio_framework_os_renderer_wgpu_test_support::repo_text("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏠️os-host/🦀️.rs")
}
#[allow(non_snake_case)]
fn GPU_SOURCE() -> &'static str {
    semio_framework_os_renderer_wgpu_test_support::repo_text("🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🎮️execution/🦀️.rs")
}
#[allow(non_snake_case)]
fn DRAW_SOURCE() -> &'static str {
    semio_framework_os_renderer_wgpu_test_support::repo_texts(&["🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🏷️types/🦀️.rs", "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🕸️mesh/🦀️.rs", "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🔧️pipeline/🏭️factory/🦀️.rs", "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🔧️pipeline/🎬️execution/🦀️.rs", "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🖼️raster/🦀️.rs"])
}
#[allow(non_snake_case)]
fn PREPARED_SOURCE() -> &'static str {
    semio_framework_os_renderer_wgpu_test_support::repo_text("🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs")
}
#[allow(non_snake_case)]
fn ENGINE_CANVAS_SOURCE() -> &'static str {
    semio_framework_os_renderer_wgpu_test_support::repo_texts(&["🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🎮️execution/🎬️canvas/🦀️.rs", "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/📦️prepared-engine-canvas/🦀️.rs"])
}
#[allow(non_snake_case)]
fn DRAW_LAWS_SOURCE() -> &'static str {
    semio_framework_os_renderer_wgpu_test_support::repo_texts(&["🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🕸️mesh/🧪️tests/🔬️unit/🦀️.rs", "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🔧️pipeline/🏭️factory/🧪️tests/🦀️.rs", "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🔧️pipeline/🎬️execution/🧪️tests/🔬️unit/🦀️.rs", "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🖼️raster/🧪️tests/🔬️unit/🦀️.rs"])
}

/// 🩺️ LAW: a frame build that ends because its PREPARATION refused must name the refusal. Nothing
/// else in the chain can: `AppFramePreparation` answers an empty `JobFault`, `ActiveFrameBuild`
/// answers it by cancelling its own token, and the job layer then reports a bare `Cancelled`. One
/// unnamed refusal per build is how the shell sat at one presented frame per boot with a silent
/// console for a whole packet (`📓️w7b-presenter-one-frame-per-boot.md` §1).
#[test]
fn a_refused_frame_preparation_names_its_fault_before_the_build_cancels() {
    let frame_job = semio_framework_os_renderer_wgpu_test_support::repo_text("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧵️frame-job/🦀️.rs");
    let prepare_at = frame_job.find("ActiveFramePhase::Prepare(preparation) => {").expect("the prepare phase");
    let refused_at = prepare_at + frame_job[prepare_at..].find("StepOutcome::Cancelled | StepOutcome::Fault(_) =>").expect("the refused arm");
    let refused = &frame_job[refused_at..][..frame_job[refused_at..].find("ActiveFrameStep::Pending").expect("the refused arm ends on its own step")];
    assert!(refused.contains("preparation.fault()"), "the refused arm reads the preparation's own fault");

    for arm in [
        "self.fault = Some(\"prepared render job admission was refused\")",
        "self.fault = Some(\"prepared render job session admission was refused\")",
        "self.fault = Some(\"prepared render job lost its session\")",
        "self.fault = Some(\"prepared render job was cancelled\")",
    ] {
        assert!(LIBRARY_SOURCE().contains(arm), "every preparation refusal names itself: {arm}");
    }
    assert!(LIBRARY_SOURCE().contains("session.checked_out_job_mut().and_then(|job| job.fault())"), "and a refusal from inside the prepared job carries that job's own fault string");
}

#[test]
fn a_presented_input_seal_refusal_keeps_its_exact_fault_name() {
    assert!(LIBRARY_SOURCE().contains("Err(fault) => return FrameBuildBoundaryStep::Fault(fault)"));
    assert!(!LIBRARY_SOURCE().contains("Err(_) => return FrameBuildBoundaryStep::Fault(\"presented input candidate generation exhausted\")"));
}

#[test]
fn a_pre_submit_input_epoch_change_retires_and_reschedules_without_faulting_the_surface() {
    let render = LIBRARY_SOURCE().split("AppPresentPhase::Render => {").nth(1).expect("the render phase");
    let render = &render[..render.find("AppPresentPhase::CloseGpu =>").expect("the close-GPU phase")];
    assert!(render.contains("PresentedInputCandidateProgress::Stale"), "the pre-submit phase names an input epoch supersession");
    assert!(!render.contains("prepared frame input authority was stale before submit"), "normal pre-submit input supersession is not a surface fault");
    let stale_progress = render.split("PresentedInputCandidateProgress::Stale => {").nth(1).expect("the stale progress arm");
    let stale_progress = &stale_progress[..stale_progress.find("}\n                }").expect("the stale progress arm end")];
    let stale_match = render.split("if !shell.presented_input_candidate_matches(input_candidate) {").nth(1).expect("the final exact witness check");
    let stale_match = &stale_match[..stale_match.find("}\n                drop(runtime);").expect("the exact witness check end")];
    for stale in [stale_progress, stale_match] {
        assert!(stale.contains("cursor.frame.packet = self.gate.abort_pending();"), "the stale observation returns the unsubmitted packet");
        assert!(stale.contains("cursor.witness = None;"), "the stale observation retires the presenter witness");
        assert!(stale.contains("cursor.phase = AppPresentPhase::Aborted;"), "the stale observation enters bounded retirement");
        assert!(stale.contains("return Ok(AppPresentStep::Pending);"), "the stale observation keeps the presenter live without a surface fault");
    }

    let aborted = LIBRARY_SOURCE().split("AppPresentPhase::Aborted => {").nth(1).expect("the abort phase");
    let aborted = &aborted[..aborted.find("AppPresentPhase::Fullscreen =>").expect("the next presenter phase")];
    assert!(aborted.contains("discard_presented_input_candidate(input_candidate)"), "the next bounded abort turn returns the exact Shell/UI witness before rescheduling");
}

#[test]
fn product_library_has_no_executor_bridge() {
    assert!(!LIBRARY_SOURCE().contains(concat!("poll", "ster")));
    assert!(!LIBRARY_SOURCE().contains(concat!("block", "_on")));
    assert!(LIBRARY_SOURCE().contains("KernelPoolFuture::spawn"));
    assert!(LIBRARY_SOURCE().contains("spawn_app_task"));
    assert!(!LIBRARY_SOURCE().contains(concat!("TASK", "_POOL")));
    assert!(!LIBRARY_SOURCE().contains(concat!("poll", "_tasks")));
    assert!(!LIBRARY_SOURCE().contains("thread_local! {\n        static REAL_WAKER"));
    assert!(!WINT_APP_SOURCE().contains(concat!("poll", "_tasks")));
    assert!(!WINT_APP_SOURCE().contains(concat!("TASK", "_POOL")));
    assert!(!LIBRARY_SOURCE().contains("type RuntimeApply = Box"));
    assert!(!LIBRARY_SOURCE().contains(concat!("mem::", "forget")));
    assert!(!WINT_APP_SOURCE().contains("dispatch_drained_events"));
    assert!(LIBRARY_SOURCE().contains("struct RuntimeDispatchCursor"));
    assert!(LIBRARY_SOURCE().contains("ResumeDispatch"));
    assert!(!LIBRARY_SOURCE().contains("response.array_buffer()"));
    assert!(!LIBRARY_SOURCE().contains("collect_pending_ui_image_fetches"));
    assert!(!LIBRARY_SOURCE().contains("collect_pending_map_tile_fetches"));
    assert!(LIBRARY_SOURCE().contains("struct AppPresentCursor"));
    assert!(!LIBRARY_SOURCE().contains(".submit_prepared("));
    assert!(!GPU_SOURCE().contains("fn apply_prepared_uploads"));
    assert!(!DRAW_SOURCE().contains("for index in 0..schema.vertices"));
    assert!(!DRAW_SOURCE().contains("for index in 0..schema.indices"));
    assert!(DRAW_SOURCE().contains("pub fn ensure_mesh_step"));
    assert!(DRAW_SOURCE().contains("pub fn close_upload_step"));
    assert!(GPU_SOURCE().contains("pub fn close_mesh_upload_step"));
    let upload_close = OS_HOST_SOURCE().find("presenter.close_world_owners_step()").expect("active upload and packet close phase");
    let world_close = OS_HOST_SOURCE().find("runtime.close_world3d_dynamic_step()").expect("world mesh close phase");
    assert!(upload_close < world_close);
}
