/// 🎡️ Maintenance rotation, download and ingress: the fair maintenance block report, the segmented download, the command page driver and the long command stream.

    /// 🎡️ The maintenance rotation is fair: one call runs every stage that still has something to give
    /// and reports a blocked stage only once nothing else can progress, so a block is the rotation's
    /// EVENTUAL answer and never the answer of the one call that happens to land on that stage. An
    /// authority that was LOST instead of blocked never reports a block at all, so waiting for it
    /// weakens nothing — it is the same assertion, made where the rotation can actually answer it.
    #[doc(hidden)]
pub fn assert_maintenance_reports_block<A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + Send + 'static>(app: &mut VcsArtifactApp<A, M>, stage: u8, what: &str,cx:&mut semio_framework_job::StepContext<'_>) {
        for _ in 0..MAINTENANCE_STAGES {
            app.maintenance.maintenance_stage = stage;
            match app_maintenance_turn(app,cx) {
                Ok(semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Blocked { .. }) => return,
                Ok(_) => {}
                Err(fault) => panic!("{what}: bounded maintenance faulted: {fault:?}"),
            }
        }
        panic!("{what} never reported its exact block");
    }

    #[semio_framework_async_macros::async_test]
    async fn segmented_download_remains_addressable_until_terminal_none_is_observed() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app_under_test(mounted_policy, &mut identity).await;
        let chunks = ArtifactOutputChunks::new(4);
        assert_eq!(chunks.push(vec![1, 2, 3, 4]), Ok(4));
        assert_eq!(chunks.seal(), Ok(4));
        assert!(app.media.segmented_downloads.insert(91, ArtifactDownloadOutput::new("exact.bin", "application/octet-stream", None, chunks).expect("bounded segmented output")).is_ok());
        assert_eq!(app.take_segmented_download_chunk(91).await.expect("last chunk"), Some(vec![1, 2, 3, 4]));
        assert!(app.media.segmented_downloads.contains(91));
        assert_eq!(app.take_segmented_download_chunk(91).await.expect("terminal none"), None);
        assert!(!app.media.segmented_downloads.contains(91));
        assert_eq!(app.take_segmented_download_chunk(91).await.expect_err("terminal none removes authority").code.0, "interactive-job.unknown-segmented-download");
        artifact_app_laws::close_registered_fixture_app(&mut *app, mounted_policy,cx).expect("original contract fixture paid terminal close");

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    // 📄️ What ONE command costs the guest's command-ingress prologue, and what a saturated page
    // authority does instead of trapping.
    //
    // Build #29 of ticket 26/09/02/PUZZLE-3D-END-TO-END ran Fill for 184 `fillBuildTick` commands
    // (~44 s at 4 Hz) and then answered every single one with
    // `plugin.command-page-allocation: fixed command page authority could not reserve its exact 64
    // slots`, thirty-two times in a row, before the guest trapped on `unreachable`. That fault is
    // `CommandPageSet::try_new`'s `try_reserve_exact` refusing — an ALLOCATION refusal, not the
    // saturation refusal (`plugin.command-page-count`) — so the guest's one fixed 512 MiB linear memory
    // could no longer hand out the block the prologue asked for.
    //
    // The block was 262 272 B: `COMMAND_MAXIMUM_PAGES * size_of::<FixedCommandPage>()`, reserved for
    // EVERY command regardless of how many pages that command declares, four times a second, for
    // commands that are one page of a few hundred bytes. The guest allocator grows its memory in 64 KiB
    // units and never shrinks it, so a quarter-megabyte contiguous request is the first thing a
    // fragmented or nearly-full guest refuses — which is exactly the order the browser observed.
    //
    // The laws below pin both halves: the reservation is the command's DECLARED page count, and every
    // refusal on this path is a `Fault` the host can display rather than a panic.

    /// 🔁️ Commands driven through the real ingress — over the 184 the browser reached before the
    /// prologue started refusing, so a per-command owner the authority never gives back shows up here.
    const INGRESS_COMMANDS: u64 = 320;

    /// 🧮️ Retained bytes one command may leave behind. The authority is transient by construction: its
    /// pages are released as the command is assembled and its deque dies with the command.
    const INGRESS_COMMAND_RETENTION_CEILING_BYTES: isize = 512;
    
    /// 📤️ Encodes one `AppCommand` into the exact page and cursor a host hands the guest — the bytes and
    /// the header come from `encode_app_command` and `CommandBatchDriver`, production code, not from a
    /// hand-written wire literal.
    async fn command_page_authority_page(instance: u32, seq: u64, command: &protocol::AppCommand) -> (semio_framework::kernel::CommandPageCursor, semio_framework::kernel::FixedCommandPage) {
        let encoded = protocol::encode_app_command(command).await.expect("the fixture command encodes");
        let mut envelopes = semio_framework::kernel::CommandEnvelopeSet::try_new().expect("fixed command batch authority");
        envelopes.try_push(semio_framework::kernel::CommandEnvelope { instance, seq, command: encoded }).unwrap_or_else(|(fault, _)| panic!("admit fixture command: {fault:?}"));
        let batch = semio_framework::kernel::CommandBatch::try_new(seq, envelopes).unwrap_or_else(|(fault, _)| panic!("admit fixture batch: {fault:?}"));
        let mut driver = semio_framework::kernel::CommandBatchDriver::new(seq, batch);
        let page = driver.next_page().expect("the host owner produces its page").expect("a nonempty command has a first page");
        assert_eq!(page.0.page_count, 1, "the fixture command is one page, which is what the 4 Hz stream that broke build #29 was");
        page
    }

    /// 🚚️ Drives one command to its terminal ingress status through the real `poll_kernel`: page 0 on the
    /// first turn, then plain turns while the guest steps the owner it retained, until it answers
    /// `CommandComplete` or `Fault`. Returns whether it completed and the turns it took.
    async fn drive_one_command(runtime: &semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<TestRuntimeApps>, page: (semio_framework::kernel::CommandPageCursor, semio_framework::kernel::FixedCommandPage),cx:&mut semio_framework_job::StepContext<'_>) -> (bool, usize) {
        let mut carried = Some(page);
        let mut turns = 0;
        loop {
            let result = semio_framework_plugin_reactor_turn::poll_kernel(runtime, &mut None, carried.take(), None, command_page_authority_budget(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(), cx).await.expect("one native ingress turn");
            turns += 1;
            match result.command_ingress {
                semio_framework::kernel::CommandIngressStatus::CommandComplete(_) => return (true, turns),
                semio_framework::kernel::CommandIngressStatus::Fault { .. } => return (false, turns),
                ref pending if turns >= INGRESS_TURNS_PER_COMMAND => panic!("one command did not reach a terminal ingress status in {turns} turns: {pending:?}"),
                _ => {}
            }
        }
    }

    /// ⚖️ LAW: a long command stream leaves the guest's retained ingress authority empty. Every command
    /// is dispatched and closed within its own turns; nothing pins a slot for a later command to queue
    /// behind, and the run retains no per-command heap.
    ///
    /// 🚚️ The commands go through the real `poll_kernel` ingress the shard drives, with cursors produced
    /// by the real `CommandBatchDriver` — not through a test-only shortcut into the app.
    ///
    /// 🧵️ This thread's own allocator scope — see `retained_heap_bytes_on_this_thread`; a
    /// process-wide difference weighs every concurrent law in the binary as this command's own.
    #[semio_framework_async_macros::async_test]
    async fn a_long_command_stream_never_pins_the_retained_ingress_authority() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original ingress fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let runtime = semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime::<TestRuntimeApps>::new({ let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
        semio_framework_plugin_runtime_instances::inst_descriptor::install_plugin_bundle(&runtime, __semio_plugin_bundle().await.unwrap());
        let instance = 4_021;
        let captured = reactor_native_lifecycle_poll(&runtime, vec![reactor_native_lifecycle_open(instance, 8, "command-page-authority".into())]).await.lifecycle_receipt.expect("Captured receipt");
        let semio_framework::kernel::ActorInstanceLifecycleReceipt::Captured { lifetime, .. } = captured else { panic!("open must emit Captured") };
        reactor_native_lifecycle_ack(&runtime, captured).await;
        for _ in 0..32 {
            reactor_native_lifecycle_poll(&runtime, Vec::new()).await;
        }
        assert_eq!(semio_framework_plugin_reactor_turn::retained_command_ingress_occupancy(), 0, "a settled reactor holds no ingress owner");
        let mut warmed = 0;
        let mut settled = 0;
        let mut faulted = 0;
        let mut peak_occupancy = 0;
        let mut turns = 0;
        for index in 0..INGRESS_COMMANDS {
            let seq = 64 + index;
            let page = command_page_authority_page(instance, seq, &protocol::AppCommand::CommandText { seq, line: format!("noop {seq}") }).await;
            let (completed, spent) = drive_one_command(&runtime, page,cx).await;
            turns += spent;
            if !completed {
                faulted += 1;
            }
            let occupancy = semio_framework_plugin_reactor_turn::retained_command_ingress_occupancy();
            peak_occupancy = peak_occupancy.max(occupancy);
            assert_eq!(occupancy, 0, "command {index} left an owner in the retained ingress authority — every later command queues behind it");
            if warmed == 0 && index + 1 == INGRESS_COMMANDS / 4 {
                warmed = index + 1;
                settled = semio_framework_trace::retained_heap_bytes_on_this_thread();
            }
        }
        let retained = semio_framework_trace::retained_heap_bytes_on_this_thread() - settled;
        let measured = (INGRESS_COMMANDS - warmed) as isize;
        let per_command = retained / measured;
        let census = format!("{INGRESS_COMMANDS} commands, {turns} turns, {faulted} faulted, peak ingress occupancy {peak_occupancy}, {retained} B over the last {measured}");
        assert!(
            per_command <= INGRESS_COMMAND_RETENTION_CEILING_BYTES,
            "the command ingress retains {per_command} B per command (ceiling {INGRESS_COMMAND_RETENTION_CEILING_BYTES}) — the guest's linear memory is fixed and never shrinks: {census}"
        );
        reactor_native_lifecycle_finish(&runtime, lifetime, 9).await;

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original ingress fixture paid terminal retirement");result
    }
