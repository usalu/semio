//! 🔗️ Immutable first-party aliases share one retained clone lifecycle without copying their payload graph.
use super::*;
/// 🎟️ A defining immutable owner declares its actual alias operation independently of payload cloning.
pub trait RetainedAlias:RetireOwned+Sync{fn alias_demand(&self)->crate::RetirementDemand;fn admit_alias(&self,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),crate::ValueError>where Self:Sized;}
struct AliasState<T:RetainedAlias>{output:Option<T>,source:Option<RetainedCloneBinding>,close:RetainedCloneClose,closing:bool,spent:bool}
/// 🧷️ The original source remains issued until the result and every cancelled alias physically close.
pub struct RetainedAliasCursor<T:RetainedAlias>{state:ManuallyDrop<AliasState<T>>}
impl<T:RetainedAlias> Default for RetainedAliasCursor<T>{fn default()->Self{Self{state:ManuallyDrop::new(AliasState{output:None,source:None,close:Default::default(),closing:false,spent:false})}}}
fn alias_invalid()->crate::ValueError{crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"original immutable alias changed its source or lifecycle")}
impl<T:RetainedAlias+RetainedClone> RetainedCloneCursor<T> for RetainedAliasCursor<T>{
 fn output_ready(&self)->bool{!self.state.closing&&!self.state.spent&&self.state.output.is_some()}
 fn advance_demands(&self,source:RetainedCloneRef<'_,T>,_:usize)->Result<crate::RetirementDemand,crate::ValueError>{if self.state.closing||self.state.spent||self.state.output.is_some(){return Err(alias_invalid())}if let Some(d)=source.binding_demand(&self.state.source)?{return Ok(d)}Ok(source.get().alias_demand())}
 fn advance(&mut self,source:RetainedCloneRef<'_,T>,body:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,crate::ValueError>{let d=self.advance_demands(source,body)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}if let Some(p)=source.bind(&mut self.state.source,g)?{return Ok(RetainedCloneStep::Progress(p))}let(output,p)=source.get().admit_alias(g)?;self.state.output=Some(output);Ok(RetainedCloneStep::Complete(p))}
 fn take(&mut self)->Option<T>{if !self.output_ready(){return None}self.state.spent=true;self.state.output.take()}
 fn begin_close(&mut self)->bool{let changed=!self.state.closing;self.state.closing=true;changed}
 fn close_started(&self)->bool{self.state.closing}
 fn close_demands(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{if !self.state.closing{return Err(alias_invalid())}self.state.close.next_owner_demands::<T>(self.state.output.is_some(),&self.state.source,body)}
 fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,crate::ValueError>{let d=self.close_demands(g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}if !self.state.close.is_empty(){return self.state.close.step_granted(g)}let state=&mut*self.state;if let Some(step)=state.close.begin_granted(&mut state.output,g)?{return Ok(step)}RetainedCloneBinding::close_one(&mut state.source,g)}
 fn terminal_is_empty(&self)->bool{self.state.output.is_none()&&self.state.source.is_none()&&self.state.close.is_empty()}
 fn next_close_copy_byte_demand(&self)->Result<usize,crate::ValueError>{self.state.close.next_copy_with_binding(&self.state.source)}
 fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,crate::ValueError>{self.state.close.next_owner_capacity_with_binding::<T>(self.state.output.is_some(),body,&self.state.source)}
 fn next_close_release_byte_demand(&self)->Result<usize,crate::ValueError>{self.state.close.next_release_with_binding(&self.state.source)}
 fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{self.state.close.next_owner_depth_with_binding(self.state.output.is_some(),&self.state.source)}
}
impl<T:RetainedAlias> Drop for RetainedAliasCursor<T>{fn drop(&mut self){if self.state.output.is_none()&&self.state.source.is_none()&&self.state.close.is_empty(){unsafe{ManuallyDrop::drop(&mut self.state)}}else{assert!(std::thread::panicking(),"original immutable alias requires funded closure")}}}
