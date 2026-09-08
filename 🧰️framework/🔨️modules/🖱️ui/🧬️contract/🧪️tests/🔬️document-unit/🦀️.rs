use super::*;

fn text(value: &str) -> crate::UiText {
    crate::UiText::try_from_str(value).expect("bounded fixture")
}

#[test]
fn ui_node_id_allocator_is_monotonic_and_never_reuses() {
    let mut allocator = UiNodeIdAllocator::default();
    let a = allocator.try_allocate().expect("first id");
    let b = allocator.try_allocate().expect("second id");
    let c = allocator.try_allocate().expect("third id");
    assert_eq!(a, UiNodeId(0));
    assert_eq!(b, UiNodeId(1));
    assert_eq!(c, UiNodeId(2));
    assert!(a.0 < b.0 && b.0 < c.0);
    assert_ne!(a, b);
    assert_ne!(b, c);
    assert_ne!(a, c);
}

#[test]
fn ui_revision_advances_by_one() {
    let revision = UiRevision(4);
    assert_eq!(revision.try_next(), Some(UiRevision(5)));
}

pub(super) fn leaf_record(id: u64, key: &str) -> UiNodeRecord {
    UiNodeRecord {
        id: UiNodeId(id),
        key: text(key),
        component: crate::Component::Separator(crate::SeparatorProps {}),
        layout: Default::default(),
        style: Default::default(),
        activity: Default::default(),
        disabled: false,
        transition: None,
        accessibility: Default::default(),
        bindings: UiNodeBindings::default(),
        menu: None,
        children: UiNodeChildren::default(),
    }
}

#[test]
fn document_lease_publishes_complete_generation_and_closes_incrementally() {
    let surface = SurfaceId::try_from("surface").expect("bounded fixture");
    let mut builder = UiDocumentBuilder::try_new(41, surface, UiRevision(7), Some(UiNodeId(1)), 3).expect("fixed document slot");
    builder.try_push(leaf_record(1, "root")).expect("one node page");
    let mut lease = builder.finish().expect("complete root");
    let header = lease.header().expect("live header");
    assert_eq!((header.generation, header.revision, header.root, header.node_count), (41, UiRevision(7), UiNodeId(1), 1));
    let page = lease.read_node_page(0).expect("live lease").expect("root page");
    assert_eq!((page.generation(), page.revision(), page.index(), page.record().id), (41, UiRevision(7), 0, UiNodeId(1)));
    assert!(lease.read_node_page(1).expect("live lease").is_none());
    let mut alias = lease.try_alias().expect("credited alias");
    assert!(!lease.close_step(), "alias retains the document owner");
    while !alias.close_step() {}
    assert!(matches!(lease.header(), Err(UiDocumentLeaseError::StaleHandle)));
}

#[test]
fn document_builder_returns_exact_max_plus_one_node_owner() {
    let surface = SurfaceId::try_from("full").expect("bounded fixture");
    let mut builder = UiDocumentBuilder::try_new(42, surface, UiRevision(1), Some(UiNodeId(0)), 0).expect("fixed document slot");
    for id in 0..UI_DOCUMENT_NODES as u64 {
        builder.try_push(leaf_record(id, "node")).expect("maximum fits");
    }
    let rejected = leaf_record(UI_DOCUMENT_NODES as u64, "overflow");
    let (error, returned) = builder.try_push(rejected).expect_err("maximum plus one rejects");
    assert_eq!(error, UiDocumentBuildError::NodeCapacity);
    assert_eq!(returned.id, UiNodeId(UI_DOCUMENT_NODES as u64));
    drop(builder);
    while !close_ui_document_page_one() {}
}

#[test]
fn document_builder_close_persists_until_terminal_after_one_step() {
    let surface = SurfaceId::try_from("builder-close").expect("bounded fixture");
    let mut builder = UiDocumentBuilder::try_new(43, surface, UiRevision(1), Some(UiNodeId(0)), 0).expect("fixed document slot");
    for id in 0..4 {
        builder.try_push(leaf_record(id, "node")).expect("fixed node owner");
    }
    assert!(!builder.close_step(), "one close opportunity cannot erase a nonterminal builder");
    assert!(!builder.terminal_is_empty());
    while !builder.close_step() {}
    assert!(builder.terminal_is_empty());
}

#[test]
fn ordinary_lease_drop_releases_then_global_closer_reaches_terminal() {
    let surface = SurfaceId::try_from("lease-drop").expect("bounded fixture");
    let mut builder = UiDocumentBuilder::try_new(44, surface, UiRevision(1), Some(UiNodeId(0)), 0).expect("fixed document slot");
    builder.try_push(leaf_record(0, "node")).expect("fixed node owner");
    drop(builder.finish().expect("published lease"));
    while !close_ui_document_page_one() {}
    assert!(close_ui_document_page_one(), "drop law leaves no active retirement owner");
}
