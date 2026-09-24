//! 🧩️ Trusted plugin module bundles: the browser plugin module of one catalog package (host shim,
//! component module, core Wasm, descriptor and vendored imports), content-addressed inside the same
//! generation as the package's component and verified like it. The normative shape is
//! `TrustedPluginModuleBundleV1` / `TrustedPluginModuleIndexV1` in [`🔣️.json`](../🧬️schema/🔣️.json);
//! the language-agnostic law is `🧫️fixtures/🧩️plugin-module/🔣️.json`.

use super::schema::{TrustedBundlePluginModuleV1, TrustedPluginModuleBundleV1, TrustedPluginModuleFileV1, TrustedPluginModuleIndexV1, TRUSTED_PLUGIN_MODULE_INDEX_SCHEMA, TRUSTED_PLUGIN_MODULE_SCHEMA};
use super::{catalog, catalog_error, decode_digest, valid_identity, valid_package_id, AuthorityError, TRUSTED_COMPONENT_MAX_BYTES};
use directory::os_directory::hex_lower;
use semio_framework_hash::{Hasher, Sha256};
use std::path::Path;

/// 🧯️ Maximum accepted bytes of one plugin module manifest.
pub const TRUSTED_PLUGIN_MODULE_BUNDLE_MAX_BYTES: u64 = 65_536;
/// 🧯️ Maximum accepted bytes of one plugin module file: a core module never exceeds its component.
pub const TRUSTED_PLUGIN_MODULE_FILE_MAX_BYTES: u64 = TRUSTED_COMPONENT_MAX_BYTES;
/// 🧯️ Fewest files a module can have: its entry and its two descriptor forms.
pub const TRUSTED_PLUGIN_MODULE_MIN_FILES: usize = 3;
/// 🧯️ Most files one plugin module manifest may list.
pub const TRUSTED_PLUGIN_MODULE_MAX_FILES: usize = 256;
/// 🧯️ Most modules one generation index may list.
pub const TRUSTED_PLUGIN_MODULE_INDEX_MAX_MODULES: usize = 4096;
/// 🧯️ Most app dialect artifact kinds one index entry may name.
pub const TRUSTED_PLUGIN_MODULE_INDEX_MAX_DIALECT_KINDS: usize = 1024;
/// 🧯️ Most segments of one plugin-module-relative path.
pub const TRUSTED_PLUGIN_MODULE_PATH_MAX_SEGMENTS: usize = 16;
/// 🧯️ Most characters of one plugin-module-relative path.
pub const TRUSTED_PLUGIN_MODULE_PATH_MAX_CHARS: usize = 1024;
/// 🧯️ Most characters of the module directory segment.
pub const TRUSTED_PLUGIN_MODULE_DIRECTORY_MAX_CHARS: usize = 128;
/// 🌉️ The module entry every plugin module directory carries.
pub const TRUSTED_PLUGIN_MODULE_BRIDGE_FILE: &str = "🌉️bridge.js";
/// 🔣️ The JSON descriptor the shell reads beside the entry before any actor starts.
pub const TRUSTED_PLUGIN_MODULE_DESCRIPTOR_JSON_FILE: &str = "🔣️.json";
/// 🛂️ The packed descriptor, byte-identical to the package's trusted descriptor.
pub const TRUSTED_PLUGIN_MODULE_DESCRIPTOR_PACK_FILE: &str = "🛂️.descriptor.semio";
/// 🪞️ The one shared root a module's vendored imports resolve under (`../🪞️vendor/…`).
pub const TRUSTED_PLUGIN_MODULE_VENDOR_DIRECTORY: &str = "🪞️vendor";
/// 🗃️ The generation directory every plugin module file is stored in, named by its SHA-256.
pub const TRUSTED_PLUGIN_MODULE_BLOB_DIRECTORY: &str = "plugin-modules";

/// 🪪️ The package a plugin module must have been derived from.
#[derive(Clone, Copy, Debug)]
pub struct TrustedPluginModuleSourceV1<'a> {
    pub plugin_id: &'a str,
    pub package_id: &'a str,
    pub version: &'a str,
    pub component_sha256: &'a str,
    pub descriptor_byte_sha256: &'a str,
}

