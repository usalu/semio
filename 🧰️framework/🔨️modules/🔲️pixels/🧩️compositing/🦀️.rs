//! 🧩️ Isolated RGBA compositing following https://www.w3.org/TR/compositing-1/.
use crate::{editing::{validate_extent,validate_image,PixelEditError,PixelProgress},RasterImage};
use crate::retirement::{RasterLease,MaskLease};
use semio_framework_2d::physical_work_retirement;
use std::{collections::BTreeMap,sync::Arc};

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
pub struct CompositeInput {pub width:u32,pub height:u32,pub origin:[f64;2],pub images:BTreeMap<String,Arc<RasterImage>>,pub layers:Vec<CompositeLayer>}

#[derive(Clone,Debug)]
#[derive(semio_framework_value::RetireOwned)]
struct Mask {width:u32,height:u32,coverage:MaskLease,inverse:CompositeAffine,invert:bool}
#[derive(Clone,Debug)]
#[derive(semio_framework_value::RetireOwned)]
struct Style {opacity:f64,blend:CompositeBlend,mask:Option<Mask>}
#[derive(Debug)]
#[derive(semio_framework_value::RetireOwned)]
enum Command {Begin,Commit,Draw {image:RasterLease,inverse:CompositeAffine,style:Style},End(Style),Adjust {slope:f64,intercept:f64,style:Style}}
#[derive(Debug)]
#[derive(semio_framework_value::RetireOwned)]
struct Step {depth:usize,command:Command}
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

struct Compiler<'a> {images:&'a [(String,RasterLease)],commands:Vec<Step>,nodes:usize,max_depth:usize}
impl Compiler<'_> {
    fn layers(&mut self,layers:&[CompositeLayer],parent:CompositeAffine,depth:usize,enabled:bool)->Result<(),PixelEditError> {
        if depth>32{return Err(PixelEditError::Invalid("Layer nesting exceeds compositor budget"));}
        self.max_depth=self.max_depth.max(depth);
        for layer in layers {
            self.nodes+=1;
            if self.nodes>1024{return Err(PixelEditError::Invalid("Layer count exceeds compositor budget"));}
            if !layer.opacity.is_finite()||!(0.0..=1.0).contains(&layer.opacity){return Err(PixelEditError::Invalid("Invalid layer opacity"));}
            inverse(layer.transform)?;
            let world=multiply(parent,layer.transform);let inv=inverse(world)?;let active=enabled&&layer.visible&&layer.opacity>0.0;
            let mask=if let Some(mask)=&layer.mask {
                if mask.coverage.len()!=validate_extent(mask.width,mask.height)?{return Err(PixelEditError::Invalid("Invalid mask coverage"));}
                inverse(mask.transform)?;
                Some(Mask {width:mask.width,height:mask.height,coverage:MaskLease(Arc::clone(&mask.coverage.0)),invert:mask.invert,inverse:inverse(multiply(world,mask.transform))?})
            }else{None};
            let style=Style {opacity:layer.opacity,blend:layer.blend,mask};
            match &layer.content {
                CompositeContent::Pixels(key)=>{
                    let image=self.images.iter().find(|(id,_)|id==key).map(|(_,image)|image).ok_or(PixelEditError::Invalid("Layer image is missing"))?;
                    if active{self.commands.push(Step {depth,command:Command::Draw {image:image.clone(),inverse:inv,style}});}
                }
                CompositeContent::Group(children)=>{
                    if active{self.commands.push(Step {depth:depth+1,command:Command::Begin});}
                    self.layers(children,world,depth+1,active)?;
                    if active{self.commands.push(Step {depth:depth+1,command:Command::End(style)});}
                }
                CompositeContent::Adjustment {brightness,contrast}=>{
                    if !brightness.is_finite()||!contrast.is_finite()||!(-1.0..=1.0).contains(brightness)||!(-1.0..=1.0).contains(contrast){return Err(PixelEditError::Invalid("Adjustment exceeds allowed range"));}
                    let slope=2.0_f64.powf(contrast*4.0);
                    if active{self.commands.push(Step {depth,command:Command::Adjust {slope,intercept:(brightness-0.5)*slope+0.5,style}});}
                }
            }
        }
        Ok(())
    }
}

