//! 📋️ Controlled identity assignment for every node of an authored subtree.
use super::publication::admit_identity;
use semio_s_artifact_draw_drawing_model::identity::DrawingIdentityKind;
use semio_framework_value::{NativeEncodeControl,ValueError};

pub fn admit_clone_identities(node:&semio_s_artifact_draw_drawing_model::DrawingLayerNode,suffix:&str,control:&mut NativeEncodeControl<'_>)->Result<semio_framework_value_persistent::list::PagedList<semio_s_artifact_draw_drawing_model::identity::DrawingIdentityAssignment,{usize::MAX}>,ValueError>{
 fn visit(node:&semio_s_artifact_draw_drawing_model::DrawingLayerNode,suffix:&str,depth:usize,count:&mut usize,control:&mut NativeEncodeControl<'_>,output:&mut Vec<semio_s_artifact_draw_drawing_model::identity::DrawingIdentityAssignment>)->Result<(),ValueError>{
  control.checkpoint()?;if depth>=64||*count>=4096{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"Drawing identity subtree limit"));}*count+=1;
  let source=semio_s_artifact_draw_drawing_model::layer_id(node);if source.is_empty()||output.iter().any(|assignment|assignment.source==*source){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"Drawing identity subtree repeats or omits a source key"));}let mut bytes=control.allocate_vec(source.len())?;for chunk in source.retained_chunks().iter(){control.step()?;bytes.extend_from_slice(chunk.as_bytes());}
  let target=admit_identity(DrawingIdentityKind::Layer,&[&bytes,suffix.as_bytes()],control)?;if output.iter().any(|assignment|assignment.target==*target.key()){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"Drawing identity subtree target repeated"));}control.charge(source.len().checked_add(size_of::<semio_s_artifact_draw_drawing_model::identity::DrawingIdentityAssignment>()).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"Drawing identity assignment ownership overflow"))?)?;
  output.push(semio_s_artifact_draw_drawing_model::identity::DrawingIdentityAssignment{source:source.clone(),target:target.into_key()});
  if let semio_s_artifact_draw_drawing_model::DrawingLayerNode::Group(group)=node{for child in &group.children{visit(child,suffix,depth+1,count,control,output)?;}}Ok(())
 }
 let mut output=Vec::new();visit(node,suffix,0,&mut 0,control,&mut output)?;if output.iter().any(|assignment|output.iter().any(|other|other.source==assignment.target)){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"Drawing identity target overlaps original subtree"));}Ok(output.into())
}
