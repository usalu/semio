//! ➕️ Physical identity preparation from retained creation facts.
use super::publication::admit_identity;
use crate::schema::identity::{DrawingIdentity,DrawingIdentityKind};
use semio_framework_value::{NativeEncodeControl,ValueError};
fn text(source:&semio_framework_value::paged::PagedUtf8<{usize::MAX}>,control:&mut NativeEncodeControl<'_>)->Result<Vec<u8>,ValueError>{let mut bytes=control.allocate_vec(source.len())?;for chunk in source.retained_chunks().iter(){control.step()?;bytes.extend_from_slice(chunk.as_bytes());}Ok(bytes)}
pub fn layer_identity(document:&crate::DrawingSnapshot,kind:&str,operation:&semio_framework_plugin::AppOperationContext,control:&mut NativeEncodeControl<'_>)->Result<DrawingIdentity,ValueError>{
 validate_operation(operation)?;let document_bytes=text(&document.id,control)?;let mut ordinal=document.layers.len()as u64;
 loop{control.checkpoint()?;let index=ordinal.to_le_bytes();let operation_id=operation.operation_id.to_le_bytes();let app=operation.app_instance_id.to_le_bytes();let generation=operation.generation.to_le_bytes();let revision=operation.canonical_base_revision.as_slice();
  let identity=admit_identity(DrawingIdentityKind::Layer,&[&document_bytes,kind.as_bytes(),&app,&operation_id,&generation,revision,operation.parent_document_id.as_bytes(),operation.authoring_seed.as_bytes(),&index],control)?;
  if crate::schema::find_drawing_layer(document,identity.key()).is_none(){return Ok(identity);}ordinal=ordinal.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"Drawing identity ordinal overflow"))?;
 }
}
pub fn shape_identity(utility:&str,geometry:[f64;4],ordinal:usize,operation:&semio_framework_plugin::AppOperationContext,control:&mut NativeEncodeControl<'_>)->Result<DrawingIdentity,ValueError>{
 validate_operation(operation)?;let mut shape=[0u8;32];for(index,value)in geometry.iter().enumerate(){shape[index*8..index*8+8].copy_from_slice(&value.to_bits().to_le_bytes());}let ordinal=(ordinal as u64).to_le_bytes();let app=operation.app_instance_id.to_le_bytes();let id=operation.operation_id.to_le_bytes();let generation=operation.generation.to_le_bytes();let document=operation.parent_document_id.as_bytes();let revision=operation.canonical_base_revision.as_slice();
 admit_identity(DrawingIdentityKind::Shape,&[utility.as_bytes(),&shape,&ordinal,&app,&id,&generation,document,revision,operation.authoring_seed.as_bytes()],control)
}

pub fn validate_operation(operation:&semio_framework_plugin::AppOperationContext)->Result<(),ValueError>{if operation.operation_id==0||operation.parent_document_id.is_empty()||operation.authoring_seed.is_empty(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"Drawing identity requires admitted original operation facts"));}Ok(())}
