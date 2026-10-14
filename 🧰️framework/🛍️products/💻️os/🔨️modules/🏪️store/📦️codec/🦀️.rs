//! 📦️ The document codec registry: registration, preflight and lookup of artifact codecs, bounded store owners for
//! codec-driven reductions, assembly registry guards and dialect migrations.
//!
//! Layering: `📓️2026-10-13-wave-e1-os-kernel.md` (ticket BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT).
use pack::snapshot::{ArtifactDsl,ArtifactPack};
use crate::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory;
use crate::os_vcs::io::binary::genesis::ArtifactGenesisCodec;
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory};
use semio_framework_value::retained_clone::{RetainedCloneBirthDemand, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use crate::os_spr::command::ArtifactMessageSource;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
use crate::os_spr::{ActorId, ArtifactId};
use crate::os_spr::{DiffRegions, Mutation, MutationDiff, OpBinary, OpText};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::sync::Arc;
use crate::os_vcs::VcsError;
use crate::os_vcs::{
    Identified, Patchable,
};
use super::store_schema::ArtifactCursor;
use super::serialization::{ArtifactCodec, ArtifactPackFiles, ArtifactSqliteSnapshotHost};
#[cfg(target_arch = "wasm32")]
use super::serialization::{ArtifactCodecApplyFuture};
use crate::os_pack::pack_rt;
use super::owned_schema_decode::artifact_retirement_self_grant;
use super::text_format::{ArtifactTextFiles, parse_document_pack, print_document_pack};
use super::text_format::project_envelope_history;
use super::canonical_edit::{
    ArtifactCanonicalJson, ArtifactCanonicalJsonTree,
};
use super::one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES;
#[cfg(target_arch = "wasm32")]
use super::backbone::resolve_backbone;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
use super::backbone::local_storage_backbone_key;
use super::{ArtifactStore, ArtifactStoreCursorDisposer};

/// 🛡️ Applies the owner's semantic policy and preserves warnings while refusing errors.
pub fn validate_owned_sqlite_snapshot_subset<P: ArtifactSqliteSnapshotHost>(
    snapshot: &P,
    dialect: &semio_framework_artifact_reference::ArtifactDialect,
    database: &crate::sqlite_snapshot::SqliteDatabase,
    control: &mut crate::sqlite_snapshot::SqliteSnapshotControl<'_>,
) -> crate::io_schema::IoResult<()> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

    let outcome = snapshot.validate_sqlite_snapshot_subset(dialect, database, control)?;
    if outcome.diagnostics.iter().any(|diagnostic| matches!(diagnostic.severity, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal)) {
        return Err(crate::io_schema::IoError { cause: ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("owned snapshot violates subset {}", dialect.to_coordinate())), diagnostics: outcome.diagnostics });
    }
    Ok(outcome)
}

/// 📌️ The alternative a hub Check In materializes. It is the canonical trunk of `document_id`;
/// a viewer's checkout is local and never selects it.
pub fn canonical_check_in_line(document_id: &str) -> String {
    crate::os_spr::trunk_alternative_id(&ArtifactId(document_id.to_string()))
}

pub(crate) static DOCUMENT_CODEC_REGISTRY: std::sync::OnceLock<std::sync::RwLock<BTreeMap<String, ArtifactCodec>>> = std::sync::OnceLock::new();

pub(crate) fn document_codec_registry() -> &'static std::sync::RwLock<BTreeMap<String, ArtifactCodec>> {
    DOCUMENT_CODEC_REGISTRY.get_or_init(|| std::sync::RwLock::new(BTreeMap::new()))
}

/// ⚠️ A document schema already has a codec owner. The established codec is never replaced.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DocumentCodecRegistryConflict {
    pub schema: String,
}

/// ⚠️ Document-codec registration cannot replace an owner or use an unavailable registry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DocumentCodecRegistryError {
    Conflict(DocumentCodecRegistryConflict),
    Binding(crate::io::ArtifactCodecBindingError),
    Unavailable,
}

impl std::fmt::Display for DocumentCodecRegistryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Conflict(error) => write!(formatter, "document codec registration conflicts for schema {}", error.schema),
            Self::Binding(error) => error.fmt(formatter),
            Self::Unavailable => formatter.write_str("document codec registry unavailable"),
        }
    }
}

