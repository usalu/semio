
use super::*;
use crate::demo_space_projection;

#[semio_framework_async_macros::async_test]
async fn patch_parameter_op_updates_numeric_value() {
    let projection = demo_space_projection().await;
    let patch = pack::json::object([("value".to_string(), Value::from(48.0))]);
    let operation = patch_parameter_operation(&projection, "param-brush-size", &patch).await.expect("operation");
    match operation {
        WorkflowMutation::ChangeParameter(ChangeParameter { parameter, .. }) => match *parameter {
            WorkflowParameter::Numeric { value, .. } => assert_eq!(value, 48.0),
            _ => panic!("expected numeric"),
        },
        _ => panic!("expected change parameter operation"),
    }
}

#[semio_framework_async_macros::async_test]
async fn compiled_dag_wire_literal_mentions_app_instances() {
    let wire = compiled_dag_wire_literal(&demo_space_projection().await).await;
    assert!(wire.contains("appInstance") || wire.contains("draw"));
}
