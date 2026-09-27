//! 🧩️ Retained document composites shared by browser and native raster surfaces.
use super::*;
use semio_framework_pixels::{compositing::{layers::{RasterStackContent,RasterStackInput,RasterStackJob,RasterStackLayer,RasterStackMask,RasterStackResult,RasterStackTransform},CompositeBlend},editing::PixelProgress,RasterImage as PixelImage};
use std::collections::BTreeMap;

fn transform(value:&TransformJson)->RasterStackTransform {RasterStackTransform {x:value.x,y:value.y,scale_x:value.scale_x,scale_y:value.scale_y,rotation:value.rotation}}
fn mask(value:&Option<MaskJson>)->Option<RasterStackMask>{value.as_ref().map(|m|RasterStackMask {enabled:m.enabled,linked:m.linked,invert:m.invert,width:m.width,height:m.height,image_key:m.image_key.clone(),transform:transform(&m.transform)})}
pub(super) fn layers(source:&[LayerNodeJson])->Result<Vec<RasterStackLayer>,String>{
    fn visit(source:&[LayerNodeJson],depth:usize,nodes:&mut usize)->Result<Vec<RasterStackLayer>,String>{
        if depth>32{return Err("Layer nesting exceeds compositor budget".into());}
        source.iter().map(|layer|{
            *nodes+=1;if *nodes>1024{return Err("Layer count exceeds compositor budget".into());}
            let (id,visible,opacity,blend,placement,mask,content)=match layer{
                LayerNodeJson::Pixel {id,visible,opacity,blend_mode,transform,mask:layer_mask,width,height,image_key}=>
                    (id,*visible,*opacity,blend_mode,transform,mask(layer_mask),RasterStackContent::Pixel {width:*width,height:*height,image_key:image_key.clone()}),
                LayerNodeJson::Group {id,visible,opacity,blend_mode,transform,mask:layer_mask,children}=>
                    (id,*visible,*opacity,blend_mode,transform,mask(layer_mask),RasterStackContent::Group(visit(children,depth+1,nodes)?)),
                LayerNodeJson::Adjustment {id,visible,opacity,blend_mode,transform,adjustment_kind,params}=>{
                    if adjustment_kind!="brightnessContrast"{return Err(format!("Unsupported adjustment {adjustment_kind}"));}
                    (id,*visible,*opacity,blend_mode,transform,None,RasterStackContent::BrightnessContrast {brightness:f64::from(params.brightness.unwrap_or(0.0)),contrast:f64::from(params.contrast.unwrap_or(0.0))})
                }
            };
            Ok(RasterStackLayer {id:id.clone(),visible,opacity:f64::from(opacity),blend_mode:if blend.is_empty(){CompositeBlend::Normal}else{blend.parse().map_err(|e:semio_framework_pixels::editing::PixelEditError|e.to_string())?},transform:transform(placement),mask,content})
        }).collect()
    }
    visit(source,0,&mut 0)
}