impl std::error::Error for DocumentCodecRegistryError {}

pub(crate) fn same_document_codec(left: &ArtifactCodec, right: &ArtifactCodec) -> bool {
    left.schema == right.schema
        && left.extension == right.extension
        && left.native_identity == right.native_identity
        && match (left.snapshot_sqlite.as_ref(), right.snapshot_sqlite.as_ref()) {
            (Some(left), Some(right)) => left.identical_to(right),
            (None, None) => true,
            _ => false,
        }
        && left.pack_schema_hash == right.pack_schema_hash
        && std::ptr::fn_addr_eq(left.compile_dsl, right.compile_dsl)
        && std::ptr::fn_addr_eq(left.print_mirror, right.print_mirror)
        && std::ptr::fn_addr_eq(left.edit_text_from_envelope, right.edit_text_from_envelope)
        && std::ptr::fn_addr_eq(left.apply_ops_binary, right.apply_ops_binary)
        && std::ptr::fn_addr_eq(left.replay_envelopes, right.replay_envelopes)
}

pub(crate) fn validate_document_codecs(registry: &BTreeMap<String, ArtifactCodec>, codecs: &[ArtifactCodec]) -> Result<(), DocumentCodecRegistryError> {
    let mut proposed: BTreeMap<&str, &ArtifactCodec> = BTreeMap::new();
    for codec in codecs {
        if !codec.native_identity.validates_provider(&codec.schema, codec.snapshot_sqlite.as_ref()) {
            return Err(DocumentCodecRegistryError::Conflict(DocumentCodecRegistryConflict { schema: codec.schema.clone() }));
        }
        match proposed.get(codec.schema.as_str()) {
            Some(existing) if same_document_codec(existing, codec) => {}
            Some(_) => return Err(DocumentCodecRegistryError::Conflict(DocumentCodecRegistryConflict { schema: codec.schema.clone() })),
            None => {
                proposed.insert(codec.schema.as_str(), codec);
            }
        }
    }
    for (schema, codec) in proposed {
        match registry.get(schema) {
            Some(existing) if same_document_codec(existing, codec) => {}
            Some(_) => return Err(DocumentCodecRegistryError::Conflict(DocumentCodecRegistryConflict { schema: schema.to_string() })),
            None => {}
        }
    }
    Ok(())
}

/// 🔬️ Verifies document codecs against all established schemas without mutating the registry.
#[must_use]
pub async fn preflight_document_codecs(codecs: &[ArtifactCodec]) -> Result<(), DocumentCodecRegistryError> {
    let assembly = semio_framework_schema_registry::assembly::begin().map_err(|_| DocumentCodecRegistryError::Unavailable)?;
    preflight_document_codecs_in_assembly(&assembly, codecs)
}

/// 🔬️ Verifies document codecs while one artifact assembly owns the shared publication barrier.
#[must_use]
pub fn preflight_document_codecs_in_assembly(_assembly: &semio_framework_schema_registry::assembly::Transaction, codecs: &[ArtifactCodec]) -> Result<(), DocumentCodecRegistryError> {
    let registry = document_codec_registry().read().map_err(|_| DocumentCodecRegistryError::Unavailable)?;
    validate_document_codecs(&registry, codecs)
}

/// 📝️ Registers one schema codec exactly once. Collisions fail deterministically before any
/// replacement can occur, so registration order never changes decoding behavior.
#[must_use]
pub fn register_document_codec(codec: ArtifactCodec) -> Result<(), DocumentCodecRegistryError> {
    register_document_codecs(vec![codec])
}

/// 📝️ Registers document codecs only when every descriptor and executable is conflict-free.
#[must_use]
pub fn register_document_codecs(codecs: Vec<ArtifactCodec>) -> Result<(), DocumentCodecRegistryError> {
    let assembly = semio_framework_schema_registry::assembly::begin().map_err(|_| DocumentCodecRegistryError::Unavailable)?;
    register_document_codecs_in_assembly(&assembly, codecs)
}

