//! 🧮️ Controlled native EXPRESS evaluator and the structural population validation over the canonical program.

pub use semio_s_artifact_stdio_ifc4x3_express::{ast, ifc4x3_express_program, metadata, IFC4X3_NATIVE_EXPRESS_EXECUTION, IFC4X3_NATIVE_EXPRESS_IMPLEMENTATION, IFC4X3_RETAINED_EXPRESS_IMPLEMENTATION, IFC4X3_RETAINED_EXPRESS_RECEIVING_IMPLEMENTATION};
#[path = "../evaluator.rs"]
pub mod evaluator;
#[path = "../population.rs"]
pub mod population;
pub use evaluator::{ExpressAggregateKind, ExpressControl, ExpressData, ExpressEvaluationError, ExpressEvaluator, ExpressLogical, ExpressProgress, ExpressResult, ExpressScope, ExpressValue};
pub use population::{prepare_ifc4x3_express, validate_ifc4x3_express, validate_ifc4x3_express_with_program, ExpressDiagnostic, ExpressPhysicalConformance, ExpressRuleKind, ExpressRuleOutcome, ExpressRuleResult, IfcExpressValidation};

#[cfg(test)]
use semio_s_artifact_stdio_ifc4x3_schema as catalog;
#[cfg(test)]
use semio_s_artifact_stdio_ifc4x3_step as io;
#[cfg(test)]
use ast::{ExpressAggregateEntry, ExpressCase, ExpressExpression, ExpressExpressionKind, ExpressFunction, ExpressGlobalRule, ExpressInverse, ExpressProgram, ExpressRule, ExpressStatement, ExpressStatementKind, ExpressVariable};

#[cfg(test)]
#[path = "../execution-tests.rs"]
mod execution_tests;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_native_program_matches_language_neutral_corpus_counts() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../native-cases.json")).unwrap();
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
