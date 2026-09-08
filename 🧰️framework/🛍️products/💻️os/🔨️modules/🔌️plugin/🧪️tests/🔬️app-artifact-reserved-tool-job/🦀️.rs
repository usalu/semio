mod artifact_reserved_tool_job_tests {
    use super::*;

    struct TerminalJob {
        terminal: bool,
        drops: std::sync::Arc<std::sync::atomic::AtomicUsize>,
    }

    impl Drop for TerminalJob {
        fn drop(&mut self) {
            self.drops.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }
    }

    impl semio_framework_job::InteractiveJob for TerminalJob {
        fn step(&mut self, _cx: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
            semio_framework_job::StepOutcome::Yield
        }

        fn begin_close(&mut self) {}

        fn close_step(&mut self, _maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
            self.terminal = true;
            semio_framework_job::InteractiveJobCloseStep::Complete
        }

        fn terminal_is_empty(&self) -> bool {
            self.terminal
        }
    }

    impl ArtifactReservedJob for TerminalJob {
        fn close_step(&mut self, _maximum_items: usize, _maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
            self.terminal = true;
            Ok(PluginCloseStep::Complete)
        }

        fn terminal_is_empty(&self) -> bool {
            self.terminal
        }
    }

    #[test]
    fn erased_dispatch_clone_release_preserves_unique_bounded_job_disposal_authority() {
        let drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let mut retained = ArtifactReservedToolJob::new(TerminalJob { terminal: false, drops: drops.clone() });
        let erased_dispatch_clone = retained.clone();
        drop(erased_dispatch_clone);
        assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 0);
        assert!(matches!(retained.close_step(1, ARTIFACT_OUTPUT_CHUNK_BYTES), Ok(PluginCloseStep::Complete)));
        assert!(retained.terminal_is_empty());
        assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 1);
    }
}
