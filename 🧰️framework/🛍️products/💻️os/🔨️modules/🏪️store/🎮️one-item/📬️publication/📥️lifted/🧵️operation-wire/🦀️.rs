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
pub struct ArtifactOperationWire<M> {
    pub source: for<'a> fn(&'a M) -> Option<ArtifactPreparedOperationSource<'a>>,
    pub schema: for<'a> fn(&'a M) -> Option<(&'a str, &'a str)>,
    pub digest: BatchDigestAdmission<M>,
}

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
    pub fn canonical() -> Self { Self { source: canonical_tree_source::<M>, schema: no_schema::<M>, digest: no_digest::<M> } }

    /// 🔖️ Wire of a document lane: canonical tree JSON plus the `(entity, kind)` schema pair its member receipts name.
    pub fn semantic<P>() -> Self where M: semio_framework_os_spr::SemanticMutation<P> { Self { source: canonical_tree_source::<M>, schema: semantic_schema::<P, M>, digest: no_digest::<M> } }
}

impl<M: ArtifactCanonicalJsonTree + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static> ArtifactOperationWire<M> {
    /// 🔏️ Grants the wire its aggregate-edit digest authority: with it, a Store whose factory slot is empty still commits multi-operation gestures and transactions.
    pub fn with_batch_digest(mut self) -> Self { self.digest = crate::batch_digest::admit_artifact_batch_digest::<M>; self }
}

impl<M> ArtifactOperationWire<M> {
    /// 📤️ Borrows the canonical wire of one original mutation.
    pub fn source<'a>(&self, mutation: &'a M) -> Option<ArtifactPreparedOperationSource<'a>> { (self.source)(mutation) }

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
