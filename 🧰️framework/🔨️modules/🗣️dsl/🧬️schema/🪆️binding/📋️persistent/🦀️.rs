//! 📋️ Persistent semantic rows retain ordered array shape and borrow their original ranked fields.
use super::*;
use semio_framework_value::persistent::{PersistentList,PersistentListIter};
use semio_framework_value::retirement::RetireOwned;

impl<T:RetireOwned+Send+Sync+'static> DslSequenceView<T> for PersistentList<T>{
    type Iter<'a>=PersistentListIter<'a,T> where T:'a;
    fn field_items(&self)->Self::Iter<'_>{self.iter()}
}
impl<T:BorrowedDslField+RetireOwned+Send+Sync+'static> BorrowedDslField for PersistentList<T>{const SHAPE:BorrowedShape=BorrowedShape::List(borrowed_field_shape::<T>);}
impl<T:DslField+RetireOwned+Send+Sync+'static> DslField for PersistentList<T>{
    fn projection_view(&self,path:&[usize])->Result<native_encoding::FieldProjectionView<'_>,ValueError>{if path.is_empty(){return Ok(native_encoding::FieldProjectionView::List(self.len()))}self.get(path[0]).ok_or_else(native_encoding::projection_path_error)?.projection_view(&path[1..])}
    fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{let child=*path.first().ok_or_else(native_encoding::projection_path_error)?;self.get(child).ok_or_else(native_encoding::projection_path_error)?.projection_key(&path[1..],index)}
    fn shape()->Shape{Shape::List(Box::new(T::shape()))}
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.scoped_depth(64,|control|Ok(Shape::List(producer::boxed(T::shape_controlled(control)?,control)?)))}
    fn to_value(&self)->FieldValue{FieldValue::List(self.iter().map(DslField::to_value).collect())}
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{native_encoding::project_list(self,control).map(FieldValue::List)}
    fn from_value(_value:&FieldValue)->Result<Self,String>{Err("persistent fields require original native receiving".into())}
    fn from_value_controlled(_value:&FieldValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{control.checkpoint()?;Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"persistent fields require original native receiving"))}
}
