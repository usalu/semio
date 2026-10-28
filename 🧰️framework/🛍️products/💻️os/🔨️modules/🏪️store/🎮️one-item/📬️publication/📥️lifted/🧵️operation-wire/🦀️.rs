//! 📥️ Items of `prepared_operation_wire` that belong to `os-store-one-item-publication` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/🧵️operation-wire/🦀️.rs`.
use semio_framework_os_store_canonical_json::ArtifactCanonicalJson;
use semio_framework_pack_json::ArtifactCanonicalJsonTree;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::{ValueError, ValueRefusalKind};
use semio_framework_os_pack::record::BorrowedProjectedPackFailure;
use semio_framework_dsl_record::{BorrowedRecordSpec,native_encoding::FieldProjectionSource};
use crate::lifted_text::ArtifactOperationText;
/// 🫳️ Installed domain codec authority over one original immutable typed operation.
#[derive(Clone, Copy)]
pub enum ArtifactPreparedOperationSource<'a> {
    Pack { tag: u64, body: &'a dyn FieldProjectionSource, spec: BorrowedRecordSpec },
    Text { header: &'static [u8], body: &'a dyn ArtifactOperationText },
    CanonicalJson { header: &'static [u8], body: &'a dyn ArtifactCanonicalJson },
    HexJson { header: &'static [u8], prefix: &'static [u8], body: &'a dyn ArtifactCanonicalJson },
}

/// ⚠️ Exact initialized output prefix remains visible when a domain source refuses.
#[derive(Debug)]
pub struct ArtifactPreparedOperationError {
    pub written_bytes: usize,
    pub reason: ValueError,
}

impl From<ValueError> for ArtifactPreparedOperationError {
    fn from(reason: ValueError) -> Self { Self { written_bytes: 0, reason } }
}

impl From<BorrowedProjectedPackFailure> for ArtifactPreparedOperationError {
    fn from(failure: BorrowedProjectedPackFailure) -> Self { Self { written_bytes: failure.written_bytes, reason: failure.reason } }
}

pub use semio_framework_os_store_canonical_json::ArtifactCanonicalTreeSource;

/// 🪪️ The canonical operation wire of one mutation vocabulary, installed exactly once on every Store that authors it.
///
/// The wire is a pure function of the original mutation, so the Store owns the default instead of every preparation factory:
/// factories, catalogs and genesis ordering cannot leave a lane without one. A factory that speaks a domain codec
/// (pack, text, hex JSON) still overrides it per mutation through
/// [`ArtifactStoreOneItemPreparationFactory::operation_wire_source`](super::ArtifactStoreOneItemPreparationFactory::operation_wire_source).
pub type OriginalBatchMessageAdmission<M> = fn(&mut Option<Box<semio_framework_os_spr::Edit<M>>>,ArtifactOperationWire<M>,RetainedCloneGrant)->Result<Option<(Box<dyn crate::batch_digest::ArtifactStoreBatchMessage<M>>,semio_framework_value::RetainedCloneProgress)>,ValueError>;

pub struct ArtifactOperationWire<M> {
    pub source: for<'a> fn(&'a M) -> Option<ArtifactPreparedOperationSource<'a>>,
    pub schema: for<'a> fn(&'a M) -> Option<(&'a str, &'a str)>,
    pub digest: BatchDigestAdmission<M>,
    pub history_digest: OriginalHistoryDigestStep<M>,
    pub original_receiving:Option<original_receiving::OriginalOperationReceivingAdmission<M>>,
    pub original_message_birth:Option<fn()->semio_framework_value::RetirementDemand>,
    pub original_message_admission:Option<OriginalBatchMessageAdmission<M>>,
    pub original_target:Option<for<'a> fn(&'a M)->Option<&'a dyn ArtifactOperationTargets>>,
}

/// 🧭️ Pure canonical codec step over an enclosing controller's original history read and actual candidate inverse.
pub type OriginalHistoryDigestStep<M> = fn(&mut semio_framework_os_store_ledger::native_digest::NativeEditDigest, &semio_framework_os_spr::Edit<M>, &semio_framework_value_persistent::list::PagedList<M,{usize::MAX}>, RetainedCloneGrant) -> Result<semio_framework_value::RetainedCloneProgress,ValueError>;

