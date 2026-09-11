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

/// ⏱️ Turns one command may take to reach its terminal ingress status before the law calls it stuck.
const INGRESS_TURNS_PER_COMMAND: usize = 512;

fn command_page_authority_budget() -> Budget {
    Budget { fuel: 64, deadline_ms: 1000, max_effects: 16, max_patch_bytes: 65536, max_frames: 16 }
}

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
async fn drive_one_command(runtime: &crate::plugin_runtime::PluginRuntime<TestRuntimeApps>, page: (semio_framework::kernel::CommandPageCursor, semio_framework::kernel::FixedCommandPage)) -> (bool, usize) {
    let mut carried = Some(page);
    let mut turns = 0;
    loop {
        let result = crate::reactor::poll_kernel(runtime, Vec::new(), carried.take(), None, command_page_authority_budget()).await.expect("one native ingress turn");
        turns += 1;
        match result.command_ingress {
            semio_framework::kernel::CommandIngressStatus::CommandComplete(_) => return (true, turns),
            semio_framework::kernel::CommandIngressStatus::Fault { .. } => return (false, turns),
            ref pending if turns >= INGRESS_TURNS_PER_COMMAND => panic!("one command did not reach a terminal ingress status in {turns} turns: {pending:?}"),
            _ => {}
        }
    }
}

