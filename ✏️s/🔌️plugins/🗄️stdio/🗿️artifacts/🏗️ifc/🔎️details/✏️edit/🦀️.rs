//! ✏️ Edit layer: value, reference, enumeration and domain edits over the retained IFC graph.

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
pub use semio_s_artifact_stdio_ifc_details_type::*;
#[path="reference.rs"]pub mod reference;
#[path="value_edit.rs"]pub mod value_edit;
#[path="../../📥️source/🗂️index/🦀️.rs"]mod source_index;
#[path="contextual.rs"]pub mod contextual;
pub use contextual::{IfcEnumerationAddress,IfcEnumerationDiagnostic,IfcEnumerationQuery,IfcEnumerationView};
#[path="contextual_render.rs"]pub mod contextual_render;
pub use contextual_render::IfcDetailsRenderWork;
#[path="contextual_edit.rs"]pub mod contextual_edit;
#[path="domain_work.rs"]pub mod domain_work;
#[path="domain_edit.rs"]pub mod domain_edit;
