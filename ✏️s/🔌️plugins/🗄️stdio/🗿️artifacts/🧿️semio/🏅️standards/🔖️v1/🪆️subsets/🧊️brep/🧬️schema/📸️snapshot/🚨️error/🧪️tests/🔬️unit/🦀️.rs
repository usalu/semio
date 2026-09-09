use super::*;

#[semio_framework_async_macros::async_test]
async fn kernel_error_displays_readable_message() {
    let e = KernelError::InvalidInput("radius must be positive".to_string());
    assert_eq!(e.to_string(), "invalid input: radius must be positive");
}

#[semio_framework_async_macros::async_test]
async fn intersect_error_converts_into_kernel_error() {
    let e: KernelError = IntersectError::Tangent.into();
    assert!(matches!(e, KernelError::Intersect(IntersectError::Tangent)));
}

#[semio_framework_async_macros::async_test]
async fn boolean_error_wraps_intersect_error() {
    let e: BooleanError = IntersectError::Degenerate("zero length".to_string()).into();
    assert!(matches!(e, BooleanError::Intersect(IntersectError::Degenerate(_))));
}

#[semio_framework_async_macros::async_test]
async fn validation_issue_displays_code_entity_message() {
    let issue = ValidationIssue { entity: "edge-3".to_string(), code: "same-parameter-violated", message: "residual 1e-3 exceeds tol 1e-6".to_string() };
    assert_eq!(issue.to_string(), "[same-parameter-violated] edge-3: residual 1e-3 exceeds tol 1e-6");
}
