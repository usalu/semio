use super::*;

#[semio_framework_async_macros::async_test]
async fn empty_diff_is_a_no_operation() {
    let base = crate::schema::empty_energy_model_snapshot();
    let diff = EnergyModelDiff::default();
    assert!(DiffAlgebra::is_empty(&diff));
    assert_eq!(protocol::apply_diff(&diff, &base).expect("valid mutation diff"), base);
}
