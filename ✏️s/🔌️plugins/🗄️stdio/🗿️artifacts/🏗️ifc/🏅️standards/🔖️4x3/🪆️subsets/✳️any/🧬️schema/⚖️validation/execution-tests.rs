//! 🧪 Native semantic execution contracts share the TypeScript and IfcOpenShell corpus.
use super::*;
use evaluator::{ExpressControl, ExpressData, ExpressEvaluator, ExpressScope};
include!("expression-cases.rs");
include!("statement-cases.rs");
include!("official-cases.rs");
include!("binding-cases.rs");

fn parse_graph(source: &str) -> semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcSnapshot {
    semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::from_part21_document("stdio.ifc", &semio_s_artifact_stdio_contract::part21::parse_part21(source).unwrap())
}
fn graph_body(body: &str) -> semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcSnapshot {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("models.json")).unwrap();
    parse_graph(&format!("{}{}{}", fixture["header"].as_str().unwrap(), body, fixture["footer"].as_str().unwrap()))
}

#[test]
fn native_scalar_cases_match_language_neutral_expectations() {
    assert_eq!(EXPRESSION_CASES_SHA256.len(), 64);
    assert_eq!(STATEMENT_CASES_SHA256.len(), 64);
    for (source, expression, kind, expected) in EXPRESSION_CASES {
        let mut evaluator = ExpressEvaluator::new(ifc4x3_express_program(), ExpressControl::default());
        let result = evaluator.evaluate(expression, &mut ExpressScope::default()).unwrap();
        assert_eq!(result.kind(), *kind, "{source}");
        if *kind == "unknown" {
            assert!(result.missing(), "{source}");
        } else {
            assert_eq!(result.scalar_text().as_deref(), Some(*expected), "{source}");
        }
    }
}

#[test]
fn native_controls_and_integer_overflow_are_explicit() {
    let expression = ExpressExpression { kind: ExpressExpressionKind::Integer, offset: 0, text: Some("170141183460469231731687303715884105728"), operator: None, name: None, operands: &[], entries: &[] };
    let mut evaluator = ExpressEvaluator::new(ifc4x3_express_program(), ExpressControl::default());
    assert_eq!(evaluator.evaluate(&expression, &mut ExpressScope::default()).unwrap_err().code, "integer-overflow");
    let cancelled = || true;
    let mut evaluator = ExpressEvaluator::new(ifc4x3_express_program(), ExpressControl { cancelled: Some(&cancelled), ..Default::default() });
    assert_eq!(evaluator.evaluate(&expression, &mut ExpressScope::default()).unwrap_err().code, "cancelled");
    let mut evaluator = ExpressEvaluator::new(ifc4x3_express_program(), ExpressControl { maximum_nodes: 0, ..Default::default() });
    assert_eq!(evaluator.make(ExpressData::Integer(1)).unwrap_err().code, "node-budget");
}

#[test]
fn native_population_retains_independent_named_failures() {
    use semio_s_artifact_stdio_contract::part21::parse_part21;
    use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::from_part21_document;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("models.json")).unwrap();
    let source = format!("{}{}{}", fixture["header"].as_str().unwrap(), fixture["body"].as_array().unwrap().iter().map(|row| row.as_str().unwrap()).collect::<Vec<_>>().join("\n"), fixture["footer"].as_str().unwrap());
    let graph = from_part21_document("stdio.ifc", &parse_part21(&source).unwrap());
    let result = population::validate_ifc4x3_express(&graph, ExpressControl::default());
    assert!(result.complete, "{:?}", result.diagnostics);
    assert!(!result.valid);
    let actual: std::collections::BTreeSet<_> = result.diagnostics.iter().filter_map(|row| Some(format!("{}.{}", row.owner.as_ref()?, row.rule.as_ref()?))).collect();
    let expected: std::collections::BTreeSet<_> = fixture["expectedFailures"].as_array().unwrap().iter().map(|row| row.as_str().unwrap().to_string()).collect();
    assert_eq!(actual, expected);
}

