/// 🔚️ Close turn, wire input and no-state disposers: the app close turn, the retained wire input, the declared child projection default and the keyed no-state disposers and retirement factories.

    /// 🎟️ Spends exactly the quoted close turn of a hosted app.
    fn app_close_turn<P: PluginApp>(app: &mut P,cx:&mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        let demand = app.close_retirement_demands(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)?;
        let grant = semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().close;
        assert!(grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth,"app demand exceeds its original fixture close policy");
        let step = semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|same,grant|Ok((app.close_step(same,grant)?,app.close_terminal_is_empty())))?;
        let _ = app.take_constructor_receipt();
        assert!(step.progress().is_none_or(|progress| progress.fits(grant)), "close receipt {step:?} exceeds its exact grant {grant:?}");
        Ok(step)
    }

    fn test_retained_wire_input(bus: &ActionBus, bytes: &[u8]) -> (ToolWireAdmission, action_bus::RetainedToolWireInput) {
        let (admission, mut input) = bus.begin_exact_wire(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, TEST_RETAINED_COMMAND_SCHEMA, bytes.len()).expect("test retained command admission");
        for page in bytes.chunks(action_bus::TOOL_WIRE_PAGE_BYTES) {
            input.admit_page(action_bus::ToolWirePage::try_copy_from(page).expect("test retained wire page")).map_err(|(fault, _)| fault).expect("test retained page admission");
        }
        input.seal().expect("test retained wire seal");
        (admission, input)
    }

    #[test]
    fn declared_child_projection_default_preserves_empty_and_full_literal_references() {
        let fixture: Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧩️composition/🔎️projection/🧪️tests/🧫️fixtures/🔣️.json"))).expect("closed projection fixture");
        for case in fixture["cases"].as_array().expect("projection cases") {
            let rows = case["rows"].as_array().expect("declared rows");
            let text = |row: &Value, name: &str| row[name].as_str().expect("literal child field").to_string();
            let snapshot = TestSnapshot { count: i32::MIN, label: "parent\0引用😀".into(), slot: rows.iter().map(|row| store::os_store::artifact_child::ArtifactChild::new(text(row, "childId"), ArtifactRef { artifact_id: text(row, "artifactId"), dialect: ArtifactDialect { artifact_kind: text(row, "kind"), standard: text(row, "standard"), subset: text(row, "subset") } })).collect() };
            let projected = <KeyedTestApp as ArtifactApp>::child_restore_projection(&snapshot).expect("default projects the actual declared snapshot");
            assert_eq!(projected.len(), rows.len());
            assert_eq!(projected.is_empty(), rows.is_empty());
            for (index, row) in rows.iter().enumerate() {
                let (slot, actual) = projected.get(index).expect("full child occurrence");
                assert_eq!(slot, row["slot"].as_str().expect("slot"));
                assert_eq!(actual.child_id, row["childId"].as_str().expect("logical"));
                assert_eq!(actual.artifact_id, row["artifactId"].as_str().expect("addressed"));
                assert_eq!(actual.artifact_kind, row["kind"].as_str().expect("kind"));
                assert_eq!(actual.standard, row["standard"].as_str().expect("standard"));
                assert_eq!(actual.subset, row["subset"].as_str().expect("subset"));
            }
            let structural = semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(&snapshot).expect("independent declared structural projection");
            projected.admit_complete((0..structural.len()).map(|index| structural.get(index).expect("declared row"))).expect("complete declared set");
            if !rows.is_empty() { assert!(projected.admit_complete(std::iter::empty()).is_err()); }
        }
    }

    #[test]
    fn keyed_fixture_no_state_disposers_and_retirement_factories_close_live_owners() {
        use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
        use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
        let page = store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;
        fn close_root<T: Send + Sync + 'static>(factory: std::sync::Arc<dyn store::SnapshotRetirementFactory<T>>, root: T) {
            let page = store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;
            let admission = semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().preparation;
            let (mut retirement, born) = match store::SnapshotRetirementFactory::retire(factory.as_ref(), std::sync::Arc::new(root), admission) {
                Ok(admitted) => admitted,
                Err((error, _)) => panic!("no-state root retirement constructor refused its page grant: {error:?}"),
            };
            assert!(born.fits(admission));
            for _ in 0..4 {
                if store::ErasedSnapshotRetirement::terminal_is_empty(retirement.as_ref()) {
                    return;
                }
                let copy = store::ErasedSnapshotRetirement::next_copy_byte_demand(retirement.as_ref()).expect("quoted copy demand");
                let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: store::ErasedSnapshotRetirement::next_capacity_byte_demand(retirement.as_ref(), copy).expect("quoted capacity demand"), maximum_release_bytes: store::ErasedSnapshotRetirement::next_release_byte_demand(retirement.as_ref()).expect("quoted release demand"), maximum_depth: store::ErasedSnapshotRetirement::next_depth_demand(retirement.as_ref()).expect("quoted depth demand").max(1) };
                let unfunded = store::ErasedSnapshotRetirement::close_step(retirement.as_mut(), RetainedCloneGrant { maximum_items: 0, ..grant }).expect("zero-item turn yields");
                assert_eq!(unfunded.progress(), RetainedCloneProgress::default());
                let step = store::ErasedSnapshotRetirement::close_step(retirement.as_mut(), grant).expect("bounded root retirement");
                assert!(step.progress().fits(grant));
                if matches!(step, store::RetainedCloneStep::Complete(_)) {
                    assert!(store::ErasedSnapshotRetirement::terminal_is_empty(retirement.as_ref()));
                    return;
                }
            }
            panic!("no-state root retirement exceeded its bounded close turns");
        }

        close_root(<KeyedTestApp as ArtifactApp>::build_presence_local_root_retirement_factory().expect("presence local factory"), NoPresence::default());
        close_root(<KeyedTestApp as ArtifactApp>::build_presence_peer_retirement_factory().expect("presence peer factory"), NoPresence::default());
        close_root(<KeyedTestApp as ArtifactApp>::build_transient_local_root_retirement_factory().expect("transient local factory"), NoTransient::default());

        let mut presence = store::PresenceStore::<NoPresence, NoPresenceMutation>::new(NoPresence::default());
        presence.install_local_retirement_factory(<KeyedTestApp as ArtifactApp>::build_presence_local_root_retirement_factory().expect("presence local factory")).expect("presence local factory installs once");
        presence.install_peer_retirement_factory(<KeyedTestApp as ArtifactApp>::build_presence_peer_retirement_factory().expect("presence peer factory")).expect("presence peer factory installs once");
        let mut presence_disposer = <KeyedTestApp as ArtifactApp>::build_presence_store_disposer().expect("presence disposer");
        let presence_grant = fixture_close_grant(presence_disposer.retirement_demands(&presence, page).expect("quoted presence demand"));
        assert_eq!(presence_disposer.close_step(&mut presence, RetainedCloneGrant { maximum_items: 0, ..presence_grant }).expect("zero grant"), PluginLifecycleStep::Progress(RetainedCloneProgress::default()));
        for _ in 0..8 {
            let grant = fixture_close_grant(presence_disposer.retirement_demands(&presence, page).expect("quoted presence demand"));
            let step = presence_disposer.close_step(&mut presence, grant).expect("presence close");
            assert!(step.progress().expect("presence close receipt").fits(grant));
            if matches!(step, PluginLifecycleStep::Complete(_)) {
                break;
            }
        }
        assert!(presence_disposer.terminal_is_empty(&presence));
        assert!(presence.retirement_started() && presence.peers_root().is_empty());

        let mut transient = store::TransientStore::<NoTransient, NoTransientMutation>::new(NoTransient::default());
        let mut transient_disposer = <KeyedTestApp as ArtifactApp>::build_transient_store_disposer().expect("transient disposer");
        let original_transient_root = transient.current_root();
        let original_transient_weak = std::sync::Arc::downgrade(&original_transient_root);
        let original_transient_generation = transient.generation_now();
        let quoted = transient_disposer.retirement_demands(&transient, page).expect("quoted transient demand");
        let exact_grant = fixture_close_grant(quoted);
        assert_eq!(transient_disposer.close_step(&mut transient, RetainedCloneGrant { maximum_items: 0, ..exact_grant }).expect("zero grant"), PluginLifecycleStep::Progress(RetainedCloneProgress::default()));
        assert!(!transient_disposer.terminal_is_empty(&transient));
        for starved in [
            (quoted.copy_bytes != 0).then(|| RetainedCloneGrant { maximum_copy_bytes: quoted.copy_bytes - 1, ..exact_grant }),
            (quoted.capacity_bytes != 0).then(|| RetainedCloneGrant { maximum_capacity_bytes: quoted.capacity_bytes - 1, ..exact_grant }),
            (quoted.release_bytes != 0).then(|| RetainedCloneGrant { maximum_release_bytes: quoted.release_bytes - 1, ..exact_grant }),
        ]
        .into_iter()
        .flatten()
        {
            assert_eq!(transient_disposer.close_step(&mut transient, starved).expect("short transient grant"), PluginLifecycleStep::Progress(RetainedCloneProgress::default()));
            assert_eq!(transient.generation_now(), original_transient_generation);
            assert!(std::sync::Arc::ptr_eq(&original_transient_root, &transient.current_root()));
        }
        for _ in 0..8 {
            if !std::sync::Arc::ptr_eq(&original_transient_root, &transient.current_root()) {
                break;
            }
            let grant = fixture_close_grant(transient_disposer.retirement_demands(&transient, page).expect("quoted transient demand"));
            let step = transient_disposer.close_step(&mut transient, grant).expect("transient owned-store retirement");
            assert!(step.progress().expect("transient receipt").fits(grant));
        }
        assert_eq!(transient.generation_now(), 0);
        assert!(!std::sync::Arc::ptr_eq(&original_transient_root, &transient.current_root()));
        drop(original_transient_root);
        assert!(original_transient_weak.upgrade().is_none());
        for _ in 0..8 {
            if transient_disposer.terminal_is_empty(&transient) {
                break;
            }
            let grant = fixture_close_grant(transient_disposer.retirement_demands(&transient, page).expect("quoted transient demand"));
            let step = transient_disposer.close_step(&mut transient, grant).expect("transient terminal completion");
            assert!(step.progress().expect("transient receipt").fits(grant));
        }
        assert!(transient_disposer.terminal_is_empty(&transient));
        let repeated = fixture_close_grant(transient_disposer.retirement_demands(&transient, page).expect("quoted terminal demand"));
        assert!(matches!(transient_disposer.close_step(&mut transient, repeated).expect("transient repeated complete"), PluginLifecycleStep::Complete(progress) if progress == RetainedCloneProgress::default()));
        transient = store::TransientStore::new(NoTransient::default());
        let drifted_transient_root = transient.current_root();
        assert!(!transient_disposer.terminal_is_empty(&transient));
        assert!(transient_disposer.close_step(&mut transient, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().preparation).is_err(), "transient terminal witness rejects owner drift");
        assert!(std::sync::Arc::ptr_eq(&drifted_transient_root, &transient.current_root()));
    }
