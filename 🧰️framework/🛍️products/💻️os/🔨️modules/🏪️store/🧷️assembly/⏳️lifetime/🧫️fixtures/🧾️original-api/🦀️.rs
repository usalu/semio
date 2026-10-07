/// 🧷️ One process-wide guard for a plugin's all-registry publication phase.
pub struct ArtifactAssemblyTransaction {
    _guard: MutexGuard<'static, ()>,
}

/// 🚫️ The all-registry publication barrier is unavailable after a writer panic.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArtifactAssemblyTransactionError {
    Unavailable,
}

impl std::fmt::Display for ArtifactAssemblyTransactionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("artifact assembly transaction unavailable")
    }
}

impl std::error::Error for ArtifactAssemblyTransactionError {}

fn artifact_assembly_lock() -> &'static Mutex<()> {
    static LOCK: std::sync::OnceLock<Mutex<()>> = std::sync::OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

/// 🧷️ Begins the only cross-registry transaction accepted by artifact registration APIs.
#[must_use]
pub fn begin_artifact_assembly() -> Result<ArtifactAssemblyTransaction, ArtifactAssemblyTransactionError> {
    Ok(ArtifactAssemblyTransaction { _guard: artifact_assembly_lock().lock().map_err(|_| ArtifactAssemblyTransactionError::Unavailable)? })
}

/// 🧷️ All writable store registries held before an artifact assembly can publish anything.
pub struct ArtifactAssemblyStoreRegistryGuards {
    document_codecs: std::sync::RwLockWriteGuard<'static, BTreeMap<String, ArtifactCodec>>,
    dialect_migrations: std::sync::RwLockWriteGuard<'static, BTreeMap<(semio_framework_artifact_reference::ArtifactDialect, semio_framework_artifact_reference::ArtifactDialect), DialectMigration>>,
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
pub fn acquire_artifact_assembly_store_registry_guards(_assembly: &ArtifactAssemblyTransaction) -> Result<ArtifactAssemblyStoreRegistryGuards, ArtifactAssemblyStoreRegistryError> {
    let document_codecs = document_codec_registry().write().map_err(|_| ArtifactAssemblyStoreRegistryError::DocumentCodec(DocumentCodecRegistryError::Unavailable))?;
    let dialect_migrations = dialect_migration_registry().write().map_err(|_| ArtifactAssemblyStoreRegistryError::DialectMigration(DialectMigrationRegistryError::Unavailable))?;
    Ok(ArtifactAssemblyStoreRegistryGuards { document_codecs, dialect_migrations })
}

