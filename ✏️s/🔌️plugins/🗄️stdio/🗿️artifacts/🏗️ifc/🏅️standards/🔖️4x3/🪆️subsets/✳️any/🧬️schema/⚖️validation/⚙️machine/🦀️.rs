//! ⚙️ Retained resumable EXPRESS machine core: values, frames, expressions, operations, statements and population operations; the IFC-domain frame kinds are executed by the hook its constructor receives.
pub use semio_s_artifact_stdio_ifc4x3_express::{ast, ifc4x3_express_program, metadata, program_index, IFC4X3_NATIVE_EXPRESS_EXECUTION, IFC4X3_NATIVE_EXPRESS_IMPLEMENTATION, IFC4X3_RETAINED_EXPRESS_IMPLEMENTATION, IFC4X3_RETAINED_EXPRESS_RECEIVING_IMPLEMENTATION};
pub use semio_s_artifact_stdio_ifc4x3_population::{admission, overlay, receive, retained};
#[path = "../machine.rs"]
pub mod machine;