#[test]
fn native_statement_cases_match_shared_control_flow_contracts() {
    for (name, arguments, expected) in STATEMENT_CASES {
        let mut evaluator = ExpressEvaluator::new(&STATEMENT_PROGRAM, ExpressControl::default());
        let arguments = arguments.iter().map(|value| evaluator.make(ExpressData::Integer(*value)).unwrap()).collect();
        let result = evaluator.call(name, arguments, &mut ExpressScope::default()).unwrap();
        assert_eq!(result.integer(), Some(*expected), "{name}");
    }
}

#[test]
fn native_official_populations_match_independent_empty_diagnostics() {
    for source in [include_str!("../../🚪️io/🧪️tests/triangulated.ifc"), include_str!("../../🚪️io/🧪️tests/alignment.ifc"), include_str!("../../🚪️io/🧪️tests/structural.ifc")] {
        let report = validate_ifc4x3_express(&parse_graph(source), ExpressControl::default());
        assert!(report.complete, "{:?}", report.diagnostics);
        assert!(report.valid, "{:?}", report.diagnostics);
        assert!(report.diagnostics.is_empty());
        assert!(report.results.len() > 100);
    }
}

#[test]
fn native_structural_admission_never_accepts_indeterminate_invalid_domains() {
    for body in ["#1=IFCDIRECTION($);", "#1=IFCDIRECTION(('wrong'));", "#1=IFCDIRECTION(());", "#1=IFCDIRECTION((1.,0.));#2=IFCAXIS2PLACEMENT3D(#1,$,$);", "#1=IFCSIUNIT($,.LENGTHUNIT.,$,.METRE.);"] {
        let report = validate_ifc4x3_express(&graph_body(body), ExpressControl::default());
        assert!(!report.complete, "{body}");
        assert!(!report.valid, "{body}");
        assert!(report.results.is_empty());
    }
}

#[test]
fn native_complex_order_and_scalar_inverse_cardinality_are_resolved() {
    let report = validate_ifc4x3_express(&graph_body("#1=(IFCREPRESENTATIONITEM() IFCGEOMETRICREPRESENTATIONITEM() IFCDIRECTION((0.,0.,0.)));"), ExpressControl::default());
    assert!(report.complete, "{:?}", report.diagnostics);
    assert!(report.diagnostics.iter().any(|row| row.rule.as_deref() == Some("MAGNITUDEGREATERZERO")));
    let fixture: serde_json::Value = serde_json::from_str(include_str!("models.json")).unwrap();
    let body =
        fixture["body"].as_array().unwrap().iter().map(|row| row.as_str().unwrap()).collect::<Vec<_>>().join("\n") + "\n#16=IFCRELVOIDSELEMENT('0000000000000000000003',$,$,$,#10,#10);\n#17=IFCRELVOIDSELEMENT('0000000000000000000004',$,$,$,#10,#10);";
    let report = validate_ifc4x3_express(&graph_body(&body), ExpressControl::default());
    assert!(report.complete, "{:?}", report.diagnostics);
    assert!(report.results.iter().any(|row| row.entity_id == Some(10) && row.kind == ExpressRuleKind::Inverse && row.rule == "VOIDSELEMENTS" && row.result == ExpressRuleOutcome::False));
}

#[test]
fn native_progress_cancellation_and_tiny_construction_budgets_are_atomic() {
    use std::cell::Cell;
    let cancelled = Cell::new(false);
    let check = || cancelled.get();
    let mut progress = |_: ExpressProgress| cancelled.set(true);
    let graph = graph_body("#1=IFCDIRECTION((1.,0.));");
    let report = validate_ifc4x3_express(&graph, ExpressControl { batch_size: 1, cancelled: Some(&check), progress: Some(&mut progress), ..Default::default() });
    assert!(!report.complete);
    assert!(!report.valid);
    assert_eq!(report.work, 1);
    assert!(report.results.is_empty());
    let report = validate_ifc4x3_express(&graph, ExpressControl { maximum_work: 3, ..Default::default() });
    assert!(!report.complete);
    assert!(!report.valid);
    assert!(report.results.is_empty());
    let mut evaluator = ExpressEvaluator::new(ifc4x3_express_program(), ExpressControl { maximum_nodes: 3, ..Default::default() });
    let value = evaluator.make(ExpressData::Integer(1)).unwrap();
    let mut items = Vec::new();
    assert!(evaluator.push_value(&mut items, value.clone()).is_ok());
    assert_eq!(evaluator.push_value(&mut items, value).unwrap_err().code, "node-budget");
}

