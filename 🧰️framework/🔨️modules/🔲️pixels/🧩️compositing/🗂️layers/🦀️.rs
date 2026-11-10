//! 🗂️ Retained preparation of centered raster layers for the shared compositor.
use super::*;
use crate::retirement::MaskBytes;
use semio_framework_value::RetainedCloneGrant;
use semio_framework_2d::retirement::EffectTurns;

/// 🎭️ Effective coverage of an imported RGBA mask, shared by compositing and mask authoring.
pub fn mask_coverage(pixel:[u8;4])->u8 {
    ((0.2126*f64::from(pixel[0])+0.7152*f64::from(pixel[1])+0.0722*f64::from(pixel[2]))*f64::from(pixel[3])/255.0).round() as u8
}

#[derive(Clone,Copy,Debug)]
#[derive(semio_framework_value::RetireOwned)]
pub struct RasterStackTransform {pub x:f64,pub y:f64,pub a:f64,pub b:f64,pub c:f64,pub d:f64}
#[derive(Clone,Debug)]
#[derive(semio_framework_value::RetireOwned)]
pub struct RasterStackMask {pub enabled:bool,pub linked:bool,pub invert:bool,pub transform:RasterStackTransform,pub width:Option<u32>,pub height:Option<u32>,pub image_key:Option<String>}
#[derive(Clone,Debug)]
#[derive(semio_framework_value::RetireOwned)]
pub enum RasterStackContent {Pixel {width:Option<u32>,height:Option<u32>,image_key:Option<String>},Group(Vec<RasterStackLayer>),BrightnessContrast {brightness:f64,contrast:f64}}
#[derive(Clone,Debug)]
#[derive(semio_framework_value::RetireOwned)]
pub struct RasterStackLayer {pub id:String,pub visible:bool,pub opacity:f64,pub blend_mode:CompositeBlend,pub transform:RasterStackTransform,pub mask:Option<RasterStackMask>,pub content:RasterStackContent}
#[derive(semio_framework_value::RetireOwned)]
pub struct RasterStackInput {pub layers:Vec<RasterStackLayer>,pub images:Vec<(String,RasterLease)>}
#[derive(semio_framework_value::RetireOwned)]
pub struct RasterStackResult {pub image:RasterImage,pub origin:[f64;2],pub empty:bool}
#[derive(Clone,Copy)]
#[derive(semio_framework_value::RetireOwned)]
struct Frame {world:CompositeAffine,enabled:bool,paint:bool}
#[derive(semio_framework_value::RetireOwned)]
struct Building {image:RasterLease,bytes:Vec<u8>,count:usize}
fn matrix(value:RasterStackTransform)->Result<CompositeAffine,PixelEditError>{
    let result=[value.a,value.b,value.c,value.d,value.x,value.y];inverse(result)?;Ok(result)
}

