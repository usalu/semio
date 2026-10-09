//! 📷️ Copies only the original fixed camera fields and admits its inverse Box and row separately.
use super::{RetainedLoadCameraConfig as State,RetainedLoadCameraConfigMutation as Mutation};
use crate::store;
use crate::component::window_config::preparation_custody::WindowEditCustody;
use store::snapshot_clone_preparation::{RetainedCloneEdit,RetainedCloneEditCursor,RetainedCloneEditStep};
use semio_framework_value::{ValueError,ValueRefusalKind,retained_clone::{RetainedClone,RetainedCloneRef,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedCloneBirthDemand,ScalarCursor}};
use std::mem::size_of;
impl RetainedClone for State{type Cursor=ScalarCursor<Self>;fn retained_clone_cursor()->Self::Cursor{Default::default()}}
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub(crate) struct CameraRetainedEdit;
pub(crate) struct CameraRetainedEditCursor{stage:u8,cancelled:bool,custody:WindowEditCustody<(Option<Box<State>>,Option<Vec<Mutation>>)>}
impl RetainedCloneEdit<State,Mutation> for CameraRetainedEdit{
 type Cursor=CameraRetainedEditCursor;
 fn preflight(&self,mutation:&Mutation,lane:store::HistoryLane)->Result<store::ArtifactStoreOneItemFootprint,String>{let Mutation::Snapshot{config}=mutation;if lane!=store::HistoryLane::Document||![config.viewport.x,config.viewport.y,config.viewport.zoom].into_iter().chain(config.eye).all(f64::is_finite)||config.viewport.zoom<=0.0{return Err("original camera mutation has invalid lane or finite viewport authority".into());}Ok(store::ArtifactStoreOneItemFootprint{work_items:2,retained_bytes:65536})}
 fn begin_demand(&self)->RetainedCloneBirthDemand{RetainedCloneBirthDemand{capacity_bytes:0,depth:1}}
 fn snapshot_cursor_birth_demand(&self)->RetainedCloneBirthDemand{RetainedCloneBirthDemand{capacity_bytes:0,depth:1}}
 fn begin(&self,grant:RetainedCloneGrant)->Result<(Self::Cursor,RetainedCloneProgress),ValueError>{let mut receipt=self.begin_demand().admit(grant)?;receipt.copied_bytes=size_of::<Self::Cursor>();if !receipt.fits(grant){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"camera original cursor transfer exceeds grant"));}Ok((CameraRetainedEditCursor{stage:0,cancelled:false,custody:WindowEditCustody::new((None,None))},receipt))}
}
impl CameraRetainedEditCursor{pub(crate) fn normal_demand(&self)->semio_framework_value::RetirementDemand{let(copy_bytes,capacity_bytes)=match self.stage{0=>(size_of::<State>()*2,0),1=>(size_of::<State>()+size_of::<Box<State>>(),size_of::<State>()),2=>(size_of::<Vec<Mutation>>(),size_of::<Mutation>()),3=>(size_of::<Mutation>()*2+size_of::<Box<State>>(),0),_=>(0,0)};semio_framework_value::RetirementDemand{copy_bytes,capacity_bytes,depth:2,..Default::default()}}}
impl RetainedCloneEditCursor<State,Mutation> for CameraRetainedEditCursor{
 fn advance(&mut self,base:RetainedCloneRef<'_,State>,post:&mut State,mutation:RetainedCloneRef<'_,Mutation>,grant:RetainedCloneGrant)->Result<RetainedCloneEditStep,String>{
  if self.cancelled||self.custody.is_closing()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneEditStep::Progress(Default::default()));}
  if self.stage==4{return Ok(RetainedCloneEditStep::Complete(Default::default()));}let demand=self.normal_demand();let(copy,capacity)=(demand.copy_bytes,demand.capacity_bytes);
  if copy>grant.maximum_copy_bytes||capacity>grant.maximum_capacity_bytes||demand.depth>grant.maximum_depth{return Ok(RetainedCloneEditStep::Progress(Default::default()));}
  match self.stage{0=>{let Mutation::Snapshot{config}=mutation.get();*post=**config;},1=>self.custody.original_mut().0=Some(Box::new(*base.get())),2=>{let mut rows=Vec::new();rows.try_reserve_exact(1).map_err(|_|"original camera inverse row allocation failed")?;self.custody.original_mut().1=Some(rows);},3=>{let fields=self.custody.original_mut();fields.1.as_mut().ok_or("original camera inverse rows absent")?.push(Mutation::Snapshot{config:fields.0.take().ok_or("original camera inverse Box absent")?});},_=>unreachable!()}
  self.stage+=1;let receipt=RetainedCloneProgress{copied_items:1,copied_bytes:copy,retained_capacity_bytes:capacity,released_bytes:0};Ok(if self.stage==4{RetainedCloneEditStep::Complete(receipt)}else{RetainedCloneEditStep::Progress(receipt)})
 }
 fn take_inverse(&mut self)->Option<Vec<Mutation>>{(self.stage==4).then(||self.custody.original_mut().1.take()).flatten()}
 fn foreign_step_presence(&self)->bool{false}
 fn cancel(&mut self){self.cancelled=true;}
 fn begin_close(&mut self)->bool{self.custody.begin_close()}
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.custody.step(grant)}
 fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.custody.demands(0)?.copy_bytes)}
 fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.custody.demands(body)?.capacity_bytes)}
 fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.custody.demands(0)?.release_bytes)}
 fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.custody.demands(0)?.depth)}
 fn terminal_is_empty(&self)->bool{self.custody.terminal_is_empty()}
}
