//! 📤️ Test host of `media_contract`: runs `🔬️app-artifact-media-export-credit` in the facade, where the names it uses are in scope.

use semio_framework::MediaType;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::bounded_ownership::ArtifactFixedQueue;
#[allow(unused_imports)]
use semio_framework_plugin_decl_media::{ARTIFACT_OUTPUT_CHUNK_BYTES, ArtifactDownloadOutput, ArtifactMediaExportCredit, ArtifactMediaExportResult, ArtifactOutputChunks, validate_media_export_structure};
include!("../../../../🧪️tests/🔬️app-artifact-media-export-credit/🦀️.rs");
