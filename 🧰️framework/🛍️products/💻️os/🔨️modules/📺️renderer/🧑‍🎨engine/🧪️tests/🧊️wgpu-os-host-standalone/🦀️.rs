#[cfg(test)]
fn terminal_os_host_retirement_state() -> OsHostRetirementState {
    OsHostRetirementState {
        runtime: None,
        presenter: None,
        scheduler: None,
        clock: None,
        caret: None,
        hot_swap: None,
        events: None,
        ui_token: None,
        snapshot_sink: None,
        frame_build: None,
        surface_resize: None,
        engine_surfaces: PairedEngineSurfaceClose {
            operation: semio_framework_trace::allocate_operation_id(),
            sequence: 0,
            scan: crate::engine_canvas::ENGINE_SURFACE_CAPACITY,
            token: None,
            cpu_present: false,
            gpu_present: false,
            phase: PairedEngineSurfaceClosePhase::Terminal,
            faulted: false,
        },
        raster_uploads: None,
        cursor_wake_requested: None,
        #[cfg(not(target_arch = "wasm32"))]
        kernel_progress_close: None,
    }
}

#[cfg(test)]
#[test]
fn interrupted_host_retirement_is_rediscovered_and_fixed_registry_refuses_max_plus_one() {
    let token = reserve_os_host_retirement_abandonment().expect("fixed host retirement reservation");
    let stale = OsHostRetirementAbandonment { slot: token.slot, generation: token.generation.checked_add(1).expect("test generation") };
    assert!(!release_os_host_retirement_abandonment(stale));
    drop(OsHostRetirement { state: Some(Box::new(terminal_os_host_retirement_state())), abandonment: Some(token) });
    assert_eq!(OS_HOST_RETIREMENT_ABANDONMENT_OCCUPIED.load(Ordering::Acquire), 1);
    let mut turns = 0usize;
    while !OsHostRetirement::close_abandoned_step() {
        turns += 1;
        assert!(turns <= OS_HOST_RETIREMENT_ABANDONMENT_CAPACITY);
    }
    assert_eq!(OS_HOST_RETIREMENT_ABANDONMENT_OCCUPIED.load(Ordering::Acquire), 0);
    let mut reservations = [None; OS_HOST_RETIREMENT_ABANDONMENT_CAPACITY];
    for reservation in &mut reservations {
        *reservation = reserve_os_host_retirement_abandonment();
        assert!(reservation.is_some());
    }
    assert!(reserve_os_host_retirement_abandonment().is_none());
    for reservation in reservations.into_iter().flatten() {
        assert!(release_os_host_retirement_abandonment(reservation));
    }
}
