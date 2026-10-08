//! 🧰️ Owns one genuine native preparation controller under exact birth and terminal grants.

use semio_framework_value::{SnapshotRetirementStep,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::mem::size_of;

pub(crate) trait Puzzle2dNativePreparation: Default {
    fn begin_close(&mut self);
    fn close_granted(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
    fn terminal_is_empty(&self)->bool;
}

pub(crate) struct Puzzle2dPreparationChild<C:Puzzle2dNativePreparation> { owner:Option<Box<C>>, closing:bool }

impl<C:Puzzle2dNativePreparation> Default for Puzzle2dPreparationChild<C> {
    fn default()->Self { Self {owner:None,closing:false} }
}

impl<C:Puzzle2dNativePreparation> Puzzle2dPreparationChild<C> {
    pub(crate) fn ensure(&mut self,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,ValueError> {
        if self.closing {return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"native preparation child is closing"));}
        if self.owner.is_some() {return Ok(None);}
        let bytes=size_of::<C>();
        if bytes>4096 {return Err(ValueError::new(ValueRefusalKind::WorkLimit,"native preparation child exceeds one exact capacity turn"));}
        if grant.maximum_items==0 || grant.maximum_capacity_bytes < bytes {return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
        self.owner=Some(Box::new(C::default()));
        Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,retained_capacity_bytes:bytes,..Default::default()})))
    }

    pub(crate) fn owner_mut(&mut self)->&mut C {self.owner.as_mut().expect("native preparation child birth was admitted")}

    pub(crate) fn begin_close(&mut self) {self.closing=true;if let Some(owner)=self.owner.as_mut(){owner.begin_close();}}

    pub(crate) fn close_granted(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        if !self.closing {return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"native preparation child must begin close"));}
        if grant.maximum_items==0 {return Ok(RetainedCloneStep::Progress(Default::default()));}
        let Some(owner)=self.owner.as_mut() else {return Ok(RetainedCloneStep::Complete(Default::default()));};
        if !owner.terminal_is_empty() {let step=owner.close_granted(grant)?;return Ok(RetainedCloneStep::Progress(step.progress()));}
        let bytes=size_of::<C>();
        if grant.maximum_release_bytes < bytes {return Ok(RetainedCloneStep::Progress(Default::default()));}
        drop(self.owner.take());
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress {copied_items:1,released_bytes:bytes,..Default::default()}))
    }

    pub(crate) fn terminal_is_empty(&self)->bool {self.owner.is_none()}
}

impl<C:Puzzle2dNativePreparation> Drop for Puzzle2dPreparationChild<C> {
    fn drop(&mut self) {assert!(std::thread::panicking() || self.owner.is_none(),"native preparation child dropped before exact terminal release");}
}

fn comparison_close(step:SnapshotRetirementStep)->RetainedCloneStep {
    RetainedCloneStep::Progress(match step {SnapshotRetirementStep::Complete | SnapshotRetirementStep::Blocked=>Default::default(),SnapshotRetirementStep::Pending {released_items,released_bytes}=>RetainedCloneProgress {copied_items:released_items,released_bytes,..Default::default()}})
}

macro_rules! borrowed_preparation {
    ($cursor:ty)=>{
        impl Puzzle2dNativePreparation for $cursor {
            fn begin_close(&mut self){<$cursor>::begin_close(self);}
            fn close_granted(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{<$cursor>::close_step(self,grant.maximum_items.min(1),grant.maximum_release_bytes).map(comparison_close)}
            fn terminal_is_empty(&self)->bool{<$cursor>::terminal_is_empty(self)}
        }
    };
}

borrowed_preparation!(crate::standards::v1::subsets::any::schema::mutations::connect_handles::prepare::Puzzle2dConnectHandlesPreparationCursor);
borrowed_preparation!(crate::standards::v1::subsets::any::schema::mutations::delete_node::preparation::Puzzle2dDeleteNodePreparationCursor);

impl Puzzle2dNativePreparation for crate::standards::v1::subsets::any::schema::mutations::connect_handles::prepare::edge::Puzzle2dConnectEdgeCursor {
    fn begin_close(&mut self){Self::begin_close(self);}
    fn close_granted(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{Self::close_step(self,grant)}
    fn terminal_is_empty(&self)->bool{Self::terminal_is_empty(self)}
}
