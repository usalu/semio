//! 📇️ Common child editor authority keeps vector admission as an explicit domain extension.
use super::{source::{ChildEditorCapability,ChildEditorRequest,ChildEditorSourceCursor,ChildEditorSourceStep},read::ChildEditorRead};
use semio_framework_value::{ValueError,ValueRefusalKind,retained_clone::RetainedCloneGrant};
use super::super::MemberKeyRef;
use crate::ArtifactView;
use semio_framework::Fault;
use semio_framework_io_ports::{EmbeddedChildAdmitterV1,EmbeddedChildVectorSourceV1};
use pack::snapshot::ArtifactPack;
use store::os_store::serialization::ArtifactStoreSnapshot;
pub struct ChildEditorVectorServices{pub admission:&'static dyn EmbeddedChildAdmitterV1,pub vector:&'static dyn EmbeddedChildVectorSourceV1}
pub struct ChildEditorRegistration<S>{pub identity:ChildEditorCapability,vector:Option<ChildEditorVectorServices>,marker:std::marker::PhantomData<fn()->S>}
impl<S> ChildEditorRegistration<S>{
 pub const fn new(identity:ChildEditorCapability)->Self{Self{identity,vector:None,marker:std::marker::PhantomData}}
 pub const fn with_vector(identity:ChildEditorCapability,admission:&'static dyn EmbeddedChildAdmitterV1,vector:&'static dyn EmbeddedChildVectorSourceV1)->Self{Self{identity,vector:Some(ChildEditorVectorServices{admission,vector}),marker:std::marker::PhantomData}}
 pub fn vector_services(&self)->Result<&ChildEditorVectorServices,ValueError>{self.vector.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"child editor has no registered vector extension"))}
 pub fn qualify(&'static self,read:&ChildEditorRead<'_,S>,request:&ChildEditorRequest<'_>,cursor:&mut ChildEditorSourceCursor,grant:RetainedCloneGrant)->Result<ChildEditorSourceStep,ValueError>{cursor.advance(request,&read.current(&self.identity),grant)}
}
impl<S:ArtifactPack+ArtifactStoreSnapshot+::semio_os_kernel::StoreState> ChildEditorRegistration<S>{pub fn read<'a,P>(&'static self,parent:&'a ArtifactView<'_,P>,key:MemberKeyRef<'_>)->Result<ChildEditorRead<'a,S>,Fault>{parent.child_editor_read::<S>(key)}}
impl<S:semio_framework_value::retirement::RetireOwned+::semio_os_kernel::StoreState> ChildEditorRegistration<S>{
 pub fn admission_provider<'a>(&'static self,custody:&'a super::read::custody::ChildEditorSourceCustody<S>,ordinal:usize,grant:RetainedCloneGrant)->Result<(super::read::admission::ChildEditorAdmissionProvider<'a,S>,semio_framework_value::retained_clone::RetainedCloneProgress),ValueError>{super::read::admission::ChildEditorAdmissionProvider::admit(custody,self,ordinal,grant)}
}
