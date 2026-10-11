//! 🧬️ Fuelled publication and exact native image inverse intent.
use crate::{BmpSnapshot,schema::{mutations::{BmpMutation,ReplaceImage,ReplaceSamples},snapshot::{BmpImage,BmpSampleRect}}};
use crate::schema::operations::{owned_validation::BmpOwnedValidationWork,checked_region};
use crate::schema::snapshot::{BmpNativeSample,BmpPixels,BmpRegion,mask_maximum};
use semio_framework_value::retained_clone::{bulk_run_elements,bulk_run_progress,RetainedClone,RetainedCloneCursor,RetainedCloneRef,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedCloneClose};
use semio_framework_value::ValueError;
use store::snapshot_clone_preparation::{RetainedCloneEdit,RetainedCloneEditCursor,RetainedCloneEditStep,RetainedClonePreparationFactory};
use std::sync::Arc;

pub fn factory()->Arc<dyn store::ArtifactStoreOneItemPreparationFactory<BmpSnapshot,BmpMutation>>{Arc::new(RetainedClonePreparationFactory::new(Arc::new(BmpPublication),Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::default()),Arc::new(semio_framework_value::retirement::SharedValueRetirementFactory::default()),64).expect("bmp retained publication depth"))}
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct BmpPublication;
impl RetainedCloneEdit<BmpSnapshot,BmpMutation> for BmpPublication{
    type Cursor=BmpPublicationCursor;
    fn preflight(&self,mutation:&BmpMutation,_lane:store::HistoryLane)->Result<store::ArtifactStoreOneItemFootprint,String>{Ok(store::ArtifactStoreOneItemFootprint::for_leaf::<BmpSnapshot,_>(mutation,store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))}
    fn begin_demand(&self)->semio_framework_value::retained_clone::RetainedCloneBirthDemand{semio_framework_value::retained_clone::RetainedCloneBirthDemand{capacity_bytes:0,depth:0}}
    fn snapshot_cursor_birth_demand(&self)->semio_framework_value::retained_clone::RetainedCloneBirthDemand{semio_framework_value::retained_clone::RetainedCloneBirthDemand{capacity_bytes:0,depth:0}}
    fn begin(&self,grant:RetainedCloneGrant)->Result<(Self::Cursor,RetainedCloneProgress),ValueError>{let progress=self.begin_demand().admit(grant)?;Ok((BmpPublicationCursor::new(),progress))}
}
pub struct BmpPublicationCursor{
    phase:u8,copy:Option<<BmpImage as RetainedClone>::Cursor>,inverse_image:Option<BmpImage>,displaced_image:Option<BmpImage>,inverse:Option<Vec<BmpMutation>>,old:Option<BmpSampleRect>,changed:bool,retirement:RetainedCloneClose,validation:BmpOwnedValidationWork,pixel:usize,closing:bool,cancelled:bool,
}
impl BmpPublicationCursor{
    pub fn new()->Self{Self{phase:0,copy:Some(BmpImage::retained_clone_cursor()),inverse_image:None,displaced_image:None,inverse:None,old:None,changed:false,retirement:RetainedCloneClose::default(),validation:BmpOwnedValidationWork::default(),pixel:0,closing:false,cancelled:false}}
    fn close_copy(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,String>{
        let cursor=self.copy.as_mut().ok_or("bmp: missing image clone cursor")?;
        let step=cursor.close_step(grant).map_err(ValueError::into_message)?;
        if matches!(step,RetainedCloneStep::Complete(_)){if !cursor.terminal_is_empty(){return Err("bmp: image clone close retained ownership".into());}self.copy=None;self.phase=if self.phase==1{8}else{4};}
        Ok(step.progress())
    }
}
fn unit()->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,..Default::default()}}
impl RetainedCloneEditCursor<BmpSnapshot,BmpMutation> for BmpPublicationCursor{
    fn advance(&mut self,base:RetainedCloneRef<'_,BmpSnapshot>,post:&mut BmpSnapshot,mutation:RetainedCloneRef<'_,BmpMutation>,grant:RetainedCloneGrant)->Result<RetainedCloneEditStep,ValueError>{(||->Result<RetainedCloneEditStep,String>{
        if self.closing||self.cancelled{return Err("bmp: publication cursor is closing".into());}if grant.maximum_items==0{return Ok(RetainedCloneEditStep::Progress(Default::default()));}
        if self.phase==0&&!matches!(mutation.get(),BmpMutation::ReplaceImage(_)){self.copy=None;self.phase=5;}
        let progress=match self.phase{
            0=>{let cursor=self.copy.as_mut().ok_or("bmp: missing inverse image clone")?;let step=cursor.advance(base.project(1,|value|&value.image),grant).map_err(ValueError::into_message)?;if matches!(step,RetainedCloneStep::Complete(_)){self.inverse_image=Some(cursor.take().ok_or("bmp: inverse image clone lost its owner")?);cursor.begin_close();self.phase=1;}step.progress()},
            1|3=>self.close_copy(grant)?,
            8=>{let bytes=std::mem::size_of::<BmpMutation>();if grant.maximum_capacity_bytes<bytes{return Ok(RetainedCloneEditStep::Progress(Default::default()));}let mut inverse=Vec::with_capacity(1);inverse.push(BmpMutation::ReplaceImage(ReplaceImage{image:self.inverse_image.take().ok_or("bmp: inverse image owner is absent")?}));self.inverse=Some(inverse);self.phase=2;RetainedCloneProgress{copied_items:1,retained_capacity_bytes:bytes,..Default::default()}},
            2=>{if let BmpMutation::ReplaceImage(_)=mutation.get(){let source=mutation.project(1,|value|match value{BmpMutation::ReplaceImage(value)=>&value.image,_=>unreachable!()});let cursor=self.copy.get_or_insert_with(BmpImage::retained_clone_cursor);let step=cursor.advance(source,grant).map_err(ValueError::into_message)?;if matches!(step,RetainedCloneStep::Complete(_)){let next=cursor.take().ok_or("bmp: incoming image clone lost its owner")?;self.displaced_image=Some(std::mem::replace(&mut post.image,next));cursor.begin_close();self.phase=3;}step.progress()}else{self.phase=5;unit()}},
            4=>{
                if let Some(step)=self.retirement.begin_granted(&mut self.displaced_image,grant).map_err(ValueError::into_message)?{step.progress()}
                else{let step=self.retirement.step_granted(grant).map_err(ValueError::into_message)?;if matches!(step,RetainedCloneStep::Complete(_)){self.phase=5;}step.progress()}
            },
            5=>{let(done,progress)=self.validation.advance(if matches!(mutation.get(),BmpMutation::ReplaceImage(_)){post}else{base.get()},grant)?;if done{match mutation.get(){
                    BmpMutation::PaintIndexedRegion(value)=>{if self.validation.revision().as_deref()!=Some(&value.revision){return Err("bmp: source revision changed".into());}checked_region(&post.image,BmpRegion{x:value.x,y:value.y,width:value.width,height:value.height})?;if !post.image.profile.is_indexed()||usize::from(value.palette_index)>=post.image.palette.len(){return Err("bmp: indexed paint needs an owned palette entry".into());}self.phase=6;},
                    BmpMutation::PaintDirectRegion(value)=>{if self.validation.revision().as_deref()!=Some(&value.revision){return Err("bmp: source revision changed".into());}checked_region(&post.image,BmpRegion{x:value.x,y:value.y,width:value.width,height:value.height})?;if post.image.profile.is_indexed(){return Err("bmp: direct paint needs direct samples".into());}self.phase=6;},
                    BmpMutation::ReplaceSamples(value)=>{post.image.region_start(value.region).ok_or("bmp: sample rectangle exceeds the owned image or is empty")?;let count=value.region.width as usize*value.region.height as usize;let fits=if post.image.profile.is_indexed(){value.indices.len()==count&&value.samples.is_empty()}else{value.samples.len()==count&&value.indices.is_empty()};if !fits{return Err("bmp: sample rectangle storage or cardinality differs from its region".into());}self.phase=6;},
                    BmpMutation::ReplaceImage(_)=>self.phase=7,
                }}progress},
            6=>{
                let region=match mutation.get(){BmpMutation::PaintIndexedRegion(value)=>BmpRegion{x:value.x,y:value.y,width:value.width,height:value.height},BmpMutation::PaintDirectRegion(value)=>BmpRegion{x:value.x,y:value.y,width:value.width,height:value.height},BmpMutation::ReplaceSamples(value)=>value.region,_=>return Err("bmp: paint lost its typed intent".into())};
                if region.width==0||region.height==0{self.inverse=Some(Vec::new());self.phase=7;unit()}else{
                    let total=region.width as usize*region.height as usize;
                    let indexed=matches!(post.image.pixels,BmpPixels::Indexed{..});
                    let width=if indexed{1}else{std::mem::size_of::<BmpNativeSample>()};
                    if self.old.is_none(){
                        let planned=total.checked_mul(width).ok_or("bmp: inverse sample capacity overflow")?;
                        if grant.maximum_capacity_bytes<planned{return Ok(RetainedCloneEditStep::Progress(Default::default()));}
                        let mut old=BmpSampleRect{region,..Default::default()};
                        let actual=if indexed{old.indices.try_reserve_exact(total).map_err(|_|"bmp: inverse sample capacity allocation failed")?;old.indices.capacity()}else{old.samples.try_reserve_exact(total).map_err(|_|"bmp: inverse sample capacity allocation failed")?;old.samples.capacity()*width};
                        if actual>grant.maximum_capacity_bytes{self.old=Some(old);return Err("bmp: inverse sample allocator exceeded its admitted capacity".into());}
                        self.old=Some(old);
                        return Ok(RetainedCloneEditStep::Progress(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:actual,..Default::default()}));
                    }
                    let run=(total-self.pixel).min(bulk_run_elements(grant,width));
                    if run==0{return Ok(RetainedCloneEditStep::Progress(Default::default()));}
                    let old=self.old.as_mut().ok_or("bmp: inverse sample owner is absent")?;
                    for pixel in self.pixel..self.pixel+run{
                        let index=(region.y as usize+pixel/region.width as usize)*post.image.width as usize+region.x as usize+pixel%region.width as usize;
                        let incoming=match mutation.get(){BmpMutation::ReplaceSamples(value) if !value.samples.is_empty()=>{let next=value.samples.get(pixel).copied().ok_or("bmp: sample rectangle cardinality differs from its region")?;post.image.validate_direct_sample(&next)?;Some(next)},_=>None};
                        match(&mut post.image.pixels,mutation.get()){
                            (BmpPixels::Indexed{indices},BmpMutation::PaintIndexedRegion(value))=>{old.indices.push(indices[index]);self.changed|=indices[index]!=value.palette_index;indices[index]=value.palette_index;},
                            (BmpPixels::Indexed{indices},BmpMutation::ReplaceSamples(value))=>{let next=*value.indices.get(pixel).ok_or("bmp: sample rectangle cardinality differs from its region")?;if usize::from(next)>=post.image.palette.len(){return Err("bmp: sample references an absent palette entry".into());}old.indices.push(indices[index]);self.changed|=indices[index]!=next;indices[index]=next;},
                            (BmpPixels::Direct{samples},BmpMutation::PaintDirectRegion(value))=>{let masks=post.image.masks;let native=|value:u8,mask:u32|((u64::from(value)*u64::from(mask_maximum(mask))+127)/255)as u32;let sample=&mut samples[index];let before=*sample;sample.red=native(value.red,masks[0]);sample.green=native(value.green,masks[1]);sample.blue=native(value.blue,masks[2]);sample.alpha=native(value.alpha,masks[3]);old.samples.push(before);self.changed|=*sample!=before;},
                            (BmpPixels::Direct{samples},BmpMutation::ReplaceSamples(_))=>{let next=incoming.ok_or("bmp: sample rectangle storage differs from its region")?;let sample=&mut samples[index];old.samples.push(*sample);self.changed|=*sample!=next;*sample=next;},
                            _=>return Err("bmp: paint storage differs from its owned profile".into()),
                        }
                    }
                    self.pixel+=run;
                    if self.pixel==total{
                        self.inverse=Some(if self.changed{self.old.take().map(|rect|BmpMutation::ReplaceSamples(ReplaceSamples{region:rect.region,indices:rect.indices,samples:rect.samples})).into_iter().collect()}else{Vec::new()});
                        self.phase=7;
                    }
                    bulk_run_progress(run,width)
                }
            },
            _=>return Ok(RetainedCloneEditStep::Complete(unit())),
        };Ok(RetainedCloneEditStep::Progress(progress))
    })().map_err(|message|ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message))}
    fn foreign_step_presence(&self)->bool{false}
    fn take_inverse(&mut self)->Option<Vec<BmpMutation>>{if self.phase==7{self.inverse.take()}else{None}}
    fn cancel(&mut self){self.cancelled=true;self.begin_close();}
    fn begin_close(&mut self)->bool{if self.closing{return false;}self.closing=true;if let Some(copy)=self.copy.as_mut(){copy.begin_close();}true}
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if let Some(copy)=self.copy.as_mut(){let step=copy.close_step(grant)?;if matches!(step,RetainedCloneStep::Complete(_)){if !copy.terminal_is_empty(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"bmp: closed clone retained owner"));}self.copy=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}
        if !self.retirement.is_empty(){return self.retirement.step_granted(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
        if let Some(step)=self.retirement.begin_granted(&mut self.old,grant)?{return Ok(step);}
        if let Some(step)=self.retirement.begin_granted(&mut self.displaced_image,grant)?{return Ok(step);}
        if let Some(step)=self.retirement.begin_granted(&mut self.inverse_image,grant)?{return Ok(step);}
        if let Some(step)=self.retirement.begin_granted(&mut self.inverse,grant)?{return Ok(step);}
        Ok(RetainedCloneStep::Complete(Default::default()))
    }
    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{if let Some(copy)=self.copy.as_ref(){return copy.next_close_copy_byte_demand();}self.retirement.next_copy_byte_demand()}
    fn next_close_capacity_byte_demand(&self,copy_bytes:usize)->Result<usize,ValueError>{
        if let Some(copy)=self.copy.as_ref(){return copy.next_close_capacity_byte_demand(copy_bytes);}
        if !self.retirement.is_empty(){return self.retirement.next_capacity_byte_demand(copy_bytes);}
        if self.old.is_some(){return self.retirement.next_owner_capacity_byte_demand::<BmpSampleRect>(true,copy_bytes);}
        if self.displaced_image.is_some()||self.inverse_image.is_some(){return self.retirement.next_owner_capacity_byte_demand::<BmpImage>(true,copy_bytes);}
        self.retirement.next_owner_capacity_byte_demand::<Vec<BmpMutation>>(self.inverse.is_some(),copy_bytes)
    }
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{if let Some(copy)=self.copy.as_ref(){return copy.next_close_release_byte_demand();}self.retirement.next_release_byte_demand()}
    fn next_close_depth_demand(&self)->Result<usize,ValueError>{if let Some(copy)=self.copy.as_ref(){return copy.next_close_depth_demand();}if !self.retirement.is_empty(){return self.retirement.next_depth_demand();}Ok(usize::from(self.old.is_some()||self.displaced_image.is_some()||self.inverse_image.is_some()||self.inverse.is_some()))}

    fn terminal_is_empty(&self)->bool{self.closing&&self.copy.is_none()&&self.inverse_image.is_none()&&self.displaced_image.is_none()&&self.inverse.is_none()&&self.old.is_none()&&self.retirement.is_empty()}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
