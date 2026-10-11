//! 🗂️ Composable persisted collection artifact.

#[cfg(test)]
#[path="../../../../../../🔨️modules/⏱️trace/🧮️memory/🧪️testing/📥️requests/🦀️.rs"]
pub(crate) mod test_allocation;
#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATION_OBSERVER:test_allocation::RequestedAllocator=test_allocation::RequestedAllocator;

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
pub use io::sqlite::snapshot::{register_sqlite_snapshot,SQLITE_SNAPSHOT_DIALECT};
extern crate semio_framework_value_derive as value_derive;

#[path = "♻️retirement/🦀️.rs"]
mod retirement;

#[path = "🌲️canonical/🦀️.rs"]
mod canonical;

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

//#region 🔖️Collection
pub const S_COLLECTION_SCHEMA: &str = "os.collection";

/// 📁️ One parent-linked folder in a collection's flat tree. `parent_id: None` means root.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct CollectionFolder {
    pub id: String,
    pub parent_id: Option<String>,
    pub name: String,
}

/// 📦️ What a `CollectionEntry` addresses: either a document artifact (`schema` + the `s.<schema>`
/// document's own id — see `🔖️Addressing`: `CollectionEntry.id == artifact id == ArtifactEnvelope.id`
/// for document artifacts) or a content-addressed blob (files/meshes/breps-as-bytes).
///
/// 🧬️ Hand-crafted `dsl::DslVariants` instead of `#[derive(dsl::DslEnum)]`: the `Blob` variant embeds
/// `store::BlobRef` verbatim (the plan's `Addressing` design ruling pins this exact type), a foreign
/// type this crate cannot implement `dsl::DslField` for under the orphan rule — same reasoning as
/// `workflow::MediaContract`'s hand-crafted `dsl::DslField` impl for its own foreign sub-values. Since
/// `ArtifactBody` itself IS local, hand-writing `DslVariants` bridges `BlobRef`'s three fields
/// (`hash`/`size`/`media_type`) directly to scalar `dsl::FieldValue`s right here.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[serde(tag = "kind", rename_all = "camelCase")]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum ArtifactBody {
    Document { schema: String, document_id: String },
    Blob { blob: store::BlobRef },
}

fn artifact_body_document_borrowed()->semio_framework_dsl_record::BorrowedRecordSpec{
 use semio_framework_dsl_record::{BorrowedFieldSpec as F,BorrowedShape as H};
 const FIELDS:&[F]=&[F::new(0,"schema",H::Text),F::new(1,"document-id",H::Text)];
 semio_framework_dsl_record::BorrowedRecordSpec{keyword:Some("document"),layout:semio_framework_dsl_record::RecordLayout::Inline,fields:FIELDS}
}
fn artifact_body_blob_borrowed()->semio_framework_dsl_record::BorrowedRecordSpec{
 use semio_framework_dsl_record::{BorrowedFieldSpec as F,BorrowedShape as H};
 const FIELDS:&[F]=&[F::new(0,"hash",H::Text),F::new(1,"size",H::UInt),F::new(2,"media-type",H::Text)];
 semio_framework_dsl_record::BorrowedRecordSpec{keyword:Some("blob"),layout:semio_framework_dsl_record::RecordLayout::Inline,fields:FIELDS}
}
impl semio_framework_dsl_record::BorrowedDslVariants for ArtifactBody{
 const VARIANTS:&'static[(&'static str,fn()->semio_framework_dsl_record::BorrowedRecordSpec)]=&[("document",artifact_body_document_borrowed),("blob",artifact_body_blob_borrowed)];
 fn projected_borrowed_variant_identity(&self)->(&'static str,usize,semio_framework_dsl_record::BorrowedRecordSpec){match self{Self::Document{..}=>("document",0,artifact_body_document_borrowed()),Self::Blob{..}=>("blob",1,artifact_body_blob_borrowed())}}

 fn projected_borrowed_variant_view(&self,path:&[usize])->Result<semio_framework_dsl_record::native_encoding::FieldProjectionView<'_>,semio_framework_value::ValueError>{<Self as semio_framework_dsl_record::DslVariants>::projected_variant_view(self,path)}
}

fn artifact_body_document_spec() -> semio_framework_dsl_record::RecordSpec {
    semio_framework_dsl_record::RecordSpec::new(Some("document"), semio_framework_dsl_record::RecordLayout::Inline, vec![semio_framework_dsl_record::FieldSpec::new(0, "schema", semio_framework_dsl_record::Shape::Text), semio_framework_dsl_record::FieldSpec::new(1, "document-id", semio_framework_dsl_record::Shape::Text)])
}

fn artifact_body_blob_spec() -> semio_framework_dsl_record::RecordSpec {
    semio_framework_dsl_record::RecordSpec::new(Some("blob"), semio_framework_dsl_record::RecordLayout::Inline, vec![semio_framework_dsl_record::FieldSpec::new(0, "hash", semio_framework_dsl_record::Shape::Text), semio_framework_dsl_record::FieldSpec::new(1, "size", semio_framework_dsl_record::Shape::UInt), semio_framework_dsl_record::FieldSpec::new(2, "media-type", semio_framework_dsl_record::Shape::Text)])
}

fn artifact_body_document_spec_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::RecordSpec,semio_framework_value::ValueError>{
    control.scoped_stage(|control|{
        control.begin_stage(2)?;
        let mut fields=control.allocate_vec(2)?;
        fields.push(semio_framework_dsl_record::producer::field(0,"schema",semio_framework_dsl_record::Shape::Text,control)?);
        control.step()?;
        fields.push(semio_framework_dsl_record::producer::field(1,"document-id",semio_framework_dsl_record::Shape::Text,control)?);
        control.step()?;
        semio_framework_dsl_record::producer::record(Some("document"),semio_framework_dsl_record::RecordLayout::Inline,fields,control)
    })
}

