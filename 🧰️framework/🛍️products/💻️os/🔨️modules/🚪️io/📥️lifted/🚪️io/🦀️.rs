//! 📥️ Items of `composer_registry` that belong to `os-io` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/📦️codec/📥️lifted/🚪️io/🦀️.rs`.
// 📥️ Lifted from `🚪️io/🦀️.rs` by S0: the composer registry, `IoKey`, subset validators, format catalog and the native-snapshot registration plan that belong to `os-store-codec-registry`.
// The `io` module mounts this file as a module and re-exports it, so every public item keeps its `io` path until the file is the codec-registry crate.
use semio_framework_os_store_codec_registry::composer_registry::bindings::catalog::{ArtifactCatalogBinding, ArtifactCatalogBindingError, ArtifactCatalogCapability, ArtifactCatalogTarget, artifact_catalog_bindings, preflight_artifact_catalog_bindings_in_assembly};
use semio_framework_os_store_codec_registry::composer_registry::bindings::{ArtifactCodecBinding, ArtifactCodecBindingChannel, ArtifactCodecBindingError, artifact_codec_bindings, preflight_artifact_codec_bindings, preflight_artifact_codec_bindings_in_assembly};
use semio_framework_artifact_reference::{ArtifactDialect, Dialect};
use std::collections::BTreeMap;
use std::sync::RwLock;
use crate::io_mechanism;
use semio_framework_os_store_codec_registry::composer_registry::ComposerEntry;
use semio_framework_os_store_codec_registry::composer_registry::io_registry;
use semio_framework_os_store_codec_registry::composer_registry::IoRegistryUnavailable;
use semio_framework_os_store_codec_registry::composer_registry::IoRegistryRegistrationError;
use semio_framework_os_store_codec_registry::composer_registry::composer_entries_by_key;
use semio_framework_os_store_codec_registry::composer_registry::validate_composer_entries;
use semio_framework_os_store_codec_registry::composer_registry::SubsetValidator;
use semio_framework_os_store_codec_registry::composer_registry::SubsetValidatorEntry;
use semio_framework_os_store_codec_registry::composer_registry::subset_validator_registry;
use semio_framework_os_store_codec_registry::composer_registry::SubsetValidatorRegistryError;
use semio_framework_os_store_codec_registry::composer_registry::validate_subset_validators;
use semio_framework_os_store_codec_registry::composer_registry::FormatDescriptor;
use semio_framework_os_store_codec_registry::composer_registry::FormatRegistryError;
use semio_framework_os_store_codec_registry::composer_registry::format_catalog;
use semio_framework_os_store_codec_registry::composer_registry::index_format_descriptors;
use semio_framework_os_store_codec_registry::composer_registry::validate_format_descriptors;
use semio_framework_os_store_codec_registry::composer_registry::bindings;
//#region 🔖️ErasedRegistry
//#region 🔖️FormatCatalog
#[derive(Default)]
pub struct ArtifactAssemblyRegistryPlan {
    pub composer_entries: Vec<&'static ComposerEntry>,
    pub subset_validators: Vec<&'static SubsetValidatorEntry>,
    pub format_descriptors: Vec<FormatDescriptor>,
    pub document_codecs: Vec<semio_framework_os_store_text_io::ArtifactCodec>,
    pub dialect_migrations: Vec<semio_framework_os_store_codec_registry::DialectMigration>,
    pub native_snapshots: Vec<io_mechanism::NativeSnapshotRegistration>,
    pub document_bindings: Vec<ArtifactCodecBinding>,
    pub catalog_bindings: Vec<ArtifactCatalogBinding>,
}

impl ArtifactAssemblyRegistryPlan {
    /// 🌱️ Starts an empty plan; callers append every owned registry row before committing once.
    pub async fn new() -> Self {
        Self { composer_entries: Vec::new(), subset_validators: Vec::new(), format_descriptors: Vec::new(), document_codecs: Vec::new(), dialect_migrations: Vec::new(), native_snapshots: Vec::new(), document_bindings: Vec::new(), catalog_bindings:Vec::new() }
    }
}

/// 🚫️ An all-registry assembly cannot acquire its locks or pass preflight.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArtifactAssemblyRegistryError {
    Transaction(semio_framework_schema_registry::assembly::Error),
    Composer(Box<IoRegistryRegistrationError>),
    SubsetValidator(SubsetValidatorRegistryError),
    Format(FormatRegistryError),
    Store(Box<semio_framework_os_store_codec_registry::ArtifactAssemblyStoreRegistryError>),
    NativeSnapshot(io_mechanism::IoRegistryError),
    Binding(ArtifactCodecBindingError),
    Catalog(ArtifactCatalogBindingError),
}