#[test]
fn native_retained_functions_and_rules_execute_indeterminate_admitted_contexts() {
    let program = ifc4x3_express_program();
    let mut evaluator = ExpressEvaluator::new(program, ExpressControl::default());
    let catalog = crate::catalog::ifc4x3_catalog();
    let mut entities = std::collections::BTreeMap::new();
    for (name, declaration) in &catalog.entities {
        let mut fields = std::collections::BTreeMap::new();
        for attribute in &declaration.all_attributes {
            fields.insert(attribute.name.to_ascii_uppercase(), evaluator.make(ExpressData::Unknown).unwrap());
        }
        let mut types = Vec::new();
        let mut owner = Some(name.clone());
        while let Some(name) = owner {
            types.push(name.clone());
            owner = catalog.entities[&name].supertype.clone();
        }
        let entity = evaluator.construct_entity(types, fields).unwrap();
        entities.insert(name.clone(), entity);
    }
    for rule in program.rules {
        let value = if let Some(entity) = entities.get(rule.owner) { entity.clone() } else { evaluator.make(ExpressData::Unknown).unwrap() };
        let mut scope = evaluator.scoped(&value).unwrap();
        let result = evaluator.evaluate(rule.expression, &mut scope).unwrap();
        assert!(["logical", "unknown"].contains(&result.kind()), "{}.{}", rule.owner, rule.label);
    }
    for function in program.functions {
        let mut arguments = Vec::new();
        for parameter in function.parameters {
            for _ in parameter.names {
                arguments.push(evaluator.make(ExpressData::Unknown).unwrap());
            }
        }
        evaluator.call(function.name, arguments, &mut ExpressScope::default()).unwrap();
    }
    assert_eq!(program.rules.len(), 777);
    assert_eq!(program.functions.len(), 48);
}

#[test]
fn native_matching_cases_preserve_multiplicity_identity_and_indeterminacy() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("matching-cases.json")).unwrap();
    for case in cases.as_array().unwrap() {
        let mut evaluator = ExpressEvaluator::new(ifc4x3_express_program(), ExpressControl::default());
        let mut aggregates = Vec::new();
        for side in ["left", "right"] {
            let mut items = Vec::new();
            for item in case[side].as_array().unwrap() {
                let text = item.as_str().unwrap();
                items.push(evaluator.make(if text == "?" { ExpressData::Unknown } else { ExpressData::Integer(text.parse().unwrap()) }).unwrap());
            }
            let kind = match case[format!("{side}Kind")].as_str().unwrap() {
                "BAG" => ExpressAggregateKind::Bag,
                "SET" => ExpressAggregateKind::Set,
                _ => ExpressAggregateKind::List,
            };
            aggregates.push(evaluator.make(ExpressData::Aggregate { items, kind, lower: 1 }).unwrap());
        }
        let result = evaluator.binary(case["operator"].as_str().unwrap(), &aggregates[0], &aggregates[1]);
        if let Some(code) = case["error"].as_str() {
            assert_eq!(result.unwrap_err().code, code, "{}", case["name"]);
        } else {
            let result = result.unwrap();
            assert_eq!(result.kind(), case["kind"].as_str().unwrap());
            assert_eq!(result.scalar_text().as_deref(), case["value"].as_str());
        }
    }
}

