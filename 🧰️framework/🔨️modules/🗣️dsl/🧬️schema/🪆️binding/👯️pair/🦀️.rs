//! 👯️ A homogeneous pair binds as the fixed two-item `Shape::Tuple`, exactly like a two-element array.
use crate::*;
use crate::native_encoding::{FieldProjectionView as View,projection_path_error};

impl<T> DslSequenceView<T> for (T,T){
 type Iter<'a>=std::array::IntoIter<&'a T,2> where T:'a;
 fn field_items(&self)->Self::Iter<'_>{[&self.0,&self.1].into_iter()}
}

impl<T:BorrowedDslField> BorrowedDslField for (T,T){const SHAPE:BorrowedShape=BorrowedShape::Tuple(borrowed_field_shape::<T>,Some(2));}

/// 🔧️ Fixed-arity `Shape::Tuple(_, Some(2))` for a pair whose items share one field type.
impl<T:DslField> DslField for (T,T){
 fn projection_view(&self,path:&[usize])->Result<View<'_>,ValueError>{match path.split_first(){None=>Ok(View::Tuple(2)),Some((0,rest))=>self.0.projection_view(rest),Some((1,rest))=>self.1.projection_view(rest),_=>Err(projection_path_error())}}
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{match path.split_first(){Some((0,rest))=>self.0.projection_key(rest,index),Some((1,rest))=>self.1.projection_key(rest,index),_=>Err(projection_path_error())}}
 fn retire_decoded(self){T::retire_decoded(self.0);T::retire_decoded(self.1);}
 fn shape()->Shape{Shape::Tuple(Box::new(T::shape()),Some(2))}
 fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.scoped_depth(64,|control|Ok(Shape::Tuple(producer::boxed(T::shape_controlled(control)?,control)?,Some(2))))}
 fn to_value(&self)->FieldValue{FieldValue::Tuple(vec![self.0.to_value(),self.1.to_value()])}
 fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{native_encoding::project_list(self,control).map(FieldValue::Tuple)}
 fn from_value(value:&FieldValue)->Result<Self,String>{match value{FieldValue::Tuple(items) if items.len()==2=>Ok((T::from_value(&items[0])?,T::from_value(&items[1])?)),other=>Err(format!("expected a 2-item Tuple, found {other:?}"))}}
 fn from_value_controlled(value:&FieldValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{
  control.step()?;
  match value{
   FieldValue::Tuple(items) if items.len()==2=>{
    let first=control.scoped_stage(|control|T::from_value_controlled(&items[0],control))?;
    match control.scoped_stage(|control|T::from_value_controlled(&items[1],control)){Ok(second)=>Ok((first,second)),Err(error)=>{T::retire_decoded(first);Err(error)}}
   },
   _=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"expected a 2-item Tuple")),
  }
 }
}
