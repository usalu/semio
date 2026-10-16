//! ⏳️ The result lane of an operation cancellation (lifted from the WP1 operation-progress file; WP2 wires it into decl-app).
use crate::tool_contract::TypedOperationResultLane;

pub fn cancellation_result_lane(user_requested: bool, worker_fault: bool) -> TypedOperationResultLane {
    if user_requested && !worker_fault {
        TypedOperationResultLane::Terminal
    } else {
        TypedOperationResultLane::Fault
    }
}