fn artifact_body_blob_spec_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::RecordSpec,semio_framework_value::ValueError>{
    control.scoped_stage(|control|{
        control.begin_stage(3)?;
        let mut fields=control.allocate_vec(3)?;
        fields.push(semio_framework_dsl_record::producer::field(0,"hash",semio_framework_dsl_record::Shape::Text,control)?);
        control.step()?;
        fields.push(semio_framework_dsl_record::producer::field(1,"size",semio_framework_dsl_record::Shape::UInt,control)?);
        control.step()?;
        fields.push(semio_framework_dsl_record::producer::field(2,"media-type",semio_framework_dsl_record::Shape::Text,control)?);
        control.step()?;
        semio_framework_dsl_record::producer::record(Some("blob"),semio_framework_dsl_record::RecordLayout::Inline,fields,control)
    })
}

fn artifact_body_document_producer()->semio_framework_dsl_record::RecordSpecProducer{
    semio_framework_dsl_record::RecordSpecProducer{ordinary:artifact_body_document_spec,decoding:|control|artifact_body_document_spec_controlled(control),encoding:|control|artifact_body_document_spec_controlled(control)}
}

fn artifact_body_blob_producer()->semio_framework_dsl_record::RecordSpecProducer{
    semio_framework_dsl_record::RecordSpecProducer{ordinary:artifact_body_blob_spec,decoding:|control|artifact_body_blob_spec_controlled(control),encoding:|control|artifact_body_blob_spec_controlled(control)}
}

impl semio_framework_dsl_record::DslVariants for ArtifactBody {
    fn projected_variant_identity(&self)->(&'static str,usize,semio_framework_dsl_record::RecordSpecProducer){
        match self{Self::Document{..}=>("document",0,artifact_body_document_producer()),Self::Blob{..}=>("blob",1,artifact_body_blob_producer())}
    }
    fn projected_variant_view(&self,path:&[usize])->Result<semio_framework_dsl_record::native_encoding::FieldProjectionView<'_>,semio_framework_value::ValueError>{
        use semio_framework_dsl_record::{DslField,native_encoding::{FieldProjectionView as V,projection_path_error}};
        if path.is_empty(){return Ok(V::Record(match self{Self::Document{..}=>&[0,1],Self::Blob{..}=>&[0,1,2]}))}
        let tail=&path[1..];
        match(self,path[0]){
            (Self::Document{schema,..},0)=>DslField::projection_view(schema,tail),
            (Self::Document{document_id,..},1)=>DslField::projection_view(document_id,tail),
            (Self::Blob{blob},0)=>DslField::projection_view(&blob.hash,tail),
            (Self::Blob{blob},1)=>DslField::projection_view(&blob.size,tail),
            (Self::Blob{blob},2)=>DslField::projection_view(&blob.media_type,tail),
            _=>Err(projection_path_error())
        }
    }
    fn projected_variant_key(&self,path:&[usize],index:usize)->Result<&str,semio_framework_value::ValueError>{
        use semio_framework_dsl_record::{DslField,native_encoding::projection_path_error};
        if path.is_empty(){return Err(projection_path_error())}
        let tail=&path[1..];
        match(self,path[0]){
            (Self::Document{schema,..},0)=>DslField::projection_key(schema,tail,index),
            (Self::Document{document_id,..},1)=>DslField::projection_key(document_id,tail,index),
            (Self::Blob{blob},0)=>DslField::projection_key(&blob.hash,tail,index),
            (Self::Blob{blob},1)=>DslField::projection_key(&blob.size,tail,index),
            (Self::Blob{blob},2)=>DslField::projection_key(&blob.media_type,tail,index),
            _=>Err(projection_path_error())
        }
    }

    fn to_named_record_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(String,semio_framework_dsl_record::RecordValue),semio_framework_value::ValueError>{
        control.scoped_stage(|control|{
            let count=match self{Self::Document{..}=>2,Self::Blob{..}=>3};control.begin_stage(count)?;let mut record=semio_framework_dsl_record::native_encoding::EncodedRecord::new(count,control)?;
            let keyword=match self{
                Self::Document{schema,document_id}=>{record.insert(0,semio_framework_dsl_record::FieldValue::Text(control.copy_text(schema)?))?;control.step()?;record.insert(1,semio_framework_dsl_record::FieldValue::Text(control.copy_text(document_id)?))?;control.step()?;"document"},
                Self::Blob{blob}=>{record.insert(0,semio_framework_dsl_record::FieldValue::Text(control.copy_text(&blob.hash)?))?;control.step()?;record.insert(1,semio_framework_dsl_record::FieldValue::UInt(blob.size))?;control.step()?;record.insert(2,semio_framework_dsl_record::FieldValue::Text(control.copy_text(&blob.media_type)?))?;control.step()?;"blob"}
            };Ok((control.copy_text(keyword)?,record.take()))
        })
    }
    fn from_named_record_controlled(keyword:&str,record:&semio_framework_dsl_record::RecordValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,semio_framework_value::ValueError>{
        fn text(record:&semio_framework_dsl_record::RecordValue,id:u16,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<String,semio_framework_value::ValueError>{control.scoped_stage(|control|{control.begin_stage(0)?;<String as semio_framework_dsl_record::DslField>::from_value_controlled(record.get(id).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"collection body text is absent"))?,control)})}
        let result:Result<Self,semio_framework_value::ValueError>=control.scoped_stage(|control|{
            match keyword{
                "document"=>{control.begin_stage(2)?;let schema=text(record,0,control)?;control.step()?;let document_id=text(record,1,control)?;control.step()?;Ok(Self::Document{schema,document_id})},
                "blob"=>{control.begin_stage(3)?;let hash=text(record,0,control)?;control.step()?;let size=control.scoped_stage(|control|{control.begin_stage(0)?;<u64 as semio_framework_dsl_record::DslField>::from_value_controlled(record.get(1).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"collection blob size is absent"))?,control)})?;control.step()?;let media_type=text(record,2,control)?;control.step()?;Ok(Self::Blob{blob:store::BlobRef{hash,size,media_type}})},
                _=>Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"collection body keyword is not declared"))
            }
        });result
    }
    fn variants() -> Vec<(String, semio_framework_dsl_record::RecordSpecProducer)> {
        vec![("document".to_string(), artifact_body_document_producer()), ("blob".to_string(), artifact_body_blob_producer())]
    }

    fn variants_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<Vec<(String,semio_framework_dsl_record::RecordSpecProducer)>,semio_framework_value::ValueError>{
        control.scoped_stage(|control|{
            control.begin_stage(2)?;
            let mut variants=control.allocate_vec(2)?;
            variants.push((control.copy_text("document")?,artifact_body_document_producer()));
            control.step()?;
            variants.push((control.copy_text("blob")?,artifact_body_blob_producer()));
            control.step()?;
            Ok(variants)
        })
    }

    fn to_named_record(&self) -> (String, semio_framework_dsl_record::RecordValue) {
        match self {
            ArtifactBody::Document { schema, document_id } => {
                let mut record = semio_framework_dsl_record::RecordValue::default();
                record.fields.insert(0, semio_framework_dsl_record::FieldValue::Text(schema.clone()));
                record.fields.insert(1, semio_framework_dsl_record::FieldValue::Text(document_id.clone()));
                ("document".to_string(), record)
            }
            ArtifactBody::Blob { blob } => {
                let mut record = semio_framework_dsl_record::RecordValue::default();
                record.fields.insert(0, semio_framework_dsl_record::FieldValue::Text(blob.hash.clone()));
                record.fields.insert(1, semio_framework_dsl_record::FieldValue::UInt(blob.size));
                record.fields.insert(2, semio_framework_dsl_record::FieldValue::Text(blob.media_type.clone()));
                ("blob".to_string(), record)
            }
        }
    }

    fn from_named_record(keyword: &str, record: &semio_framework_dsl_record::RecordValue) -> Result<Self, semio_framework_diagnostic::TextError> {
        match keyword {
            "document" => {
                let schema = match record.get(0) {
                    Some(semio_framework_dsl_record::FieldValue::Text(s)) => s.clone(),
                    other => return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("expected schema, found {other:?}")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1))),
                };
                let document_id = match record.get(1) {
                    Some(semio_framework_dsl_record::FieldValue::Text(s)) => s.clone(),
                    other => return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("expected document-id, found {other:?}")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1))),
                };
                Ok(ArtifactBody::Document { schema, document_id })
            }
            "blob" => {
                let hash = match record.get(0) {
                    Some(semio_framework_dsl_record::FieldValue::Text(s)) => s.clone(),
                    other => return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("expected hash, found {other:?}")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1))),
                };
                let size = match record.get(1) {
                    Some(semio_framework_dsl_record::FieldValue::UInt(v)) => *v,
                    other => return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("expected size, found {other:?}")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1))),
                };
                let media_type = match record.get(2) {
                    Some(semio_framework_dsl_record::FieldValue::Text(s)) => s.clone(),
                    other => return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("expected media-type, found {other:?}")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1))),
                };
                Ok(ArtifactBody::Blob { blob: store::BlobRef { hash, size, media_type } })
            }
            other => Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown ArtifactBody keyword '{other}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1))),
        }
    }
}