/// 📝️ Publishes preflighted document codecs while one artifact assembly owns the shared barrier.
#[must_use]
pub fn register_document_codecs_in_assembly(_assembly: &semio_framework_schema_registry::assembly::Transaction, codecs: Vec<ArtifactCodec>) -> Result<(), DocumentCodecRegistryError> {
    let mut registry = document_codec_registry().write().map_err(|_| DocumentCodecRegistryError::Unavailable)?;
    validate_document_codecs(&registry, &codecs)?;
    let mut bindings=crate::io::bindings::registry().write().map_err(|_|DocumentCodecRegistryError::Unavailable)?;
    let rows:Vec<_>=codecs.iter().map(crate::io::ArtifactCodecBinding::schema_only).collect();
    let proposed=crate::io::bindings::propose(&bindings,&rows,&codecs).map_err(DocumentCodecRegistryError::Binding)?;
    for codec in codecs {
        if !registry.contains_key(&codec.schema) {
            registry.insert(codec.schema.clone(), codec);
        }
    }
    bindings.extend(proposed);
    Ok(())
}

/// 🔎️ Looks up the codec registered for `schema`, if any.
#[must_use]
pub async fn document_codec(schema: &str) -> Result<Option<ArtifactCodec>, DocumentCodecRegistryError> {
    let registry = document_codec_registry().read().map_err(|_| DocumentCodecRegistryError::Unavailable)?;
    Ok(registry.get(schema).cloned())
}

/// 🧬️ A component codec answer still in flight. `Send` natively, where document owners run on
/// the shared worker pool; the browser's single isolate needs no `Send`.
#[cfg(not(target_arch = "wasm32"))]
pub type ComponentDocumentCodecFuture<'a, T> = std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, VcsError>> + Send + 'a>>;

#[cfg(target_arch = "wasm32")]
pub type ComponentDocumentCodecFuture<'a, T> = std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, VcsError>> + 'a>>;

/// 🧬️ One document kind's codec as the COMPONENT that owns the kind answers it — the
/// `world actor` `codec` interface (ticket 26/09/18 slice TC3b) — installed by the host that mounted
/// that component. A process that links no Rust codec for a kind identifies and validates its documents
/// through this, exactly as the hub's trusted catalog does for a package it links no codec for
/// (`🌎️hub/🗿️artifact-authority/🔏️trusted-catalog`'s `VerifiedNativeArtifactCodec`: its linked `codec`,
/// else its component `guest`).
pub trait ComponentDocumentCodec: Send + Sync {
    /// 🏷️ The document schema the component owns.
    fn schema(&self) -> &str;
    /// 🧬️ `codec.pack-schema-hash`: the kind's structural snapshot fingerprint.
    fn pack_schema_hash(&self) -> ComponentDocumentCodecFuture<'_, [u8; 32]>;
    /// 📥️ `codec.print-mirror`: the pair's text mirror, which is also its validation fence.
    fn print_mirror<'a>(&'a self, pack: &'a [u8], spr: &'a [u8]) -> ComponentDocumentCodecFuture<'a, ArtifactTextFiles>;
}

/// 🧭️ The one codec a document owner resolves for a schema: the Rust codec this binary links
/// when it links one, else the component codec installed for the mounted component that owns the kind.
#[derive(Clone)]
pub enum DocumentKindCodec {
    Linked(ArtifactCodec),
    Component(Arc<dyn ComponentDocumentCodec>),
}

impl DocumentKindCodec {
    /// 🧬️ The kind's structural snapshot fingerprint.
    pub async fn pack_schema_hash(&self) -> Result<[u8; 32], VcsError> {
        match self {
            Self::Linked(codec) => Ok(codec.pack_schema_hash),
            Self::Component(codec) => codec.pack_schema_hash().await,
        }
    }

    /// 📥️ The pair's text mirror; an `Err` means the pair is not a document of this kind.
    pub async fn print_mirror(&self, pack: &[u8], spr: &[u8]) -> Result<ArtifactTextFiles, VcsError> {
        match self {
            Self::Linked(codec) => (codec.print_mirror)(pack, spr).await,
            Self::Component(codec) => codec.print_mirror(pack, spr).await,
        }
    }
}

pub(crate) static COMPONENT_DOCUMENT_CODEC_REGISTRY: std::sync::OnceLock<std::sync::RwLock<BTreeMap<String, Arc<dyn ComponentDocumentCodec>>>> = std::sync::OnceLock::new();

