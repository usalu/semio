//! 🧬️ Fuelled publication of exact owned BMP edits from retained mutation authority.
use crate::schema::{mutations::{BmpMutation,SetSnapshot},snapshot::{BmpPixels,BmpRegion,mask_maximum}};
use crate::BmpSnapshot;
use crate::schema::operations::{owned_validation::BmpOwnedValidationWork,checked_region};
use semio_framework_value::retained_clone::{RetainedClone,RetainedCloneCursor,RetainedCloneRef,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedCloneClose};
use semio_framework_value::{SnapshotRetirementStep,ValueError};
use store::snapshot_clone_preparation::{RetainedCloneEdit,RetainedCloneEditCursor,RetainedCloneEditStep,RetainedClonePreparationFactory};
use std::sync::Arc;

pub fn factory()->Arc<dyn store::ArtifactStoreOneItemPreparationFactory<BmpSnapshot,BmpMutation>> {
    Arc::new(RetainedClonePreparationFactory::new(Arc::new(BmpPublication),Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::default()),Arc::new(semio_framework_value::retirement::SharedValueRetirementFactory::default()),64).expect("BMP retained publication depth"))
}

pub struct BmpPublication;
impl RetainedCloneEdit<BmpSnapshot,BmpMutation> for BmpPublication {
    type Cursor=BmpPublicationCursor;
    fn preflight(&self,mutation:&BmpMutation,_lane:store::HistoryLane)->Result<store::ArtifactStoreOneItemFootprint,String> {
        if matches!(mutation,BmpMutation::PatchSnapshot(_)) {return Err("bmp: path patch requires typed retained field preparation".into());}
        Ok(store::ArtifactStoreOneItemFootprint::for_leaf::<BmpSnapshot,_>(mutation,store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }
    fn begin(&self)->Self::Cursor {BmpPublicationCursor::new()}
}

pub struct BmpPublicationCursor {
    phase:u8,
    copy:Option<<BmpSnapshot as RetainedClone>::Cursor>,
    inverse:Option<Vec<BmpMutation>>,
    retirement:RetainedCloneClose,
    validation:BmpOwnedValidationWork,
    pixel:usize,
    closing:bool,
    cancelled:bool,
}
impl BmpPublicationCursor {
    pub fn new()->Self {Self{phase:0,copy:Some(BmpSnapshot::retained_clone_cursor()),inverse:None,retirement:RetainedCloneClose::default(),validation:BmpOwnedValidationWork::default(),pixel:0,closing:false,cancelled:false}}
    fn close_copy(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,String> {
        let cursor=self.copy.as_mut().ok_or("bmp: missing copy cursor")?;
        let step=cursor.close_step(grant.maximum_items,grant.maximum_copy_bytes).map_err(ValueError::into_message)?;
        match step {
            SnapshotRetirementStep::Complete=>{if !cursor.terminal_is_empty(){return Err("bmp: copy close retained ownership".into());}self.copy=None;self.phase+=1;Ok(unit())},
            SnapshotRetirementStep::Pending{released_items,released_bytes}=>Ok(RetainedCloneProgress{copied_items:released_items,copied_bytes:released_bytes,retained_capacity_bytes:0}),
            SnapshotRetirementStep::Blocked=>Ok(RetainedCloneProgress::default()),
        }
    }
}
fn unit()->RetainedCloneProgress {RetainedCloneProgress{copied_items:1,..Default::default()}}
impl RetainedCloneEditCursor<BmpSnapshot,BmpMutation> for BmpPublicationCursor {
    fn advance(&mut self,base:RetainedCloneRef<'_,BmpSnapshot>,post:&mut BmpSnapshot,mutation:RetainedCloneRef<'_,BmpMutation>,grant:RetainedCloneGrant)->Result<RetainedCloneEditStep,String> {
        if self.closing||self.cancelled {return Err("bmp: publication cursor is closing".into());}
        if grant.maximum_items==0 {return Ok(RetainedCloneEditStep::Progress(Default::default()));}
        let progress=match self.phase {
            0=>{
                let cursor=self.copy.as_mut().ok_or("bmp: missing inverse copy")?;
                let step=cursor.advance(base,grant).map_err(ValueError::into_message)?;
                if matches!(step,RetainedCloneStep::Complete(_)) {
                    self.inverse=Some(vec![BmpMutation::SetSnapshot(SetSnapshot{snapshot:cursor.take().ok_or("bmp: inverse copy has no owner")?})]);
                    cursor.begin_close();self.phase=1;
                }
                step.progress()
            },
            1|3=>self.close_copy(grant)?,
            2=>{
                if !matches!(mutation.get(),BmpMutation::SetSnapshot(_)) {self.phase=5;unit()}else{
                    let source=mutation.project(1,|value|match value {BmpMutation::SetSnapshot(value)=>&value.snapshot,_=>unreachable!()});
                    let cursor=self.copy.get_or_insert_with(BmpSnapshot::retained_clone_cursor);
                    let step=cursor.advance(source,grant).map_err(ValueError::into_message)?;
                    if matches!(step,RetainedCloneStep::Complete(_)) {
                        let next=cursor.take().ok_or("bmp: result copy has no owner")?;
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
                let (done,progress)=if matches!(mutation.get(),BmpMutation::SetSnapshot(_)){self.validation.advance(post,grant)?}else{self.validation.advance(base.get(),grant)?};
                if done {match mutation.get() {
                    BmpMutation::PaintIndexedRegion(value)=>{
                        if self.validation.revision().as_deref()!=Some(&value.revision){return Err("bmp: source revision changed".into());}
                        checked_region(&post.image,BmpRegion{x:value.x,y:value.y,width:value.width,height:value.height})?;
                        if !post.image.profile.is_indexed()||usize::from(value.palette_index)>=post.image.palette.len(){return Err("bmp: indexed paint requires an owned palette entry".into());}self.phase=6;
                    },
                    BmpMutation::PaintDirectRegion(value)=>{
                        if self.validation.revision().as_deref()!=Some(&value.revision){return Err("bmp: source revision changed".into());}
                        checked_region(&post.image,BmpRegion{x:value.x,y:value.y,width:value.width,height:value.height})?;
                        if post.image.profile.is_indexed(){return Err("bmp: direct paint requires direct samples".into());}self.phase=6;
                    },
                    BmpMutation::PatchSnapshot(_)=>return Err("bmp: path patch has no retained typed field cursor".into()),
                    _=>self.phase=7,
                }}
                progress
            },
            6=>{
                let (x,y,width,height)=match mutation.get(){BmpMutation::PaintIndexedRegion(v)=>(v.x,v.y,v.width,v.height),BmpMutation::PaintDirectRegion(v)=>(v.x,v.y,v.width,v.height),_=>return Err("bmp: paint publication lost its mutation".into())};
                if width==0||height==0 {self.phase=7;unit()}else{
                    let index=(y as usize+self.pixel/width as usize)*post.image.width as usize+x as usize+self.pixel%width as usize;
                    let bytes=match (&mut post.image.pixels,mutation.get()) {
                        (BmpPixels::Indexed{indices},BmpMutation::PaintIndexedRegion(value))=>{if grant.maximum_copy_bytes<1{return Err("bmp: index publication requires a byte grant".into());}indices[index]=value.palette_index;1},
                        (BmpPixels::Direct{samples},BmpMutation::PaintDirectRegion(value))=>{
                            if grant.maximum_copy_bytes<16{return Err("bmp: component publication requires a sixteen byte grant".into());}
                            let sample=&mut samples[index];let color=[value.red,value.green,value.blue,value.alpha];let masks=post.image.masks;
                            let native=|value:u8,mask:u32|((u64::from(value)*u64::from(mask_maximum(mask))+127)/255)as u32;
                            sample.red=native(color[0],masks[0]);sample.green=native(color[1],masks[1]);sample.blue=native(color[2],masks[2]);sample.alpha=native(color[3],masks[3]);16
                        },
                        _=>return Err("bmp: paint storage differs from its owned profile".into()),
                    };
                    self.pixel+=1;if self.pixel==width as usize*height as usize {self.phase=7;}
                    RetainedCloneProgress{copied_items:1,copied_bytes:bytes,retained_capacity_bytes:0}
                }
            },
            _=>return Ok(RetainedCloneEditStep::Complete(unit())),
        };
        Ok(RetainedCloneEditStep::Progress(progress))
    }
    fn take_inverse(&mut self)->Option<Vec<BmpMutation>> {if self.phase>=7 {self.inverse.take()}else{None}}
    fn cancel(&mut self){self.cancelled=true;self.begin_close();}
    fn begin_close(&mut self)->bool {if self.closing{return false;}self.closing=true;if let Some(copy)=self.copy.as_mut(){copy.begin_close();}true}
    fn close_step(&mut self,maximum_items:usize,maximum_bytes:usize)->Result<SnapshotRetirementStep,ValueError> {
        if maximum_items==0{return Ok(SnapshotRetirementStep::Pending{released_items:0,released_bytes:0});}
        if let Some(copy)=self.copy.as_mut(){let step=copy.close_step(maximum_items,maximum_bytes)?;if step==SnapshotRetirementStep::Complete {if !copy.terminal_is_empty(){return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"bmp: copy retained owner after close"));}self.copy=None;return Ok(SnapshotRetirementStep::Pending{released_items:1,released_bytes:0});}return Ok(step);}
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
