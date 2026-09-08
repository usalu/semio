
use super::*;
use ui_contract::{Component, SeparatorProps, SurfaceId, UiDocumentBuilder, UiNodeChildren, UiNodeId, UiNodeRecord, UiRevision};

fn record(id: u64, children: &[u64]) -> UiNodeRecord {
    let mut child_ids = UiNodeChildren::default();
    for child in children {
        child_ids.try_push(UiNodeId(*child)).expect("bounded hostile child");
    }
    UiNodeRecord {
        id: UiNodeId(id),
        key: format!("node-{id}").try_into().expect("bounded hostile key"),
        component: Component::Separator(SeparatorProps {}),
        layout: Default::default(),
        style: Default::default(),
        activity: Default::default(),
        disabled: false,
        transition: None,
        accessibility: Default::default(),
        bindings: Default::default(),
        menu: None,
        children: child_ids,
    }
}

fn lease(generation: u64, nodes: &[(u64, &[u64])]) -> ui_contract::UiDocumentLease {
    let surface = SurfaceId::try_from("hostile.surface").expect("bounded hostile surface");
    let mut builder = UiDocumentBuilder::try_new(generation, surface, UiRevision(generation), Some(UiNodeId(nodes[0].0)), generation).expect("hostile builder");
    for (id, children) in nodes {
        builder.try_push(record(*id, children)).expect("hostile page");
    }
    builder.finish().expect("hostile lease")
}

fn step(generation: u64, preview: &mut u64) -> StepContext<'_> {
    let now = semio_framework_job::default_now_us();
    StepContext::new(
        semio_framework_job::OperationId(generation),
        semio_framework_job::Generation(generation),
        now.and_then(|now| semio_framework_job::StepBudget::from_duration(1, now, 100000)).unwrap_or(semio_framework_job::StepBudget::new(0, 0)),
        semio_framework_job::CancelToken::root_now(),
        semio_framework_job::default_now_us,
        preview,
    )
}

fn cancelled_step(generation: u64, preview: &mut u64) -> StepContext<'_> {
    let cancel = semio_framework_job::CancelToken::root_now();
    cancel.cancel_now();
    StepContext::new(semio_framework_job::OperationId(generation), semio_framework_job::Generation(generation), semio_framework_job::StepBudget::new(1, u64::MAX), cancel, semio_framework_job::default_now_us, preview)
}

#[test]
fn max_plus_one_stale_aba_interrupted_close_nested_depth_lost_handle_device_drop_and_last_valid_snapshot() {
    let mut leases = Vec::new();
    for generation in 1..=ui_contract::UI_DOCUMENT_LEASE_SLOTS as u64 {
        leases.push(lease(generation, &[(1, &[])]));
    }
    let rejected_surface = SurfaceId::try_from("hostile.max-plus-one").expect("bounded hostile surface");
    let rejected = UiDocumentBuilder::try_new(99, rejected_surface, UiRevision(99), Some(UiNodeId(1)), 99).expect_err("max plus one returns surface owner");
    assert_eq!(rejected.1.as_ref(), "hostile.max-plus-one");
    drop(leases);
    while !ui_contract::close_ui_document_page_one() {}

    let mut ui = Ui::new();
    let first = lease(101, &[(1, &[])]);
    let header = first.header().expect("first header");
    let mut preview = 0;
    ui.begin_document("window", header, &mut step(101, &mut preview)).expect("first begin");
    ui.apply_document_page("window", first.read_node_page(0).expect("first read").expect("first page"), &mut step(101, &mut preview)).expect("first apply");
    loop {
        match ui.finish_document("window", 101, &mut step(101, &mut preview)) {
            Ok(()) => break,
            Err(UiDocumentIngressFault::ValidationPending) => {}
            Err(fault) => panic!("first publish failed: {fault:?}"),
        }
    }
    assert_eq!(ui.windows.get("window").and_then(|window| window.tree.document()).map(UiDocumentTree::generation), Some(101));

    let aba = lease(100, &[(1, &[])]);
    let aba_rejected = ui.begin_document("window", aba.header().expect("aba header"), &mut step(100, &mut preview));
    assert!(matches!(aba_rejected, Err((UiDocumentIngressFault::StaleGeneration, _))));
    let cancelled = lease(104, &[(1, &[])]);
    let cancelled_rejected = ui.begin_document("window", cancelled.header().expect("cancelled header"), &mut cancelled_step(104, &mut preview));
    assert!(matches!(cancelled_rejected, Err((UiDocumentIngressFault::Cancelled, _))));

    let nested = lease(102, &[(1, &[2]), (2, &[3]), (3, &[])]);
    ui.begin_document("window", nested.header().expect("nested header"), &mut step(102, &mut preview)).expect("nested begin");
    ui.apply_document_page("window", nested.read_node_page(0).expect("nested read").expect("nested page"), &mut step(102, &mut preview)).expect("nested first page");
    let stale = lease(103, &[(1, &[])]);
    let interrupted = ui.begin_document("window", stale.header().expect("stale header"), &mut step(103, &mut preview));
    assert!(matches!(interrupted, Err((UiDocumentIngressFault::InterruptedClose, _))));
    assert_eq!(ui.windows.get("window").and_then(|window| window.tree.document()).map(UiDocumentTree::generation), Some(101));
    drop(aba);
    drop(cancelled);
    drop(first);
    drop(nested);
    drop(stale);
    while !ui.close_document_step("window") {}
    while !ui_contract::close_ui_document_page_one() {}
    assert!(ui.windows.get("window").and_then(|window| window.tree.document()).is_none());
}