pub(crate) fn component_document_codec_registry() -> &'static std::sync::RwLock<BTreeMap<String, Arc<dyn ComponentDocumentCodec>>> {
    COMPONENT_DOCUMENT_CODEC_REGISTRY.get_or_init(|| std::sync::RwLock::new(BTreeMap::new()))
}

/// 📝️ Installs a mounted component's codec for the schema it owns. A later mount of that kind's
/// owner replaces an earlier one: in this process the component a host mounted IS the kind's owner.
#[must_use]
pub fn register_component_document_codec(codec: Arc<dyn ComponentDocumentCodec>) -> Result<(), DocumentCodecRegistryError> {
    let mut registry = component_document_codec_registry().write().map_err(|_| DocumentCodecRegistryError::Unavailable)?;
    registry.insert(codec.schema().to_string(), codec);
    Ok(())
}

/// 🔎️ Resolves `schema` to the one codec a document owner uses — see [`DocumentKindCodec`].
#[must_use]
pub async fn document_kind_codec(schema: &str) -> Result<Option<DocumentKindCodec>, DocumentCodecRegistryError> {
    if let Some(codec) = document_codec(schema).await? {
        return Ok(Some(DocumentKindCodec::Linked(codec)));
    }
    let registry = component_document_codec_registry().read().map_err(|_| DocumentCodecRegistryError::Unavailable)?;
    Ok(registry.get(schema).cloned().map(DocumentKindCodec::Component))
}

/// 📜️ Reads the document schema id from an encoded `.spr` history log.
pub async fn lane_schema_from_spr(spr: &[u8]) -> Option<String> {
    if spr.is_empty() {
        return None;
    }
    crate::os_spr::decode_history(spr, &crate::os_spr::DecodeOptions::default()).await.ok().map(|log| log.schema).filter(|schema| !schema.is_empty())
}

/// 🧬️ One registered lossy/lossless pack-bytes transform between two dialects of the SAME
/// artifact kind (`from.artifact_kind == to.artifact_kind` is a convention this registry doesn't
/// itself enforce — callers key by the exact `(from, to)` pair they registered). `migrate_pack` is
/// a bare `fn` pointer (same non-capturing-bridge-function shape `artifact_codec` already uses
/// for `compile_dsl`/`print_mirror`/etc above) so a migration can be a plain top-level fn with zero
/// closure-capture ceremony. Additive-only: nothing in `ArtifactCodec`/`document_codec`/
/// `ArtifactStore::dispatch` reads this registry yet — see `26/08/10` D4 evolution slice scope note.
#[derive(Clone)]
pub struct DialectMigration {
    pub from: semio_framework_artifact_reference::ArtifactDialect,
    pub to: semio_framework_artifact_reference::ArtifactDialect,
    pub lossless: bool,
    pub migrate_pack: fn(&[u8]) -> Result<Vec<u8>, String>,
}

pub(crate) static DIALECT_MIGRATION_REGISTRY: std::sync::OnceLock<std::sync::RwLock<BTreeMap<(semio_framework_artifact_reference::ArtifactDialect, semio_framework_artifact_reference::ArtifactDialect), DialectMigration>>> = std::sync::OnceLock::new();

pub(crate) fn dialect_migration_registry() -> &'static std::sync::RwLock<BTreeMap<(semio_framework_artifact_reference::ArtifactDialect, semio_framework_artifact_reference::ArtifactDialect), DialectMigration>> {
    DIALECT_MIGRATION_REGISTRY.get_or_init(|| std::sync::RwLock::new(BTreeMap::new()))
}

/// ⚠️ Dialect-migration registration cannot overwrite an owner or cross artifact kinds.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DialectMigrationRegistryError {
    CrossArtifactKind { from: semio_framework_artifact_reference::ArtifactDialect, to: semio_framework_artifact_reference::ArtifactDialect },
    Conflict { from: semio_framework_artifact_reference::ArtifactDialect, to: semio_framework_artifact_reference::ArtifactDialect },
    Unavailable,
}

impl std::fmt::Display for DialectMigrationRegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CrossArtifactKind { from, to } => write!(f, "dialect migration crosses artifact kinds from {from:?} to {to:?}"),
            Self::Conflict { from, to } => write!(f, "dialect migration conflict from {from:?} to {to:?}"),
            Self::Unavailable => write!(f, "dialect migration registry unavailable"),
        }
    }
}

