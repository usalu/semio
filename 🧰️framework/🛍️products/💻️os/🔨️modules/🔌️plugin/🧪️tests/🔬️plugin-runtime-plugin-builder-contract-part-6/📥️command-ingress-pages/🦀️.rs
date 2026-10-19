/// 📥️ Command ingress pages: the page sets and turn slack, the authority driver, terminal status in one turn, out-of-order page refusal and the abandoned ingress owner.

    /// 🧱️ Page counts the page-set law drives. The host splits at `COMMAND_PAGE_MAXIMUM_BYTES`, so an
    /// 8-page command is the shape a `registerBrushMesh`-scale payload reaches — and 67 is the
    /// 272 089-char scoped contributions pack the procedural plugin pushed on 2026-09-14, which a
    /// fixed 64-page ceiling refused outright (`command ingress exceeds 64 pages`), and 80 is past it
    /// by a margin no constant explains.
    const INGRESS_PAGE_SETS: [usize; 6] = [1, 2, 4, 8, 67, 80];

    /// ⏱️ Turns a page set may spend beyond one per page. The reactor wire admits exactly ONE page per
    /// turn, so `pages` is the protocol's own floor; the slack covers a single instance-authority
    /// contention retry (`plugin_exchange` hands the owner back when `try_lock` would block).
    const INGRESS_TURN_SLACK: usize = 1;

    /// 📤️ The production host owner for one multi-page command — the `CommandBatchDriver` a shard keeps
    /// per command, which hands out its next page only once the guest's status has been observed.
    async fn command_page_authority_driver(instance: u32, seq: u64, command: &protocol::AppCommand) -> semio_framework::kernel::CommandBatchDriver {
        let encoded = protocol::encode_app_command(command).await.expect("the fixture command encodes");
        let mut envelopes = semio_framework::kernel::CommandEnvelopeSet::try_new().expect("fixed command batch authority");
        envelopes.try_push(semio_framework::kernel::CommandEnvelope { instance, seq, command: encoded }).unwrap_or_else(|(fault, _)| panic!("admit fixture command: {fault:?}"));
        let batch = semio_framework::kernel::CommandBatch::try_new(seq, envelopes).unwrap_or_else(|(fault, _)| panic!("admit fixture batch: {fault:?}"));
        semio_framework::kernel::CommandBatchDriver::new(seq, batch)
    }

    /// 🚚️ Drives a whole page set to its terminal ingress status exactly as a shard does: hand the owner's
    /// next page (the wire carries at most one per turn), observe the status it produced, repeat. Returns
    /// whether it completed, the turns it spent and the pages the owner declared.
    async fn drive_one_page_set(runtime: &semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<TestRuntimeApps>, driver: &mut semio_framework::kernel::CommandBatchDriver,cx:&mut semio_framework_job::StepContext<'_>) -> (bool, usize, usize) {
        let mut turns = 0;
        let mut declared = 0;
        let mut carried = driver.next_page().expect("the host owner produces its first page");
        loop {
            declared = declared.max(carried.as_ref().map_or(0, |(cursor, _)| cursor.page_count as usize));
            let result = semio_framework_plugin_reactor_turn::poll_kernel(runtime, &mut None, carried.take(), None, command_page_authority_budget(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(), cx).await.expect("one native ingress turn");
            turns += 1;
            match result.command_ingress {
                semio_framework::kernel::CommandIngressStatus::CommandComplete(_) => return (true, turns, declared),
                semio_framework::kernel::CommandIngressStatus::Fault { .. } => return (false, turns, declared),
                ref pending if turns >= INGRESS_TURNS_PER_COMMAND => panic!("a page set did not reach a terminal ingress status in {turns} turns: {pending:?}"),
                ref accepted @ (semio_framework::kernel::CommandIngressStatus::PageAccepted(_) | semio_framework::kernel::CommandIngressStatus::Backpressure(_)) => {
                    driver.observe(accepted, semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES).expect("the host owner accepts the status its own page produced");
                    carried = driver.next_page().expect("the host owner produces its pages");
                }
                _ => {}
            }
        }
    }

    /// ⚖️ LAW: a command's page set reaches its terminal ingress status in ONE TURN PER PAGE — the cost is
    /// the wire's own page count, never the number of internal moves the ingress owner makes.
    ///
    /// 🐢️ Before ticket 26/09/02 wave B24 `plugin_exchange` answered `Pending` after EVERY move of
    /// `PluginCommandIngress::step` — `Encoded → Decoding`, the header, one read per decoded field, then
    /// `Decoded → Ready` — and each `Pending` costs the host a whole turn round trip. A one-page
    /// `CommandText` measured 4.006 turns over 320 commands and an `AppCommand::Command` five, so the real
    /// cost was `pages + k` with `k` set by the command's decode shape rather than by the wire. This law
    /// fails for every page count under that pacing, which is what makes it discriminating.
    #[semio_framework_async_macros::async_test]
    async fn a_command_page_set_reaches_its_terminal_status_in_one_turn_per_page() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original ingress fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let runtime = semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime::<TestRuntimeApps>::new({ let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
        semio_framework_plugin_runtime_instances::inst_descriptor::install_plugin_bundle(&runtime, __semio_plugin_bundle().await.unwrap());
        let instance = 4_022;
        let captured = reactor_native_lifecycle_poll(&runtime, vec![reactor_native_lifecycle_open(instance, 8, "command-page-set".into())]).await.lifecycle_receipt.expect("Captured receipt");
        let semio_framework::kernel::ActorInstanceLifecycleReceipt::Captured { lifetime, .. } = captured else { panic!("open must emit Captured") };
        reactor_native_lifecycle_ack(&runtime, captured).await;
        let mut census = Vec::new();
        for (index, declared) in INGRESS_PAGE_SETS.into_iter().enumerate() {
            let seq = 512 + index as u64;
            let line = "p".repeat((declared - 1) * semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES + 1);
            let mut driver = command_page_authority_driver(instance, seq, &protocol::AppCommand::CommandText { seq, line }).await;
            let (_, turns, pages) = drive_one_page_set(&runtime, &mut driver,cx).await;
            assert_eq!(pages, declared, "the fixture payload must produce exactly {declared} host pages");
            census.push(format!("{declared}→{turns}"));
            assert!(
                turns <= declared + INGRESS_TURN_SLACK,
                "a {declared}-page command spent {turns} turns reaching its terminal ingress status (ceiling {}) — the ingress is stepping a per-move ladder the host has to drive from outside",
                declared + INGRESS_TURN_SLACK
            );
            assert_eq!(semio_framework_plugin_reactor_turn::retained_command_ingress_occupancy(), 0, "a {declared}-page command must leave no retained ingress owner");
        }
        reactor_native_lifecycle_finish(&runtime, lifetime, 9).await;
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original ingress fixture paid terminal retirement");result
    }
    
    
    
    /// 📥️ The cross-language paged command-ingress contract — the rows the kernel's own reassembly law
    /// and the TypeScript host writer's twin read.
    #[derive(serde::Deserialize)]
    struct CommandIngressDisorderFixture {
        disorder: Vec<CommandIngressDisorderRow>,
    }

    #[derive(serde::Deserialize)]
    struct CommandIngressDisorderRow {
        pub name: String,
        pub pages: usize,
        pub deliver: Vec<usize>,
        #[serde(rename = "rejectedDelivery")]
        rejected_delivery: usize,
    }

    /// 📤️ Every page of one command with the cursor the host stamps on it — the bytes come from the real
    /// `encode_app_command`, cut at the page extent exactly as `CommandBatchDriver` cuts them, so a law
    /// may deliver them in an order no production owner would.
    async fn command_ingress_pages_of(instance: u32, seq: u64, command: &protocol::AppCommand) -> Vec<(semio_framework::kernel::CommandPageCursor, semio_framework::kernel::FixedCommandPage)> {
        let encoded = protocol::encode_app_command(command).await.expect("the fixture command encodes");
        let byte_len = encoded.byte_len();
        let mut reader = semio_framework::kernel::PagedCommandReader::new(encoded);
        let mut body = Vec::with_capacity(byte_len);
        for _ in 0..byte_len {
            body.push(reader.read_byte().expect("the encoded command reads back"));
        }
        let page_count = body.len().div_ceil(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES);
        body.chunks(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES)
            .enumerate()
            .map(|(index, chunk)| {
                let cursor = semio_framework::kernel::CommandPageCursor {
                    owner: u64::from(instance),
                    generation: 1,
                    command_index: 0,
                    command_count: 1,
                    instance,
                    seq,
                    kind: body[0],
                    page_index: index as u32,
                    page_count: page_count as u32,
                    item_count: 0,
                    metadata: 0,
                };
                (cursor, semio_framework::kernel::FixedCommandPage::try_copy_from(chunk).expect("a cut page"))
            })
            .collect()
    }

    /// ⚖️ LAW: the guest reassembles a multi-page command in ARRIVAL ORDER and refuses every page that is
    /// not the next one — a duplicate, a skip and a swap all answer `plugin.command-page-order`.
    ///
    /// 🧨️ A stream with no page ceiling is only safe if the reassembly is exact: the ingress admits one
    /// page per turn over an unbounded number of turns, so a retried or reordered turn must not be able
    /// to splice a command together out of the wrong bytes.
    #[semio_framework_async_macros::async_test]
    async fn a_command_page_that_is_not_the_next_one_is_refused_by_its_order() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original ingress fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let fixture: CommandIngressDisorderFixture =
            serde_json::from_str(include_str!("../../../../../../../🔨️modules/📡️replication/📡️wire/🎮️command/📥️ingress/🧪️tests/🧫️fixtures/📄️pages/🔣️.json")).expect("📥️command-ingress-pages fixture parses");
        let runtime = semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime::<TestRuntimeApps>::new({ let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
        semio_framework_plugin_runtime_instances::inst_descriptor::install_plugin_bundle(&runtime, __semio_plugin_bundle().await.unwrap());
        let instance = 4_031;
        let captured = reactor_native_lifecycle_poll(&runtime, vec![reactor_native_lifecycle_open(instance, 8, "command-page-order".into())]).await.lifecycle_receipt.expect("Captured receipt");
        let semio_framework::kernel::ActorInstanceLifecycleReceipt::Captured { lifetime, .. } = captured else { panic!("open must emit Captured") };
        reactor_native_lifecycle_ack(&runtime, captured).await;
        for (index, row) in fixture.disorder.iter().enumerate() {
            let seq = 1_024 + index as u64;
            let line = "p".repeat((row.pages - 1) * semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES + 1);
            let pages = command_ingress_pages_of(instance, seq, &protocol::AppCommand::CommandText { seq, line }).await;
            assert_eq!(pages.len(), row.pages, "row {} must cut into {} pages", row.name, row.pages);
            for (delivery, page_index) in row.deliver.iter().copied().enumerate() {
                let (cursor, page) = pages[page_index].clone();
                let result = semio_framework_plugin_reactor_turn::poll_kernel(&runtime, &mut None, Some((cursor, page)), None, command_page_authority_budget(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(), cx).await.expect("one native ingress turn");
                match (&result.command_ingress, delivery == row.rejected_delivery) {
                    (semio_framework::kernel::CommandIngressStatus::Fault { fault, .. }, true) => {
                        assert_eq!(fault.as_slice(), b"plugin.command-page-order", "row {} delivery {delivery} must be refused by its order", row.name);
                    }
                    (semio_framework::kernel::CommandIngressStatus::PageAccepted(_), false) => {}
                    (observed, _) => panic!("row {} delivery {delivery} (page {page_index}) answered {observed:?}", row.name),
                }
                if delivery == row.rejected_delivery {
                    break;
                }
            }
            assert_eq!(semio_framework_plugin_reactor_turn::retained_command_ingress_occupancy(), 0, "row {} must leave no retained ingress owner behind", row.name);
        }
        reactor_native_lifecycle_finish(&runtime, lifetime, 9).await;
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original ingress fixture paid terminal retirement");result
    }

    /// ⚖️ LAW: an ingress owner the host has stopped driving never answers for the command it IS
    /// driving, and never pins the ingress lane against it.
    ///
    /// 🧭️ Nothing on this wire tells the guest that a command was abandoned — the host's
    /// `CommandBatchDriver` simply stops existing when its slice loop gives up, its wall budget runs
    /// out or its own cleanup tears the command down. The one piece of evidence the guest can read is
    /// a page naming a DIFFERENT command on the same instance, because the host drives one command
    /// per instance at a time. So that page must supersede whatever is retained, and from then on
    /// every status the new command's own driver observes must be one it can acknowledge — including
    /// on the turns the host carries no page at all, which is exactly when a stale owner used to
    /// speak.
    ///
    /// 🐛️ Before the retiring lane the superseded owner stayed in the single used ingress slot: every
    /// page of the new command answered `Backpressure` forever, and every page-less turn answered
    /// `CommandPending` with the STALE cursor, which `validate_cursor` correctly refused as
    /// `plugin.command-cursor-mismatch: … owner is 1, the retained owner's is 554` — the fault that
    /// held every semio-MCP mutation verb (ticket 26/09/18,
    /// `📓️a2-mcp-plugin-host-instance-open.md` §6.1, `📓️r2-reactor-retained-command-owner.md`).
    #[semio_framework_async_macros::async_test]
    async fn an_abandoned_ingress_owner_never_answers_the_command_the_host_is_driving() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original ingress fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let runtime = semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime::<TestRuntimeApps>::new({ let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
        semio_framework_plugin_runtime_instances::inst_descriptor::install_plugin_bundle(&runtime, __semio_plugin_bundle().await.unwrap());
        let instance = 4_024;
        let captured = reactor_native_lifecycle_poll(&runtime, vec![reactor_native_lifecycle_open(instance, 8, "abandoned-ingress-owner".into())]).await.lifecycle_receipt.expect("Captured receipt");
        let semio_framework::kernel::ActorInstanceLifecycleReceipt::Captured { lifetime, .. } = captured else { panic!("open must emit Captured") };
        reactor_native_lifecycle_ack(&runtime, captured).await;

        let abandoned_seq = 4_096;
        let line = "p".repeat(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES + 1);
        let mut abandoned = command_page_authority_driver(instance, abandoned_seq, &protocol::AppCommand::CommandText { seq: abandoned_seq, line }).await;
        let first = abandoned.next_page().expect("the abandoned owner produces its first page").expect("a two-page command has a first page");
        let opened = semio_framework_plugin_reactor_turn::poll_kernel(&runtime, &mut None, Some(first), None, command_page_authority_budget(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(), cx).await.expect("one native ingress turn");
        assert!(
            matches!(opened.command_ingress, semio_framework::kernel::CommandIngressStatus::PageAccepted(_)),
            "the guest accepts the first page of a two-page command: {:?}",
            opened.command_ingress
        );
        assert_eq!(semio_framework_plugin_reactor_turn::retained_command_ingress_occupancy(), 1, "a half-assembled command is retained");
        drop(abandoned);

        let seq = 4_097;
        let mut driver = command_page_authority_driver(instance, seq, &protocol::AppCommand::CommandText { seq, line: "after the abandoned owner".into() }).await;
        let mut completed = false;
        for turn in 0..INGRESS_TURNS_PER_COMMAND {
            let carried = driver.next_page().expect("the host owner produces its pages");
            let result = semio_framework_plugin_reactor_turn::poll_kernel(&runtime, &mut None, carried, None, command_page_authority_budget(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(), cx).await.expect("one native ingress turn");
            let progress = driver.observe(&result.command_ingress, semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES).unwrap_or_else(|fault| {
                panic!("turn {turn}: the guest answered {:?} for the command this host owns as seq {seq}: {}: {}", result.command_ingress, fault.code.0, fault.message)
            });
            match progress {
                semio_framework::kernel::CommandBatchProgress::Complete => {
                    completed = true;
                    break;
                }
                semio_framework::kernel::CommandBatchProgress::Faulted => panic!("turn {turn}: the command the host drives faulted: {:?}", result.command_ingress),
                _ => {}
            }
        }
        assert!(completed, "the command the host drives never completed in {INGRESS_TURNS_PER_COMMAND} turns — the abandoned owner still holds the ingress lane");
        for _ in 0..INGRESS_TURNS_PER_COMMAND {
            if semio_framework_plugin_reactor_turn::retained_command_ingress_occupancy() == 0 {
                break;
            }
            semio_framework_plugin_reactor_turn::poll_kernel(&runtime, &mut None, None, None, command_page_authority_budget(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(), cx).await.expect("one native ingress turn");
        }
        assert_eq!(semio_framework_plugin_reactor_turn::retained_command_ingress_occupancy(), 0, "the superseded owner retires on its own budget instead of pinning the authority");
        reactor_native_lifecycle_finish(&runtime, lifetime, 9).await;
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original ingress fixture paid terminal retirement");result
    }
