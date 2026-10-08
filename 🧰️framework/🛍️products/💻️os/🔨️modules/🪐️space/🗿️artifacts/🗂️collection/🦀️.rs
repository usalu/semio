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
/// 🔗️ Sparse per-field delta shared by folder re-parenting and entry re-filing — the item's id plus
/// its new container link. Named for derivation rule 5 (`move-to-<container>{id, new_parent}`), which
/// is why `MoveToCollection`/`MoveToFolder` below both carry this same shape despite addressing
/// different collections (`CollectionFolder.parent_id` vs `CollectionEntry.folder_id`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
pub struct MovedToContainer {
    pub id: String,
    pub new_parent: Option<String>,
}

/// ✏️ Sparse per-field delta for a rename — the item's id plus its new name.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
pub struct RenamedItem {
    pub id: String,
    pub new_name: String,
}

/// 📦️ Sparse per-field delta for `ReplaceEntryBody` — the entry id plus its new body. Mirrors
/// `CollectionEntry.body`'s own `#[dsl(statements)]` handling of the foreign-shaped `ArtifactBody`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
pub struct ReplacedEntryBody {
    pub entry_id: String,
    #[dsl(statements)]
    pub new_body: Box<ArtifactBody>,
}

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

/// 📏️ Root-to-`folder_id` chain length (root folders are depth 0) — used to order a cascade
/// delete's inverse leaves-first (deepest folders recreated before their ancestors).
fn folder_depth(folders: &[CollectionFolder], folder_id: &str) -> usize {
    let by_id: HashMap<&str, &CollectionFolder> = folders.iter().map(|folder| (folder.id.as_str(), folder)).collect();
    let mut depth = 0usize;
    let mut current = folder_id.to_string();
    let mut guard = 0usize;
    while let Some(folder) = by_id.get(current.as_str()) {
        match &folder.parent_id {
            Some(parent) => {
                current = parent.clone();
                depth += 1;
            }
            None => break,
        }
        guard += 1;
        if guard > folders.len() {
            break;
        }
    }
    depth
}

/// 🧬️ Sparse per-field collection delta — every field records WHAT CHANGED (an id, a new value), never
/// a whole post-mutation record. Handcrafted rather than relying on `#[derive(dsl::)]`'s field
/// lowering alone for `MovedToContainer`/`RenamedItem`/`ReplacedEntryBody`: SMO's ruling on this file's
/// design doc adopted verbatim — *"replayability isn't the property the rule protects; mergeability
/// is. A whole-record diff asserts every field, so two users renaming a folder and moving it cannot
/// merge."* `deleted_folder_ids`/`deleted_entry_ids` are id lists (never full records) so a cascade
/// delete's diff stays a set of removed ids — the removed folders'/entries' full payload lives only in
/// `CollectionMutation::inverse`'s own reconstruction from `base`, never duplicated into the diff.
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, Default, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
pub struct CollectionDiff {
    pub renamed_collection: Option<String>,

    #[dsl(block)]
    pub created_folder: Option<CollectionFolder>,
    /// 🔢️ Companion to `created_folder` — the insertion index. Always `Some` exactly when
    /// `created_folder` is, kept as a sibling field rather than nested inside it since the derive
    /// engine has no first-class "record + position" shape (see `📓️wave3c-reports/flow-space-report.md`'s
    /// derive-engine-gap finding).
    pub created_folder_at: Option<u32>,
    /// 🗑️ Every folder id removed by a `DeleteFolder` — the target plus its full cascade subtree.
    pub deleted_folder_ids: Option<Vec<String>>,
    #[dsl(block)]
    pub moved_folder: Option<MovedToContainer>,
    #[dsl(block)]
    pub renamed_folder: Option<RenamedItem>,

