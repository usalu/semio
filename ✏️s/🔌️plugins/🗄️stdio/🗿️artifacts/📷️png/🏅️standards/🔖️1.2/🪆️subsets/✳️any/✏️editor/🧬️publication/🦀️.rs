//! 🧬️ Fuelled publication of exact owned PNG edits from retained mutation authority.
use crate::schema::{mutations::{PngMutation,SetSnapshot},snapshot::{PngColorType,PngRegion}};
use crate::PngSnapshot;
use crate::schema::operations::{owned_validation::PngOwnedValidationWork,validate_native_paint_target};
use semio_framework_value::retained_clone::{RetainedClone,RetainedCloneCursor,RetainedCloneRef,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedCloneClose};
use semio_framework_value::{SnapshotRetirementStep,ValueError};
use store::snapshot_clone_preparation::{RetainedCloneEdit,RetainedCloneEditCursor,RetainedCloneEditStep,RetainedClonePreparationFactory};
use std::sync::Arc;

pub fn factory()->Arc<dyn store::ArtifactStoreOneItemPreparationFactory<PngSnapshot,PngMutation>> {
    Arc::new(RetainedClonePreparationFactory::new(Arc::new(PngPublication),Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::default()),Arc::new(semio_framework_value::retirement::SharedValueRetirementFactory::default()),64).expect("PNG retained publication depth"))
}

