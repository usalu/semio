//! ⚖️ Schema-neutral IFC declaration domains, changed-subtree admission and bounded declaration cursors over owned IFC graph values.
use semio_s_artifact_stdio_ifc_declarations::{IfcCatalog,IfcTypeExpression};
#[path="core.rs"]pub mod domain;
pub use domain::*;
#[path="cursor.rs"]pub mod cursor;
#[path="ifc4.rs"]mod values;
