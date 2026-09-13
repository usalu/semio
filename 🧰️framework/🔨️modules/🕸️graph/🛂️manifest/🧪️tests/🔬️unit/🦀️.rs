
use super::*;

// 🚫️async: E5-class executor bridge, sanctioned per R4 clause 5 — `#[test]` cannot run
// an `async fn` directly (std has no executor for it), so every async test body in this
// module runs through this instead. Sound because this crate performs no real I/O: every
// future here resolves on its first poll, so a single poll (never a spin-park loop) is
// enough — panics loudly if that invariant is ever violated rather than hanging.
fn block_on_test<F: std::future::Future>(fut: F) -> F::Output {
    use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
    fn noop(_: *const ()) {}
    fn clone_raw(_: *const ()) -> RawWaker {
        RawWaker::new(std::ptr::null(), &VTABLE)
    }
    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone_raw, noop, noop, noop);
    let raw = RawWaker::new(std::ptr::null(), &VTABLE);
    let waker = unsafe { Waker::from_raw(raw) };
    let mut cx = Context::from_waker(&waker);
    let mut fut = Box::pin(fut);
    match fut.as_mut().poll(&mut cx) {
        Poll::Ready(v) => v,
        Poll::Pending => panic!("block_on_test: future did not complete synchronously"),
    }
}

#[test]
fn nakagin_manifest_loads() {
    block_on_test(async {
        let m = nakagin::nakagin_manifest();
        assert_eq!(m.id, "nakagin");
        assert!(m.node_kind("Piece").is_some());
        assert!(m.edge_kind("Connection").is_some());
    });
}

#[test]
fn validator_rejects_unknown_node_kind() {
    block_on_test(async {
        let m = nakagin::nakagin_manifest();
        let v = ManifestValidator::new(&m);
        assert!(v.validate_node_kind("NoSuchNode").is_err());
    });
}

#[test]
fn manifest_by_id_resolves() {
    block_on_test(async {
        let m = manifest_by_id("nakagin").expect("nakagin");
        assert!(m.node_kind("Balcony").is_some());
    });
}

#[test]
fn flow_dag_manifest_resolves_from_the_permission_prefixed_source() {
    block_on_test(async {
        let m = manifest_by_id("flow-dag").expect("flow-dag");
        assert_eq!(m.id, "flow-dag");
        assert!(m.node_kind("computation").is_some());
        assert_eq!(flow_dag::FlowDagNodeKind::parse("appInstance"), Ok(flow_dag::FlowDagNodeKind::AppInstance));
    });
}

#[test]
fn property_value_dsl_field_round_trips_nested_array_and_object() {
    block_on_test(async {
        // 🌳️ Nested case: an Object containing an Array containing an Object — proves the
        // `dsl_core::DslField` bridge (via `dsl_core::DslValue`) recurses correctly at every depth, not just
        // for a flat value.
        let mut inner_obj = std::collections::BTreeMap::new();
        inner_obj.insert("flag".to_string(), PropertyValue::Bool(true));
        inner_obj.insert("label".to_string(), PropertyValue::String("leaf".to_string()));

        let array_of_objects = PropertyValue::Array(vec![PropertyValue::Number(1.0), PropertyValue::Object(inner_obj), PropertyValue::Null]);

        let mut root = std::collections::BTreeMap::new();
        root.insert("id".to_string(), PropertyValue::String("root".to_string()));
        root.insert("count".to_string(), PropertyValue::Number(3.0));
        root.insert("items".to_string(), array_of_objects);
        let value = PropertyValue::Object(root);

        let field_value = <PropertyValue as ::dsl_core::DslField>::to_value(&value);
        let round_tripped = <PropertyValue as ::dsl_core::DslField>::from_value(&field_value).expect("round trip must succeed");
        assert_eq!(round_tripped, value, "PropertyValue dsl_core::DslField round trip diverged for a nested Object/Array/Object value");
    });
}

