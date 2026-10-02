//! 🏭️ Lazy schema ownership under the same native input or output control.
use super::{FieldSpec,RecordSpec};
use crate::os_dsl::{NativeDecodeControl,NativeEncodeControl};

/// 🧮️ The allocation authority used by explicit schema constructors in both directions.
pub trait NativeSchemaControl:Sized{
    fn checkpoint(&mut self)->Result<(),String>;
    fn begin_stage(&mut self,total:usize)->Result<(),String>;
    fn step(&mut self)->Result<(),String>;
    fn advance(&mut self,units:usize)->Result<(),String>;
    fn charge(&mut self,bytes:usize)->Result<(),String>;
    fn copy_text(&mut self,text:&str)->Result<String,String>;
    fn allocate_vec<T>(&mut self,count:usize)->Result<Vec<T>,String>;
    fn scoped_stage<T>(&mut self,operation:impl FnOnce(&mut Self)->Result<T,String>)->Result<T,String>;
    fn scoped_depth<T>(&mut self,maximum:usize,operation:impl FnOnce(&mut Self)->Result<T,String>)->Result<T,String>;
    fn produce(&mut self,producer:&RecordSpecProducer)->Result<RecordSpec,String>;
}

macro_rules! control_port{
    ($control:ident,$direction:ident)=>{impl NativeSchemaControl for $control<'_>{
        fn checkpoint(&mut self)->Result<(),String>{$control::checkpoint(self)}
        fn begin_stage(&mut self,total:usize)->Result<(),String>{$control::begin_stage(self,total)}
        fn step(&mut self)->Result<(),String>{$control::step(self)}
        fn advance(&mut self,units:usize)->Result<(),String>{$control::advance(self,units)}
        fn charge(&mut self,bytes:usize)->Result<(),String>{$control::charge(self,bytes)}
        fn copy_text(&mut self,text:&str)->Result<String,String>{$control::copy_text(self,text)}
        fn allocate_vec<T>(&mut self,count:usize)->Result<Vec<T>,String>{$control::allocate_vec(self,count)}
        fn scoped_stage<T>(&mut self,operation:impl FnOnce(&mut Self)->Result<T,String>)->Result<T,String>{$control::scoped_stage(self,operation)}
        fn scoped_depth<T>(&mut self,maximum:usize,operation:impl FnOnce(&mut Self)->Result<T,String>)->Result<T,String>{$control::scoped_depth(self,maximum,operation)}
        fn produce(&mut self,producer:&RecordSpecProducer)->Result<RecordSpec,String>{producer.$direction(self)}
    }};
}
control_port!(NativeDecodeControl,decode);
control_port!(NativeEncodeControl,encode);

/// 🪆️ Carries three explicit owner factories without unfolding a recursive schema.
#[derive(Clone,Copy,Debug)]
pub struct RecordSpecProducer{
    pub ordinary:fn()->RecordSpec,
    pub decoding:fn(&mut NativeDecodeControl<'_>)->Result<RecordSpec,String>,
    pub encoding:fn(&mut NativeEncodeControl<'_>)->Result<RecordSpec,String>,
}

impl RecordSpecProducer{
    /// 🛬️ Owns schema metadata before the physical parser can allocate native values.
    pub fn decode(&self,control:&mut NativeDecodeControl<'_>)->Result<RecordSpec,String>{control.scoped_depth(64,|control|control.scoped_stage(|control|{control.begin_stage(0)?;(self.decoding)(control)}))}
    /// 🛫️ Owns schema metadata before typed projection or physical output begins.
    pub fn encode(&self,control:&mut NativeEncodeControl<'_>)->Result<RecordSpec,String>{control.scoped_depth(64,|control|control.scoped_stage(|control|{control.begin_stage(0)?;(self.encoding)(control)}))}
}

/// 🏷️ Copies a literal authored field key under the caller's cumulative metadata ceiling.
pub fn field<C:NativeSchemaControl>(id:u16,key:&str,shape:super::Shape,control:&mut C)->Result<FieldSpec,String>{
    Ok(FieldSpec{id,key:control.copy_text(key)?,position:None,shape,optional:false,flatten:false,defines:None,is_call_name:false})
}

/// 🧾️ Copies the authored keyword while retaining the already admitted field frontier.
pub fn record<C:NativeSchemaControl>(keyword:Option<&str>,layout:super::RecordLayout,fields:Vec<FieldSpec>,control:&mut C)->Result<RecordSpec,String>{
    Ok(RecordSpec{keyword:keyword.map(|keyword|control.copy_text(keyword)).transpose()?,layout,fields})
}

/// 📦️ Admits one shape wrapper before allocating its owned child slot.
pub fn boxed<C:NativeSchemaControl>(shape:super::Shape,control:&mut C)->Result<Box<super::Shape>,String>{control.charge(std::mem::size_of::<super::Shape>())?;Ok(Box::new(shape))}
