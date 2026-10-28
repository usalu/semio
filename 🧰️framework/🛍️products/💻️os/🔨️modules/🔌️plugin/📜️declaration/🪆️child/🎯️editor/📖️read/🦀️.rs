//! 📖️ A child editor borrows the exact published member entry and parent operation instead of decoding a mirror.
use super::super::{ChildContentView,ChildContentEntry,MemberKeyRef};
use super::source::{ChildEditorCapability,ChildEditorCurrent,ChildEditorFields};
use semio_framework::Fault;
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
use pack::snapshot::ArtifactPack;
use store::os_store::{serialization::ArtifactStoreSnapshot,snapshot_read::SnapshotReadRef};
pub struct ChildEditorRead<'a,S>{pub read:SnapshotReadRef<'a,S>,entry:&'a ChildContentEntry,parent_identity:usize,parent_artifact:&'a str,parent_revision:&'a[u8;32]}
impl<'a,S> ChildEditorRead<'a,S>{
 pub fn current(&self,capability:&'static ChildEditorCapability)->ChildEditorCurrent<'_>{ChildEditorCurrent{parent_identity:self.parent_identity,child_identity:&self.entry.snapshot as*const _ as usize,capability,fields:ChildEditorFields{parent_artifact:self.parent_artifact,owner:&self.entry.owner,slot:&self.entry.slot,child_id:&self.entry.child_id,artifact_id:&self.entry.artifact_id,artifact_kind:&self.entry.dialect.artifact_kind,standard:&self.entry.dialect.standard,subset:&self.entry.dialect.subset,editor_id:capability.editor_id,command_id:capability.command_id,parent_revision:self.parent_revision,child_revision:&self.entry.revision}}}
}
impl ChildContentView{
 pub(crate) fn editor_read_at<'a,S:ArtifactPack+ArtifactStoreSnapshot+Send+Sync+'static>(&'a self,key:MemberKeyRef<'_>,parent_identity:usize,parent_artifact:&'a str,parent_revision:&'a[u8;32])->Result<ChildEditorRead<'a,S>,Fault>{let entry=self.find(key)?.ok_or_else(||plugin_sdk_fault("no live member for child editor"))?;let read=entry.snapshot.typed::<S>().ok_or_else(||plugin_sdk_fault("registered child editor snapshot type differs from member owner"))?;Ok(ChildEditorRead{read,entry,parent_identity,parent_artifact,parent_revision})}
}

#[path="🔗️custody/🦀️.rs"]pub mod custody;

impl semio_framework_io_ports::ChildAdmissionQualificationV1 for super::source::ChildEditorStamp{
 fn child_admission_stamp(&self)->semio_framework_io_ports::ChildAdmissionStamp{semio_framework_io_ports::ChildAdmissionStamp{parent_identity:self.parent_identity(),expected_reference_identity:self.expected_reference_identity(),child_identity:self.child_identity(),capability_identity:self.capability_identity(),request_identity:self.command_identity()as u64,parent_revision:*self.parent_revision(),child_revision:*self.child_revision()}}
}

#[path="🧩️admission/🦀️.rs"]pub mod admission;
pub use admission::ChildEditorAdmissionProvider;

#[path="🧾️metadata/🦀️.rs"]pub mod metadata;
pub use metadata::ChildMemberReadMetadata;

#[cfg(test)]#[path="../../📖️vector/🧪️tests.rs"]mod vector_read_tests;

#[path="🧭️lookup/🦀️.rs"]pub mod lookup;
