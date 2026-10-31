//! 🧩️ Isolated RGBA compositing following https://www.w3.org/TR/compositing-1/.
use crate::{editing::{validate_extent,validate_image,PixelEditError,PixelProgress},RasterImage};
use crate::retirement::{RasterLease,MaskLease};
use semio_framework_2d::physical_work_retirement;
use std::{collections::BTreeMap,sync::Arc};
use semio_framework_value::{work::RetainedWorkLedger,RetirementDemand};

pub type CompositeAffine = [f64;6];
#[path="📐️frames/🦀️.rs"]
pub mod frames;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum CompositeBlend {Normal,Multiply,Screen,Overlay,Darken,Lighten,ColorDodge,ColorBurn,HardLight,SoftLight,Difference,Exclusion,Hue,Saturation,Color,Luminosity}

impl std::str::FromStr for CompositeBlend {
    type Err=PixelEditError;
    fn from_str(value:&str)->Result<Self,Self::Err>{
        Ok(match value {
            "normal"=>Self::Normal,"multiply"=>Self::Multiply,"screen"=>Self::Screen,"overlay"=>Self::Overlay,
            "darken"=>Self::Darken,"lighten"=>Self::Lighten,"colorDodge"=>Self::ColorDodge,"colorBurn"=>Self::ColorBurn,
            "hardLight"=>Self::HardLight,"softLight"=>Self::SoftLight,"difference"=>Self::Difference,"exclusion"=>Self::Exclusion,
            "hue"=>Self::Hue,"saturation"=>Self::Saturation,"color"=>Self::Color,"luminosity"=>Self::Luminosity,
            _=>return Err(PixelEditError::Invalid("Unknown blend mode")),
        })
    }
}

#[derive(Clone,Debug)]
#[derive(semio_framework_value::RetireOwned)]
pub struct CompositeMask {pub width:u32,pub height:u32,pub coverage:MaskLease,pub transform:CompositeAffine,pub invert:bool}
#[derive(Clone,Debug)]
#[derive(semio_framework_value::RetireOwned)]
pub enum CompositeContent {Pixels(String),Group(Vec<CompositeLayer>),Adjustment {brightness:f64,contrast:f64}}
#[derive(Clone,Debug)]
#[derive(semio_framework_value::RetireOwned)]
pub struct CompositeLayer {pub opacity:f64,pub blend:CompositeBlend,pub visible:bool,pub transform:CompositeAffine,pub mask:Option<CompositeMask>,pub content:CompositeContent}
#[derive(Clone,Debug)]
#[derive(semio_framework_value::RetireOwned)]
pub struct CompositeInput {pub width:u32,pub height:u32,pub origin:[f64;2],pub images:BTreeMap<String,RasterLease>,pub layers:Vec<CompositeLayer>}

#[derive(Clone,Debug)]
#[derive(semio_framework_value::RetireOwned)]
struct Mask {pub width:u32,pub height:u32,pub coverage:MaskLease,inverse:CompositeAffine,invert:bool}
#[derive(Clone,Debug)]
#[derive(semio_framework_value::RetireOwned)]
struct Style {pub opacity:f64,pub blend:CompositeBlend,pub mask:Option<Mask>}
#[derive(Debug)]
#[derive(semio_framework_value::RetireOwned)]
enum Command {Begin,Commit,Draw {image:RasterLease,inverse:CompositeAffine,style:Style},End(Style),Adjust {slope:f64,intercept:f64,style:Style}}
#[derive(Debug)]
#[derive(semio_framework_value::RetireOwned)]
#[doc(hidden)] pub struct Step {depth:usize,command:Command}
const IDENTITY:CompositeAffine=[1.0,0.0,0.0,1.0,0.0,0.0];
const TILE:usize=256;

