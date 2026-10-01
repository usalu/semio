use super::*;
use semio_framework_os_kernel::{FromValue,ToValue};
use semio_framework_async::{CancelToken,TraceId};
fn fixture()->serde_json::Value {serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap()}
fn value(input:&serde_json::Value)->DslValue {semio_framework_os_kernel::os_pack::json::from_json_str(&input.to_string()).unwrap()}
fn declaration()->DocumentHttpPortDeclarationV1 {DocumentHttpPortDeclarationV1::from_value(value(&fixture()["declaration"])).unwrap()}
fn encode(_action:&str,payload:&DslValue)->Result<DslValue,InferenceRouteErrorV1> {Ok(payload.clone())}
fn receipt(payload:DslValue)->Result<DslValue,InferenceRouteErrorV1> {Ok(payload)}
fn page(payload:DslValue,_job:&str)->Result<DslValue,InferenceRouteErrorV1> {Ok(payload)}
fn approval(payload:DslValue,_scope:&DocumentScope,_job:&str,_hash:&str)->Result<DslValue,InferenceRouteErrorV1> {Ok(payload)}
fn undo(payload:DslValue,_request:&DslValue)->Result<DslValue,InferenceRouteErrorV1> {Ok(payload)}
fn protocol()->RemoteInferenceProtocolV1 {RemoteInferenceProtocolV1 {owner:"neutral",service_id:"neutral.service",route:"operations/signal",declaration,encode,receipt,page,approval,undo,error:|status,_body|InferenceRouteErrorV1::from_status(status)}}
struct Transport {retire:bool,seen:std::sync::Mutex<Vec<String>>}
impl InferenceHubTransport for Transport {
    async fn request(&self,context:&OperationContext,request:&InferenceHubRequestV1)->Result<InferenceHubResponseV1,InferenceHubTransportErrorV1> {
        self.seen.lock().unwrap().push(request.path.clone());
        if self.retire {install_remote_inference_protocols_v1(Vec::new()).unwrap();assert!(context.cancel.is_cancelled_now());}
        Ok(InferenceHubResponseV1 {status:200,body:serde_json::to_vec(&fixture()["reply"]).unwrap()})
    }
}
#[test]
fn installed_protocol_revokes_actual_calls_and_reinstallation_creates_fresh_authority() {
    let law=fixture();let owned=value(&law);
    let independent:serde_json::Value=serde_json::from_str(&semio_framework_os_kernel::os_pack::json::to_json_string(&owned)).unwrap();
    assert_eq!(independent,law);
    for hostile in law["hostilePayloads"].as_array().unwrap() {
        let payload=match hostile["kind"].as_str().unwrap() {
            "depth"=>{let mut value=DslValue::Null;for _ in 0..=law["payloadBounds"]["maximumDepth"].as_u64().unwrap(){value=DslValue::Array(vec![value]);}value},
            "nodes"=>DslValue::Array(vec![DslValue::Null;law["payloadBounds"]["maximumNodes"].as_u64().unwrap() as usize+1]),
            "bytes"=>DslValue::String("x".repeat(law["payloadBounds"]["maximumBytes"].as_u64().unwrap() as usize+1)),
            "duplicate-key"=>DslValue::Object(vec![("same".into(),DslValue::Null),("same".into(),DslValue::Null)]),
            "unsafe-integer"=>9_007_199_254_740_992u64.to_value(),
            "unsafe-float-integer"=>9_007_199_254_740_992f64.to_value(),
            "nonfinite-number"=>f64::NAN.to_value(),
            _=>panic!("unimplemented hostile fixture"),
        };
        let code=CompiledDocumentHttpPortV1::validate_payload(&payload,law["payloadBounds"]["maximumBytes"].as_u64().unwrap() as usize).unwrap_err();
        assert_eq!(code,if hostile["expected"]=="bounds"{DocumentHttpPortCodeV1::Bounds}else{DocumentHttpPortCodeV1::Invalid});
    }

    let run=tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
    let cancel=CancelToken::root_now();let context=OperationContext {actor:1,generation:0,trace:TraceId(1),lane:1,deadline_ms:Some(u64::MAX),cancel:cancel.clone(),capability:None};
    let scope=DocumentScope::new("space:one","document:one");let payload=value(&law["payload"]);
    install_remote_inference_protocols_v1(vec![protocol()]).unwrap();
    assert!(install_remote_inference_protocols_v1(vec![protocol(),protocol()]).is_err());
    let transport=Transport {retire:true,seen:Default::default()};
    let result=run.block_on(call_remote_inference_v1(&transport,&context,"https://owner.invalid",&scope,"operations/signal","submit",&payload));
    assert_eq!(result.unwrap_err().code(),law["expectedCodes"][0].as_str().unwrap());
    assert!(!cancel.is_cancelled_now());
    assert_eq!(transport.seen.lock().unwrap().as_slice(),["/spaces/space%3Aone/documents/document%3Aone/operations/signal"]);
    assert_eq!(remote_inference_protocol_v1("operations/signal").err().unwrap().code(),law["expectedCodes"][1].as_str().unwrap());
    install_remote_inference_protocols_v1(vec![protocol()]).unwrap();
    let transport=Transport {retire:false,seen:Default::default()};
    assert_eq!(run.block_on(call_remote_inference_v1(&transport,&context,"https://owner.invalid",&scope,"operations/signal","submit",&payload)).unwrap(),value(&law["reply"]));
    install_remote_inference_protocols_v1(Vec::new()).unwrap();
}
