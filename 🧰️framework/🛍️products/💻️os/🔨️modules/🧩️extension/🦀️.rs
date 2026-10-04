//! 🧩️ Runtime-installable `.sxt` extension package format — semio binary envelope over a
//! deterministic deflate zip (`🛂️manifest.semio` + `component.wasm` + optional `assets/`).

use std::collections::BTreeMap;

use semio_framework_deflate::zip_archive::{ZipArchive, ZipArchiveError, ZipWriter};

use crate::os_semio::{unwrap_binary, wrap_binary, Component, SemioEnvelope, SemioError};

//#region 🔖️Errors
/// ⚠️ Failures packing, unpacking, or verifying an `.sxt` extension package.
#[derive(Debug)]
pub enum ExtensionPackageError {
    Envelope(SemioError),
    UnexpectedEnvelope(String),
    Zip(ZipArchiveError),
    ManifestJson(String),
    MissingEntry(String),
    InvalidPackageFormat(u16),
    EmptyComponent,
}

impl std::fmt::Display for ExtensionPackageError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Envelope(error) => write!(formatter, "semio envelope error: {error}"),
            Self::UnexpectedEnvelope(envelope) => write!(formatter, "unexpected extension package envelope: {envelope}"),
            Self::Zip(error) => write!(formatter, "zip error: {error}"),
            Self::ManifestJson(error) => write!(formatter, "manifest json error: {error}"),
            Self::MissingEntry(entry) => write!(formatter, "missing zip entry: {entry}"),
            Self::InvalidPackageFormat(version) => write!(formatter, "invalid package format version: {version}"),
            Self::EmptyComponent => formatter.write_str("empty component.wasm"),
        }
    }
}

impl std::error::Error for ExtensionPackageError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Envelope(error) => Some(error),
            Self::Zip(error) => Some(error),
            _ => None,
        }
    }
}

impl From<SemioError> for ExtensionPackageError {
    fn from(error: SemioError) -> Self {
        Self::Envelope(error)
    }
}

impl From<ZipArchiveError> for ExtensionPackageError {
    fn from(error: ZipArchiveError) -> Self {
        match error {
            ZipArchiveError::MissingEntry(name) => Self::MissingEntry(name),
            other => Self::Zip(other),
        }
    }
}

//#endregion 🔖️Errors

//#region 🔖️Constants
/// 🛂️ Zip path of the package manifest (JSON `ExtensionPackageManifest`).
pub const MANIFEST_ENTRY: &str = "🛂️manifest.semio";

/// 🧬️ Zip path of the raw wasip2 component bytes.
pub const COMPONENT_ENTRY: &str = "component.wasm";

/// 🗂️ Zip directory prefix for optional package assets.
pub const ASSETS_PREFIX: &str = "assets/";

/// 🏷️ Semio envelope plugin segment for `.sxt` packages.
pub const EXTENSION_PACKAGE_PLUGIN: &str = "os";

/// 🏷️ Semio envelope artifact segment for `.sxt` packages.
pub const EXTENSION_PACKAGE_ARTIFACT: &str = "extension";

/// 🔢 Current `.sxt` package format version (envelope + manifest `packageFormat`).
pub const EXTENSION_PACKAGE_FORMAT: u16 = 1;
//#endregion 🔖️Constants

//#region 🔖️Manifest
/// 🔗️ Package-manifest-local mirror of `semio_framework::PluginDependency` — this crate
/// (`semio-framework-os-kernel`) must never depend on `semio-framework` (contract freeze §0
/// dependency edge law: `semio-framework` depends on `semio-framework-os-kernel`, never the
/// reverse), so the `.sxt` wire shape is duplicated here byte-identically instead of imported.
/// `version` is the exact pin's display string `=X.Y.Z` — the only form `semio_framework::VersionPin::parse`
/// accepts at any call site that does depend on that crate (e.g. the guest `ExtensionManifest`); `from_json`
/// refuses every range (`*`, `^`, `~`, `>=`, a bare triple) the same way.
#[derive(Clone, Debug, PartialEq)]
pub struct PackagePluginDependency {
    pub plugin_id: String,
    pub version: String,
}

impl PackagePluginDependency {
    fn to_json(&self) -> semio_framework_pack_json::Value {
        use semio_framework_pack_json::{object, Value};
        object([("pluginId".to_string(), Value::from(self.plugin_id.clone())), ("version".to_string(), Value::from(self.version.clone()))])
    }

    fn from_json(value: &semio_framework_pack_json::Value) -> Result<Self, String> {
        use semio_framework_pack_json::Value;
        Ok(Self {
            plugin_id: value.get("pluginId").and_then(Value::as_str).map(str::to_owned).ok_or_else(|| "missing field pluginId".to_string())?,
            version: value.get("version").and_then(Value::as_str).filter(|version| is_exact_pin(version)).map(str::to_owned).ok_or_else(|| "field version must be an exact pin `=X.Y.Z`".to_string())?,
        })
    }
}

