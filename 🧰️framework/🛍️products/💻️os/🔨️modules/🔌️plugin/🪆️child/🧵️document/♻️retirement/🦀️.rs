//! ♻️ Unmounted document cursor delegates child-root disposal to its actual member registry.
use super::*;
fn invariant(message:&str)->Fault{Fault::new(FaultOrigin::Framework,FaultCode::new("interactive-job.tool-document-retirement-invariant"),message)}

pub(crate) struct ArtifactOwnedToolDocumentRetirement<A:ArtifactApp>{
 parent:Option<store::SnapshotRead<A::Snapshot>>,
 children:Option<std::sync::Arc<ChildContentView>>,
 child_retirement:Option<ChildContentRetirement>,
 returned:Option<store::SnapshotReadReturn>,
}
impl<A:ArtifactApp> ArtifactOwnedToolDocumentRetirement<A>{
 pub(crate) fn new(mut document:ArtifactOwnedToolDocument<A>)->Self{
  Self{parent:document.parent.take(),children:document.children.take(),child_retirement:None,returned:None}
 }
 pub(crate) fn terminal_is_empty(&self)->bool{self.parent.is_none()&&self.children.is_none()&&self.child_retirement.is_none()&&self.returned.is_none()}
 pub(crate) fn close_step<M:SpaceMember>(
  &mut self,members:&mut ChildMemberRegistry<M>,retiring:Option<&mut ArtifactFixedRegistry<ChildMemberRetirement<M>>>,
  current:&ChildContentView,owners:&ChildContentOwners,maximum_items:usize,maximum_bytes:usize,
 )->Result<PluginCloseStep,Fault>{
  if maximum_items==0{return Ok(PluginCloseStep::Pending{released_items:0,released_bytes:0})}
  if let Some(active)=self.child_retirement.as_mut(){
   let step=active.close_step(members,retiring,current,owners,maximum_items,maximum_bytes)?;
   if matches!(step,PluginCloseStep::Complete){
    if !active.terminal_is_empty(){return Err(invariant("tool document child retirement completed with retained authority"))}
    self.child_retirement.take();
    return Ok(PluginCloseStep::Pending{released_items:1,released_bytes:0})
   }
   return Ok(step)
  }
  if self.children.is_some(){
   let width=std::mem::size_of::<std::sync::Arc<ChildContentView>>();
   if maximum_bytes<width{return Ok(PluginCloseStep::Pending{released_items:0,released_bytes:0})}
   let children=self.children.take().ok_or_else(||invariant("tool document child root disappeared"))?;
   match std::sync::Arc::try_unwrap(children){
    Ok(view)=>self.child_retirement=Some(ChildContentRetirement::new(view,false)),
    Err(children)=>{self.children=Some(children);return Ok(PluginCloseStep::Blocked{reason:"tool document child root still has an exact command owner"})}
   }
   return Ok(PluginCloseStep::Pending{released_items:1,released_bytes:width})
  }
  if self.parent.is_some(){
   let width=std::mem::size_of::<store::SnapshotRead<A::Snapshot>>();
   if maximum_bytes<width{return Ok(PluginCloseStep::Pending{released_items:0,released_bytes:0})}
   let parent=self.parent.take().ok_or_else(||invariant("tool document parent read disappeared"))?;
   self.returned=Some(parent.return_to_registry_witness().ok_or_else(||invariant("tool document parent read did not return its exact registry witness"))?);
   return Ok(PluginCloseStep::Pending{released_items:1,released_bytes:width})
  }
  if let Some(returned)=self.returned.as_ref(){
   if !returned.terminal_is_empty(){return Ok(PluginCloseStep::Blocked{reason:"tool document parent return awaits its actual store retirement pump"})}
   let width=std::mem::size_of::<store::SnapshotReadReturn>();
   if maximum_bytes<width{return Ok(PluginCloseStep::Pending{released_items:0,released_bytes:0})}
   self.returned.take();
   return Ok(PluginCloseStep::Pending{released_items:1,released_bytes:width})
  }
  Ok(PluginCloseStep::Complete)
 }
}
