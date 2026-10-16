//! 🧪️ Unit 1 of the facade test host of `document-media`: out of line because its `#[path]` mounts must not pass through a module directory that does not exist.

use semio_framework::MediaError;
use semio_framework_plugin_document_media::media_operation_refusal::MediaOperationRefusal;
use semio_framework_plugin_document_media::media_operation_refusal::MediaOperationSource;
use semio_framework::MediaType;
use semio_framework_job::OriginalDomainProjection;
use semio_framework_value::RetainedCloneProgress;
use semio_framework_job::OriginalDomainView as V;
use semio_framework_plugin_decl_app::media_type;
use semio_framework_job::OriginalDomainFailure;
#[cfg(any(test,feature="media-turn-testing"))]
#[path="../../../../../🎞️media/⚠️turn/🧪️tests/🦀️.rs"]
pub mod tests;
