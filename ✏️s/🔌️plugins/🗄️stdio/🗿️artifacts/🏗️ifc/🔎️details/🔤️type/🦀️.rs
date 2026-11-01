//! 🔤️ Type layer: guided type input cursors, type remapping and the constituent roster over declaration metadata.

use semio_s_artifact_stdio_ifc_declarations::{IfcAttributeDeclaration, IfcCatalog, IfcTypeExpression};
use semio_framework_plugin::Fault;
use semio_framework_ui_locale::Locale;
use semio_framework_value::{DslValue, FromValue, RetainedWorkOutcome, ToValue};
use semio_s_artifact_stdio_contract::editing;
use semio_s_artifact_stdio_contract::editing::{DslSnapshotDetailsProvider, SnapshotDetailItemCapabilities, SnapshotDetailPathSegment as P, SnapshotDetailPresentation, SnapshotDetailValue, SnapshotDetailVariant, SnapshotDetailsProvider};
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::{ifc_arguments_identical, ifc_complex_identical, IfcComplexType, IfcEntity, IfcSnapshot, IfcTypedValue, IfcValue};
use semio_s_artifact_stdio_ifc_4_mutations::standards::v4::subsets::any::schema::mutations::{edit_rules, IfcMutation};
pub use semio_s_artifact_stdio_ifc_details_wire::*;
pub use semio_s_artifact_stdio_ifc_domain as domain;
pub use semio_s_artifact_stdio_ifc_domain::cursor as domain_cursor;
#[path="type_remap.rs"]pub mod type_remap;
#[path="type_input.rs"]pub mod type_input;
#[path="type_input_kind.rs"]pub mod type_input_kind;
#[path="type_input_select.rs"]pub mod type_input_select;
#[path="constituent_roster.rs"]pub mod constituent_roster;

pub fn requested_type(value: &DslValue) -> Option<&str> {
    type_input::request(value).map(|(name,_)|name)
}
