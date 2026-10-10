//! 🔗️ Original peer read captures retain one exact root and factory pair through receipt-funded retirement.
use super::*;

pub struct PresencePeersRead<P>{root:std::mem::ManuallyDrop<Option<Arc<PresencePeersRoot<P>>>>,factory:std::mem::ManuallyDrop<Option<Arc<dyn SnapshotRetirementFactory<P>>>>}
impl<P> PresencePeersRead<P>{
 pub fn root(&self)->&PresencePeersRoot<P>{self.root.as_deref().expect("original peer read root")}
 pub fn terminal_is_empty(&self)->bool{self.root.is_none()&&self.factory.is_none()}
}
impl<P:Send+Sync+'static> PresencePeersRead<P>{
 /// 🫴️ Moves the same original aliases into the existing base-root retirement cursor after preflight.
 pub fn into_retirement(mut self,grant:RetainedCloneGrant)->Result<(PresencePeersRetirement<P>,RetainedCloneProgress),(ValueError,Self)>{
  let copied_bytes=size_of::<PresencePeersRetirement<P>>();
  if let Err(error)=admit(grant,copied_bytes){return Err((error,self))}
  let root=self.root.take().expect("original peer root");let factory=self.factory.take().expect("original peer factory");
  Ok((PresencePeersRetirement::from_root(root,Some(factory),PresencePeerRootCustody::BaseAlias),RetainedCloneProgress{copied_items:1,copied_bytes,..Default::default()}))
 }
}
fn admit(grant:RetainedCloneGrant,copied_bytes:usize)->Result<(),ValueError>{if grant.maximum_items==0||grant.maximum_copy_bytes<copied_bytes{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original peer read requires its complete root and factory alias transfer"))}if grant.maximum_depth<2{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original peer read requires root and factory depth"))}Ok(())}
impl<P:Clone+Send+Sync+'static,M:Mutation<P>> PresenceStore<P,M>{
 /// 👁️ Captures the installed original root and factory without copying any peer payload.
 pub fn capture_peer_root(&self,grant:RetainedCloneGrant)->Result<(PresencePeersRead<P>,RetainedCloneProgress),ValueError>{
  admit(grant,size_of::<PresencePeersRead<P>>())?;let root=self.peers.as_ref().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original peer store root is absent"))?;let factory=self.peer_retirement_factory.as_ref().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original peer store factory is absent"))?;
  Ok((PresencePeersRead{root:std::mem::ManuallyDrop::new(Some(root.clone())),factory:std::mem::ManuallyDrop::new(Some(factory.clone()))},RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<PresencePeersRead<P>>(),..Default::default()}))
 }
}
impl<P> Drop for PresencePeersRead<P>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original peer read abandoned root and factory custody");if self.terminal_is_empty(){unsafe{std::mem::ManuallyDrop::drop(&mut self.root);std::mem::ManuallyDrop::drop(&mut self.factory)}}}}