/// 🧾️ One addressable artifact placed in a collection folder tree. `id == artifact id ==
/// ArtifactEnvelope.id` for document bodies (see `🔖️Addressing`). `folder_id: None` means root-level.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct CollectionEntry {
    pub id: String,
    pub folder_id: Option<String>,
    pub name: String,
    pub kind_id: String,
    #[dsl(statements)]
    pub body: Box<ArtifactBody>,
}

/// 🗂️ A collection's flat parent-linked folder tree plus its artifact entries.
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, semio_framework_os_kernel::DslArtifact, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[artifact(id = "os.collection")]
pub struct CollectionSnapshot {
    pub schema: String,
    pub name: String,
    #[dsl(table)]
    pub folders: Vec<CollectionFolder>,
    // 🧮️ NOT `#[dsl(table)]`: a `#[dsl(statements)]` field (`CollectionEntry.body`) is not a
    // self-delimiting shape, so it cannot be a compact Structure-of-Arrays table COLUMN — only the
    // expanded Array-of-Structs `Shape::List(Record)` form (a full nested record per entry) can carry
    // it. `folders` above has no such field, so it stays compact.
    pub entries: Vec<CollectionEntry>,
}

pub fn empty_collection_snapshot(name: &str) -> CollectionSnapshot {
    CollectionSnapshot { schema: S_COLLECTION_SCHEMA.into(), name: name.into(), folders: Vec::new(), entries: Vec::new() }
}




//#region 🔖️CollectionMutation
/// ⚡️ One settled collection-tree mutation. Folders/entries are id-keyed entities (`create`/`delete`),
/// re-parenting is derivation rule 5's hierarchy verb (`move-to-<container>`, not `change-*` — SMO
/// corrected DKM's first `ChangeFolderParent`/`ChangeEntryFolder` proposal on exactly this point), and
/// `RenameCollection` replaces `SetName` since the target is the document root's identity field.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum)]
pub enum CollectionMutation {
    RenameCollection {
        new_name: String,
    },
    CreateFolder {
        #[dsl(block)]
        folder: CollectionFolder,
        index: u32,
    },
    DeleteFolder {
        folder_id: String,
    },
    MoveToCollection {
        folder_id: String,
        new_parent: Option<String>,
    },
    RenameFolder {
        folder_id: String,
        new_name: String,
    },
    CreateEntry {
        #[dsl(block)]
        entry: CollectionEntry,
        index: u32,
    },
    DeleteEntry {
        entry_id: String,
    },
    MoveToFolder {
        entry_id: String,
        new_folder: Option<String>,
    },
    RenameEntry {
        entry_id: String,
        new_name: String,
    },
    ReplaceEntryBody {
        entry_id: String,
        #[dsl(statements)]
        new_body: Box<ArtifactBody>,
    },
}

