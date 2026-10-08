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

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

//#region 🔖️Collection
pub const S_COLLECTION_SCHEMA: &str = "os.collection";

/// 📁️ One parent-linked folder in a collection's flat tree. `parent_id: None` means root.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
pub struct CollectionEntry {
    pub id: String,
    pub folder_id: Option<String>,
    pub name: String,
    pub kind_id: String,
    #[dsl(statements)]
    pub body: Box<ArtifactBody>,
}

/// 🗂️ A collection's flat parent-linked folder tree plus its artifact entries.
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, semio_framework_os_kernel::DslArtifact)]
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

/// 🧬️ Sparse collection delta: the collection name is a present slot and folders and entries are id-keyed row deltas
/// (`added`/`removed`/`patched`/`reordered`), so a cascading folder delete lists every removed subtree folder and entry
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct CollectionOptionalLink {
    pub value: Option<String>,
}

/// 🩹 Field patch of one folder; every present slot is the new value of exactly that field.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct CollectionFolderPatch {
    pub id: String,
    pub parent_id: Option<CollectionOptionalLink>,
    pub name: Option<String>,
}

/// 🩹 Field patch of one entry; every present slot is the new value of exactly that field.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct CollectionEntryPatch {
    pub id: String,
    pub folder_id: Option<CollectionOptionalLink>,
    pub name: Option<String>,
    pub kind_id: Option<String>,
    pub body: Option<Box<ArtifactBody>>,
}

/// 🧩️ Id-keyed row delta of the folders.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct CollectionFoldersDelta {
    pub added: Vec<CollectionFolder>,
    pub removed: Vec<String>,
    pub patched: Vec<CollectionFolderPatch>,
    pub reordered: Option<Vec<String>>,
}

/// 🧩️ Id-keyed row delta of the entries.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct CollectionEntriesDelta {
    pub added: Vec<CollectionEntry>,
    pub removed: Vec<String>,
    pub patched: Vec<CollectionEntryPatch>,
    pub reordered: Option<Vec<String>>,
}

//#region 🧺️KeyedDelta
/// 🧺️ Id-keyed ordered-collection delta (`added`/`removed`/`patched`/`reordered`) and its algebra: apply, composition (create∘delete
/// cancels, delete∘create replaces, patch∘patch composes, patch∘create folds), negative delta and state delta.
pub trait KeyedDelta: Sized {
    type Row: Clone;
    type Patch: Clone;
    fn added(&self) -> &[Self::Row];
    fn removed(&self) -> &[String];
    fn patched(&self) -> &[Self::Patch];
    fn reordered(&self) -> Option<&[String]>;
    fn assemble(added: Vec<Self::Row>, removed: Vec<String>, patched: Vec<Self::Patch>, reordered: Option<Vec<String>>) -> Self;
    fn row_key(row: &Self::Row) -> &str;
    fn patch_key(patch: &Self::Patch) -> &str;
    fn patch_fold(patch: &Self::Patch, row: &mut Self::Row) -> Result<(), protocol::MutationApplyError>;
    fn patch_compose(first: &Self::Patch, later: &Self::Patch) -> Self::Patch;
    fn patch_inverse(patch: &Self::Patch, base: &Self::Row) -> Self::Patch;
    fn patch_between(base: &Self::Row, other: &Self::Row) -> Option<Self::Patch>;
    fn patch_is_empty(patch: &Self::Patch) -> bool;
}

fn keyed_error(code: &str, message: &str, at: [&str; 2]) -> protocol::MutationApplyError {
    protocol::MutationApplyError::new(code, message).at(at)
}