/// 📌️ Whether `raw` is an exact dependency pin `=X.Y.Z` (three all-numeric segments), the `.sxt` twin of
/// `semio_framework::VersionPin::parse`.
fn is_exact_pin(raw: &str) -> bool {
    raw.trim().strip_prefix('=').is_some_and(|version| {
        let segments: Vec<&str> = version.split('.').collect();
        segments.len() == 3 && segments.iter().all(|segment| !segment.is_empty() && segment.bytes().all(|byte| byte.is_ascii_digit()))
    })
}

/// 📦️ On-disk package manifest carried as `🛂️manifest.semio` inside the zip payload.
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionPackageManifest {
    pub extension_id: String,
    pub directory_name: String,
    pub label: String,
    pub version: String,
    pub extends: String,
    pub capabilities: Vec<String>,
    /// 🗂️ Open plugin contributions (mirrors the guest `ExtensionManifest.topic_contributions`) —
    /// renamed from the former bare `contributions` field to free that name for the typed
    /// artifact-kind contribution roster below (contract freeze §3/§4).
    pub topic_contributions: semio_framework_pack_json::Value,
    /// 🔗️ Direct plugin dependencies this extension requires — see `PackagePluginDependency`.
    pub dependencies: Vec<PackagePluginDependency>,
    /// 🗂️ Artifact-kind contributions (mutations/inferences) this extension contributes onto
    /// artifact kinds it depends on — a raw JSON array of
    /// `semio_framework::ArtifactContributionDescriptor`, kept untyped here for the same
    /// dependency-edge-law reason as `PackagePluginDependency` above.
    pub contributions: semio_framework_pack_json::Value,
    pub package_format: u16,
}

impl ExtensionPackageManifest {
    /// ✅️ Contract freeze §4 registration gate: `extends` must equal the first declared
    /// dependency's plugin id (vacuously true when both are empty — an extension that declares no
    /// host and no dependencies yet).
    pub async fn extends_matches_primary_dependency(&self) -> bool {
        match self.dependencies.first() {
            Some(dependency) => dependency.plugin_id == self.extends,
            None => self.extends.is_empty(),
        }
    }

    fn to_json(&self) -> semio_framework_pack_json::Value {
        use semio_framework_pack_json::{object, Value};
        object([
            ("extensionId".to_string(), Value::from(self.extension_id.clone())),
            ("directoryName".to_string(), Value::from(self.directory_name.clone())),
            ("label".to_string(), Value::from(self.label.clone())),
            ("version".to_string(), Value::from(self.version.clone())),
            ("extends".to_string(), Value::from(self.extends.clone())),
            ("capabilities".to_string(), Value::Array(self.capabilities.iter().map(|c| Value::from(c.clone())).collect())),
            ("topicContributions".to_string(), self.topic_contributions.clone()),
            ("dependencies".to_string(), Value::Array(self.dependencies.iter().map(PackagePluginDependency::to_json).collect())),
            ("contributions".to_string(), self.contributions.clone()),
            ("packageFormat".to_string(), Value::from(self.package_format as u64)),
        ])
    }

    fn from_json(value: &semio_framework_pack_json::Value) -> Result<Self, String> {
        use semio_framework_pack_json::Value;
        let field_str = |key: &str| value.get(key).and_then(Value::as_str).map(str::to_owned).ok_or_else(|| format!("missing field {key}"));
        let capabilities = match value.get("capabilities").and_then(Value::as_array) {
            Some(entries) => entries.iter().map(|entry| entry.as_str().map(str::to_owned).ok_or_else(|| "capabilities entries must be strings".to_string())).collect::<Result<Vec<_>, _>>()?,
            None => Vec::new(),
        };
        let dependencies = match value.get("dependencies").and_then(Value::as_array) {
            Some(entries) => entries.iter().map(PackagePluginDependency::from_json).collect::<Result<Vec<_>, _>>()?,
            None => Vec::new(),
        };
        let package_format = value.get("packageFormat").and_then(Value::as_u64).ok_or_else(|| "missing field packageFormat".to_string())? as u16;
        Ok(Self {
            extension_id: field_str("extensionId")?,
            directory_name: field_str("directoryName")?,
            label: field_str("label")?,
            version: field_str("version")?,
            extends: field_str("extends")?,
            capabilities,
            topic_contributions: value.get("topicContributions").cloned().unwrap_or(Value::Null),
            dependencies,
            contributions: value.get("contributions").cloned().unwrap_or(Value::Null),
            package_format,
        })
    }
}
//#endregion 🔖️Manifest

//#region 🔖️Package
/// 🧩️ Unpacked `.sxt` contents: manifest, component bytes, and optional named assets.
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionPackage {
    pub manifest: ExtensionPackageManifest,
    pub component_wasm: Vec<u8>,
    pub assets: BTreeMap<String, Vec<u8>>,
}