//#region 🔖️HandcraftedOpCodecs



//#endregion 🔖️HandcraftedOpCodecs

/// 🌳️ Every folder id in `folder_id`'s subtree, root-first (`folder_id` itself is `ids[0]`), by
/// BFS over `parent_id` links — the cascade `DeleteFolder`'s `diff`/`inverse` both need to know
/// exactly which folders a delete removes.
fn folder_subtree_ids(folders: &[CollectionFolder], folder_id: &str) -> Vec<String> {
    let mut ids = vec![folder_id.to_string()];
    let mut frontier = vec![folder_id.to_string()];
    while let Some(current) = frontier.pop() {
        for folder in folders {
            if folder.parent_id.as_deref() == Some(current.as_str()) {
                ids.push(folder.id.clone());
                frontier.push(folder.id.clone());
            }
        }
    }
    ids
}

/// 🧬️ Sparse collection delta: the collection name is a present slot and folders and entries are positional row deltas
/// (`removed`/`inserted`/`moved`/`patched`, see `protocol::list_delta`), so a cascading folder delete lists every removed subtree folder and entry
/// and its negative delta restores all of them at their original positions.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct CollectionDiff {
    #[value(skip_serializing_if = "Option::is_none")]
    pub renamed_collection: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub folders: Option<CollectionFoldersDelta>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub entries: Option<CollectionEntriesDelta>,
}

/// 🧱️ Carries an optional container link (`parent_id`/`folder_id`) as a present slot, so moving to root stays distinct from leaving the link untouched on every wire.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct CollectionOptionalLink {
    pub value: Option<String>,
}

/// 🩹 Field patch of one folder; every present slot is the new value of exactly that field.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct CollectionFolderPatch {
    pub parent_id: Option<CollectionOptionalLink>,
    pub name: Option<String>,
}

/// 🩹 Field patch of one entry; every present slot is the new value of exactly that field.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct CollectionEntryPatch {
    pub folder_id: Option<CollectionOptionalLink>,
    pub name: Option<String>,
    pub kind_id: Option<String>,
    pub body: Option<Box<ArtifactBody>>,
}

protocol::list_delta! {
    /// 🧩️ Positional row delta of the folders.
    #[derive(Serialize, Deserialize)]
    pub CollectionFoldersDelta { removal: CollectionFolderRemoval, insertion: CollectionFolderInsertion, relocation: CollectionFolderRelocation, modification: CollectionFoldersModification, row: CollectionFolder, patch: CollectionFolderPatch, key: id, values_only }
}

protocol::list_delta! {
    /// 🧩️ Positional row delta of the entries.
    #[derive(Serialize, Deserialize)]
    pub CollectionEntriesDelta { removal: CollectionEntryRemoval, insertion: CollectionEntryInsertion, relocation: CollectionEntryRelocation, modification: CollectionEntriesModification, row: CollectionEntry, patch: CollectionEntryPatch, key: id, values_only }
}






