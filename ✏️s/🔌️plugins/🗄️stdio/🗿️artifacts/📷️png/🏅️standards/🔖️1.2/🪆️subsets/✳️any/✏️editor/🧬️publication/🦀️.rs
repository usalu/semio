//! 🧬️ Fuelled publication and exact native image inverse intent.
use crate::{PngSnapshot,schema::{mutations::{PngMutation,ReplaceImage},snapshot::PngImage}};
use crate::schema::operations::{owned_validation::PngOwnedValidationWork,validate_native_paint_target};
use crate::schema::snapshot::{PngColorType,PngRegion};
use semio_framework_value::retained_clone::{RetainedClone,RetainedCloneCursor,RetainedCloneRef,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedCloneClose};
use semio_framework_value::{SnapshotRetirementStep,ValueError};
use store::snapshot_clone_preparation::{RetainedCloneEdit,RetainedCloneEditCursor,RetainedCloneEditStep,RetainedClonePreparationFactory};
use std::sync::Arc;

pub fn factory()->Arc<dyn store::ArtifactStoreOneItemPreparationFactory<PngSnapshot,PngMutation>>{Arc::new(RetainedClonePreparationFactory::new(Arc::new(PngPublication),Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::default()),Arc::new(semio_framework_value::retirement::SharedValueRetirementFactory::default()),64).expect("png retained publication depth"))}
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct PngPublication;
impl RetainedCloneEdit<PngSnapshot,PngMutation> for PngPublication{
    type Cursor=PngPublicationCursor;
    fn preflight(&self,mutation:&PngMutation,_lane:store::HistoryLane)->Result<store::ArtifactStoreOneItemFootprint,String>{Ok(store::ArtifactStoreOneItemFootprint::for_leaf::<PngSnapshot,_>(mutation,store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))}
    fn begin(&self)->Self::Cursor{PngPublicationCursor::new()}
}
pub struct PngPublicationCursor{
    phase:u8,copy:Option<<PngImage as RetainedClone>::Cursor>,inverse_image:Option<PngImage>,inverse:Option<Vec<PngMutation>>,retirement:RetainedCloneClose,validation:PngOwnedValidationWork,pixel:usize,closing:bool,cancelled:bool,
}
impl PngPublicationCursor{
    pub fn new()->Self{Self{phase:0,copy:Some(PngImage::retained_clone_cursor()),inverse_image:None,inverse:None,retirement:RetainedCloneClose::default(),validation:PngOwnedValidationWork::default(),pixel:0,closing:false,cancelled:false}}
    fn close_copy(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,String>{
        let cursor=self.copy.as_mut().ok_or("png: missing image clone cursor")?;
        match cursor.close_step(grant.maximum_items,grant.maximum_release_bytes).map_err(ValueError::into_message)?{
            SnapshotRetirementStep::Complete=>{if !cursor.terminal_is_empty(){return Err("png: image clone close retained ownership".into());}self.copy=None;self.phase=if self.phase==1{8}else{4};Ok(unit())},
            SnapshotRetirementStep::Pending{released_items,released_bytes}=>Ok(RetainedCloneProgress{copied_items:released_items,released_bytes,..Default::default()}),SnapshotRetirementStep::Blocked=>Ok(Default::default()),
        }
    }
}
fn unit()->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,..Default::default()}}
impl RetainedCloneEditCursor<PngSnapshot,PngMutation> for PngPublicationCursor{
    fn advance(&mut self,base:RetainedCloneRef<'_,PngSnapshot>,post:&mut PngSnapshot,mutation:RetainedCloneRef<'_,PngMutation>,grant:RetainedCloneGrant)->Result<RetainedCloneEditStep,String>{
        if self.closing||self.cancelled{return Err("png: publication cursor is closing".into());}if grant.maximum_items==0{return Ok(RetainedCloneEditStep::Progress(Default::default()));}
        let progress=match self.phase{
            0=>{let cursor=self.copy.as_mut().ok_or("png: missing inverse image clone")?;let step=cursor.advance(base.project(1,|value|&value.image),grant).map_err(ValueError::into_message)?;if matches!(step,RetainedCloneStep::Complete(_)){self.inverse_image=Some(cursor.take().ok_or("png: inverse image clone lost its owner")?);cursor.begin_close();self.phase=1;}step.progress()},
            1|3=>self.close_copy(grant)?,
            8=>{let bytes=std::mem::size_of::<PngMutation>();if grant.maximum_capacity_bytes<bytes{return Ok(RetainedCloneEditStep::Progress(Default::default()));}let mut inverse=Vec::with_capacity(1);inverse.push(PngMutation::ReplaceImage(ReplaceImage{image:self.inverse_image.take().ok_or("png: inverse image owner is absent")?}));self.inverse=Some(inverse);self.phase=2;RetainedCloneProgress{copied_items:1,retained_capacity_bytes:bytes,..Default::default()}},
            2=>{if let PngMutation::ReplaceImage(_)=mutation.get(){let source=mutation.project(1,|value|match value{PngMutation::ReplaceImage(value)=>&value.image,_=>unreachable!()});let cursor=self.copy.get_or_insert_with(PngImage::retained_clone_cursor);let step=cursor.advance(source,grant).map_err(ValueError::into_message)?;if matches!(step,RetainedCloneStep::Complete(_)){let next=cursor.take().ok_or("png: incoming image clone lost its owner")?;self.retirement.begin(std::mem::replace(&mut post.image,next)).map_err(ValueError::into_message)?;cursor.begin_close();self.phase=3;}step.progress()}else{self.phase=5;unit()}},
            4=>{match self.retirement.step(grant.maximum_items,grant.maximum_release_bytes).map_err(ValueError::into_message)?{SnapshotRetirementStep::Complete=>{self.phase=5;unit()},SnapshotRetirementStep::Pending{released_items,released_bytes}=>RetainedCloneProgress{copied_items:released_items,released_bytes,..Default::default()},SnapshotRetirementStep::Blocked=>Default::default()}},
            5=>{let(done,progress)=self.validation.advance(if matches!(mutation.get(),PngMutation::ReplaceImage(_)){post}else{base.get()},None,grant)?;if done{match mutation.get(){
                    PngMutation::ChangeGamma(value)=>{if self.validation.revision().as_deref()!=Some(&value.revision){return Err("png: source revision changed".into());}if value.gama==Some(0){return Err("png: gamma must be positive".into());}self.phase=6;},
                    PngMutation::PaintNativeSamples(value)=>{if self.validation.revision().as_deref()!=Some(&value.revision){return Err("png: source revision changed".into());}validate_native_paint_target(post,value.region,value.paint)?;self.phase=6;},
                    PngMutation::PatchPixels(value)=>{if self.validation.revision().as_deref()!=Some(&value.revision){return Err("png: source revision changed".into());}if post.image.color_type!=PngColorType::Rgba||post.image.bit_depth!=8{return Err("png: pixel patches need RGBA8".into());}validate_native_paint_target(post,PngRegion{x:value.x,y:value.y,width:value.width,height:value.height},crate::schema::snapshot::PngNativePaint::rgba(value.red.into(),value.green.into(),value.blue.into(),value.alpha.into()))?;self.phase=6;},
                    PngMutation::ReplaceImage(_)=>self.phase=7,
                }}progress},
            6=>{
                if let PngMutation::ChangeGamma(value)=mutation.get(){let bytes=std::mem::size_of::<Option<u32>>();if grant.maximum_copy_bytes<bytes{return Ok(RetainedCloneEditStep::Progress(Default::default()));}post.image.gamma=value.gama;self.phase=7;return Ok(RetainedCloneEditStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}));}
                let(region,paint)=match mutation.get(){PngMutation::PatchPixels(value)=>(PngRegion{x:value.x,y:value.y,width:value.width,height:value.height},crate::schema::snapshot::PngNativePaint::rgba(value.red.into(),value.green.into(),value.blue.into(),value.alpha.into())),PngMutation::PaintNativeSamples(value)=>(value.region,value.paint),_=>return Err("png: paint lost its typed intent".into())};
                let spp=post.image.color_type.samples_per_pixel();let bytes=spp*std::mem::size_of::<u16>();
                if grant.maximum_copy_bytes<bytes{return Ok(RetainedCloneEditStep::Progress(Default::default()));}
                let start=((region.y as usize+self.pixel/region.width as usize)*post.image.width as usize+region.x as usize+self.pixel%region.width as usize)*spp;
                post.image.samples[start..start+spp].copy_from_slice(&paint.samples()[..spp]);self.pixel+=1;
                if self.pixel==region.width as usize*region.height as usize{self.phase=7;}
                RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}
            },
            _=>return Ok(RetainedCloneEditStep::Complete(unit())),
        };Ok(RetainedCloneEditStep::Progress(progress))
    }
    fn take_inverse(&mut self)->Option<Vec<PngMutation>>{if self.phase==7{self.inverse.take()}else{None}}
    fn cancel(&mut self){self.cancelled=true;self.begin_close();}
    fn begin_close(&mut self)->bool{if self.closing{return false;}self.closing=true;if let Some(copy)=self.copy.as_mut(){copy.begin_close();}true}
    fn close_step(&mut self,maximum_items:usize,maximum_bytes:usize)->Result<SnapshotRetirementStep,ValueError>{
        if maximum_items==0{return Ok(SnapshotRetirementStep::Pending{released_items:0,released_bytes:0});}
        if let Some(copy)=self.copy.as_mut(){let step=copy.close_step(maximum_items,maximum_bytes)?;if step==SnapshotRetirementStep::Complete{if !copy.terminal_is_empty(){return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"png: closed clone retained owner"));}self.copy=None;return Ok(SnapshotRetirementStep::Pending{released_items:1,released_bytes:0});}return Ok(step);}
        if !self.retirement.is_empty(){return self.retirement.step(maximum_items,maximum_bytes);}
        if let Some(image)=self.inverse_image.take(){self.retirement.begin(image)?;return Ok(SnapshotRetirementStep::Pending{released_items:1,released_bytes:0});}
        if let Some(inverse)=self.inverse.take(){self.retirement.begin(inverse)?;return Ok(SnapshotRetirementStep::Pending{released_items:1,released_bytes:0});}Ok(SnapshotRetirementStep::Complete)
    }
    fn terminal_is_empty(&self)->bool{self.closing&&self.copy.is_none()&&self.inverse_image.is_none()&&self.inverse.is_none()&&self.retirement.is_empty()}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
