//! 🧲️ A bounded record leaf receives through one macro declaration exactly like its DslField encoding, and a refusal keeps the source.
use semio_framework_dsl_record::receiving::owned::{DslOwnedReceivingField,OwnedScalar,invalid};
use semio_framework_dsl_record::{DslField,DslReceivingField,FieldValue,RecordValue,Shape};

macro_rules! dsl_owned_scalar_receiving{
 (copy $name:ty,$convert:expr)=>{
  impl semio_framework_dsl_record::receiving::owned::scalar_sealed::Sealed for $name{}
  impl semio_framework_dsl_record::receiving::owned::OwnedScalar for $name{
   fn creation_bytes()->usize{::core::mem::size_of::<Self>()}
   fn take_source(source:&mut Option<semio_framework_dsl_record::FieldValue>)->Result<Self,semio_framework_dsl_record::receiving::owned::ValueError>{let convert:fn(&semio_framework_dsl_record::FieldValue)->Result<Self,semio_framework_dsl_record::receiving::owned::ValueError>=$convert;convert(source.as_ref().ok_or_else(semio_framework_dsl_record::receiving::owned::invalid)?)}
  }
  impl semio_framework_dsl_record::receiving::owned::DslOwnedReceivingField for $name{type Cursor=semio_framework_dsl_record::receiving::owned::ScalarReceiving<Self>;fn receiving_cursor()->Self::Cursor{::core::default::Default::default()}}
  impl semio_framework_dsl_record::DslReceivingField for $name{fn receive_value(value:&semio_framework_dsl_record::FieldValue,owner:&mut semio_framework_dsl_record::receiving::NativeSnapshotDecodeOwner<'_,'_>)->semio_framework_dsl_record::receiving::owned::RetainedWorkOutcome<Self,semio_framework_dsl_record::receiving::NativeSnapshotFailure>{
   let convert:fn(&semio_framework_dsl_record::FieldValue)->Result<Self,semio_framework_dsl_record::receiving::owned::ValueError>=$convert;
   semio_framework_dsl_record::receiving::receiving_turn(|meter|{meter.admit(owner.native())?;let output=convert(value).map_err(semio_framework_dsl_record::receiving::NativeSnapshotFailure::from)?;semio_framework_dsl_record::receiving::work(owner,::core::mem::size_of::<Self>(),1)?;Ok(output)})
  }}
 };
 (move $name:ty,$payload:expr,$convert:expr,$take:expr)=>{
  impl semio_framework_dsl_record::receiving::owned::scalar_sealed::Sealed for $name{}
  impl semio_framework_dsl_record::receiving::owned::OwnedScalar for $name{
   fn creation_bytes()->usize{0}
   fn take_source(source:&mut Option<semio_framework_dsl_record::FieldValue>)->Result<Self,semio_framework_dsl_record::receiving::owned::ValueError>{let take:fn(&mut Option<semio_framework_dsl_record::FieldValue>)->Result<Self,semio_framework_dsl_record::receiving::owned::ValueError>=$take;take(source)}
  }
  impl semio_framework_dsl_record::receiving::owned::DslOwnedReceivingField for $name{type Cursor=semio_framework_dsl_record::receiving::owned::ScalarReceiving<Self>;fn receiving_cursor()->Self::Cursor{::core::default::Default::default()}}
  impl semio_framework_dsl_record::DslReceivingField for $name{fn receive_value(value:&semio_framework_dsl_record::FieldValue,owner:&mut semio_framework_dsl_record::receiving::NativeSnapshotDecodeOwner<'_,'_>)->semio_framework_dsl_record::receiving::owned::RetainedWorkOutcome<Self,semio_framework_dsl_record::receiving::NativeSnapshotFailure>{
   let payload:fn(&semio_framework_dsl_record::FieldValue)->usize=$payload;let convert:fn(&semio_framework_dsl_record::FieldValue)->Result<Self,semio_framework_dsl_record::receiving::owned::ValueError>=$convert;
   semio_framework_dsl_record::receiving::receiving_turn(|meter|{meter.admit(owner.native())?;semio_framework_dsl_record::receiving::work(owner,payload(value).max(::core::mem::size_of::<Self>()),1)?;let output=convert(value).map_err(semio_framework_dsl_record::receiving::NativeSnapshotFailure::from)?;Ok(output)})
  }}
 };
}

#[derive(Clone,Copy,Debug,PartialEq,semio_framework_value::RetireOwned)]
struct Pose{x:f64,y:f64}
fn pose(value:&FieldValue)->Result<Pose,semio_framework_value::ValueError>{
 let FieldValue::Record(record)=value else{return Err(invalid())};
 if record.fields.len()>2{return Err(invalid())}
 match(record.get(1),record.get(2)){(Some(FieldValue::Float(x)),Some(FieldValue::Float(y)))=>Ok(Pose{x:*x,y:*y}),_=>Err(invalid())}
}
impl DslField for Pose{
 fn shape()->Shape{Shape::Float}
 fn to_value(&self)->FieldValue{FieldValue::Record(RecordValue{fields:[(1,FieldValue::Float(self.x)),(2,FieldValue::Float(self.y))].into_iter().collect()})}
 fn from_value(value:&FieldValue)->Result<Self,String>{pose(value).map_err(|error|error.to_string())}
}
dsl_owned_scalar_receiving!(copy Pose,pose);
fn implements<T:DslReceivingField+DslOwnedReceivingField+OwnedScalar>(){}

#[test]
fn bounded_leaf_receives_its_dsl_field_encoding_and_agrees_with_the_codec(){
 implements::<Pose>();
 for value in [Pose{x:0.0,y:-1.5},Pose{x:1e300,y:f64::MIN_POSITIVE}]{
  let encoded=value.to_value();
  let mut source=Some(encoded.clone());
  assert_eq!(<Pose as OwnedScalar>::take_source(&mut source).unwrap(),value);
  assert_eq!(<Pose as OwnedScalar>::take_source(&mut source).unwrap(),Pose::from_value(&encoded).unwrap(),"agrees with the existing codec");
  assert_eq!(<Pose as OwnedScalar>::creation_bytes(),std::mem::size_of::<Pose>());
 }
}

#[test]
fn bounded_leaf_refuses_foreign_and_oversized_records_without_consuming_the_source(){
 let oversized=FieldValue::Record(RecordValue{fields:(0..64u16).map(|id|(id,FieldValue::Float(0.0))).collect()});
 for foreign in [FieldValue::Bool(true),FieldValue::Absent,oversized]{
  let mut source=Some(foreign.clone());
  assert!(<Pose as OwnedScalar>::take_source(&mut source).is_err(),"refused: {foreign:?}");
  assert!(source.is_some());
 }
}