impl protocol::list_delta::RowPatch<CollectionFolder> for CollectionFolderPatch {
    fn commit_into(&self, row: &mut CollectionFolder, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(parent_id) = &self.parent_id {
            row.parent_id = parent_id.value.clone();
        }
        if let Some(name) = &self.name {
            row.name = name.clone();
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        self.parent_id = later.parent_id.or(self.parent_id.take());
        self.name = later.name.or(self.name.take());
    }
    fn inverse(&self, row: &CollectionFolder) -> Self {
        Self { parent_id: self.parent_id.as_ref().map(|_| CollectionOptionalLink { value: row.parent_id.clone() }), name: self.name.as_ref().map(|_| row.name.clone()) }
    }
    fn is_empty(&self) -> bool {
        self.parent_id.is_none() && self.name.is_none()
    }
}

impl protocol::list_delta::RowPatch<CollectionEntry> for CollectionEntryPatch {
    fn commit_into(&self, row: &mut CollectionEntry, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(folder_id) = &self.folder_id {
            row.folder_id = folder_id.value.clone();
        }
        if let Some(name) = &self.name {
            row.name = name.clone();
        }
        if let Some(kind_id) = &self.kind_id {
            row.kind_id = kind_id.clone();
        }
        if let Some(body) = &self.body {
            row.body = body.clone();
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        self.folder_id = later.folder_id.or(self.folder_id.take());
        self.name = later.name.or(self.name.take());
        self.kind_id = later.kind_id.or(self.kind_id.take());
        self.body = later.body.or(self.body.take());
    }
    fn inverse(&self, row: &CollectionEntry) -> Self {
        Self {
                        folder_id: self.folder_id.as_ref().map(|_| CollectionOptionalLink { value: row.folder_id.clone() }),
            name: self.name.as_ref().map(|_| row.name.clone()),
            kind_id: self.kind_id.as_ref().map(|_| row.kind_id.clone()),
            body: self.body.as_ref().map(|_| row.body.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        self.folder_id.is_none() && self.name.is_none() && self.kind_id.is_none() && self.body.is_none()
    }
}

impl protocol::MutationDiff<CollectionSnapshot> for CollectionDiff {
    fn apply(&self, base: &CollectionSnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<CollectionSnapshot> {
        let mut next = base.clone();
        if let Some(name) = &self.renamed_collection {
            next.name = name.clone();
        }
        if let Some(delta) = &self.folders {
            next.folders = delta.commit_onto(&next.folders, capability).map_err(|error| error.under(["folders"]))?;
        }
        if let Some(delta) = &self.entries {
            next.entries = delta.commit_onto(&next.entries, capability).map_err(|error| error.under(["entries"]))?;
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.renamed_collection.is_some() {
            self.renamed_collection = other.renamed_collection;
        }
        self.folders = match (self.folders.take(), other.folders) {
            (Some(mut first), Some(later)) => {
                first.absorb(later);
                Some(first)
            }
            (first, later) => later.or(first),
        };
        self.entries = match (self.entries.take(), other.entries) {
            (Some(mut first), Some(later)) => {
                first.absorb(later);
                Some(first)
            }
            (first, later) => later.or(first),
        };
    }
}

impl protocol::DiffAlgebra<CollectionSnapshot> for CollectionDiff {
    fn inverse(&self, base: &CollectionSnapshot) -> Self {
        Self {
            renamed_collection: self.renamed_collection.as_ref().map(|_| base.name.clone()),
            folders: self.folders.as_ref().map(|delta| delta.inverse(&base.folders)),
            entries: self.entries.as_ref().map(|delta| delta.inverse(&base.entries)),
        }
    }

    fn is_empty(&self) -> bool {
        self.renamed_collection.is_none() && self.folders.as_ref().is_none_or(CollectionFoldersDelta::is_empty) && self.entries.as_ref().is_none_or(CollectionEntriesDelta::is_empty)
    }
}

/// 🧷️ Hand-built `protocol::Mutation::DESCRIPTORS` roster for `CollectionMutation`'s 10 leaves —
/// same reasoning as `SPACE_MUTATION_DESCRIPTORS` above (inline-field variants, not a `MutationLeaf`
/// wrapper, so `#[derive(dsl::Mutations)]` doesn't apply). One entry per variant, declaration order.
const COLLECTION_MUTATION_DESCRIPTORS: &[protocol::MutationLeafDescriptor] = &[
    protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/🧬️schema/🧬️mutations/rename-collection",
        semantic_kind: "rename-collection",
        display_name: "Rename Collection",
        emoji: "✏️",
        aggregate_variant: "RenameCollection",
        payload_schema: S_COLLECTION_SCHEMA,
        text_opcode: Some("rename-collection"),
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::ApplyOnly,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust],
    },
    protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/🧬️schema/🧬️mutations/create-folder",
        semantic_kind: "create-folder",
        display_name: "Create Folder",
        emoji: "📁",
        aggregate_variant: "CreateFolder",
        payload_schema: S_COLLECTION_SCHEMA,
        text_opcode: Some("create-folder"),
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::ApplyOnly,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust],
    },
    protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/🧬️schema/🧬️mutations/delete-folder",
        semantic_kind: "delete-folder",
        display_name: "Delete Folder",
        emoji: "🗑️",
        aggregate_variant: "DeleteFolder",
        payload_schema: S_COLLECTION_SCHEMA,
        text_opcode: Some("delete-folder"),
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::ApplyOnly,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust],
    },
    protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/🧬️schema/🧬️mutations/move-to-collection",
        semantic_kind: "move-to-collection",
        display_name: "Move To Collection",
        emoji: "🚚",
        aggregate_variant: "MoveToCollection",
        payload_schema: S_COLLECTION_SCHEMA,
        text_opcode: Some("move-to-collection"),
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::ApplyOnly,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust],
    },
    protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/🧬️schema/🧬️mutations/rename-folder",
        semantic_kind: "rename-folder",
        display_name: "Rename Folder",
        emoji: "✏️",
        aggregate_variant: "RenameFolder",
        payload_schema: S_COLLECTION_SCHEMA,
        text_opcode: Some("rename-folder"),
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::ApplyOnly,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust],
    },
    protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/🧬️schema/🧬️mutations/create-entry",
        semantic_kind: "create-entry",
        display_name: "Create Entry",
        emoji: "🧾",
        aggregate_variant: "CreateEntry",
        payload_schema: S_COLLECTION_SCHEMA,
        text_opcode: Some("create-entry"),
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::ApplyOnly,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust],
    },
    protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/🧬️schema/🧬️mutations/delete-entry",
        semantic_kind: "delete-entry",
        display_name: "Delete Entry",
        emoji: "🗑️",
        aggregate_variant: "DeleteEntry",
        payload_schema: S_COLLECTION_SCHEMA,
        text_opcode: Some("delete-entry"),
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::ApplyOnly,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust],
    },
    protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/🧬️schema/🧬️mutations/move-to-folder",
        semantic_kind: "move-to-folder",
        display_name: "Move To Folder",
        emoji: "🚚",
        aggregate_variant: "MoveToFolder",
        payload_schema: S_COLLECTION_SCHEMA,
        text_opcode: Some("move-to-folder"),
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::ApplyOnly,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust],
    },
    protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/🧬️schema/🧬️mutations/rename-entry",
        semantic_kind: "rename-entry",
        display_name: "Rename Entry",
        emoji: "✏️",
        aggregate_variant: "RenameEntry",
        payload_schema: S_COLLECTION_SCHEMA,
        text_opcode: Some("rename-entry"),
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::ApplyOnly,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust],
    },
    protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/🧬️schema/🧬️mutations/replace-entry-body",
        semantic_kind: "replace-entry-body",
        display_name: "Replace Entry Body",
        emoji: "📦",
        aggregate_variant: "ReplaceEntryBody",
        payload_schema: S_COLLECTION_SCHEMA,
        text_opcode: Some("replace-entry-body"),
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::ApplyOnly,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust],
    },
];

