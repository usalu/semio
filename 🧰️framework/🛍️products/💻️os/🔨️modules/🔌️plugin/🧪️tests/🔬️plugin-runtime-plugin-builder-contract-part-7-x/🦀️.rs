/// 🔬️ Items of Part 7 of 7 of the plugin builder contract laws that came from included fixture files.

/// 🧾️ Owned factories for every verb, but NO `bounded_first_step_tool_proofs` row. A proof row is
/// only admissible while the LIVE registry declares its `OpBinary::TOOL_JOB_IDS` id `Migrated`
/// (`validate_tool_job_rows`'s `seen != expected` gate), so a registry-LESS wrapper can only ever
/// carry zero rows — which is exactly what `registry_less_construction_rejects_before_the_reducer`
/// constructs, and why it needs its own mode rather than the full one.
const TEST_APP_TOOLS_FACTORIES: u8 = 1;
const TEST_APP_COMMAND_WORK_ITEMS: usize = 1;

#[test]
fn checkpoint_restart_transient_close_retains_the_exact_store_until_granted() {
    use semio_framework_trace::observe_heap_allocations_on_this_thread;
    use semio_framework_value::retained_clone::RetainedCloneGrant;
    let fixture: Value = serde_json::from_str(include_str!("../🧫️fixtures/⏳️completion/🔣️.json")).unwrap();
    let law = &fixture["transientClose"];
    let grant: RetainedCloneGrant = serde_json::from_value(law["originalGrant"].clone()).unwrap();
    let ((mut owner, mut close), original_heap) = observe_heap_allocations_on_this_thread(|| (
        store::TransientStore::<PublicationTransient, PublicationTransientMutation>::new(PublicationTransient { revision: law["sourceRevision"].as_u64().unwrap() }),
        TestApp::<true>::build_transient_store_disposer().expect("actual owned transient issuer"),
    ));
    let source = owner.current_root();
    let pointer = std::sync::Arc::as_ptr(&source);
    assert_eq!(serde_json::to_value(source.as_ref()).unwrap(), json!({ "revision": law["sourceRevision"] }));
    drop(source);
    assert_eq!(close.terminal_is_empty(&owner), law["initiallyTerminal"].as_bool().unwrap());
    let mut births = 0usize;
    let mut releases = 0usize;
    let mut complete = false;
    for _ in 0..65_536 {
        let demand = close.retirement_demands(&owner, grant.maximum_copy_bytes).unwrap();
        assert!(grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth,"original transient caller must fund the exact quoted close frontier");
        for axis in law["deniedAxes"].as_array().unwrap() {
            let mut denied = grant;
            let positive = match axis.as_str().unwrap() {
                "items" => { denied.maximum_items = 0; true },
                "copy" => { denied.maximum_copy_bytes = 0; demand.copy_bytes > 0 },
                "capacity" => { denied.maximum_capacity_bytes = 0; demand.capacity_bytes > 0 },
                "release" => { denied.maximum_release_bytes = 0; demand.release_bytes > 0 },
                "depth" => { denied.maximum_depth = 0; demand.depth > 0 },
                _ => panic!("declared independent grant axis"),
            };
            if !positive { continue; }
            let (step, heap) = observe_heap_allocations_on_this_thread(|| close.close_step(&mut owner, denied));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            if let Ok(step) = step { assert_eq!(step.progress().unwrap_or_default(), Default::default()); }
            if !owner.detached_terminal_is_empty() {
                let source = owner.current_root();
                assert_eq!(std::sync::Arc::as_ptr(&source), pointer);
            }
        }
        let (step, heap) = observe_heap_allocations_on_this_thread(|| close.close_step(&mut owner, grant).unwrap());
        let progress = step.progress().expect("physical transient lifecycle receipt");
        assert!(progress.fits(grant));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
        births += heap.requested_bytes;
        releases += heap.released_bytes;
        if close.terminal_is_empty(&owner) { complete = true; break; }
    }
    assert!(complete);
    assert!(owner.detached_terminal_is_empty());
    let mut foreign = store::TransientStore::<PublicationTransient, PublicationTransientMutation>::new(PublicationTransient::default());
    assert_eq!(close.terminal_is_empty(&foreign), law["foreignTerminal"].as_bool().unwrap());
    let frame = close.terminal_frame_release_bytes().expect("actual terminal disposer Box");
    let ((), heap) = observe_heap_allocations_on_this_thread(|| drop(close));
    assert_eq!(heap.released_bytes, frame);
    releases += heap.released_bytes;
    assert_eq!(releases, original_heap.requested_bytes + births);
    let result = json!({ "terminal": complete, "detachesWithoutReplacement": owner.detached_terminal_is_empty() });
    assert_eq!(result, law["expected"]);
    drop(foreign);
}// 📏️ The size of the future `wit_bindgen`'s `start_task` boxes once per `poll` export call.
//
// `__export_poll_cabi` → `async_support::start_task(fut)` → `Box::pin(fut)`: ONE heap allocation
// of `size_of_val(&fut)` bytes per guest turn, on wasm only. Boot #9b died on exactly that
// allocation at 189 328 bytes. The number is a property of the Rust generator layout, which is
// computed in MIR (`StateTransform`) and is therefore IDENTICAL natively — so this law weighs it
// without a wasm build.
//
// See `📓️poll-task-leak-2026-09-10.md`.