    #[dsl(block)]
    pub created_entry: Option<CollectionEntry>,
    /// 🔢️ Companion to `created_entry`, same convention as `created_folder_at`.
    pub created_entry_at: Option<u32>,
    /// 🗑️ Every entry id removed — either one `DeleteEntry` target, or every entry filed under a
    /// `DeleteFolder`'s cascade subtree.
    pub deleted_entry_ids: Option<Vec<String>>,
    #[dsl(block)]
    pub moved_entry: Option<MovedToContainer>,
    #[dsl(block)]
    pub renamed_entry: Option<RenamedItem>,
    #[dsl(block)]
    pub replaced_entry_body: Option<ReplacedEntryBody>,
}

impl protocol::DiffAlgebra<CollectionSnapshot> for CollectionDiff {
    fn inverse(&self, base: &CollectionSnapshot) -> Self {
        let mut inverse = Self::default();
        inverse.renamed_collection = self.renamed_collection.as_ref().map(|_| base.name.clone());
        if let Some(folder) = &self.created_folder {
            inverse.deleted_folder_ids = Some(vec![folder.id.clone()]);
        }
        if let Some(ids) = &self.deleted_folder_ids {
            if let Some((index, folder)) = base.folders.iter().enumerate().find(|(_, folder)| ids.contains(&folder.id)) {
                inverse.created_folder = Some(folder.clone());
                inverse.created_folder_at = Some(index as u32);
            }
        }
        inverse.moved_folder = self.moved_folder.as_ref().and_then(|moved| base.folders.iter().find(|folder| folder.id == moved.id).map(|folder| MovedToContainer { id: moved.id.clone(), new_parent: folder.parent_id.clone() }));
        inverse.renamed_folder = self.renamed_folder.as_ref().and_then(|renamed| base.folders.iter().find(|folder| folder.id == renamed.id).map(|folder| RenamedItem { id: renamed.id.clone(), new_name: folder.name.clone() }));
        if let Some(entry) = &self.created_entry {
            inverse.deleted_entry_ids = Some(vec![entry.id.clone()]);
        }
        if let Some(ids) = &self.deleted_entry_ids {
            if let Some((index, entry)) = base.entries.iter().enumerate().find(|(_, entry)| ids.contains(&entry.id)) {
                inverse.created_entry = Some(entry.clone());
                inverse.created_entry_at = Some(index as u32);
            }
        }
        inverse.moved_entry = self.moved_entry.as_ref().and_then(|moved| base.entries.iter().find(|entry| entry.id == moved.id).map(|entry| MovedToContainer { id: moved.id.clone(), new_parent: entry.folder_id.clone() }));
        inverse.renamed_entry = self.renamed_entry.as_ref().and_then(|renamed| base.entries.iter().find(|entry| entry.id == renamed.id).map(|entry| RenamedItem { id: renamed.id.clone(), new_name: entry.name.clone() }));
        inverse.replaced_entry_body = self.replaced_entry_body.as_ref().and_then(|replaced| base.entries.iter().find(|entry| entry.id == replaced.entry_id).map(|entry| ReplacedEntryBody { entry_id: replaced.entry_id.clone(), new_body: entry.body.clone() }));
        inverse
    }

