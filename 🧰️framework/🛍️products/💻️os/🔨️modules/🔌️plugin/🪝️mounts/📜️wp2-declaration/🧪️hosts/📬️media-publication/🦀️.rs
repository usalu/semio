//! 📬️ Test host of `media_contract`: runs `🎟️publication` in the facade, where the names it uses are in scope.

use semio_framework::MediaError;
use semio_framework::MediaType;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_plugin_decl_app::ArtifactApp;
#[allow(unused_imports)]
use semio_framework_plugin_decl_media::{ArtifactDownloadOutput, ArtifactMediaExportCompletion, ArtifactMediaExportHandle, ArtifactMediaExportPoll, ArtifactMediaExportResult, ArtifactOutputChunks};
#[doc(hidden)]
#[path = "../../../../📜️declaration/🎞️media/🧪️tests/🎟️publication/🦀️.rs"]
pub mod original_tests;
pub(crate) use crate::media_publication_tests_host::original_tests::original_completion_result_receiving_and_physical_close;
pub(crate) use crate::media_publication_tests_host::original_tests::original_handle_receiving_and_physical_close;
pub(crate) use crate::media_publication_tests_host::original_tests::original_download_receiving_chunk_and_physical_close;
