//! ⚖️ Native canonical EXPRESS program and controlled semantic implementation.

pub mod ast;
pub mod evaluator;
pub mod population;
pub mod metadata;
mod retained;
pub mod overlay;
pub mod receive;
pub mod admission;
pub mod machine;
pub mod local;
#[path = "program-index.rs"]
pub mod program_index;
use ast::{ExpressAggregateEntry, ExpressCase, ExpressExpression, ExpressExpressionKind, ExpressFunction, ExpressGlobalRule, ExpressInverse, ExpressProgram, ExpressRule, ExpressStatement, ExpressStatementKind, ExpressVariable};
pub use evaluator::{ExpressAggregateKind, ExpressControl, ExpressData, ExpressEvaluationError, ExpressEvaluator, ExpressLogical, ExpressProgress, ExpressResult, ExpressScope, ExpressValue};
pub use population::{prepare_ifc4x3_express, validate_ifc4x3_express, validate_ifc4x3_express_with_program, ExpressDiagnostic, ExpressPhysicalConformance, ExpressRuleKind, ExpressRuleOutcome, ExpressRuleResult, IfcExpressValidation};

include!("program.rs");

pub const IFC4X3_NATIVE_EXPRESS_EXECUTION: bool = false;
pub const IFC4X3_NATIVE_EXPRESS_IMPLEMENTATION: bool = true;
pub const IFC4X3_RETAINED_EXPRESS_RECEIVING_IMPLEMENTATION: bool = true;
pub const IFC4X3_RETAINED_EXPRESS_IMPLEMENTATION: bool = false;

/// 📖️ Borrow the complete precompiled normative syntax without parsing or allocation.
pub fn ifc4x3_express_program() -> &'static ExpressProgram {
    &PROGRAM
}

#[cfg(test)]
#[path = "execution-tests.rs"]
mod execution_tests;

#[cfg(test)]
#[path = "retained-tests.rs"]
mod retained_tests;

#[cfg(test)]
#[path = "overlay-tests.rs"]
mod overlay_tests;

#[cfg(test)]
#[path = "receive-tests.rs"]
mod receive_tests;

#[cfg(test)]
#[path = "receiving-goldens.rs"]
mod receiving_goldens;

#[cfg(test)]
#[path = "machine-tests.rs"]
mod machine_tests;

#[cfg(test)]
#[path = "machine-model-tests.rs"]
mod machine_model_tests;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_native_program_matches_language_neutral_corpus_counts() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("native-cases.json")).unwrap();
        let program = ifc4x3_express_program();
        assert_eq!(program.schema, fixture["schema"].as_str().unwrap());
        for (name, actual) in [
            ("rules", program.rules.len()),
            ("derived", program.derived.len()),
            ("functions", program.functions.len()),
            ("globalRules", program.global_rules.len()),
            ("inverse", program.inverse.len()),
            ("bounds", program.bounds.len()),
            ("unique", program.unique.len()),
        ] {
            assert_eq!(actual, fixture["compiled"][name].as_u64().unwrap() as usize, "{name}");
        }
        for name in fixture["globalRules"].as_array().unwrap() {
            assert!(program.global_rules.iter().any(|rule| rule.name == name.as_str().unwrap()));
        }
        assert_eq!(IFC4X3_NATIVE_EXPRESS_EXECUTION, fixture["nativeSemanticExecution"].as_bool().unwrap());
    }
}
