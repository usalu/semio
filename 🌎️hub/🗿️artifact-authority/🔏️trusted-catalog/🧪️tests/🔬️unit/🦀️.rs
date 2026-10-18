use directory::os_directory::io::binary::artifact_hash::hex_lower;

use semio_hub_artifact_authority::adapters::AUTHORITY_MAX_DIAGNOSTIC_BYTES;
use semio_hub_native_openable::NativeCodecProviderSetV1;
use semio_hub_trusted_catalog_contract::schema::{TrustedBundleBrowserActorV1, TrustedBundleCodecV1, TrustedBundleComponentV1, TrustedBundlePluginModuleV1, TrustedBundleProfileOpenTargetV1};
use semio_hub_artifact_authority::{AuthorityLimits, AuthorityOperationControl, TrustedArtifactCatalog};
use directory::os_store::{serialization::ArtifactPackFiles, text_format::ArtifactTextFiles, VcsError, codec_registry::document_codec};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static FIXTURE_SEQUENCE: AtomicUsize = AtomicUsize::new(0);

include!("🧊️guest-residency-and-codec-laws/🦀️.rs");

include!("📮️publication-laws/🦀️.rs");

include!("🎭️fixture-harness/🦀️.rs");

include!("🧭️provider-selection-laws/🦀️.rs");

include!("🛡️loader-trust-laws/🦀️.rs");

include!("🧪️owned-guest-laws/🦀️.rs");

include!("🎯️open-target-laws/🦀️.rs");

include!("📦️plugin-module-laws/🦀️.rs");