fn valid_segment(segment: &str) -> bool {
    !segment.is_empty() && segment != "." && segment != ".." && !segment.chars().any(|character| character <= '\u{1f}' || character == '\u{7f}' || matches!(character, '/' | '\\' | '?' | '#' | '%'))
}

/// 📛️ `TrustedPluginModulePathV1`: 1–16 valid segments and at most 1024 characters.
pub fn valid_plugin_module_path(path: &str) -> bool {
    let characters = path.chars().count();
    (1..=TRUSTED_PLUGIN_MODULE_PATH_MAX_CHARS).contains(&characters) && path.split('/').count() <= TRUSTED_PLUGIN_MODULE_PATH_MAX_SEGMENTS && path.split('/').all(valid_segment)
}

fn valid_digest(value: &str) -> bool {
    decode_digest(value, "plugin module digest").is_ok()
}

/// 🗃️ The generation-relative path a plugin module file with this SHA-256 is stored at.
pub fn plugin_module_blob_path(sha256: &str) -> String {
    format!("{TRUSTED_PLUGIN_MODULE_BLOB_DIRECTORY}/{sha256}")
}

/// 🏷️ The media type a plugin module file is served with, from its extension alone.
pub fn plugin_module_content_type(path: &str) -> &'static str {
    match path.rsplit_once('.').map(|(_, extension)| extension) {
        Some("js" | "mjs") => "text/javascript; charset=utf-8",
        Some("wasm") => "application/wasm",
        Some("json") => "application/json",
        _ => "application/octet-stream",
    }
}

/// 🧩️ Checks every rule of one manifest and binds it to the package it claims to derive from.
pub fn validate_plugin_module_bundle(bundle: &TrustedPluginModuleBundleV1, source: TrustedPluginModuleSourceV1<'_>) -> Result<(), AuthorityError> {
    let refused = |reason: &str| Err(catalog(&format!("trusted plugin module {reason}")));
    if bundle.schema != TRUSTED_PLUGIN_MODULE_SCHEMA || !valid_identity(&bundle.plugin_id) || !valid_package_id(&bundle.package_id) || !valid_identity(&bundle.version) || !valid_digest(&bundle.source_component_sha256) || !valid_digest(&bundle.source_descriptor_byte_sha256) {
        return refused("identity or schema is invalid");
    }
    if bundle.module_directory.chars().count() > TRUSTED_PLUGIN_MODULE_DIRECTORY_MAX_CHARS || !valid_segment(&bundle.module_directory) || !valid_plugin_module_path(&bundle.entry) {
        return refused("directory or entry is not a canonical path");
    }
    if !(TRUSTED_PLUGIN_MODULE_MIN_FILES..=TRUSTED_PLUGIN_MODULE_MAX_FILES).contains(&bundle.files.len()) {
        return refused("file count is out of bounds");
    }
    let directory = format!("{}/", bundle.module_directory);
    let vendor = format!("{TRUSTED_PLUGIN_MODULE_VENDOR_DIRECTORY}/");
    for file in &bundle.files {
        if !valid_plugin_module_path(&file.path) || !(1..=TRUSTED_PLUGIN_MODULE_FILE_MAX_BYTES).contains(&file.byte_length) || !valid_digest(&file.sha256) || !valid_digest(&file.blake3) {
            return refused("file record is invalid");
        }
        if !file.path.starts_with(&directory) && !file.path.starts_with(&vendor) {
            return refused("file lies outside its module directory and the vendor root");
        }
    }
    if bundle.files.windows(2).any(|pair| pair[0].path.as_bytes() >= pair[1].path.as_bytes()) {
        return refused("files are not in strictly ascending byte order");
    }
    let file = |name: &str| bundle.files.iter().find(|file| file.path == format!("{directory}{name}"));
    if bundle.entry != format!("{directory}{TRUSTED_PLUGIN_MODULE_BRIDGE_FILE}") || file(TRUSTED_PLUGIN_MODULE_BRIDGE_FILE).is_none() || file(TRUSTED_PLUGIN_MODULE_DESCRIPTOR_JSON_FILE).is_none() {
        return refused("entry or JSON descriptor is missing");
    }
    if file(TRUSTED_PLUGIN_MODULE_DESCRIPTOR_PACK_FILE).is_none_or(|pack| pack.sha256 != bundle.source_descriptor_byte_sha256) {
        return refused("packed descriptor is not the package's own descriptor");
    }
    if bundle.plugin_id != source.plugin_id || bundle.package_id != source.package_id || bundle.version != source.version || bundle.source_component_sha256 != source.component_sha256 || bundle.source_descriptor_byte_sha256 != source.descriptor_byte_sha256 {
        return refused("was not derived from its package");
    }
    Ok(())
}

