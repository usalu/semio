//! 🖌️ Bounded pixel and mask stroke intent: the layer, the tool and the samples in the target image's pixels, dispatched
//! once on release as `paintStroke`. The editor's paint tool builds the parametric leaf from them and the session brush,
//! and its ONE deterministic rasterizer paints it; the target revision only decides whether the gesture still holds.
use super::{affine_from_json,Affine,LayerNode,MaskJson,MaskState,Point,RasterHost};

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum PaintTarget {Pixels,Mask}

#[derive(Clone,Debug,PartialEq)]
enum StrokeRevision {Pixels(Option<String>),Mask(String)}

#[derive(Clone,Debug,PartialEq)]
pub struct PaintStrokeCommand {
    pub layer_id:String,
    revision:StrokeRevision,
    pub tool:&'static str,
    pub points:Vec<[f64;2]>,
}

enum RevisionRef<'a> {Pixels(Option<&'a str>),Mask(&'a MaskJson)}

pub(super) struct PaintGesture {
    pub(super) command:PaintStrokeCommand,
    mask:Option<MaskJson>,
    pub(super) world:Affine,
    inverse:Affine,
    pub(super) points:Vec<[f64;2]>,
}

fn centered(world:Affine,width:u32,height:u32,extent:(u32,u32))->Affine {
    world*Affine::new([f64::from(width)/f64::from(extent.0.max(1)),0.0,0.0,f64::from(height)/f64::from(extent.1.max(1)),-f64::from(width)/2.0,-f64::from(height)/2.0])
}

fn mask_placement<'a>(host:&RasterHost,mask:&'a MaskState,parent:Affine,owner:Affine)->(Affine,RevisionRef<'a>){
    let descriptor=&mask.descriptor;
    let extent=descriptor.image_key.as_deref().and_then(|key|host.buffers.paint.get(key)).map(|image|(image.width,image.height)).unwrap_or((mask.width,mask.height));
    let world=if descriptor.linked{parent*owner}else{parent}*affine_from_json(&descriptor.transform);
    (centered(world,descriptor.width.unwrap_or(extent.0),descriptor.height.unwrap_or(extent.1),extent),RevisionRef::Mask(descriptor))
}

fn target<'a>(host:&'a RasterHost,layers:&'a [LayerNode],id:&str,parent:Affine)->Option<(Affine,RevisionRef<'a>)>{
    for layer in layers {match layer{
        LayerNode::Pixel {id:layer_id,visible:true,locked:false,transform,width,height,image_key,mask,..} if layer_id==id=>{
            return match host.paint_target{
                PaintTarget::Mask=>mask.as_ref().map(|mask|mask_placement(host,mask,parent,*transform)),
                PaintTarget::Pixels=>{
                    let extent=image_key.as_deref().and_then(|key|host.buffers.paint.get(key)).map(|image|(image.width,image.height)).unwrap_or((*width,*height));
                    Some((centered(parent* *transform,*width,*height,extent),RevisionRef::Pixels(image_key.as_deref())))
                }
            };
        }
        LayerNode::Group {id:layer_id,visible:true,locked:false,transform,children,mask,..}=>{
            if layer_id==id&&host.paint_target==PaintTarget::Mask{return mask.as_ref().map(|mask|mask_placement(host,mask,parent,*transform));}
            if let Some(value)=target(host,children,id,parent* *transform){return Some(value);}
        }
        _=>{}
    }}
    None
}

impl PaintGesture {
    pub fn begin(host:&RasterHost,point:Point)->Option<Self>{
        let id=host.selected_ids.first()?;
        let (matrix,revision)=target(host,&host.document.layers,id,Affine::IDENTITY)?;
        let [a,b,c,d,e,f]=matrix.as_coeffs();let determinant=a*d-b*c;
        if !determinant.is_finite()||determinant.abs()<1e-12{return None;}
        let inverse=Affine::new([d/determinant,-b/determinant,-c/determinant,a/determinant,(c*f-d*e)/determinant,(b*e-a*f)/determinant]);
        let (revision,mask)=match revision{
            RevisionRef::Pixels(key)=>(StrokeRevision::Pixels(key.map(str::to_owned)),None),
            RevisionRef::Mask(mask)=>(StrokeRevision::Mask(dsl::json::to_json_string(mask)),Some(mask.clone())),
        };
        let mut gesture=Self{command:PaintStrokeCommand{layer_id:id.clone(),revision,tool:"brush",points:Vec::new()},mask,world:matrix,inverse,points:Vec::with_capacity(2048)};gesture.push(point);Some(gesture)
    }

    pub fn matches(&self,host:&RasterHost)->bool{
        host.selected_ids.first()==Some(&self.command.layer_id)&&target(host,&host.document.layers,&self.command.layer_id,Affine::IDENTITY).is_some_and(|(world,revision)|{
            world.as_coeffs()==self.world.as_coeffs()&&match (&self.command.revision,revision){
                (StrokeRevision::Pixels(expected),RevisionRef::Pixels(key))=>expected.as_deref()==key,
                (StrokeRevision::Mask(_),RevisionRef::Mask(mask))=>self.mask.as_ref()==Some(mask),
                _=>false,
            }
        })
    }

    pub fn push(&mut self,world:Point){
        let local=self.inverse*world;
        if !local.x.is_finite()||!local.y.is_finite()||local.x.abs()>1_000_000.0||local.y.abs()>1_000_000.0||self.points.len()>=2048{return;}
        let point=[(local.x*1000.0).round()/1000.0,(local.y*1000.0).round()/1000.0];
        if self.points.last()!=Some(&point){self.points.push(point);}
    }

    pub fn finish(mut self,erase:bool)->Option<PaintStrokeCommand>{
        if self.points.is_empty(){return None;}
        self.command.tool=if erase {"eraser"} else {"brush"};
        self.command.points=std::mem::take(&mut self.points);
        Some(self.command)
    }

    pub fn close_step(&mut self)->bool{
        if self.points.pop().is_some()||self.mask.as_mut().and_then(|mask|mask.image_key.as_mut()).is_some_and(|key|key.pop().is_some()){return false;}
        self.mask=None;self.command.close_step()
    }
}

impl PaintStrokeCommand {
    /// 🏷️ The verb a released stroke dispatches, whatever its target: the editor reads the target from its session.
    pub fn action(&self)->&'static str{"paintStroke"}
    /// 🎯️ The image the stroke was drawn against: `Pixels` with the layer's image key, `Mask` with its mask's JSON.
    pub fn target(&self)->PaintTarget{match self.revision{StrokeRevision::Pixels(_)=>PaintTarget::Pixels,StrokeRevision::Mask(_)=>PaintTarget::Mask}}
    pub fn revision_value(&self)->Option<&str>{match &self.revision{StrokeRevision::Pixels(key)=>key.as_deref(),StrokeRevision::Mask(mask)=>Some(mask)}}
    pub fn close_step(&mut self)->bool{
        if self.layer_id.pop().is_some()||self.points.pop().is_some(){return false;}
        match &mut self.revision{
            StrokeRevision::Pixels(key)=>{if key.as_mut().is_some_and(|key|key.pop().is_some()){return false;}*key=None;}
            StrokeRevision::Mask(mask)=>if mask.pop().is_some(){return false;},
        }
        true
    }
}
