//! 🧩️ Runtime-installable `.sxt` extension package format — semio binary envelope over a
//! deterministic deflate zip (`🛂️manifest.semio` + `component.wasm` + optional `assets/`).

use std::collections::BTreeMap;
use std::io::{Cursor, Read, Seek, Write};

use crate::os_semio::{unwrap_binary, wrap_binary, Component, SemioEnvelope, SemioError};

//#region 🔖️Errors
/// ⚠️ Failures packing, unpacking, or verifying an `.sxt` extension package.
#[derive(Debug)]
pub enum ExtensionPackageError {
    Envelope(SemioError),
    UnexpectedEnvelope(String),
    Zip(zip::result::ZipError),
    Io(std::io::Error),
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
            Self::Io(error) => write!(formatter, "io error: {error}"),
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
            Self::Io(error) => Some(error),
            _ => None,
        }
    }
}

impl From<SemioError> for ExtensionPackageError {
    fn from(error: SemioError) -> Self {
        Self::Envelope(error)
    }
}

impl From<zip::result::ZipError> for ExtensionPackageError {
    fn from(error: zip::result::ZipError) -> Self {
        Self::Zip(error)
    }
}

impl From<std::io::Error> for ExtensionPackageError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
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
/// `version` is the plain `VersionReq` display string (`=X.Y.Z`/`^X.Y.Z`/`~X.Y.Z`/`>=X.Y.Z`/`*`,
/// contract freeze §3) — round-trips losslessly through `semio_framework::VersionReq::parse` at
/// any call site that does depend on that crate (e.g. the guest `ExtensionManifest`).
#[derive(Clone, Debug, PartialEq)]
pub struct PackagePluginDependency {
    pub plugin_id: String,
    pub version: String,
}

impl PackagePluginDependency {
    fn to_json(&self) -> crate::os_pack::json::Value {
        use crate::os_pack::json::{object, Value};
        object([("pluginId".to_string(), Value::from(self.plugin_id.clone())), ("version".to_string(), Value::from(self.version.clone()))])
    }

    fn from_json(value: &crate::os_pack::json::Value) -> Result<Self, String> {
        use crate::os_pack::json::Value;
        Ok(Self {
            plugin_id: value.get("pluginId").and_then(Value::as_str).map(str::to_owned).ok_or_else(|| "missing field pluginId".to_string())?,
            version: value.get("version").and_then(Value::as_str).map(str::to_owned).ok_or_else(|| "missing field version".to_string())?,
        })
    }
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
    pub topic_contributions: crate::os_pack::json::Value,
    /// 🔗️ Direct plugin dependencies this extension requires — see `PackagePluginDependency`.
    pub dependencies: Vec<PackagePluginDependency>,
    /// 🗂️ Artifact-kind contributions (mutations/inferences) this extension contributes onto
    /// artifact kinds it depends on — a raw JSON array of
    /// `semio_framework::ArtifactContributionDescriptor`, kept untyped here for the same
    /// dependency-edge-law reason as `PackagePluginDependency` above.
    pub contributions: crate::os_pack::json::Value,
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

    fn to_json(&self) -> crate::os_pack::json::Value {
        use crate::os_pack::json::{object, Value};
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

    fn from_json(value: &crate::os_pack::json::Value) -> Result<Self, String> {
        use crate::os_pack::json::Value;
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
async fn zip_file_options() -> zip::write::SimpleFileOptions {
    zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated).last_modified_time(zip::DateTime::default())
}

async fn write_zip_file<W: Write + Seek>(writer: &mut zip::ZipWriter<W>, name: &str, bytes: &[u8], options: zip::write::SimpleFileOptions) -> Result<(), ExtensionPackageError> {
    writer.start_file(name, options)?;
    writer.write_all(bytes)?;
    Ok(())
}

async fn read_zip_entry<R: Read + Seek>(archive: &mut zip::ZipArchive<R>, name: &str) -> Result<Vec<u8>, ExtensionPackageError> {
    let mut file = archive.by_name(name).map_err(|_| ExtensionPackageError::MissingEntry(name.into()))?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    Ok(bytes)
}

async fn build_zip_payload(manifest: &ExtensionPackageManifest, component_wasm: &[u8], assets: &[(String, Vec<u8>)]) -> Result<Vec<u8>, ExtensionPackageError> {
    if component_wasm.is_empty() {
        return Err(ExtensionPackageError::EmptyComponent);
    }
    if manifest.package_format != EXTENSION_PACKAGE_FORMAT {
        return Err(ExtensionPackageError::InvalidPackageFormat(manifest.package_format));
    }

    let mut writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
    // 🪡️ `SimpleFileOptions` is `Copy` (zip 2.x `write.rs`); `options` is awaited exactly ONCE here
    // and reused by value below — the original awaited the same future 3 times, E0382 (R10 residue #2).
    let options = zip_file_options().await;
    let manifest_bytes = crate::os_pack::json::to_string(&manifest.to_json()).into_bytes();
    write_zip_file(&mut writer, MANIFEST_ENTRY, &manifest_bytes, options).await?;
    write_zip_file(&mut writer, COMPONENT_ENTRY, component_wasm, options).await?;

    let mut sorted_assets: Vec<&(String, Vec<u8>)> = assets.iter().collect();
    sorted_assets.sort_by(|a, b| a.0.cmp(&b.0));
    for (name, bytes) in sorted_assets {
        let entry = if name.starts_with(ASSETS_PREFIX) { name.clone() } else { format!("{ASSETS_PREFIX}{name}") };
        write_zip_file(&mut writer, &entry, bytes, options).await?;
    }

    Ok(writer.finish()?.into_inner())
}

async fn parse_zip_payload(payload: &[u8]) -> Result<ExtensionPackage, ExtensionPackageError> {
    let mut archive = zip::ZipArchive::new(Cursor::new(payload))?;
    let manifest_bytes = read_zip_entry(&mut archive, MANIFEST_ENTRY).await?;
    let manifest_json = crate::os_pack::json::parse_bytes(&manifest_bytes).map_err(|error| ExtensionPackageError::ManifestJson(error.to_string()))?;
    let manifest = ExtensionPackageManifest::from_json(&manifest_json).map_err(ExtensionPackageError::ManifestJson)?;
    if manifest.package_format != EXTENSION_PACKAGE_FORMAT {
        return Err(ExtensionPackageError::InvalidPackageFormat(manifest.package_format));
    }
    let component_wasm = read_zip_entry(&mut archive, COMPONENT_ENTRY).await?;
    if component_wasm.is_empty() {
        return Err(ExtensionPackageError::EmptyComponent);
    }

    let mut assets = BTreeMap::new();
    for index in 0..archive.len() {
        let mut file = archive.by_index(index)?;
        let name = file.name().to_string();
        if name == MANIFEST_ENTRY || name == COMPONENT_ENTRY || name.ends_with('/') {
            continue;
        }
        if let Some(relative) = name.strip_prefix(ASSETS_PREFIX) {
            if relative.is_empty() {
                continue;
            }
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes)?;
            assets.insert(relative.to_string(), bytes);
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