impl std::fmt::Display for ArtifactAssemblyRegistryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Transaction(error) => error.fmt(formatter),
            Self::Composer(error) => error.fmt(formatter),
            Self::SubsetValidator(error) => error.fmt(formatter),
            Self::Format(error) => error.fmt(formatter),
            Self::Store(error) => error.fmt(formatter),
            Self::NativeSnapshot(error) => error.fmt(formatter),
            Self::Binding(error) => error.fmt(formatter),
            Self::Catalog(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for ArtifactAssemblyRegistryError {}

/// 🧰️ Publishes an imperative native codec and its exact snapshot dialect atomically.
pub fn register_native_snapshot_codec(dialect: Dialect, codec: semio_framework_os_store_text_io::ArtifactCodec) -> Result<(), ArtifactAssemblyRegistryError> {
    let assembly = semio_framework_schema_registry::assembly::begin().map_err(ArtifactAssemblyRegistryError::Transaction)?;
    let contributor=match &codec.native_identity{semio_framework_os_store_text_io::ArtifactNativeSnapshotIdentity::Typed{owner,..}=>owner.to_string(),semio_framework_os_store_text_io::ArtifactNativeSnapshotIdentity::Guest{plugin_id,..}=>plugin_id.clone()};
    let binding=ArtifactCodecBinding::from_codec(ArtifactCodecBindingChannel::DirectNative,contributor,Some(dialect.artifact_kind.to_string()),Vec::new(),Some(dialect.into()),None,&codec);
    let plan = ArtifactAssemblyRegistryPlan { document_bindings:vec![binding],document_codecs: vec![codec.clone()], native_snapshots: vec![io_mechanism::NativeSnapshotRegistration { dialect: dialect.into(), codec }], ..Default::default() };
    commit_artifact_assembly_registry_plan(&assembly, plan)
}

/// 📣️ Publishes the exact native registration a snapshot owner declares through
/// [`pack::snapshot::ArtifactPack::native_snapshot_registration`]; an owner that declares none publishes nothing.
pub fn publish_native_snapshot<P: semio_framework_os_store_text_io::ArtifactStoreSnapshot>() -> Result<(), ArtifactAssemblyRegistryError> {
    match P::native_snapshot_registration() {
        Some((dialect, codec)) => register_native_snapshot_codec(dialect, codec),
        None => Ok(()),
    }
}

/// 📦️ Publishes a native document codec and any actual owner-declared relational capability.
pub fn register_native_document_codec(dialect: Dialect, codec: semio_framework_os_store_text_io::ArtifactCodec) -> Result<(), ArtifactAssemblyRegistryError> {
    let assembly = semio_framework_schema_registry::assembly::begin().map_err(ArtifactAssemblyRegistryError::Transaction)?;
    let contributor=match &codec.native_identity{semio_framework_os_store_text_io::ArtifactNativeSnapshotIdentity::Typed{owner,..}=>owner.to_string(),semio_framework_os_store_text_io::ArtifactNativeSnapshotIdentity::Guest{plugin_id,..}=>plugin_id.clone()};
    let binding=ArtifactCodecBinding::from_codec(ArtifactCodecBindingChannel::DirectDocumentNative,contributor,Some(dialect.artifact_kind.to_string()),Vec::new(),Some(dialect.into()),None,&codec);
    let plan = ArtifactAssemblyRegistryPlan { document_bindings:vec![binding],document_codecs: vec![codec.clone()], native_snapshots: io_mechanism::NativeSnapshotRegistration::from_capability(dialect.into(), codec).into_iter().collect(), ..Default::default() };
    commit_artifact_assembly_registry_plan(&assembly, plan)
}

/// 📌️ Acquires every affected write lock, preflights every candidate, then commits without any
/// fallible operation after the first registry mutation.
pub fn commit_artifact_assembly_registry_plan(assembly: &semio_framework_schema_registry::assembly::Transaction, plan: ArtifactAssemblyRegistryPlan) -> Result<(), ArtifactAssemblyRegistryError> {
    let mut store_guards = semio_framework_os_store_codec_registry::acquire_artifact_assembly_store_registry_guards(assembly).map_err(|error| ArtifactAssemblyRegistryError::Store(Box::new(error)))?;
    let mut composers = io_registry().write().map_err(|_| ArtifactAssemblyRegistryError::Composer(Box::new(IoRegistryRegistrationError::Unavailable(IoRegistryUnavailable { registry: "io-composer" }))))?;
    let mut subset_validators = subset_validator_registry().write().map_err(|_| ArtifactAssemblyRegistryError::SubsetValidator(SubsetValidatorRegistryError::Unavailable(IoRegistryUnavailable { registry: "subset-validator" })))?;
    let mut formats = format_catalog().write().map_err(|_| ArtifactAssemblyRegistryError::Format(FormatRegistryError::Unavailable(IoRegistryUnavailable { registry: "format-catalog" })))?;
    let mut native_snapshots = io_mechanism::native_snapshot_registry().write().map_err(|_| ArtifactAssemblyRegistryError::NativeSnapshot(io_mechanism::IoRegistryError::Unavailable))?;
    let mut bindings=semio_framework_os_store_codec_registry::composer_registry::bindings::registry().write().map_err(|_|ArtifactAssemblyRegistryError::Binding(ArtifactCodecBindingError::Unavailable))?;
    let mut catalog=semio_framework_os_store_codec_registry::composer_registry::bindings::catalog::registry().write().map_err(|_|ArtifactAssemblyRegistryError::Catalog(ArtifactCatalogBindingError::Unavailable))?;
    for codec in &plan.document_codecs {
        if !plan.document_bindings.iter().any(|row|row.validates(codec)) { return Err(ArtifactAssemblyRegistryError::Binding(ArtifactCodecBindingError::Invalid{schema:codec.schema.clone()})); }
    }
    for registration in &plan.native_snapshots {
        if !plan.document_bindings.iter().any(|row|row.dialect.as_ref()==Some(&registration.dialect)&&row.validates(&registration.codec)) { return Err(ArtifactAssemblyRegistryError::Binding(ArtifactCodecBindingError::Invalid{schema:registration.codec.schema.clone()})); }
    }
    for row in &plan.document_bindings {
        if let Some(dialect)=row.dialect.as_ref().filter(|_|row.sqlite_schema.is_some()) {
            if !plan.native_snapshots.iter().any(|registration|&registration.dialect==dialect&&row.validates(&registration.codec))&&!native_snapshots.get(dialect).is_some_and(|codec|row.validates(codec)) { return Err(ArtifactAssemblyRegistryError::Binding(ArtifactCodecBindingError::Invalid{schema:row.schema.clone()})); }
        }
    }
    let proposed_bindings=semio_framework_os_store_codec_registry::composer_registry::bindings::propose(&bindings,&plan.document_bindings,&plan.document_codecs).map_err(ArtifactAssemblyRegistryError::Binding)?;
    let proposed_catalog=semio_framework_os_store_codec_registry::composer_registry::bindings::catalog::propose(&catalog,&plan.catalog_bindings,&plan.document_codecs).map_err(ArtifactAssemblyRegistryError::Catalog)?;
    let proposed_snapshots = io_mechanism::propose_native_snapshots(&native_snapshots, &plan.native_snapshots).map_err(ArtifactAssemblyRegistryError::NativeSnapshot)?;
    let proposed_composers = composer_entries_by_key(plan.composer_entries.iter().copied()).map_err(|error| ArtifactAssemblyRegistryError::Composer(Box::new(error)))?;
    validate_composer_entries(&composers, &proposed_composers).map_err(|error| ArtifactAssemblyRegistryError::Composer(Box::new(error)))?;
    validate_subset_validators(&subset_validators, &plan.subset_validators).map_err(ArtifactAssemblyRegistryError::SubsetValidator)?;
    let (proposed_formats, proposed_formats_by_kind) = index_format_descriptors(plan.format_descriptors.iter().cloned()).map_err(|error| ArtifactAssemblyRegistryError::Format(FormatRegistryError::Conflict(error)))?;
    validate_format_descriptors(&formats, &proposed_formats, &proposed_formats_by_kind).map_err(|error| ArtifactAssemblyRegistryError::Format(FormatRegistryError::Conflict(error)))?;
    semio_framework_os_store_codec_registry::preflight_artifact_assembly_store_registry_guards(&store_guards, &plan.document_codecs, &plan.dialect_migrations).map_err(|error| ArtifactAssemblyRegistryError::Store(Box::new(error)))?;
    for (key, entry) in proposed_composers {
        composers.entry(key).or_insert(entry);
    }
    for entry in plan.subset_validators {
        subset_validators.entry(ArtifactDialect::from(entry.dialect)).or_insert(entry);
    }
    for (key, descriptor) in proposed_formats {
        formats.entry(key).or_insert(descriptor);
    }
    semio_framework_os_store_codec_registry::commit_artifact_assembly_store_registry_guards(&mut store_guards, plan.document_codecs, plan.dialect_migrations);
    native_snapshots.extend(proposed_snapshots);
    bindings.extend(proposed_bindings);
    catalog.extend(proposed_catalog);
    Ok(())
}
//#endregion 🔖️FormatCatalog
//#endregion 🔖️ErasedRegistry