#[test]
fn native_constructor_typeof_closes_the_independent_schema_hierarchy() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("constructor-cases.json")).unwrap();
    for case in cases.as_array().unwrap() {
        let mut evaluator = ExpressEvaluator::new(ifc4x3_express_program(), ExpressControl::default());
        let argument = if case["constructor"] == "IFCCARTESIANPOINT" {
            let zero = evaluator.make(ExpressData::Real(0.0)).unwrap();
            evaluator.make(ExpressData::Aggregate { items: vec![zero.clone(), zero.clone(), zero], kind: ExpressAggregateKind::List, lower: 1 }).unwrap()
        } else {
            evaluator.make(ExpressData::Real(1.0)).unwrap()
        };
        let value = evaluator.call(case["constructor"].as_str().unwrap(), vec![argument], &mut ExpressScope::default()).unwrap();
        let result = evaluator.call("TYPEOF", vec![value], &mut ExpressScope::default()).unwrap();
        let actual: std::collections::BTreeSet<_> = result.items().iter().filter_map(|value| value.scalar_text()).filter(|value| value.starts_with("IFC4X3_ADD2.")).map(|value| value.into_owned()).collect();
        let expected: std::collections::BTreeSet<_> = case["schemaTypes"].as_array().unwrap().iter().map(|value| value.as_str().unwrap().to_string()).collect();
        assert_eq!(result.items().iter().filter_map(|value| value.scalar_text()).filter(|value| value.starts_with("IFC4X3_ADD2.")).count(), actual.len());
        assert_eq!(actual, expected);
    }
}

#[test]
fn native_released_colour_selects_enforce_max_one_colour() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("typeof-select-cases.json")).unwrap();
    for case in cases["models"].as_array().unwrap() {
        let graph = graph_body(case["body"].as_str().unwrap());
        let report = validate_ifc4x3_express(&graph, ExpressControl::default());
        assert!(report.complete, "{:?}", report.diagnostics);
        assert_eq!(report.valid, case["valid"].as_bool().unwrap());
        let result = report.results.iter().find(|row| row.entity_id == Some(3) && row.rule == "MAXONECOLOUR").unwrap();
        assert_eq!(result.result, if case["valid"] == true { ExpressRuleOutcome::True } else { ExpressRuleOutcome::False });
    }
}

#[test]
fn native_prepared_computed_attributes_preserve_source_and_physical_boundary() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("models.json")).unwrap();
    let body = fixture["body"].as_array().unwrap().iter().map(|row| row.as_str().unwrap()).collect::<Vec<_>>().join("\n");
    let graph = graph_body(&body);
    let saved = graph.clone();
    let mut context = prepare_ifc4x3_express(&graph, ExpressControl::default()).unwrap();
    let weights = context.attribute_by_id(13, "WEIGHTS").unwrap();
    assert_eq!(weights.aggregate_kind(), ExpressAggregateKind::Array);
    assert_eq!(weights.lower(), 0);
    assert_eq!(weights.items()[0].scalar_text().as_deref(), Some("-1"));
    assert_eq!(graph, saved);
    let report = validate_ifc4x3_express(&graph, ExpressControl::default());
    assert_eq!(report.physical_conformance, ExpressPhysicalConformance::NotAssessed);
}

#[test]
fn native_integer_limit_cannot_publish_semantic_validity() {
    static LIMIT_PROGRAM: ExpressProgram = ExpressProgram {
        schema: "IFC4X3_ADD2",
        functions: &[],
        global_rules: &[],
        rules: &[ExpressRule {
            owner: "IFCDIRECTION",
            label: "CHECKED_LIMIT",
            expression: &ExpressExpression { kind: ExpressExpressionKind::Integer, offset: 17, text: Some("170141183460469231731687303715884105728"), operator: None, name: None, operands: &[], entries: &[] },
        }],
        derived: &[],
        unique: &[],
        bounds: &[],
        inverse: &[],
    };
    let report = validate_ifc4x3_express_with_program(&graph_body("#1=IFCDIRECTION((1.,0.));"), &LIMIT_PROGRAM, ExpressControl::default());
    assert!(!report.complete);
    assert!(!report.valid);
    assert!(report.results.iter().any(|row| row.result == ExpressRuleOutcome::Incomplete));
    assert!(report.diagnostics.iter().any(|row| row.code == "express.integer-overflow"));
}

