//! ⚖️ Native canonical EXPRESS program and controlled semantic implementation, assembled from the express, evaluator and machine crates.

pub use semio_s_artifact_stdio_ifc4x3_express::{ast, ifc4x3_express_program, metadata, program_index, IFC4X3_NATIVE_EXPRESS_EXECUTION, IFC4X3_NATIVE_EXPRESS_IMPLEMENTATION, IFC4X3_RETAINED_EXPRESS_IMPLEMENTATION, IFC4X3_RETAINED_EXPRESS_RECEIVING_IMPLEMENTATION};
pub use semio_s_artifact_stdio_ifc4x3_evaluator::{evaluator, population, prepare_ifc4x3_express, validate_ifc4x3_express, validate_ifc4x3_express_with_program, ExpressAggregateKind, ExpressControl, ExpressData, ExpressDiagnostic, ExpressEvaluationError, ExpressEvaluator, ExpressLogical, ExpressPhysicalConformance, ExpressProgress, ExpressResult, ExpressRuleKind, ExpressRuleOutcome, ExpressRuleResult, ExpressScope, ExpressValue, IfcExpressValidation};
pub use semio_s_artifact_stdio_ifc4x3_population::{admission, overlay, receive};
pub use semio_s_artifact_stdio_ifc4x3_rules::{local, machine, NewMachine};

#[cfg(test)]
#[path = "receiving-goldens.rs"]
mod receiving_goldens;
