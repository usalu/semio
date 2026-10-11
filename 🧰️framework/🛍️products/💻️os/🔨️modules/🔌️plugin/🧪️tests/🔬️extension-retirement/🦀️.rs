mod extension_retirement_tests {
    use super::*;
    use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
    use semio_framework_value::{RetirementDemand, ValueError, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
    use crate::app::PluginLifecycleStep;

    fn echo_step(bytes: Vec<u8>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ExtensionInvokeStep, Fault> {
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: bytes.len(), retained_capacity_bytes: bytes.capacity(), ..Default::default() };
        cx.consume_retained(progress).map_err(|error| semio_framework_diagnostic::FaultFrom::to_fault(&error))?;
        Ok(ExtensionInvokeStep { payload: Some(bytes), retained_progress: progress, refusal: None })
    }

    fn invoked(step: Result<ExtensionInvokeStep, Fault>) -> Result<Vec<u8>, Fault> { Ok(step?.payload.expect("fixture invocation completes in one turn")) }

    struct Resource {
        value: Option<String>,
        close: semio_framework_value::retained_clone::RetainedCloneClose,
        sealed: Arc<AtomicBool>,
        terminal: Arc<AtomicBool>,
        expected_bytes: usize,
        released_bytes: usize,
        born_bytes: usize,
    }

    impl ExtensionResourceOwner for Resource {
        fn invoke(&self, _capability: &str, request: &[u8], cx: &mut semio_framework_job::StepContext<'_>) -> Result<ExtensionInvokeStep, Fault> { echo_step(request.to_vec(), cx) }
        fn begin_close(&mut self) { self.sealed.store(true, Ordering::SeqCst); }
        fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
            let refusal = |error: ValueError| Fault::new(semio_framework::FaultOrigin::Framework, error.kind.as_str(), error.into_message());
            if let Some(step) = self.close.begin_granted(&mut self.value, grant).map_err(refusal)? { self.born_bytes += step.progress().retained_capacity_bytes; return Ok(PluginLifecycleStep::Progress(step.progress())); }
            let step = self.close.step_granted(grant).map_err(refusal)?;
            self.released_bytes += step.progress().released_bytes;
            self.born_bytes += step.progress().retained_capacity_bytes;
            if matches!(step, RetainedCloneStep::Complete(_)) { assert_eq!(self.released_bytes, self.expected_bytes + self.born_bytes); self.terminal.store(true, Ordering::SeqCst); }
            Ok(PluginLifecycleStep::retained(step, self.close.is_empty()))
        }
        fn terminal_is_empty(&self) -> bool { self.terminal.load(Ordering::SeqCst) }
        fn retirement_demands(&self, maximum_body_bytes: usize) -> Result<RetirementDemand, ValueError> {
            Ok(RetirementDemand { copy_bytes: self.close.next_copy_byte_demand()?, capacity_bytes: self.close.next_owner_capacity_byte_demand::<String>(self.value.is_some(), maximum_body_bytes)?, release_bytes: self.close.next_release_byte_demand()?, depth: self.close.next_depth_demand()?.max(usize::from(self.value.is_some())) })
        }
    }

    fn grant_of(items: usize, bytes: usize) -> RetainedCloneGrant {
        RetainedCloneGrant { maximum_items: items, maximum_copy_bytes: bytes, maximum_capacity_bytes: bytes, maximum_release_bytes: bytes, maximum_depth: 4096 }
    }

    fn quoted_grant(demand: RetirementDemand) -> RetainedCloneGrant {
        crate::app::plugin_demand_grant(demand)
    }

    fn yielded(step: PluginLifecycleStep) -> bool {
        step == PluginLifecycleStep::Progress(RetainedCloneProgress::default())
    }

    fn fixture_bundle(id: &str, fixture: &serde_json::Value) -> (ExtensionBundle, Arc<AtomicBool>, Arc<AtomicBool>) {
        let sealed = Arc::new(AtomicBool::new(false));
        let terminal = Arc::new(AtomicBool::new(false));
        let resource = Resource { value: Some(fixture["resource"].as_str().unwrap().into()), close: Default::default(), sealed: sealed.clone(), terminal: terminal.clone(), expected_bytes: fixture["resourceBytes"].as_u64().unwrap() as usize, released_bytes: 0, born_bytes: 0 };
        let bundle = ExtensionBundle::new(id, id, "1").resource_owner(resource).owned_handler(fixture["capability"].as_str().unwrap()).contributes_topic("metadata", semio_framework_value::DslValue::from(&fixture["metadata"]));
        (bundle, sealed, terminal)
    }

    fn extension_fixture() -> serde_json::Value {
        let text = include_str!("../../🧫️fixtures/🧩️extension-retirement/🔣️.json");
        let oracle: serde_json::Value = serde_json::from_str(text).unwrap();
        let first_party = semio_framework_value::DslValue::from(&oracle);
        assert_eq!(serde_json::Value::from(first_party), oracle);
        oracle
    }

    const QUOTED_BODY_BYTES: usize = 4096;

    fn bundle_grant(bundle: &ExtensionBundle) -> RetainedCloneGrant {
        let demand = bundle.retirement_demands(QUOTED_BODY_BYTES).unwrap();
        RetainedCloneGrant { maximum_copy_bytes: demand.copy_bytes.max(QUOTED_BODY_BYTES), ..quoted_grant(demand) }
    }

    fn drain_bundle(bundle: &mut ExtensionBundle) {
        for _ in 0..10000 {
            let grant = bundle_grant(bundle);
            match bundle.close_step(grant).unwrap() {
                PluginLifecycleStep::Progress(progress) => assert!(progress.fits(grant)),
                PluginLifecycleStep::Complete(progress) => { assert!(progress.fits(grant)); assert!(bundle.terminal_is_empty()); return; }
                step => panic!("unexpected close status {step:?}"),
            }
        }
        panic!("retained bundle close did not finish");
    }

    #[test]
    fn extension_bundle_resource_retirement_preserves_zero_cancel_progress_and_terminal_owner() {
        let fixture = extension_fixture();
        let (mut bundle,sealed,terminal) = fixture_bundle("old", &fixture);
        assert_eq!(invoked(bundle.invoke("echo", b"owned request", &mut crate::app::artifact_app_laws::fixture_step_context())).unwrap(),b"owned request");
        for grant in fixture["zeroGrants"].as_array().unwrap() {
            assert!(yielded(bundle.close_step(grant_of(grant[0].as_u64().unwrap() as usize,grant[1].as_u64().unwrap() as usize)).unwrap()));
            assert!(!sealed.load(Ordering::SeqCst));
        }
        bundle.begin_close();
        assert!(sealed.load(Ordering::SeqCst));
        assert_eq!(invoked(bundle.invoke("echo", b"request", &mut crate::app::artifact_app_laws::fixture_step_context())).unwrap_err().code.0.as_str(),fixture["faults"]["sealed"].as_str().unwrap());
        bundle.cancel_close();
        assert!(matches!(bundle.close_step(bundle_grant(&bundle)).unwrap(),PluginLifecycleStep::Blocked { .. }));
        assert!(!terminal.load(Ordering::SeqCst));
        bundle.resume_close();
        let grant = bundle_grant(&bundle);
        assert!(matches!(bundle.close_step(grant).unwrap(),PluginLifecycleStep::Progress(progress) if progress.copied_items == 1 && progress.released_bytes == 0 && progress.fits(grant)));
        drain_bundle(&mut bundle);
        assert!(terminal.load(Ordering::SeqCst));
        assert_eq!(bundle.close_step(grant_of(0,0)).unwrap(),PluginLifecycleStep::Complete(RetainedCloneProgress::default()));
    }

    #[test]
    fn extension_bundle_resource_retirement_replacement_retains_old_and_backpressured_candidates() {
        let fixture = extension_fixture();
        let (old,_,old_terminal) = fixture_bundle("old",&fixture);
        let (new,_,new_terminal) = fixture_bundle("new",&fixture);
        let (third,_,third_terminal) = fixture_bundle("third",&fixture);
        let mut registry = ExtensionBundleRegistry::new();
        let mut candidate = Some(old);
        assert!(registry.install(&mut candidate).unwrap());
        assert!(candidate.is_none());
        registry.activate().unwrap();
        let mut candidate = Some(new);
        assert!(registry.install(&mut candidate).unwrap());
        assert!(!old_terminal.load(Ordering::SeqCst));
        let mut refused = Some(third);
        assert!(!registry.install(&mut refused).unwrap());
        assert_eq!(refused.as_ref().unwrap().manifest.extension_id,"third");
        assert!(!third_terminal.load(Ordering::SeqCst));
        registry.activate().unwrap();
        for _ in 0..10000 {
            registry.close_step(registry_grant(&registry)).unwrap();
            if registry.current().is_some_and(|bundle| bundle.manifest.extension_id == "new") { break; }
        }
        assert!(old_terminal.load(Ordering::SeqCst));
        assert!(!new_terminal.load(Ordering::SeqCst));
        assert_eq!(invoked(registry.invoke("echo", b"new", &mut crate::app::artifact_app_laws::fixture_step_context())).unwrap(),b"new");
        registry.begin_close();
        for _ in 0..10000 { registry.close_step(registry_grant(&registry)).unwrap(); if registry.terminal_is_empty() { break; } }
        assert!(registry.terminal_is_empty());
        assert!(new_terminal.load(Ordering::SeqCst));
        let mut third = refused.take().unwrap();
        third.begin_close();
        drain_bundle(&mut third);
    }

    fn registry_grant(registry: &ExtensionBundleRegistry) -> RetainedCloneGrant {
        let demand = registry.retirement_demands(QUOTED_BODY_BYTES).unwrap();
        RetainedCloneGrant { maximum_copy_bytes: demand.copy_bytes.max(QUOTED_BODY_BYTES), ..quoted_grant(demand) }
    }

    struct HostileResource { complete: bool, recover: Arc<AtomicBool> }
    impl ExtensionResourceOwner for HostileResource {
        fn invoke(&self, _capability: &str, _request: &[u8], cx: &mut semio_framework_job::StepContext<'_>) -> Result<ExtensionInvokeStep, Fault> { echo_step(Vec::new(), cx) }
        fn begin_close(&mut self) {}
        fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
            Ok(if self.complete || self.recover.load(Ordering::SeqCst) { PluginLifecycleStep::Complete(RetainedCloneProgress::default()) } else { PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: grant.maximum_items + 1, ..Default::default() }) })
        }
        fn terminal_is_empty(&self) -> bool { self.recover.load(Ordering::SeqCst) }
        fn retirement_demands(&self, _: usize) -> Result<RetirementDemand, ValueError> { Ok(RetirementDemand::default()) }
    }

    struct TerminalOnStep { terminal: bool, dropped: Arc<AtomicBool>, exact_grant: bool }
    impl ExtensionResourceOwner for TerminalOnStep {
        fn invoke(&self, _capability: &str, _request: &[u8], cx: &mut semio_framework_job::StepContext<'_>) -> Result<ExtensionInvokeStep, Fault> { echo_step(Vec::new(), cx) }
        fn begin_close(&mut self) {}
        fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
            self.terminal = true;
            Ok(if self.exact_grant { PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: grant.maximum_items, copied_bytes: grant.maximum_copy_bytes, retained_capacity_bytes: grant.maximum_capacity_bytes, released_bytes: grant.maximum_release_bytes }) } else { PluginLifecycleStep::Complete(RetainedCloneProgress::default()) })
        }
        fn terminal_is_empty(&self) -> bool { self.terminal }
        fn retirement_demands(&self, _: usize) -> Result<RetirementDemand, ValueError> { Ok(RetirementDemand::default()) }
    }
    impl Drop for TerminalOnStep {
        fn drop(&mut self) { assert!(self.terminal); self.dropped.store(true,Ordering::SeqCst); }
    }

    #[test]
    fn extension_bundle_resource_retirement_rejects_false_terminal_and_exceeded_grants() {
        let fixture = extension_fixture();
        for (complete, key) in [(false, "exceeded"), (true, "falseComplete")] {
            let recover = Arc::new(AtomicBool::new(false));
            let mut bundle = ExtensionBundle::new("hostile", "Hostile", "1").resource_owner(HostileResource { complete, recover: recover.clone() });
            assert_eq!(bundle.close_step(grant_of(1,4)).unwrap_err().code.0.as_str(),fixture["faults"][key].as_str().unwrap());
            assert!(bundle.resource_owner.is_some());
            assert!(!bundle.terminal_is_empty());
            recover.store(true,Ordering::SeqCst);
            drain_bundle(&mut bundle);
        }
        for exact_grant in [false,true] {
            let dropped = Arc::new(AtomicBool::new(false));
            let mut bundle = ExtensionBundle::new("exact-grant", "Exact Grant", "1").resource_owner(TerminalOnStep { terminal: false, dropped: dropped.clone(), exact_grant });
            let step = bundle.close_step(grant_of(1,4)).unwrap();
            assert_eq!(step,PluginLifecycleStep::Progress(if exact_grant { RetainedCloneProgress { copied_items: 1, copied_bytes: 4, retained_capacity_bytes: 4, released_bytes: 4 } } else { RetainedCloneProgress::default() }));
            assert!(bundle.resource_owner.is_some());
            assert!(!dropped.load(Ordering::SeqCst));
            assert!(yielded(bundle.close_step(grant_of(1,4)).unwrap()));
            assert!(!dropped.load(Ordering::SeqCst));
            let shell_bytes = std::alloc::Layout::new::<TerminalOnStep>().size();
            assert_eq!(bundle.retirement_demands(0).unwrap(),RetirementDemand { release_bytes: shell_bytes, depth: 1, ..Default::default() });
            assert_eq!(bundle.close_step(RetainedCloneGrant { maximum_release_bytes: shell_bytes, ..grant_of(1,4) }).unwrap(),PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: shell_bytes, ..Default::default() }));
            assert!(dropped.load(Ordering::SeqCst));
            drain_bundle(&mut bundle);
        }
    }

    #[test]
    fn extension_bundle_resource_retirement_refuses_implicit_live_drop_and_disposes_cold_explicitly() {
        let fixture = extension_fixture();
        let dropped = Arc::new(AtomicBool::new(false));
        let mut bundle = ExtensionBundle::new("drop-boundary", "Drop Boundary", "1").resource_owner(TerminalOnStep { terminal: false, dropped: dropped.clone(), exact_grant: false }).owned_handler("echo").contributes_topic("metadata", semio_framework_value::DslValue::from(&fixture["metadata"]));
        let address = &mut bundle as *mut ExtensionBundle;
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe { std::ptr::drop_in_place(address) }));
        let panic = result.unwrap_err();
        assert_eq!(panic.downcast_ref::<String>().map(String::as_str).or_else(|| panic.downcast_ref::<&str>().copied()), fixture["dropBoundary"]["fault"].as_str());
        assert_eq!(usize::from(dropped.load(Ordering::SeqCst)), fixture["dropBoundary"]["liveOwnerDrops"].as_u64().unwrap() as usize);
        assert!(!bundle.manifest.topic_contributions.is_empty());
        assert_eq!(invoked(bundle.invoke("echo", b"retained", &mut crate::app::artifact_app_laws::fixture_step_context())).unwrap(), Vec::<u8>::new());
        bundle.begin_close();
        drain_bundle(&mut bundle);
        assert_eq!(usize::from(dropped.load(Ordering::SeqCst)), fixture["dropBoundary"]["terminalOwnerDrops"].as_u64().unwrap() as usize);
        drop(bundle);
        let mut cold = ExtensionBundle::new("cold", "Cold", "1").contributes_topic("metadata", semio_framework_value::DslValue::from(&fixture["metadata"]));
        cold.dispose_cold().unwrap();
        assert!(cold.terminal_is_empty());
        drop(cold);
        let manifest = ExtensionBundle::new("inspection", "Inspection", "1").into_manifest_cold().unwrap();
        assert_eq!(manifest.extension_id,"inspection");
    }

    #[test]
    fn extension_bundle_resource_retirement_admits_exact_backing_and_boxed_shell_allocations() {
        struct LargeOwner { padding: [u8;4096], dropped: Arc<AtomicBool> }
        impl ExtensionResourceOwner for LargeOwner {
            fn invoke(&self, _: &str, _: &[u8], cx: &mut semio_framework_job::StepContext<'_>) -> Result<ExtensionInvokeStep, Fault> { echo_step(Vec::new(), cx) }
            fn begin_close(&mut self) {}
            fn close_step(&mut self, _: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> { Ok(PluginLifecycleStep::Complete(RetainedCloneProgress::default())) }
            fn terminal_is_empty(&self) -> bool { true }
            fn retirement_demands(&self, _: usize) -> Result<RetirementDemand, ValueError> { Ok(RetirementDemand::default()) }
        }
        impl Drop for LargeOwner { fn drop(&mut self) { self.dropped.store(true,Ordering::SeqCst); } }
        let fixture = extension_fixture();
        let allocation = &fixture["allocationAdmission"];
        let tiny = allocation["tinyBytes"].as_u64().unwrap() as usize;
        let dropped = Arc::new(AtomicBool::new(false));
        let owner = LargeOwner { padding: [0;4096], dropped: dropped.clone() };
        assert_eq!(owner.padding.len(),allocation["ownerPaddingBytes"].as_u64().unwrap() as usize);
        let shell_bytes = std::alloc::Layout::for_value(&owner).size();
        let mut bundle = ExtensionBundle::new("allocation", "Allocation", "1").resource_owner(owner);
        bundle.begin_close();
        assert_eq!(bundle.retirement_demands(0).unwrap(),RetirementDemand { release_bytes: shell_bytes, depth: 1, ..Default::default() });
        assert!(yielded(bundle.close_step(grant_of(1,tiny)).unwrap()));
        assert!(!dropped.load(Ordering::SeqCst));
        assert_eq!(bundle.close_step(RetainedCloneGrant { maximum_release_bytes: shell_bytes, ..grant_of(1,tiny) }).unwrap(),PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: shell_bytes, ..Default::default() }));
        assert!(dropped.load(Ordering::SeqCst));
        drain_bundle(&mut bundle);
        use super::extension_retirement::{MetadataOwner, MetadataRetirement, PendingMetadata};
        struct Pod(u64);
        impl MetadataOwner for Pod { fn expand(self: Box<Self>, _: &mut MetadataRetirement) -> Result<(), String> { let _ = self.0; Ok(()) } }
        let capacity = allocation["vectorCapacity"].as_u64().unwrap() as usize;
        assert_eq!(std::mem::size_of::<Pod>(),allocation["podWidth"].as_u64().unwrap() as usize);
        let values: Vec<Pod> = Vec::with_capacity(capacity);
        let expected = std::alloc::Layout::array::<Pod>(values.capacity()).unwrap().size() + std::alloc::Layout::new::<Vec<Pod>>().size() + std::alloc::Layout::new::<PendingMetadata>().size();
        let mut metadata = MetadataRetirement::default();
        metadata.push(values);
        assert_eq!(metadata.retirement_demands(),RetirementDemand { release_bytes: expected, depth: 1, ..Default::default() });
        for (items,bytes) in [(0,expected),(1,0),(1,tiny),(1,expected-1)] {
            assert!(yielded(metadata.step(grant_of(items,bytes)).unwrap()));
            assert!(!metadata.is_empty());
            assert_eq!(metadata.retirement_demands(),RetirementDemand { release_bytes: expected, depth: 1, ..Default::default() });
        }
        assert_eq!(metadata.step(quoted_grant(metadata.retirement_demands())).unwrap(),PluginLifecycleStep::Complete(RetainedCloneProgress { copied_items: 1, released_bytes: expected, ..Default::default() }));
        assert!(metadata.is_empty());
        assert_eq!(metadata.step(grant_of(0,0)).unwrap(),PluginLifecycleStep::Complete(RetainedCloneProgress::default()));
        let mut text = String::with_capacity(allocation["stringCapacity"].as_u64().unwrap() as usize);
        text.push_str(fixture["resource"].as_str().unwrap());
        let text_bytes = text.capacity() + std::alloc::Layout::new::<String>().size() + std::alloc::Layout::new::<PendingMetadata>().size();
        let mut values = Vec::with_capacity(capacity);
        values.push(text);
        let vector_bytes = std::alloc::Layout::array::<String>(values.capacity()).unwrap().size() + std::alloc::Layout::new::<Vec<String>>().size() + std::alloc::Layout::new::<PendingMetadata>().size();
        metadata.push(values);
        let expansion_bytes = std::alloc::Layout::new::<Vec<String>>().size() + std::alloc::Layout::new::<PendingMetadata>().size();
        let birth_bytes = expansion_bytes + std::alloc::Layout::new::<String>().size() + std::alloc::Layout::new::<PendingMetadata>().size();
        assert_eq!(metadata.retirement_demands(),RetirementDemand { capacity_bytes: birth_bytes, release_bytes: expansion_bytes, depth: 1, ..Default::default() });
        assert_eq!(metadata.step(quoted_grant(metadata.retirement_demands())).unwrap(),PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: birth_bytes, released_bytes: expansion_bytes, ..Default::default() }));
        assert_eq!(metadata.retirement_demands(),RetirementDemand { release_bytes: text_bytes, depth: 1, ..Default::default() });
        assert!(yielded(metadata.step(grant_of(1,tiny)).unwrap()));
        assert_eq!(metadata.step(quoted_grant(metadata.retirement_demands())).unwrap(),PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: text_bytes, ..Default::default() }));
        assert_eq!(metadata.retirement_demands(),RetirementDemand { release_bytes: vector_bytes, depth: 1, ..Default::default() });
        assert!(yielded(metadata.step(grant_of(1,vector_bytes-1)).unwrap()));
        assert_eq!(metadata.step(quoted_grant(metadata.retirement_demands())).unwrap(),PluginLifecycleStep::Complete(RetainedCloneProgress { copied_items: 1, released_bytes: vector_bytes, ..Default::default() }));
        assert!(metadata.is_empty());
        let mut cold = ExtensionBundle::new("inline", "Inline", "1").handler("echo", |bytes, cx| echo_step(bytes.to_vec(), cx));
        assert_eq!(invoked(cold.invoke("echo", b"unchanged", &mut crate::app::artifact_app_laws::fixture_step_context())).unwrap(),b"unchanged");
        cold.dispose_cold().unwrap();
        assert_eq!(cold.handlers.capacity(),0);
        assert!(cold.metadata_retirement.is_empty());
        assert!(cold.resource_owner.is_none());
        let mut registry = ExtensionBundleRegistry::new();
        assert!(registry.install(&mut Some(cold)).unwrap());
        assert!(registry.current().unwrap().terminal_is_empty());
        assert_eq!(registry.close_step(grant_of(1,tiny)).unwrap(),PluginLifecycleStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        assert!(registry.terminal_is_empty());
    }

    #[test]
    fn extension_bundle_resource_retirement_runs_through_suspend_and_exported_poll() {
        ::semio_framework_async::poll::resolve_ready(async {
            let fixture = extension_fixture();
            let (bundle, sealed, terminal) = fixture_bundle("poll", &fixture);
            let mut candidate = Some(bundle);
            assert!(install_extension_bundle(&mut candidate).await.unwrap());
            extension_activate().await.unwrap();
            let runtime = PluginRuntime::<crate::app::NoPluginApp>::new({ let grant = crate::app::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; crate::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
            let budget = semio_framework::kernel::Budget { retained: crate::app::artifact_app_laws::fixture_retained_turn(), fuel: 0, deadline_ms: 1000, max_effects: 16, max_patch_bytes: 65536, max_frames: 16 };
            for change in fixture["nonClosingChanges"].as_array().unwrap() {
                use semio_framework::kernel::{BrokerCapabilityGrant, CapabilityChange, CapabilityId, CapabilityToken, Event, QuotaSchema};
                let grant = BrokerCapabilityGrant { id: CapabilityId("echo".into()), token: CapabilityToken(42), scope: "fixture".into(), expires_ms: None };
                let event = match change.as_str().unwrap() {
                    "granted" => Event::CapabilityChanged { change: CapabilityChange::Granted { id: grant.id.clone(), grant } },
                    "narrowed" => Event::CapabilityChanged { change: CapabilityChange::Narrowed { id: grant.id.clone(), grant } },
                    "revoked" => Event::CapabilityChanged { change: CapabilityChange::Revoked { id: grant.id } },
                    "quotaChanged" => Event::QuotaChanged { quotas: QuotaSchema::default() },
                    value => panic!("unknown non-closing fixture change {value}"),
                };
                crate::reactor::poll_kernel(&runtime, vec![event], None, None, budget, &mut crate::app::artifact_app_laws::fixture_identity(), &mut crate::app::artifact_app_laws::fixture_step_context()).await.unwrap();
                assert!(!sealed.load(Ordering::SeqCst));
                assert_eq!(invoked(extension_invoke("echo",b"open", &mut crate::app::artifact_app_laws::fixture_step_context()).await).unwrap(),b"open");
            }
            let first = crate::reactor::poll_kernel(&runtime, vec![semio_framework::kernel::Event::SuspendRequest], None, None, budget, &mut crate::app::artifact_app_laws::fixture_identity(), &mut crate::app::artifact_app_laws::fixture_step_context()).await.unwrap();
            assert!(sealed.load(Ordering::SeqCst));
            assert!(!terminal.load(Ordering::SeqCst));
            assert_eq!(first.status, semio_framework::kernel::TurnStatus::MoreWork);
            assert!(invoked(extension_invoke("echo", b"sealed", &mut crate::app::artifact_app_laws::fixture_step_context()).await).is_err());
            let control = |req, capability: &str| semio_framework::kernel::Event::Request { req: semio_framework::kernel::RequestId(req), from: semio_framework::kernel::MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId("0".into()) }, capability: capability.into(), payload: Vec::new() };
            let paused = crate::reactor::poll_kernel(&runtime, vec![control(1,fixture["controls"]["cancel"].as_str().unwrap())], None, None, semio_framework::kernel::Budget { retained: crate::app::artifact_app_laws::fixture_retained_turn(), fuel: 1, deadline_ms: 1000, max_effects: 16, max_patch_bytes: 65536, max_frames: 16 }, &mut crate::app::artifact_app_laws::fixture_identity(), &mut crate::app::artifact_app_laws::fixture_step_context()).await.unwrap();
            assert!(paused.effects.iter().any(|effect| matches!(effect, semio_framework::kernel::Effect::Respond { req, result: semio_framework::kernel::RequestOutcome::Ok(bytes) } if req.0 == 1 && bytes.is_empty())));
            assert!(!terminal.load(Ordering::SeqCst));
            let resumed = crate::reactor::poll_kernel(&runtime, vec![control(2,fixture["controls"]["resume"].as_str().unwrap())], None, None, semio_framework::kernel::Budget { retained: crate::app::artifact_app_laws::fixture_retained_turn(), fuel: 1, deadline_ms: 1000, max_effects: 16, max_patch_bytes: 65536, max_frames: 16 }, &mut crate::app::artifact_app_laws::fixture_identity(), &mut crate::app::artifact_app_laws::fixture_step_context()).await.unwrap();
            assert!(resumed.effects.iter().any(|effect| matches!(effect, semio_framework::kernel::Effect::Respond { req, result: semio_framework::kernel::RequestOutcome::Ok(bytes) } if req.0 == 2 && bytes.is_empty())));
            for _ in 0..10000 {
                let turn = crate::reactor::poll_kernel(&runtime, Vec::new(), None, None, semio_framework::kernel::Budget { retained: crate::app::artifact_app_laws::fixture_retained_turn(), fuel: 1, deadline_ms: 1000, max_effects: 16, max_patch_bytes: 65536, max_frames: 16 }, &mut crate::app::artifact_app_laws::fixture_identity(), &mut crate::app::artifact_app_laws::fixture_step_context()).await.unwrap();
                if extension_terminal_is_empty() { assert_eq!(turn.status, semio_framework::kernel::TurnStatus::Idle); assert!(terminal.load(Ordering::SeqCst)); break; }
            }
            assert!(extension_terminal_is_empty(),"exported polling did not retire extension resources");
            let allocation = &fixture["actorAllocation"];
            let small_bytes = allocation["smallBytes"].as_u64().unwrap() as u32;
            let adequate_bytes = allocation["adequateBytes"].as_u64().unwrap() as u32;
            let oversized = ExtensionBundle::new("oversized","x".repeat(allocation["stringBytes"].as_u64().unwrap() as usize),"1");
            assert!(install_extension_bundle(&mut Some(oversized)).await.unwrap());
            let small = semio_framework::kernel::Budget { retained: crate::app::artifact_app_laws::fixture_retained_turn(), fuel: 1, max_patch_bytes: small_bytes, ..budget };
            crate::reactor::poll_kernel(&runtime,vec![semio_framework::kernel::Event::SuspendRequest],None,None,small, &mut crate::app::artifact_app_laws::fixture_identity(), &mut crate::app::artifact_app_laws::fixture_step_context()).await.unwrap();
            for _ in 0..1000 {
                crate::reactor::poll_kernel(&runtime,Vec::new(),None,None,small, &mut crate::app::artifact_app_laws::fixture_identity(), &mut crate::app::artifact_app_laws::fixture_step_context()).await.unwrap();
                if extension_retirement_demands(small_bytes as usize).unwrap().release_bytes > small_bytes as usize { break; }
            }
            let demand = extension_retirement_demands(small_bytes as usize).unwrap();
            let lifecycle = crate::plugin_runtime::runtime_lifecycle_grant();
            assert!(demand.release_bytes > small_bytes as usize && demand.release_bytes <= adequate_bytes as usize && demand.release_bytes <= lifecycle.maximum_release_bytes);
            assert!(yielded(extension_close_step(RetainedCloneGrant { maximum_release_bytes: small_bytes as usize, ..lifecycle }).unwrap()));
            assert_eq!(extension_retirement_demands(small_bytes as usize).unwrap(),demand);
            assert!(!extension_terminal_is_empty());
            let zero = semio_framework::kernel::Budget { retained: crate::app::artifact_app_laws::fixture_retained_turn(), fuel: 0, max_patch_bytes: adequate_bytes, ..budget };
            crate::reactor::poll_kernel(&runtime,Vec::new(),None,None,zero, &mut crate::app::artifact_app_laws::fixture_identity(), &mut crate::app::artifact_app_laws::fixture_step_context()).await.unwrap();
            assert_eq!(extension_retirement_demands(small_bytes as usize).unwrap(),demand);
            let adequate = semio_framework::kernel::Budget { retained: crate::app::artifact_app_laws::fixture_retained_turn(), fuel: 1, max_patch_bytes: adequate_bytes, ..budget };
            extension_cancel_close();
            crate::reactor::poll_kernel(&runtime,Vec::new(),None,None,adequate, &mut crate::app::artifact_app_laws::fixture_identity(), &mut crate::app::artifact_app_laws::fixture_step_context()).await.unwrap();
            assert_eq!(extension_retirement_demands(small_bytes as usize).unwrap(),demand);
            assert!(!extension_terminal_is_empty());
            extension_resume_close();
            crate::reactor::poll_kernel(&runtime,vec![semio_framework::kernel::Event::QuotaChanged { quotas: semio_framework::kernel::QuotaSchema::default() }],None,None,adequate, &mut crate::app::artifact_app_laws::fixture_identity(), &mut crate::app::artifact_app_laws::fixture_step_context()).await.unwrap();
            assert!(extension_retirement_demands(small_bytes as usize).unwrap().release_bytes < demand.release_bytes);
            assert!(invoked(extension_invoke("echo",b"sealed", &mut crate::app::artifact_app_laws::fixture_step_context()).await).is_err());
            for _ in 0..1000 {
                crate::reactor::poll_kernel(&runtime,Vec::new(),None,None,small, &mut crate::app::artifact_app_laws::fixture_identity(), &mut crate::app::artifact_app_laws::fixture_step_context()).await.unwrap();
                if extension_terminal_is_empty() { return; }
            }
            panic!("the lifecycle release ceiling does not retire the retained allocation under the small actor patch budget");
        });
    }
}
