/// 🔓️ Reservation release and registry idle: the dropped render reservation, the unacknowledged alias window surface, the resident credit ledger refusal, two hundred publications and the oversized node record.

/// 🕹️ Wave B48 LAW: a render reservation that ends WITHOUT `cancel` releases exactly what `cancel`
/// releases — so the surface it held is reservable again, and its reserved output is not leaked.
///
/// 🐛️ `MountedReconcileGrant`'s `Drop` used to release only the reconciler and the reservation, while
/// `cancel` also cleared `slot.output_index` and closed the reserved `ready` output. Every
/// `commit_source` failure exit takes the `Drop` path (it returns `Err(root)` without disarming), and
/// `reserve_mounted_owned` refuses any slot whose `output_index.is_some()` — so ONE refused commit made
/// that surface permanently un-reservable: every later dirty render was refused, only `defer`red,
/// answered `deferred_surface_ready` (no producer, no job, reconciler restored, revision acknowledged),
/// re-dirtied, and refused again, forever. The surface stays frozen at its last published revision while
/// the actor answers `more-work` and publishes nothing — measured live on `:6013` at wasm #58 as a pick
/// whose refresh asked for all three world bodies, dropped none, and got `changed:[]` with every
/// retained surface still at revision 1 (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B46 §5/§8.1, wave
/// B48 §3). The loop below runs past [`READY_PATCH_CAPACITY`] because each such exit also leaked one
/// output slot, and once those are gone `reserve_mounted` refuses EVERY surface in the shell.
///
/// 🧹️ A released output is CLOSED, not yet retired — retirement is incremental and a turn drives
/// it, so the law drives it too. Without this the loop would only prove the surface slot is free;
/// with it, it also proves the released output returns to the fixed `ready` pool.
#[test]
fn a_dropped_render_reservation_releases_its_surface_slot_and_its_output() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("independently authored original patch fixture caller");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=(||{
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let tracker = PatchTracker::new();
    let key = NativeCloseKey::fixture(5, 1);
    let surface = ui_contract::SurfaceId::try_from("5:window").expect("bounded surface");
    for attempt in 0..(READY_PATCH_CAPACITY + 2) {
        let grant = match tracker.reserve_mounted(surface.clone(), key) {
            Ok(grant) => grant,
            Err(_refused) => panic!("attempt {attempt} must still be reservable after {attempt} dropped reservations: {}", tracker.debug_state()),
        };
        drop(grant);
        for _ in 0..1_024 {
            tracker.drive_one(cx).expect("original patch fixture semantic step");
            if tracker.close_step(cx).expect("original patch fixture close step") {
                break;
            }
        }
    }
    let mut grant = tracker.reserve_mounted(surface.clone(), key).expect("a released slot reserves");
    grant.cancel();
    assert!(tracker.reserve_mounted(surface, key).is_ok(), "`cancel` and `Drop` must leave the slot in the same reservable state: {}", tracker.debug_state());

    })();
    let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original patch fixture caller paid source close");result
}

/// 🪟️ WAVE B52 LAW: an alias window surface that still HOLDS an output, and whose revision the host
/// never acknowledges, must not hold its instance's retirement ladder open.
///
/// 🧾️ The `1:window` alias is a Window binding (wave B25), so it is the one surface of a puzzle3d
/// instance that is published without the host ever asking for it by body, and B50's live census caught it
/// mid-publication with its reconciler checked out and its revision unacknowledged
/// (`1:window#g14:-J-:ack1/rev0:outSome(0)` — output SLOT 0, not zero bytes). `reserve_mounted` rightly
/// refuses such a slot and the turn defers the render; what must never follow is the CLOSE ladder waiting
/// on the same slot, because then the instance never reaches its terminal and the actor answers
/// `more-work` for ever with nothing to publish (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B52 §2,
/// wave B50's 7 843-turn streak with `sources=["reconcile"]`).
#[test]
fn an_alias_window_surface_the_host_never_acknowledges_does_not_block_the_retirement_ladder() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("independently authored original patch fixture caller");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=(||{
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let tracker = PatchTracker::new();
    tracker.begin("21:window".into(), tree_with_owned_child("alias")).expect("admitted alias publication");
    let patch = finish(&tracker,cx).expect("alias surface publishes");
    close_test_patch(patch);
    {
        let mut state = tracker.state.borrow_mut();
        let slot = state.slots.iter_mut().flatten().find(|slot| slot.surface.as_ref() == "21:window").expect("mounted alias surface");
        let revision = slot.reconciler.as_ref().expect("published canonical root").revision();
        slot.acknowledged_revision = ui_contract::UiRevision(revision.0.saturating_sub(1));
    }
    assert!(tracker.defer(ui_contract::SurfaceId::try_from("21:window").expect("bounded surface")).is_ok());
    assert!(tracker.take_deferred_ready().is_none(), "the unacknowledged alias stays deferred: {}", tracker.debug_state());
    close_instance_to_empty(&tracker, 21,cx);

    })();
    let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original patch fixture caller paid source close");result
}

