//! 🔗️ Member link resolution and blob store: the content id, the artifact refs, the link state and resolver, the member directory, the no blob store, the member link resolver and the blob store. See [`super`].

use super::*;

//#region 🔖️EphemeralLanes
//#region 🔖️Backbone
//#region 🔖️BlobStore
//#region 🔖️BlobStore
//#region 🔖️Schemas
/// 🆔️ The content-addressed id of a composed child or persisted scene: `<prefix>-` + the first 16 lowercase hex digits
/// of SHA-256 over `bytes` (the child schema's `ContentId`). Specified, stable across processes, platforms and
/// toolchains, and reproducible by any implementation — unlike a `std` hasher, whose algorithm Rust leaves unspecified.
/// A site that addresses several fields joins them with U+001F. TypeScript twin: `contentId` (`🪆️child/🧬️schema/🟦️.ts`).
pub fn content_id(prefix: &str, bytes: &[u8]) -> String {
    format!("{prefix}-{}", semio_framework_hash::hex_lower(&semio_framework_hash::Sha256::digest(bytes)[..8]))
}
//#endregion 🔖️Schemas

//#region 🔖️Composition
/// 🌳️ Lets a snapshot type declare its own composed children / referenced links. Both
/// methods default to empty, so a leaf artifact (the overwhelming majority) needs zero
/// boilerplate — only a technology that actually embeds `ArtifactChild<S>`/`ArtifactLink` fields
/// overrides them (typically derive-generated in a later wave, per the design doc's
/// `ChildSlotSpec`/`LinkSlotSpec` schema facets).
pub trait ArtifactRefs {
    async fn child_refs(&self) -> Vec<ChildRef> {
        Vec::new()
    }
    async fn links(&self) -> Vec<ArtifactLink> {
        Vec::new()
    }
}

/// 🔎️ What resolving one `ArtifactLink` found: the target materialized as real bytes, that
/// it is simply absent, or (for a `LinkPin::Snapshot`) only the escrowed blob reference is known
/// without the resolver having fetched its bytes yet.
#[derive(Clone, Debug, PartialEq)]
pub enum LinkState {
    Resolved { pack_bytes: Vec<u8>, dialect: semio_framework_artifact_reference::ArtifactDialect },
    Missing,
    PinnedOnly { blob: BlobRef },
}

/// 🕵️ Host-side, LAZY link resolution — never called during `CompositionCoordinator`
/// dispatch (links have no bearing on group-atomicity: unlike a child, a link's target is not part
/// of this artifact's own document and dispatch never touches it). A UI/renderer calls this only
/// when it actually needs to show/open the linked content.
pub trait LinkResolver {
    async fn resolve(&self, link: &ArtifactLink) -> LinkState;
}

/// 🗂️ The read-only directory a `MemberLinkResolver` resolves against — whatever holds the
/// live members (a `SpaceHost`, or a test fixture). Returns owned PACK bytes rather than a
/// `&dyn SpaceMember` so the resolver can be OWNED and stored (a borrowed member reference could
/// never live in a long-lived host field), and `Option<Result<..>>` distinguishes "no such member"
/// (→ `LinkState::Missing`) from "member exists but reading it failed" (→ a real error, never
/// silently reported as absent).
pub trait MemberDirectory {
    async fn head_pack(&self, artifact_id: &str) -> Option<Result<Vec<u8>, VcsError>>;
    async fn checkpoint_pack(&self, artifact_id: &str, checkpoint_id: &str) -> Option<Result<Vec<u8>, VcsError>>;
}
//#endregion 🔖️Composition

//#region 🔖️Composition
/// 🕵️ The one production `LinkResolver`: resolves each `LinkPin` against a `MemberDirectory`
/// (plus an optional `BlobStore` for escrowed snapshot pins). This is what makes "referenced
/// artifacts with their own version history" real — `Head` reads the target's live tip, while
/// `Checkpoint` reads the target's content AS OF that checkpoint, so a pinned reference keeps
/// showing what it was pinned to no matter how far the target has since moved.
///
/// A `Snapshot` pin with no blob store, or whose bytes are not in the store, degrades to
/// `PinnedOnly` — the blob reference is still known and displayable (a chip can render "pinned to
/// <hash>" without the content), which is precisely the state `LinkState::PinnedOnly` exists for.
/// A read error is never laundered into `Missing`: absence and failure are different answers, and
/// only absence is benign.
/// 🕳️ The default `B` for a `MemberLinkResolver` that carries no blob escrow — uninhabited,
/// same stand-in shape as `BackboneChannelPorts`/`NoMembers`, so `MemberLinkResolver::new` never
/// needs its caller to name a concrete `BlobStore` implementor it isn't using.
pub enum NoBlobStore {}

