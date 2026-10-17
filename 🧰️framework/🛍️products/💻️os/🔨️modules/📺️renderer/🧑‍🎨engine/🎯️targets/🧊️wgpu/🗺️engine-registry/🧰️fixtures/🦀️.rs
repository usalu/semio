//! 🧰️ Test fixtures of the engine surface registry: the process-wide law lock every law that attaches an engine surface holds.
//!
//! 🧭️ Behind `testkit`: the monolith test unit held the lock in the EngineCanvas standalone law file; laws in the test bins name it through this crate.

/// 🔒️ `ENGINE_SURFACES` is ONE process-wide registry and `take_engine_surface_registrations` drains it
/// globally, so two laws attaching engine surfaces at the same time steal each other's registrations
/// and each other's close tokens. Every law that attaches one holds this first
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub static ENGINE_SURFACE_LAW_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// 🔒️ The guard itself, poison-tolerant: a law that panicked already reported its own failure and
/// must not turn every later law into a second, misleading one.
pub fn engine_surface_law_guard() -> std::sync::MutexGuard<'static, ()> {
    ENGINE_SURFACE_LAW_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// 🎟️ The retained grant one engine close step is funded with: far above any fixture scene's copy, capacity and release demand.
const ENGINE_CLOSE_GRANT: semio_framework_job::RetainedCloneGrant = semio_framework_job::RetainedCloneGrant {
    maximum_items: 1 << 16,
    maximum_copy_bytes: 1 << 24,
    maximum_capacity_bytes: 1 << 24,
    maximum_release_bytes: 1 << 24,
    maximum_depth: 256,
};

/// 🧪️ Runs one bounded step of the engine close ladder under a funded original caller context.
pub fn with_engine_close_context<T>(fuel: u64, step: impl FnOnce(&mut semio_framework_job::StepContext<'_>) -> T) -> T {
    let mut sequence = 0;
    let mut retained_progress = semio_framework_job::RetainedCloneProgress::default();
    let mut context = semio_framework_job::StepContext::new(
        semio_framework_job::OperationId(1),
        semio_framework_job::Generation(1),
        semio_framework_job::StepBudget::new(fuel, u64::MAX, ENGINE_CLOSE_GRANT),
        semio_framework_job::root_cancel_token(),
        semio_framework_job::default_now_us,
        &mut sequence,
        &mut retained_progress,
    );
    step(&mut context)
}

/// 📦️ Runs one funded close step of a prepared engine canvas packet and reports whether it reached its exact terminal release.
pub fn close_engine_canvas_packet_step(packet: &mut crate::prepared_engine_canvas::EngineCanvasPacket) -> bool {
    with_engine_close_context(1, |context| {
        matches!(packet.close_step(context).expect("packet close stays inside its funded original context"), semio_framework_job::InteractiveJobCloseStep::Complete { .. })
    })
}
