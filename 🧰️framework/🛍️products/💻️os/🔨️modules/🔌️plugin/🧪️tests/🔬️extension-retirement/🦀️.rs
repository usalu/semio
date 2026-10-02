mod extension_retirement_tests {
    use super::*;
    use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

    struct Resource {
        value: Option<String>,
        close: semio_framework_value::retained_clone::RetainedCloneClose,
        sealed: Arc<AtomicBool>,
        terminal: Arc<AtomicBool>,
        expected_bytes: usize,
        released_bytes: usize,
    }

    impl ExtensionResourceOwner for Resource {
        fn invoke(&self, _capability: &str, request: &[u8]) -> Result<Vec<u8>, Fault> { Ok(request.to_vec()) }
        fn begin_close(&mut self) { self.sealed.store(true, Ordering::SeqCst); }
        fn close_step(&mut self, items: usize, bytes: usize) -> Result<PluginCloseStep, Fault> {
            if let Some(step) = self.close.begin_option(&mut self.value, items).map_err(plugin_internal_fault)? { return Ok(extension_retirement::snapshot_close_step(step)); }
            let step = self.close.step(items, bytes).map_err(plugin_internal_fault)?;
            if let store::os_store::SnapshotRetirementStep::Pending { released_bytes, .. } = step { self.released_bytes += released_bytes; }
            if step == store::os_store::SnapshotRetirementStep::Complete { assert_eq!(self.released_bytes, self.expected_bytes); self.terminal.store(true, Ordering::SeqCst); }
            Ok(extension_retirement::snapshot_close_step(step))
        }
        fn terminal_is_empty(&self) -> bool { self.terminal.load(Ordering::SeqCst) }
    }

    fn fixture_bundle(id: &str, fixture: &serde_json::Value) -> (ExtensionBundle, Arc<AtomicBool>, Arc<AtomicBool>) {
        let sealed = Arc::new(AtomicBool::new(false));
        let terminal = Arc::new(AtomicBool::new(false));
        let resource = Resource { value: Some(fixture["resource"].as_str().unwrap().into()), close: Default::default(), sealed: sealed.clone(), terminal: terminal.clone(), expected_bytes: fixture["resourceBytes"].as_u64().unwrap() as usize, released_bytes: 0 };
        let bundle = ExtensionBundle::new(id, id, "1").resource_owner(resource).owned_handler(fixture["capability"].as_str().unwrap()).contributes_topic("metadata", store::DslValue::from(&fixture["metadata"]));
        (bundle, sealed, terminal)
    }

    fn extension_fixture() -> serde_json::Value {
        let text = include_str!("../../🧫️fixtures/🧩️extension-retirement/🔣️.json");
        let oracle: serde_json::Value = serde_json::from_str(text).unwrap();
        let first_party = store::DslValue::from(&oracle);
        assert_eq!(serde_json::Value::from(first_party), oracle);
        oracle
    }

    fn drain_bundle(bundle: &mut ExtensionBundle, fixture: &serde_json::Value) {
        for _ in 0..10000 {
            let grant = &fixture["handlerGrant"];
            let items = grant[0].as_u64().unwrap() as usize;
            let bytes = grant[1].as_u64().unwrap() as usize;
            match bundle.close_step(items,bytes).unwrap() {
                PluginCloseStep::Pending { released_items, released_bytes } => { assert!(released_items <= items); assert!(released_bytes <= bytes); }
                PluginCloseStep::Complete => { assert!(bundle.terminal_is_empty()); return; }
                step => panic!("unexpected close status {step:?}"),
            }
        }
        panic!("retained bundle close did not finish");
    }

    #[test]
    fn extension_bundle_resource_retirement_preserves_zero_cancel_progress_and_terminal_owner() {
        let fixture = extension_fixture();
        let (mut bundle,sealed,terminal) = fixture_bundle("old", &fixture);
        assert_eq!(bundle.invoke("echo",b"owned request").unwrap(),b"owned request");
        for grant in fixture["zeroGrants"].as_array().unwrap() {
            assert_eq!(bundle.close_step(grant[0].as_u64().unwrap() as usize,grant[1].as_u64().unwrap() as usize).unwrap(),PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
            assert!(!sealed.load(Ordering::SeqCst));
        }
        bundle.begin_close();
        assert!(sealed.load(Ordering::SeqCst));
        assert_eq!(bundle.invoke("echo",b"request").unwrap_err().code.0.as_str(),fixture["faults"]["sealed"].as_str().unwrap());
        bundle.cancel_close();
        assert!(matches!(bundle.close_step(1,4).unwrap(),PluginCloseStep::Blocked { .. }));
        assert!(!terminal.load(Ordering::SeqCst));
        bundle.resume_close();
        let grant = &fixture["grant"];
        assert!(matches!(bundle.close_step(grant[0].as_u64().unwrap() as usize,grant[1].as_u64().unwrap() as usize).unwrap(),PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }));
        drain_bundle(&mut bundle,&fixture);
        assert!(terminal.load(Ordering::SeqCst));
        assert_eq!(bundle.close_step(0,0).unwrap(),PluginCloseStep::Complete);
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
            registry.close_step(1,1024).unwrap();
            if registry.current().is_some_and(|bundle| bundle.manifest.extension_id == "new") { break; }
        }
        assert!(old_terminal.load(Ordering::SeqCst));
        assert!(!new_terminal.load(Ordering::SeqCst));
        assert_eq!(registry.invoke("echo",b"new").unwrap(),b"new");
        registry.begin_close();
        for _ in 0..10000 { registry.close_step(1,1024).unwrap(); if registry.terminal_is_empty() { break; } }
        assert!(registry.terminal_is_empty());
        assert!(new_terminal.load(Ordering::SeqCst));
        let mut third = refused.take().unwrap();
        third.begin_close();
        drain_bundle(&mut third,&fixture);
    }

    struct HostileResource { complete: bool, recover: Arc<AtomicBool> }
    impl ExtensionResourceOwner for HostileResource {
        fn invoke(&self, _capability: &str, _request: &[u8]) -> Result<Vec<u8>, Fault> { Ok(Vec::new()) }
        fn begin_close(&mut self) {}
        fn close_step(&mut self, items: usize, bytes: usize) -> Result<PluginCloseStep, Fault> {
            Ok(if self.complete || self.recover.load(Ordering::SeqCst) { PluginCloseStep::Complete } else { PluginCloseStep::Pending { released_items: items + 1, released_bytes: bytes + 1 } })
        }
        fn terminal_is_empty(&self) -> bool { self.recover.load(Ordering::SeqCst) }
    }

    struct TerminalOnStep { terminal: bool, dropped: Arc<AtomicBool>, exact_grant: bool }
    impl ExtensionResourceOwner for TerminalOnStep {
        fn invoke(&self, _capability: &str, _request: &[u8]) -> Result<Vec<u8>, Fault> { Ok(Vec::new()) }
        fn begin_close(&mut self) {}
        fn close_step(&mut self, items: usize, bytes: usize) -> Result<PluginCloseStep, Fault> {
            self.terminal = true;
            Ok(if self.exact_grant { PluginCloseStep::Pending { released_items: items, released_bytes: bytes } } else { PluginCloseStep::Complete })
        }
        fn terminal_is_empty(&self) -> bool { self.terminal }
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
            assert_eq!(bundle.close_step(1,4).unwrap_err().code.0.as_str(),fixture["faults"][key].as_str().unwrap());
            assert!(bundle.resource_owner.is_some());
            assert!(!bundle.terminal_is_empty());
            recover.store(true,Ordering::SeqCst);
            drain_bundle(&mut bundle,&fixture);
        }
        for exact_grant in [false,true] {
            let dropped = Arc::new(AtomicBool::new(false));
            let mut bundle = ExtensionBundle::new("exact-grant", "Exact Grant", "1").resource_owner(TerminalOnStep { terminal: false, dropped: dropped.clone(), exact_grant });
            let step = bundle.close_step(1,4).unwrap();
            assert_eq!(step,if exact_grant { PluginCloseStep::Pending { released_items: 1, released_bytes: 4 } } else { PluginCloseStep::Pending { released_items: 0, released_bytes: 0 } });
            assert!(bundle.resource_owner.is_some());
            assert!(!dropped.load(Ordering::SeqCst));
            assert!(matches!(bundle.close_step(1,4).unwrap(),PluginCloseStep::AwaitingInput { .. }));
            assert!(!dropped.load(Ordering::SeqCst));
            let shell_bytes = std::alloc::Layout::new::<TerminalOnStep>().size();
            assert_eq!(bundle.close_step(1,shell_bytes).unwrap(),PluginCloseStep::Pending { released_items: 1, released_bytes: shell_bytes });
            assert!(dropped.load(Ordering::SeqCst));
            drain_bundle(&mut bundle,&fixture);
        }
    }

    #[test]
    fn extension_bundle_resource_retirement_refuses_implicit_live_drop_and_disposes_cold_explicitly() {
        let fixture = extension_fixture();
        let dropped = Arc::new(AtomicBool::new(false));
        let mut bundle = ExtensionBundle::new("drop-boundary", "Drop Boundary", "1").resource_owner(TerminalOnStep { terminal: false, dropped: dropped.clone(), exact_grant: false }).owned_handler("echo").contributes_topic("metadata", store::DslValue::from(&fixture["metadata"]));
        let address = &mut bundle as *mut ExtensionBundle;
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe { std::ptr::drop_in_place(address) }));
        let panic = result.unwrap_err();
        assert_eq!(panic.downcast_ref::<String>().map(String::as_str).or_else(|| panic.downcast_ref::<&str>().copied()), fixture["dropBoundary"]["fault"].as_str());
        assert_eq!(usize::from(dropped.load(Ordering::SeqCst)), fixture["dropBoundary"]["liveOwnerDrops"].as_u64().unwrap() as usize);
        assert!(!bundle.manifest.topic_contributions.is_empty());
        assert_eq!(bundle.invoke("echo", b"retained").unwrap(), Vec::<u8>::new());
        bundle.begin_close();
        drain_bundle(&mut bundle,&fixture);
        assert_eq!(usize::from(dropped.load(Ordering::SeqCst)), fixture["dropBoundary"]["terminalOwnerDrops"].as_u64().unwrap() as usize);
        drop(bundle);
        let mut cold = ExtensionBundle::new("cold", "Cold", "1").contributes_topic("metadata", store::DslValue::from(&fixture["metadata"]));
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
            fn invoke(&self, _: &str, _: &[u8]) -> Result<Vec<u8>, Fault> { Ok(Vec::new()) }
            fn begin_close(&mut self) {}
            fn close_step(&mut self, _: usize, _: usize) -> Result<PluginCloseStep, Fault> { Ok(PluginCloseStep::Complete) }
            fn terminal_is_empty(&self) -> bool { true }
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
        assert!(matches!(bundle.close_step(1,tiny).unwrap(),PluginCloseStep::AwaitingInput { .. }));
        assert!(!dropped.load(Ordering::SeqCst));
        assert_eq!(bundle.close_step(1,shell_bytes).unwrap(),PluginCloseStep::Pending { released_items: 1, released_bytes: shell_bytes });
        assert!(dropped.load(Ordering::SeqCst));
        drain_bundle(&mut bundle,&fixture);
        use super::extension_retirement::{MetadataOwner, MetadataRetirement, PendingMetadata};
        struct Pod(u64);
        impl MetadataOwner for Pod { fn expand(self: Box<Self>, _: &mut MetadataRetirement) -> Result<(), String> { let _ = self.0; Ok(()) } }
        let capacity = allocation["vectorCapacity"].as_u64().unwrap() as usize;
        assert_eq!(std::mem::size_of::<Pod>(),allocation["podWidth"].as_u64().unwrap() as usize);
        let values: Vec<Pod> = Vec::with_capacity(capacity);
        let expected = std::alloc::Layout::array::<Pod>(values.capacity()).unwrap().size() + std::alloc::Layout::new::<Vec<Pod>>().size() + std::alloc::Layout::new::<PendingMetadata>().size();
        let mut metadata = MetadataRetirement::default();
        metadata.push(values);
        assert_eq!(metadata.next_close_byte_demand(),expected);
        for (items,bytes) in [(0,expected),(1,0),(1,tiny),(1,expected-1)] {
            let step = metadata.step(items,bytes).unwrap();
            if items == 0 || bytes == 0 { assert_eq!(step,PluginCloseStep::Pending { released_items: 0, released_bytes: 0 }); }
            else { assert!(matches!(step,PluginCloseStep::AwaitingInput { .. })); }
            assert!(!metadata.is_empty());
            assert_eq!(metadata.next_close_byte_demand(),expected);
        }
        assert_eq!(metadata.step(1,expected).unwrap(),PluginCloseStep::Pending { released_items: 1, released_bytes: expected });
        assert!(metadata.is_empty());
        assert_eq!(metadata.step(0,0).unwrap(),PluginCloseStep::Complete);
        let mut text = String::with_capacity(allocation["stringCapacity"].as_u64().unwrap() as usize);
        text.push_str(fixture["resource"].as_str().unwrap());
        let text_bytes = text.capacity() + std::alloc::Layout::new::<String>().size() + std::alloc::Layout::new::<PendingMetadata>().size();
        let mut values = Vec::with_capacity(capacity);
        values.push(text);
        let vector_bytes = std::alloc::Layout::array::<String>(values.capacity()).unwrap().size() + std::alloc::Layout::new::<Vec<String>>().size() + std::alloc::Layout::new::<PendingMetadata>().size();
        metadata.push(values);
        let expansion_bytes = std::alloc::Layout::new::<Vec<String>>().size() + std::alloc::Layout::new::<PendingMetadata>().size();
        assert_eq!(metadata.step(1,expansion_bytes).unwrap(),PluginCloseStep::Pending { released_items: 1, released_bytes: expansion_bytes });
        assert_eq!(metadata.next_close_byte_demand(),text_bytes);
        assert!(matches!(metadata.step(1,tiny).unwrap(),PluginCloseStep::AwaitingInput { .. }));
        assert_eq!(metadata.step(1,text_bytes).unwrap(),PluginCloseStep::Pending { released_items: 1, released_bytes: text_bytes });
        assert_eq!(metadata.next_close_byte_demand(),vector_bytes);
        assert!(matches!(metadata.step(1,vector_bytes-1).unwrap(),PluginCloseStep::AwaitingInput { .. }));
        assert_eq!(metadata.step(1,vector_bytes).unwrap(),PluginCloseStep::Pending { released_items: 1, released_bytes: vector_bytes });
        assert!(metadata.is_empty());
        let mut cold = ExtensionBundle::new("inline", "Inline", "1").handler("echo", |bytes| Ok(bytes.to_vec()));
        assert_eq!(cold.invoke("echo",b"unchanged").unwrap(),b"unchanged");
        cold.dispose_cold().unwrap();
        assert_eq!(cold.handlers.capacity(),0);
        assert!(cold.metadata_retirement.is_empty());
        assert!(cold.resource_owner.is_none());
        let mut registry = ExtensionBundleRegistry::new();
        assert!(registry.install(&mut Some(cold)).unwrap());
        assert!(registry.current().unwrap().terminal_is_empty());
        assert_eq!(registry.close_step(1,tiny).unwrap(),PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        assert!(registry.terminal_is_empty());
    }

    #[test]
    fn extension_bundle_resource_retirement_runs_through_suspend_and_exported_poll() {
        semio_framework::io::resolve_ready(async {
            let fixture = extension_fixture();
            let (bundle, sealed, terminal) = fixture_bundle("poll", &fixture);
            let mut candidate = Some(bundle);
            assert!(install_extension_bundle(&mut candidate).await.unwrap());
            extension_activate().await.unwrap();
            let runtime = PluginRuntime::<crate::app::NoPluginApp>::new();
            let budget = semio_framework::kernel::Budget { fuel: 0, deadline_ms: 1000, max_effects: 16, max_patch_bytes: 65536, max_frames: 16 };
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
                crate::reactor::poll_kernel(&runtime, vec![event], None, None, budget).await.unwrap();
                assert!(!sealed.load(Ordering::SeqCst));
                assert_eq!(extension_invoke("echo",b"open").await.unwrap(),b"open");
            }
            let first = crate::reactor::poll_kernel(&runtime, vec![semio_framework::kernel::Event::SuspendRequest], None, None, budget).await.unwrap();
            assert!(sealed.load(Ordering::SeqCst));
            assert!(!terminal.load(Ordering::SeqCst));
            assert_eq!(first.status, semio_framework::kernel::TurnStatus::MoreWork);
            assert!(extension_invoke("echo", b"sealed").await.is_err());
            let control = |req, capability: &str| semio_framework::kernel::Event::Request { req: semio_framework::kernel::RequestId(req), from: semio_framework::kernel::MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId("0".into()) }, capability: capability.into(), payload: Vec::new() };
            let paused = crate::reactor::poll_kernel(&runtime, vec![control(1,fixture["controls"]["cancel"].as_str().unwrap())], None, None, semio_framework::kernel::Budget { fuel: 1, deadline_ms: 1000, max_effects: 16, max_patch_bytes: 65536, max_frames: 16 }).await.unwrap();
            assert!(paused.effects.iter().any(|effect| matches!(effect, semio_framework::kernel::Effect::Respond { req, result: semio_framework::kernel::RequestOutcome::Ok(bytes) } if req.0 == 1 && bytes.is_empty())));
            assert!(!terminal.load(Ordering::SeqCst));
            let resumed = crate::reactor::poll_kernel(&runtime, vec![control(2,fixture["controls"]["resume"].as_str().unwrap())], None, None, semio_framework::kernel::Budget { fuel: 1, deadline_ms: 1000, max_effects: 16, max_patch_bytes: 65536, max_frames: 16 }).await.unwrap();
            assert!(resumed.effects.iter().any(|effect| matches!(effect, semio_framework::kernel::Effect::Respond { req, result: semio_framework::kernel::RequestOutcome::Ok(bytes) } if req.0 == 2 && bytes.is_empty())));
            for _ in 0..10000 {
                let turn = crate::reactor::poll_kernel(&runtime, Vec::new(), None, None, semio_framework::kernel::Budget { fuel: 1, deadline_ms: 1000, max_effects: 16, max_patch_bytes: 65536, max_frames: 16 }).await.unwrap();
                if extension_terminal_is_empty() { assert_eq!(turn.status, semio_framework::kernel::TurnStatus::Idle); assert!(terminal.load(Ordering::SeqCst)); break; }
            }
            assert!(extension_terminal_is_empty(),"exported polling did not retire extension resources");
            let allocation = &fixture["actorAllocation"];
            let small_bytes = allocation["smallBytes"].as_u64().unwrap() as u32;
            let adequate_bytes = allocation["adequateBytes"].as_u64().unwrap() as u32;
            let oversized = ExtensionBundle::new("oversized","x".repeat(allocation["stringBytes"].as_u64().unwrap() as usize),"1");
            assert!(install_extension_bundle(&mut Some(oversized)).await.unwrap());
            let small = semio_framework::kernel::Budget { fuel: 1, max_patch_bytes: small_bytes, ..budget };
            crate::reactor::poll_kernel(&runtime,vec![semio_framework::kernel::Event::SuspendRequest],None,None,small).await.unwrap();
            for _ in 0..1000 {
                crate::reactor::poll_kernel(&runtime,Vec::new(),None,None,small).await.unwrap();
                if extension_next_close_byte_demand().unwrap() > small_bytes as usize { break; }
            }
            let demand = extension_next_close_byte_demand().unwrap();
            assert!(demand > small_bytes as usize && demand <= adequate_bytes as usize);
            assert!(matches!(extension_close_step(1,small_bytes as usize).unwrap(),PluginCloseStep::AwaitingInput { .. }));
            assert_eq!(extension_next_close_byte_demand().unwrap(),demand);
            assert!(!extension_terminal_is_empty());
            let zero = semio_framework::kernel::Budget { fuel: 0, max_patch_bytes: adequate_bytes, ..budget };
            crate::reactor::poll_kernel(&runtime,Vec::new(),None,None,zero).await.unwrap();
            assert_eq!(extension_next_close_byte_demand().unwrap(),demand);
            let adequate = semio_framework::kernel::Budget { fuel: 1, max_patch_bytes: adequate_bytes, ..budget };
            extension_cancel_close();
            crate::reactor::poll_kernel(&runtime,Vec::new(),None,None,adequate).await.unwrap();
            assert_eq!(extension_next_close_byte_demand().unwrap(),demand);
            assert!(!extension_terminal_is_empty());
            extension_resume_close();
            crate::reactor::poll_kernel(&runtime,vec![semio_framework::kernel::Event::QuotaChanged { quotas: semio_framework::kernel::QuotaSchema::default() }],None,None,adequate).await.unwrap();
            assert!(extension_next_close_byte_demand().unwrap() < demand);
            assert!(extension_invoke("echo",b"sealed").await.is_err());
            for _ in 0..1000 {
                crate::reactor::poll_kernel(&runtime,Vec::new(),None,None,adequate).await.unwrap();
                if extension_terminal_is_empty() { return; }
            }
            panic!("actual maintenance-sized actor grant did not retire the retained allocation");
        });
    }
}
