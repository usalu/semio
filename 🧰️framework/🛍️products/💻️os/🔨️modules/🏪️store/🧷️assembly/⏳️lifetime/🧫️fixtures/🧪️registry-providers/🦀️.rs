use std::collections::BTreeMap;
use std::sync::{Mutex, MutexGuard, RwLock};
#[derive(Clone)] struct ArtifactCodec;
#[derive(Clone)] struct DialectMigration;
mod os_io { #[derive(Clone, PartialEq, Eq, PartialOrd, Ord)] pub struct ArtifactDialect; }
#[derive(Clone, Debug, PartialEq, Eq)] pub enum DocumentCodecRegistryError { Unavailable }
#[derive(Clone, Debug, PartialEq, Eq)] pub enum DialectMigrationRegistryError { Unavailable }
impl std::fmt::Display for DocumentCodecRegistryError { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str("document unavailable") } }
impl std::fmt::Display for DialectMigrationRegistryError { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str("migration unavailable") } }
fn document_codec_registry() -> &'static RwLock<BTreeMap<String, ArtifactCodec>> { static VALUE: std::sync::OnceLock<RwLock<BTreeMap<String, ArtifactCodec>>> = std::sync::OnceLock::new(); VALUE.get_or_init(|| RwLock::new(BTreeMap::new())) }
fn dialect_migration_registry() -> &'static RwLock<BTreeMap<(os_io::ArtifactDialect, os_io::ArtifactDialect), DialectMigration>> { static VALUE: std::sync::OnceLock<RwLock<BTreeMap<(os_io::ArtifactDialect, os_io::ArtifactDialect), DialectMigration>>> = std::sync::OnceLock::new(); VALUE.get_or_init(|| RwLock::new(BTreeMap::new())) }

