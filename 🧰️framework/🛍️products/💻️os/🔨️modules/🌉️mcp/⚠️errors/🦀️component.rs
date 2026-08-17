//! ⚠️ Gateway-domain error type and its JSON-RPC/MCP-tool-result projections, shared by every
//! capability surface `semio-framework-os-mcp` exposes. The code set mirrors
//! `📋️master.md` §3.3 verbatim (frozen — a later packet that needs a new failure mode maps it onto
//! one of these twelve, it does not add a thirteenth). This facet has ZERO dependency on
//! `crate::protocol` — errors flow outward TO the protocol layer (which builds a JSON-RPC error
//! response or a `CallToolResult{isError:true}` payload from a `GatewayError`), never the reverse,
//! so a future backend crate can construct one without pulling in JSON-RPC framing at all.

use serde::{Deserialize, Serialize};

//#region 🔖️GatewayErrorCode
/// 🏷️ The twelve gateway failure codes every capability surface can raise — see this file's module
/// doc for the frozen source of truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GatewayErrorCode {
    InputInvalid,
    PreconditionFailed,
    RevisionConflict,
    PermissionDenied,
    ApprovalRequired,
    PluginUnavailable,
    SideEffectRejected,
    Cancelled,
    CompensationFailed,
    NotFound,
    BudgetExceeded,
    Internal,
}

impl GatewayErrorCode {
    /// 🔢️ The nearest-fit JSON-RPC 2.0 standard error code for this gateway code when it surfaces as
    /// a PROTOCOL-level failure (as opposed to a tool-result `isError:true` payload) — see
    /// `luna-mcpspec-audit.md` §"Error Codes (JSON-RPC)". Client-fault codes (bad input, unknown
    /// target, conflicting precondition) map to Invalid Params; everything else — the gateway itself
    /// or a downstream plugin/policy refusing — maps to Internal Error, since none of them are the
    /// caller having sent a malformed request.
    pub fn json_rpc_code(self) -> i64 {
        match self {
            GatewayErrorCode::InputInvalid | GatewayErrorCode::NotFound | GatewayErrorCode::PreconditionFailed | GatewayErrorCode::RevisionConflict => -32602,
            GatewayErrorCode::PermissionDenied
            | GatewayErrorCode::ApprovalRequired
            | GatewayErrorCode::PluginUnavailable
            | GatewayErrorCode::SideEffectRejected
            | GatewayErrorCode::Cancelled
            | GatewayErrorCode::CompensationFailed
            | GatewayErrorCode::BudgetExceeded
            | GatewayErrorCode::Internal => -32603,
        }
    }
}
//#endregion 🔖️GatewayErrorCode

//#region 🔖️GatewayError
/// 🚧️ The one error shape every gateway operation returns internally, before a call site decides
/// whether it becomes a JSON-RPC protocol error or a tool-result `isError:true` payload. Implements
/// `std::error::Error` (via `thiserror`) so it composes with `?` in ordinary fallible Rust code, not
/// only at the JSON-RPC/tool-result boundary.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema, thiserror::Error)]
#[error("{code:?}: {message}")]
pub struct GatewayError {
    pub code: GatewayErrorCode,
    pub message: String,
    #[serde(default)]
    pub details: serde_json::Value,
    pub retryable: bool,
}

impl GatewayError {
    /// 🆕️ A fresh, non-retryable error with empty `details`.
    pub fn new(code: GatewayErrorCode, message: impl Into<String>) -> Self {
        Self { code, message: message.into(), details: serde_json::Value::Null, retryable: false }
    }

    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = details;
        self
    }

    pub fn retryable(mut self) -> Self {
        self.retryable = true;
        self
    }

    /// 🧾️ Structured payload for an MCP `CallToolResult{isError:true, structuredContent: ..}` — a
    /// TOOL-execution failure, never turned into a JSON-RPC error response (`protocol`'s dispatcher
    /// is the single call site that makes this choice; see its own module doc).
    pub fn to_tool_error_payload(&self) -> serde_json::Value {
        serde_json::json!({ "code": self.code, "message": self.message, "details": self.details, "retryable": self.retryable })
    }

    /// 🧾️ `(json_rpc_code, message, data)` for a JSON-RPC error response — used only when a
    /// `GatewayError` represents a PROTOCOL failure (malformed request, unknown method/capability id
    /// resolved before any tool body ran).
    pub fn to_json_rpc_parts(&self) -> (i64, String, serde_json::Value) {
        (self.code.json_rpc_code(), self.message.clone(), serde_json::json!({ "gatewayCode": self.code, "details": self.details, "retryable": self.retryable }))
    }
}
//#endregion 🔖️GatewayError

//#region 🧪️Tests
#[cfg(test)]
mod quick {
    use super::*;

    #[test]
    fn code_serializes_screaming_snake() {
        assert_eq!(serde_json::to_string(&GatewayErrorCode::InputInvalid).unwrap(), "\"INPUT_INVALID\"");
        assert_eq!(serde_json::to_string(&GatewayErrorCode::SideEffectRejected).unwrap(), "\"SIDE_EFFECT_REJECTED\"");
    }

    #[test]
    fn tool_error_payload_carries_all_fields() {
        let error = GatewayError::new(GatewayErrorCode::NotFound, "no such capability").with_details(serde_json::json!({"capabilityId": "x"}));
        let payload = error.to_tool_error_payload();
        assert_eq!(payload["code"], "NOT_FOUND");
        assert_eq!(payload["message"], "no such capability");
        assert_eq!(payload["details"]["capabilityId"], "x");
        assert_eq!(payload["retryable"], false);
    }

    #[test]
    fn json_rpc_parts_map_input_invalid_to_invalid_params() {
        let error = GatewayError::new(GatewayErrorCode::InputInvalid, "bad arg");
        let (code, message, _data) = error.to_json_rpc_parts();
        assert_eq!(code, -32602);
        assert_eq!(message, "bad arg");
    }

    #[test]
    fn implements_std_error_via_thiserror() {
        let error = GatewayError::new(GatewayErrorCode::Internal, "boom");
        let boxed: Box<dyn std::error::Error> = Box::new(error.clone());
        assert_eq!(boxed.to_string(), "Internal: boom");
        assert_eq!(error.to_string(), "Internal: boom");
    }

    #[test]
    fn retryable_flag_round_trips() {
        let error = GatewayError::new(GatewayErrorCode::BudgetExceeded, "quota").retryable();
        assert!(error.retryable);
        let payload = error.to_tool_error_payload();
        assert_eq!(payload["retryable"], true);
    }

    #[test]
    fn every_code_round_trips_through_json() {
        let codes = [
            GatewayErrorCode::InputInvalid,
            GatewayErrorCode::PreconditionFailed,
            GatewayErrorCode::RevisionConflict,
            GatewayErrorCode::PermissionDenied,
            GatewayErrorCode::ApprovalRequired,
            GatewayErrorCode::PluginUnavailable,
            GatewayErrorCode::SideEffectRejected,
            GatewayErrorCode::Cancelled,
            GatewayErrorCode::CompensationFailed,
            GatewayErrorCode::NotFound,
            GatewayErrorCode::BudgetExceeded,
            GatewayErrorCode::Internal,
        ];
        for code in codes {
            let json = serde_json::to_string(&code).unwrap();
            let back: GatewayErrorCode = serde_json::from_str(&json).unwrap();
            assert_eq!(back, code);
        }
    }
}
//#endregion 🧪️Tests
