
use super::JsonRpcId;
use super::JsonRpcRequest;

pub fn request_with(id: i64, method: &str, params: serde_json::Value) -> JsonRpcRequest {
    JsonRpcRequest { jsonrpc: "2.0".to_string(), id: Some(JsonRpcId::Number(id)), method: method.to_string(), params: Some(params) }
}