impl protocol::Mutation<CollectionSnapshot> for CollectionMutation {
    type Diff = CollectionDiff;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = COLLECTION_MUTATION_DESCRIPTORS;

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        let index = match self {
            CollectionMutation::RenameCollection { .. } => 0,
            CollectionMutation::CreateFolder { .. } => 1,
            CollectionMutation::DeleteFolder { .. } => 2,
            CollectionMutation::MoveToCollection { .. } => 3,
            CollectionMutation::RenameFolder { .. } => 4,
            CollectionMutation::CreateEntry { .. } => 5,
            CollectionMutation::DeleteEntry { .. } => 6,
            CollectionMutation::MoveToFolder { .. } => 7,
            CollectionMutation::RenameEntry { .. } => 8,
            CollectionMutation::ReplaceEntryBody { .. } => 9,
        };
        &Self::DESCRIPTORS[index]
    }

    /// 🧮️ Mechanical wrap only (26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-
    /// CONFLICTS W0): no `Error`/`Warning`/`Fatal` messages added here yet — that is the W3
    /// fan-out's job per verb family.
    fn diff(&self, base: &CollectionSnapshot) -> protocol::MutationOutcome<CollectionDiff> {
        let missing = |what: &str, id: &str| protocol::MutationOutcome::error("mutation.target-missing", format!("{what} {id} does not exist."), [id.to_string()]);
        let folder_exists = |id: &String| base.folders.iter().any(|folder| &folder.id == id);
        let entry_exists = |id: &String| base.entries.iter().any(|entry| &entry.id == id);
        let folders = |delta: CollectionFoldersDelta| protocol::MutationOutcome::new(CollectionDiff { folders: Some(delta), ..Default::default() });
        let entries = |delta: CollectionEntriesDelta| protocol::MutationOutcome::new(CollectionDiff { entries: Some(delta), ..Default::default() });
        match self {
            CollectionMutation::RenameCollection { new_name } => protocol::MutationOutcome::new(CollectionDiff { renamed_collection: Some(new_name.clone()), ..Default::default() }),
            CollectionMutation::CreateFolder { folder, index } => folders(CollectionFoldersDelta::insertion((*index as usize).min(base.folders.len()), folder.clone())),
            CollectionMutation::DeleteFolder { folder_id } if folder_exists(folder_id) => {
                let cascade_folder_ids = folder_subtree_ids(&base.folders, folder_id);
                let cascade_entry_ids: Vec<String> = base.entries.iter().filter(|entry| entry.folder_id.as_deref().is_some_and(|folder_id| cascade_folder_ids.iter().any(|id| id == folder_id))).map(|entry| entry.id.clone()).collect();
                let folder_indices: Vec<usize> = base.folders.iter().enumerate().filter(|(_, folder)| cascade_folder_ids.contains(&folder.id)).map(|(at, _)| at).collect();
                let entry_indices: Vec<usize> = base.entries.iter().enumerate().filter(|(_, entry)| cascade_entry_ids.contains(&entry.id)).map(|(at, _)| at).collect();
                protocol::MutationOutcome::new(CollectionDiff {
                    folders: Some(CollectionFoldersDelta::removals(&base.folders, &folder_indices)),
                    entries: (!entry_indices.is_empty()).then(|| CollectionEntriesDelta::removals(&base.entries, &entry_indices)),
                    ..Default::default()
                })
            }
            CollectionMutation::DeleteFolder { folder_id } => missing("Folder", folder_id),
            CollectionMutation::MoveToCollection { folder_id, new_parent } if folder_exists(folder_id) => {
                folders(CollectionFoldersDelta::modification(folder_id.clone(), CollectionFolderPatch { parent_id: Some(CollectionOptionalLink { value: new_parent.clone() }), name: None }))
            }
            CollectionMutation::MoveToCollection { folder_id, .. } => missing("Folder", folder_id),
            CollectionMutation::RenameFolder { folder_id, new_name } if folder_exists(folder_id) => {
                folders(CollectionFoldersDelta::modification(folder_id.clone(), CollectionFolderPatch { parent_id: None, name: Some(new_name.clone()) }))
            }
            CollectionMutation::RenameFolder { folder_id, .. } => missing("Folder", folder_id),
            CollectionMutation::CreateEntry { entry, index } => entries(CollectionEntriesDelta::insertion((*index as usize).min(base.entries.len()), entry.clone())),
            CollectionMutation::DeleteEntry { entry_id } => match base.entries.iter().position(|entry| &entry.id == entry_id) {
                Some(at) => entries(CollectionEntriesDelta::removal(&base.entries, at)),
                None => missing("Entry", entry_id),
            },
            CollectionMutation::MoveToFolder { entry_id, new_folder } if entry_exists(entry_id) => {
                entries(CollectionEntriesDelta::modification(entry_id.clone(), CollectionEntryPatch { folder_id: Some(CollectionOptionalLink { value: new_folder.clone() }), ..Default::default() }))
            }
            CollectionMutation::MoveToFolder { entry_id, .. } => missing("Entry", entry_id),
            CollectionMutation::RenameEntry { entry_id, new_name } if entry_exists(entry_id) => {
                entries(CollectionEntriesDelta::modification(entry_id.clone(), CollectionEntryPatch { name: Some(new_name.clone()), ..Default::default() }))
            }
            CollectionMutation::RenameEntry { entry_id, .. } => missing("Entry", entry_id),
            CollectionMutation::ReplaceEntryBody { entry_id, new_body } if entry_exists(entry_id) => {
                entries(CollectionEntriesDelta::modification(entry_id.clone(), CollectionEntryPatch { body: Some(new_body.clone()), ..Default::default() }))
            }
            CollectionMutation::ReplaceEntryBody { entry_id, .. } => missing("Entry", entry_id),
        }
    }

    fn inverse(&self, base: &CollectionSnapshot) -> Result<Vec<Self>, semio_framework_value::ValueError> {
    Ok((|| {
        match self {
            CollectionMutation::RenameCollection { .. } => vec![CollectionMutation::RenameCollection { new_name: base.name.clone() }],
            CollectionMutation::CreateFolder { folder, .. } => vec![CollectionMutation::DeleteFolder { folder_id: folder.id.clone() }],
            CollectionMutation::DeleteFolder { folder_id } => {
                if !base.folders.iter().any(|folder| &folder.id == folder_id) {
                    return Vec::new();
                }
                let cascade_folder_ids = folder_subtree_ids(&base.folders, folder_id);
                let mut mutations = Vec::new();
                for (at, folder) in base.folders.iter().enumerate() {
                    if cascade_folder_ids.contains(&folder.id) {
                        mutations.push(CollectionMutation::CreateFolder { folder: folder.clone(), index: at as u32 });
                    }
                }
                for (at, entry) in base.entries.iter().enumerate() {
                    if entry.folder_id.as_deref().is_some_and(|folder_id| cascade_folder_ids.iter().any(|id| id == folder_id)) {
                        mutations.push(CollectionMutation::CreateEntry { entry: entry.clone(), index: at as u32 });
                    }
                }
                mutations
            }
            CollectionMutation::MoveToCollection { folder_id, .. } => {
                base.folders.iter().find(|folder| &folder.id == folder_id).map(|folder| vec![CollectionMutation::MoveToCollection { folder_id: folder_id.clone(), new_parent: folder.parent_id.clone() }]).unwrap_or_default()
            }
            CollectionMutation::RenameFolder { folder_id, .. } => {
                base.folders.iter().find(|folder| &folder.id == folder_id).map(|folder| vec![CollectionMutation::RenameFolder { folder_id: folder_id.clone(), new_name: folder.name.clone() }]).unwrap_or_default()
            }
            CollectionMutation::CreateEntry { entry, .. } => vec![CollectionMutation::DeleteEntry { entry_id: entry.id.clone() }],
            CollectionMutation::DeleteEntry { entry_id } => base.entries.iter().position(|entry| &entry.id == entry_id).map(|at| vec![CollectionMutation::CreateEntry { entry: base.entries[at].clone(), index: at as u32 }]).unwrap_or_default(),
            CollectionMutation::MoveToFolder { entry_id, .. } => {
                base.entries.iter().find(|entry| &entry.id == entry_id).map(|entry| vec![CollectionMutation::MoveToFolder { entry_id: entry_id.clone(), new_folder: entry.folder_id.clone() }]).unwrap_or_default()
            }
            CollectionMutation::RenameEntry { entry_id, .. } => {
                base.entries.iter().find(|entry| &entry.id == entry_id).map(|entry| vec![CollectionMutation::RenameEntry { entry_id: entry_id.clone(), new_name: entry.name.clone() }]).unwrap_or_default()
            }
            CollectionMutation::ReplaceEntryBody { entry_id, .. } => {
                base.entries.iter().find(|entry| &entry.id == entry_id).map(|entry| vec![CollectionMutation::ReplaceEntryBody { entry_id: entry_id.clone(), new_body: entry.body.clone() }]).unwrap_or_default()
            }
        }
    
    })())
}
}
//#endregion 🔖️CollectionMutation
//#endregion 🔖️Collection
/// 🌳️ Which folder ids are cyclic (each folder's own id is in the returned set exactly when walking
/// its `parent_id` chain eventually revisits it).
fn folders_in_cycle(folders: &[CollectionFolder]) -> HashSet<String> {
    let parents: HashMap<&str, Option<&str>> = folders.iter().map(|folder| (folder.id.as_str(), folder.parent_id.as_deref())).collect();
    let mut in_cycle = HashSet::new();
    for folder in folders {
        let mut path: Vec<&str> = Vec::new();
        let mut current = folder.id.as_str();
        loop {
            if let Some(position) = path.iter().position(|id| *id == current) {
                for id in &path[position..] {
                    in_cycle.insert((*id).to_string());
                }
                break;
            }
            path.push(current);
            match parents.get(current).copied().flatten() {
                Some(parent) => current = parent,
                None => break,
            }
            if path.len() > folders.len() + 1 {
                break;
            }
        }
    }
    in_cycle
}