/// 📨 Canonical semio binary envelope for an `.sxt` package.
pub async fn extension_package_envelope() -> SemioEnvelope {
    SemioEnvelope { plugin: EXTENSION_PACKAGE_PLUGIN.into(), artifact: EXTENSION_PACKAGE_ARTIFACT.into(), component: Component::Pack, version: EXTENSION_PACKAGE_FORMAT }
}
//#endregion 🔖️Package

//#region 🔖️Zip
/// 📏️ The largest single entry an `.sxt` package may inflate to (a wasip2 component with its assets stays far below).
const MAX_PACKAGE_ENTRY_BYTES: usize = 256 * 1024 * 1024;

async fn build_zip_payload(manifest: &ExtensionPackageManifest, component_wasm: &[u8], assets: &[(String, Vec<u8>)]) -> Result<Vec<u8>, ExtensionPackageError> {
    if component_wasm.is_empty() {
        return Err(ExtensionPackageError::EmptyComponent);
    }
    if manifest.package_format != EXTENSION_PACKAGE_FORMAT {
        return Err(ExtensionPackageError::InvalidPackageFormat(manifest.package_format));
    }
    let mut writer = ZipWriter::new();
    writer.add(MANIFEST_ENTRY, semio_framework_pack_json::to_string(&manifest.to_json()).as_bytes())?;
    writer.add(COMPONENT_ENTRY, component_wasm)?;
    let mut sorted_assets: Vec<&(String, Vec<u8>)> = assets.iter().collect();
    sorted_assets.sort_by(|a, b| a.0.cmp(&b.0));
    for (name, bytes) in sorted_assets {
        let entry = if name.starts_with(ASSETS_PREFIX) { name.clone() } else { format!("{ASSETS_PREFIX}{name}") };
        writer.add(&entry, bytes)?;
    }
    Ok(writer.finish()?)
}

async fn parse_zip_payload(payload: &[u8]) -> Result<ExtensionPackage, ExtensionPackageError> {
    let archive = ZipArchive::parse(payload)?;
    let manifest_bytes = archive.read(MANIFEST_ENTRY, MAX_PACKAGE_ENTRY_BYTES)?;
    let manifest_json = semio_framework_pack_json::parse_bytes(&manifest_bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| ExtensionPackageError::ManifestJson(error.to_string()))?;
    let manifest = ExtensionPackageManifest::from_json(&manifest_json).map_err(ExtensionPackageError::ManifestJson)?;
    if manifest.package_format != EXTENSION_PACKAGE_FORMAT {
        return Err(ExtensionPackageError::InvalidPackageFormat(manifest.package_format));
    }
    let component_wasm = archive.read(COMPONENT_ENTRY, MAX_PACKAGE_ENTRY_BYTES)?;
    if component_wasm.is_empty() {
        return Err(ExtensionPackageError::EmptyComponent);
    }
    let mut assets = BTreeMap::new();
    for entry in archive.entries() {
        if let Some(relative) = entry.name.strip_prefix(ASSETS_PREFIX).filter(|relative| !relative.is_empty() && !entry.name.ends_with('/')) {
            assets.insert(relative.to_string(), archive.read_entry(entry, MAX_PACKAGE_ENTRY_BYTES)?);
        }
    }
    Ok(ExtensionPackage { manifest, component_wasm, assets })
}

async fn expect_extension_envelope(envelope: &SemioEnvelope) -> Result<(), ExtensionPackageError> {
    let expected = extension_package_envelope().await;
    if envelope != &expected {
        return Err(ExtensionPackageError::UnexpectedEnvelope(envelope.binary_token()));
    }
    Ok(())
}
//#endregion 🔖️Zip

//#region 🔖️Api
/// 📦️ Packs an extension into a `.sxt` byte stream (semio binary envelope + deterministic zip).
pub async fn pack(manifest: &ExtensionPackageManifest, component_wasm: &[u8], assets: &[(String, Vec<u8>)]) -> Result<Vec<u8>, ExtensionPackageError> {
    let payload = build_zip_payload(manifest, component_wasm, assets).await?;
    Ok(wrap_binary(&extension_package_envelope().await, &payload))
}

/// 📥️ Unpacks a `.sxt` byte stream into manifest, component, and assets.
pub async fn unpack(bytes: &[u8]) -> Result<ExtensionPackage, ExtensionPackageError> {
    let (envelope, payload) = unwrap_binary(bytes)?;
    expect_extension_envelope(&envelope).await?;
    parse_zip_payload(&payload).await
}

/// ✅ Verifies a `.sxt` byte stream and returns its package manifest.
pub async fn verify(bytes: &[u8]) -> Result<ExtensionPackageManifest, ExtensionPackageError> {
    Ok(unpack(bytes).await?.manifest)
}

/// 🔓️ Blake3 content hash of the full `.sxt` bytes (same primitive as `BlobStore::put` dedup).
pub fn content_hash(bytes: &[u8]) -> String {
    semio_framework_hash::hash_bytes(bytes)
}
//#endregion 🔖️Api

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
