//! 🧩️ Runtime-installable `.sxt` extension package format — semio binary envelope over a
//! deterministic deflate zip (`🛂️manifest.semio` + `component.wasm` + optional `assets/`).

use std::collections::BTreeMap;
use std::io::{Cursor, Read, Seek, Write};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::os_semio::{unwrap_binary, wrap_binary, Component, SemioEnvelope, SemioError};

//#region 🔖️Errors
/// ⚠️ Failures packing, unpacking, or verifying an `.sxt` extension package.
#[derive(Debug, Error)]
pub enum ExtensionPackageError {
    #[error("semio envelope error: {0}")]
    Envelope(#[from] SemioError),
    #[error("unexpected extension package envelope: {0}")]
    UnexpectedEnvelope(String),
    #[error("zip error: {0}")]
    Zip(#[from] zip::result::ZipError),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("manifest json error: {0}")]
    ManifestJson(#[from] serde_json::Error),
    #[error("missing zip entry: {0}")]
    MissingEntry(String),
    #[error("invalid package format version: {0}")]
    InvalidPackageFormat(u16),
    #[error("empty component.wasm")]
    EmptyComponent,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackagePluginDependency {
    pub plugin_id: String,
    pub version: String,
}

/// 📦️ On-disk package manifest carried as `🛂️manifest.semio` inside the zip payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionPackageManifest {
    pub extension_id: String,
    pub label: String,
    pub version: String,
    pub extends: String,
    pub capabilities: Vec<String>,
    /// 🗂️ Open plugin contributions (mirrors the guest `ExtensionManifest.topic_contributions`) —
    /// renamed from the former bare `contributions` field to free that name for the typed
    /// artifact-kind contribution roster below (contract freeze §3/§4).
    pub topic_contributions: serde_json::Value,
    /// 🔗️ Direct plugin dependencies this extension requires — see `PackagePluginDependency`.
    #[serde(default)]
    pub dependencies: Vec<PackagePluginDependency>,
    /// 🗂️ Artifact-kind contributions (mutations/inferences) this extension contributes onto
    /// artifact kinds it depends on — a raw JSON array of
    /// `semio_framework::ArtifactContributionDescriptor`, kept untyped here for the same
    /// dependency-edge-law reason as `PackagePluginDependency` above.
    #[serde(default)]
    pub contributions: serde_json::Value,
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
    SemioEnvelope {
        plugin: EXTENSION_PACKAGE_PLUGIN.into(),
        artifact: EXTENSION_PACKAGE_ARTIFACT.into(),
        component: Component::Pack,
        version: EXTENSION_PACKAGE_FORMAT,
    }
}
//#endregion 🔖️Package

//#region 🔖️Zip
async fn zip_file_options() -> zip::write::SimpleFileOptions {
    zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .last_modified_time(zip::DateTime::default())
}

async fn write_zip_file<W: Write + Seek>(
    writer: &mut zip::ZipWriter<W>,
    name: &str,
    bytes: &[u8],
    options: zip::write::SimpleFileOptions,
) -> Result<(), ExtensionPackageError> {
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

async fn build_zip_payload(
    manifest: &ExtensionPackageManifest,
    component_wasm: &[u8],
    assets: &[(String, Vec<u8>)],
) -> Result<Vec<u8>, ExtensionPackageError> {
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
    let manifest_bytes = serde_json::to_vec(manifest)?;
    write_zip_file(&mut writer, MANIFEST_ENTRY, &manifest_bytes, options).await?;
    write_zip_file(&mut writer, COMPONENT_ENTRY, component_wasm, options).await?;

    let mut sorted_assets: Vec<&(String, Vec<u8>)> = assets.iter().collect();
    sorted_assets.sort_by(|a, b| a.0.cmp(&b.0));
    for (name, bytes) in sorted_assets {
        let entry = if name.starts_with(ASSETS_PREFIX) {
            name.clone()
        } else {
            format!("{ASSETS_PREFIX}{name}")
        };
        write_zip_file(&mut writer, &entry, bytes, options).await?;
    }

    Ok(writer.finish()?.into_inner())
}

async fn parse_zip_payload(payload: &[u8]) -> Result<ExtensionPackage, ExtensionPackageError> {
    let mut archive = zip::ZipArchive::new(Cursor::new(payload))?;
    let manifest_bytes = read_zip_entry(&mut archive, MANIFEST_ENTRY).await?;
    let manifest: ExtensionPackageManifest = serde_json::from_slice(&manifest_bytes)?;
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

    Ok(ExtensionPackage {
        manifest,
        component_wasm,
        assets,
    })
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
pub async fn pack(
    manifest: &ExtensionPackageManifest,
    component_wasm: &[u8],
    assets: &[(String, Vec<u8>)],
) -> Result<Vec<u8>, ExtensionPackageError> {
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
pub async fn content_hash(bytes: &[u8]) -> String {
    semio_framework_hash::hash_bytes(bytes).await
}
//#endregion 🔖️Api

#[cfg(test)]
mod tests {
    use super::*;

    async fn sample_manifest() -> ExtensionPackageManifest {
        ExtensionPackageManifest {
            extension_id: "flow.math".into(),
            label: "Flow Math".into(),
            version: "0.1.0".into(),
            extends: "flow".into(),
            capabilities: vec!["flow.operator".into()],
            topic_contributions: serde_json::json!([{ "kind": "flowExtension", "id": "math.add" }]),
            dependencies: vec![PackagePluginDependency { plugin_id: "flow".into(), version: "^1.0.0".into() }],
            contributions: serde_json::json!([]),
            package_format: EXTENSION_PACKAGE_FORMAT,
        }
    }

    //#region 🔖️DependencyAndContributionTests
    #[semio_framework_async_macros::async_test]
    async fn extends_matches_primary_dependency_holds_for_the_sample_and_the_vacuous_case() {
        assert!(sample_manifest().await.extends_matches_primary_dependency());

        let vacuous = ExtensionPackageManifest { extends: String::new(), dependencies: Vec::new(), ..sample_manifest().await };
        assert!(vacuous.extends_matches_primary_dependency());
    }

    #[semio_framework_async_macros::async_test]
    async fn extends_matches_primary_dependency_rejects_mismatch_and_missing_dependency() {
        let mismatched = ExtensionPackageManifest { extends: "cad".into(), ..sample_manifest().await };
        assert!(!mismatched.extends_matches_primary_dependency());

        let no_dependencies = ExtensionPackageManifest { dependencies: Vec::new(), ..sample_manifest().await };
        assert!(!no_dependencies.extends_matches_primary_dependency(), "non-empty extends with no dependencies is inconsistent");
    }

    #[semio_framework_async_macros::async_test]
    async fn dependencies_default_absent_on_the_wire() {
        let bare = serde_json::json!({
            "extensionId": "flow.math",
            "label": "Flow Math",
            "version": "0.1.0",
            "extends": "",
            "capabilities": [],
            "topicContributions": [],
            "packageFormat": EXTENSION_PACKAGE_FORMAT,
        });
        let parsed: ExtensionPackageManifest = serde_json::from_value(bare).unwrap();
        assert!(parsed.dependencies.is_empty());
        assert_eq!(parsed.contributions, serde_json::Value::Null);
    }

    #[semio_framework_async_macros::async_test]
    async fn package_plugin_dependency_round_trips_as_a_plain_string_pair() {
        let dependency = PackagePluginDependency { plugin_id: "cad".into(), version: "^1.0.0".into() };
        let json = serde_json::to_value(&dependency).unwrap();
        assert_eq!(json, serde_json::json!({ "pluginId": "cad", "version": "^1.0.0" }));
        let round_tripped: PackagePluginDependency = serde_json::from_value(json).unwrap();
        assert_eq!(round_tripped, dependency);
    }
    //#endregion 🔖️DependencyAndContributionTests

    #[semio_framework_async_macros::async_test]
    async fn pack_unpack_verify_round_trip() {
        let manifest = sample_manifest();
        let component = b"\0asm\x01\x00\x00\x00fake-component".to_vec();
        let assets = vec![
            ("readme.txt".into(), b"hello".to_vec()),
            ("nested/icon.svg".into(), b"<svg/>".to_vec()),
        ];

        let packed = pack(&manifest, &component, &assets).await.expect("pack");
        assert!(packed.starts_with(&crate::os_semio::BINARY_MAGIC));

        let verified = verify(&packed).await.expect("verify");
        assert_eq!(verified, manifest.await);

        let unpacked = unpack(&packed).await.expect("unpack");
        assert_eq!(unpacked.manifest, manifest.await);
        assert_eq!(unpacked.component_wasm, component);
        assert_eq!(unpacked.assets.get("readme.txt").map(Vec::as_slice), Some(b"hello".as_slice()));
        assert_eq!(unpacked.assets.get("nested/icon.svg").map(Vec::as_slice), Some(b"<svg/>".as_slice()));

        let again = pack(&unpacked.manifest, &unpacked.component_wasm, &unpacked.assets.iter().map(|(k, v)| (k.clone(), v.clone())).collect::<Vec<_>>()).await.expect("repack");
        assert_eq!(packed, again);
        assert_eq!(content_hash(&packed), content_hash(&again));
    }

    #[semio_framework_async_macros::async_test]
    async fn content_hash_is_stable_blake3() {
        let packed = pack(&sample_manifest(), b"component-bytes", &[]).await.expect("pack");
        assert_eq!(content_hash(&packed), semio_framework_hash::hash_bytes(&packed));
        assert_ne!(content_hash(&packed), content_hash(b"other"));
    }

    #[semio_framework_async_macros::async_test]
    async fn verify_rejects_wrong_envelope() {
        let foreign = wrap_binary(
            &SemioEnvelope {
                plugin: "os".into(),
                artifact: "collection".into(),
                component: Component::Pack,
                version: 1,
            },
            b"not-an-sxt",
        );
        assert!(matches!(verify(&foreign).await, Err(ExtensionPackageError::UnexpectedEnvelope(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn pack_rejects_empty_component() {
        assert!(matches!(
            pack(&sample_manifest(), b"", &[]).await,
            Err(ExtensionPackageError::EmptyComponent)
        ));
    }
}
