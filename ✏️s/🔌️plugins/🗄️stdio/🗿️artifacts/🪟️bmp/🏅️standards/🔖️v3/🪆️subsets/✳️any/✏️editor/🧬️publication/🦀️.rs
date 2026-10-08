//! 🧬️ Fuelled publication and exact native image inverse intent.
use crate::{BmpSnapshot,schema::{mutations::{BmpMutation,ReplaceImage},snapshot::BmpImage}};
use crate::schema::operations::{owned_validation::BmpOwnedValidationWork,checked_region};
use crate::schema::snapshot::{BmpPixels,BmpRegion,mask_maximum};
use semio_framework_value::retained_clone::{RetainedClone,RetainedCloneCursor,RetainedCloneRef,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedCloneClose};
use semio_framework_value::{SnapshotRetirementStep,ValueError};
use store::snapshot_clone_preparation::{RetainedCloneEdit,RetainedCloneEditCursor,RetainedCloneEditStep,RetainedClonePreparationFactory};
use std::sync::Arc;

pub fn factory()->Arc<dyn store::ArtifactStoreOneItemPreparationFactory<BmpSnapshot,BmpMutation>>{Arc::new(RetainedClonePreparationFactory::new(Arc::new(BmpPublication),Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::default()),Arc::new(semio_framework_value::retirement::SharedValueRetirementFactory::default()),64).expect("bmp retained publication depth"))}
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct BmpPublication;
impl RetainedCloneEdit<BmpSnapshot,BmpMutation> for BmpPublication{
    type Cursor=BmpPublicationCursor;
    fn preflight(&self,mutation:&BmpMutation,_lane:store::HistoryLane)->Result<store::ArtifactStoreOneItemFootprint,String>{Ok(store::ArtifactStoreOneItemFootprint::for_leaf::<BmpSnapshot,_>(mutation,store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))}
    fn begin(&self)->Self::Cursor{BmpPublicationCursor::new()}
}
pub struct BmpPublicationCursor{
    phase:u8,copy:Option<<BmpImage as RetainedClone>::Cursor>,inverse_image:Option<BmpImage>,inverse:Option<Vec<BmpMutation>>,retirement:RetainedCloneClose,validation:BmpOwnedValidationWork,pixel:usize,closing:bool,cancelled:bool,
}
impl BmpPublicationCursor{
    pub fn new()->Self{Self{phase:0,copy:Some(BmpImage::retained_clone_cursor()),inverse_image:None,inverse:None,retirement:RetainedCloneClose::default(),validation:BmpOwnedValidationWork::default(),pixel:0,closing:false,cancelled:false}}
    fn close_copy(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,String>{
        let cursor=self.copy.as_mut().ok_or("bmp: missing image clone cursor")?;
        match cursor.close_step(grant.maximum_items,grant.maximum_release_bytes).map_err(ValueError::into_message)?{
            SnapshotRetirementStep::Complete=>{if !cursor.terminal_is_empty(){return Err("bmp: image clone close retained ownership".into());}self.copy=None;self.phase=if self.phase==1{8}else{4};Ok(unit())},
            SnapshotRetirementStep::Pending{released_items,released_bytes}=>Ok(RetainedCloneProgress{copied_items:released_items,released_bytes,..Default::default()}),SnapshotRetirementStep::Blocked=>Ok(Default::default()),
        }
    }
}
fn unit()->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,..Default::default()}}
impl RetainedCloneEditCursor<BmpSnapshot,BmpMutation> for BmpPublicationCursor{
    fn advance(&mut self,base:RetainedCloneRef<'_,BmpSnapshot>,post:&mut BmpSnapshot,mutation:RetainedCloneRef<'_,BmpMutation>,grant:RetainedCloneGrant)->Result<RetainedCloneEditStep,String>{
        if self.closing||self.cancelled{return Err("bmp: publication cursor is closing".into());}if grant.maximum_items==0{return Ok(RetainedCloneEditStep::Progress(Default::default()));}
        let progress=match self.phase{
            0=>{let cursor=self.copy.as_mut().ok_or("bmp: missing inverse image clone")?;let step=cursor.advance(base.project(1,|value|&value.image),grant).map_err(ValueError::into_message)?;if matches!(step,RetainedCloneStep::Complete(_)){self.inverse_image=Some(cursor.take().ok_or("bmp: inverse image clone lost its owner")?);cursor.begin_close();self.phase=1;}step.progress()},
            1|3=>self.close_copy(grant)?,
            8=>{let bytes=std::mem::size_of::<BmpMutation>();if grant.maximum_capacity_bytes<bytes{return Ok(RetainedCloneEditStep::Progress(Default::default()));}let mut inverse=Vec::with_capacity(1);inverse.push(BmpMutation::ReplaceImage(ReplaceImage{image:self.inverse_image.take().ok_or("bmp: inverse image owner is absent")?}));self.inverse=Some(inverse);self.phase=2;RetainedCloneProgress{copied_items:1,retained_capacity_bytes:bytes,..Default::default()}},
            2=>{if let BmpMutation::ReplaceImage(_)=mutation.get(){let source=mutation.project(1,|value|match value{BmpMutation::ReplaceImage(value)=>&value.image,_=>unreachable!()});let cursor=self.copy.get_or_insert_with(BmpImage::retained_clone_cursor);let step=cursor.advance(source,grant).map_err(ValueError::into_message)?;if matches!(step,RetainedCloneStep::Complete(_)){let next=cursor.take().ok_or("bmp: incoming image clone lost its owner")?;self.retirement.begin(std::mem::replace(&mut post.image,next)).map_err(ValueError::into_message)?;cursor.begin_close();self.phase=3;}step.progress()}else{self.phase=5;unit()}},
            4=>{match self.retirement.step(grant.maximum_items,grant.maximum_release_bytes).map_err(ValueError::into_message)?{SnapshotRetirementStep::Complete=>{self.phase=5;unit()},SnapshotRetirementStep::Pending{released_items,released_bytes}=>RetainedCloneProgress{copied_items:released_items,released_bytes,..Default::default()},SnapshotRetirementStep::Blocked=>Default::default()}},
            5=>{let(done,progress)=self.validation.advance(if matches!(mutation.get(),BmpMutation::ReplaceImage(_)){post}else{base.get()},grant)?;if done{match mutation.get(){
                    BmpMutation::PaintIndexedRegion(value)=>{if self.validation.revision().as_deref()!=Some(&value.revision){return Err("bmp: source revision changed".into());}checked_region(&post.image,BmpRegion{x:value.x,y:value.y,width:value.width,height:value.height})?;if !post.image.profile.is_indexed()||usize::from(value.palette_index)>=post.image.palette.len(){return Err("bmp: indexed paint needs an owned palette entry".into());}self.phase=6;},
                    BmpMutation::PaintDirectRegion(value)=>{if self.validation.revision().as_deref()!=Some(&value.revision){return Err("bmp: source revision changed".into());}checked_region(&post.image,BmpRegion{x:value.x,y:value.y,width:value.width,height:value.height})?;if post.image.profile.is_indexed(){return Err("bmp: direct paint needs direct samples".into());}self.phase=6;},
                    BmpMutation::ReplaceImage(_)=>self.phase=7,
                }}progress},
            6=>{
                let(x,y,width,height)=match mutation.get(){BmpMutation::PaintIndexedRegion(value)=>(value.x,value.y,value.width,value.height),BmpMutation::PaintDirectRegion(value)=>(value.x,value.y,value.width,value.height),_=>return Err("bmp: paint lost its typed intent".into())};
                if width==0||height==0{self.phase=7;unit()}else{
                    let index=(y as usize+self.pixel/width as usize)*post.image.width as usize+x as usize+self.pixel%width as usize;
                    let bytes=if matches!(mutation.get(),BmpMutation::PaintIndexedRegion(_)){1}else{16};
                    if grant.maximum_copy_bytes<bytes{return Ok(RetainedCloneEditStep::Progress(Default::default()));}
                    match(&mut post.image.pixels,mutation.get()){
                        (BmpPixels::Indexed{indices},BmpMutation::PaintIndexedRegion(value))=>indices[index]=value.palette_index,
                        (BmpPixels::Direct{samples},BmpMutation::PaintDirectRegion(value))=>{let sample=&mut samples[index];let masks=post.image.masks;let native=|value:u8,mask:u32|((u64::from(value)*u64::from(mask_maximum(mask))+127)/255)as u32;sample.red=native(value.red,masks[0]);sample.green=native(value.green,masks[1]);sample.blue=native(value.blue,masks[2]);sample.alpha=native(value.alpha,masks[3]);},
                        _=>return Err("bmp: paint storage differs from its owned profile".into()),
                    }
                    self.pixel+=1;if self.pixel==width as usize*height as usize{self.phase=7;}RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}
                }
            },
            _=>return Ok(RetainedCloneEditStep::Complete(unit())),
        };Ok(RetainedCloneEditStep::Progress(progress))
    }
    fn take_inverse(&mut self)->Option<Vec<BmpMutation>>{if self.phase==7{self.inverse.take()}else{None}}
    fn cancel(&mut self){self.cancelled=true;self.begin_close();}
    fn begin_close(&mut self)->bool{if self.closing{return false;}self.closing=true;if let Some(copy)=self.copy.as_mut(){copy.begin_close();}true}
    fn close_step(&mut self,maximum_items:usize,maximum_bytes:usize)->Result<SnapshotRetirementStep,ValueError>{
        if maximum_items==0{return Ok(SnapshotRetirementStep::Pending{released_items:0,released_bytes:0});}
        if let Some(copy)=self.copy.as_mut(){let step=copy.close_step(maximum_items,maximum_bytes)?;if step==SnapshotRetirementStep::Complete{if !copy.terminal_is_empty(){return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"bmp: closed clone retained owner"));}self.copy=None;return Ok(SnapshotRetirementStep::Pending{released_items:1,released_bytes:0});}return Ok(step);}
        if !self.retirement.is_empty(){return self.retirement.step(maximum_items,maximum_bytes);}
        if let Some(image)=self.inverse_image.take(){self.retirement.begin(image)?;return Ok(SnapshotRetirementStep::Pending{released_items:1,released_bytes:0});}
        if let Some(inverse)=self.inverse.take(){self.retirement.begin(inverse)?;return Ok(SnapshotRetirementStep::Pending{released_items:1,released_bytes:0});}Ok(SnapshotRetirementStep::Complete)
    }
    fn terminal_is_empty(&self)->bool{self.closing&&self.copy.is_none()&&self.inverse_image.is_none()&&self.inverse.is_none()&&self.retirement.is_empty()}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
