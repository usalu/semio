//! 🗂️ Rule layout owns literal unique keys and concrete contiguous point slots.
use crate::LayoutPoint;
use semio_framework_value::{DecodedValue,DslValue,FromValue,NativeDecodeControl,NativeEncodeControl,ToValue,ValueError,ValueRefusalKind};
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::RetainedClone)]
pub struct RuleLayout { members:Vec<(String,LayoutPoint)> }
impl RuleLayout {
 pub fn new()->Self{Self::default()}
 pub fn len(&self)->usize{self.members.len()}
 pub fn is_empty(&self)->bool{self.members.is_empty()}
 pub fn iter(&self)->impl DoubleEndedIterator<Item=(&String,&LayoutPoint)>+ExactSizeIterator{self.members.iter().map(|(key,value)|(key,value))}
 pub fn keys(&self)->impl DoubleEndedIterator<Item=&String>+ExactSizeIterator{self.members.iter().map(|(key,_)|key)}
 pub fn values(&self)->impl DoubleEndedIterator<Item=&LayoutPoint>+ExactSizeIterator{self.members.iter().map(|(_,value)|value)}
 pub fn into_values(self)->impl DoubleEndedIterator<Item=LayoutPoint>+ExactSizeIterator{self.members.into_iter().map(|(_,value)|value)}
 pub fn get(&self,key:&str)->Option<&LayoutPoint>{self.members.binary_search_by(|(name,_)|name.as_str().cmp(key)).ok().map(|index|&self.members[index].1)}
 pub fn get_mut(&mut self,key:&str)->Option<&mut LayoutPoint>{self.members.binary_search_by(|(name,_)|name.as_str().cmp(key)).ok().map(|index|&mut self.members[index].1)}
 pub fn contains_key(&self,key:&str)->bool{self.get(key).is_some()}
 pub fn insert(&mut self,key:String,value:LayoutPoint)->Option<LayoutPoint>{match self.members.binary_search_by(|(name,_)|name.cmp(&key)){Ok(index)=>Some(std::mem::replace(&mut self.members[index].1,value)),Err(index)=>{self.members.insert(index,(key,value));None}}}
 pub fn remove(&mut self,key:&str)->Option<LayoutPoint>{self.members.binary_search_by(|(name,_)|name.as_str().cmp(key)).ok().map(|index|self.members.remove(index).1)}
 pub fn pop_last(&mut self)->Option<(String,LayoutPoint)>{self.members.pop()}
 pub fn successor(&self,key:&str)->Option<(&String,&LayoutPoint)>{self.members.get(self.members.partition_point(|(name,_)|name.as_str()<=key)).map(|(key,value)|(key,value))}
 pub fn first_key_value(&self)->Option<(&String,&LayoutPoint)>{self.members.first().map(|(key,value)|(key,value))}
 pub fn from_admitted(members:Vec<(String,LayoutPoint)>)->Self{Self{members}}
 pub fn admitted_insert(&mut self,key:String,value:LayoutPoint,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{
  self.insert_controlled(key,value,&mut||control.step())
 }
 pub fn insert_controlled(&mut self,key:String,value:LayoutPoint,checkpoint:&mut dyn FnMut()->Result<(),ValueError>)->Result<(),ValueError>{
  let value=DecodedValue::new(value,<LayoutPoint as FromValue>::retire_decoded);
  let mut index=0;while index<self.members.len(){checkpoint()?;match compare(&self.members[index].0,&key,checkpoint)?{std::cmp::Ordering::Less=>index+=1,std::cmp::Ordering::Equal=>{<LayoutPoint as FromValue>::retire_decoded(std::mem::replace(&mut self.members[index].1,value.take()));return Ok(())},std::cmp::Ordering::Greater=>break}}
  if self.members.len()==self.members.capacity(){return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"rule layout exceeds admitted point slots"));}
  self.members.push((key,value.take()));let mut position=self.members.len()-1;while position>index{self.members.swap(position,position-1);position-=1;checkpoint()?;}Ok(())
 }
}
fn compare(left:&str,right:&str,checkpoint:&mut dyn FnMut()->Result<(),ValueError>)->Result<std::cmp::Ordering,ValueError>{for(index,(left,right))in left.bytes().zip(right.bytes()).enumerate(){if index%256==0{checkpoint()?;}if left!=right{return Ok(left.cmp(&right))}}Ok(left.len().cmp(&right.len()))}
impl<const N:usize> From<[(String,LayoutPoint);N]> for RuleLayout{fn from(values:[(String,LayoutPoint);N])->Self{values.into_iter().collect()}}
impl FromIterator<(String,LayoutPoint)> for RuleLayout{fn from_iter<T:IntoIterator<Item=(String,LayoutPoint)>>(values:T)->Self{let mut result=Self::new();for(key,value)in values{if let Some(previous)=result.insert(key,value){<LayoutPoint as FromValue>::retire_decoded(previous)}}result}}
impl IntoIterator for RuleLayout{type Item=(String,LayoutPoint);type IntoIter=std::vec::IntoIter<Self::Item>;fn into_iter(self)->Self::IntoIter{self.members.into_iter()}}
impl<'a> IntoIterator for &'a RuleLayout{type Item=(&'a String,&'a LayoutPoint);type IntoIter=std::iter::Map<std::slice::Iter<'a,(String,LayoutPoint)>,fn(&'a(String,LayoutPoint))->Self::Item>;fn into_iter(self)->Self::IntoIter{self.members.iter().map(|(key,value)|(key,value))}}
impl semio_framework_value::retirement::RetireOwned for RuleLayout{
 fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{semio_framework_value::retirement::RetireOwned::retirement(self.members)}
 fn retirement_birth_bytes(&self)->Option<usize>{semio_framework_value::retirement::RetireOwned::retirement_birth_bytes(&self.members)}
 fn controlled_retirement_supported()->bool{<Vec<(String,LayoutPoint)> as semio_framework_value::retirement::RetireOwned>::controlled_retirement_supported()}
}
impl FromValue for RuleLayout {
 fn from_value(value:DslValue)->Result<Self,ValueError>{let DslValue::Object(fields)=value else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"rule layout object required"))};fields.into_iter().map(|(key,value)|LayoutPoint::from_value(value).map(|value|(key,value))).collect()}
 fn from_value_controlled(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{control.scoped_depth(64,|control|control.scoped_stage(|control|{let fields=value.object_controlled(control)?;control.begin_stage(0)?;let mut output=DecodedValue::new(Self::from_admitted(control.allocate_vec(fields.len())?),<Self as FromValue>::retire_decoded);for(key,value)in fields{let key=control.copy_text(key)?;let value=LayoutPoint::from_value_controlled(value,control)?;output.get_mut().admitted_insert(key,value,control)?;}control.checkpoint()?;Ok(output.take())}))}
 fn default_value_controlled(control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{control.checkpoint()?;Ok(Self::new())}
 fn retire_decoded(self){for(_,value)in self.members{<LayoutPoint as FromValue>::retire_decoded(value)}}
}
impl ToValue for RuleLayout {
 fn to_value(&self)->DslValue{DslValue::Object(self.iter().map(|(key,value)|(key.clone(),value.to_value())).collect())}
 fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{control.scoped_depth(64,|control|control.scoped_stage(|control|{control.begin_stage(0)?;let mut output=DslValue::object_encoding_controlled(self.len(),control)?;for(key,value)in self{let value=value.to_value_controlled(control)?;DslValue::push_encoding_controlled(output.get_mut(),key,value,control)?;control.step()?;}control.checkpoint()?;Ok(DslValue::Object(output.take()))}))}
}
impl semio_framework_dsl_record::BorrowedDslField for RuleLayout{const SHAPE:semio_framework_dsl_record::BorrowedShape=semio_framework_dsl_record::BorrowedShape::Map(||<LayoutPoint as semio_framework_dsl_record::BorrowedDslField>::SHAPE);}
impl semio_framework_dsl_record::DslField for RuleLayout {
 fn shape()->semio_framework_dsl_record::Shape{semio_framework_dsl_record::Shape::Map(Box::new(<LayoutPoint as semio_framework_dsl_record::DslField>::shape()))}
 fn shape_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::Shape,ValueError>{semio_framework_dsl_record::producer::boxed(<LayoutPoint as semio_framework_dsl_record::DslField>::shape_controlled(control)?,control).map(semio_framework_dsl_record::Shape::Map)}
 fn to_value(&self)->semio_framework_dsl_record::FieldValue{semio_framework_dsl_record::FieldValue::Map(self.iter().map(|(key,value)|(key.clone(),<LayoutPoint as semio_framework_dsl_record::DslField>::to_value(value))).collect())}
 fn from_value(value:&semio_framework_dsl_record::FieldValue)->Result<Self,String>{let semio_framework_dsl_record::FieldValue::Map(fields)=value else{return Err("rule layout map required".into())};fields.iter().map(|(key,value)|<LayoutPoint as semio_framework_dsl_record::DslField>::from_value(value).map(|value|(key.clone(),value))).collect()}
 fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,ValueError>{control.scoped_stage(|control|{control.begin_stage(0)?;let mut output=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(control.allocate_vec(self.len())?,|values:Vec<(String,semio_framework_dsl_record::FieldValue)>|{for(_,value)in values{semio_framework_dsl_record::native_encoding::retire_field(value)}});for(key,value)in self{let key=control.copy_text(key)?;let value=<LayoutPoint as semio_framework_dsl_record::DslField>::to_value_controlled(value,control)?;output.as_mut().push((key,value));control.step()?;}Ok(semio_framework_dsl_record::FieldValue::Map(output.take()))})}
 fn from_value_controlled(value:&semio_framework_dsl_record::FieldValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{let semio_framework_dsl_record::FieldValue::Map(fields)=value else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"rule layout map required"))};control.scoped_stage(|control|{control.begin_stage(0)?;let mut output=DecodedValue::new(Self::from_admitted(control.allocate_vec(fields.len())?),<Self as FromValue>::retire_decoded);for(key,value)in fields{let key=control.copy_text(key)?;let value=<LayoutPoint as semio_framework_dsl_record::DslField>::from_value_controlled(value,control)?;output.get_mut().admitted_insert(key,value,control)?;}Ok(output.take())})}
 fn retire_decoded(self){<Self as FromValue>::retire_decoded(self)}
}

impl replication::mutation::MapDeltaTarget<LayoutPoint> for RuleLayout { fn contains_key(&self,key:&str)->bool{self.contains_key(key)} fn set_owned(&mut self,key:String,value:LayoutPoint){self.insert(key,value);} fn remove_owned(&mut self,key:&str){self.remove(key);} }
