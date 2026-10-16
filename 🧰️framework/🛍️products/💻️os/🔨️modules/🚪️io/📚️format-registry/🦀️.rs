//! 📚️ Format registry: the atomic format descriptor registration, lookup and file-picker filter.
//!
//! Holds the `🔖️FormatCatalog` region of the module; the root re-exports its items.

use super::*;

//#region 🔖️FormatCatalog









/// 📌️ Registers format rows atomically. Identity, extension, and non-empty MIME claims are each
/// globally singular; equal duplicate rows are idempotent and never replace an established owner.
pub async fn register_format_descriptors(descriptors: impl IntoIterator<Item = FormatDescriptor>) -> Result<(), FormatRegistryError> {
    let assembly = semio_framework_schema_registry::assembly::begin().map_err(|_| FormatRegistryError::Unavailable(IoRegistryUnavailable { registry: "artifact-assembly" }))?;
    register_format_descriptors_in_assembly(&assembly, descriptors)
}

/// 📌️ Publishes preflighted format rows while one artifact assembly owns the shared barrier.
pub fn register_format_descriptors_in_assembly(_assembly: &semio_framework_schema_registry::assembly::Transaction, descriptors: impl IntoIterator<Item = FormatDescriptor>) -> Result<(), FormatRegistryError> {
    let (proposed, proposed_by_kind) = index_format_descriptors(descriptors).map_err(FormatRegistryError::Conflict)?;
    let mut registry = format_catalog().write().map_err(|_| FormatRegistryError::Unavailable(IoRegistryUnavailable { registry: "format-catalog" }))?;
    validate_format_descriptors(&registry, &proposed, &proposed_by_kind).map_err(FormatRegistryError::Conflict)?;
    for (key, descriptor) in proposed {
        registry.entry(key).or_insert(descriptor);
    }
    Ok(())
}

/// 🔬️ Verifies format rows against the catalog without mutating their global ownership.
pub async fn preflight_format_descriptors(rows: &[FormatDescriptor]) -> Result<(), FormatRegistryError> {
    let assembly = semio_framework_schema_registry::assembly::begin().map_err(|_| FormatRegistryError::Unavailable(IoRegistryUnavailable { registry: "artifact-assembly" }))?;
    preflight_format_descriptors_in_assembly(&assembly, rows)
}

/// 🔬️ Verifies format rows while one artifact assembly owns the shared publication barrier.
pub fn preflight_format_descriptors_in_assembly(_assembly: &semio_framework_schema_registry::assembly::Transaction, rows: &[FormatDescriptor]) -> Result<(), FormatRegistryError> {
    let (proposed, proposed_by_kind) = index_format_descriptors(rows.iter().cloned()).map_err(FormatRegistryError::Conflict)?;
    let registry = format_catalog().read().map_err(|_| FormatRegistryError::Unavailable(IoRegistryUnavailable { registry: "format-catalog" }))?;
    validate_format_descriptors(&registry, &proposed, &proposed_by_kind).map_err(FormatRegistryError::Conflict)
}







/// 🔎️ Resolves a format by its `kind_id`, `short_id`, or registered alias.
pub fn format_descriptor(kind_or_short_or_alias: &str) -> Result<Option<FormatDescriptor>, FormatRegistryError> {
    let registry = format_catalog().read().map_err(|_| FormatRegistryError::Unavailable(IoRegistryUnavailable { registry: "format-catalog" }))?;
    Ok(registry.get(kind_or_short_or_alias).cloned())
}

/// 🏷️ Normalize any recognized form (kind id, short id, alias) to the canonical `kind_id`.
pub fn normalize_format_kind(input: &str) -> Result<Option<String>, FormatRegistryError> {
    Ok(format_descriptor(input)?.map(|descriptor| descriptor.kind_id))
}

/// 🗂️ File-picker `accept` filter (comma-joined extensions) for a list of kind/short/alias
/// strings -- the generic successor to `mesh::stdio_accept_filter`.
pub fn format_accept_filter(kind_ids: &[&str]) -> Result<String, FormatRegistryError> {
    let registry = format_catalog().read().map_err(|_| FormatRegistryError::Unavailable(IoRegistryUnavailable { registry: "format-catalog" }))?;
    let mut extensions = Vec::new();
    for kind_id in kind_ids {
        let descriptor = registry.get(*kind_id).ok_or_else(|| FormatRegistryError::Unknown { input: (*kind_id).to_string() })?;
        extensions.extend(descriptor.extensions.iter().cloned());
    }
    Ok(extensions.join(","))
}

//#endregion 🔖️FormatCatalog