pub fn keyed_apply<D: KeyedDelta>(rows: &[D::Row], delta: &D) -> Result<Vec<D::Row>, protocol::MutationApplyError> {
    let key = D::row_key;
    for (index, id) in delta.removed().iter().enumerate() {
        if delta.removed()[..index].contains(id) {
            return Err(keyed_error("mutation.apply.duplicate-target", "row is removed more than once", ["removed", &index.to_string()]));
        }
        if !rows.iter().any(|row| key(row) == id) {
            return Err(keyed_error("mutation.apply.missing-target", "removed row does not exist", ["removed", &index.to_string()]));
        }
    }
    let mut next: Vec<D::Row> = rows.iter().filter(|row| !delta.removed().iter().any(|id| id == key(row))).cloned().collect();
    for (index, row) in delta.added().iter().enumerate() {
        if next.iter().any(|existing| key(existing) == key(row)) {
            return Err(keyed_error("mutation.apply.duplicate-target", "added row identity already exists", ["added", &index.to_string()]));
        }
        next.push(row.clone());
    }
    for (index, patch) in delta.patched().iter().enumerate() {
        if delta.patched()[..index].iter().any(|earlier| D::patch_key(earlier) == D::patch_key(patch)) {
            return Err(keyed_error("mutation.apply.duplicate-target", "row is patched more than once", ["patched", &index.to_string()]));
        }
        let row = next.iter_mut().find(|row| key(row) == D::patch_key(patch)).ok_or_else(|| keyed_error("mutation.apply.missing-target", "patched row does not exist", ["patched", &index.to_string()]))?;
        D::patch_fold(patch, row).map_err(|error| error.under(["patched".to_string(), index.to_string()]))?;
    }
    let Some(order) = delta.reordered() else { return Ok(next) };
    if order.len() != next.len() || order.iter().enumerate().any(|(index, id)| order[..index].contains(id) || !next.iter().any(|row| key(row) == id)) {
        return Err(protocol::MutationApplyError::new("mutation.apply.invalid-order", "reorder must be a complete unique permutation").at(["reordered".to_string()]));
    }
    Ok(order.iter().filter_map(|id| next.iter().find(|row| key(row) == id).cloned()).collect())
}

fn canonical<D: KeyedDelta>(mut added: Vec<D::Row>, mut removed: Vec<String>, mut patched: Vec<D::Patch>, reordered: Option<Vec<String>>) -> D {
    if let Some(order) = &reordered {
        added.sort_by_key(|row| order.iter().position(|id| id == D::row_key(row)).unwrap_or(usize::MAX));
    }
    let reordered = reordered.filter(|order| {
        let tail = added.len();
        !(order.len() <= tail + 1 && order.len() >= tail && order[order.len() - tail..].iter().map(String::as_str).eq(added.iter().map(D::row_key)))
    });
    removed.sort();
    removed.dedup();
    patched.retain(|patch| !D::patch_is_empty(patch));
    patched.sort_by(|left, right| D::patch_key(left).cmp(D::patch_key(right)));
    D::assemble(added, removed, patched, reordered)
}

/// ➕️ Normal form of `first` then `later`: create∘delete cancels, delete∘create replaces, patch∘patch composes, patch∘create folds.
pub fn keyed_absorb<D: KeyedDelta>(first: &D, later: &D) -> D {
    let mut added: Vec<D::Row> = first.added().to_vec();
    let mut removed: Vec<String> = first.removed().to_vec();
    let mut patched: Vec<D::Patch> = Vec::new();
    for patch in first.patched() {
        match added.iter_mut().find(|row| D::row_key(row) == D::patch_key(patch)) {
            Some(row) => {
                let _ = D::patch_fold(patch, row);
            }
            None => patched.push(patch.clone()),
        }
    }
    for id in later.removed() {
        if let Some(position) = added.iter().position(|row| D::row_key(row) == id) {
            added.remove(position);
        } else {
            patched.retain(|patch| D::patch_key(patch) != id);
            if !removed.contains(id) {
                removed.push(id.clone());
            }
        }
    }
    added.extend(later.added().iter().cloned());
    for patch in later.patched() {
        let key = D::patch_key(patch);
        if let Some(row) = added.iter_mut().find(|row| D::row_key(row) == key) {
            let _ = D::patch_fold(patch, row);
        } else if let Some(existing) = patched.iter_mut().find(|existing| D::patch_key(existing) == key) {
            *existing = D::patch_compose(existing, patch);
        } else {
            patched.push(patch.clone());
        }
    }
    let reordered = match (later.reordered(), first.reordered()) {
        (Some(order), _) => Some(order.to_vec()),
        (None, Some(order)) => Some(order.iter().filter(|id| !later.removed().contains(id)).cloned().chain(later.added().iter().map(|row| D::row_key(row).to_string())).collect()),
        (None, None) => None,
    };
    canonical::<D>(added, removed, patched, reordered)
}

fn ids_after<D: KeyedDelta>(base: &[String], delta: &D) -> Vec<String> {
    match delta.reordered() {
        Some(order) => order.to_vec(),
        None => base.iter().filter(|id| !delta.removed().contains(id)).cloned().chain(delta.added().iter().map(|row| D::row_key(row).to_string())).collect(),
    }
}

