//! 🖌️ Bounded pixel and mask edit intent: the layer, the tool and the samples in the target image's pixels — a brush or
//! eraser stroke dispatched as `paintStroke` (once on release when short, else streamed in batches of
//! [`RASTER_STROKE_STREAM_BATCH`] samples under one press id and committed on release), a bucket click dispatched at once
//! as `fillRegion`. The editor's tools build the parametric leaf from them and the session style, and its ONE
//! deterministic engine paints or floods it — the editor previews a streamed stroke from its window transient. A stroke
//! holds while its target's placement holds (the layer selected, paintable, at the same pixel grid in the world): the
//! leaf names samples and brush, never the image under them, so the image's content changing — a preview, a peer's
//! edit — never cancels it.
use super::{affine_from_json,Affine,LayerNode,MaskState,Point,RasterHost};
use std::sync::atomic::{AtomicU64,Ordering};

/// 🌊️ How many new samples a stroke gathers before it streams them as one `paintStroke{phase: stream}` tick.
pub const RASTER_STROKE_STREAM_BATCH:usize=16;

static STROKE_PRESS:AtomicU64=AtomicU64::new(0);

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum PaintTarget {Pixels,Mask}

#[derive(Clone,Debug,PartialEq)]
pub struct PaintEditCommand {
    pub layer_id:String,
    target:PaintTarget,
    pub tool:&'static str,
    pub points:Vec<[f64;2]>,
    /// 🌊️ Where the edit sits in a streamed stroke (`stream`, `commit`, `abort`); `None` for a whole stroke or a click.
    pub phase:Option<&'static str>,
    /// 🧯️ Why a streamed stroke was abandoned (`captureLost`); `None` otherwise.
    pub reason:Option<&'static str>,
    /// 🆔️ The press a streamed stroke's ticks name; `None` for a whole stroke or a click.
    pub gesture:Option<String>,
}

pub(super) struct PaintGesture {
    pub(super) command:PaintEditCommand,
    pub(super) world:Affine,
    inverse:Affine,
    extent:(u32,u32),
    pub(super) points:Vec<[f64;2]>,
    gesture:String,
    sent:usize,
}

fn centered(world:Affine,width:u32,height:u32,extent:(u32,u32))->Affine {
    world*Affine::new([f64::from(width)/f64::from(extent.0.max(1)),0.0,0.0,f64::from(height)/f64::from(extent.1.max(1)),-f64::from(width)/2.0,-f64::from(height)/2.0])
}

fn mask_placement(host:&RasterHost,mask:&MaskState,parent:Affine,owner:Affine)->(Affine,(u32,u32)){
    let descriptor=&mask.descriptor;
    let extent=descriptor.image_key.as_deref().and_then(|key|host.buffers.paint.get(key)).map(|image|(image.width,image.height)).unwrap_or((mask.width,mask.height));
    let world=if descriptor.linked{parent*owner}else{parent}*affine_from_json(&descriptor.transform);
    (centered(world,descriptor.width.unwrap_or(extent.0),descriptor.height.unwrap_or(extent.1),extent),extent)
}

