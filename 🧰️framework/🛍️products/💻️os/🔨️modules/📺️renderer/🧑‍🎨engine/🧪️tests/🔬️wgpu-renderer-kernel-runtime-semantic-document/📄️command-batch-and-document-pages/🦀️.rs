/// 📄️ Command batch and document pages: the retained batch document, page saturation, stale cancel and fault builds and the first render opportunities.

/// 📄️ One published page is exactly [`UI_DOCUMENT_LEASE_SLOTS`] owners, and the owner AFTER that
/// page stays qualified in the close lane across a nonterminal close.
///
/// 🐛️ ticket 26/09/17/WGPU-RENDERER-REACT-PARITY wave 2–6 integration: this law was written when a
/// page was eight owners and still filled a nine-slot list, then admitted `UI_DOCUMENT_LEASE_SLOTS`
/// of them — which has been SIXTY-FOUR since the lease slots followed `UI_RESIDENT_SLOTS`, so the
/// tenth `swap_remove` panicked "first page owner" before the law asserted anything. A page's worth
/// of owners is now built from the page size itself: one lease yields
/// [`ui_contract::UI_DOCUMENT_LEASE_ALIASES`] owners (the primary plus its aliases), so the page
/// needs `UI_DOCUMENT_LEASE_SLOTS / UI_DOCUMENT_LEASE_ALIASES` leases.
#[test]
fn command_batch_slots_plus_one_document_is_retained_after_nonterminal_close_and_exactly_returned_or_retired() {
    const OWNERS_PER_LEASE: usize = ui_contract::UI_DOCUMENT_LEASE_ALIASES as usize;
    const PAGE_LEASES: usize = UI_DOCUMENT_LEASE_SLOTS / OWNERS_PER_LEASE;
    assert_eq!(PAGE_LEASES * OWNERS_PER_LEASE, UI_DOCUMENT_LEASE_SLOTS, "a page is a whole number of fully aliased leases");
    let mut owners = UiFixedList::<ExchangeSurfaceDocument, { UI_DOCUMENT_LEASE_SLOTS + 1 }>::default();
    for lease in 0..PAGE_LEASES {
        let primary = document(91_001 + lease as u64, &format!("command.retirement.lease-{lease}"));
        for alias in 0..OWNERS_PER_LEASE - 1 {
            owners
                .try_push(ExchangeSurfaceDocument { surface: SurfaceId::try_from(format!("command.retirement.alias-{lease}-{alias}")).expect("bounded alias surface"), document: primary.try_alias().expect("fixed command document alias") })
                .unwrap_or_else(|_| panic!("command alias owner"));
        }
        owners.try_push(ExchangeSurfaceDocument { surface: SurfaceId::try_from(format!("command.retirement.primary-{lease}")).expect("bounded primary surface"), document: primary }).unwrap_or_else(|_| panic!("command primary owner"));
    }
    owners
        .try_push(ExchangeSurfaceDocument { surface: SurfaceId::try_from("command.retirement.slots-plus-one").expect("bounded overflow surface"), document: document(91_099, "command.retirement.slots-plus-one-document") })
        .unwrap_or_else(|_| panic!("command slots-plus-one owner"));
    let mut registry = CommandDocumentRetirementRegistry::new();
    let mut first_page = registry.try_reserve_page(7, 91_101).expect("first fixed page destinations");
    for _ in 0..UI_DOCUMENT_LEASE_SLOTS {
        registry.admit(&mut first_page, owners.swap_remove(0).expect("first page owner"));
    }
    registry.release(&mut first_page);
    let mut second_page = registry.try_reserve_page(7, 91_101).expect("second fixed page destinations");
    registry.admit(&mut second_page, owners.swap_remove(0).expect("slots-plus-one page owner"));
    registry.release(&mut second_page);
    registry.begin_close_batch(7, 91_100);
    assert!(!registry.has_close_work(), "stale batch generation cannot mutate pending document owners");
    let mut output = UiFixedList::default();
    registry.publish_batch(7, 91_101, &mut output);
    assert_eq!(output.len(), UI_DOCUMENT_LEASE_SLOTS);
    assert!(!registry.batch_is_empty(7, 91_101), "the slots-plus-one owner remains qualified in the close lane");
    while !registry.close_one() {}
    assert!(!registry.batch_is_empty(7, 91_101), "one nonterminal close opportunity cannot discard the slots-plus-one owner");
    let mut cleanup = registry.try_reserve_page(8, 91_102).expect("output cleanup destinations");
    for owner in output {
        registry.admit(&mut cleanup, owner);
    }
    registry.release(&mut cleanup);
    registry.begin_close_batch(8, 91_102);
    let mut opportunities = 1;
    while !registry.batch_is_empty(7, 91_101) || !registry.batch_is_empty(8, 91_102) {
        let _ = registry.close_one();
        opportunities += 1;
        assert!(opportunities < 16_384, "command document retirement must converge");
    }
}

