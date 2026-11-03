//! 🧶️ Guidance layer: staged type-input sources and guided candidates.

use semio_s_artifact_stdio_ifc_declarations::{IfcAttributeDeclaration, IfcCatalog, IfcTypeExpression};
use semio_framework_plugin::Fault;
use semio_framework_ui_locale::Locale;
use semio_framework_value::{DslValue, FromValue, RetainedWorkOutcome, ToValue};
use semio_s_artifact_stdio_contract::editing;
use semio_s_artifact_stdio_contract::editing::{DslSnapshotDetailsProvider, SnapshotDetailItemCapabilities, SnapshotDetailPathSegment as P, SnapshotDetailPresentation, SnapshotDetailValue, SnapshotDetailVariant, SnapshotDetailsProvider};
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::{ifc_arguments_identical, ifc_complex_identical, IfcComplexType, IfcEntity, IfcSnapshot, IfcTypedValue, IfcValue};
use semio_s_artifact_stdio_ifc_4_mutations::standards::v4::subsets::any::schema::mutations::{edit_rules, IfcMutation};
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
pub use semio_s_artifact_stdio_ifc_details_edit::*;
#[path="type_guidance_request.rs"]pub mod type_guidance_request;
#[path="type_guidance_candidate.rs"]pub mod type_guidance_candidate;
#[path="type-guidance.rs"]pub mod type_guidance;
pub use type_guidance::{IfcTypeInputSourceWork,IfcPreparedTypeInputSource};