#[test]
fn native_complete_official_corpus_preserves_step_and_expected_schema_outcomes() {
    let manifest: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/official/cases.json")).unwrap();
    let mut total = 0usize;
    assert_eq!(OFFICIAL_CASES.len(), 46);
    for (name, source, count) in OFFICIAL_CASES {
        let expected = manifest["cases"].as_array().unwrap().iter().find(|row| row["file"].as_str() == Some(*name)).unwrap();
        let graph = crate::io::parse_ifc4x3_step(source).unwrap();
        assert_eq!(graph.entities.len(), *count, "{name}");
        let exported = crate::io::write_ifc4x3_step(&graph).unwrap();
        assert_eq!(crate::io::parse_ifc4x3_step(&exported).unwrap(), graph, "{name}");
        total += graph.entities.len();
        let report = validate_ifc4x3_express(&graph, ExpressControl { maximum_work: 100_000_000, maximum_nodes: 20_000_000, ..Default::default() });
        if expected["diagnostics"].as_array().unwrap().is_empty() {
            assert!(report.complete, "{name}: {:?}", report.diagnostics);
            assert!(report.valid, "{name}: {:?}", report.diagnostics);
            assert!(report.diagnostics.is_empty(), "{name}: {:?}", report.diagnostics);
        } else {
            assert!(!report.complete, "{name}");
            assert!(!report.valid, "{name}");
            assert!(report.diagnostics.iter().any(|row| row.entity_id == Some(52) && row.attribute.as_deref() == Some("Identification") && row.code == "express.domain"), "{name}: {:?}", report.diagnostics);
        }
    }
    assert_eq!(total, 8172);
}

#[test]
fn native_structural_uniqueness_matches_shared_domain_vectors() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("domain-cases.json")).unwrap();
    for case in cases.as_array().unwrap() {
        let graph = graph_body(case["body"].as_str().unwrap());
        let result = prepare_ifc4x3_express(&graph, ExpressControl::default());
        if case["codes"].as_array().unwrap().is_empty() {
            assert!(result.is_ok(), "{}", case["name"]);
        } else {
            let failure = match result {
                Err(failure) => failure,
                Ok(_) => panic!("{} unexpectedly admitted", case["name"]),
            };
            assert_eq!(failure.code, "domain.aggregate-unique");
            assert!(failure.entity_id.is_some());
        }
    }
}

#[test]
fn native_inherited_derived_bindings_match_full_independent_population_outcomes() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("derived-binding-cases.json")).unwrap();
    for case in cases.as_array().unwrap() {
        let graph = graph_body(case["body"].as_str().unwrap());
        assert!(prepare_ifc4x3_express(&graph, ExpressControl::default()).is_ok());
        let report = validate_ifc4x3_express(&graph, ExpressControl::default());
        assert!(report.complete, "{:?}", report.diagnostics);
        let mut actual: Vec<_> = report.results.iter().filter(|row| row.rule == "NORTH2D" && row.result == ExpressRuleOutcome::False).map(|row| row.entity_id.unwrap().to_string()).collect();
        actual.sort();
        let mut expected: Vec<_> = case["northFailures"].as_array().unwrap().iter().map(|value| value.as_str().unwrap().to_string()).collect();
        expected.sort();
        assert_eq!(actual, expected);
        assert_eq!(report.valid, expected.is_empty());
        let mut context = prepare_ifc4x3_express(&graph, ExpressControl::default()).unwrap();
        let value = context.entity_by_id(case["subContextId"].as_str().unwrap().parse().unwrap()).unwrap();
        let mut scope = context.scoped(&value).unwrap();
        assert!(!scope.values.contains_key("TRUENORTH"));
        for (_, expression) in BINDING_READS {
            let result = context.evaluate(expression, &mut scope).unwrap();
            assert_eq!(result.entity_id(), Some(3));
        }
        let local = context.make(ExpressData::Integer(42)).unwrap();
        scope.values.insert("TRUENORTH".into(), local);
        assert_eq!(context.evaluate(&BINDING_READS[0].1, &mut scope).unwrap().integer(), Some(42));
        assert_eq!(context.evaluate(&BINDING_READS[1].1, &mut scope).unwrap().entity_id(), Some(3));
        let unknown = context.make(ExpressData::Unknown).unwrap();
        scope.values.insert("TRUENORTH".into(), unknown);
        assert!(context.evaluate(&BINDING_READS[0].1, &mut scope).unwrap().missing());
        assert_eq!(context.evaluate(&BINDING_QUERY, &mut scope).unwrap().integer(), Some(1));
    }
    for (name, arguments, expected) in SHADOW_CASES {
        let mut context = ExpressEvaluator::new(&SHADOW_PROGRAM, ExpressControl::default());
        let arguments = arguments.iter().map(|value| context.make(ExpressData::Integer(*value)).unwrap()).collect();
        assert_eq!(context.call(name, arguments, &mut ExpressScope::default()).unwrap().integer(), Some(*expected));
    }
}

