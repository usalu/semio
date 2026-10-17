mod artifact_reserved_tool_job_tests {
    use super::*;

    struct TerminalJob {
        pub terminal: bool,
        pub drops: std::sync::Arc<std::sync::atomic::AtomicUsize>,
    }

    impl Drop for TerminalJob {
        fn drop(&mut self) {
            self.drops.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }
    }

    impl semio_framework_job::InteractiveJob for TerminalJob {
        fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
            semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(semio_framework_job::OriginalTurnFailure::from)
        }

        fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
            semio_framework_plugin_testing_laws::artifact_app_laws::fixture_job_outcome(descriptor)
        }

        fn begin_close(&mut self) {}

        fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
            if self.terminal{return Ok(semio_framework_job::InteractiveJobCloseStep::Complete{progress:Default::default()})}
            semio_framework_job::close_original_retained_step(cx,semio_framework_value::RetirementDemand{depth:1,..Default::default()},|_|{self.terminal=true;Ok(semio_framework_value::RetainedCloneStep::Complete(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()}))})
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
        let mut caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original reserved fixture caller");let mut cx=caller.context();let recipient=cx.original_recipient_identity();
        let frame=std::mem::size_of::<TerminalJob>();
        let grant=RetainedCloneGrant{maximum_items:1,maximum_release_bytes:4096,maximum_depth:1,..Default::default()};
        assert!(frame<=grant.maximum_release_bytes);
        assert_eq!(cx.with_retained_loan(grant,|same|retained.close_step(same)).unwrap(),semio_framework_job::InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,..Default::default()}});
        assert!(!retained.terminal_is_empty());assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst),0);
        for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_release_bytes:frame-1,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{
            let before=cx.retained_progress();let fuel=cx.fuel_remaining();
            let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(denied,|same|retained.close_step(same)).unwrap());
            assert_eq!(step,semio_framework_job::InteractiveJobCloseStep::Pending{progress:Default::default()});assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(!retained.terminal_is_empty());
            assert_eq!(cx.retained_progress(),before);assert_eq!(cx.fuel_remaining(),fuel);assert_eq!(cx.original_recipient_identity(),recipient);
        }
        assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst),0);
        let before=cx.retained_progress();
        let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(grant,|same|retained.close_step(same)).unwrap());
        assert_eq!(step,semio_framework_job::InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,released_bytes:frame,..Default::default()}});assert_eq!((heap.requested_bytes,heap.released_bytes),(0,frame));
        assert_eq!(cx.retained_progress().released_bytes-before.released_bytes,frame);assert_eq!(cx.original_recipient_identity(),recipient);
        assert_eq!(cx.with_retained_loan(grant,|same|retained.close_step(same)).unwrap(),semio_framework_job::InteractiveJobCloseStep::Complete{progress:Default::default()});
        assert!(retained.terminal_is_empty());
        assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 1);
        let fuel=cx.fuel_remaining();drop(cx);caller.retire(fuel).expect("same caller source root close");
    }
}