pub fn multiply(a:CompositeAffine,b:CompositeAffine)->CompositeAffine {
    [a[0]*b[0]+a[2]*b[1],a[1]*b[0]+a[3]*b[1],a[0]*b[2]+a[2]*b[3],a[1]*b[2]+a[3]*b[3],a[0]*b[4]+a[2]*b[5]+a[4],a[1]*b[4]+a[3]*b[5]+a[5]]
}
pub fn inverse(m:CompositeAffine)->Result<CompositeAffine,PixelEditError> {
    let det=m[0]*m[3]-m[1]*m[2];
    if !m.iter().all(|v|v.is_finite())||!det.is_finite()||det.abs()<1e-12 {return Err(PixelEditError::Invalid("Transform must be finite and invertible"));}
    let out=[m[3]/det,-m[1]/det,-m[2]/det,m[0]/det,(m[2]*m[5]-m[3]*m[4])/det,(m[1]*m[4]-m[0]*m[5])/det];
    if !out.iter().all(|v|v.is_finite()){return Err(PixelEditError::Invalid("Inverse transform exceeds numeric limits"));}
    Ok(out)
}
fn index_at(m:&CompositeAffine,x:f64,y:f64,width:u32,height:u32)->Option<usize> {
    let px=(m[0]*x+m[2]*y+m[4]).floor();let py=(m[1]*x+m[3]*y+m[5]).floor();
    (px>=0.0&&px<f64::from(width)&&py>=0.0&&py<f64::from(height)).then(||py as usize*width as usize+px as usize)
}
fn coverage(mask:&Option<Mask>,x:f64,y:f64)->f64 {
    let Some(mask)=mask else {return 1.0;};
    let value=index_at(&mask.inverse,x,y,mask.width,mask.height).map_or(0.0,|i|f64::from(mask.coverage[i])/255.0);
    if mask.invert {1.0-value}else{value}
}
fn luminosity(c:[f64;3])->f64 {0.3*c[0]+0.59*c[1]+0.11*c[2]}
fn saturation(c:[f64;3])->f64 {c[0].max(c[1]).max(c[2])-c[0].min(c[1]).min(c[2])}
fn set_luminosity(c:[f64;3],value:f64)->[f64;3] {
    let delta=value-luminosity(c);let mut out=c.map(|v|v+delta);
    let low=out[0].min(out[1]).min(out[2]);let high=out[0].max(out[1]).max(out[2]);
    if low<0.0 {for c in &mut out {*c=value+(*c-value)*value/(value-low);}}
    if high>1.0 {for c in &mut out {*c=value+(*c-value)*(1.0-value)/(high-value);}}
    out
}
fn set_saturation(c:[f64;3],value:f64)->[f64;3] {
    let mut order=[0,1,2];order.sort_by(|a,b|c[*a].total_cmp(&c[*b]));
    let [lo,mid,hi]=order;let mut out=[0.0;3];
    if c[hi]>c[lo] {out[mid]=(c[mid]-c[lo])*value/(c[hi]-c[lo]);out[hi]=value;}
    out
}
fn channel(back:f64,front:f64,mode:CompositeBlend)->f64 {
    use CompositeBlend::*;
    match mode {
        Multiply=>back*front,Screen=>back+front-back*front,
        Overlay=>if back<=0.5{2.0*back*front}else{1.0-2.0*(1.0-back)*(1.0-front)},
        Darken=>back.min(front),Lighten=>back.max(front),
        ColorDodge=>if back==0.0{0.0}else if front==1.0{1.0}else{(back/(1.0-front)).min(1.0)},
        ColorBurn=>if back==1.0{1.0}else if front==0.0{0.0}else{1.0-((1.0-back)/front).min(1.0)},
        HardLight=>if front<=0.5{2.0*back*front}else{1.0-2.0*(1.0-back)*(1.0-front)},
        SoftLight=>if front<=0.5{back-(1.0-2.0*front)*back*(1.0-back)}else{back+(2.0*front-1.0)*((if back<=0.25{((16.0*back-12.0)*back+4.0)*back}else{back.sqrt()})-back)},
        Difference=>(back-front).abs(),Exclusion=>back+front-2.0*back*front,_=>front,
    }
}
fn blend(back:[f64;3],front:[f64;3],mode:CompositeBlend)->[f64;3] {
    use CompositeBlend::*;
    match mode {
        Hue=>set_luminosity(set_saturation(front,saturation(back)),luminosity(back)),
        Saturation=>set_luminosity(set_saturation(back,saturation(front)),luminosity(back)),
        Color=>set_luminosity(front,luminosity(back)),Luminosity=>set_luminosity(back,luminosity(front)),
        _=>std::array::from_fn(|c|channel(back[c],front[c],mode)),
    }
}
fn source_over(back:[f64;4],front:[f64;3],alpha:f64,mode:CompositeBlend)->[f64;4] {
    if alpha==0.0{return back;}
    let ba=back[3];let out_alpha=alpha+ba*(1.0-alpha);let mixed=blend([back[0],back[1],back[2]],front,mode);
    let mut out=[0.0;4];
    for c in 0..3 {out[c]=(alpha*((1.0-ba)*front[c]+ba*mixed[c])+(1.0-alpha)*ba*back[c])/out_alpha;}
    out[3]=out_alpha;out
}