fn original_history_digest<M:ArtifactCanonicalJsonTree>(cursor:&mut semio_framework_os_store_ledger::native_digest::NativeEditDigest,edit:&semio_framework_os_spr::Edit<M>,inverse:&semio_framework_value_persistent::list::PagedList<M,{usize::MAX}>,grant:RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneProgress,ValueError>{cursor.advance_with_inverse(edit,inverse,grant)}

/// 🔏️ Admission of the canonical digest owner of one original aggregate edit: a pure function of the mutation vocabulary, so the Store owns it beside the wire.
pub type BatchDigestAdmission<M> = fn(&mut Option<Box<semio_framework_os_spr::Edit<M>>>, RetainedCloneGrant) -> Result<Option<(Box<dyn crate::batch_digest::ArtifactStoreBatchDigest<M>>, semio_framework_value::RetainedCloneProgress)>, ValueError>;

impl<M> Clone for ArtifactOperationWire<M> {
    fn clone(&self) -> Self { *self }
}

impl<M> Copy for ArtifactOperationWire<M> {}

fn canonical_tree_source<M: ArtifactCanonicalJsonTree>(mutation: &M) -> Option<ArtifactPreparedOperationSource<'_>> {
    Some(ArtifactPreparedOperationSource::CanonicalJson { header: b"op", body: ArtifactCanonicalTreeSource::view(mutation) })
}

fn semantic_schema<P, M: semio_framework_os_spr::SemanticMutation<P>>(mutation: &M) -> Option<(&str, &str)> {
    let semantics = mutation.semantics();
    Some((semantics.entity, semantics.kind))
}

fn no_schema<M>(_mutation: &M) -> Option<(&str, &str)> { None }

fn no_digest<M>(_edit: &mut Option<Box<semio_framework_os_spr::Edit<M>>>, _grant: RetainedCloneGrant) -> Result<Option<(Box<dyn crate::batch_digest::ArtifactStoreBatchDigest<M>>, semio_framework_value::RetainedCloneProgress)>, ValueError> {
    Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "the installed operation wire carries no batch digest authority"))
}

impl<M: ArtifactCanonicalJsonTree> ArtifactOperationWire<M> {
    /// 🧬️ Wire of a lane whose mutations carry no semantic vocabulary (config, draft, interaction): canonical tree JSON, no schema pair.
    pub fn canonical() -> Self { Self { source: canonical_tree_source::<M>, schema: no_schema::<M>, digest: no_digest::<M>,history_digest:original_history_digest::<M>,original_receiving:None,original_target:None,original_message_birth:None,original_message_admission:None } }

    /// 🔖️ Wire of a document lane: canonical tree JSON plus the `(entity, kind)` schema pair its member receipts name.
    pub fn semantic<P>() -> Self where M: semio_framework_os_spr::SemanticMutation<P> { Self { source: canonical_tree_source::<M>, schema: semantic_schema::<P, M>, digest: no_digest::<M>,history_digest:original_history_digest::<M>,original_receiving:None,original_target:None,original_message_birth:None,original_message_admission:None } }
}

impl<M: ArtifactCanonicalJsonTree + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static> ArtifactOperationWire<M> {
    /// 🔏️ Grants the wire its aggregate-edit digest authority: with it, a Store whose factory slot is empty still commits multi-operation gestures and transactions.
    pub fn with_batch_digest(mut self) -> Self { self.digest = crate::batch_digest::admit_artifact_batch_digest::<M>; self }
}

/// 🎯️ Borrows authored conflict path segments without manufacturing an owned path.
pub trait ArtifactOperationTargets:Sync{
    fn len(&self)->usize;
    fn segment_len(&self,index:usize)->Result<usize,ValueError>;
    fn read_segment(&self,index:usize,offset:usize,output:&mut[u8])->Result<usize,ValueError>;
}
impl ArtifactOperationTargets for Vec<String>{
    fn len(&self)->usize{Vec::len(self)}
    fn segment_len(&self,index:usize)->Result<usize,ValueError>{self.get(index).map(String::len).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original target segment is absent"))}
    fn read_segment(&self,index:usize,offset:usize,output:&mut[u8])->Result<usize,ValueError>{let bytes=self.get(index).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original target segment is absent"))?.as_bytes();let suffix=bytes.get(offset..).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original target segment offset is outside its source"))?;let count=suffix.len().min(output.len());output[..count].copy_from_slice(&suffix[..count]);Ok(count)}
}
/// 🌐️ Explicit domain declaration for an operation whose target is the whole document.
pub fn whole_document_target<M>(_:&M)->Option<&dyn ArtifactOperationTargets>{None}

