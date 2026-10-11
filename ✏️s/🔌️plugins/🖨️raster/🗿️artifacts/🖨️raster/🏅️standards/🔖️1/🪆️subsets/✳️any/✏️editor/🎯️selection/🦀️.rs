//! 🎯️ Completed selection coverage shared by paint hosts and authoritative commands.
use semio_framework_plugin::{Fault,FaultCode,FaultOrigin};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, schema::ArtifactSchema, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all="camelCase")]
#[dsl(keyword="pixel-selection")]
#[artifact_schema(id="s.raster.raster.pixelselection")]
pub struct RasterPixelSelection {
    #[state(config)]
    pub layer_id:String,
    #[state(config)]
    pub target:String,
    #[state(config)]
    pub width:u32,
    #[state(config)]
    pub height:u32,
    #[state(config)]
    pub spans:Vec<crate::mutations::paint_stroke::RasterSelectionSpan>,
}

impl RasterPixelSelection {
    pub fn validate(&self)->Result<usize,Fault>{
        if self.layer_id.is_empty()||self.layer_id.chars().count()>256||!matches!(self.target.as_str(),"pixels"|"mask"){return Err(fault("Invalid selection target"));}
        let count=semio_framework_pixels::editing::validate_extent(self.width,self.height).map_err(|_|fault("Invalid selection extent"))?;
        selection_spans(&self.spans,count)?;
        Ok(count)
    }
    pub fn validate_current(&self,document:&crate::RasterSnapshot,expected_image_key:Option<&str>)->Result<(),Fault>{
        use crate::RasterLayerNode;
        self.validate()?;
        let layer=visible_layer(&document.layers,&self.layer_id).ok_or_else(||fault("Selection layer is missing or hidden"))?;
        let (image_key,width,height)=match (self.target.as_str(),layer){
            ("pixels",RasterLayerNode::Pixel{image_key,width,height,..})=>(image_key,width.unwrap_or(512),height.unwrap_or(512)),
            ("mask",RasterLayerNode::Pixel{mask:Some(mask),width,height,..})=>(&mask.image_key,mask.width.unwrap_or(width.unwrap_or(512)),mask.height.unwrap_or(height.unwrap_or(512))),
            ("mask",RasterLayerNode::Group{mask:Some(mask),..})=>(&mask.image_key,mask.width.unwrap_or(512),mask.height.unwrap_or(512)),
            _=>return Err(fault("Selection target is unavailable")),
        };
        if image_key.as_deref()!=expected_image_key{return Err(fault("Selection image changed"));}
        let (width,height)=if let Some(key)=image_key{
            let image=document.assets.get(key).and_then(|asset|asset.local_owner::<crate::SemioImageSnapshot>()).ok_or_else(||fault("Selection image is unavailable"))?;
            (image.width,image.height)
        }else{(width,height)};
        if (self.width,self.height)!=(width,height){return Err(fault("Selection dimensions changed"));}
        Ok(())
    }
    pub fn spans(&self)->Result<Vec<(usize,usize,u8)>,Fault>{
        let count=semio_framework_pixels::editing::validate_extent(self.width,self.height).map_err(|_|fault("Invalid selection extent"))?;
        selection_spans(&self.spans,count)
    }
}

fn fault(message:impl Into<String>)->Fault{Fault::new(FaultOrigin::App,FaultCode::new("raster.pixel-selection"),message.into())}

pub fn selection_spans(spans:&[crate::mutations::paint_stroke::RasterSelectionSpan],count:usize)->Result<Vec<(usize,usize,u8)>,Fault> {
    let mut result=Vec::with_capacity(spans.len());
    let mut previous=0;
    for span in spans {
        let start=span.start as usize;
        let end=start.checked_add(span.length as usize).ok_or_else(||fault("Selection overflow"))?;
        if start<previous||span.length==0||end>count||span.coverage>255{return Err(fault("Selection spans overlap or exceed image"));}
        result.push((start,end,span.coverage as u8));
        previous=end;
    }
    Ok(result)
}

fn visible_layer<'a>(layers:&'a [crate::RasterLayerNode],id:&str)->Option<&'a crate::RasterLayerNode>{
    use crate::standards::v1::subsets::any::schema::{layer_node_id,layer_visible};
    for layer in layers{
        if !layer_visible(layer){continue;}
        if layer_node_id(layer)==id{return Some(layer);}
        if let crate::RasterLayerNode::Group{children,..}=layer{if let Some(found)=visible_layer(children,id){return Some(found);}}
    }
    None
}

#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
