//! 🏗️ Feature-gated builder for one real GIS Map editor profile, loadable by every crate target.
//!
//! `mod tests` cannot serve the `os-hub` binary target or any other crate, because `#[cfg(test)]` is
//! per-compilation-unit. This module is compiled under the `test-support` Cargo feature instead, so a
//! binary-target law can build a genuine [`VerifiedTrustedCatalog`] and the frozen
//! [`VerifiedGisMapArtifactBindingV1`] the hub inference runtime requires. It goes through the exact
//! production loader — no second, divergent trust check exists — and it binds real
//! `semio_s_plugin_gis` descriptor, service and native-codec metadata. Only the component bytes are
//! synthetic, as are the browser actor bytes: this profile never executes either, and must never be offered as evidence
//! that one was executed.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use super::{TrustedCatalogLoader, VerifiedTrustedCatalog};
use crate::artifact_authority::native_openable_provider::NativeCodecProviderSetV1;
use crate::artifact_authority::{AuthorityError, AuthorityLimits, AuthorityOperationControl, AuthorityProgress, OperationContext};
use crate::inference::VerifiedGisMapArtifactBindingV1;
use directory::os_directory::hex_lower;
use directory::os_store;
use semio_framework::to_dsl_value;
use semio_framework_hash::{Hasher, Sha256};

const SYNTHETIC_COMPONENT: &[u8] = b"synthetic-gis-component-for-hub-test-support-profile";

/// 🆔️ The profile identifier every caller of this builder passes to the loader.
pub const GIS_MAP_TEST_PROFILE_ID: &str = "gis-map-test-support";

/// ⏱️ An uncancelled fixed-clock control; the builder never exercises cancellation itself.
struct BuilderControl;

impl AuthorityOperationControl for BuilderControl {
    fn now_ms(&self) -> u64 {
        1_000
    }

    fn is_cancelled(&self) -> bool {
        false
    }

    fn report(&self, _progress: AuthorityProgress) {}
}

/// 🗂️ One retained on-disk GIS Map profile; its directory is removed when the value is dropped.
pub struct VerifiedGisMapTestProfileV1 {
    root: PathBuf,
    bundle_path: PathBuf,
    catalog: Arc<VerifiedTrustedCatalog>,
    binding: Arc<VerifiedGisMapArtifactBindingV1>,
}

impl VerifiedGisMapTestProfileV1 {
    /// 🗺️ Returns the frozen GIS Map editor binding this profile verified.
    pub fn binding(&self) -> &Arc<VerifiedGisMapArtifactBindingV1> {
        &self.binding
    }

    /// 🗂️ Returns the verified catalog the binding retains for its whole lifetime.
    pub fn catalog(&self) -> &Arc<VerifiedTrustedCatalog> {
        &self.catalog
    }

    /// 📄️ Returns the bundle path a candidate hub would receive through its startup environment.
    pub fn bundle_path(&self) -> &Path {
        &self.bundle_path
    }
}