/// 🧱️ Incremental tile compositor sharing immutable source assets and retaining a private candidate.
#[derive(semio_framework_value::RetireOwned)]
pub struct CompositeJob {images:Vec<(String,RasterLease)>,layers:Vec<CompositeLayer>,commands:Vec<Step>,buffers:Vec<Vec<[f64;4]>>,candidate:Option<RasterImage>,origin:[f64;2],width:usize,count:usize,total:usize,tile:usize,command:usize,offset:usize,completed:usize,cancelled:bool}
impl CompositeJob {
    pub fn new(input:CompositeInput)->Result<Self,PixelEditError> {
        Self::new_owned(input.width,input.height,input.origin,input.images.into_iter().map(|(key,image)|(key,RasterLease(image))).collect(),input.layers)
    }
    /// 🎟️ Retains original ordered image and layer backing for explicitly funded retirement.
    pub fn new_owned(width:u32,height:u32,origin:[f64;2],images:Vec<(String,RasterLease)>,layers:Vec<CompositeLayer>)->Result<Self,PixelEditError> {
        let count=validate_extent(width,height)?;
        if !origin.iter().all(|v|v.is_finite()){return Err(PixelEditError::Invalid("Origin must contain finite coordinates"));}
        if images.len()>1024{return Err(PixelEditError::Invalid("Image count exceeds compositor budget"));}
        for (at,(key,image)) in images.iter().enumerate(){if images[..at].iter().any(|(id,_)|id==key){return Err(PixelEditError::Invalid("Image identifiers must be unique"));}validate_image(image)?;}
        let mut compiler=Compiler {images:&images,commands:Vec::new(),nodes:0,max_depth:0};
        compiler.layers(&layers,IDENTITY,0,true)?;
        compiler.commands.push(Step {depth:0,command:Command::Commit});
        let total=compiler.commands.len().checked_mul(count).ok_or(PixelEditError::Invalid("Compositing work exceeds numeric limits"))?;
        let buffers=vec![vec![[0.0;4];TILE];compiler.max_depth+1];let commands=compiler.commands;
        Ok(Self {images,layers,commands,buffers,candidate:Some(RasterImage {width,height,pixels:vec![0;count*4]}),origin,width:width as usize,count,total,tile:0,command:0,offset:0,completed:0,cancelled:false})
    }
    pub fn advance(&mut self,budget:usize)->Result<PixelProgress,PixelEditError> {
        if self.cancelled{return Err(PixelEditError::Cancelled);}
        if !(1..=1_048_576).contains(&budget){return Err(PixelEditError::Invalid("Compositing grant must be between 1 and 1048576"));}
        for _ in 0..budget {
            if self.tile>=self.count{break;}
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
        }
        Ok(PixelProgress {completed:self.completed,total:self.total,done:self.completed==self.total})
    }
    pub fn cancel(&mut self){self.cancelled=true;}
    pub fn result(&self)->Result<&RasterImage,PixelEditError> {
        if self.cancelled{return Err(PixelEditError::Cancelled);}
        if self.completed!=self.total{return Err(PixelEditError::Incomplete);}
        Ok(self.candidate.as_ref().unwrap())
    }
    pub fn into_retirement(mut self)->(CompositeRetirement,Option<RasterImage>){
        let output=if !self.cancelled&&self.completed==self.total{self.candidate.take()}else{None};self.cancelled=true;
        (CompositeRetirement::new(self),output)
    }
}


semio_framework_value::artifact_retire_leaf!(CompositeBlend);
physical_work_retirement!(CompositeRetirement,CompositeJob,PixelEditError,|_:&str|PixelEditError::Invalid("Compositor retirement refused its physical grant"));

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

#[path="🗂️layers/🦀️.rs"]
pub mod layers;
