//! 🗂️ Immutable trusted-catalog bundle verification for headless hub authority startup.

use semio_hub_artifact_authority::adapters::{bounded_message, AUTHORITY_MAX_CODEC_TEXT_BYTES, TRUSTED_CATALOG_MAX_CODECS};
use semio_hub_artifact_authority::{AcceptedArtifactOperation, ArtifactPair, ArtifactValidationStage, AuthorityError, AuthorityProgress, AuthorityProgressStage, OperationContext, TrustedArtifactCatalog, TrustedArtifactCodec, TrustedArtifactGenesisCodec, TrustedArtifactIdentity, TrustedArtifactReplayCodec};
use directory::os_directory::{DocumentDescriptor, DocumentExecutionProtocolV1, DocumentOpenArtifactV1, DocumentOpenGrantV1, DocumentOpenPackageV1, DocumentOpenRendererTargetV1, DocumentOpenSurfaceRoleV1, DocumentOpenSurfaceV1};
use directory::os_directory::io::binary::artifact_hash::hex_lower;
use directory::os_store::{self, serialization::ArtifactCodec};
use directory::io::{ArtifactAssemblyRegistryPlan,ArtifactCodecBinding,ArtifactCatalogBinding,ArtifactCatalogCapability,ArtifactCatalogTarget};
use semio_framework::{DslValue, PackageDescriptor, Version};
use semio_framework_hash::{Hasher, Sha256};
use semio_framework_plugin_host::{PackageHash, PackageId, PackageRef};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::sync::Arc;

use semio_hub_trusted_catalog_contract::schema::{publication_revision, TrustedBundleFileV1, TrustedBundleOpenRole, TrustedBundleOpenTargetV1, TrustedBundlePackageV1, TrustedBundleRendererTarget, TrustedBundleV1, TrustedCatalogCurrentPointerV1, TrustedCatalogPublicationCommandV1, TrustedCatalogPublicationReceiptV1, TRUSTED_CATALOG_PUBLICATION_MAX_BYTES, TRUSTED_CATALOG_PUBLICATION_OUTCOME_DURABLE, TRUSTED_CATALOG_PUBLICATION_OUTCOME_UNCONFIRMED, TRUSTED_CATALOG_PUBLICATION_RECEIPT_SCHEMA, TRUSTED_CATALOG_PUBLICATION_SCHEMA, TRUSTED_CATALOG_GUEST_RESIDENCY, TrustedCatalogGuestResidencyStateV1, TrustedCatalogGuestResidencyV1, TrustedCatalogLoadProgressV1, TrustedCatalogPackagePhaseV1, TrustedCatalogPackageProgressV1, GuestCodecVerificationV1, GUEST_CODEC_VERIFICATION_SCHEMA};
use directory::os_component_receipt::ComponentReceiptV1;
use directory::os_plugin_module_schema::{TrustedPluginModuleBundleV1, TrustedPluginModuleFileV1, TrustedPluginModuleIndexEntryV1, TrustedPluginModuleIndexV1, TRUSTED_PLUGIN_MODULE_INDEX_SCHEMA};

use semio_hub_trusted_catalog_contract::plugin_module::{plugin_module_blob_path, TrustedPluginModuleSourceV1};
use semio_hub_trusted_catalog_contract::{TrustedCatalogDataRoot, TrustedCatalogGenerationRoot, TrustedCatalogRelativePathV1, TRUSTED_READ_CHUNK_BYTES};
#[cfg(all(any(test, feature = "testkit"),unix))]
use semio_hub_trusted_catalog_contract::create_fifo_fixture;
use directory::os_directory::schema::{DocumentBrowserActorSourceV1, DocumentOpenBrowserActorV1, DOCUMENT_BROWSER_ACTOR_MAX_BYTES};
use semio_hub_trusted_catalog_descriptor::{CodecKey, NativeCodecProviderPackageV1, NativeCodecProviderRequirementV1, NativeCodecProviderSourceV1, SelectedTrustedBundleV1, TrustedCatalogAsset, TrustedRetainedFile, VerifiedDocumentOpenSelectionV1, VerifiedExecutionTargetAssets, VerifiedPluginModule, VerifiedTrustedPackage, declared_kind, decode_package_descriptor, document_open_target_sort_key, hosted_codec_targets, most_general_dialect, open_target_codec_package, owner_preferred_creation, package_actor_renderer, report_package_progress, sort_open_targets, trusted_profile_generation, validate_bundle, validate_descriptor, validate_descriptor_open_target, validate_native_bindings, verify_plugin_module};
use semio_hub_trusted_catalog_contract::{TRUSTED_BUNDLE_MAX_BYTES, TRUSTED_CATALOG_MAX_OPEN_TARGETS, TRUSTED_COMPONENT_MAX_BYTES, TRUSTED_DESCRIPTOR_MAX_BYTES, TRUSTED_IDENTITY_MAX_BYTES, catalog, catalog_error, decode_digest, dual_hash, sha256, verify_digest, verify_length};

#[path = "📮️publisher/🦀️.rs"]
mod publisher;
pub use publisher::*;

#[path = "🧊️guest-residency/🦀️.rs"]
mod guest_residency;
pub use guest_residency::*;

#[path = "📈️load-progress/🦀️.rs"]
mod load_progress;
pub use load_progress::*;

#[path = "🗜️guest-codec/🦀️.rs"]
mod guest_codec;
pub use guest_codec::*;

#[path = "🔗️native-codec/🦀️.rs"]
mod native_codec;
pub use native_codec::*;

#[path = "📚️verified-catalog/🦀️.rs"]
mod verified_catalog;
pub use verified_catalog::*;

#[path = "🗃️verification-cache/🦀️.rs"]
mod verification_cache;
pub use verification_cache::*;

#[path = "🏗️loader/🦀️.rs"]
mod loader;
pub use loader::*;

#[cfg(any(test, feature = "integration-fixtures"))]
#[path = "🎭️fixture-builders/🦀️.rs"]
mod fixture_builders;
#[cfg(any(test, feature = "integration-fixtures"))]
pub use fixture_builders::*;
