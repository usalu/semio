//! 📜️ `UiDocumentLease::read_paged_text` over `🧬️contract/🧫️fixtures/📜️paged-text.json`: the retained
//! wire home of every reserved refresh section reassembles byte-for-byte from any record insertion
//! order, and `serde_json` independently parses the recovered payload back to the authored value.

use super::*;

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../🧫️fixtures/📜️paged-text.json")).expect("paged-text fixture parses")
}

fn text(value: &str) -> crate::UiText {
    crate::UiText::try_from_str(value).expect("bounded fixture text")
}

fn record(value: &serde_json::Value) -> UiNodeRecord {
    let mut children = UiNodeChildren::default();
    for child in value["children"].as_array().expect("record children") {
        children.try_push(UiNodeId(child.as_u64().expect("child id"))).expect("bounded children");
    }
    let component = match value.get("text") {
        Some(leaf) => crate::Component::Text(crate::TextProps {
            value: crate::Label(text(leaf["value"].as_str().expect("leaf value"))),
            emphasize: None,
            data_attributes: leaf.get("dataAttributes").map(|attributes| serde_json::from_value(attributes.clone()).expect("packed attributes")),
        }),
        None => crate::Component::Container(crate::ContainerProps { role: crate::ContainerRole::Plain, label: None, description: None, required: None, error: None, default_open: None, drop_overlay: None }),
    };
    UiNodeRecord {
        id: UiNodeId(value["id"].as_u64().expect("record id")),
        key: text(value["key"].as_str().expect("record key")),
        component,
        layout: Default::default(),
        style: Default::default(),
        activity: Default::default(),
        disabled: false,
        transition: None,
        accessibility: Default::default(),
        bindings: UiNodeBindings::default(),
        menu: None,
        children,
    }
}

fn publish(case: &serde_json::Value, generation: u64) -> UiDocumentLease {
    let surface = SurfaceId::try_from(case["name"].as_str().expect("case name")).expect("bounded surface");
    let identity = UiDocumentAssemblyIdentity { generation, revision: UiRevision(1), root: Some(UiNodeId(case["root"].as_u64().expect("root id"))), layout_epoch: 0 };
    UiDocumentLease::try_publish(surface, identity, case["records"].as_array().expect("case records").iter().map(record).collect()).expect("right-sized carrier document")
}

#[test]
fn paged_text_carrier_reassembles_from_any_record_order() {
    for (index, case) in fixture()["cases"].as_array().expect("fixture cases").iter().enumerate() {
        let name = case["name"].as_str().expect("case name");
        let mut lease = publish(case, 9_000 + index as u64);
        let read = lease.read_paged_text();
        match case.get("expectedError") {
            Some(error) => assert_eq!(read, Err(UiPagedTextError::MissingNode(UiNodeId(error["missingNode"].as_u64().expect("missing node"))))),
            None => {
                let payload = read.unwrap_or_else(|error| panic!("{name}: {error:?}"));
                assert_eq!(payload, case["expectedPayload"].as_str().expect("expected payload"), "{name}");
                if let Some(expected) = case.get("expectedJson") {
                    assert_eq!(&serde_json::from_str::<serde_json::Value>(&payload).expect("oracle parses payload"), expected, "{name}: serde_json oracle");
                }
            }
        }
        while !lease.close_step() {}
    }
}

#[test]
fn right_sized_publication_refuses_an_empty_record_list_and_a_rootless_one() {
    let surface = || SurfaceId::try_from("right-sized").expect("bounded surface");
    let identity = UiDocumentAssemblyIdentity { generation: 9_200, revision: UiRevision(1), root: Some(UiNodeId(7)), layout_epoch: 0 };
    assert_eq!(UiDocumentLease::try_publish(surface(), identity, Vec::new()).err(), Some(UiDocumentPublishError::Empty));
    let rootless = UiDocumentLease::try_publish(surface(), identity, vec![tests::leaf_record(1, "orphan")]).err();
    assert!(matches!(rootless, Some(UiDocumentPublishError::Assembly(UiDocumentAssemblyError { kind: UiDocumentAssemblyErrorKind::MissingRoot, .. }))), "{rootless:?}");
    let before = UiResidentPermit::snapshot().expect("ledger").bytes;
    let mut lease = UiDocumentLease::try_publish(surface(), identity, vec![tests::leaf_record(7, "root")]).expect("rooted document");
    assert!(UiResidentPermit::snapshot().expect("ledger").bytes - before < UI_RESIDENT_SURFACE_BYTES, "the root is priced by its records, never by the surface ceiling");
    while !lease.close_step() {}
}

#[test]
fn paged_text_read_of_a_retired_lease_names_the_lease_fault() {
    let case = &fixture()["cases"][0];
    let mut lease = publish(case, 9_100);
    while !lease.close_step() {}
    assert_eq!(lease.read_paged_text(), Err(UiPagedTextError::Lease(UiDocumentLeaseError::StaleHandle)));
}
