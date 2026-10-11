//! 🗂️ Unique literal OPC relationship owners retain directly admitted contiguous groups.
use super::OpcRelationship;
use semio_framework_value::{DslValue,FromValue,ToValue,NativeDecodeControl,NativeEncodeControl,ValueEdit,ValueShape,ValueError,ValueRefusalKind};
#[derive(Clone,Debug,Default,PartialEq,semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
pub struct OpcRelationshipOwners{groups:Vec<(String,Vec<OpcRelationship>)>}
trait OrderingControl{
 fn compare(&mut self,left:&str,right:&str)->Result<std::cmp::Ordering,ValueError>;
 fn step(&mut self)->Result<(),ValueError>;
 fn checkpoint(&mut self)->Result<(),ValueError>;
}
fn compare_keys(left:&str,right:&str,mut advance:impl FnMut(usize)->Result<(),ValueError>)->Result<std::cmp::Ordering,ValueError>{
 let count=left.len().min(right.len());let mut position=0usize;
 while position<count{let end=position.saturating_add(256).min(count);let order=left.as_bytes()[position..end].cmp(&right.as_bytes()[position..end]);advance(end-position)?;if !order.is_eq(){return Ok(order)}position=end;}
 Ok(left.len().cmp(&right.len()))
}
impl OrderingControl for NativeDecodeControl<'_>{
 fn compare(&mut self,left:&str,right:&str)->Result<std::cmp::Ordering,ValueError>{self.scoped_stage(|control|{control.begin_stage(left.len().min(right.len()))?;compare_keys(left,right,|count|control.advance(count))})}
 fn step(&mut self)->Result<(),ValueError>{NativeDecodeControl::step(self)}
 fn checkpoint(&mut self)->Result<(),ValueError>{NativeDecodeControl::checkpoint(self)}
}
impl OrderingControl for NativeEncodeControl<'_>{
 fn compare(&mut self,left:&str,right:&str)->Result<std::cmp::Ordering,ValueError>{self.scoped_stage(|control|{control.begin_stage(left.len().min(right.len()))?;compare_keys(left,right,|count|control.advance(count))})}
 fn step(&mut self)->Result<(),ValueError>{NativeEncodeControl::step(self)}
 fn checkpoint(&mut self)->Result<(),ValueError>{NativeEncodeControl::checkpoint(self)}
}
fn adopt<C:OrderingControl>(mut groups:Vec<(String,Vec<OpcRelationship>)>,control:&mut C)->Result<OpcRelationshipOwners,ValueError>{
 fn sift<C:OrderingControl>(groups:&mut[(String,Vec<OpcRelationship>)],mut root:usize,end:usize,control:&mut C)->Result<(),ValueError>{loop{let Some(left)=root.checked_mul(2).and_then(|value|value.checked_add(1)).filter(|value|*value<end)else{return Ok(())};let right=left+1;let next=if right<end&&control.compare(&groups[left].0,&groups[right].0)?.is_lt(){right}else{left};control.step()?;if !control.compare(&groups[root].0,&groups[next].0)?.is_lt(){return Ok(())}groups.swap(root,next);root=next;}}
 for root in(0..groups.len()/2).rev(){let end=groups.len();sift(&mut groups,root,end,control)?;}
 for end in(1..groups.len()).rev(){groups.swap(0,end);sift(&mut groups,0,end,control)?;}
 for pair in groups.windows(2){control.step()?;if control.compare(&pair[0].0,&pair[1].0)?.is_eq(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"OPC relationship owner is duplicated"));}}
 control.checkpoint()?;Ok(OpcRelationshipOwners{groups})
}
impl OpcRelationshipOwners{
 pub fn new()->Self{Self::default()}
 pub fn owner_count(&self)->usize{self.groups.len()}
 pub fn groups(&self)->impl DoubleEndedIterator<Item=(&String,&Vec<OpcRelationship>)>+ExactSizeIterator{self.groups.iter().map(|(owner,relationships)|(owner,relationships))}
 pub fn groups_mut(&mut self)->impl DoubleEndedIterator<Item=(&str,&mut Vec<OpcRelationship>)>+ExactSizeIterator{self.groups.iter_mut().map(|(owner,relationships)|(owner.as_str(),relationships))}
 pub fn relationships(&self,owner:&str)->Option<&Vec<OpcRelationship>>{self.groups.binary_search_by(|(path,_)|path.as_str().cmp(owner)).ok().map(|index|&self.groups[index].1)}
 pub fn relationships_mut(&mut self,owner:&str)->Option<&mut Vec<OpcRelationship>>{self.groups.binary_search_by(|(path,_)|path.as_str().cmp(owner)).ok().map(|index|&mut self.groups[index].1)}
 pub fn replace_owner(&mut self,owner:String,relationships:Vec<OpcRelationship>)->Option<Vec<OpcRelationship>>{match self.groups.binary_search_by(|(path,_)|path.cmp(&owner)){Ok(index)=>Some(std::mem::replace(&mut self.groups[index].1,relationships)),Err(index)=>{self.groups.insert(index,(owner,relationships));None}}}
 pub fn remove_owner(&mut self,owner:&str)->Option<Vec<OpcRelationship>>{self.groups.binary_search_by(|(path,_)|path.as_str().cmp(owner)).ok().map(|index|self.groups.remove(index).1)}
 pub fn into_groups(self)->std::vec::IntoIter<(String,Vec<OpcRelationship>)>{self.groups.into_iter()}
 /// 🎟️ Adopts concrete paid groups under the same cumulative decode controller.
 pub fn adopt_admitted(groups:Vec<(String,Vec<OpcRelationship>)>,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{control.scoped_stage(|control|{control.begin_stage(0)?;adopt(groups,control)})}
 /// 🎟️ Adopts concrete paid groups under the same cumulative encode controller.
 pub fn adopt_encoded_admitted(groups:Vec<(String,Vec<OpcRelationship>)>,control:&mut NativeEncodeControl<'_>)->Result<Self,ValueError>{control.scoped_stage(|control|{control.begin_stage(0)?;adopt(groups,control)})}
}
impl FromValue for OpcRelationshipOwners{
 fn from_value(value:DslValue)->Result<Self,ValueError>{let DslValue::Object(fields)=value else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"OPC relationship owners require an object"))};let mut output=Self::new();for(owner,value)in fields{let relationships=Vec::<OpcRelationship>::from_value(value)?;if output.replace_owner(owner,relationships).is_some(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"OPC relationship owner is duplicated"));}}Ok(output)}
 fn from_value_controlled(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{control.scoped_stage(|control|{let fields=value.object_controlled(control)?;control.begin_stage(0)?;let mut groups=control.allocate_vec(fields.len())?;for(owner,value)in fields{groups.push((control.copy_text(owner)?,Vec::<OpcRelationship>::from_value_controlled(value,control)?));control.step()?;}Self::adopt_admitted(groups,control)})}
 fn default_value_controlled(control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{control.checkpoint()?;Ok(Self::new())}
 fn retire_decoded(self){for(_, relationships)in self.into_groups(){Vec::<OpcRelationship>::retire_decoded(relationships)}}
 fn edit_value_at_path(&mut self,path:&[&str],edit:ValueEdit)->Result<(),ValueError>{
  let Some((owner,rest))=path.split_first()else{return match edit{ValueEdit::Set(value)=>{*self=Self::from_value(value)?;Ok(())},_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"OPC relationship owner root cannot be inserted or removed"))}};
  if !rest.is_empty(){return self.relationships_mut(owner).ok_or_else(||missing_owner(owner))?.edit_value_at_path(rest,edit).map_err(|error|error.under(owner));}
  match edit{
   ValueEdit::Set(value)=>{let replacement=Vec::<OpcRelationship>::from_value(value).map_err(|error|error.under(owner))?;let target=self.relationships_mut(owner).ok_or_else(||missing_owner(owner))?;*target=replacement;},
   ValueEdit::Insert(value)|ValueEdit::InsertAt{value,..}=>{if self.relationships(owner).is_some(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"OPC relationship owner already exists").under(owner));}let values=Vec::<OpcRelationship>::from_value(value).map_err(|error|error.under(owner))?;self.replace_owner((*owner).to_owned(),values);},
   ValueEdit::Remove=>{self.remove_owner(owner).ok_or_else(||missing_owner(owner))?;}
  }Ok(())
 }
}
fn missing_owner(owner:&str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,"OPC relationship owner does not exist").under(owner)}
impl ToValue for OpcRelationshipOwners{
 fn to_value(&self)->DslValue{DslValue::Object(self.groups().map(|(owner,relationships)|(owner.clone(),relationships.to_value())).collect())}
 fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{control.scoped_stage(|control|{control.begin_stage(0)?;let mut output=DslValue::object_encoding_controlled(self.owner_count(),control)?;for(owner,relationships)in self.groups(){let value=relationships.to_value_controlled(control)?;DslValue::push_encoding_controlled(output.get_mut(),owner,value,control)?;control.step()?;}control.checkpoint()?;Ok(DslValue::Object(output.take()))})}
 fn value_at_path(&self,path:&[&str])->Result<DslValue,ValueError>{let Some((owner,rest))=path.split_first()else{return Ok(self.to_value())};self.relationships(owner).ok_or_else(||missing_owner(owner))?.value_at_path(rest).map_err(|error|error.under(owner))}
 fn value_shape_at_path(&self,path:&[&str])->Result<ValueShape,ValueError>{let Some((owner,rest))=path.split_first()else{return Ok(ValueShape::Object{len:self.owner_count()})};self.relationships(owner).ok_or_else(||missing_owner(owner))?.value_shape_at_path(rest).map_err(|error|error.under(owner))}
 fn value_key_at_path(&self,path:&[&str],index:usize)->Result<String,ValueError>{let Some((owner,rest))=path.split_first()else{return self.groups.get(index).map(|(owner,_)|owner.clone()).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"OPC relationship owner key index is out of range"))};self.relationships(owner).ok_or_else(||missing_owner(owner))?.value_key_at_path(rest,index).map_err(|error|error.under(owner))}
}