#[test]
fn command_document_page_saturation_refuses_before_turn_owner_production() {
    let mut registry = CommandDocumentRetirementRegistry::new();
    let mut reservations = UiFixedList::<CommandDocumentPageReservation, { COMMAND_DOCUMENT_RETIREMENT_CAPACITY / UI_DOCUMENT_LEASE_SLOTS }>::default();
    for generation in 1..=COMMAND_DOCUMENT_RETIREMENT_CAPACITY / UI_DOCUMENT_LEASE_SLOTS {
        reservations.try_push(registry.try_reserve_page(17, generation as u64).expect("absolute command document page credit")).unwrap_or_else(|_| panic!("fixed reservation owner"));
    }
    assert!(registry.try_reserve_page(17, 999).is_err(), "max plus one refuses before a guest turn can produce another document owner");
    for mut reservation in reservations {
        registry.release(&mut reservation);
    }
    assert!(registry.instance_is_empty(17));
}

#[test]
fn stale_cancel_and_fault_document_builds_retain_closer_until_terminal() {
    let surface = SurfaceId::try_from("hostile.document-close").expect("bounded hostile surface");
    let mut retained = RetainedSurface::new(surface.clone());
    retained.admit_patch(retained_patch(surface, 0, 1)).expect("retained patch");
    while retained.patch.is_some() {
        let _ = retained.advance_patch_one();
    }
    retained.begin_document_build();
    retained.advance_document_one();
    let build = retained.build.as_mut().expect("builder retained");
    build.closing = true;
    retained.advance_document_one();
    assert!(retained.build.is_some(), "cancel/stale/fault first close opportunity retains a nonterminal closer");
    while retained.build.is_some() {
        retained.advance_document_one();
    }
    while !retained.close_step() {}
}

#[test]
fn first_render_requires_multiple_fixed_opportunities_stale_patch_keeps_last_valid_and_close_reaches_terminal() {
    let surface_id = SurfaceId::try_from("hostile.first-render").expect("bounded hostile surface");
    let mut retained = RetainedSurface::new(surface_id.clone());
    retained.admit_patch(retained_patch(surface_id.clone(), 0, 1)).expect("first retained patch");
    assert!(retained.try_alias().is_none());
    let mut opportunities = 0;
    while retained.try_alias().is_none() {
        assert!(retained.advance_patch_one().is_none());
        if retained.patch.is_none() {
            retained.advance_document_one();
        }
        opportunities += 1;
        assert!(opportunities <= ui_contract::UI_DOCUMENT_PATCH_OPS + ui_contract::UI_DOCUMENT_NODES * ui_contract::UI_DOCUMENT_NODES);
    }
    assert!(opportunities > 1);
    let mut last_valid = retained.try_alias().expect("published first document");
    let generation = last_valid.generation();
    retained.admit_patch(retained_patch(surface_id, 0, 2)).expect("stale patch owner retained");
    while retained.advance_patch_one().is_none() {}
    assert_eq!(retained.try_alias().expect("last valid survives stale patch").generation(), generation);
    let _ = last_valid.close_step();
    drop(last_valid);
    while !retained.close_step() {}
}
