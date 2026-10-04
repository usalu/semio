
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
fn validator_rejects_unknown_node_kind() {
    block_on_test(async {
        let m = <Manifest as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(r#"{"schema":"manifest","id":"neutral.validation","nodeKinds":[{"id":"Entry"}]}"#,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap())).unwrap();
        let v = ManifestValidator::new(&m);
        assert!(v.validate_node_kind("NoSuchNode").is_err());
    });
}



#[test]
fn property_value_dsl_field_round_trips_nested_array_and_object() {
    block_on_test(async {
        // 🌳️ Nested case: an Object containing an Array containing an Object — proves the
        // `dsl_core::DslField` bridge (via `dsl_core::DslValue`) recurses correctly at every depth, not just
        // for a flat value.
        let mut inner_obj = PropertyBag::new();
        inner_obj.insert("flag".to_string(), PropertyValue::Bool(true));
        inner_obj.insert("label".to_string(), PropertyValue::String("leaf".to_string()));

        let array_of_objects = PropertyValue::Array(vec![PropertyValue::Number(1.0), PropertyValue::Object(inner_obj), PropertyValue::Null]);

        let mut root = PropertyBag::new();
        root.insert("id".to_string(), PropertyValue::String("root".to_string()));
        root.insert("count".to_string(), PropertyValue::Number(3.0));
        root.insert("items".to_string(), array_of_objects);
        let value = PropertyValue::Object(root);

        let field_value = <PropertyValue as semio_framework_dsl_record::DslField>::to_value(&value);
        let round_tripped = <PropertyValue as semio_framework_dsl_record::DslField>::from_value(&field_value).expect("round trip must succeed");
        assert_eq!(round_tripped, value, "PropertyValue dsl_core::DslField round trip diverged for a nested Object/Array/Object value");
    });
}
