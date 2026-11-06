//! 🏭️ Lazy schema ownership under the same native input or output control.
use super::{FieldSpec,RecordSpec};
use semio_framework_value::ValueError;
use crate::{NativeDecodeControl,NativeEncodeControl};

/// 🧮️ The allocation authority used by explicit schema constructors in both directions.
pub trait NativeSchemaControl:Sized{
    fn select_schema(&mut self)->Result<(),ValueError>;
    fn checkpoint(&mut self)->Result<(),ValueError>;
    fn begin_stage(&mut self,total:usize)->Result<(),ValueError>;
    fn step(&mut self)->Result<(),ValueError>;
    fn advance(&mut self,units:usize)->Result<(),ValueError>;
    fn charge(&mut self,bytes:usize)->Result<(),ValueError>;
    fn copy_text(&mut self,text:&str)->Result<String,ValueError>;
    fn allocate_vec<T>(&mut self,count:usize)->Result<Vec<T>,ValueError>;
    fn scoped_stage<T>(&mut self,operation:impl FnOnce(&mut Self)->Result<T,ValueError>)->Result<T,ValueError>;
    fn scoped_depth<T>(&mut self,maximum:usize,operation:impl FnOnce(&mut Self)->Result<T,ValueError>)->Result<T,ValueError>;
    fn produce(&mut self,producer:&RecordSpecProducer)->Result<RecordSpec,ValueError>;
}

trait SchemaSelection{fn select_schema(&mut self)->Result<(),ValueError>;}
impl SchemaSelection for NativeDecodeControl<'_>{fn select_schema(&mut self)->Result<(),ValueError>{
 if self.has_work_recipient(){if !self.prepare_work(1)?{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"native schema selection exceeds original work loan"))}self.commit_work(1)?;}Ok(())
}}
impl SchemaSelection for NativeEncodeControl<'_>{fn select_schema(&mut self)->Result<(),ValueError>{
 if self.has_work_recipient(){if !self.prepare_work(1)?{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"native schema selection exceeds original encoder work loan"))}self.commit_work(1)?;}Ok(())
}}

macro_rules! control_port{
    ($control:ident,$direction:ident)=>{impl NativeSchemaControl for $control<'_>{
        fn select_schema(&mut self)->Result<(),ValueError>{SchemaSelection::select_schema(self)}
        fn checkpoint(&mut self)->Result<(),ValueError>{$control::checkpoint(self)}
        fn begin_stage(&mut self,total:usize)->Result<(),ValueError>{$control::begin_stage(self,total)}
        fn step(&mut self)->Result<(),ValueError>{$control::step(self)}
        fn advance(&mut self,units:usize)->Result<(),ValueError>{$control::advance(self,units)}
        fn charge(&mut self,bytes:usize)->Result<(),ValueError>{$control::charge(self,bytes)}
        fn copy_text(&mut self,text:&str)->Result<String,ValueError>{$control::copy_text(self,text)}
        fn allocate_vec<T>(&mut self,count:usize)->Result<Vec<T>,ValueError>{$control::allocate_vec(self,count)}
        fn scoped_stage<T>(&mut self,operation:impl FnOnce(&mut Self)->Result<T,ValueError>)->Result<T,ValueError>{$control::scoped_stage(self,operation)}
        fn scoped_depth<T>(&mut self,maximum:usize,operation:impl FnOnce(&mut Self)->Result<T,ValueError>)->Result<T,ValueError>{$control::scoped_depth(self,maximum,operation)}
        fn produce(&mut self,producer:&RecordSpecProducer)->Result<RecordSpec,ValueError>{producer.$direction(self)}
    }};
}
control_port!(NativeDecodeControl,decode);
control_port!(NativeEncodeControl,encode);

