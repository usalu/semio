
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