impl Drop for VerifiedGisMapTestProfileV1 {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

/// 🏗️ Writes one bundle with real GIS metadata under `root` and loads it through the real loader.
///
/// The returned binding satisfies every check in `verified_gis_map_binding`: plugin `gis`, package
/// `semio:gis`, artifact kind `s.gis.gismap`, an `Editor` surface with `read`+`write`+`observe`, the
/// declared `s.gis.gismap.inference` service, and an executable identity equal to the compiled
/// `gis_map_inference_service()`.
pub async fn verified_gis_map_test_profile(root: &Path) -> Result<VerifiedGisMapTestProfileV1, AuthorityError> {
    let runtime = semio_framework_plugin::plugin_runtime::PluginRuntime::new();
    semio_framework_plugin::plugin_runtime::install_plugin_bundle(&runtime, semio_s_plugin_gis::plugin().map_err(|error| AuthorityError::Catalog(format!("GIS assembly unavailable: {error:?}")))?);
    let emitted = semio_framework_plugin::describe::describe_plugin(&runtime).await;
    let mut descriptor = super::decode_package_descriptor(&emitted)?;
    semio_s_plugin_stdio::registry::validate_native_artifact_catalog_dependency(&descriptor.manifest.dependencies).map_err(super::catalog_error)?;
    semio_s_plugin_stdio::registry::validate_native_artifact_catalog_contributions(&descriptor.manifest.topic_contributions).map_err(super::catalog_error)?;
    let component_sha256 = hex_lower(&Sha256::digest(SYNTHETIC_COMPONENT));
    let mut component_blake3 = Hasher::new();
    component_blake3.update(SYNTHETIC_COMPONENT);
    descriptor.hashes.wasm_sha256 = component_sha256.clone();
    descriptor.hashes.core_wasm_sha256 = component_sha256.clone();
    descriptor.hashes.descriptor_sha256.clear();
    descriptor.hashes.descriptor_sha256 = hex_lower(&Sha256::digest(&os_store::pack_rt::encode_wire_value(&to_dsl_value(&descriptor).map_err(|error| AuthorityError::Catalog(format!("GIS descriptor self-hash projection failed: {error}")))?)));
    let descriptor_bytes = os_store::pack_rt::encode_wire_value(&to_dsl_value(&descriptor).map_err(|error| AuthorityError::Catalog(format!("GIS descriptor projection failed: {error}")))?);
    let native_codecs: Vec<_> = semio_s_plugin_gis::native_codecs::native_codec_factory_receipts()
        .map_err(|error| AuthorityError::Catalog(format!("GIS native codec receipts unavailable: {error:?}")))?
        .into_iter()
        .map(|receipt| {
            let identity = receipt.identity();
            serde_json::json!({ "artifactKind": identity.artifact_kind, "artifactSchema": identity.schema, "packSchemaHash": hex_lower(&identity.pack_schema_hash) })
        })
        .collect();
    let map_pack_schema_hash = native_codecs.iter().find(|codec| codec["artifactKind"] == "s.gis.gismap").ok_or_else(|| AuthorityError::Catalog("GIS receipts declare no s.gis.gismap codec".to_owned()))?["packSchemaHash"].clone();
    let editor =
        descriptor.manifest.apps.iter().find(|app| app.role == semio_framework::AppRole::Editor && app.dialect.artifact_kind == "s.gis.gismap").ok_or_else(|| AuthorityError::Catalog("GIS assembly declares no s.gis.gismap editor".to_owned()))?;
    let window_kind = editor.window_kinds.first();
    let package = serde_json::json!({ "pluginId": descriptor.manifest.plugin_id, "packageId": descriptor.package_id, "version": descriptor.manifest.version });
    let target = serde_json::json!({
        "artifactKind": "s.gis.gismap",
        "artifactSchema": "gis.map",
        "packSchemaHash": map_pack_schema_hash,
        "surfaceId": editor.id,
        "appId": editor.id,
        "windowKindId": window_kind.id,
        "role": "editor",
        "rendererTarget": "wasm",
        "parentDialect": { "artifactKind": "s.gis.gismap", "standard": "1", "subset": "*" },
        "grant": { "read": true, "write": true, "observe": true }
    });
    std::fs::create_dir_all(root).map_err(|error| AuthorityError::Catalog(format!("test-support profile directory unavailable: {error}")))?;
    std::fs::write(root.join("component.wasm"), SYNTHETIC_COMPONENT).map_err(|error| AuthorityError::Catalog(format!("component write failed: {error}")))?;
    std::fs::write(root.join("descriptor.semio"), &descriptor_bytes).map_err(|error| AuthorityError::Catalog(format!("descriptor write failed: {error}")))?;
    std::fs::write(root.join("closed-actor.mjs"), SYNTHETIC_COMPONENT).map_err(|error| AuthorityError::Catalog(format!("synthetic actor write failed: {error}")))?;
    let (stdio_identity, stdio_record) = super::headless_stdio_fixture_package(root)?;
    let mut bundle = serde_json::json!({
        "schemaVersion": 2,
        "profiles": [{
            "id": GIS_MAP_TEST_PROFILE_ID,
            "selectedClosure": [package.clone(), stdio_identity.clone()],
            "selectedClosureSha256": "01".repeat(32),
            "openTarget": { "package": package.clone(), "target": target.clone() },
            "generationId": "02".repeat(32)
        }],
        "packages": [{
            "pluginId": package["pluginId"], "packageId": package["packageId"], "version": package["version"], "role": "plugin", "dependencies": [stdio_identity],
            "executionProtocol": { "appChannelVersion": descriptor.execution_protocol.app_channel_version },
            "component": { "path": "component.wasm", "byteLength": SYNTHETIC_COMPONENT.len(), "sha256": component_sha256, "blake3": hex_lower(component_blake3.finalize().as_bytes()) },
            "descriptor": { "path": "descriptor.semio", "byteLength": descriptor_bytes.len(), "sha256": hex_lower(&Sha256::digest(&descriptor_bytes)) },
            "browserActor": {
                "kind":"closed-browser-actor", "schema":"semio.os.closed-browser-actor.v1", "codegenPolicy":"semio.os.browser-jco-1.27.0-jspi.v1",
                "path":"closed-actor.mjs", "byteLength":SYNTHETIC_COMPONENT.len(), "sha256":component_sha256,
                "sourceComponentSha256":component_sha256, "sourceDescriptorByteSha256":hex_lower(&Sha256::digest(&descriptor_bytes)), "policySha256":"41".repeat(32), "importInterfaces":[]
            },
            "nativeCodecs": native_codecs, "openTargets": [target]
        }, stdio_record]
    });
    let decoded: super::TrustedBundleV1 = serde_json::from_value(bundle.clone()).map_err(|error| AuthorityError::Catalog(format!("test-support bundle shape invalid: {error}")))?;
    bundle["profiles"][0]["selectedClosureSha256"] = hex_lower(&super::selected_closure_digest(&decoded.profiles[0].selected_closure)?).into();
    let regenerated: super::TrustedBundleV1 = serde_json::from_value(bundle.clone()).map_err(|error| AuthorityError::Catalog(format!("test-support bundle shape invalid: {error}")))?;
    bundle["profiles"][0]["generationId"] = super::trusted_profile_generation(&regenerated, &regenerated.profiles[0])?.into();
    let bundle_path = root.join("trusted-catalog.json");
    std::fs::write(&bundle_path, serde_json::to_vec_pretty(&bundle).map_err(|error| AuthorityError::Catalog(format!("bundle encode failed: {error}")))?).map_err(|error| AuthorityError::Catalog(format!("bundle write failed: {error}")))?;
    let control = BuilderControl;
    let context = OperationContext::new(u64::MAX, AuthorityLimits::maximum(), &control);
    let catalog = Arc::new(TrustedCatalogLoader::load_fixture(&bundle_path, GIS_MAP_TEST_PROFILE_ID, &NativeCodecProviderSetV1::linked(), &context).await?);
    let binding = crate::inference::verified_gis_map_binding(catalog.clone())
        .map_err(|error| AuthorityError::Catalog(format!("verified GIS Map binding rejected the test-support profile: {error:?}")))?
        .ok_or_else(|| AuthorityError::Catalog("test-support profile did not select a writable GIS Map editor".to_owned()))?;
    Ok(VerifiedGisMapTestProfileV1 { root: root.to_path_buf(), bundle_path, catalog, binding })
}

/// 📂️ Returns a process-unique directory under the system temporary root for one profile.
pub fn unique_profile_root(label: &str) -> PathBuf {
    static SEQUENCE: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let sequence = SEQUENCE.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map_or_else(std::env::temp_dir, PathBuf::from).join(format!("semio-hub-gis-map-{label}-{}-{sequence}", std::process::id()))
}