/// 🎫️ WAVE B56 LAW: the reconcile registry that refuses a render reservation NAMES itself, and the
/// table that runs out first is the aggregate resident CREDIT ledger — not the handback registry.
///
/// 🦾️ This is the reading wave B48 residual 1 asked for and wave B54 §6.4 could not take: its census
/// carried the single opaque word `registry-reservation-unavailable`, so the refusal that left six surfaces
/// deferred and the reactor answering `more-work` 1 008 turns in a row could have been any of three
/// different defects. The handback registry has `UI_RESIDENT_SLOTS * 6` slots, an order of magnitude more
/// headroom than the aggregate byte budget, so the credit ledger is always the table that runs out first.
///
/// 🧾️ WAVE B58 refinement: credit is now priced by the body a surface actually reconciles, so an aggregate
/// full of CEILING-sized bodies is what it takes to refuse one — the arithmetic that used to hold for every
/// surface (`UI_RESIDENT_AGGREGATE_BYTES` / `UI_RESIDENT_SURFACE_BYTES` = a handful, against a session that
/// mounts thirteen) now holds only for bodies that genuinely fill the per-surface maximum. This law holds
/// that many ceiling-sized reservations OUTRIGHT and then requires the refusal to name the ledger it came
/// from; `twelve_mounted_surfaces_at_their_real_sizes_all_hold_a_reconcile_reservation_at_once` pins the
/// other half — that ordinary bodies are never refused at all.
#[test]
fn the_refused_reconcile_reservation_names_the_resident_credit_ledger_not_the_handback_registry() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("independently authored original patch fixture caller");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=(||{
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let baseline = semio_framework_ui_runtime::surface_reconcile_registry_census();
    let ceiling = (ui_contract::UI_RESIDENT_AGGREGATE_BYTES - baseline.resident_bytes) / ui_contract::UI_RESIDENT_SURFACE_BYTES;
    assert!(ceiling > 0 && ceiling < ui_contract::UI_RESIDENT_SLOTS, "the ceiling-sized credit ceiling must be a handful of surfaces, not the slot count: {ceiling}");
    let held = fill_resident_aggregate();
    let tracker = PatchTracker::new();
    let over = ui_contract::SurfaceId::try_from("51:surface-over").expect("bounded surface");
    assert!(reserve(&tracker, over).is_err(), "a reconcile reservation must be refused once the aggregate is full of ceiling-sized bodies: {}", tracker.debug_state());
    let state = tracker.debug_state();
    assert!(state.contains("reserve_refusal=unmounted:registry-resident-credit-exhausted"), "the refusal must name the resident credit ledger, not an opaque registry — and a surface refused before it ever mounted has no slot to name: {state}");
    assert!(state.contains(&format!("of {}B", ui_contract::UI_RESIDENT_AGGREGATE_BYTES)), "the census must price the refusal against the aggregate budget: {state}");
    let refused_census = semio_framework_ui_runtime::surface_reconcile_registry_census();
    assert!(refused_census.handback_free > 0, "the handback registry must still have headroom when credit runs out: {refused_census:?}");
    release_resident_aggregate(held);

    })();
    let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original patch fixture caller paid source close");result
}

