//! ⚠️ Record schema construction preserves refusal authority across each ownership boundary.
use crate::{NativeSchemaControl,RecordLayout,RecordSpec,RecordSpecProducer,Shape,producer};
use semio_framework_value::{ValueError,ValueRefusalKind};

struct RefusingControl { kind:ValueRefusalKind, message:String }
impl RefusingControl { fn refusal(&self)->ValueError { ValueError::new(self.kind,self.message.clone()) } }
impl NativeSchemaControl for RefusingControl {
    fn checkpoint(&mut self)->Result<(),ValueError>{Ok(())}
    fn begin_stage(&mut self,_total:usize)->Result<(),ValueError>{Ok(())}
    fn step(&mut self)->Result<(),ValueError>{Ok(())}
    fn advance(&mut self,_units:usize)->Result<(),ValueError>{Ok(())}
    fn charge(&mut self,_bytes:usize)->Result<(),ValueError>{Err(self.refusal())}
    fn copy_text(&mut self,_text:&str)->Result<String,ValueError>{Err(self.refusal())}
    fn allocate_vec<T>(&mut self,_count:usize)->Result<Vec<T>,ValueError>{Err(self.refusal())}
    fn scoped_stage<T>(&mut self,operation:impl FnOnce(&mut Self)->Result<T,ValueError>)->Result<T,ValueError>{operation(self)}
    fn scoped_depth<T>(&mut self,_maximum:usize,operation:impl FnOnce(&mut Self)->Result<T,ValueError>)->Result<T,ValueError>{operation(self)}
    fn produce(&mut self,_producer:&RecordSpecProducer)->Result<RecordSpec,ValueError>{Err(self.refusal())}
}

#[test]
fn schema_constructors_retain_every_owned_refusal_kind_through_path_wrapping(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/⚠️refusal/🔣️.json")).unwrap();
    let kinds=[ValueRefusalKind::InvalidValue,ValueRefusalKind::Canceled,ValueRefusalKind::OwnershipLimit,ValueRefusalKind::AllocationFailed,ValueRefusalKind::WorkLimit,ValueRefusalKind::DepthLimit,ValueRefusalKind::UnsupportedOwner,ValueRefusalKind::InvariantViolated];
    for row in fixture["cases"].as_array().unwrap(){
        let kind=*kinds.iter().find(|kind|kind.as_str()==row["kind"].as_str().unwrap()).unwrap();
        let boundary=row["boundary"].as_str().unwrap();let mut control=RefusingControl{kind,message:row["message"].as_str().unwrap().to_owned()};
        let error=match boundary {
            "field"=>producer::field(1,"name",Shape::Text,&mut control).unwrap_err(),
            "record"=>producer::record(Some("record"),RecordLayout::Inline,Vec::new(),&mut control).unwrap_err(),
            "boxed"=>producer::boxed(Shape::Text,&mut control).unwrap_err(),
            _=>panic!("unknown constructor boundary"),
        }.under(boundary);
        assert_eq!(error.kind,kind);assert_eq!(serde_json::json!({"kind":error.kind.as_str(),"message":error.message}),row["expected"]);
    }
}