/// 🔁️ The negative delta: removes what `delta` added, restores what it removed, undoes its patches, restores the base order.
pub fn keyed_inverse<D: KeyedDelta>(delta: &D, base: &[D::Row]) -> D {
    let base_ids: Vec<String> = base.iter().map(|row| D::row_key(row).to_string()).collect();
    let removed: Vec<String> = delta.added().iter().map(|row| D::row_key(row).to_string()).collect();
    let added: Vec<D::Row> = delta.removed().iter().filter_map(|id| base.iter().find(|row| D::row_key(row) == id).cloned()).collect();
    let patched: Vec<D::Patch> = delta
        .patched()
        .iter()
        .filter(|patch| !removed.iter().any(|id| id == D::patch_key(patch)))
        .filter_map(|patch| base.iter().find(|row| D::row_key(row) == D::patch_key(patch)).map(|row| D::patch_inverse(patch, row)))
        .collect();
    let after = ids_after(&base_ids, delta);
    let natural: Vec<String> = after.iter().filter(|id| !removed.contains(id)).cloned().chain(added.iter().map(|row| D::row_key(row).to_string())).collect();
    let reordered = (natural != base_ids).then_some(base_ids);
    canonical::<D>(added, removed, patched, reordered)
}

/// 🧭️ The delta turning `base` into `other` (sync/import only).
pub fn keyed_between<D: KeyedDelta>(base: &[D::Row], other: &[D::Row]) -> D {
    let base_ids: Vec<String> = base.iter().map(|row| D::row_key(row).to_string()).collect();
    let other_ids: Vec<String> = other.iter().map(|row| D::row_key(row).to_string()).collect();
    let removed: Vec<String> = base_ids.iter().filter(|id| !other_ids.contains(id)).cloned().collect();
    let added: Vec<D::Row> = other.iter().filter(|row| !base_ids.iter().any(|id| id == D::row_key(row))).cloned().collect();
    let patched: Vec<D::Patch> = other.iter().filter_map(|row| base.iter().find(|candidate| D::row_key(candidate) == D::row_key(row)).and_then(|candidate| D::patch_between(candidate, row))).collect();
    let natural: Vec<String> = base_ids.iter().filter(|id| !removed.contains(id)).cloned().chain(added.iter().map(|row| D::row_key(row).to_string())).collect();
    let reordered = (natural != other_ids).then_some(other_ids);
    canonical::<D>(added, removed, patched, reordered)
}

pub fn keyed_is_empty<D: KeyedDelta>(delta: &D) -> bool {
    delta.added().is_empty() && delta.removed().is_empty() && delta.patched().iter().all(D::patch_is_empty) && delta.reordered().is_none()
}
//#endregion 🧺️KeyedDelta

macro_rules! keyed_delta_impl {
    ($delta:ident, $row:ty, $patch:ty, $row_key:ident, $patch_key:ident, $fold:expr, $compose:expr, $inverse:expr, $patch_between:expr, $is_empty:expr) => {
        impl KeyedDelta for $delta {
            type Row = $row;
            type Patch = $patch;
            fn added(&self) -> &[$row] {
                &self.added
            }
            fn removed(&self) -> &[String] {
                &self.removed
            }
            fn patched(&self) -> &[$patch] {
                &self.patched
            }
            fn reordered(&self) -> Option<&[String]> {
                self.reordered.as_deref()
            }
            fn assemble(added: Vec<$row>, removed: Vec<String>, patched: Vec<$patch>, reordered: Option<Vec<String>>) -> Self {
                Self { added, removed, patched, reordered }
            }
            fn row_key(row: &$row) -> &str {
                row.$row_key.as_str()
            }
            fn patch_key(patch: &$patch) -> &str {
                patch.$patch_key.as_str()
            }
            fn patch_fold(patch: &$patch, row: &mut $row) -> Result<(), protocol::MutationApplyError> {
                ($fold)(patch, row);
                Ok(())
            }
            fn patch_compose(first: &$patch, later: &$patch) -> $patch {
                ($compose)(first, later)
            }
            fn patch_inverse(patch: &$patch, base: &$row) -> $patch {
                ($inverse)(patch, base)
            }
            fn patch_between(base: &$row, other: &$row) -> Option<$patch> {
                ($patch_between)(base, other)
            }
            fn patch_is_empty(patch: &$patch) -> bool {
                ($is_empty)(patch)
            }
        }
    };
}

