//! 🗂️ Expanded preview paths retain sorted unique literal strings in concrete slots.
use semio_framework_value::{DslValue,FromValue,NativeDecodeControl,NativeEncodeControl,ToValue,ValueError,ValueRefusalKind};
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DagExpandedPaths{values:Vec<String>}
impl DagExpandedPaths{
 pub fn new()->Self{Self::default()}
 pub fn len(&self)->usize{self.values.len()}
 pub fn is_empty(&self)->bool{self.values.is_empty()}
 pub fn iter(&self)->std::slice::Iter<'_,String>{self.values.iter()}
 pub fn contains(&self,key:&str)->bool{self.values.binary_search_by(|name|name.as_str().cmp(key)).is_ok()}
 pub fn insert(&mut self,key:String)->bool{match self.values.binary_search(&key){Ok(_)=>false,Err(index)=>{self.values.insert(index,key);true}}}
 pub fn remove(&mut self,key:&str)->bool{if let Ok(index)=self.values.binary_search_by(|name|name.as_str().cmp(key)){self.values.remove(index);true}else{false}}
 pub fn pop_last(&mut self)->Option<String>{self.values.pop()}
 pub fn from_admitted(values:Vec<String>)->Self{Self{values}}
 pub fn insert_admitted(&mut self,key:String,control:&mut NativeDecodeControl<'_>)->Result<bool,ValueError>{self.insert_controlled(key,&mut||control.step())}
 pub fn insert_controlled(&mut self,key:String,checkpoint:&mut dyn FnMut()->Result<(),ValueError>)->Result<bool,ValueError>{
 let mut index=0;while index<self.values.len(){checkpoint()?;let mut order=std::cmp::Ordering::Equal;for(position,(left,right))in self.values[index].bytes().zip(key.bytes()).enumerate(){if position%256==0{checkpoint()?;}if left!=right{order=left.cmp(&right);break;}}if order.is_eq(){order=self.values[index].len().cmp(&key.len());}match order{std::cmp::Ordering::Less=>index+=1,std::cmp::Ordering::Equal=>return Ok(false),std::cmp::Ordering::Greater=>break}}
 if self.values.len()==self.values.capacity(){return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"expanded paths exceed admitted slots"))}self.values.push(key);let mut position=self.values.len()-1;while position>index{self.values.swap(position,position-1);position-=1;checkpoint()?;}Ok(true)
 }
}
impl<const N:usize> From<[String;N]> for DagExpandedPaths{fn from(values:[String;N])->Self{values.into_iter().collect()}}
impl FromIterator<String> for DagExpandedPaths{fn from_iter<T:IntoIterator<Item=String>>(values:T)->Self{let mut result=Self::new();for value in values{result.insert(value);}result}}
impl IntoIterator for DagExpandedPaths{type Item=String;type IntoIter=std::vec::IntoIter<String>;fn into_iter(self)->Self::IntoIter{self.values.into_iter()}}
impl<'a> IntoIterator for &'a DagExpandedPaths{type Item=&'a String;type IntoIter=std::slice::Iter<'a,String>;fn into_iter(self)->Self::IntoIter{self.values.iter()}}
impl semio_framework_value::retirement::RetireOwned for DagExpandedPaths{fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{semio_framework_value::retirement::RetireOwned::retirement(self.values)}}
impl ToValue for DagExpandedPaths{
 fn to_value(&self)->DslValue{self.values.to_value()}
 fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{self.values.to_value_controlled(control)}
}
impl FromValue for DagExpandedPaths{
 fn from_value(value:DslValue)->Result<Self,ValueError>{Vec::<String>::from_value(value).map(|values|values.into_iter().collect())}
 fn from_value_controlled(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{let DslValue::Array(values)=value else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"expanded paths require text array"))};control.scoped_stage(|control|{control.begin_stage(0)?;let mut output=Self::from_admitted(control.allocate_vec(values.len())?);for value in values{let value=String::from_value_controlled(value,control)?;output.insert_admitted(value,control)?;}control.checkpoint()?;Ok(output)})}
 fn default_value_controlled(control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{control.checkpoint()?;Ok(Self::new())}
 fn retire_decoded(self){drop(self)}
}
impl semio_framework_dsl_record::DslField for DagExpandedPaths{
 fn shape()->semio_framework_dsl_record::Shape{<Vec<String> as semio_framework_dsl_record::DslField>::shape()}
 fn shape_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::Shape,ValueError>{<Vec<String> as semio_framework_dsl_record::DslField>::shape_controlled(control)}
 fn to_value(&self)->semio_framework_dsl_record::FieldValue{<Vec<String> as semio_framework_dsl_record::DslField>::to_value(&self.values)}
 fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,ValueError>{<Vec<String> as semio_framework_dsl_record::DslField>::to_value_controlled(&self.values,control)}
 fn from_value(value:&semio_framework_dsl_record::FieldValue)->Result<Self,String>{<Vec<String> as semio_framework_dsl_record::DslField>::from_value(value).map(|values|values.into_iter().collect())}
 fn from_value_controlled(value:&semio_framework_dsl_record::FieldValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{let semio_framework_dsl_record::FieldValue::List(values)=value else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"expanded paths require text list"))};control.scoped_stage(|control|{control.begin_stage(0)?;let mut output=Self::from_admitted(control.allocate_vec(values.len())?);for value in values{let value=<String as semio_framework_dsl_record::DslField>::from_value_controlled(value,control)?;output.insert_admitted(value,control)?;}control.checkpoint()?;Ok(output)})}
 fn retire_decoded(self){drop(self)}
}