pub struct PngPublication;
impl RetainedCloneEdit<PngSnapshot,PngMutation> for PngPublication {
    type Cursor=PngPublicationCursor;
    fn preflight(&self,mutation:&PngMutation,_lane:store::HistoryLane)->Result<store::ArtifactStoreOneItemFootprint,String> {
        if matches!(mutation,PngMutation::PatchSnapshot(_)) {return Err("png: path patch requires typed retained field preparation".into());}
        Ok(store::ArtifactStoreOneItemFootprint::for_leaf::<PngSnapshot,_>(mutation,store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }
    fn begin(&self)->Self::Cursor {PngPublicationCursor::new()}
}

pub struct PngPublicationCursor {
    phase:u8,
    copy:Option<<PngSnapshot as RetainedClone>::Cursor>,
    inverse:Option<Vec<PngMutation>>,
    retirement:RetainedCloneClose,
    validation:PngOwnedValidationWork,
    pixel:usize,
    closing:bool,
    cancelled:bool,
}
impl PngPublicationCursor {
    pub fn new()->Self {Self{phase:0,copy:Some(PngSnapshot::retained_clone_cursor()),inverse:None,retirement:RetainedCloneClose::default(),validation:PngOwnedValidationWork::default(),pixel:0,closing:false,cancelled:false}}
    fn close_copy(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,String> {
        let cursor=self.copy.as_mut().ok_or("png: missing copy cursor")?;
        let step=cursor.close_step(grant.maximum_items,grant.maximum_copy_bytes).map_err(ValueError::into_message)?;
        match step {
            SnapshotRetirementStep::Complete=>{if !cursor.terminal_is_empty(){return Err("png: copy close retained ownership".into());}self.copy=None;self.phase+=1;Ok(unit())},
            SnapshotRetirementStep::Pending{released_items,released_bytes}=>Ok(RetainedCloneProgress{copied_items:released_items,copied_bytes:released_bytes,retained_capacity_bytes:0}),
            SnapshotRetirementStep::Blocked=>Ok(RetainedCloneProgress::default()),
        }
    }
}
fn unit()->RetainedCloneProgress {RetainedCloneProgress{copied_items:1,..Default::default()}}
impl RetainedCloneEditCursor<PngSnapshot,PngMutation> for PngPublicationCursor {
    fn advance(&mut self,base:RetainedCloneRef<'_,PngSnapshot>,post:&mut PngSnapshot,mutation:RetainedCloneRef<'_,PngMutation>,grant:RetainedCloneGrant)->Result<RetainedCloneEditStep,String> {
        if self.closing||self.cancelled {return Err("png: publication cursor is closing".into());}
        if grant.maximum_items==0 {return Ok(RetainedCloneEditStep::Progress(Default::default()));}
        let progress=match self.phase {
            0=>{
                let cursor=self.copy.as_mut().ok_or("png: missing inverse copy")?;
                let step=cursor.advance(base,grant).map_err(ValueError::into_message)?;
                if matches!(step,RetainedCloneStep::Complete(_)) {
                    self.inverse=Some(vec![PngMutation::SetSnapshot(SetSnapshot{snapshot:cursor.take().ok_or("png: inverse copy has no owner")?})]);
                    cursor.begin_close();self.phase=1;
                }
                step.progress()
            },
            1|3=>self.close_copy(grant)?,
            2=>{
                if !matches!(mutation.get(),PngMutation::SetSnapshot(_)|PngMutation::PaintNativeSamples(_)) {self.phase=5;unit()}else{
                    let source=mutation.project(1,|value|match value {PngMutation::SetSnapshot(value)=>&value.snapshot,PngMutation::PaintNativeSamples(value)=>&value.result,_=>unreachable!()});
                    let cursor=self.copy.get_or_insert_with(PngSnapshot::retained_clone_cursor);
                    let step=cursor.advance(source,grant).map_err(ValueError::into_message)?;
                    if matches!(step,RetainedCloneStep::Complete(_)) {
                        let next=cursor.take().ok_or("png: result copy has no owner")?;
                        self.retirement.begin(std::mem::replace(post,next)).map_err(ValueError::into_message)?;
                        cursor.begin_close();self.phase=3;
                    }
                    step.progress()
                }
            },
            4=>{
                match self.retirement.step(grant.maximum_items,grant.maximum_copy_bytes).map_err(ValueError::into_message)? {
                    SnapshotRetirementStep::Complete=>{self.phase=5;unit()},
                    SnapshotRetirementStep::Pending{released_items,released_bytes}=>RetainedCloneProgress{copied_items:released_items,copied_bytes:released_bytes,retained_capacity_bytes:0},
                    SnapshotRetirementStep::Blocked=>Default::default(),
                }
            },
            5=>{
                let (done,progress)=match mutation.get() {
                    PngMutation::PaintNativeSamples(value)=>self.validation.advance(base.get(),Some((post,value.region,value.paint,&value.revision)),grant)?,
                    PngMutation::SetSnapshot(_)=>self.validation.advance(post,None,grant)?,
                    _=>self.validation.advance(base.get(),None,grant)?,
                };
                if done {match mutation.get() {
                    PngMutation::ChangeGamma(value)=>{if self.validation.revision().as_deref()!=Some(&value.revision){return Err("png: source revision changed".into());}if value.gama==Some(0){return Err("png: gamma must be positive".into());}post.image.gamma=value.gama;self.phase=7;},
                    PngMutation::PatchPixels(value)=>{
                        if self.validation.revision().as_deref()!=Some(&value.revision){return Err("png: source revision changed".into());}
                        if post.image.color_type!=PngColorType::Rgba||post.image.bit_depth!=8{return Err("png: pixel patches need an 8-bit RGBA image".into());}
                        validate_native_paint_target(post,PngRegion{x:value.x,y:value.y,width:value.width,height:value.height},crate::schema::snapshot::PngNativePaint::rgba(value.red.into(),value.green.into(),value.blue.into(),value.alpha.into()))?;
                        self.phase=6;
                    },
                    PngMutation::PatchSnapshot(_)=>return Err("png: path patch has no retained typed field cursor".into()),
                    _=>self.phase=7,
                }}
                progress
            },
            6=>{
                let PngMutation::PatchPixels(value)=mutation.get() else{return Err("png: pixel publication lost its mutation".into())};
                if grant.maximum_copy_bytes<8 {return Err("png: pixel publication requires an eight byte copy grant".into());}
                let x=value.x as usize+self.pixel%value.width as usize;let y=value.y as usize+self.pixel/value.width as usize;
                let start=(y*post.image.width as usize+x)*4;
                post.image.samples[start..start+4].copy_from_slice(&[value.red.into(),value.green.into(),value.blue.into(),value.alpha.into()]);self.pixel+=1;
                if self.pixel==value.width as usize*value.height as usize {self.phase=7;}
                RetainedCloneProgress{copied_items:1,copied_bytes:8,retained_capacity_bytes:0}
            },
            _=>return Ok(RetainedCloneEditStep::Complete(unit())),
        };
        Ok(RetainedCloneEditStep::Progress(progress))
    }
    fn take_inverse(&mut self)->Option<Vec<PngMutation>> {if self.phase>=7 {self.inverse.take()}else{None}}
    fn cancel(&mut self){self.cancelled=true;self.begin_close();}
    fn begin_close(&mut self)->bool {if self.closing{return false;}self.closing=true;if let Some(copy)=self.copy.as_mut(){copy.begin_close();}true}
    fn close_step(&mut self,maximum_items:usize,maximum_bytes:usize)->Result<SnapshotRetirementStep,ValueError> {
        if maximum_items==0{return Ok(SnapshotRetirementStep::Pending{released_items:0,released_bytes:0});}
        if let Some(copy)=self.copy.as_mut(){let step=copy.close_step(maximum_items,maximum_bytes)?;if step==SnapshotRetirementStep::Complete {if !copy.terminal_is_empty(){return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"png: copy retained owner after close"));}self.copy=None;return Ok(SnapshotRetirementStep::Pending{released_items:1,released_bytes:0});}return Ok(step);}
        if !self.retirement.is_empty(){return self.retirement.step(maximum_items,maximum_bytes);}
        if let Some(inverse)=self.inverse.take(){self.retirement.begin(inverse)?;return Ok(SnapshotRetirementStep::Pending{released_items:1,released_bytes:0});}
        Ok(SnapshotRetirementStep::Complete)
    }
    fn terminal_is_empty(&self)->bool {self.closing&&self.copy.is_none()&&self.inverse.is_none()&&self.retirement.is_empty()}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

#[path="🔏️canonical/🦀️.rs"]
mod canonical;
