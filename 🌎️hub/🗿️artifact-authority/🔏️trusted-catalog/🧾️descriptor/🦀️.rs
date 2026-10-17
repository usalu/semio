//! 🧾️ Trusted-catalog descriptor: bundle and descriptor validation, document-open target derivation, native codec provider vocabulary and the retained asset handles of a verified package.

use std::sync::Arc;
use directory::os_store::serialization::ArtifactCodec;
use semio_hub_artifact_authority::{AuthorityError, AuthorityProgress, AuthorityProgressStage, OperationContext};
use std::collections::{BTreeMap, BTreeSet};
use directory::os_component_receipt::ComponentReceiptV1;
use directory::os_directory::schema::{DocumentBrowserActorSourceV1, DocumentOpenBrowserActorV1};
use directory::os_directory::{DocumentOpenArtifactV1, DocumentOpenGrantV1, DocumentOpenPackageV1, DocumentOpenRendererTargetV1, DocumentOpenSurfaceRoleV1, DocumentOpenSurfaceV1};
use semio_framework::{DslValue, PackageDescriptor, Version};
use semio_framework_hash::{Hasher, Sha256};
use semio_framework_plugin_host::PackageRef;
use semio_hub_trusted_catalog_contract::{TRUSTED_BUNDLE_MAX_PROFILES, TRUSTED_CATALOG_MAX_OPEN_TARGETS, TRUSTED_COMPONENT_MAX_BYTES, TRUSTED_DESCRIPTOR_MAX_BYTES, TRUSTED_DESCRIPTOR_MAX_MATERIALIZATION, TRUSTED_PACKAGE_MAX_DEPENDENCIES, TRUSTED_READ_CHUNK_BYTES, TRUSTED_RELATIVE_PATH_MAX_BYTES, TrustedCatalogGenerationRoot, TrustedCatalogRelativePathV1, append_document_open_catalog_field, catalog, catalog_error, decode_digest, dual_hash, ensure_count, sha256, valid_identity, valid_open_identity, valid_package_id, validate_file, validate_identity, verify_digest, verify_length};
use semio_hub_trusted_catalog_contract::schema::{TRUSTED_BUNDLE_SCHEMA_VERSION, TrustedBundleFileV1, TrustedBundleGrantV1, TrustedBundleIdentityV1, TrustedBundleOpenRole, TrustedBundleOpenTargetV1, TrustedBundlePackageRole, TrustedBundlePackageV1, TrustedBundleProfileV1, TrustedBundleRendererTarget, TrustedBundleV1};
use semio_hub_artifact_authority::adapters::{TRUSTED_CATALOG_MAX_CODECS, TRUSTED_CATALOG_MAX_PACKAGES};
use semio_hub_trusted_catalog_contract::plugin_module::{TRUSTED_PLUGIN_MODULE_BUNDLE_MAX_BYTES, TrustedPluginModuleSourceV1, decode_plugin_module_bundle, plugin_module_blob_path, verify_plugin_module_file};
use directory::os_plugin_module_schema::{TrustedPluginModuleBundleV1, TrustedPluginModuleFileV1};
use directory::os_directory::io::binary::artifact_hash::hex_lower;
use directory::os_store;

#[path = "📋️bundle-validation/🦀️.rs"]
mod bundle_validation;
pub use bundle_validation::*;

#[path = "🔌️native-provider/🦀️.rs"]
mod native_provider;
pub use native_provider::*;

#[path = "💾️retained-assets/🦀️.rs"]
mod retained_assets;
pub use retained_assets::*;

#[path = "🎯️open-targets/🦀️.rs"]
mod open_targets;
pub use open_targets::*;

#[path = "🔍️descriptor-validation/🦀️.rs"]
mod descriptor_validation;
pub use descriptor_validation::*;