fn dedupe_folder_names(folders: &mut [CollectionFolder], messages: &mut Vec<protocol::MutationMessage>) {
    let mut seen: HashSet<(Option<String>, String)> = HashSet::new();
    for folder in folders.iter_mut() {
        let mut key = (folder.parent_id.clone(), folder.name.clone());
        if seen.contains(&key) {
            let mut suffix = 2u32;
            let mut candidate = format!("{} ({suffix})", folder.name);
            while seen.contains(&(folder.parent_id.clone(), candidate.clone())) {
                suffix += 1;
                candidate = format!("{} ({suffix})", folder.name);
            }
            messages.push(protocol::MutationMessage::info("mutation.cascade", format!("folder {} renamed to '{candidate}' to avoid a sibling name collision", folder.id)).at(vec!["collection/folder-name-collision".to_string(), folder.id.clone()]));
            folder.name = candidate.clone();
            key = (folder.parent_id.clone(), candidate);
        }
        seen.insert(key);
    }
}

fn dedupe_entry_names(entries: &mut [CollectionEntry], messages: &mut Vec<protocol::MutationMessage>) {
    let mut seen: HashSet<(Option<String>, String)> = HashSet::new();
    for entry in entries.iter_mut() {
        let mut key = (entry.folder_id.clone(), entry.name.clone());
        if seen.contains(&key) {
            let mut suffix = 2u32;
            let mut candidate = format!("{} ({suffix})", entry.name);
            while seen.contains(&(entry.folder_id.clone(), candidate.clone())) {
                suffix += 1;
                candidate = format!("{} ({suffix})", entry.name);
            }
            messages.push(protocol::MutationMessage::info("mutation.cascade", format!("entry {} renamed to '{candidate}' to avoid a sibling name collision", entry.id)).at(vec!["collection/entry-name-collision".to_string(), entry.id.clone()]));
            entry.name = candidate.clone();
            key = (entry.folder_id.clone(), candidate);
        }
        seen.insert(key);
    }
}