/// 🎯️ Where the selected layer's paint target sits: its pixel grid in the world and its pixel extent; `None` when the
/// layer is hidden, protected, missing or has no such target.
fn target(host:&RasterHost,layers:&[LayerNode],id:&str,parent:Affine)->Option<(Affine,(u32,u32))>{
    for layer in layers {match layer{
        LayerNode::Pixel {id:layer_id,visible:true,locked:false,transform,width,height,image_key,mask,..} if layer_id==id=>{
            return match host.paint_target{
                PaintTarget::Mask=>mask.as_ref().map(|mask|mask_placement(host,mask,parent,*transform)),
                PaintTarget::Pixels=>{
                    let extent=image_key.as_deref().and_then(|key|host.buffers.paint.get(key)).map(|image|(image.width,image.height)).unwrap_or((*width,*height));
                    Some((centered(parent* *transform,*width,*height,extent),extent))
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
    pub fn begin(host:&RasterHost,point:Point,erase:bool)->Option<Self>{
        let id=host.selected_ids.first()?;
        let (matrix,extent)=target(host,&host.document.layers,id,Affine::IDENTITY)?;
        let [a,b,c,d,e,f]=matrix.as_coeffs();let determinant=a*d-b*c;
        if !determinant.is_finite()||determinant.abs()<1e-12{return None;}
        let inverse=Affine::new([d/determinant,-b/determinant,-c/determinant,a/determinant,(c*f-d*e)/determinant,(b*e-a*f)/determinant]);
        let tool=if erase {"eraser"} else {"brush"};
        let press=format!("stroke:{}",STROKE_PRESS.fetch_add(1,Ordering::Relaxed));
        let mut gesture=Self{command:PaintEditCommand{layer_id:id.clone(),target:host.paint_target,tool,points:Vec::new(),phase:None,reason:None,gesture:None},world:matrix,inverse,extent,points:Vec::with_capacity(2048),gesture:press,sent:0};gesture.push(point);Some(gesture)
    }

    /// 🪣️ One bucket click on the selected layer's target: the clicked point in its image's pixels, or nothing when the
    /// layer cannot be filled or the point is off its pixel grid.
    pub fn click(host:&RasterHost,point:Point)->Option<PaintEditCommand>{
        let mut gesture=Self::begin(host,point,false)?;
        let [x,y]=*gesture.points.first()?;
        if !(x>=0.0&&y>=0.0&&x<f64::from(gesture.extent.0)&&y<f64::from(gesture.extent.1)){return None;}
        gesture.command.tool="bucket";
        gesture.command.points=std::mem::take(&mut gesture.points);
        Some(gesture.command)
    }

    /// 🎯️ Whether the gesture still holds: the same layer selected, its same target paintable at the same pixel grid.
    pub fn matches(&self,host:&RasterHost)->bool{
        host.selected_ids.first()==Some(&self.command.layer_id)&&host.paint_target==self.command.target&&target(host,&host.document.layers,&self.command.layer_id,Affine::IDENTITY).is_some_and(|(world,extent)|world.as_coeffs()==self.world.as_coeffs()&&extent==self.extent)
    }

    pub fn push(&mut self,world:Point){
        let local=self.inverse*world;
        if !local.x.is_finite()||!local.y.is_finite()||local.x.abs()>1_000_000.0||local.y.abs()>1_000_000.0||self.points.len()>=2048{return;}
        let point=[(local.x*1000.0).round()/1000.0,(local.y*1000.0).round()/1000.0];
        if self.points.last()!=Some(&point){self.points.push(point);}
    }

    /// 🌊️ The next stream tick once [`RASTER_STROKE_STREAM_BATCH`] new samples gathered: those samples under the press id.
    pub fn stream(&mut self)->Option<PaintEditCommand>{
        if self.points.len()-self.sent<RASTER_STROKE_STREAM_BATCH{return None;}
        let points=self.points[self.sent..].to_vec();
        self.sent=self.points.len();
        Some(PaintEditCommand{points,phase:Some("stream"),gesture:Some(self.gesture.clone()),..self.command.clone()})
    }

    /// 🏁️ The release: the whole stroke at once when nothing streamed, else the commit of the samples not yet streamed.
    pub fn finish(mut self)->Option<PaintEditCommand>{
        if self.sent>0{
            return Some(PaintEditCommand{points:self.points.split_off(self.sent),phase:Some("commit"),gesture:Some(self.gesture),..self.command});
        }
        if self.points.is_empty(){return None;}
        self.command.points=std::mem::take(&mut self.points);
        Some(self.command)
    }

    /// 🧯️ A dropped gesture: the abort of its press when it already streamed (the editor holds an open stroke), else nothing.
    pub fn abandon(self)->Option<PaintEditCommand>{
        (self.sent>0).then(||PaintEditCommand{points:Vec::new(),phase:Some("abort"),reason:Some("captureLost"),gesture:Some(self.gesture),..self.command})
    }

    pub fn close_step(&mut self)->bool{
        if self.points.pop().is_some()||self.gesture.pop().is_some(){return false;}
        self.command.close_step()
    }
}

impl PaintEditCommand {
    /// 🏷️ The verb the edit dispatches, whatever its target: a bucket click fills, a stroke paints; the editor reads the
    /// target from its session.
    pub fn action(&self)->&'static str{if self.tool=="bucket"{"fillRegion"}else{"paintStroke"}}
    /// 🎯️ The target the edit paints: the layer's pixels or its mask.
    pub fn target(&self)->PaintTarget{self.target}
    pub fn close_step(&mut self)->bool{
        if self.layer_id.pop().is_some()||self.points.pop().is_some()||self.gesture.as_mut().is_some_and(|gesture|gesture.pop().is_some()){return false;}
        self.gesture=None;
        true
    }
}