#[derive(Clone,PartialEq,Eq,Default)]
pub(super) enum View {#[default] Composite,Layer(String),Mask(String)}

#[derive(Default)]
pub(super) struct RetainedComposite {
    pub output:Option<RasterStackResult>,
    pub image:Option<Arc<RasterImage>>,
    pub pending:bool,
    pub error:Option<String>,
    pub view:View,
    job:Option<RasterStackJob>,
    progress:Option<PixelProgress>,
}
impl RetainedComposite {
    pub fn invalidate(&mut self){self.job=None;self.pending=true;self.error=None;self.progress=None;}
    pub fn select(&mut self,view:View){if self.view!=view{self.view=view;self.output=None;self.image=None;self.invalidate();}}
    pub fn cancel(&mut self){self.job=None;self.pending=false;self.error=None;self.progress=None;}
    pub fn progress_json(&self)->String{serde_json::json!({"pending":self.pending,"completed":self.progress.as_ref().map_or(0,|p|p.completed),"total":self.progress.as_ref().map_or(0,|p|p.total),"error":self.error}).to_string()}
    fn input(&self,source:&[RasterStackLayer],buffers:&HashMap<String,Arc<PixelImage>>)->Result<RasterStackInput,String>{
        fn isolated(layers:&[RasterStackLayer],id:&str)->Option<RasterStackLayer>{
            for layer in layers{
                if layer.id==id{let mut result=layer.clone();result.visible=true;return Some(result);}
                if let RasterStackContent::Group(children)=&layer.content{if let Some(child)=isolated(children,id){let mut result=layer.clone();result.visible=true;result.opacity=1.0;result.blend_mode=CompositeBlend::Normal;result.mask=None;result.content=RasterStackContent::Group(vec![child]);return Some(result);}}
            }
            None
        }
        let mut layers=match &self.view{View::Composite=>source.to_vec(),View::Layer(id)|View::Mask(id)=>isolated(source,id).into_iter().collect()};
        let mut images=BTreeMap::new();
        if let View::Mask(id)=&self.view{
            fn contains_id(layers:&[RasterStackLayer],id:&str)->bool{layers.iter().any(|layer|layer.id==id||matches!(&layer.content,RasterStackContent::Group(children) if contains_id(children,id)))}
            let mut white_key="surface-mask-white".to_owned();while buffers.contains_key(&white_key)||contains_id(source,&white_key){white_key.push('_');}
            fn replace(layers:&mut Vec<RasterStackLayer>,id:&str,key:&str,buffers:&HashMap<String,Arc<PixelImage>>){
                for layer in layers.iter_mut(){
                    if layer.id==id{
                        let Some(mask)=layer.mask.take() else{layers.clear();return};
                        let image=mask.image_key.as_ref().and_then(|key|buffers.get(key));
                        let width=mask.width.or_else(||image.map(|i|i.width)).unwrap_or(512);let height=mask.height.or_else(||image.map(|i|i.height)).unwrap_or(512);
                        let identity=RasterStackTransform {x:0.0,y:0.0,scale_x:1.0,scale_y:1.0,rotation:0.0};
                        let child=RasterStackLayer {id:key.into(),visible:true,opacity:1.0,blend_mode:CompositeBlend::Normal,transform:mask.transform,mask:Some(RasterStackMask {enabled:true,linked:true,invert:mask.invert,width:Some(width),height:Some(height),image_key:mask.image_key,transform:identity}),content:RasterStackContent::Pixel {width:Some(width),height:Some(height),image_key:Some(key.into())}};
                        if !mask.linked{layer.transform=identity;}layer.opacity=1.0;layer.blend_mode=CompositeBlend::Normal;layer.content=RasterStackContent::Group(vec![child]);return;
                    }
                    if let RasterStackContent::Group(children)=&mut layer.content{replace(children,id,key,buffers);}
                }
            }
            replace(&mut layers,id,&white_key,buffers);
            images.insert(white_key,Arc::new(PixelImage {width:1,height:1,pixels:vec![255;4]}));
        }
        fn collect(layers:&mut [RasterStackLayer],buffers:&HashMap<String,Arc<PixelImage>>,images:&mut BTreeMap<String,Arc<PixelImage>>){
            for layer in layers{
                if let Some(mask)=&layer.mask{if mask.enabled{if let Some(key)=&mask.image_key{if let Some(image)=buffers.get(key){images.insert(key.clone(),Arc::clone(image));}}}}
                match &mut layer.content{
                    RasterStackContent::Pixel {image_key,..}=>{
                        if image_key.is_none(){let key=RasterHost::layer_pixel_buffer_key(&layer.id);if buffers.contains_key(&key){*image_key=Some(key);}}
                        if let Some(key)=image_key{if let Some(image)=buffers.get(key){images.insert(key.clone(),Arc::clone(image));}}
                    }
                    RasterStackContent::Group(children)=>collect(children,buffers,images),
                    RasterStackContent::BrightnessContrast {..}=>{}
                }
            }
        }
        collect(&mut layers,buffers,&mut images);Ok(RasterStackInput {layers,images})
    }
    pub fn advance(&mut self,budget:usize,layers:&[RasterStackLayer],buffers:&HashMap<String,Arc<PixelImage>>)->Result<PixelProgress,String>{
        if !(1..=1048576).contains(&budget){return Err("Compositing grant must be between 1 and 1048576".into());}
        if !self.pending{return self.error.clone().map_or_else(||Ok(self.progress.unwrap_or(PixelProgress {completed:0,total:0,done:true})),Err);}
        let result:Result<PixelProgress,String>=(||{
            if self.job.is_none(){self.job=Some(RasterStackJob::new(self.input(layers,buffers)?).map_err(|e|e.to_string())?);}
            let progress=self.job.as_mut().unwrap().advance(budget).map_err(|e|e.to_string())?;
            if progress.done{
                let output=self.job.take().unwrap().into_result().map_err(|e|e.to_string())?;
                self.image=(!output.empty).then(||Arc::new(image_from_rgba(output.image.width,output.image.height,output.image.pixels.clone())));
                self.output=Some(output);self.pending=false;
            }
            self.progress=Some(progress);Ok(progress)
        })();
        if let Err(error)=&result{self.error=Some(error.clone());self.pending=false;self.job=None;}result
    }
    pub fn close_step(&mut self)->bool{
        if self.job.take().is_some(){return false;}
        if self.image.take().is_some(){return false;}
        if self.output.take().is_some(){return false;}
        self.error=None;self.progress=None;self.pending=false;self.view=View::Composite;true
    }
}