/// 🎫️ WAVE B56 LAW: after two hundred leftover-producing publications the process-wide reconcile
/// registries hold NOTHING — neither resident credit nor a handback slot.
///
/// 🐛️ Every render reservation takes one 8 MiB slice of a 32 MiB aggregate resident budget
/// (`UI_RESIDENT_AGGREGATE_BYTES` / `SurfaceReconcileLimits::default().max_bytes`), so at most a handful of
/// surfaces can hold one at a time while a puzzle3d instance mounts thirteen. A publication that leaks its
/// credit therefore does not slow the guest down, it STOPS it: the next reservation is refused
/// `registry-resident-credit-exhausted`, `reserve_mounted` hands the surface back, the turn defers the
/// render, `redirty_acknowledged_deferred_surfaces` re-dirties it, and the reactor answers `more-work` for
/// ever with `effects=0` — wave B54 §6.4 measured a streak of 1 008 with every slot clean and every
/// command's settled count 0. One leak is unobservable in a single-publication law and fatal in a session,
/// so the count is the session's, not the unit's (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B56).
#[test]
fn two_hundred_publications_leave_the_reconcile_registries_holding_nothing() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("independently authored original patch fixture caller");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=(||{
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let baseline = semio_framework_ui_runtime::surface_reconcile_registry_census();
    let baseline_fixed = ui_contract::UiResidentPermit::fixed_backing_bytes().expect("resident ledger");
    let tracker = PatchTracker::new();
    for round in 0..200u32 {
        tracker.begin("41:puzzle3d-main".into(), tree_with_owned_child(&round.to_string())).unwrap_or_else(|_| panic!("round {round} admits its publication: {}", tracker.debug_state()));
        let patch = acknowledge_one_publication(&tracker, "41:puzzle3d-main",cx).unwrap_or_else(|| panic!("round {round} publishes: {}", tracker.debug_state()));
        close_test_patch(patch);
        drain_released_output(&tracker, "41:puzzle3d-main", round,cx);
        let census = semio_framework_ui_runtime::surface_reconcile_registry_census();
        assert!(census.resident_bytes <= baseline.resident_bytes + ui_contract::UI_RESIDENT_SURFACE_BYTES, "round {round} left more than one live reservation's credit held: {census:?} against baseline {baseline:?} — {}", tracker.debug_state());
    }
    close_instance_to_empty(&tracker, 41,cx);
    let after = semio_framework_ui_runtime::surface_reconcile_registry_census();
    assert_eq!(after.resident_slots, baseline.resident_slots, "resident slots leaked across 200 publications: {after:?} against {baseline:?}");
    let after_fixed = ui_contract::UiResidentPermit::fixed_backing_bytes().expect("resident ledger");
    assert_eq!(
        after.resident_bytes - after_fixed,
        baseline.resident_bytes - baseline_fixed,
        "resident credit leaked across 200 publications (the reconcile runtime backing registers once, on the first reservation, and is fixed thereafter): {after:?} against {baseline:?}"
    );
    assert_eq!(after.handback_free, baseline.handback_free, "handback slots leaked across 200 publications: {after:?} against {baseline:?}");

    })();
    let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original patch fixture caller paid source close");result
}

/// 🧮️ WAVE B52 LAW: the retirement ladder of a published surface must not be priced by a grant
/// narrower than one indivisible allocation of the value it retires.
///
/// 🐛️ `PagedList::release_empty_page` frees a page whole or not at all, and refuses a grant that does
/// not fit it — its own laws pin that refusal. One `UiNodeRecord` is 6 416 bytes against the reconciler's
/// 4 096-byte `SURFACE_COMPONENT_COPY_WORK_BYTES`, so the document ladder answered
/// `progressed: false, complete: false` 67 667 consecutive times on one catalogue surface and its terminal
/// never emptied. This law publishes a document-scaled surface, retires it, and requires the whole ladder
/// to reach terminal empty — which the fixed `retire_exact` grant makes possible and which its failure
/// message prices in node records so a future record growth cannot silently re-break it (ticket
/// 26/09/02/PUZZLE-3D-END-TO-END wave B52 §2).
#[test]
fn a_node_record_wider_than_the_copy_grant_still_retires_its_document_to_terminal_empty() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("independently authored original patch fixture caller");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=(||{
    let _guard = semio_framework_ui_runtime::surface_reconcile_registry_test_guard();
    let record = size_of::<ui_contract::UiNodeRecord>();
    let tracker = PatchTracker::new();
    let mut root = leaf("catalogue", "Catalogue").root;
    for row in 0..32u32 {
        root.children.try_push(leaf(&row.to_string(), "Row").root).expect("bounded fixture row");
    }
    {let mut grant=reserve(&tracker, ui_contract::SurfaceId::try_from("22:catalogue").expect("bounded surface")).expect("fixed mounted reservation");let mut source=Some(ComponentTree::new(root));assert!(grant.commit_source(&mut source,cx).expect("admitted publication"), "actual original source admitted");assert!(source.is_none());}
    let patch = finish(&tracker,cx).expect("catalogue surface publishes");
    close_test_patch(patch);
    close_instance_to_empty(&tracker, 22,cx);
    assert!(!tracker.has_work(), "one node record is {record} bytes and its document ladder still holds work: {}", tracker.debug_state());

    })();
    let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original patch fixture caller paid source close");result
}
