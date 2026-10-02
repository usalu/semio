use super::*;
use crate::{FromValue, os_directory::client::{DirectoryWsConnection, DirectoryWsPoll, HttpResponse, TransportError}};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct TestTransport { response: Vec<u8>, calls: Arc<Mutex<Vec<String>>> }
struct TestWs;
impl DirectoryWsConnection for TestWs {
    fn send_text(&mut self, _: String) -> Result<(), TransportError> { unreachable!() }
    fn send_binary(&mut self, _: Vec<u8>) -> Result<(), TransportError> { unreachable!() }
    fn try_recv_text(&mut self) -> Result<DirectoryWsPoll, TransportError> { unreachable!() }
    fn close(&mut self) {}
}
impl DirectoryTransport for TestTransport {
    type Ws = TestWs;
    async fn http(&self, _: &OperationContext, _: HttpMethod, url: &str, _: Option<&str>, _: Option<Vec<u8>>) -> Result<HttpResponse, TransportError> {
        self.calls.lock().unwrap().push(url.into());
        Ok(HttpResponse { status: 200, body: self.response.clone() })
    }
    async fn get_accepting(&self, _: &OperationContext, _: &str, _: Option<&str>, _: &str) -> Result<HttpResponse, TransportError> { unreachable!() }
    fn issue_socket_grant(&self, _: &OperationContext, _: &str, _: &str, _: &[u8], _: u64) -> Result<HttpResponse, TransportError> { unreachable!() }
    fn open_ws(&self, _: &OperationContext, _: &str, _: &[String], _: u64) -> Result<Self::Ws, TransportError> { unreachable!() }
}
fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap() }
fn declaration(value: &serde_json::Value) -> DocumentHttpPortDeclarationV1 { DocumentHttpPortDeclarationV1::from_value(pack::json::from_json_str(&value.to_string()).unwrap()).unwrap() }
fn context() -> OperationContext { OperationContext { actor: 0, generation: 0, trace: semio_framework_async::TraceId(0), lane: 0, deadline_ms: None, cancel: semio_framework_async::CancelToken::root_now(), capability: None } }

#[semio_framework_async_macros::async_test]
async fn owner_removal_preserves_neutral_document_transport() {
    let fixture = fixture();
    let transport = TestTransport { response: fixture["reply"].to_string().into_bytes(), calls: Arc::new(Mutex::new(Vec::new())) };
    let client = DirectoryClient::new(transport.clone(), "http://neutral.test");
    let scope = DocumentScope::new("space", "document");
    let payload = pack::json::from_json_str(&fixture["request"].to_string()).unwrap();
    let mut installed = std::collections::BTreeMap::new();
    for event in fixture["lifecycle"].as_array().unwrap() {
        match event.as_str().unwrap() {
            "install-neutral" => { installed.insert("neutral", CompiledDocumentHttpPortV1::compile("neutral", declaration(&fixture["neutral"])).unwrap()); }
            "install-secondary" => { installed.insert("secondary", CompiledDocumentHttpPortV1::compile("secondary", declaration(&fixture["secondary"])).unwrap()); }
            "remove-secondary" => { installed.remove("secondary"); }
            _ => unreachable!(),
        }
        let reply = installed["neutral"].call(&client, &context(), &scope, "run", &payload).await.unwrap();
        let oracle: serde_json::Value = serde_json::from_str(&pack::json::to_json_string(&reply)).unwrap();
        assert_eq!(oracle, fixture["reply"]);
    }
    assert_eq!(transport.calls.lock().unwrap().len(), 5);
    assert!(installed.get("secondary").is_none());
    let neutral = installed.get("neutral").unwrap();
    assert_eq!(neutral.call(&client, &context(), &scope, "missing", &payload).await, Err(DocumentHttpPortCodeV1::Unavailable));
    let mut invalid = declaration(&fixture["neutral"]);
    invalid.operations[0].input_schema.clear();
    assert!(matches!(CompiledDocumentHttpPortV1::compile("neutral", invalid), Err(DocumentHttpPortCodeV1::Invalid)));
    assert!(matches!(CompiledDocumentHttpPortV1::compile("foreign", declaration(&fixture["neutral"])), Err(DocumentHttpPortCodeV1::Invalid)));
    let mut foreign_schema = declaration(&fixture["neutral"]);
    foreign_schema.operations[0].output_schema = foreign_schema.operations[0].output_schema.replace("neutral:reply", "secondary:reply");
    assert!(matches!(CompiledDocumentHttpPortV1::compile("neutral", foreign_schema), Err(DocumentHttpPortCodeV1::Invalid)));
    let bad_transport = TestTransport { response: br#"{"schema":"secondary.reply/v1","value":7}"#.to_vec(), calls: Arc::new(Mutex::new(Vec::new())) };
    let bad_client = DirectoryClient::new(bad_transport, "http://neutral.test");
    assert_eq!(neutral.call(&bad_client, &context(), &scope, "run", &payload).await, Err(DocumentHttpPortCodeV1::Invalid));
    let ctx = context(); ctx.cancel.cancel_now();
    assert_eq!(neutral.call(&client, &ctx, &scope, "run", &payload).await, Err(DocumentHttpPortCodeV1::Cancelled));
    assert_eq!(transport.calls.lock().unwrap().len(), 5);
}

#[test]
fn schema_vectors_match_owned_validator() {
    let fixture = fixture();
    let neutral = CompiledDocumentHttpPortV1::compile("neutral", declaration(&fixture["neutral"])).unwrap();
    for vector in fixture["vectors"].as_array().unwrap() {
        assert_eq!(neutral.schemas[0].0.is_valid_json(&vector["value"].to_string()), vector["valid"].as_bool().unwrap(), "{}", vector["name"]);
    }
}

#[test]
fn decoded_replies_obey_the_same_node_bounds_as_owner_inputs() {
    let fixture=fixture();let mut owner=declaration(&fixture["neutral"]);
    owner.operations[0].output_schema=fixture["replyNodeBounds"]["ownerSchema"].to_string();
    owner.operations[0].response_max_bytes=64*1024;
    let port=CompiledDocumentHttpPortV1::compile("neutral",owner).unwrap();
    for vector in fixture["replyNodeBounds"]["vectors"].as_array().unwrap() {
        let input=serde_json::Value::Array(vec![serde_json::Value::Null;vector["items"].as_u64().unwrap() as usize]);
        let bytes=serde_json::to_vec(&input).unwrap();let reply=port.decode("run",&bytes);
        if vector["valid"]==true {assert_eq!(serde_json::from_str::<serde_json::Value>(&pack::json::to_json_string(&reply.unwrap())).unwrap(),input)} else {assert_eq!(reply,Err(DocumentHttpPortCodeV1::Bounds));}
    }
}
