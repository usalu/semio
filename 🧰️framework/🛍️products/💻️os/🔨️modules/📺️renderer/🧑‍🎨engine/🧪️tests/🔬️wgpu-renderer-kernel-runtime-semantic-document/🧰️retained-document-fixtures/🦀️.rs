/// 🧰️ Retained document fixtures: the replay recovery job, the retained document and the retained patch builders.

fn recovery_job(owner: &MountedProductReplayRecoveryOwner) -> u64 {
    match owner {
        MountedProductReplayRecoveryOwner::Request(request) | MountedProductReplayRecoveryOwner::Claim { request, .. } | MountedProductReplayRecoveryOwner::Authority { request, .. } => request.raw.job,
    }
}

fn document(generation: u64, surface: &str) -> UiDocumentLease {
    let id = ui_contract::UiNodeId(1);
    let surface = SurfaceId::try_from(surface).expect("bounded command retirement surface");
    let mut builder = UiDocumentBuilder::try_new(generation, surface, UiRevision(generation), Some(id), generation).expect("command retirement document slot");
    builder
        .try_push(ui_contract::UiNodeRecord {
            id,
            key: ui_contract::UiText::try_from_str("root").expect("bounded command retirement key"),
            component: ui_contract::Component::Separator(ui_contract::SeparatorProps {}),
            layout: Default::default(),
            style: Default::default(),
            activity: Default::default(),
            disabled: false,
            transition: None,
            accessibility: Default::default(),
            bindings: Default::default(),
            menu: None,
            children: Default::default(),
        })
        .expect("command retirement root");
    builder.finish().expect("command retirement document")
}

fn retained_patch(surface: SurfaceId, base: u64, revision: u64) -> KernelUiPatch {
    let id = ui_contract::UiNodeId(1);
    let record = ui_contract::UiNodeRecord {
        id,
        key: ui_contract::UiText::try_from("root").expect("bounded hostile key"),
        component: ui_contract::Component::Separator(ui_contract::SeparatorProps {}),
        layout: Default::default(),
        style: Default::default(),
        activity: Default::default(),
        disabled: false,
        transition: None,
        accessibility: Default::default(),
        bindings: Default::default(),
        menu: None,
        children: Default::default(),
    };
    let mut ops = ui_contract::UiPatchOps::default();
    ops.try_push(ui_contract::UiPatchOp::Upsert(record)).expect("hostile upsert owner");
    ops.try_push(ui_contract::UiPatchOp::SetRoot { id }).expect("hostile root owner");
    KernelUiPatch { surface, base_revision: UiRevision(base), revision: UiRevision(revision), ops }
}
