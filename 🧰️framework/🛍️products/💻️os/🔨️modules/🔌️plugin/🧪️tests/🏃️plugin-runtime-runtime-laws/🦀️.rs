#[test]
fn debug_runtime_lines_are_silent_when_diagnostics_are_off() {
    assert!(!semio_framework_trace::runtime_diagnostics_enabled());
}

#[test]
fn drive_self_waking_ready_completes_a_plugin_job_yield() {
    crate::plugin_runtime::drive_self_waking_ready(async {
        crate::app::plugin_job_yield_once().await;
    });
}