/// 🪆️ Carries three explicit owner factories without unfolding a recursive schema.
#[derive(Clone,Copy,Debug)]
pub struct RecordSpecProducer{
    pub ordinary:fn()->RecordSpec,
    pub decoding:fn(&mut NativeDecodeControl<'_>)->Result<RecordSpec,ValueError>,
    pub encoding:fn(&mut NativeEncodeControl<'_>)->Result<RecordSpec,ValueError>,
}

impl RecordSpecProducer{
    /// 🛬️ Owns schema metadata before the physical parser can allocate native values.
    pub fn decode(&self,control:&mut NativeDecodeControl<'_>)->Result<RecordSpec,ValueError>{control.scoped_depth(64,|control|control.scoped_stage(|control|{control.begin_stage(0)?;(self.decoding)(control)}))}
    /// 🛫️ Owns schema metadata before typed projection or physical output begins.
    pub fn encode(&self,control:&mut NativeEncodeControl<'_>)->Result<RecordSpec,ValueError>{control.scoped_depth(64,|control|control.scoped_stage(|control|{control.begin_stage(0)?;(self.encoding)(control)}))}
}

/// 🏷️ Copies a literal authored field key under the caller's cumulative metadata ceiling.
pub fn field<C:NativeSchemaControl>(id:u16,key:&str,shape:super::Shape,control:&mut C)->Result<FieldSpec,ValueError>{
    control.select_schema()?;Ok(FieldSpec{id,key:control.copy_text(key)?,position:None,shape,optional:false,flatten:false,defines:None,is_call_name:false})
}

/// 🧾️ Copies the authored keyword while retaining the already admitted field frontier.
pub fn record<C:NativeSchemaControl>(keyword:Option<&str>,layout:super::RecordLayout,fields:Vec<FieldSpec>,control:&mut C)->Result<RecordSpec,ValueError>{
    control.select_schema()?;Ok(RecordSpec{keyword:keyword.map(|keyword|control.copy_text(keyword)).transpose()?,layout,fields})
}

/// 📦️ Admits one shape wrapper before allocating its owned child slot.
pub fn boxed<C:NativeSchemaControl>(shape:super::Shape,control:&mut C)->Result<Box<super::Shape>,ValueError>{control.select_schema()?;control.charge(size_of::<super::Shape>())?;Ok(Box::new(shape))}

#[cfg(test)]
mod work_tests{
 use super::*;
 use semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient;
 struct Loan(u64);
 impl NativeDecodeWorkRecipient for Loan{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{let limit:u64=self.0;limit.checked_sub(observed).ok_or_else(||ValueError::invariant("native observed work exceeds its actual finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|total|total<=self.0))}}
 #[test]
 fn original_schema_field_selection_obeys_real_work_loan(){
  let f:serde_json::Value=serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🏭️producer/🎟️work.json")).unwrap();
  for case in f["cases"].as_array().unwrap(){
   let mut observer=|_|true;let mut native=NativeDecodeControl::new(1024,&mut observer);let mut loan=Loan(case["allowance"].as_u64().unwrap());
   native.with_work_recipient(&mut loan,|native|{
    let result=field(1,f["key"].as_str().unwrap(),super::super::Shape::Text,native);
    let kind=match result{Ok(field)=>{assert_eq!(field.key,f["key"].as_str().unwrap());let grant=serde_json::from_value(f["close"].clone()).unwrap();let mut owner=semio_framework_value::retirement::controlled::ControlledRetirement::new(field).ok().unwrap();for _ in 0..4096{if owner.terminal_is_empty(){break}owner.step(grant).unwrap();}assert!(owner.terminal_is_empty());"success"},Err(error)=>{assert_eq!(native.owned_bytes(),0);assert_eq!(error.kind.as_str(),case["kind"].as_str().unwrap());"workLimit"}};
    assert_eq!(serde_json::json!({"allowance":case["allowance"],"work":native.actual_work(),"kind":kind}),*case);Ok::<(),ValueError>(())
   }).unwrap();
  }
 }
}
