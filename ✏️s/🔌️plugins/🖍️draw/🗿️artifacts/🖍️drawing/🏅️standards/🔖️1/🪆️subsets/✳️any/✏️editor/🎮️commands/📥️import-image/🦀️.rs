//! 📥️ Encoded PNG import enters the retained image-admission route.
use crate::editor::drawing::commands::canvas_pointer_down::DrawingSession;
use semio_framework_plugin::{ArtifactView,ConfigView,Emit,Fault,NoConfig,NoConfigMutation,Effect,RequestId};
use crate::{DrawingSnapshot,DrawingMutation,DrawingImageAsset};
#[derive(Clone,Debug,PartialEq,semio_framework_value::ToValue,semio_framework_value::FromValue,semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword="import-image")]
pub struct ImportImage {pub payload:String,#[value(default)] pub name:Option<String>,#[value(default)] pub parent_id:Option<String>,#[value(default)] pub index:Option<usize>}
pub const MAX_SOURCE_BYTES:usize=89_478_512;
pub const MAX_PIXELS:usize=16_777_216;
pub fn request()->Emit<DrawingMutation,NoConfigMutation> {request_at(None,None)}
pub fn request_at(parent:Option<String>,index:Option<usize>)->Emit<DrawingMutation,NoConfigMutation> {use semio_framework_value::ToValue;let args=semio_framework_value::DslValue::object([("parentId".into(),parent.to_value()),("index".into(),index.to_value())]);Emit::effect(Effect::RequestFileOpen {req:RequestId(0x4452494d),accept:"image/png,.png".into(),read_as:Some("dataUrl".into()),import_action:"importImage".into(),multiple:false,args:Some(args)})}
pub fn validate(input:&ImportImage)->Result<(),Fault> {if !input.payload.starts_with("data:image/png;base64,")||input.payload.len()>MAX_SOURCE_BYTES||input.payload.len()<=22||input.name.as_ref().is_some_and(|name|name.len()>1024)||input.parent_id.as_ref().is_some_and(|parent|parent.is_empty()||parent.len()>1024) {return Err(Fault::from("Choose a PNG image within the image import limits"));}Ok(())}
pub(crate) fn publish(snapshot:&DrawingSnapshot,operation:&semio_framework_plugin::AppOperationContext,payload:&ImportImage,asset:DrawingImageAsset,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<Emit<DrawingMutation,NoConfigMutation>,Fault> {
 let app=operation.app_instance_id.to_le_bytes();let id=operation.operation_id.to_le_bytes();let generation=operation.generation.to_le_bytes();
 let asset_identity=crate::standards::v1::subsets::any::io::text::identity::publication::admit_identity(crate::schema::identity::DrawingIdentityKind::ImageAsset,&[&operation.canonical_base_revision,&app,&id,&generation],control).map_err(|error|Fault::from(error.to_string()))?;
 let asset_id=asset_identity.key().to_string_owner();if snapshot.assets.contains_key(&asset_id){return Err(Fault::from("Image import identity already exists"));}
 let identity=super::add_layer::prepare_identity(snapshot,"image",operation,control)?;
 let mut layer=crate::schema::create_drawing_image_layer(identity,payload.name.as_deref().filter(|name|!name.is_empty()).unwrap_or("Image"),&asset_id);
 if let crate::DrawingLayerNode::Image(image)=&mut layer {image.width=asset.width as f64;image.height=asset.height as f64;}
 let id=crate::schema::layer_id(&layer).to_string();
 let mut emit=Emit::mutations(vec![crate::mutations::import_image_asset(asset_id,asset),crate::mutations::create_layer(payload.parent_id.clone().map(Into::into),payload.index.or_else(||payload.parent_id.is_none().then_some(snapshot.layers.len())),layer)]);
 emit.effects.push(super::canvas_pointer_down::interaction_select_effect(&[id],"replace"));Ok(emit)
}
pub fn handle(_payload:&ImportImage,_doc:&ArtifactView<'_,DrawingSnapshot>,_cfg:&ConfigView<'_,NoConfig>,_session:&mut DrawingSession)->Result<Emit<DrawingMutation,NoConfigMutation>,Fault> {Err(Fault::from("Encoded image import requires its retained admission job"))}
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