    fn between(base: &CollectionSnapshot, other: &CollectionSnapshot) -> Self {
        let mut diff = Self::default();
        diff.renamed_collection = (base.name != other.name).then(|| other.name.clone());
        if let Some((index, folder)) = other.folders.iter().enumerate().find(|(_, folder)| !base.folders.iter().any(|existing| existing.id == folder.id)) {
            diff.created_folder = Some(folder.clone());
            diff.created_folder_at = Some(index as u32);
        }
        let deleted_folders: Vec<String> = base.folders.iter().filter(|folder| !other.folders.iter().any(|existing| existing.id == folder.id)).map(|folder| folder.id.clone()).collect();
        diff.deleted_folder_ids = (!deleted_folders.is_empty()).then_some(deleted_folders);
        diff.moved_folder = other.folders.iter().find(|folder| base.folders.iter().any(|existing| existing.id == folder.id && existing.parent_id != folder.parent_id)).map(|folder| MovedToContainer { id: folder.id.clone(), new_parent: folder.parent_id.clone() });
        diff.renamed_folder = other.folders.iter().find(|folder| base.folders.iter().any(|existing| existing.id == folder.id && existing.name != folder.name)).map(|folder| RenamedItem { id: folder.id.clone(), new_name: folder.name.clone() });
        if let Some((index, entry)) = other.entries.iter().enumerate().find(|(_, entry)| !base.entries.iter().any(|existing| existing.id == entry.id)) {
            diff.created_entry = Some(entry.clone());
            diff.created_entry_at = Some(index as u32);
        }
        let deleted_entries: Vec<String> = base.entries.iter().filter(|entry| !other.entries.iter().any(|existing| existing.id == entry.id)).map(|entry| entry.id.clone()).collect();
        diff.deleted_entry_ids = (!deleted_entries.is_empty()).then_some(deleted_entries);
        diff.moved_entry = other.entries.iter().find(|entry| base.entries.iter().any(|existing| existing.id == entry.id && existing.folder_id != entry.folder_id)).map(|entry| MovedToContainer { id: entry.id.clone(), new_parent: entry.folder_id.clone() });
        diff.renamed_entry = other.entries.iter().find(|entry| base.entries.iter().any(|existing| existing.id == entry.id && existing.name != entry.name)).map(|entry| RenamedItem { id: entry.id.clone(), new_name: entry.name.clone() });
        diff.replaced_entry_body = other.entries.iter().find(|entry| base.entries.iter().any(|existing| existing.id == entry.id && existing.body != entry.body)).map(|entry| ReplacedEntryBody { entry_id: entry.id.clone(), new_body: entry.body.clone() });
        diff
    }

    fn is_empty(&self) -> bool {
        *self == Self::default()
    }
}