#[test]
fn native_foreign_self_and_constructor_payloads_refuse_before_arena_access() {
    let graph = graph_body("#1=IFCCARTESIANPOINT((0.,0.,0.));");
    let mut owner = prepare_ifc4x3_express(&graph, ExpressControl::default()).unwrap();
    let foreign = owner.entity_by_id(1).unwrap();
    let foreign_aggregate = owner.make(ExpressData::Aggregate { items: vec![foreign.clone()], kind: ExpressAggregateKind::List, lower: 1 }).unwrap();
    let zero = owner.make(ExpressData::Real(0.0)).unwrap();
    let portable = owner.make(ExpressData::Aggregate { items: vec![zero.clone(), zero.clone(), zero], kind: ExpressAggregateKind::List, lower: 1 }).unwrap();
    for coincident in [false, true] {
        let mut context = if coincident { prepare_ifc4x3_express(&graph, ExpressControl::default()).unwrap() } else { ExpressEvaluator::new(ifc4x3_express_program(), ExpressControl::default()) };
        let mut scope = ExpressScope::default();
        scope.values.insert("SELF".into(), foreign.clone());
        assert_eq!(context.evaluate(&BINDING_READS[0].1, &mut scope).unwrap_err().code, "entity-domain");
        let self_expression = ExpressExpression { kind: ExpressExpressionKind::Identifier, offset: 0, text: None, operator: None, name: Some("SELF"), operands: &[], entries: &[] };
        assert_eq!(context.evaluate(&self_expression, &mut scope).unwrap_err().code, "entity-domain");
        assert_eq!(context.attribute(&foreign, "COORDINATES").unwrap_err().code, "entity-domain");
        assert_eq!(context.make(foreign.data().clone()).unwrap_err().code, "entity-domain");
        let missing = context.make(ExpressData::Unknown).unwrap();
        assert_eq!(context.equal(&foreign, &missing, false).unwrap_err().code, "entity-domain");
        assert_eq!(context.attribute(&foreign_aggregate, "COORDINATES").unwrap_err().code, "entity-domain");
        assert_eq!(context.call("TYPEOF", vec![foreign.clone()], &mut ExpressScope::default()).unwrap_err().code, "entity-domain");
        assert_eq!(context.call("IFCDIRECTION", vec![foreign_aggregate.clone()], &mut ExpressScope::default()).unwrap_err().code, "entity-domain");
        assert_eq!(context.make(ExpressData::Aggregate { items: vec![foreign.clone()], kind: ExpressAggregateKind::List, lower: 1 }).unwrap_err().code, "entity-domain");
        if coincident {
            assert_eq!(context.attribute_by_id(1, "COORDINATES").unwrap().items().len(), 3);
        }
        let constructed = context.call("IFCDIRECTION", vec![portable.clone()], &mut ExpressScope::default()).unwrap();
        assert_eq!(context.attribute(&constructed, "DIRECTIONRATIOS").unwrap().items().len(), 3);
    }
}
