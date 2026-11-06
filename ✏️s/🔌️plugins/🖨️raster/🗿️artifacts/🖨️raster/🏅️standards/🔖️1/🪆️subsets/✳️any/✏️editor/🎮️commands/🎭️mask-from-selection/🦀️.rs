//! 🎭️ A small retained mask intent consumes the original completed selection.
use crate::RasterCommand;
use crate::commands::work_app::RasterWorkApp;
use crate::config::{RasterConfig,RasterConfigMutation};
use semio_s_artifact_raster_raster_model::{RasterLayerMask,RasterLayerNode,RasterTransform,RASTER_OWNED_MAP_CAPACITY};
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
use semio_s_artifact_raster_raster_mutations::RasterMutation;
use semio_plugin::{ArtifactView,ConfigView,Emit,Fault};
use semio_plugin::retained_command::{ArtifactCommandInputs,ArtifactCommandWork,ArtifactCommandWorkStep};
use semio_framework_value_derive::{FromValue,ToValue};

#[derive(Clone,Debug,PartialEq,ToValue,FromValue,semio_framework_dsl_record_derive::DslRecord,semio_framework_value::RetireOwned)]
#[dsl(keyword="mask-from-selection")]
#[value(rename_all="camelCase")]
pub struct MaskFromSelection {pub layer_id:String,pub expected_image_key:Option<String>}

impl MaskFromSelection {
    /// 🪟️ Business values move while the original action retains its reserved window metadata.
    pub fn from_original_action_value(args:&mut Option<semio_framework_value::DslValue>)->Result<Self,semio_framework_value::ValueError>{
        use semio_framework_value::{DslValue,ValueError,ValueRefusalKind};
        let invalid=||ValueError::literal(ValueRefusalKind::InvalidValue,"raster.mask-action-fields");let Some(DslValue::Object(entries))=args.as_ref()else{return Err(invalid());};if !(2..=3).contains(&entries.len()){return Err(invalid());}
        let(mut layer,mut image,mut window,mut bytes)=(None,None,None,0usize);
        for(index,(key,value))in entries.iter().enumerate(){match key.as_str(){
            "layerId" if layer.is_none()=>{let DslValue::String(text)=value else{return Err(invalid());};if text.is_empty()||text.len()>semio_s_artifact_raster_raster_model::RASTER_OWNED_FIELD_BYTES{return Err(invalid());}bytes=bytes.checked_add(text.len()).ok_or_else(invalid)?;layer=Some(index);},
            "expectedImageKey" if image.is_none()=>{match value{DslValue::Null=>{},DslValue::String(text)=>{if text.len()>semio_s_artifact_raster_raster_model::RASTER_OWNED_FIELD_BYTES{return Err(invalid());}bytes=bytes.checked_add(text.len()).ok_or_else(invalid)?;},_=>return Err(invalid())}image=Some(index);},
            "windowId" if window.is_none()=>{let DslValue::String(text)=value else{return Err(invalid());};if text.is_empty()||text.len()>semio_s_artifact_raster_raster_model::RASTER_OWNED_FIELD_BYTES{return Err(invalid());}window=Some(index);},
            _=>return Err(invalid()),
        }}
        let(layer,image)=(layer.ok_or_else(invalid)?,image.ok_or_else(invalid)?);if bytes>65536{return Err(invalid());}
        let Some(DslValue::Object(entries))=args.as_mut()else{unreachable!()};let DslValue::String(layer_id)=std::mem::replace(&mut entries[layer].1,DslValue::Null)else{unreachable!()};
        let expected_image_key=match std::mem::replace(&mut entries[image].1,DslValue::Null){DslValue::Null=>None,DslValue::String(text)=>Some(text),_=>unreachable!()};Ok(Self{layer_id,expected_image_key})
    }
}

pub fn handle(_payload:&MaskFromSelection,_doc:&ArtifactView<'_,RasterSnapshot>,_cfg:&ConfigView<'_,RasterConfig>)->Result<Emit<RasterMutation,RasterConfigMutation>,Fault>{Err(Fault::from("raster.mask-requires-retained-work"))}

#[path="🧵️work/🦀️.rs"]
mod work;
pub use work::{MaskFromSelectionWork,MaskSelectionWork};

