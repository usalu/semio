#[derive(value_derive::FromValue)]
struct Vector {
    value: Option<f64>,
    availability: String,
}
#[derive(value_derive::FromValue)]
struct Contract {
    vectors: Vec<Vector>,
}
#[semio_framework_async_macros::async_test]
async fn shared_analytic_unavailable_and_deterministic_vectors_are_typed() {
    let contract: Contract = pack::from_json_str(include_str!("../../🧪️contract/🔣️.json")).unwrap();
    assert_eq!(contract.vectors[0].value, Some(5.0));
    assert_eq!(contract.vectors[0].availability, "available");
    assert_eq!(contract.vectors[1].value, None);
    assert_eq!(contract.vectors[1].availability, "unavailable");
}