fn centered(value:CompositeAffine,width:u32,height:u32,source_width:u32,source_height:u32)->Result<CompositeAffine,PixelEditError>{
    validate_extent(width,height)?;
    Ok(multiply(value,[f64::from(width)/f64::from(source_width),0.0,0.0,f64::from(height)/f64::from(source_height),-f64::from(width)/2.0,-f64::from(height)/2.0]))
}
const MASK_CHUNK:usize=256;
const MASK_BUDGET:usize=67_108_864;
fn missing()->PixelEditError {PixelEditError::Invalid("Layer image is missing")}
fn source<'a>(images:&'a [(String,RasterLease)],key:&str)->Result<&'a RasterLease,PixelEditError> {
    if key.is_empty(){return Err(missing());}
    images.iter().find(|(id,_)|id==key).map(|(_,image)|image).ok_or_else(missing)
}
fn siblings<'a>(mut layers:&'a [RasterStackLayer],path:&[usize])->Option<&'a [RasterStackLayer]> {
    for index in path {layers=match &layers.get(*index)?.content {RasterStackContent::Group(children)=>children,_=>return None};}
    Some(layers)
}
fn destination<'a>(mut layers:&'a mut Vec<CompositeLayer>,path:&[usize])->Option<&'a mut Vec<CompositeLayer>> {
    for index in path {layers=match &mut layers.get_mut(*index)?.content {CompositeContent::Group(children)=>children,_=>return None};}
    Some(layers)
}
fn contains(layer:&RasterStackLayer,id:&str)->bool {layer.id==id||matches!(&layer.content,RasterStackContent::Group(children) if children.iter().any(|child|contains(child,id)))}
fn seen(layers:&[RasterStackLayer],cursor:&[usize],id:&str)->bool {
    let Some((&at,rest))=cursor.split_first() else {return false};
    layers.iter().take(at).any(|layer|contains(layer,id))||(!rest.is_empty()&&layers.get(at).is_some_and(|layer|layer.id==id||matches!(&layer.content,RasterStackContent::Group(children) if seen(children,rest,id))))
}
fn refused(error:&'static str)->PixelEditError {PixelEditError::Invalid(error)}

/// 🧵️ Plans centered layers, converts unique masks and composites them, every step funded by the caller's retained-work ledger; a keyless pixel layer keeps its bounds and composes as the identity adjustment.
#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
pub struct RasterStackJob {#[doc(hidden)] pub input:RasterStackInput,layers:Vec<CompositeLayer>,prepared:Vec<(RasterLease,MaskLease)>,building:Option<Building>,scratch:String,composite:Option<CompositeJob>,output:Option<RasterStackResult>,phase:&'static str,frames:[Frame;33],at:[usize;33],depth:usize,nodes:usize,bounds:[f64;4],coverage_total:usize,planned:usize,origin:[f64;2],empty:bool,cancelled:bool,failed:Option<PixelEditError>,pending:RetirementDemand,last_work:u64,last_semantic:u64,step_cpu:u64}
impl RasterStackJob {
    pub fn new(input:RasterStackInput)->Self {
        Self {input,layers:Vec::new(),prepared:Vec::new(),building:None,scratch:String::new(),composite:None,output:None,phase:"planning",frames:[Frame {world:IDENTITY,enabled:true,paint:true};33],at:[0;33],depth:0,nodes:0,bounds:[f64::INFINITY,f64::INFINITY,f64::NEG_INFINITY,f64::NEG_INFINITY],coverage_total:0,planned:0,origin:[0.0;2],empty:true,cancelled:false,failed:None,pending:RetirementDemand {depth:1,..Default::default()},last_work:0,last_semantic:0,step_cpu:0}
    }
    pub fn progress(&self)->PixelProgress {
        let done=self.phase=="complete";
        let(completed,total)=match &self.composite {Some(job)=>{let p=job.progress();(self.planned+p.completed,self.planned+p.total)},None=>(self.planned,self.planned)};
        PixelProgress {completed,total:if done{total}else{total.max(completed+1)},done}
    }
    pub fn next_work_demand(&self)->RetirementDemand {self.composite.as_ref().map_or(self.pending,CompositeJob::next_work_demand)}
    pub fn last_turn_work(&self)->u64 {self.last_work}
    pub fn last_turn_semantic_work(&self)->u64 {self.last_semantic}
    pub fn failure_pending(&self)->bool {self.failed.is_some()}
    pub fn settlement_pending(&self)->bool {self.composite.as_ref().is_some_and(CompositeJob::settlement_pending)}
    fn mask(&mut self,image:RasterLease,work:&mut RetainedWorkLedger,alloc:&mut Option<usize>)->Result<bool,PixelEditError> {
        if self.building.is_none() {
            validate_image(&image)?;let count=validate_extent(image.width,image.height)?;
            if self.coverage_total+count>MASK_BUDGET{return Err(PixelEditError::Invalid("Mask coverage exceeds 64 MiB preparation budget"));}
            self.coverage_total+=count;self.building=Some(Building {image,bytes:Vec::new(),count});self.step_cpu=1;return Ok(true);
        }
        let Some(building)=self.building.as_mut() else {return Err(refused("Mask preparation vanished"));};
        if building.bytes.capacity()<building.count&&!work.reserve_vec(&mut building.bytes,building.count,alloc).map_err(refused)?{return Ok(false);}
        if building.bytes.len()<building.count {
            let have=building.bytes.len();let take=(building.count-have).min(MASK_CHUNK);
            if !work.copy_bytes(take){return Ok(false);}
            for index in have..have+take {let at=index*4;let pixel=&building.image.pixels;building.bytes.push(mask_coverage([pixel[at],pixel[at+1],pixel[at+2],pixel[at+3]]));}
            self.planned+=1;self.step_cpu=1;return Ok(true);
        }
        if self.prepared.capacity()==self.prepared.len() {let target=self.prepared.len().checked_mul(2).ok_or(refused("Mask table overflow"))?.max(4);if !work.reserve_vec(&mut self.prepared,target,alloc).map_err(refused)?{return Ok(false);}}
        if !work.arc_ready::<MaskBytes>().map_err(refused)?||!work.vec_slot(&mut self.prepared).map_err(refused)?{return Ok(false);}
        let Some(finished)=self.building.take() else {return Err(refused("Mask preparation vanished"));};
        let lease=MaskLease(work.arc_new(MaskBytes(finished.bytes),alloc).map_err(|(error,_)|refused(error))?);
        work.vec_push(&mut self.prepared,(finished.image,lease)).map_err(|(error,_)|refused(error))?;
        self.step_cpu=1;Ok(true)
    }
    fn measure(&mut self)->Result<bool,PixelEditError> {
        self.step_cpu=1;
        let empty=self.bounds[0]==f64::INFINITY;
        let origin=if empty{[0.0;2]}else{[self.bounds[0],self.bounds[1]]};
        let extents=if empty{[1.0;2]}else{[(self.bounds[2]-origin[0]).ceil().max(1.0),(self.bounds[3]-origin[1]).ceil().max(1.0)]};
        if !origin.iter().all(|v|v.is_finite())||!extents.iter().all(|v|v.is_finite()&&*v<=16384.0){return Err(PixelEditError::Invalid("Raster stack bounds exceed allowed extents"));}
        let width=extents[0] as u32;let height=extents[1] as u32;validate_extent(width,height)?;
        self.origin=origin;self.empty=empty;
        self.composite=Some(CompositeJob::new_owned(width,height,origin,std::mem::take(&mut self.input.images),std::mem::take(&mut self.layers)));
        self.phase="compositing";Ok(true)
    }
    fn plan(&mut self,work:&mut RetainedWorkLedger,alloc:&mut Option<usize>)->Result<bool,PixelEditError> {
        if self.building.is_some(){let image=self.building.as_ref().map(|b|b.image.clone());return match image {Some(image)=>self.mask(image,work,alloc),None=>Ok(false)};}
        if self.phase=="measuring"{return self.measure();}
        let d=self.depth;
        let Some(level)=siblings(&self.input.layers,&self.at[..d]) else {return Err(refused("Layer tree diverged from its plan"));};
        let at=self.at[d];
        if at==level.len() {self.step_cpu=1;if d==0{self.phase="measuring";}else{self.depth=d-1;self.at[d-1]+=1;}return Ok(true);}
        let Some(destination)=destination(&mut self.layers,&self.at[..d]) else {return Err(refused("Layer tree diverged from its plan"));};
        if destination.capacity()<level.len()&&!work.reserve_vec(destination,level.len(),alloc).map_err(refused)?{return Ok(false);}
        let layer=&level[at];
        if layer.id.is_empty()||seen(&self.input.layers,&self.at[..=d],&layer.id)||self.nodes>=1024{return Err(PixelEditError::Invalid("Layer identifiers must be unique within 1024 nodes"));}
        if !layer.opacity.is_finite()||!(0.0..=1.0).contains(&layer.opacity){return Err(PixelEditError::Invalid("Invalid layer opacity"));}
        let frame=self.frames[d];let transform=matrix(layer.transform)?;let visible=frame.enabled&&layer.visible;
        let opacity=if layer.mask.as_ref().is_some_and(|m|m.enabled&&m.image_key.is_none()&&m.invert){0.0}else{layer.opacity};
        let active=frame.paint&&visible&&opacity>0.0;let mut local=transform;
        if let RasterStackContent::Pixel {width,height,image_key}=&layer.content {
            let image=image_key.as_deref().map(|key|source(&self.input.images,key)).transpose()?;
            let(source_width,source_height)=image.map_or((1,1),|image|(image.width,image.height));
            local=centered(transform,width.unwrap_or(if image.is_some(){source_width}else{512}),height.unwrap_or(if image.is_some(){source_height}else{512}),source_width,source_height)?;
            let world=multiply(frame.world,local);
            if visible {for(x,y) in [(0.0,0.0),(f64::from(source_width),0.0),(0.0,f64::from(source_height)),(f64::from(source_width),f64::from(source_height))] {
                let px=world[0]*x+world[2]*y+world[4];let py=world[1]*x+world[3]*y+world[5];
                self.bounds[0]=self.bounds[0].min(px);self.bounds[1]=self.bounds[1].min(py);self.bounds[2]=self.bounds[2].max(px);self.bounds[3]=self.bounds[3].max(py);
            }}
        }
        let world=multiply(frame.world,local);inverse(world)?;
        if matches!(layer.content,RasterStackContent::BrightnessContrast {..})&&layer.mask.is_some(){return Err(PixelEditError::Invalid("Adjustment layers cannot own masks"));}
        let mut mask=None;
        if let Some(spec)=&layer.mask {
            let placement=matrix(spec.transform)?;
            if let Some(width)=spec.width {validate_extent(width,1)?;}
            if let Some(height)=spec.height {validate_extent(1,height)?;}
            if let(Some(width),Some(height))=(spec.width,spec.height){validate_extent(width,height)?;}
            if let(true,Some(key))=(spec.enabled,spec.image_key.as_deref()) {
                let image=source(&self.input.images,key)?.clone();
                let mapping=multiply(inverse(local)?,multiply(if spec.linked{transform}else{IDENTITY},centered(placement,spec.width.unwrap_or(image.width),spec.height.unwrap_or(image.height),image.width,image.height)?));
                inverse(mapping)?;inverse(multiply(world,mapping))?;
                let Some(coverage)=self.prepared.iter().find(|(prepared,_)|Arc::ptr_eq(&prepared.0,&image.0)).map(|(_,coverage)|coverage.clone()) else {return self.mask(image,work,alloc);};
                mask=Some(CompositeMask {width:image.width,height:image.height,coverage,transform:mapping,invert:spec.invert});
            }
        }
        let mut blend=layer.blend_mode;
        let content=match &layer.content {
            RasterStackContent::Pixel {image_key:Some(key),..}=>{
                if self.scratch.capacity()<key.len()&&!work.reserve_string(&mut self.scratch,key.len(),alloc).map_err(refused)?{return Ok(false);}
                if self.scratch.is_empty()&&!work.string_push(&mut self.scratch,key).map_err(refused)?{return Ok(false);}
                CompositeContent::Pixels(std::mem::take(&mut self.scratch))
            },
            RasterStackContent::Pixel {..}=>{blend=CompositeBlend::Normal;mask=None;CompositeContent::Adjustment {brightness:0.0,contrast:0.0}},
            RasterStackContent::Group(_)=>{if d+1>32{return Err(PixelEditError::Invalid("Invalid layer nesting"));}CompositeContent::Group(Vec::new())},
            RasterStackContent::BrightnessContrast {brightness,contrast}=>{
                if !brightness.is_finite()||!contrast.is_finite()||!(-1.0..=1.0).contains(brightness)||!(-1.0..=1.0).contains(contrast){return Err(PixelEditError::Invalid("Adjustment exceeds allowed range"));}
                CompositeContent::Adjustment {brightness:*brightness,contrast:*contrast}
            },
        };
        if !work.vec_slot(destination).map_err(refused)?{if let CompositeContent::Pixels(key)=content{self.scratch=key;}return Ok(false);}
        work.vec_push(destination,CompositeLayer {opacity,blend,visible:layer.visible,transform:local,mask,content}).map_err(|(error,_)|refused(error))?;
        self.nodes+=1;self.planned+=1;self.step_cpu=1;
        if matches!(layer.content,RasterStackContent::Group(_)) {self.frames[d+1]=Frame {world,enabled:visible,paint:active};self.depth=d+1;self.at[d+1]=0;}else{self.at[d]+=1;}
        Ok(true)
    }
    pub fn advance(&mut self,mut budget:usize,work:&mut RetainedWorkLedger,alloc:&mut Option<usize>)->Result<PixelProgress,PixelEditError> {
        self.last_work=0;self.last_semantic=0;
        if self.cancelled{return Err(PixelEditError::Cancelled);}
        if let Some(error)=&self.failed{return Err(error.clone());}
        if !(1..=1_048_576).contains(&budget){return Err(PixelEditError::Invalid("Stack grant must be between 1 and 1048576"));}
        if self.composite.is_none() {
            if !work.admits_absolute(self.pending){work.require_absolute(self.pending);return Ok(self.progress());}
            let Some(root)=work.enter() else {self.pending=work.next_demand();return Ok(self.progress());};
            let mut used=0;
            let result=(||{for _ in 0..budget {
                if self.composite.is_some()||!work.can_step(){break;}
                used+=1;self.step_cpu=0;let before=work.receipt();let result=self.plan(work,alloc);
                if self.step_cpu>0||!matches!(&result,Ok(false))||work.receipt()!=before {self.last_work+=1;work.note_source(root);}
                if matches!(&result,Ok(true)){self.last_semantic+=1;}
                self.pending=work.next_demand();
                if let Err(error)=result {self.failed=Some(error.clone());return Err(error);}
                if work.blocked(){break;}
            }Ok(())})();
            work.exit();result?;budget-=used;
        }
        if budget>0&&!work.blocked() {
            if let Some(job)=self.composite.as_mut() {
                let outcome=job.advance(budget,work,alloc);
                self.last_work+=job.last_turn_work();self.last_semantic+=job.last_turn_semantic_work();self.pending=job.next_work_demand();
                match outcome {
                    Err(error)=>{self.failed=Some(error.clone());return Err(error);},
                    Ok(progress)=>if progress.done&&self.output.is_none() {
                        let Some(image)=job.candidate.take() else {self.failed=Some(PixelEditError::Incomplete);return Err(PixelEditError::Incomplete);};
                        self.output=Some(RasterStackResult {image,origin:self.origin,empty:self.empty});self.phase="complete";
                    },
                }
            }
        }
        Ok(self.progress())
    }
    pub fn result(&self)->Result<&RasterStackResult,PixelEditError> {
        if self.cancelled{return Err(PixelEditError::Cancelled);}
        if let Some(error)=&self.failed{return Err(error.clone());}
        self.output.as_ref().ok_or(PixelEditError::Incomplete)
    }
    pub fn cancel(&mut self){self.cancelled=true;}
    /// 🧹️ Moves the finished output out without allocation and leaves every private owner in controlled retirement.
    pub fn into_retirement(mut self)->(RasterStackRetirement,Option<RasterStackResult>) {
        let output=if !self.cancelled&&self.failed.is_none()&&self.phase=="complete"{self.output.take()}else{None};
        self.cancelled=true;(RasterStackRetirement::new(self),output)
    }
    /// 🎁️ Returns the finished output after closing each retirement step inside an original effect the caller's host opens.
    pub fn into_result(self,turns:&mut impl EffectTurns)->Result<RasterStackResult,PixelEditError> {
        let status=self.result().map(|_|());let(mut retired,output)=self.into_retirement();
        let denied=|_|refused("Stack retirement refused its quote");
        while !retired.terminal_is_empty() {
            let copy=retired.next_copy_byte_demand().map_err(denied)?;let release=retired.next_release_byte_demand().map_err(denied)?;
            let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:retired.next_capacity_byte_demand(if copy>0{copy}else{release}).map_err(denied)?,maximum_release_bytes:release,maximum_depth:retired.next_depth_demand().map_err(denied)?};
            let semio_framework_value::RetainedWorkPoll::Complete(turn)=turns.turn(grant,|effect|retired.close_step(copy,effect)).map_err(denied)? else {return Err(refused("Stack retirement stayed pending"));};
            let work=turn.work;turn.result.map_err(|_|refused("Stack retirement refused its physical grant"))?;
            if work==0 {return Err(refused("Stack retirement made no funded progress"));}
        }
        status?;output.ok_or(PixelEditError::Incomplete)
    }
}
physical_work_retirement!(RasterStackRetirement,RasterStackJob);
