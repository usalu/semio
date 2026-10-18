/// 🛂️ Continuation, publication and host crossing: concurrent typed operation pages, the many-mutation publication and a host crossing that outranks background holds.

    /// ⏱️ Continuation turns a staged operation may spend beyond one per acknowledgeable result page —
    /// the terminating turn on which the scan finally answers "nothing runnable".
    const TYPED_OPERATION_CONTINUATION_SLACK: u64 = 1;

    /// 📄️ LAW: k live operations each holding a presentable result page retire in ONE host crossing.
    ///
    /// 🐛️ `advance_typed_operation_output` took exactly one page and `TypedOperationGrant::spent` ended
    /// the turn the instant it existed, so every acknowledgeable publication unit of every live
    /// operation cost its own worker round trip. Served measurement on the React door of 🧊️generation3d
    /// (`📓️shard-message-crossings-2026-09-15.md` §2): 200 acknowledgement crossings over 69
    /// operations, **every one of them exactly one ack wide**, 10.0 of a `flowEvalTick` hop's 36.2
    /// crossings. A page's ACK is per-page by protocol; the crossing that carries it is not — the wire's
    /// `Event::Message` is a list and the reactor's poll acknowledges every entry of it.
    ///
    /// The law is about CARDINALITY, so it admits whatever lane each page carries: four simultaneous
    /// edits of one immutable document root make three of them rebase refusals, and a refusal is
    /// published as its own acknowledgeable `Fault` page on exactly the wire this law measures.
    #[semio_framework_async_macros::async_test]
    async fn concurrent_typed_operations_hand_every_presentable_page_to_one_turn() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("closed finite original reactor builder caller");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        const OPERATIONS: usize = 4;
        let fixture: Value = serde_json::from_str(include_str!("../../../⚛️reactor/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
        let id = fixture["wire"]["receiver"].as_u64().unwrap() as u32;
        let mut app = VcsArtifactApp::<KeyedTestApp>::with_registry(KeyedTestApp, keyed_test_registry().await, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, &mut identity).await;
        semio_framework_plugin_testing_laws::artifact_app_laws::settle_store_constructor(&mut app, mounted_policy);
        app.bind_instance_id(id).await;
        let meta = ActionMeta { actor: "fixture".into(), instance_id: id, view_state: None };
        for index in 0..OPERATIONS {
            let command = TestCommand::CompositeEdit { slot: String::new(), child_id: format!("batch-{index}").into(), child_value: index as i32 + 1 };
            app.dispatch_typed(command, &meta, &mut identity,cx).await.unwrap();
        }
        let runtime = semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime::new({ let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
        let cell = std::sync::Arc::new(semio_framework_plugin_reactor_lifecycle::life_app_cell::RuntimeAppCell::new(AppInstance { id, app, surface_contexts: Default::default() }, runtime.mounted_policy));
        runtime.instances.borrow_mut().insert_admitted(id, cell.clone());
        let (mut pages, mut crossings, mut widest) = (0usize, 0usize, 0usize);
        for _ in 0..fixture["command"]["maximumTurns"].as_u64().unwrap() * OPERATIONS as u64 {
            semio_framework_plugin_runtime_instances::inst_cleanup_turns::plugin_step_live_cleanup(&runtime,cx).unwrap();
            let (output, scan) = semio_framework_plugin_runtime_ops::ops_typed_operations::plugin_continue_typed_operations(&runtime, semio_framework_plugin_runtime_ops::ops_typed_operations::TypedOperationGrant::turn(reactor_native_budget(cx),cx), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(), cx).await.unwrap();
            let more = scan.runnable || scan.contended;
            if let Some((receiver, output)) = output {
                assert_eq!(receiver, id);
                if !output.typed_operation_results.is_empty() {
                    crossings += 1;
                    widest = widest.max(output.typed_operation_results.len());
                }
                for page in output.typed_operation_results {
                    pages += 1;
                    semio_framework_plugin_runtime_ops::ops_typed_operations::plugin_acknowledge_typed_operation_result(&runtime, page.token).await.unwrap();
                }
            }
            if !more {
                break;
            }
            std::thread::yield_now();
        }
        assert!(pages >= OPERATIONS, "each of the {OPERATIONS} admitted operations owes at least one result page, saw {pages}");
        assert!(widest >= 2, "{OPERATIONS} concurrently publishing operations must share a crossing; the widest batch was {widest}");
        assert!(crossings < pages, "{pages} result pages retired in {crossings} crossings — a page per crossing is the cardinality this law removes");
        let active = cell.instance.lock().unwrap();
        assert!(!active.app.has_pending_typed_operations());
        drop(active);
        drop(cell);
        semio_framework_plugin_runtime_instances::inst_cleanup_turns::plugin_destroy_app(&runtime, id).await.unwrap();
        for _ in 0..100_000 {
            semio_framework_plugin_runtime_instances::inst_cleanup_turns::plugin_step_close_cleanup(&runtime,cx).unwrap();
            if runtime.close_quarantine.borrow().get(id).is_none() {
                break;
            }
        }
        }.await;let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("same original reactor builder caller terminal retirement");result
    }

    /// 🧮️ A many-mutation edit publishes every one of its mutations and reaches its terminal page,
    /// spending many publication units under the stall guard without being terminated as stalled.
    ///
    /// 🩺️ The guard (`fault_stalled_typed_operation_publication`) terminates a `Publishing`
    /// operation once 4 096 consecutive units leave its stall witness unchanged, and the witness
    /// had no term for the ladder's own progress. This law is the biggest single document
    /// publication the store admits — `ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES` is 1 MiB and one
    /// unit spends up to `TYPED_OPERATION_RESULT_PAGE_BYTES` of it — which is why it is expressed
    /// on the fold being COMPLETE and fault-free rather than on a unit count: a single batched
    /// document publication cannot reach 4 096 units at all (measured: 4 400 mutations = 126 units,
    /// 150 000 are refused as "batched preparation footprint exceeds its fixed item or byte
    /// capacity").
    #[semio_framework_async_macros::async_test]
    async fn a_many_mutation_publication_folds_every_mutation_and_is_never_terminated_as_stalled() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("closed finite original reactor builder caller");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        const ROWS: i32 = 4_400;
        let id = 4_103;
        let mut app = VcsArtifactApp::<KeyedTestApp>::with_registry(KeyedTestApp, keyed_test_registry().await, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, &mut identity).await;
        semio_framework_plugin_testing_laws::artifact_app_laws::settle_store_constructor(&mut app, mounted_policy);
        app.bind_instance_id(id).await;
        app.dispatch_typed(TestCommand::BulkEdit { rows: ROWS }, &ActionMeta { actor: "fixture".into(), instance_id: id, view_state: None }, &mut identity).await.unwrap();
        let runtime = semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime::new({ let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
        let cell = std::sync::Arc::new(semio_framework_plugin_reactor_lifecycle::life_app_cell::RuntimeAppCell::new(AppInstance { id, app, surface_contexts: Default::default() }, runtime.mounted_policy));
        runtime.instances.borrow_mut().insert_admitted(id, cell.clone());
        let mut terminal = false;
        let mut faults: Vec<String> = Vec::new();
        let mut units = 0u64;
        for _ in 0..(ROWS as u64 * 8) {
            let (output, scan) = semio_framework_plugin_runtime_ops::ops_typed_operations::plugin_continue_typed_operations(&runtime, semio_framework_plugin_runtime_ops::ops_typed_operations::TypedOperationGrant::UNIT, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(), cx).await.unwrap();
            units += 1;
            if let Some((receiver, output)) = output {
                assert_eq!(receiver, id);
                for page in output.typed_operation_results {
                    if page.lane == TypedOperationResultLane::Fault {
                        faults.push(String::from_utf8_lossy(page.bytes()).into_owned());
                    }
                    terminal |= page.lane == TypedOperationResultLane::Terminal;
                    semio_framework_plugin_runtime_ops::ops_typed_operations::plugin_acknowledge_typed_operation_result(&runtime, page.token).await.unwrap();
                }
            }
            if !(scan.runnable || scan.contended) {
                break;
            }
        }
        assert!(faults.is_empty(), "a moving publication ladder must never be terminated as stalled, saw {faults:?} after {units} publication units");
        assert!(terminal, "the {ROWS}-mutation publication owes a terminal result page, saw {units} publication units");
        assert!(units > 64, "this law only exercises the stall guard while the publication spends more units than its 64-unit witness floor, it spent {units}");
        let active = cell.instance.lock().unwrap();
        assert_eq!(active.app.snapshot().unwrap().count, ROWS, "every one of the {ROWS} folded mutations reached the document");
        assert!(!active.app.has_pending_typed_operations());
        drop(active);
        drop(cell);
        semio_framework_plugin_runtime_instances::inst_cleanup_turns::plugin_destroy_app(&runtime, id).await.unwrap();
        for _ in 0..100_000 {
            semio_framework_plugin_runtime_instances::inst_cleanup_turns::plugin_step_close_cleanup(&runtime,cx).unwrap();
            if runtime.close_quarantine.borrow().get(id).is_none() {
                break;
            }
        }
        }.await;let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("same original reactor builder caller terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn a_host_crossing_outranks_every_background_hold_on_the_same_app() {
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let id = 4_101;
        let mut app = VcsArtifactApp::<KeyedTestApp>::with_registry(KeyedTestApp, keyed_test_registry().await, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, &mut identity).await;
        semio_framework_plugin_testing_laws::artifact_app_laws::settle_store_constructor(&mut app, mounted_policy);
        app.bind_instance_id(id).await;
        let runtime = semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime::new({ let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
        let cell = std::sync::Arc::new(semio_framework_plugin_reactor_lifecycle::life_app_cell::RuntimeAppCell::new(AppInstance { id, app, surface_contexts: Default::default() }, runtime.mounted_policy));
        runtime.instances.borrow_mut().insert_admitted(id, cell.clone());
        let background_entered = std::sync::atomic::AtomicBool::new(false);
        std::thread::scope(|scope| {
            let holder = scope.spawn(|| {
                let held = cell.background_instance().expect("an idle app admits a background turn");
                background_entered.store(true, std::sync::atomic::Ordering::SeqCst);
                while cell.host_crossings.load(std::sync::atomic::Ordering::SeqCst) == 0 {
                    std::thread::yield_now();
                }
                drop(held);
            });
            while !background_entered.load(std::sync::atomic::Ordering::SeqCst) {
                std::thread::yield_now();
            }
            assert_eq!(cell.background_holds.load(std::sync::atomic::Ordering::SeqCst), 1, "a live background turn counts its own hold so the host can tell it from an owner that never releases");
            let crossing = cell.host_instance();
            assert!(crossing.is_some(), "a host crossing waits out a background turn instead of being refused over it");
            let crossing = crossing.expect("the host crossing was admitted");
            assert_eq!(cell.host_crossings.load(std::sync::atomic::Ordering::SeqCst), 1);
            assert_eq!(cell.background_holds.load(std::sync::atomic::Ordering::SeqCst), 0);
            assert!(matches!(cell.background_instance(), Err(semio_framework_plugin_reactor_lifecycle::life_app_cell::RuntimeInstanceRefusal::Contended)), "a background turn never competes with a live host crossing");
            drop(crossing);
            holder.join().expect("the background holder released its turn to the host crossing");
        });
        assert_eq!(cell.host_crossings.load(std::sync::atomic::Ordering::SeqCst), 0);
        assert!(cell.background_instance().is_ok(), "the app is free again once the crossing ends");
        drop(cell);
        semio_framework_plugin_runtime_instances::inst_cleanup_turns::plugin_destroy_app(&runtime, id).await.unwrap();
        for _ in 0..100_000 {
            semio_framework_plugin_runtime_instances::inst_cleanup_turns::plugin_step_close_cleanup(&runtime).unwrap();
            if runtime.close_quarantine.borrow().get(id).is_none() {
                break;
            }
        }
    }
