/// 🔢️ Sequence and registry max plus one: the renderer sequence exhaustion with its transactional rollback and the exact max plus one surface and patch owners of the fixed registries.

#[test]
fn renderer_sequence_maximum_is_once_then_permanently_exhausted_and_rollback_is_transactional() {
    let mut authority = RendererSequenceAuthority { committed: u64::MAX - 1, reserved: None, exhausted: false };
    let maximum = authority.try_reserve().expect("maximum is admitted once");
    assert_eq!(maximum, u64::MAX);
    authority.commit(maximum).expect("maximum commits once");
    let witness = (authority.committed, authority.reserved, authority.exhausted);
    assert!(authority.try_reserve().is_err());
    assert!(authority.try_reserve().is_err());
    assert_eq!((authority.committed, authority.reserved, authority.exhausted), witness);

    let mut rollback = RendererSequenceAuthority::default();
    let first = rollback.try_reserve().expect("first reservation");
    rollback.rollback(first);
    assert_eq!(rollback.try_reserve().expect("rolled back identity is reusable before commit"), first);
}

#[test]
fn renderer_fixed_registries_return_exact_max_plus_one_surface_and_patch_owners() {
    let mut retained = RetainedSurfaceRegistry::new();
    for index in 0..RETAINED_SURFACE_CAPACITY {
        retained.try_admit(1, SurfaceId::try_from(format!("surface-{index}")).expect("bounded surface")).expect("maximum retained surface");
    }
    let overflow = SurfaceId::try_from("surface-overflow").expect("bounded surface");
    assert_eq!(retained.try_admit(1, overflow.clone()), Err(overflow));

    let mut rejections = PendingSurfaceRejectionRegistry::new();
    for index in 0..RETAINED_SURFACE_CAPACITY {
        let surface = SurfaceId::try_from(format!("rejection-{index}")).expect("bounded surface");
        rejections
            .try_admit(PendingSurfaceRejection {
                generation: index as u64 + 1,
                instance: 1,
                surface: surface.clone(),
                revision: UiRevision(index as u64),
                reason: ui_contract::UiText::try_from_str("capacity").expect("bounded reason"),
                patch: Some(retained_patch(surface, 0, index as u64 + 1)),
                receipt: None,
            })
            .unwrap_or_else(|_| panic!("maximum pending rejection"));
    }
    let surface = SurfaceId::try_from("rejection-overflow").expect("bounded surface");
    let rejected = rejections
        .try_admit(PendingSurfaceRejection {
            generation: 999,
            instance: 1,
            surface: surface.clone(),
            revision: UiRevision(999),
            reason: ui_contract::UiText::try_from_str("capacity").expect("bounded reason"),
            patch: Some(retained_patch(surface, 0, 999)),
            receipt: None,
        })
        .expect_err("maximum plus one rejection");
    assert_eq!(rejected.patch.as_ref().map(|patch| patch.revision), Some(UiRevision(999)));
    while !rejections.terminal_is_empty() {
        let _ = rejections.take_any_one();
    }
    while !retained.advance_realm_close_one() {}
}
