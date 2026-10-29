//! 🗂️ Persistent ordered maps and sets bind like their `BTreeMap` and string-list oracles, so documents print and parse identically whichever owner holds the members.
use super::*;
use semio_framework_value_ordered::ordered::{OrderedMap,OrderedSet,RetirementStep};
use semio_framework_value::retained_clone::RetainedCloneGrant;

fn retire_entries<V>(map:OrderedMap<V>,mut each:impl FnMut(V)){
 let mut retirement=map.retire();
 loop{
  let grant=RetainedCloneGrant::one_release_turn(retirement.next_close_byte_demand().expect("finite ordered cold release demand"),64);
  match retirement.advance(grant){RetirementStep::OwnedValue(value)=>each(value),RetirementStep::Complete=>break,RetirementStep::Failure(error)=>panic!("ordered field cold retirement refused: {error}"),_=>{}}
 }
}

/// 🪢️ The persistent ordered map binds exactly like `BTreeMap<String, T>`: the same `Shape::Map`, key order and duplicate-key replacement.
impl<T:BorrowedDslField> BorrowedDslField for OrderedMap<T>{const SHAPE:BorrowedShape=BorrowedShape::Map(borrowed_field_shape::<T>);}
impl<T:DslField> DslField for OrderedMap<T>{
 fn projection_view(&self,path:&[usize])->Result<native_encoding::FieldProjectionView<'_>,ValueError>{if path.is_empty(){return Ok(native_encoding::FieldProjectionView::Map(self.len()))}self.entry_at_rank(path[0]).map(|(_,value)|value).ok_or_else(native_encoding::projection_path_error)?.projection_view(&path[1..])}
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{if path.is_empty(){return self.entry_at_rank(index).map(|(key,_)|key.as_str()).ok_or_else(native_encoding::projection_path_error)}self.entry_at_rank(path[0]).map(|(_,value)|value).ok_or_else(native_encoding::projection_path_error)?.projection_key(&path[1..],index)}
 fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{native_encoding::project_entries(self.len(),self.iter(),control)}
 fn retire_decoded(self){retire_entries(self,T::retire_decoded);}
 fn shape()->Shape{Shape::Map(Box::new(T::shape()))}
 fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.scoped_depth(64,|control|Ok(Shape::Map(producer::boxed(T::shape_controlled(control)?,control)?)))}
 fn to_value(&self)->FieldValue{FieldValue::Map(self.iter().map(|(key,value)|(key.clone(),value.to_value())).collect())}
 fn from_value(value:&FieldValue)->Result<Self,String>{
  match value{
   FieldValue::Map(entries)=>{let mut out=Self::new();for(key,value) in entries{if let Some(previous)=out.insert(key.clone(),T::from_value(value)?){T::retire_decoded(previous);}}Ok(out)},
   other=>Err(format!("expected Map, found {other:?}")),
  }
 }
 fn from_value_controlled(value:&FieldValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{
  control.step()?;
  match value{
   FieldValue::Map(entries)=>{
    let slot=size_of::<(String,T)>().checked_add(128).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"map slot size overflow"))?;
    control.charge(entries.len().checked_mul(slot).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"map ownership size overflow"))?)?;
    let mut output=__rt::DecodedFieldOwner::new(Self::new(),Self::retire_decoded);
    for(key,value) in entries{if let Some(previous)=output.as_mut().insert(control.copy_text(key)?,control.scoped_stage(|control|T::from_value_controlled(value,control))?){T::retire_decoded(previous);}}
    Ok(output.take())
   },
   _=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"expected Map")),
  }
 }
}

/// 🔢️ Ordered string membership binds as a list of strings; the members are re-ordered on the way in.
impl BorrowedDslField for OrderedSet{const SHAPE:BorrowedShape=BorrowedShape::List(borrowed_field_shape::<String>);}
impl DslField for OrderedSet{
 fn projection_view(&self,path:&[usize])->Result<native_encoding::FieldProjectionView<'_>,ValueError>{if path.is_empty(){return Ok(native_encoding::FieldProjectionView::List(self.len()))}if path.len()==1{return self.iter().nth(path[0]).map(|member|native_encoding::FieldProjectionView::Text(member.as_str())).ok_or_else(native_encoding::projection_path_error)}Err(native_encoding::projection_path_error())}
 fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{
  control.step()?;let mut items=control.allocate_vec::<FieldValue>(self.len())?;
  for member in self.iter(){items.push(FieldValue::Text(control.copy_text(member)?));control.step()?;}
  Ok(FieldValue::List(items))
 }
 fn retire_decoded(self){self.retire_cold();}
 fn shape()->Shape{Shape::List(Box::new(Shape::Text))}
 fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.scoped_depth(64,|control|Ok(Shape::List(producer::boxed(Shape::Text,control)?)))}
 fn to_value(&self)->FieldValue{FieldValue::List(self.iter().map(|member|FieldValue::Text(member.clone())).collect())}
 fn from_value(value:&FieldValue)->Result<Self,String>{
  match value{
   FieldValue::List(items)=>items.iter().map(|item|match item{FieldValue::Text(text)=>Ok(text.clone()),other=>Err(format!("expected Text, found {other:?}"))}).collect(),
   other=>Err(format!("expected List, found {other:?}")),
  }
 }
 fn from_value_controlled(value:&FieldValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{
  control.step()?;
  match value{
   FieldValue::List(items)=>{
    control.charge(items.len().checked_mul(size_of::<String>()+128).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"set ownership size overflow"))?)?;
    let mut output=__rt::DecodedFieldOwner::new(OrderedSet::new(),Self::retire_decoded);
    for item in items{let FieldValue::Text(text)=item else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"expected a Text set member"))};let member=control.copy_text(text)?;output.as_mut().insert(member);control.step()?;}
    Ok(output.take())
   },
   _=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"expected List")),
  }
 }
}
