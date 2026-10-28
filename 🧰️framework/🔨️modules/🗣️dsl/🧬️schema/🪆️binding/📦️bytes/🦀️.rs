//! 📦️ Paged octet fields keep their original borrowed projection and canonical Bytes64 shape.
use super::*;
use semio_framework_value::paged::PagedBytes;
impl<const N:usize> DslField for PagedBytes<N>{
 fn shape()->Shape{Shape::Bytes64}
 fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.checkpoint()?;Ok(Shape::Bytes64)}
 fn projection_view(&self,path:&[usize])->Result<native_encoding::FieldProjectionView<'_>,ValueError>{if !path.is_empty(){return Err(native_encoding::projection_path_error())}Ok(native_encoding::FieldProjectionView::BytesSource(self))}
 fn to_value(&self)->FieldValue{FieldValue::Bytes64(PagedBytes::from_retained_bytes(semio_framework_value_persistent::list::PagedList::try_from_iter(self.iter()).expect("addressable original octets fit canonical Bytes64")))}
 fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{Ok(FieldValue::Bytes64(PagedBytes::copy_encoded(self,control)?))}
 fn from_value(value:&FieldValue)->Result<Self,String>{let FieldValue::Bytes64(bytes)=value else{return Err("expected Bytes64".into())};Ok(Self::from_retained_bytes(semio_framework_value_persistent::list::PagedList::try_from_iter(bytes.iter()).map_err(|error|error.to_string())?))}
 fn from_value_controlled(value:&FieldValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{control.checkpoint()?;let FieldValue::Bytes64(bytes)=value else{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"expected Bytes64"))};if bytes.len()>N{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"Bytes64 exceeds its declared paged capacity"))}let mut output=Self::default();for byte in bytes.iter(){output.push_decoded(byte,control)?;control.step()?;}Ok(output)}
}