/// 🤝️ Post-materialization collection integrity pass, run in order: (1) a folder whose `parent_id`
/// references a missing folder reparents to root, (2) a folder participating in a parent cycle is cut
/// to root, (3) sibling folders/entries with a name collision (same parent) get a numeric suffix, (4)
/// an entry whose `folder_id` references a missing folder moves to root. Each rule operates on the
/// state the previous one produced — mirrors os-core's `reconcile_os_workflow` ordered-rules style.
pub fn reconcile_collection_integrity(mut snapshot: CollectionSnapshot) -> (CollectionSnapshot, Vec<protocol::MutationMessage>) {
    let mut messages = Vec::new();

    //#region OrphanFolderReparent
    let folder_ids: HashSet<String> = snapshot.folders.iter().map(|folder| folder.id.clone()).collect();
    for folder in &mut snapshot.folders {
        if let Some(parent_id) = &folder.parent_id {
            if !folder_ids.contains(parent_id) {
                messages.push(protocol::MutationMessage::warning("mutation.clamped", format!("folder {} referenced missing parent {parent_id}; reparented to root", folder.id)).at(vec!["collection/folder-orphaned".to_string(), folder.id.clone()]));
                folder.parent_id = None;
            }
        }
    }
    //#endregion OrphanFolderReparent

    //#region FolderCycleCut
    let cyclic = folders_in_cycle(&snapshot.folders);
    for folder in &mut snapshot.folders {
        if cyclic.contains(&folder.id) {
            messages.push(protocol::MutationMessage::warning("mutation.clamped", format!("folder {} participates in a parent cycle; cut to root", folder.id)).at(vec!["collection/folder-cycle".to_string(), folder.id.clone()]));
            folder.parent_id = None;
        }
    }
    //#endregion FolderCycleCut

    //#region SiblingNameCollisionSuffix
    dedupe_folder_names(&mut snapshot.folders, &mut messages);
    dedupe_entry_names(&mut snapshot.entries, &mut messages);
    //#endregion SiblingNameCollisionSuffix

    //#region EntryMissingFolderReparent
    let folder_ids: HashSet<String> = snapshot.folders.iter().map(|folder| folder.id.clone()).collect();
    for entry in &mut snapshot.entries {
        if let Some(folder_id) = &entry.folder_id {
            if !folder_ids.contains(folder_id) {
                messages.push(protocol::MutationMessage::warning("mutation.clamped", format!("entry {} referenced missing folder {folder_id}; moved to root", entry.id)).at(vec!["collection/entry-folder-missing".to_string(), entry.id.clone()]));
                entry.folder_id = None;
            }
        }
    }
    //#endregion EntryMissingFolderReparent

    (snapshot, messages)
}

/// 🧵️ Root-to-leaf folder path (slash-joined names), or `None` if `folder_id` is absent or its parent
/// chain is cyclic (a resolver never persists — never trust it over reconciled data with a real cycle).
pub fn folder_path(collection: &CollectionSnapshot, folder_id: &str) -> Option<String> {
    let by_id: HashMap<&str, &CollectionFolder> = collection.folders.iter().map(|folder| (folder.id.as_str(), folder)).collect();
    let mut segments: Vec<String> = Vec::new();
    let mut current = folder_id.to_string();
    let mut guard = 0usize;
    loop {
        let folder = by_id.get(current.as_str())?;
        segments.push(folder.name.clone());
        guard += 1;
        if guard > collection.folders.len() {
            return None;
        }
        match &folder.parent_id {
            Some(parent) => current = parent.clone(),
            None => break,
        }
    }
    segments.reverse();
    Some(segments.join("/"))
}

/// 🧵️ Full slash-joined path to an entry (its folder path plus its own name), or `None` if the entry
/// doesn't exist or its folder chain doesn't resolve.
pub fn entry_path(collection: &CollectionSnapshot, entry_id: &str) -> Option<String> {
    let entry = collection.entries.iter().find(|entry| entry.id == entry_id)?;
    let prefix = match &entry.folder_id {
        Some(folder_id) => folder_path(collection, folder_id)?,
        None => String::new(),
    };
    Some(if prefix.is_empty() { entry.name.clone() } else { format!("{prefix}/{}", entry.name) })
}

/// 🔎️ Resolves a slash-joined path to its `CollectionEntry`, pure over the live snapshot — moves
/// and renames never break a persisted ref because paths are never persisted, only ids (see
/// `🔖️Addressing`).
pub fn resolve_entry_by_path<'a>(collection: &'a CollectionSnapshot, path: &str) -> Option<&'a CollectionEntry> {
    collection.entries.iter().find(|entry| entry_path(collection, &entry.id).as_deref() == Some(path))
}
/// 🔗️ `space://<space_id>/collection/<collection_id>`.
pub fn collection_backbone_uri(space_id: &str, collection_id: &str) -> String {
    format!("space://{space_id}/collection/{collection_id}")
}

/// 🔗️ `space://<space_id>/artifact/<artifact_id>`.
pub fn artifact_backbone_uri(space_id: &str, artifact_id: &str) -> String {
    format!("space://{space_id}/artifact/{artifact_id}")
}

#[path = "🧬️schema/📦️package/🦀️.rs"]
pub mod package;
pub use package::{admit_collection_package_declaration, package_descriptor, CollectionArtifactPackage, CollectionPackageDeclaration, CollectionPackageError};

#[cfg(test)]
#[path = "🧪️tests/🗂️collection/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path="🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod sqlite_snapshot_baseline;

#[path = "🚪️io/🦀️.rs"]
pub mod io;