impl std::error::Error for DialectMigrationRegistryError {}

/// 🚫️ A migration was unavailable or the registered executable rejected its input.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DialectMigrationError {
    Missing { from: semio_framework_artifact_reference::ArtifactDialect, to: semio_framework_artifact_reference::ArtifactDialect },
    Unavailable,
    Rejected(String),
}

pub(crate) fn same_dialect_migration(left: &DialectMigration, right: &DialectMigration) -> bool {
    left.from == right.from && left.to == right.to && left.lossless == right.lossless && std::ptr::fn_addr_eq(left.migrate_pack, right.migrate_pack)
}

pub(crate) fn validate_dialect_migrations(registry: &BTreeMap<(semio_framework_artifact_reference::ArtifactDialect, semio_framework_artifact_reference::ArtifactDialect), DialectMigration>, migrations: &[DialectMigration]) -> Result<(), DialectMigrationRegistryError> {
    let mut proposed: BTreeMap<(semio_framework_artifact_reference::ArtifactDialect, semio_framework_artifact_reference::ArtifactDialect), &DialectMigration> = BTreeMap::new();
    for migration in migrations {
        if migration.from.artifact_kind != migration.to.artifact_kind {
            return Err(DialectMigrationRegistryError::CrossArtifactKind { from: migration.from.clone(), to: migration.to.clone() });
        }
        let key = (migration.from.clone(), migration.to.clone());
        match proposed.get(&key) {
            Some(existing) if same_dialect_migration(existing, migration) => {}
            Some(_) => return Err(DialectMigrationRegistryError::Conflict { from: key.0, to: key.1 }),
            None => {
                proposed.insert(key, migration);
            }
        }
    }
    for (key, migration) in proposed {
        match registry.get(&key) {
            Some(existing) if same_dialect_migration(existing, migration) => {}
            Some(_) => return Err(DialectMigrationRegistryError::Conflict { from: key.0, to: key.1 }),
            None => {}
        }
    }
    Ok(())
}

/// 🔬️ Verifies a migration set against established dialect pairs without mutation.
#[must_use]
pub async fn preflight_dialect_migrations(migrations: &[DialectMigration]) -> Result<(), DialectMigrationRegistryError> {
    let assembly = semio_framework_schema_registry::assembly::begin().map_err(|_| DialectMigrationRegistryError::Unavailable)?;
    preflight_dialect_migrations_in_assembly(&assembly, migrations).await
}

/// 🔬️ Verifies migrations while one artifact assembly owns the shared publication barrier.
#[must_use]
pub async fn preflight_dialect_migrations_in_assembly(_assembly: &semio_framework_schema_registry::assembly::Transaction, migrations: &[DialectMigration]) -> Result<(), DialectMigrationRegistryError> {
    let registry = dialect_migration_registry().read().map_err(|_| DialectMigrationRegistryError::Unavailable)?;
    validate_dialect_migrations(&registry, migrations)
}

/// 📝️ Registers a migration only when its full descriptor and executable identity match.
#[must_use]
pub fn register_dialect_migration(migration: DialectMigration) -> Result<(), DialectMigrationRegistryError> {
    register_dialect_migrations(vec![migration])
}

/// 📝️ Registers migrations only when every candidate pair is conflict-free.
#[must_use]
pub fn register_dialect_migrations(migrations: Vec<DialectMigration>) -> Result<(), DialectMigrationRegistryError> {
    let assembly = semio_framework_schema_registry::assembly::begin().map_err(|_| DialectMigrationRegistryError::Unavailable)?;
    register_dialect_migrations_in_assembly(&assembly, migrations)
}

/// 📝️ Publishes preflighted migrations while one artifact assembly owns the shared barrier.
#[must_use]
pub fn register_dialect_migrations_in_assembly(_assembly: &semio_framework_schema_registry::assembly::Transaction, migrations: Vec<DialectMigration>) -> Result<(), DialectMigrationRegistryError> {
    let mut registry = dialect_migration_registry().write().map_err(|_| DialectMigrationRegistryError::Unavailable)?;
    validate_dialect_migrations(&registry, &migrations)?;
    for migration in migrations {
        let key = (migration.from.clone(), migration.to.clone());
        registry.entry(key).or_insert(migration);
    }
    Ok(())
}