/// 📖️ Decodes one manifest, requiring the exact canonical bytes its own encoding produces.
pub fn decode_plugin_module_bundle(bytes: &[u8], source: TrustedPluginModuleSourceV1<'_>) -> Result<TrustedPluginModuleBundleV1, AuthorityError> {
    if bytes.is_empty() || bytes.len() as u64 > TRUSTED_PLUGIN_MODULE_BUNDLE_MAX_BYTES {
        return Err(catalog("trusted plugin module manifest exceeds its bound"));
    }
    let bundle: TrustedPluginModuleBundleV1 = serde_json::from_slice(bytes).map_err(catalog_error)?;
    if encode_plugin_module_bundle(&bundle)? != bytes {
        return Err(catalog("trusted plugin module manifest is not canonical"));
    }
    validate_plugin_module_bundle(&bundle, source)?;
    Ok(bundle)
}

/// 📤️ The canonical manifest bytes: compact JSON in schema field order and one trailing newline.
pub fn encode_plugin_module_bundle(bundle: &TrustedPluginModuleBundleV1) -> Result<Vec<u8>, AuthorityError> {
    let mut bytes = serde_json::to_vec(bundle).map_err(catalog_error)?;
    bytes.push(b'\n');
    Ok(bytes)
}

/// 🔐️ Accepts a file's bytes only when their length and both digests are the manifest's own.
pub fn verify_plugin_module_file(file: &TrustedPluginModuleFileV1, byte_length: usize, sha256: [u8; 32], blake3: [u8; 32]) -> Result<(), AuthorityError> {
    if u64::try_from(byte_length).map_err(catalog_error)? != file.byte_length || decode_digest(&file.sha256, "plugin module file sha256")? != sha256 || decode_digest(&file.blake3, "plugin module file blake3")? != blake3 {
        return Err(catalog("trusted plugin module file differs from its manifest"));
    }
    Ok(())
}

/// 📇️ Checks every rule of one generation index: canonical entries in strictly ascending plugin order.
pub fn validate_plugin_module_index(index: &TrustedPluginModuleIndexV1) -> Result<(), AuthorityError> {
    if index.schema != TRUSTED_PLUGIN_MODULE_INDEX_SCHEMA || !valid_digest(&index.generation_id) || index.modules.len() > TRUSTED_PLUGIN_MODULE_INDEX_MAX_MODULES {
        return Err(catalog("trusted plugin module index shape is invalid"));
    }
    for entry in &index.modules {
        let mut dependencies = std::collections::BTreeSet::new();
        if !valid_identity(&entry.plugin_id)
            || !valid_package_id(&entry.package_id)
            || !valid_identity(&entry.version)
            || !valid_digest(&entry.component_sha256)
            || !valid_digest(&entry.descriptor_byte_sha256)
            || !valid_digest(&entry.bundle_sha256)
            || !(1..=TRUSTED_PLUGIN_MODULE_BUNDLE_MAX_BYTES).contains(&entry.bundle_byte_length)
            || !valid_plugin_module_path(&entry.entry)
            || entry.dependencies.len() > super::TRUSTED_PACKAGE_MAX_DEPENDENCIES
            || !entry.dependencies.iter().all(|dependency| valid_identity(dependency) && dependencies.insert(dependency.as_str()))
            || entry.dialect_artifact_kinds.len() > TRUSTED_PLUGIN_MODULE_INDEX_MAX_DIALECT_KINDS
            || !entry.dialect_artifact_kinds.iter().all(|kind| valid_identity(kind))
            || entry.dialect_artifact_kinds.windows(2).any(|pair| pair[0].as_bytes() >= pair[1].as_bytes())
            || entry.extends_plugin_id.as_ref().is_some_and(|parent| !dependencies.contains(parent.as_str()))
        {
            return Err(catalog("trusted plugin module index entry is invalid"));
        }
    }
    if index.modules.windows(2).any(|pair| pair[0].plugin_id.as_bytes() >= pair[1].plugin_id.as_bytes()) {
        return Err(catalog("trusted plugin module index is not in strictly ascending plugin order"));
    }
    Ok(())
}

