use super::*;

#[derive(Debug)]
struct Refusing(ValueError);
impl DslField for Refusing {
    fn shape()->Shape{Shape::Text}
    fn to_value(&self)->FieldValue{panic!("ordinary conversion is outside this controlled law")}
    fn from_value(_: &FieldValue)->Result<Self,String>{panic!("ordinary conversion is outside this controlled law")}
    fn to_value_controlled(&self,_:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{Err(self.0.clone())}
    fn from_value_controlled(value:&FieldValue,_:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{let FieldValue::Text(text)=value else{panic!("authored refusal carrier")};let fields:serde_json::Value=serde_json::from_str(text).unwrap();Err(ValueError::new(refusal_kind(fields["kind"].as_str().unwrap()),fields["message"].as_str().unwrap()))}
}
#[derive(Debug,DslRecord)]
struct ControlledRefusalRecord { value:Box<Refusing> }

fn refusal_kind(name:&str)->ValueRefusalKind{
    match name{"InvalidValue"=>ValueRefusalKind::InvalidValue,"Canceled"=>ValueRefusalKind::Canceled,"OwnershipLimit"=>ValueRefusalKind::OwnershipLimit,"AllocationFailed"=>ValueRefusalKind::AllocationFailed,"WorkLimit"=>ValueRefusalKind::WorkLimit,"DepthLimit"=>ValueRefusalKind::DepthLimit,"UnsupportedOwner"=>ValueRefusalKind::UnsupportedOwner,"InvariantViolated"=>ValueRefusalKind::InvariantViolated,_=>panic!("unknown authored refusal kind")}
}

#[test]
fn os_controlled_field_derive_retains_all_refusal_categories_and_position(){
    let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../../../../🔨️modules/🗣️dsl/🧬️schema/🧫️fixtures/🪆️refusal/🔣️.json")).unwrap();
    for name in corpus["kinds"].as_array().unwrap(){
        let kind=refusal_kind(name.as_str().unwrap());let mut record=RecordValue::default();record.fields.insert(0,FieldValue::Text(serde_json::to_string(&serde_json::json!({"kind":name,"message":"refused field"})).unwrap()));
        let error=ControlledRefusalRecord::__dsl_from_record_controlled(&record,&mut NativeDecodeControl::new(4096,&mut |_|true)).unwrap_err();assert_eq!(error.kind,kind);assert_eq!(error.message,"refused field");
        let position=TextSpan::at(7,11);let positioned=TextError::from_value_error(error,position);assert_eq!(positioned.kind,kind);assert_eq!(positioned.span,position);
        let value=ControlledRefusalRecord{value:Box::new(Refusing(ValueError::new(kind,"refused field")))};
        let error=value.__dsl_to_record_controlled(&mut NativeEncodeControl::new(4096,&mut |_|true)).unwrap_err();assert_eq!(error.kind,kind);assert_eq!(error.message,"refused field");
        let error=native_encoding::project_list(&[value],&mut NativeEncodeControl::new(4096,&mut |_|true)).unwrap_err();assert_eq!(error.kind,kind);
    }
}
