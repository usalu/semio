//! 🔁️ Packed selection pages project the same flat ordered array through original ranked rows.
use super::{RasterSelectionSpans,RasterSelectionSpanIter,RasterSelectionSpan};
use semio_framework_value::{DslValue,ValueError,ValueRefusalKind,ValueShape,ToValue,FromValue,NativeDecodeControl,NativeEncodeControl,path_index,controlled_sequence};
use semio_framework_dsl_record::{DslField,BorrowedDslField,DslSequenceView,FieldValue,Shape,NativeSchemaControl,BorrowedShape,native_encoding};

impl ToValue for RasterSelectionSpans{
    fn to_value(&self)->DslValue{DslValue::Array(self.iter().map(ToValue::to_value).collect())}
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{controlled_sequence(self.iter(),control)}
    fn value_at_path(&self,path:&[&str])->Result<DslValue,ValueError>{let Some((segment,rest))=path.split_first()else{return Ok(ToValue::to_value(self))};self.get(path_index(segment,self.len(),false)?).unwrap().value_at_path(rest).map_err(|error|error.under(segment))}
    fn value_shape_at_path(&self,path:&[&str])->Result<ValueShape,ValueError>{let Some((segment,rest))=path.split_first()else{return Ok(ValueShape::Array{len:self.len()})};self.get(path_index(segment,self.len(),false)?).unwrap().value_shape_at_path(rest).map_err(|error|error.under(segment))}
    fn value_key_at_path(&self,path:&[&str],index:usize)->Result<String,ValueError>{let (segment,rest)=path.split_first().ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"selection array has no root object keys"))?;self.get(path_index(segment,self.len(),false)?).unwrap().value_key_at_path(rest,index).map_err(|error|error.under(segment))}
}
impl FromValue for RasterSelectionSpans{
    fn from_value(_value:DslValue)->Result<Self,ValueError>{Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"packed selection coverage requires original native receiving"))}
    fn from_value_controlled(_value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{control.checkpoint()?;Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"packed selection coverage requires original native receiving"))}
    fn default_value_controlled(control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{control.checkpoint()?;Ok(Self::default())}
}
impl DslSequenceView<RasterSelectionSpan> for RasterSelectionSpans{type Iter<'a>=RasterSelectionSpanIter<'a>;fn field_items(&self)->Self::Iter<'_>{self.iter()}}
impl BorrowedDslField for RasterSelectionSpans{const SHAPE:BorrowedShape=<Vec<RasterSelectionSpan> as BorrowedDslField>::SHAPE;}
impl DslField for RasterSelectionSpans{
    fn projection_view(&self,path:&[usize])->Result<native_encoding::FieldProjectionView<'_>,ValueError>{if path.is_empty(){return Ok(native_encoding::FieldProjectionView::List(self.len()))}self.get(path[0]).ok_or_else(native_encoding::projection_path_error)?.projection_view(&path[1..])}
    fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{let child=*path.first().ok_or_else(native_encoding::projection_path_error)?;self.get(child).ok_or_else(native_encoding::projection_path_error)?.projection_key(&path[1..],index)}
    fn shape()->Shape{<Vec<RasterSelectionSpan> as DslField>::shape()}
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{<Vec<RasterSelectionSpan> as DslField>::shape_controlled(control)}
    fn to_value(&self)->FieldValue{FieldValue::List(self.iter().map(DslField::to_value).collect())}
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{native_encoding::project_list(self,control).map(FieldValue::List)}
    fn from_value(_value:&FieldValue)->Result<Self,String>{Err("packed selection coverage requires original native receiving".into())}
    fn from_value_controlled(_value:&FieldValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{control.checkpoint()?;Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"packed selection coverage requires original native receiving"))}
}
impl semio_framework_pack_json::ArtifactCanonicalJsonTree for RasterSelectionSpans{
    fn canonical_tree_node(&self)->Result<semio_framework_pack_json::ArtifactCanonicalJsonNode<'_>,ValueError>{Ok(semio_framework_pack_json::ArtifactCanonicalJsonNode::Array(self.len()))}
    fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn semio_framework_pack_json::ArtifactCanonicalJsonTree,ValueError>{self.get(ordinal).map(|row|row as&dyn semio_framework_pack_json::ArtifactCanonicalJsonTree).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"selection array ordinal is absent"))}
}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