impl protocol::MutationDiff<CollectionSnapshot> for CollectionDiff {
    fn apply(&self, base: &CollectionSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<CollectionSnapshot> {
        let mut next = base.clone();
        if self.created_folder.is_some() != self.created_folder_at.is_some() {
            return Err(protocol::MutationApplyError::new("mutation.apply.incomplete-diff", "created folder requires an exact final index").at(["folders"]));
        }
        if self.created_entry.is_some() != self.created_entry_at.is_some() {
            return Err(protocol::MutationApplyError::new("mutation.apply.incomplete-diff", "created entry requires an exact final index").at(["entries"]));
        }
        if let Some(new_name) = &self.renamed_collection {
            next.name = new_name.clone();
        }
        if let Some(folder) = &self.created_folder {
            if next.folders.iter().any(|existing| existing.id == folder.id) {
                return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", format!("folder {} already exists", folder.id)).at(["folders", folder.id.as_str()]));
            }
            let at = self.created_folder_at.unwrap_or_default() as usize;
            if at > next.folders.len() {
                return Err(protocol::MutationApplyError::new("mutation.apply.invalid-index", format!("folder index {at} is out of range for length {}", next.folders.len())).at(["folders".to_string(), at.to_string()]));
            }
            next.folders.insert(at, folder.clone());
        }
        if let Some(ids) = &self.deleted_folder_ids {
            // 🧮️ Mechanical replay only removes the folders themselves — a dangling `parent_id`/
            // `folder_id` left pointing at one is `reconcile_collection_integrity`'s job (rules
            // `collection/folder-orphaned`/`collection/entry-folder-missing`), run separately after
            // `apply`, never inline here.
            for (index, id) in ids.iter().enumerate() {
                if ids[..index].contains(id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", format!("folder {id} is deleted more than once")).at(["folders", id.as_str()]));
                }
                if !next.folders.iter().any(|folder| folder.id == *id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", format!("folder {id} does not exist")).at(["folders", id.as_str()]));
                }
            }
            next.folders.retain(|folder| !ids.contains(&folder.id));
        }
        if let Some(moved) = &self.moved_folder {
            let folder =
                next.folders.iter_mut().find(|folder| folder.id == moved.id).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", format!("folder {} does not exist", moved.id)).at(["folders", moved.id.as_str()]))?;
            folder.parent_id = moved.new_parent.clone();
        }
        if let Some(renamed) = &self.renamed_folder {
            let folder =
                next.folders.iter_mut().find(|folder| folder.id == renamed.id).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", format!("folder {} does not exist", renamed.id)).at(["folders", renamed.id.as_str()]))?;
            folder.name = renamed.new_name.clone();
        }
        if let Some(entry) = &self.created_entry {
            if next.entries.iter().any(|existing| existing.id == entry.id) {
                return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", format!("entry {} already exists", entry.id)).at(["entries", entry.id.as_str()]));
            }
            let at = self.created_entry_at.unwrap_or_default() as usize;
            if at > next.entries.len() {
                return Err(protocol::MutationApplyError::new("mutation.apply.invalid-index", format!("entry index {at} is out of range for length {}", next.entries.len())).at(["entries".to_string(), at.to_string()]));
            }
            next.entries.insert(at, entry.clone());
        }
        if let Some(ids) = &self.deleted_entry_ids {
            for (index, id) in ids.iter().enumerate() {
                if ids[..index].contains(id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", format!("entry {id} is deleted more than once")).at(["entries", id.as_str()]));
                }
                if !next.entries.iter().any(|entry| entry.id == *id) {
                    return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", format!("entry {id} does not exist")).at(["entries", id.as_str()]));
                }
            }
            next.entries.retain(|entry| !ids.contains(&entry.id));
        }
        if let Some(moved) = &self.moved_entry {
            let entry = next.entries.iter_mut().find(|entry| entry.id == moved.id).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", format!("entry {} does not exist", moved.id)).at(["entries", moved.id.as_str()]))?;
            entry.folder_id = moved.new_parent.clone();
        }
        if let Some(renamed) = &self.renamed_entry {
            let entry =
                next.entries.iter_mut().find(|entry| entry.id == renamed.id).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", format!("entry {} does not exist", renamed.id)).at(["entries", renamed.id.as_str()]))?;
            entry.name = renamed.new_name.clone();
        }
        if let Some(replaced) = &self.replaced_entry_body {
            let entry = next
                .entries
                .iter_mut()
                .find(|entry| entry.id == replaced.entry_id)
                .ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", format!("entry {} does not exist", replaced.entry_id)).at(["entries", replaced.entry_id.as_str()]))?;
            entry.body = replaced.new_body.clone();
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.renamed_collection.is_some() {
            self.renamed_collection = other.renamed_collection;
        }
        if other.created_folder.is_some() {
            self.created_folder = other.created_folder;
            self.created_folder_at = other.created_folder_at;
        }
        if let Some(ids) = other.deleted_folder_ids {
            self.deleted_folder_ids.get_or_insert_with(Vec::new).extend(ids);
        }
        if other.moved_folder.is_some() {
            self.moved_folder = other.moved_folder;
        }
        if other.renamed_folder.is_some() {
            self.renamed_folder = other.renamed_folder;
        }
        if other.created_entry.is_some() {
            self.created_entry = other.created_entry;
            self.created_entry_at = other.created_entry_at;
        }
        if let Some(ids) = other.deleted_entry_ids {
            self.deleted_entry_ids.get_or_insert_with(Vec::new).extend(ids);
        }
        if other.moved_entry.is_some() {
            self.moved_entry = other.moved_entry;
        }
        if other.renamed_entry.is_some() {
            self.renamed_entry = other.renamed_entry;
        }
        if other.replaced_entry_body.is_some() {
            self.replaced_entry_body = other.replaced_entry_body;
        }
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
        let mut diff = CollectionDiff::default();
        match self {
            CollectionMutation::RenameCollection { new_name } => diff.renamed_collection = Some(new_name.clone()),
            CollectionMutation::CreateFolder { folder, index } => {
                diff.created_folder = Some(folder.clone());
                diff.created_folder_at = Some(*index);
            }
            CollectionMutation::DeleteFolder { folder_id } => {
                if base.folders.iter().any(|folder| &folder.id == folder_id) {
                    let cascade_folder_ids = folder_subtree_ids(&base.folders, folder_id);
                    let cascade_entry_ids: Vec<String> = base.entries.iter().filter(|entry| entry.folder_id.as_deref().is_some_and(|folder_id| cascade_folder_ids.iter().any(|id| id == folder_id))).map(|entry| entry.id.clone()).collect();
                    diff.deleted_folder_ids = Some(cascade_folder_ids);
                    if !cascade_entry_ids.is_empty() {
                        diff.deleted_entry_ids = Some(cascade_entry_ids);
                    }
                }
            }
            CollectionMutation::MoveToCollection { folder_id, new_parent } => {
                if base.folders.iter().any(|folder| &folder.id == folder_id) {
                    diff.moved_folder = Some(MovedToContainer { id: folder_id.clone(), new_parent: new_parent.clone() });
                }
            }
            CollectionMutation::RenameFolder { folder_id, new_name } => {
                if base.folders.iter().any(|folder| &folder.id == folder_id) {
                    diff.renamed_folder = Some(RenamedItem { id: folder_id.clone(), new_name: new_name.clone() });
                }
            }
            CollectionMutation::CreateEntry { entry, index } => {
                diff.created_entry = Some(entry.clone());
                diff.created_entry_at = Some(*index);
            }
            CollectionMutation::DeleteEntry { entry_id } => diff.deleted_entry_ids = Some(vec![entry_id.clone()]),
            CollectionMutation::MoveToFolder { entry_id, new_folder } => {
                if base.entries.iter().any(|entry| &entry.id == entry_id) {
                    diff.moved_entry = Some(MovedToContainer { id: entry_id.clone(), new_parent: new_folder.clone() });
                }
            }
            CollectionMutation::RenameEntry { entry_id, new_name } => {
                if base.entries.iter().any(|entry| &entry.id == entry_id) {
                    diff.renamed_entry = Some(RenamedItem { id: entry_id.clone(), new_name: new_name.clone() });
                }
            }
            CollectionMutation::ReplaceEntryBody { entry_id, new_body } => {
                if base.entries.iter().any(|entry| &entry.id == entry_id) {
                    diff.replaced_entry_body = Some(ReplacedEntryBody { entry_id: entry_id.clone(), new_body: new_body.clone() });
                }
            }
        }
        protocol::MutationOutcome::new(diff)
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
                // 🍃️ Entries are always leaves (no descendants) — recreate every cascaded entry
                // before any cascaded folder.
                for entry in &base.entries {
                    if entry.folder_id.as_deref().is_some_and(|folder_id| cascade_folder_ids.iter().any(|id| id == folder_id)) {
                        if let Some(at) = base.entries.iter().position(|candidate| candidate.id == entry.id) {
                            mutations.push(CollectionMutation::CreateEntry { entry: entry.clone(), index: at as u32 });
                        }
                    }
                }
                // 🌳️ Deepest folders first, the originally-deleted folder (shallowest in the
                // cascade) last — leaves-first, mirroring the entries above.
                let mut ordered_folder_ids = cascade_folder_ids;
                ordered_folder_ids.sort_by_key(|id| std::cmp::Reverse(folder_depth(&base.folders, id)));
                for id in ordered_folder_ids {
                    if let Some(at) = base.folders.iter().position(|folder| folder.id == id) {
                        mutations.push(CollectionMutation::CreateFolder { folder: base.folders[at].clone(), index: at as u32 });
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