impl<M> ArtifactOperationWire<M> {
    /// 📤️ Installs the actual original precommit message owner through a typed defining admission port.
    pub fn with_original_message(mut self,birth:fn()->semio_framework_value::RetirementDemand,admission:OriginalBatchMessageAdmission<M>)->Self{self.original_message_birth=Some(birth);self.original_message_admission=Some(admission);self}

    /// 🎯️ Installs the domain owner's original borrowed conflict path authority.
    pub fn with_original_target(mut self,target:for<'a> fn(&'a M)->Option<&'a dyn ArtifactOperationTargets>)->Self{self.original_target=Some(target);self}

    /// 🎯️ Requires the installed original path source before outbound construction.
    pub fn targets<'a>(&self,mutation:&'a M)->Result<Option<&'a dyn ArtifactOperationTargets>,ValueError>{self.original_target.map(|source|source(mutation)).ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original operation has no authored conflict path issuer"))}

    /// 📤️ Borrows the canonical wire of one original mutation.
    pub fn source<'a>(&self, mutation: &'a M) -> Option<ArtifactPreparedOperationSource<'a>> { (self.source)(mutation) }

    /// 🧭️ Uses the installed canonical vocabulary without manufacturing a source loan or requiring it on every generic model.
    pub fn advance_history_digest(&self,cursor:&mut semio_framework_os_store_ledger::native_digest::NativeEditDigest,edit:&semio_framework_os_spr::Edit<M>,inverse:&semio_framework_value_persistent::list::PagedList<M,{usize::MAX}>,grant:RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneProgress,ValueError>{(self.history_digest)(cursor,edit,inverse,grant)}

    /// 🔏️ Admits the canonical digest owner of one original aggregate edit through the Store's own vocabulary.
    pub fn begin_batch_digest(&self, edit: &mut Option<Box<semio_framework_os_spr::Edit<M>>>, grant: RetainedCloneGrant) -> Result<Option<(Box<dyn crate::batch_digest::ArtifactStoreBatchDigest<M>>, semio_framework_value::RetainedCloneProgress)>, ValueError> { (self.digest)(edit, grant) }

    /// 📛️ Borrows the `(entity, kind)` schema pair of one original mutation, when its lane has a semantic vocabulary.
    pub fn schema<'a>(&self, mutation: &'a M) -> Option<(&'a str, &'a str)> { (self.schema)(mutation) }
}

impl<M:ArtifactCanonicalJsonTree+semio_framework_value_native_owned::native_decoding::NativeReceive<semio_framework_value::DslValue>+Sync> ArtifactOperationWire<M>{
    /// 📏️ Quotes original canonical operation ingress before any bytes are parsed or typed owners are created.
    pub fn original_receiving_birth_demand(&self)->semio_framework_value::RetirementDemand{semio_framework_pack_json::OriginalJsonReceiving::<M>::constructor_demand()}
    /// 🧳️ Receives this installed canonical vocabulary through its original wire projection and mutation issuer.
    pub fn begin_original_receiving(&self,wire:semio_framework_value::RetainedOwnedProjection<Vec<u8>>,factory:&std::sync::Arc<dyn semio_framework_value::ArtifactOwnedValueRetirementFactory<M>>,control:&semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,grant:RetainedCloneGrant)->Result<(semio_framework_pack_json::OriginalJsonReceiving<M>,semio_framework_value::RetainedCloneProgress),(ValueError,semio_framework_value::RetainedOwnedProjection<Vec<u8>>)>{semio_framework_pack_json::OriginalJsonReceiving::admit(wire,factory,b"op",|mutation|mutation,control,grant)}
}

#[path="🧳️receiving/🦀️.rs"]
pub mod original_receiving;
pub use original_receiving::ArtifactOriginalOperationReceiving;

#[cfg(test)]
#[path="🎯️target/🦀️.rs"]
mod original_target_tests;
