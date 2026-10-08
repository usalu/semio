//! 🧰️ Owns one genuine native preparation controller under exact birth and terminal grants.

use semio_framework_value::{ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::mem::size_of;


#[derive(Clone,Copy)]
pub(crate) enum Puzzle2dCloseAxis { Copy,Capacity(usize),Release,Depth }

impl Puzzle2dCloseAxis {
    pub(crate) fn inline<T>(self)->Result<usize,ValueError> {Ok(match self {Self::Copy=>size_of::<T>(),Self::Depth=>1,_=>0})}
    pub(crate) fn retained<T:semio_framework_value::retained_clone::RetainedClone,C:semio_framework_value::retained_clone::RetainedCloneCursor<T>>(self,owner:&C)->Result<usize,ValueError> {
        match self {Self::Copy=>owner.next_close_copy_byte_demand(),Self::Capacity(body)=>owner.next_close_capacity_byte_demand(body),Self::Release=>owner.next_close_release_byte_demand(),Self::Depth=>owner.next_close_depth_demand()}
    }
    pub(crate) fn retirement<T:semio_framework_value::retirement::RetireOwned>(self,owner:&semio_framework_value::retirement::controlled::ControlledRetirement<T>)->Result<usize,ValueError> {
        match self {Self::Copy=>owner.next_copy_byte_demand(),Self::Capacity(body)=>owner.next_capacity_byte_demand(body),Self::Release=>owner.next_release_byte_demand(),Self::Depth=>owner.next_depth_demand()}
    }
    pub(crate) fn binding(self,owner:&Option<semio_framework_value::retained_clone::RetainedCloneBinding>)->Result<usize,ValueError> {
        use semio_framework_value::retained_clone::RetainedCloneBinding;
        match self {Self::Copy=>RetainedCloneBinding::copy_demand(owner),Self::Capacity(body)=>RetainedCloneBinding::capacity_demand(owner,body),Self::Release=>RetainedCloneBinding::release_demand(owner),Self::Depth=>RetainedCloneBinding::depth_demand(owner)}
    }
}

macro_rules! close_child_demand {
    ($axis:expr,$owner:expr)=>{match $axis {Puzzle2dCloseAxis::Copy=>$owner.next_close_copy_byte_demand(),Puzzle2dCloseAxis::Capacity(body)=>$owner.next_close_capacity_byte_demand(body),Puzzle2dCloseAxis::Release=>$owner.next_close_release_byte_demand(),Puzzle2dCloseAxis::Depth=>$owner.next_close_depth_demand()}};
}
pub(crate) use close_child_demand;

macro_rules! close_demand_methods {
    ($copy:ident,$capacity:ident,$release:ident,$depth:ident)=>{
        pub fn $copy(&self)->Result<usize,ValueError>{self.close_demand(Puzzle2dCloseAxis::Copy)}
        pub fn $capacity(&self,body:usize)->Result<usize,ValueError>{self.close_demand(Puzzle2dCloseAxis::Capacity(body))}
        pub fn $release(&self)->Result<usize,ValueError>{self.close_demand(Puzzle2dCloseAxis::Release)}
        pub fn $depth(&self)->Result<usize,ValueError>{self.close_demand(Puzzle2dCloseAxis::Depth)}
    };
}
pub(crate) use close_demand_methods;

pub(crate) trait Puzzle2dNativePreparation: Default {
    fn begin_close(&mut self);
    fn close_granted(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
    fn terminal_is_empty(&self)->bool;
    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>;
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>;
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>;
    fn next_close_depth_demand(&self)->Result<usize,ValueError>;
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
        if grant.maximum_items==0 || grant.maximum_depth==0 || grant.maximum_capacity_bytes < bytes {return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
        self.owner=Some(Box::new(C::default()));
        Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,retained_capacity_bytes:bytes,..Default::default()})))
    }

    pub(crate) fn owner_mut(&mut self)->&mut C {self.owner.as_mut().expect("native preparation child birth was admitted")}

    pub(crate) fn begin_close(&mut self) {self.closing=true;if let Some(owner)=self.owner.as_mut(){owner.begin_close();}}

    fn close_demand(&self,axis:Puzzle2dCloseAxis)->Result<usize,ValueError> {
        let Some(owner)=self.owner.as_ref()else{return Ok(0)};
        if !owner.terminal_is_empty(){return close_child_demand!(axis,owner);}
        Ok(match axis {Puzzle2dCloseAxis::Release=>size_of::<C>(),Puzzle2dCloseAxis::Depth=>1,_=>0})
    }

    close_demand_methods!(next_close_copy_byte_demand,next_close_capacity_byte_demand,next_close_release_byte_demand,next_close_depth_demand);

    pub(crate) fn close_granted(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        if !self.closing {return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"native preparation child must begin close"));}
        if grant.maximum_items==0 || grant.maximum_depth==0 {return Ok(RetainedCloneStep::Progress(Default::default()));}
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

macro_rules! borrowed_preparation {
    ($cursor:ty)=>{
        impl Puzzle2dNativePreparation for $cursor {
            fn begin_close(&mut self){<$cursor>::begin_close(self);}
            fn close_granted(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{<$cursor>::close_step(self,grant)}
            fn terminal_is_empty(&self)->bool{<$cursor>::terminal_is_empty(self)}
            fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{<$cursor>::next_close_copy_byte_demand(self)}
            fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{<$cursor>::next_close_capacity_byte_demand(self,body)}
            fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{<$cursor>::next_close_release_byte_demand(self)}
            fn next_close_depth_demand(&self)->Result<usize,ValueError>{<$cursor>::next_close_depth_demand(self)}
        }
    };
}

borrowed_preparation!(crate::standards::v1::subsets::any::schema::mutations::connect_handles::prepare::Puzzle2dConnectHandlesPreparationCursor);
borrowed_preparation!(crate::standards::v1::subsets::any::schema::mutations::delete_node::preparation::Puzzle2dDeleteNodePreparationCursor);

impl Puzzle2dNativePreparation for crate::standards::v1::subsets::any::schema::mutations::connect_handles::prepare::edge::Puzzle2dConnectEdgeCursor {
    fn begin_close(&mut self){Self::begin_close(self);}
    fn close_granted(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{Self::close_step(self,grant)}
    fn terminal_is_empty(&self)->bool{Self::terminal_is_empty(self)}
    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Self::next_close_copy_byte_demand(self)}
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Self::next_close_capacity_byte_demand(self,body)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Self::next_close_release_byte_demand(self)}
    fn next_close_depth_demand(&self)->Result<usize,ValueError>{Self::next_close_depth_demand(self)}
}