/// 🧱️ Page counts the page-set law drives. The host splits at `COMMAND_PAGE_MAXIMUM_BYTES`, so an
/// 8-page command is the shape a `registerBrushMesh`-scale payload reaches.
const INGRESS_PAGE_SETS: [usize; 4] = [1, 2, 4, 8];

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
async fn drive_one_page_set(runtime: &crate::plugin_runtime::PluginRuntime<TestRuntimeApps>, driver: &mut semio_framework::kernel::CommandBatchDriver) -> (bool, usize, usize) {
    let mut turns = 0;
    let mut declared = 0;
    let mut carried = driver.next_page().expect("the host owner produces its first page");
    loop {
        declared = declared.max(carried.as_ref().map_or(0, |(cursor, _)| cursor.page_count as usize));
        let result = crate::reactor::poll_kernel(runtime, Vec::new(), carried.take(), None, command_page_authority_budget()).await.expect("one native ingress turn");
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
    let runtime = crate::plugin_runtime::PluginRuntime::<TestRuntimeApps>::new();
    crate::plugin_runtime::install_plugin_bundle(&runtime, __semio_plugin_bundle().await.unwrap());
    let instance = 4_022;
    let captured = reactor_native_lifecycle_poll(&runtime, vec![reactor_native_lifecycle_open(instance, 8, "command-page-set".into())]).await.lifecycle_receipt.expect("Captured receipt");
    let semio_framework::kernel::ActorInstanceLifecycleReceipt::Captured { lifetime, .. } = captured else { panic!("open must emit Captured") };
    reactor_native_lifecycle_ack(&runtime, captured).await;
    let mut census = Vec::new();
    for (index, declared) in INGRESS_PAGE_SETS.into_iter().enumerate() {
        let seq = 512 + index as u64;
        let line = "p".repeat((declared - 1) * semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES + 1);
        let mut driver = command_page_authority_driver(instance, seq, &protocol::AppCommand::CommandText { seq, line }).await;
        let (_, turns, pages) = drive_one_page_set(&runtime, &mut driver).await;
        assert_eq!(pages, declared, "the fixture payload must produce exactly {declared} host pages");
        census.push(format!("{declared}→{turns}"));
        assert!(
            turns <= declared + INGRESS_TURN_SLACK,
            "a {declared}-page command spent {turns} turns reaching its terminal ingress status (ceiling {}) — the ingress is stepping a per-move ladder the host has to drive from outside",
            declared + INGRESS_TURN_SLACK
        );
        assert_eq!(crate::reactor::retained_command_ingress_occupancy(), 0, "a {declared}-page command must leave no retained ingress owner");
    }
    eprintln!("[DEBUG] command page set turns: {}", census.join(" "));
    reactor_native_lifecycle_finish(&runtime, lifetime, 9).await;
}

/// ⚖️ LAW: the prologue reserves EXACTLY the pages its command declares, and that reservation stays
/// inside one guest-allocator growth unit.
///
/// 🧊️ The measurement is the heap witness's own peak across `CommandPageSet::try_new`, so it reads
/// the block the allocator was actually asked for. Before the fix the one-page reading is 262 272 B
/// — four times the ceiling the guest can be relied on to serve — and after it is one page's worth.
#[test]
fn a_command_page_authority_reserves_only_the_pages_its_command_declares() {
    for declared in [1usize, 2, 8, semio_framework::kernel::COMMAND_MAXIMUM_PAGES] {
        let reserved = semio_framework::kernel::CommandPageSet::reservation_bytes(declared) as isize;
        assert_eq!(reserved, (declared * size_of::<semio_framework::kernel::FixedCommandPage>()) as isize);
        semio_framework_trace::reset_heap_peak();
        let baseline = semio_framework_trace::retained_heap_bytes();
        let pages = semio_framework::kernel::CommandPageSet::try_new(declared).expect("a declared page authority");
        let measured = semio_framework_trace::peak_heap_bytes() - baseline;
        assert_eq!(pages.declared(), declared);
        assert!(measured >= reserved, "a {declared}-page authority must actually reserve its {reserved} B; the witness saw {measured} B");
        assert!(
            measured < semio_framework::kernel::CommandPageSet::reservation_bytes(semio_framework::kernel::COMMAND_MAXIMUM_PAGES) as isize || declared == semio_framework::kernel::COMMAND_MAXIMUM_PAGES,
            "a {declared}-page authority reserved {measured} B — the 64-page ceiling, not its own declaration"
        );
        drop(pages);
    }
    let one_page = semio_framework::kernel::CommandPageSet::reservation_bytes(1);
    assert!(
        one_page <= semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES,
        "a one-page command's ingress reservation is {one_page} B, over the {} B a routine guest path may ask a fixed linear memory for",
        semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES
    );
    assert!(
        semio_framework::kernel::CommandPageSet::reservation_bytes(semio_framework::kernel::COMMAND_MAXIMUM_PAGES) > semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES,
        "the 64-page ceiling is over that bound — which is why it may not be reserved for every command"
    );
}

/// ⚖️ LAW: a page authority refuses every over-declaration and every over-push with a `Fault` the
/// host can display, and never panics. Exhaustion on this path reaches a user as a refused command,
/// not as a guest that stops answering.
#[test]
fn a_saturated_command_page_authority_refuses_without_panicking() {
    assert_eq!(semio_framework::kernel::CommandPageSet::try_new(0).expect_err("a zero-page command has no authority").code.0, "plugin.command-page-count");
    assert_eq!(
        semio_framework::kernel::CommandPageSet::try_new(semio_framework::kernel::COMMAND_MAXIMUM_PAGES + 1).expect_err("a command may not declare more than the ceiling").code.0,
        "plugin.command-page-count"
    );
    let mut pages = semio_framework::kernel::CommandPageSet::try_new(2).expect("a two-page authority");
    for _ in 0..2 {
        pages.try_push(semio_framework::kernel::FixedCommandPage::try_copy_from(&[7; semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES]).expect("a full page")).expect("the declared pages are admitted");
    }
    let (fault, returned) = pages.try_push(semio_framework::kernel::FixedCommandPage::try_copy_from(b"third").expect("a page")).expect_err("a third page is over the declared authority");
    assert_eq!(fault.code.0, "plugin.command-page-count");
    assert_eq!(returned.as_slice(), b"third", "a refused page is handed back, never dropped");
    assert_eq!(pages.len(), 2);
    let mut saturated = semio_framework::kernel::CommandPageSet::try_new(semio_framework::kernel::COMMAND_MAXIMUM_PAGES).expect("the ceiling authority");
    for _ in 0..semio_framework::kernel::COMMAND_MAXIMUM_PAGES {
        saturated.try_push(semio_framework::kernel::FixedCommandPage::try_copy_from(&[3; semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES]).expect("a full page")).expect("64 full pages are the whole byte authority");
    }
    let (fault, _) = saturated.try_push(semio_framework::kernel::FixedCommandPage::try_copy_from(b"over").expect("a page")).expect_err("the byte authority is spent");
    assert_eq!(fault.code.0, "plugin.command-page-count");
    let mut released = 0;
    while !saturated.close_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES).0 {
        released += 1;
        assert!(released <= semio_framework::kernel::COMMAND_MAXIMUM_PAGES, "closing a saturated authority must terminate");
    }
    assert!(saturated.is_empty());
    let wire = dsl::decode_fault_bytes(&dsl::encode_fault_bytes(&fault));
    assert_eq!(wire.code.0, fault.code.0, "the refusal survives the exact wire the ingress status carries it on");
    assert_eq!(wire.message, fault.message);
    assert!(!wire.retryable, "a refusal the guest cannot serve is not something the host should retry");
}