#[test]
fn generated_manifest_enum_value_mappings_are_exact_and_total() {
    macro_rules! assert_mapping {
        ($kind:ty, $rows:expr) => {{
            let source_ids: Vec<String> = ($rows).iter().map(|row| row.id.clone()).collect();
            let wire_ids: Vec<String> = <$kind>::ALL.iter().map(|variant| variant.as_str().to_string()).collect();
            assert_eq!(wire_ids, source_ids);
            let mut wires = std::collections::BTreeSet::new();
            for variant in <$kind>::ALL {
                let wire = variant.as_str();
                assert!(wires.insert(wire), "duplicate wire identity {wire}");
                assert_eq!(dsl_core::ToValue::to_value(variant), dsl_core::DslValue::String(wire.to_string()));
                let decoded = <$kind as dsl_core::FromValue>::from_value(dsl_core::DslValue::String(wire.to_string())).expect("declared wire must decode");
                assert_eq!(&decoded, variant);
            }
            assert_eq!(wires.len(), <$kind>::ALL.len());
            assert!(<$kind as dsl_core::FromValue>::from_value(dsl_core::DslValue::String("unknown-wire".to_string())).is_err());
            assert!(<$kind as dsl_core::FromValue>::from_value(dsl_core::DslValue::Bool(false)).is_err());
        }};
    }
    assert_mapping!(flow_dag::FlowDagNodeKind, flow_dag::flow_dag_manifest().node_kinds);
    assert_mapping!(rewrite_lhs::RewriteLhsNodeKind, rewrite_lhs::rewrite_lhs_manifest().node_kinds);
    assert_mapping!(rewrite_lhs::RewriteLhsEdgeKind, rewrite_lhs::rewrite_lhs_manifest().edge_kinds);
    assert_mapping!(rewrite_lhs::RewriteLhsPortKind, rewrite_lhs::rewrite_lhs_manifest().port_kinds);
    assert_mapping!(rewrite_lhs::RewriteLhsWireKind, rewrite_lhs::rewrite_lhs_manifest().wire_kinds);
    assert_mapping!(puzzle2d_default::Puzzle2dDefaultEdgeKind, puzzle2d_default::puzzle2d_default_manifest().edge_kinds);
    assert_mapping!(puzzle2d_default::Puzzle2dDefaultPortKind, puzzle2d_default::puzzle2d_default_manifest().port_kinds);
    assert_mapping!(puzzle2d_default::Puzzle2dDefaultWireKind, puzzle2d_default::puzzle2d_default_manifest().wire_kinds);
    assert_mapping!(nakagin::NakaginNodeKind, nakagin::nakagin_manifest().node_kinds);
    assert_mapping!(nakagin::NakaginEdgeKind, nakagin::nakagin_manifest().edge_kinds);
    assert_mapping!(nakagin::NakaginPortKind, nakagin::nakagin_manifest().port_kinds);
    assert_mapping!(nakagin::NakaginWireKind, nakagin::nakagin_manifest().wire_kinds);
    assert_mapping!(puzzle5d_default::Puzzle5dDefaultEdgeKind, puzzle5d_default::puzzle5d_default_manifest().edge_kinds);
    assert_mapping!(puzzle5d_default::Puzzle5dDefaultPortKind, puzzle5d_default::puzzle5d_default_manifest().port_kinds);
    assert_mapping!(puzzle5d_default::Puzzle5dDefaultWireKind, puzzle5d_default::puzzle5d_default_manifest().wire_kinds);
    assert_mapping!(writer_languages::WriterLanguagesLanguageKind, writer_languages::writer_languages_manifest().language_kinds);
    assert_mapping!(wires::WiresEdgeKind, wires::wires_manifest().edge_kinds);
    assert_mapping!(drawing_layers::DrawingLayersLayerKind, drawing_layers::drawing_layers_manifest().layer_kinds);
    assert_mapping!(puzzle3d_default::Puzzle3dDefaultEdgeKind, puzzle3d_default::puzzle3d_default_manifest().edge_kinds);
    assert_mapping!(puzzle3d_default::Puzzle3dDefaultPortKind, puzzle3d_default::puzzle3d_default_manifest().port_kinds);
    assert_mapping!(puzzle3d_default::Puzzle3dDefaultWireKind, puzzle3d_default::puzzle3d_default_manifest().wire_kinds);
}