fn digests(bytes: &[u8]) -> ([u8; 32], [u8; 32]) {
    let mut blake3 = Hasher::new();
    blake3.update(bytes);
    (Sha256::digest(bytes), *blake3.finalize().as_bytes())
}

/// 🧩️ Assembles one package's manifest from its files: each hashed, the list in ascending byte order,
/// and the whole held to every rule a loader applies before anything is written.
pub fn assemble_plugin_module_bundle(source: TrustedPluginModuleSourceV1<'_>, module_directory: &str, files: &[(String, Vec<u8>)]) -> Result<TrustedPluginModuleBundleV1, AuthorityError> {
    let mut listed = files
        .iter()
        .map(|(path, bytes)| {
            let (sha256, blake3) = digests(bytes);
            Ok(TrustedPluginModuleFileV1 { path: path.clone(), byte_length: u64::try_from(bytes.len()).map_err(catalog_error)?, sha256: hex_lower(&sha256), blake3: hex_lower(&blake3) })
        })
        .collect::<Result<Vec<_>, AuthorityError>>()?;
    listed.sort_by(|left, right| left.path.as_bytes().cmp(right.path.as_bytes()));
    let bundle = TrustedPluginModuleBundleV1 {
        schema: TRUSTED_PLUGIN_MODULE_SCHEMA.into(),
        plugin_id: source.plugin_id.into(),
        package_id: source.package_id.into(),
        version: source.version.into(),
        source_component_sha256: source.component_sha256.into(),
        source_descriptor_byte_sha256: source.descriptor_byte_sha256.into(),
        module_directory: module_directory.into(),
        entry: format!("{module_directory}/{TRUSTED_PLUGIN_MODULE_BRIDGE_FILE}"),
        files: listed,
    };
    validate_plugin_module_bundle(&bundle, source)?;
    Ok(bundle)
}

/// 💾️ Writes one assembled plugin module beneath a generation `root`: every file content-addressed
/// under [`TRUSTED_PLUGIN_MODULE_BLOB_DIRECTORY`] and the canonical manifest at `manifest_path`, and
/// answers the package record that names the manifest.
pub fn write_plugin_module_bundle(root: &Path, manifest_path: &str, source: TrustedPluginModuleSourceV1<'_>, module_directory: &str, files: &[(String, Vec<u8>)]) -> Result<TrustedBundlePluginModuleV1, AuthorityError> {
    let bundle = assemble_plugin_module_bundle(source, module_directory, files)?;
    std::fs::create_dir_all(root.join(TRUSTED_PLUGIN_MODULE_BLOB_DIRECTORY)).map_err(catalog_error)?;
    for (path, bytes) in files {
        let file = bundle.files.iter().find(|file| &file.path == path).ok_or_else(|| catalog("assembled plugin module lost a file"))?;
        std::fs::write(root.join(plugin_module_blob_path(&file.sha256)), bytes).map_err(catalog_error)?;
    }
    let manifest = encode_plugin_module_bundle(&bundle)?;
    let destination = root.join(manifest_path);
    if let Some(parent) = destination.parent() {
        std::fs::create_dir_all(parent).map_err(catalog_error)?;
    }
    std::fs::write(destination, &manifest).map_err(catalog_error)?;
    let (sha256, blake3) = digests(&manifest);
    Ok(TrustedBundlePluginModuleV1 { path: manifest_path.into(), byte_length: u64::try_from(manifest.len()).map_err(catalog_error)?, sha256: hex_lower(&sha256), blake3: hex_lower(&blake3) })
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
