use super::*;

fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../🧫️fixture/🔣️.json")).unwrap() }

fn close_typed<T: UiTypedRetire>(root: &mut T, grant: usize) -> usize {
    let mut cursor = UiTypedRetirementCursor::default();
    assert_eq!(cursor.advance(root, 0, grant).unwrap(), UiValueRetirementStep::default());
    assert_eq!(cursor.advance(root, 1, 0).unwrap(), UiValueRetirementStep::default());
    let mut bytes = 0;
    for _ in 0..200_000 {
        let step = cursor.advance(root, 1, grant).unwrap();
        assert!(step.released_items <= 1 && step.released_bytes <= grant);
        bytes += step.released_bytes;
        if step.complete { assert!(cursor.terminal_is_empty()); return bytes; }
        if !step.progressed { std::thread::yield_now(); }
    }
    panic!("typed retirement failed to finish");
}

fn node(component: crate::Component) -> crate::UiNodeRecord {
    crate::UiNodeRecord {
        id: crate::UiNodeId(1), key: UiText::try_from_str("node").unwrap(), component,
        layout: Default::default(), style: Default::default(), activity: Default::default(), disabled: false,
        transition: None, accessibility: Default::default(), bindings: Default::default(), menu: None, children: Default::default(),
    }
}

#[test]
fn instance_lifetime_ui_document_terminal_includes_typed_value_descendants() {
    let fixture = fixture();
    let row = &fixture["document"];
    let value: UiValue = serde_json::from_value(row["value"].clone()).unwrap();
    let handles = super::tests::descendants(&value);
    let record = crate::UiNodeRecord {
        id: crate::UiNodeId(row["nodeId"].as_u64().unwrap()), key: UiText::try_from_str("extension").unwrap(),
        component: crate::Component::Extension(crate::ExtensionProps { extension: UiText::try_from_str("owned.test").unwrap(), props: value }),
        layout: Default::default(), style: Default::default(), activity: Default::default(), disabled: false,
        transition: None, accessibility: Default::default(), bindings: Default::default(), menu: None, children: Default::default(),
    };
    let mut builder = crate::UiDocumentBuilder::try_new(row["generation"].as_u64().unwrap(), crate::SurfaceId::try_from(row["surface"].as_str().unwrap()).unwrap(), crate::UiRevision(1), Some(record.id), 0).unwrap();
    builder.try_push(record).unwrap();
    let mut lease = builder.finish().unwrap();
    for _ in 0..100_000 { if lease.close_step() { break; } }
    let terminal = lease.terminal_is_empty();
    let descendants_retired = with_ui_value_arena(|arena| handles.iter().all(|handle| arena.collection(*handle).is_none()));
    while !close_ui_value_page_one() {}
    assert!(terminal);
    assert_eq!(descendants_retired, row["terminalDescendantsRetired"].as_bool().unwrap());
}

#[test]
fn instance_lifetime_ui_typed_all_components_account_exact_payload_bytes() {
    let components: serde_json::Value = serde_json::from_str(include_str!("../🧩️components.json")).unwrap();
    let fixture = fixture();
    let actual: Vec<_> = components["cases"].as_array().unwrap().iter().map(|row| row["component"]["type"].clone()).collect();
    assert_eq!(serde_json::Value::Array(actual), fixture["componentVariants"]);
    for grant in fixture["grants"].as_array().unwrap() {
        for row in components["cases"].as_array().unwrap() {
            let mut component: crate::Component = serde_json::from_value(row["component"].clone()).unwrap();
            assert_eq!(close_typed(&mut component, grant.as_u64().unwrap() as usize), row["bytes"].as_u64().unwrap() as usize, "{}", row["component"]["type"]);
            let mut second = UiTypedRetirementCursor::default();
            for _ in 0..10_000 {
                let step = second.advance(&mut component, 1, 1).unwrap();
                assert_eq!(step.released_bytes, 0);
                if step.complete { break; }
            }
            assert!(second.terminal_is_empty());
        }
    }
}

#[test]
fn instance_lifetime_ui_typed_all_patch_variants_close_and_max_surface_progresses() {
    use crate::*;
    let raw = serde_json::json!({"x":["🌊"]});
    for grant in [1, 64, 4096] {
        let component = || Component::Extension(ExtensionProps { extension: UiText::try_from_str("E").unwrap(), props: serde_json::from_value(raw.clone()).unwrap() });
        let mut bindings = UiNodeBindings::default();
        bindings.try_push(ActionBinding { trigger: Trigger::Activate, action: ActionId::try_v1("s", "n").unwrap(), args: Some(serde_json::from_value(raw.clone()).unwrap()), capability: None }).unwrap();
        let mut children = UiNodeChildren::default();
        children.try_push(UiNodeId(2)).unwrap();
        let mut patches = [
            (UiPatchOp::Upsert(node(component())), 10), (UiPatchOp::SetComponent { id: UiNodeId(1), component: component() }, 6),
            (UiPatchOp::SetLayout { id: UiNodeId(1), layout: LayoutSpec::Grid(GridLayout::default()) }, 0),
            (UiPatchOp::SetActivity { id: UiNodeId(1), activity: Activity::Idle, disabled: false }, 0),
            (UiPatchOp::SetChildren { id: UiNodeId(1), children }, 0), (UiPatchOp::SetStyle { id: UiNodeId(1), style: Default::default() }, 0),
            (UiPatchOp::SetAccessibility { id: UiNodeId(1), accessibility: AccessibilitySpec { label: Some(Label(UiText::try_from_str("L").unwrap())), ..Default::default() } }, 1),
            (UiPatchOp::SetBindings { id: UiNodeId(1), bindings }, 7),
            (UiPatchOp::SetMenu { id: UiNodeId(1), menu: Some(MenuRef { id: UiText::try_from_str("M").unwrap(), args: Some(serde_json::from_value(raw.clone()).unwrap()) }) }, 6),
            (UiPatchOp::Remove { id: UiNodeId(1) }, 0), (UiPatchOp::SetRoot { id: UiNodeId(1) }, 0),
        ];
        let roster: Vec<_> = patches.iter().map(|(patch, _)| serde_json::to_value(patch).unwrap()["type"].clone()).collect();
        assert_eq!(serde_json::Value::Array(roster), fixture()["patchVariants"]);
        for (patch, bytes) in &mut patches { assert_eq!(close_typed(patch, grant), *bytes); }
        let text = "🌊".repeat(UI_TEXT_MAX_BYTES / 4);
        let mut surface = Component::Surface(SurfaceProps { kind: SurfaceKind::Canvas2d, doc_schema: UiText::try_from_str(&text).unwrap(), doc: SurfaceDoc { bytes: UiFixedBytes::try_from_vec(vec![7; UI_FIXED_BYTES]).unwrap() }, bindings: Default::default() });
        assert_eq!(close_typed(&mut surface, grant), UI_TEXT_MAX_BYTES + UI_FIXED_BYTES);
    }
    assert_eq!(<UiSnapshot as UiTypedRetire>::DEPTH, 8);
    assert_eq!(<UiPatch as UiTypedRetire>::DEPTH, 8);
}
