//! 📨️ Wire layer: borrowed pointers, owned receivers, publications and source loans over the retained IFC graph.

use semio_s_artifact_stdio_ifc_declarations::{IfcAttributeDeclaration, IfcCatalog, IfcTypeExpression};
use semio_plugin::Fault;
use semio_ui_locale::Locale;
use semio_framework_value::{DslValue, FromValue, RetainedWorkOutcome, ToValue};
use semio_s_artifact_stdio_contract::editing;
use semio_s_artifact_stdio_contract::editing::{DslSnapshotDetailsProvider, SnapshotDetailItemCapabilities, SnapshotDetailPathSegment as P, SnapshotDetailPresentation, SnapshotDetailValue, SnapshotDetailVariant, SnapshotDetailsProvider};
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::{ifc_arguments_identical, ifc_complex_identical, IfcComplexType, IfcEntity, IfcSnapshot, IfcTypedValue, IfcValue};
use semio_s_artifact_stdio_ifc_4_mutations::standards::v4::subsets::any::schema::mutations::{edit_rules, IfcMutation};
extern crate semio_os_kernel as store;
extern crate semio_os_kernel as protocol;
pub use semio_s_artifact_stdio_ifc_details_profile::*;
#[path="receive.rs"]pub mod receive;
pub use receive::{IfcValueReceiveMode, IfcValueReceiver};
#[path="pointer.rs"]pub mod pointer;
#[path="wire.rs"]pub mod wire;
pub use wire::{IfcPublication, SnapshotSource};
#[path="source_loan.rs"]pub mod source_loan;