keyed_delta_impl!(
    CollectionFoldersDelta,
    CollectionFolder,
    CollectionFolderPatch,
    id,
    id,
    |patch: &CollectionFolderPatch, row: &mut CollectionFolder| {
        if let Some(parent_id) = &patch.parent_id {
            row.parent_id = parent_id.value.clone();
        }
        if let Some(name) = &patch.name {
            row.name = name.clone();
        }
    },
    |first: &CollectionFolderPatch, later: &CollectionFolderPatch| CollectionFolderPatch { id: first.id.clone(), parent_id: later.parent_id.clone().or_else(|| first.parent_id.clone()), name: later.name.clone().or_else(|| first.name.clone()) },
    |patch: &CollectionFolderPatch, base: &CollectionFolder| CollectionFolderPatch { id: patch.id.clone(), parent_id: patch.parent_id.as_ref().map(|_| CollectionOptionalLink { value: base.parent_id.clone() }), name: patch.name.as_ref().map(|_| base.name.clone()) },
    |base: &CollectionFolder, other: &CollectionFolder| {
        let patch = CollectionFolderPatch { id: other.id.clone(), parent_id: (base.parent_id != other.parent_id).then(|| CollectionOptionalLink { value: other.parent_id.clone() }), name: (base.name != other.name).then(|| other.name.clone()) };
        (patch.parent_id.is_some() || patch.name.is_some()).then_some(patch)
    },
    |patch: &CollectionFolderPatch| patch.parent_id.is_none() && patch.name.is_none()
);

keyed_delta_impl!(
    CollectionEntriesDelta,
    CollectionEntry,
    CollectionEntryPatch,
    id,
    id,
    |patch: &CollectionEntryPatch, row: &mut CollectionEntry| {
        if let Some(folder_id) = &patch.folder_id {
            row.folder_id = folder_id.value.clone();
        }
        if let Some(name) = &patch.name {
            row.name = name.clone();
        }
        if let Some(kind_id) = &patch.kind_id {
            row.kind_id = kind_id.clone();
        }
        if let Some(body) = &patch.body {
            row.body = body.clone();
        }
    },
    |first: &CollectionEntryPatch, later: &CollectionEntryPatch| CollectionEntryPatch {
        id: first.id.clone(),
        folder_id: later.folder_id.clone().or_else(|| first.folder_id.clone()),
        name: later.name.clone().or_else(|| first.name.clone()),
        kind_id: later.kind_id.clone().or_else(|| first.kind_id.clone()),
        body: later.body.clone().or_else(|| first.body.clone()),
    },
    |patch: &CollectionEntryPatch, base: &CollectionEntry| CollectionEntryPatch {
        id: patch.id.clone(),
        folder_id: patch.folder_id.as_ref().map(|_| CollectionOptionalLink { value: base.folder_id.clone() }),
        name: patch.name.as_ref().map(|_| base.name.clone()),
        kind_id: patch.kind_id.as_ref().map(|_| base.kind_id.clone()),
        body: patch.body.as_ref().map(|_| base.body.clone()),
    },
    |base: &CollectionEntry, other: &CollectionEntry| {
        let patch = CollectionEntryPatch {
            id: other.id.clone(),
            folder_id: (base.folder_id != other.folder_id).then(|| CollectionOptionalLink { value: other.folder_id.clone() }),
            name: (base.name != other.name).then(|| other.name.clone()),
            kind_id: (base.kind_id != other.kind_id).then(|| other.kind_id.clone()),
            body: (base.body != other.body).then(|| other.body.clone()),
        };
        (patch.folder_id.is_some() || patch.name.is_some() || patch.kind_id.is_some() || patch.body.is_some()).then_some(patch)
    },
    |patch: &CollectionEntryPatch| patch.folder_id.is_none() && patch.name.is_none() && patch.kind_id.is_none() && patch.body.is_none()
);

