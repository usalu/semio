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

        fn close_step(&mut self,grant:semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
            if grant.maximum_items==0||grant.maximum_depth==0{return semio_framework_job::InteractiveJobCloseStep::Pending{progress:Default::default()};}
            self.terminal = true;
            semio_framework_job::InteractiveJobCloseStep::Complete{progress:semio_framework_value::retained_clone::RetainedCloneProgress{copied_items:1,..Default::default()}}
        }

        fn next_close_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
        fn next_close_capacity_byte_demand(&self,_:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
        fn next_close_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
        fn next_close_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(usize::from(!self.terminal))}

        fn terminal_is_empty(&self) -> bool {
            self.terminal
        }
    }

    impl ArtifactReservedJob for TerminalJob {}

    #[test]
    fn erased_dispatch_clone_release_preserves_unique_bounded_job_disposal_authority() {
        let drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let mut retained = ArtifactReservedToolJob::new(TerminalJob { terminal: false, drops: drops.clone() });
        let erased_dispatch_clone = retained.clone();
        drop(erased_dispatch_clone);
        assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 0);
        use semio_framework_job::InteractiveJob;
        use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress};
        let frame=std::mem::size_of::<TerminalJob>();
        let grant=RetainedCloneGrant{maximum_items:1,maximum_release_bytes:frame,maximum_depth:1,..Default::default()};
        assert_eq!(retained.close_step(grant),semio_framework_job::InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,..Default::default()}});
        assert!(!retained.terminal_is_empty());assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst),0);
        for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_release_bytes:frame-1,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{
            let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||retained.close_step(denied));
            assert_eq!(step,semio_framework_job::InteractiveJobCloseStep::Pending{progress:Default::default()});assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(!retained.terminal_is_empty());
        }
        assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst),0);
        let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||retained.close_step(grant));
        assert_eq!(step,semio_framework_job::InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,released_bytes:frame,..Default::default()}});assert_eq!((heap.requested_bytes,heap.released_bytes),(0,frame));
        assert_eq!(retained.close_step(grant),semio_framework_job::InteractiveJobCloseStep::Complete{progress:Default::default()});
        assert!(retained.terminal_is_empty());
        assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 1);
        eprintln!("[DEBUG] original reserved dispatch retains its job under item/depth/release refusals and releases its exact physical frame={frame}");
    }
}