/// 🔁️ Looks up the exact `(from, to)` migration and runs its `migrate_pack` over
/// `pack_bytes`, or a clear `Err` naming both dialect coordinates when none is registered.
#[must_use]
pub async fn migrate_document(from: &semio_framework_artifact_reference::ArtifactDialect, to: &semio_framework_artifact_reference::ArtifactDialect, pack_bytes: &[u8]) -> Result<Vec<u8>, DialectMigrationError> {
    let registry = dialect_migration_registry().read().map_err(|_| DialectMigrationError::Unavailable)?;
    let migration = registry.get(&(from.clone(), to.clone())).ok_or_else(|| DialectMigrationError::Missing { from: from.clone(), to: to.clone() })?;
    (migration.migrate_pack)(pack_bytes).map_err(DialectMigrationError::Rejected)
}

/// 🧷️ All writable store registries held before an artifact assembly can publish anything.
pub struct ArtifactAssemblyStoreRegistryGuards<'assembly> {
    document_codecs: std::sync::RwLockWriteGuard<'static, BTreeMap<String, ArtifactCodec>>,
    dialect_migrations: std::sync::RwLockWriteGuard<'static, BTreeMap<(semio_framework_artifact_reference::ArtifactDialect, semio_framework_artifact_reference::ArtifactDialect), DialectMigration>>,
    _assembly: &'assembly semio_framework_schema_registry::assembly::Transaction,
}

/// 🚫️ A staged store registry assembly cannot be preflighted or committed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArtifactAssemblyStoreRegistryError {
    DocumentCodec(DocumentCodecRegistryError),
    DialectMigration(DialectMigrationRegistryError),
}

impl std::fmt::Display for ArtifactAssemblyStoreRegistryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DocumentCodec(error) => error.fmt(formatter),
            Self::DialectMigration(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for ArtifactAssemblyStoreRegistryError {}

/// 🧷️ Acquires every store registry write lock before an assembly validates or mutates state.
#[must_use]
pub fn acquire_artifact_assembly_store_registry_guards<'assembly>(assembly: &'assembly semio_framework_schema_registry::assembly::Transaction) -> Result<ArtifactAssemblyStoreRegistryGuards<'assembly>, ArtifactAssemblyStoreRegistryError> {
    let document_codecs = document_codec_registry().write().map_err(|_| ArtifactAssemblyStoreRegistryError::DocumentCodec(DocumentCodecRegistryError::Unavailable))?;
    let dialect_migrations = dialect_migration_registry().write().map_err(|_| ArtifactAssemblyStoreRegistryError::DialectMigration(DialectMigrationRegistryError::Unavailable))?;
    Ok(ArtifactAssemblyStoreRegistryGuards { document_codecs, dialect_migrations, _assembly: assembly })
}

/// 🔬️ Verifies all staged store rows while every affected write lock is already held.
#[must_use]
pub fn preflight_artifact_assembly_store_registry_guards(guards: &ArtifactAssemblyStoreRegistryGuards<'_>, document_codecs: &[ArtifactCodec], dialect_migrations: &[DialectMigration]) -> Result<(), ArtifactAssemblyStoreRegistryError> {
    validate_document_codecs(&guards.document_codecs, document_codecs).map_err(ArtifactAssemblyStoreRegistryError::DocumentCodec)?;
    validate_dialect_migrations(&guards.dialect_migrations, dialect_migrations).map_err(ArtifactAssemblyStoreRegistryError::DialectMigration)
}

/// 📌️ Publishes rows proven safe by `preflight_artifact_assembly_store_registry_guards`.
pub fn commit_artifact_assembly_store_registry_guards(guards: &mut ArtifactAssemblyStoreRegistryGuards<'_>, document_codecs: Vec<ArtifactCodec>, dialect_migrations: Vec<DialectMigration>) {
    for codec in document_codecs {
        guards.document_codecs.entry(codec.schema.clone()).or_insert(codec);
    }
    for migration in dialect_migrations {
        let key = (migration.from.clone(), migration.to.clone());
        guards.dialect_migrations.entry(key).or_insert(migration);
    }
}