#[derive(semio_framework_value::RetireOwned)]
struct CompileFrame{at:usize,world:CompositeAffine,enabled:bool,end:Option<Style>}
fn layers_at<'a>(mut layers:&'a[CompositeLayer],path:&[usize])->&'a[CompositeLayer]{for index in path{let CompositeContent::Group(children)=&layers[*index].content else{unreachable!()};layers=children;}layers}
fn reserve<T>(values:&mut Vec<T>,needed:usize,minimum:usize,work:&mut RetainedWorkLedger)->Result<bool,PixelEditError>{if values.capacity()>=needed{return Ok(true);}let target=values.capacity().checked_mul(2).ok_or(PixelEditError::Invalid("Compositor capacity overflow"))?.max(needed).max(minimum);work.reserve_vec(values,target).map_err(PixelEditError::Invalid)}
fn slot<T>(values:&mut Vec<T>,work:&mut RetainedWorkLedger)->Result<bool,PixelEditError>{let needed=values.len()+1;if !reserve(values,needed,1,work)?{return Ok(false);}work.vec_slot(values).map_err(PixelEditError::Invalid)}

/// 🧱️ Incremental tile compositor sharing immutable source assets and retaining a private candidate.
#[derive(semio_framework_value::RetireOwned)]
pub struct CompositeJob {#[doc(hidden)] pub images:Vec<(String,RasterLease)>,map_images:BTreeMap<String,RasterLease>,#[doc(hidden)] pub layers:Vec<CompositeLayer>,#[doc(hidden)] pub commands:Vec<Step>,#[doc(hidden)] pub buffers:Vec<Vec<[f64;4]>>,#[doc(hidden)] pub candidate:Option<RasterImage>,#[doc(hidden)] pub origin:[f64;2],#[doc(hidden)] pub width:usize,#[doc(hidden)] pub count:usize,#[doc(hidden)] pub total:usize,tile:usize,command:usize,offset:usize,#[doc(hidden)] pub completed:usize,cancelled:bool,phase:&'static str,failed:Option<PixelEditError>,pending:RetirementDemand,last_work:u64,last_semantic:u64,step_cpu:u64,frames:[CompileFrame;33],path:[usize;32],frame_len:usize,nodes:usize,max_depth:usize,validate_at:usize,buffer_at:usize,refused:Option<Step>}
impl CompositeJob {
    pub fn new(input:CompositeInput)->Self{Self::from_parts(input.width,input.height,input.origin,Vec::new(),input.images,input.layers)}
    /// 🎟️ Retains original ordered image and layer backing for explicitly funded retirement.
    pub fn new_owned(width:u32,height:u32,origin:[f64;2],images:Vec<(String,RasterLease)>,layers:Vec<CompositeLayer>)->Self{Self::from_parts(width,height,origin,images,BTreeMap::new(),layers)}
    fn from_parts(width:u32,height:u32,origin:[f64;2],images:Vec<(String,RasterLease)>,map_images:BTreeMap<String,RasterLease>,layers:Vec<CompositeLayer>)->Self{let result=(||{let count=validate_extent(width,height)?;if !origin.iter().all(|v|v.is_finite()){return Err(PixelEditError::Invalid("Origin must contain finite coordinates"));}if images.len()+map_images.len()>1024{return Err(PixelEditError::Invalid("Image count exceeds compositor budget"));}Ok(count)})();let(count,failed)=match result{Ok(count)=>(count,None),Err(error)=>(0,Some(error))};Self{images,map_images,layers,commands:Vec::new(),buffers:Vec::new(),candidate:Some(RasterImage{width,height,pixels:Vec::new()}),origin,width:width as usize,count,total:0,tile:0,command:0,offset:0,completed:0,cancelled:false,phase:"validating",failed,pending:RetirementDemand{depth:1,..Default::default()},last_work:0,last_semantic:0,step_cpu:0,frames:std::array::from_fn(|_|CompileFrame{at:0,world:IDENTITY,enabled:true,end:None}),path:[0;32],frame_len:1,nodes:0,max_depth:0,validate_at:0,buffer_at:0,refused:None}}
    fn command(&mut self,step:Step,work:&mut RetainedWorkLedger)->Result<(),PixelEditError>{match work.vec_push(&mut self.commands,step){Ok(())=>Ok(()),Err((error,value))=>{self.refused=Some(value);Err(PixelEditError::Invalid(error))}}}
    fn prepare(&mut self,work:&mut RetainedWorkLedger)->Result<bool,PixelEditError>{
     if self.phase=="publication"{self.phase="complete";return Ok(true);}
     if self.phase=="validating"{self.step_cpu=1;if let Some((key,image))=self.images.get(self.validate_at){if self.images[..self.validate_at].iter().any(|(id,_)|id==key){return Err(PixelEditError::Invalid("Image identifiers must be unique"));}validate_image(image)?;self.validate_at+=1;return Ok(true);}let at=self.validate_at-self.images.len();if let Some((_,image))=self.map_images.iter().nth(at){validate_image(image)?;self.validate_at+=1;return Ok(true);}self.phase="compiling";return Ok(true);}
     if self.phase=="compiling"{self.step_cpu=1;let depth=self.frame_len-1;let frame=&self.frames[depth];let layers=layers_at(&self.layers,&self.path[..depth]);if frame.at==layers.len(){if depth==0{if !slot(&mut self.commands,work)?{return Ok(false);}self.command(Step{depth:0,command:Command::Commit},work)?;self.total=self.commands.len().checked_mul(self.count).ok_or(PixelEditError::Invalid("Compositing work exceeds numeric limits"))?;self.phase="buffers";return Ok(true);}if frame.end.is_some(){if !slot(&mut self.commands,work)?{return Ok(false);}let style=self.frames[depth].end.take().unwrap();self.command(Step{depth,command:Command::End(style)},work)?;}self.frame_len-=1;return Ok(true);}let at=frame.at;let layer=&layers[at];if self.nodes>=1024{return Err(PixelEditError::Invalid("Layer count exceeds compositor budget"));}if !layer.opacity.is_finite()||!(0.0..=1.0).contains(&layer.opacity){return Err(PixelEditError::Invalid("Invalid layer opacity"));}inverse(layer.transform)?;let world=multiply(frame.world,layer.transform);let inv=inverse(world)?;let active=frame.enabled&&layer.visible&&layer.opacity>0.0;let mask=if let Some(mask)=&layer.mask{if mask.coverage.len()!=validate_extent(mask.width,mask.height)?{return Err(PixelEditError::Invalid("Invalid mask coverage"));}inverse(mask.transform)?;Some(Mask{width:mask.width,height:mask.height,coverage:MaskLease(Arc::clone(&mask.coverage.0)),invert:mask.invert,inverse:inverse(multiply(world,mask.transform))?})}else{None};let style=Style{opacity:layer.opacity,blend:layer.blend,mask};
      match &layer.content{CompositeContent::Pixels(key)=>{let image=self.images.iter().find(|(id,_)|id==key).map(|(_,image)|image).or_else(||self.map_images.get(key)).ok_or(PixelEditError::Invalid("Layer image is missing"))?;if active{if !slot(&mut self.commands,work)?{return Ok(false);}let step=Step{depth,command:Command::Draw{image:image.clone(),inverse:inv,style}};self.command(step,work)?;}},CompositeContent::Group(_)=>{if depth==32{return Err(PixelEditError::Invalid("Layer nesting exceeds compositor budget"));}if active&&!slot(&mut self.commands,work)?{return Ok(false);}if active{self.command(Step{depth:depth+1,command:Command::Begin},work)?;}self.frames[depth].at+=1;self.frames[depth+1]=CompileFrame{at:0,world,enabled:active,end:active.then_some(style)};self.path[depth]=at;self.frame_len+=1;self.max_depth=self.max_depth.max(depth+1);self.nodes+=1;return Ok(true);},CompositeContent::Adjustment{brightness,contrast}=>{if !brightness.is_finite()||!contrast.is_finite()||!(-1.0..=1.0).contains(brightness)||!(-1.0..=1.0).contains(contrast){return Err(PixelEditError::Invalid("Adjustment exceeds allowed range"));}let slope=2.0_f64.powf(contrast*4.0);if active{if !slot(&mut self.commands,work)?{return Ok(false);}let step=Step{depth,command:Command::Adjust{slope,intercept:(brightness-0.5)*slope+0.5,style}};self.command(step,work)?;}}}self.frames[depth].at+=1;self.nodes+=1;return Ok(true);
     }
     if self.phase=="buffers"{if self.buffers.len()<=self.max_depth{if !slot(&mut self.buffers,work)?{return Ok(false);}work.vec_push(&mut self.buffers,Vec::new()).map_err(|(e,_)|PixelEditError::Invalid(e))?;return Ok(true);}if self.buffer_at<self.buffers.len(){let buffer=&mut self.buffers[self.buffer_at];if buffer.capacity()<TILE&&!work.reserve_vec(buffer,TILE).map_err(PixelEditError::Invalid)?{return Ok(false);}if buffer.len()<TILE{let n=(TILE-buffer.len()).min(16);if !work.copy_bytes(n*std::mem::size_of::<[f64;4]>()){return Ok(false);}buffer.resize(buffer.len()+n,[0.0;4]);return Ok(true);}self.buffer_at+=1;return Ok(true);}self.phase="output";return Ok(true);}
     let output=&mut self.candidate.as_mut().unwrap().pixels;let count=self.count*4;if output.capacity()<count&&!work.reserve_vec(output,count).map_err(PixelEditError::Invalid)?{return Ok(false);}if output.len()<count{let n=(count-output.len()).min(64);if !work.copy_bytes(n){return Ok(false);}output.resize(output.len()+n,0);return Ok(true);}self.phase="compositing";Ok(true)
    }
    fn render_step(&mut self,work:&mut RetainedWorkLedger)->Result<bool,PixelEditError>{if matches!(&self.commands[self.command].command,Command::Commit)&&!work.copy_bytes(4){return Ok(false);}
            let step=&self.commands[self.command];let p=self.tile+self.offset;
            let x=self.origin[0]+(p%self.width) as f64+0.5;let y=self.origin[1]+(p/self.width) as f64+0.5;
            let target=self.buffers[step.depth][self.offset];
            match &step.command {
                Command::Begin=>self.buffers[step.depth][self.offset]=[0.0;4],
                Command::Commit=>{
                    let output=&mut self.candidate.as_mut().unwrap().pixels[p*4..p*4+4];
                    for c in 0..4 {output[c]=(target[c].clamp(0.0,1.0)*255.0).round() as u8;}
                    self.buffers[0][self.offset]=[0.0;4];
                }
                Command::Draw {image,inverse,style}=>{
                    if let Some(at)=index_at(inverse,x,y,image.width,image.height) {
                        let data=&image.pixels[at*4..at*4+4];let amount=style.opacity*coverage(&style.mask,x,y);
                        let front=std::array::from_fn(|c|f64::from(data[c])/255.0);
                        self.buffers[step.depth][self.offset]=source_over(target,front,f64::from(data[3])/255.0*amount,style.blend);
                    }
                }
                Command::End(style)=>{
                    let amount=style.opacity*coverage(&style.mask,x,y);let back=self.buffers[step.depth-1][self.offset];
                    self.buffers[step.depth-1][self.offset]=source_over(back,[target[0],target[1],target[2]],target[3]*amount,style.blend);
                }
                Command::Adjust {slope,intercept,style}=>{
                    if target[3]>0.0 {
                        let amount=style.opacity*coverage(&style.mask,x,y);let back=[target[0],target[1],target[2]];
                        let mixed=blend(back,back.map(|c|(c*slope+intercept).clamp(0.0,1.0)),style.blend);
                        for c in 0..3 {self.buffers[step.depth][self.offset][c]=back[c]*(1.0-amount)+mixed[c]*amount;}
                    }
                }
            }
            self.completed+=1;self.offset+=1;
            if self.offset==TILE.min(self.count-self.tile) {
                self.offset=0;self.command+=1;
                if self.command==self.commands.len(){self.command=0;self.tile+=TILE.min(self.count-self.tile);}
            }
        if self.completed==self.total{self.phase="publication";}Ok(true)
    }
    pub fn progress(&self)->PixelProgress{PixelProgress{completed:self.completed,total:self.total,done:self.phase=="complete"}}
    pub fn next_work_demand(&self)->RetirementDemand{self.pending}
    pub fn last_turn_work(&self)->u64{self.last_work}
    pub fn last_turn_semantic_work(&self)->u64{self.last_semantic}
    pub fn failure_pending(&self)->bool{self.failed.is_some()}
    pub fn settlement_pending(&self)->bool{matches!(self.phase,"publication"|"complete")}
    pub fn advance(&mut self,budget:usize,work:&mut RetainedWorkLedger)->Result<PixelProgress,PixelEditError>{self.last_work=0;self.last_semantic=0;if self.cancelled{return Err(PixelEditError::Cancelled);}if let Some(error)=&self.failed{return Err(error.clone());}if !(1..=1_048_576).contains(&budget){return Err(PixelEditError::Invalid("Compositing grant must be between 1 and 1048576"));}if !work.admits_absolute(self.pending){work.require_absolute(self.pending);return Ok(self.progress());}let Some(root)=work.enter()else{self.pending=work.next_demand();return Ok(self.progress());};let result=(||{for _ in 0..budget{if self.phase=="complete"||!work.can_step(){break;}let semantic=self.phase=="compositing";self.step_cpu=0;let before=work.receipt();let result=if semantic{self.render_step(work)}else{self.prepare(work)};if self.step_cpu>0||!matches!(&result,Ok(false))||work.receipt()!=before{self.last_work+=1;work.note_source(root);}if semantic&&!matches!(&result,Ok(false)){self.last_semantic+=1;}self.pending=work.next_demand();if let Err(error)=result{self.failed=Some(error.clone());return Err(error);}if work.blocked(){break;}}Ok(self.progress())})();work.exit();result}
    pub fn cancel(&mut self){self.cancelled=true;}
    pub fn result(&self)->Result<&RasterImage,PixelEditError> {
        if self.cancelled{return Err(PixelEditError::Cancelled);}
        if let Some(error)=&self.failed{return Err(error.clone());}if self.phase!="complete"{return Err(PixelEditError::Incomplete);}
        Ok(self.candidate.as_ref().unwrap())
    }
    pub fn into_retirement(mut self)->(CompositeRetirement,Option<RasterImage>){
        let output=if !self.cancelled&&self.failed.is_none()&&self.phase=="complete"{self.candidate.take()}else{None};self.cancelled=true;
        (CompositeRetirement::new(self),output)
    }
}


semio_framework_value::artifact_retire_leaf!(CompositeBlend);
physical_work_retirement!(CompositeRetirement,CompositeJob);


#[path="🗂️layers/🦀️.rs"]
pub mod layers;