impl BlobStore for NoBlobStore {
    async fn put(&self, _bytes: &[u8], _media_type: &str) -> Result<BlobRef, VcsError> {
        match *self {}
    }

    async fn get(&self, _hash: &str) -> Result<Option<Vec<u8>>, VcsError> {
        match *self {}
    }

    async fn has(&self, _hash: &str) -> Result<bool, VcsError> {
        match *self {}
    }

    async fn delete(&self, _hash: &str) -> Result<(), VcsError> {
        match *self {}
    }
}

pub struct MemberLinkResolver<D, B = NoBlobStore> {
    directory: D,
    blobs: Option<Arc<B>>,
}

impl<D: MemberDirectory> MemberLinkResolver<D, NoBlobStore> {
    /// 🏗️ Resolver without blob escrow — `LinkPin::Snapshot` always degrades to `PinnedOnly`.
    pub async fn new(directory: D) -> Self {
        Self { directory, blobs: None }
    }
}

impl<D: MemberDirectory, B: BlobStore> MemberLinkResolver<D, B> {
    /// 🏗️ Resolver that can also materialize content-addressed `LinkPin::Snapshot` blobs.
    pub async fn with_blobs(directory: D, blobs: Arc<B>) -> Self {
        Self { directory, blobs: Some(blobs) }
    }
}

impl<D: MemberDirectory, B: BlobStore> LinkResolver for MemberLinkResolver<D, B> {
    async fn resolve(&self, link: &ArtifactLink) -> LinkState {
        let dialect = link.target.dialect.clone();
        match &link.pin {
            LinkPin::Head => match self.directory.head_pack(&link.target.artifact_id).await {
                Some(Ok(pack_bytes)) => LinkState::Resolved { pack_bytes, dialect },
                Some(Err(_)) | None => LinkState::Missing,
            },
            LinkPin::Checkpoint { id } => match self.directory.checkpoint_pack(&link.target.artifact_id, id).await {
                Some(Ok(pack_bytes)) => LinkState::Resolved { pack_bytes, dialect },
                Some(Err(_)) | None => LinkState::Missing,
            },
            LinkPin::Snapshot { blob } => {
                let resolved = match &self.blobs {
                    Some(blobs) => blobs.get(&blob.hash).await.ok().flatten(),
                    None => None,
                };
                match resolved {
                    Some(pack_bytes) => LinkState::Resolved { pack_bytes, dialect },
                    None => LinkState::PinnedOnly { blob: blob.clone() },
                }
            }
        }
    }
}
//#endregion 🔖️Composition

//#region 🔖️Authority
//#region 🔖️EphemeralLanes
//#region 🔖️Text
//#region 🔖️Pack
//#region 🔖️CodecRegistry
//#region 🔖️TextFormat
//#region 🔖️History
//#region 🔖️ArtifactStore
//#region 🔖️Backbone
//#region 🔖️BlobStore
//#region 🔖️BlobStore
/// 🗄️ Content-addressed blob persistence backing any `ArtifactKind::ContentAddressedBlob`-
/// shaped field that needs to reference bytes durably without embedding them inline. `put` is
/// idempotent — it dedupes by the Blake3 hash of the bytes ({@link framework_hash::hash_bytes}), so
/// writing the same content twice never rewrites storage. Implementors decide the backing medium
/// (sqlite here, a semio_hub HTTP route in a later ticket, an IndexedDB cache in the browser).
pub trait BlobStore: Send + Sync {
    async fn put(&self, bytes: &[u8], media_type: &str) -> Result<BlobRef, VcsError>;
    async fn get(&self, hash: &str) -> Result<Option<Vec<u8>>, VcsError>;
    async fn has(&self, hash: &str) -> Result<bool, VcsError>;
    async fn delete(&self, hash: &str) -> Result<(), VcsError>;
}
//#endregion 🔖️BlobStore
//#endregion 🔖️BlobStore
//#endregion 🔖️Backbone
//#endregion 🔖️ArtifactStore
//#endregion 🔖️History
//#endregion 🔖️TextFormat
//#endregion 🔖️CodecRegistry
//#endregion 🔖️Pack
//#endregion 🔖️Text
//#endregion 🔖️EphemeralLanes
//#endregion 🔖️Authority
//#endregion 🔖️BlobStore
//#endregion 🔖️BlobStore
//#endregion 🔖️Backbone
//#endregion 🔖️EphemeralLanes