/// ⚖️ LAW: a long command stream leaves the guest's retained ingress authority empty. Every command
/// is dispatched and closed within its own turns; nothing pins a slot for a later command to queue
/// behind, and the run retains no per-command heap.
///
/// 🚚️ The commands go through the real `poll_kernel` ingress the shard drives, with cursors produced
/// by the real `CommandBatchDriver` — not through a test-only shortcut into the app.
#[semio_framework_async_macros::async_test]
async fn a_long_command_stream_never_pins_the_retained_ingress_authority() {
    let runtime = crate::plugin_runtime::PluginRuntime::<TestRuntimeApps>::new();
    crate::plugin_runtime::install_plugin_bundle(&runtime, __semio_plugin_bundle().await.unwrap());
    let instance = 4_021;
    let captured = reactor_native_lifecycle_poll(&runtime, vec![reactor_native_lifecycle_open(instance, 8, "command-page-authority".into())]).await.lifecycle_receipt.expect("Captured receipt");
    let semio_framework::kernel::ActorInstanceLifecycleReceipt::Captured { lifetime, .. } = captured else { panic!("open must emit Captured") };
    reactor_native_lifecycle_ack(&runtime, captured).await;
    for _ in 0..32 {
        reactor_native_lifecycle_poll(&runtime, Vec::new()).await;
    }
    assert_eq!(crate::reactor::retained_command_ingress_occupancy(), 0, "a settled reactor holds no ingress owner");
    let mut warmed = 0;
    let mut settled = 0;
    let mut faulted = 0;
    let mut peak_occupancy = 0;
    let mut turns = 0;
    for index in 0..INGRESS_COMMANDS {
        let seq = 64 + index;
        let page = command_page_authority_page(instance, seq, &protocol::AppCommand::CommandText { seq, line: format!("noop {seq}") }).await;
        let (completed, spent) = drive_one_command(&runtime, page).await;
        turns += spent;
        if !completed {
            faulted += 1;
        }
        let occupancy = crate::reactor::retained_command_ingress_occupancy();
        peak_occupancy = peak_occupancy.max(occupancy);
        assert_eq!(occupancy, 0, "command {index} left an owner in the retained ingress authority — every later command queues behind it");
        if warmed == 0 && index + 1 == INGRESS_COMMANDS / 4 {
            warmed = index + 1;
            settled = semio_framework_trace::retained_heap_bytes();
        }
    }
    let retained = semio_framework_trace::retained_heap_bytes() - settled;
    let measured = (INGRESS_COMMANDS - warmed) as isize;
    let per_command = retained / measured;
    let census = format!("{INGRESS_COMMANDS} commands, {turns} turns, {faulted} faulted, peak ingress occupancy {peak_occupancy}, {retained} B over the last {measured}");
    assert!(
        per_command <= INGRESS_COMMAND_RETENTION_CEILING_BYTES,
        "the command ingress retains {per_command} B per command (ceiling {INGRESS_COMMAND_RETENTION_CEILING_BYTES}) — the guest's linear memory is fixed and never shrinks: {census}"
    );
    eprintln!("[DEBUG] command page authority: {census}, {per_command} B/command");
    reactor_native_lifecycle_finish(&runtime, lifetime, 9).await;
}
