//! 📇️ A registered child snapshot owner binds its mandatory native admission producer and editor identity together.
use super::{source::{ChildEditorCapability,ChildEditorRequest,ChildEditorSourceCursor,ChildEditorSourceStep},read::ChildEditorRead};
use semio_framework_value::{ValueError,retained_clone::RetainedCloneGrant};
use super::super::MemberKeyRef;
use crate::ArtifactView;
use semio_framework::Fault;
use semio_framework_io_ports::{EmbeddedChildAdmitterV1,EmbeddedChildVectorSourceV1};
use pack::snapshot::ArtifactPack;
use store::os_store::serialization::ArtifactStoreSnapshot;
pub struct ChildEditorRegistration<S>{pub identity:ChildEditorCapability,pub admission:&'static dyn EmbeddedChildAdmitterV1,pub vector:&'static dyn EmbeddedChildVectorSourceV1,marker:std::marker::PhantomData<fn()->S>}
impl<S> ChildEditorRegistration<S>{pub const fn new(identity:ChildEditorCapability,admission:&'static dyn EmbeddedChildAdmitterV1,vector:&'static dyn EmbeddedChildVectorSourceV1)->Self{Self{identity,admission,vector,marker:std::marker::PhantomData}}}
impl<S:ArtifactPack+ArtifactStoreSnapshot+Send+Sync+'static> ChildEditorRegistration<S>{pub fn read<'a,P>(&'static self,parent:&'a ArtifactView<'_,P>,key:MemberKeyRef<'_>)->Result<ChildEditorRead<'a,S>,Fault>{parent.child_editor_read::<S>(key)}}

impl<S> ChildEditorRegistration<S>{
 /// 🧭️ Qualifies against the same typed member loan and exact registered capability, without a caller-supplied current source.
 pub fn qualify(&'static self,read:&ChildEditorRead<'_,S>,request:&ChildEditorRequest<'_>,cursor:&mut ChildEditorSourceCursor,grant:RetainedCloneGrant)->Result<ChildEditorSourceStep,ValueError>{cursor.advance(request,&read.current(&self.identity),grant)}
}

impl<S:semio_framework_value::retirement::RetireOwned+Sync> ChildEditorRegistration<S>{
 /// 🤝️ The actual held member custody supplies the same registered vector and admission pair.
 pub fn admission_provider<'a>(&'static self,custody:&'a super::read::custody::ChildEditorSourceCustody<S>,ordinal:usize,grant:RetainedCloneGrant)->Result<(super::read::admission::ChildEditorAdmissionProvider<'a,S>,semio_framework_value::retained_clone::RetainedCloneProgress),ValueError>{super::read::admission::ChildEditorAdmissionProvider::admit(custody,self,ordinal,grant)}
}