/// 📐️ `size_of_val` of the real turn future, exactly as `wit_bridge::poll` composes it:
/// `poll_kernel_output` → `with_turn_execution` → `poll_kernel_turn`, with the same generic
/// arguments the WIT bridge instantiates.
fn reactor_turn_future_bytes() -> usize {
    let runtime = semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime::<TestRuntimeApps>::new({ let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
    let budget = Budget::from_retained(semio_framework_plugin_testing_laws::artifact_app_laws::fixture_retained_turn(), 64, 16, 65536, 16);
    let mut identity = semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity();
    let mut context = semio_framework_plugin_testing_laws::artifact_app_laws::fixture_step_context();
    let future = semio_framework_plugin_reactor_turn::poll_kernel(&runtime, &mut None, None, None, budget, &mut identity, &mut context);
    let bytes = size_of_val(&future);
    drop(future);
    bytes
}

/// 📐️ One wasm page. `start_task` allocates the turn future ONCE PER GUEST TURN, and it is the
/// largest single allocation the guest ever makes — boot #9b died on exactly that request at
/// 189 328 B (252 272 B here, the same generator at 64-bit pointer width). Measured after the fix:
/// 53 872 B. The ceiling is a ratchet against the two shapes that produced the quarter megabyte —
/// a future taken by value and re-pinned, and a large `async fn` awaited inline — not a target;
/// §8 of `📓️poll-task-leak-2026-09-10.md` carries the byte accounting for the next step.
const REACTOR_TURN_FUTURE_CEILING_BYTES: usize = 65_536;

#[test]
fn the_reactor_turn_future_fits_the_one_page_ceiling() {
    let bytes = reactor_turn_future_bytes();
    eprintln!("reactor turn future size_of_val = {bytes} B");
    assert!(bytes <= REACTOR_TURN_FUTURE_CEILING_BYTES, "the turn future is {bytes} B — `wit_bindgen::rt::async_support::start_task` boxes exactly this much on every guest turn");
}

/// 🎟️ The fixture's original close policy remains independent of every retirement quote.
fn composed_close_turn<A: PluginApp>(app: &mut A, body: usize, cx:&mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
    assert_eq!(body,COMPOSITION_MAINTENANCE_GRANT.maximum_copy_bytes);
    let step = app.close_step(cx,COMPOSITION_MAINTENANCE_GRANT);
    let _ = app.take_constructor_receipt();
    step
}