impl protocol::MutationDiff<CollectionSnapshot> for CollectionDiff {
    fn apply(&self, base: &CollectionSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<CollectionSnapshot> {
        let mut next = base.clone();
        if let Some(name) = &self.renamed_collection {
            next.name = name.clone();
        }
        if let Some(delta) = &self.folders {
            next.folders = keyed_apply(&next.folders, delta).map_err(|error| error.under(["folders"]))?;
        }
        if let Some(delta) = &self.entries {
            next.entries = keyed_apply(&next.entries, delta).map_err(|error| error.under(["entries"]))?;
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.renamed_collection.is_some() {
            self.renamed_collection = other.renamed_collection;
        }
        self.folders = match (self.folders.take(), other.folders) {
            (Some(first), Some(later)) => Some(keyed_absorb(&first, &later)),
            (first, later) => later.or(first),
        };
        self.entries = match (self.entries.take(), other.entries) {
            (Some(first), Some(later)) => Some(keyed_absorb(&first, &later)),
            (first, later) => later.or(first),
        };
    }
}

impl protocol::DiffAlgebra<CollectionSnapshot> for CollectionDiff {
    fn inverse(&self, base: &CollectionSnapshot) -> Self {
        Self {
            renamed_collection: self.renamed_collection.as_ref().map(|_| base.name.clone()),
            folders: self.folders.as_ref().map(|delta| keyed_inverse(delta, &base.folders)),
            entries: self.entries.as_ref().map(|delta| keyed_inverse(delta, &base.entries)),
        }
    }

    fn between(base: &CollectionSnapshot, other: &CollectionSnapshot) -> Self {
        let folders = keyed_between::<CollectionFoldersDelta>(&base.folders, &other.folders);
        let entries = keyed_between::<CollectionEntriesDelta>(&base.entries, &other.entries);
        Self { renamed_collection: (base.name != other.name).then(|| other.name.clone()), folders: (!keyed_is_empty(&folders)).then_some(folders), entries: (!keyed_is_empty(&entries)).then_some(entries) }
    }

    fn is_empty(&self) -> bool {
        self.renamed_collection.is_none() && self.folders.as_ref().is_none_or(keyed_is_empty) && self.entries.as_ref().is_none_or(keyed_is_empty)
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
        let order = |ids: Vec<String>, id: &str, index: u32| {
            let mut order: Vec<String> = ids.into_iter().filter(|existing| existing != id).collect();
            order.insert((index as usize).min(order.len()), id.to_string());
            order
        };
        match self {
            CollectionMutation::RenameCollection { new_name } => protocol::MutationOutcome::new(CollectionDiff { renamed_collection: Some(new_name.clone()), ..Default::default() }),
            CollectionMutation::CreateFolder { folder, index } => {
                let natural = *index as usize >= base.folders.len();
                folders(CollectionFoldersDelta {
                    added: vec![folder.clone()],
                    reordered: (!natural).then(|| order(base.folders.iter().map(|existing| existing.id.clone()).collect(), &folder.id, *index)),
                    ..Default::default()
                })
            }
            CollectionMutation::DeleteFolder { folder_id } if folder_exists(folder_id) => {
                let cascade_folder_ids = folder_subtree_ids(&base.folders, folder_id);
                let cascade_entry_ids: Vec<String> = base.entries.iter().filter(|entry| entry.folder_id.as_deref().is_some_and(|folder_id| cascade_folder_ids.iter().any(|id| id == folder_id))).map(|entry| entry.id.clone()).collect();
                protocol::MutationOutcome::new(CollectionDiff {
                    folders: Some(CollectionFoldersDelta { removed: cascade_folder_ids, ..Default::default() }),
                    entries: (!cascade_entry_ids.is_empty()).then(|| CollectionEntriesDelta { removed: cascade_entry_ids, ..Default::default() }),
                    ..Default::default()
                })
            }
            CollectionMutation::DeleteFolder { folder_id } => missing("Folder", folder_id),
            CollectionMutation::MoveToCollection { folder_id, new_parent } if folder_exists(folder_id) => {
                folders(CollectionFoldersDelta { patched: vec![CollectionFolderPatch { id: folder_id.clone(), parent_id: Some(CollectionOptionalLink { value: new_parent.clone() }), name: None }], ..Default::default() })
            }
            CollectionMutation::MoveToCollection { folder_id, .. } => missing("Folder", folder_id),
            CollectionMutation::RenameFolder { folder_id, new_name } if folder_exists(folder_id) => {
                folders(CollectionFoldersDelta { patched: vec![CollectionFolderPatch { id: folder_id.clone(), parent_id: None, name: Some(new_name.clone()) }], ..Default::default() })
            }
            CollectionMutation::RenameFolder { folder_id, .. } => missing("Folder", folder_id),
            CollectionMutation::CreateEntry { entry, index } => {
                let natural = *index as usize >= base.entries.len();
                entries(CollectionEntriesDelta {
                    added: vec![entry.clone()],
                    reordered: (!natural).then(|| order(base.entries.iter().map(|existing| existing.id.clone()).collect(), &entry.id, *index)),
                    ..Default::default()
                })
            }
            CollectionMutation::DeleteEntry { entry_id } if entry_exists(entry_id) => entries(CollectionEntriesDelta { removed: vec![entry_id.clone()], ..Default::default() }),
            CollectionMutation::DeleteEntry { entry_id } => missing("Entry", entry_id),
            CollectionMutation::MoveToFolder { entry_id, new_folder } if entry_exists(entry_id) => {
                entries(CollectionEntriesDelta { patched: vec![CollectionEntryPatch { id: entry_id.clone(), folder_id: Some(CollectionOptionalLink { value: new_folder.clone() }), ..Default::default() }], ..Default::default() })
            }
            CollectionMutation::MoveToFolder { entry_id, .. } => missing("Entry", entry_id),
            CollectionMutation::RenameEntry { entry_id, new_name } if entry_exists(entry_id) => {
                entries(CollectionEntriesDelta { patched: vec![CollectionEntryPatch { id: entry_id.clone(), name: Some(new_name.clone()), ..Default::default() }], ..Default::default() })
            }
            CollectionMutation::RenameEntry { entry_id, .. } => missing("Entry", entry_id),
            CollectionMutation::ReplaceEntryBody { entry_id, new_body } if entry_exists(entry_id) => {
                entries(CollectionEntriesDelta { patched: vec![CollectionEntryPatch { id: entry_id.clone(), body: Some(new_body.clone()), ..Default::default() }], ..Default::default() })
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
                mutations.reverse();
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

#[derive(Clone, value_derive::FromValue, value_derive::ToValue)]
#[value(deny_unknown_fields)]
struct CollectionPackageSource {
    definition_version: u8,
    id: String,
    artifact: String,
    directory: String,
    rust_package: String,
    nx_project: String,
    dependencies: Vec<String>,
}

/// 📦️ Validated package identity for the builtin collection artifact.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CollectionArtifactPackage {
    pub id: String,
    pub artifact: String,
    pub directory: String,
    pub rust_package: String,
    pub nx_project: String,
    pub dependencies: Vec<String>,
}

/// 🚫️ Invalid builtin collection package declaration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CollectionPackageSchemaError(pub String);

impl std::fmt::Display for CollectionPackageSchemaError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for CollectionPackageSchemaError {}

/// 🧬️ Language-neutral package declaration owned by this artifact.
pub const COLLECTION_ARTIFACT_DEFINITION_SCHEMA: &str = include_str!("🧬️schema/📜️artifact-definition.json");

/// 📦️ Parses and validates the builtin collection package declaration.
pub fn collection_package_from_schema(source: &str) -> Result<CollectionArtifactPackage, CollectionPackageSchemaError> {
    let parsed = semio_framework_pack_json::from_json_str::<CollectionPackageSource>(source, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| CollectionPackageSchemaError(error.to_string()))?;
    if parsed.definition_version != 1 || parsed.id != "os.collection" || parsed.artifact != "collection" || parsed.directory != "🗂️collection" || parsed.rust_package != "semio-framework-artifact-space-collection" || parsed.nx_project != "@semio-tech/framework-space-collection-rs" || !parsed.dependencies.is_empty() {
        return Err(CollectionPackageSchemaError("builtin collection package identity does not match its canonical declaration".into()));
    }
    Ok(CollectionArtifactPackage {
        id: parsed.id,
        artifact: parsed.artifact,
        directory: parsed.directory,
        rust_package: parsed.rust_package,
        nx_project: parsed.nx_project,
        dependencies: parsed.dependencies,
    })
}

/// 📦️ Returns this artifact's validated package declaration.
pub fn package_descriptor() -> Result<CollectionArtifactPackage, CollectionPackageSchemaError> {
    collection_package_from_schema(COLLECTION_ARTIFACT_DEFINITION_SCHEMA)
}

#[cfg(test)]
#[path = "🧪️tests/🗂️collection/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path="🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod sqlite_snapshot_baseline;

#[path = "🚪️io/🦀️.rs"]
pub mod io;
