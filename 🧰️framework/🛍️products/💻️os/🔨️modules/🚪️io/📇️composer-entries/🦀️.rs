//! 📇️ Composer entries: the composer entry preflight, registration and listing.
//!
//! Holds items of the module; the root re-exports them.

use super::*;

/// 🔬️ Verifies a static composer table against all established keys without mutating the registry.
pub async fn preflight_composer_entries(entries: &'static [ComposerEntry]) -> Result<(), IoRegistryRegistrationError> {
    preflight_composer_entry_refs(&entries.iter().collect::<Vec<_>>()).await
}

/// 🔬️ Verifies independently declared static composers as one atomic candidate set.
pub async fn preflight_composer_entry_refs(entries: &[&'static ComposerEntry]) -> Result<(), IoRegistryRegistrationError> {
    let assembly = semio_framework_schema_registry::assembly::begin().map_err(|_| IoRegistryRegistrationError::Unavailable(IoRegistryUnavailable { registry: "artifact-assembly" }))?;
    preflight_composer_entry_refs_in_assembly(&assembly, entries).await
}

/// 🔬️ Verifies composers while one artifact assembly owns the shared publication barrier.
pub async fn preflight_composer_entry_refs_in_assembly(_assembly: &semio_framework_schema_registry::assembly::Transaction, entries: &[&'static ComposerEntry]) -> Result<(), IoRegistryRegistrationError> {
    let proposed = composer_entries_by_key(entries.iter().copied())?;
    let registry = io_registry().read().map_err(|_| IoRegistryRegistrationError::Unavailable(IoRegistryUnavailable { registry: "io-composer" }))?;
    validate_composer_entries(&registry, &proposed)
}

/// 📌️ Registers one artifact's composer entries atomically. Re-registering the exact static entry
/// is idempotent; a different entry for any exact key fails and leaves the registry unchanged.
pub fn register_composer_entries(entries: &'static [ComposerEntry]) -> Result<(), IoRegistryRegistrationError> {
    register_composer_entry_refs(&entries.iter().collect::<Vec<_>>())
}

/// 📇️ Every registered composer entry, erased to owned dialects -- the shape the WIT
/// `list-artifact-dialects` guest export mirrors verbatim (one row per distinct `writes` entry
/// registered locally, each carrying the full `reads` list).
pub async fn list_composer_entries() -> Result<Vec<(ArtifactDialect, Vec<ArtifactDialect>)>, IoRegistryUnavailable> {
    let reg = io_registry().read().map_err(|_| IoRegistryUnavailable { registry: "io-composer" })?;
    Ok(composer_dialect_rows(reg.values().copied()))
}

fn composer_dialect_rows<'entry>(entries: impl IntoIterator<Item = &'entry ComposerEntry>) -> Vec<(ArtifactDialect, Vec<ArtifactDialect>)> {
    let mut rows: BTreeMap<ArtifactDialect, std::collections::BTreeSet<ArtifactDialect>> = BTreeMap::new();
    for entry in entries {
        rows.entry(entry.writes.into()).or_default().extend(entry.reads.iter().copied().map(ArtifactDialect::from));
    }
    rows.into_iter().map(|(writes, reads)| (writes, reads.into_iter().collect())).collect()
}

#[cfg(test)]
#[path = "🧪️tests/route-discovery.rs"]
mod route_discovery_tests;
