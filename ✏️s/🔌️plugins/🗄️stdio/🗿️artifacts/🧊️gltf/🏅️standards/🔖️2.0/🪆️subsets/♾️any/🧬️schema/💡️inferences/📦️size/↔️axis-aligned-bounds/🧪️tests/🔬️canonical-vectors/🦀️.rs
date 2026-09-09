#[derive(value_derive::FromValue)]
struct Value {
    min: [f64; 3],
    max: [f64; 3],
    dimensions: [f64; 3],
}
#[derive(value_derive::FromValue)]
struct Vector {
    value: Option<Value>,
}
#[derive(value_derive::FromValue)]
struct Contract {
    vectors: Vec<Vector>,
}
#[semio_framework_async_macros::async_test]
async fn shared_analytic_unavailable_and_deterministic_bounds_vectors_are_typed() {
    let contract: Contract = pack::from_json_str(include_str!("../../🧪️contract/🔣️.json")).unwrap();
    assert_eq!(contract.vectors[0].value.as_ref().unwrap().dimensions, [3.0, 4.0, 5.0]);
    assert_eq!(contract.vectors[0].value.as_ref().unwrap().min, [1.0, 2.0, 3.0]);
    assert_eq!(contract.vectors[0].value.as_ref().unwrap().max, [4.0, 6.0, 8.0]);
    assert!(contract.vectors[1].value.is_none());
}
