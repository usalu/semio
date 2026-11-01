//! 🧪️ Laws of the canonical program, the declaration metadata and the owner indices: each equals its neutral JSON under third-party `serde_json`, and hostile resources are refused with closed causes.
use super::ast::{ExpressExpression, ExpressRule, ExpressStatement, ExpressVariable};
use super::*;
use serde_json::{json, Map, Value};
use std::path::PathBuf;

fn neutral(relative: &str) -> Value {
    let path: PathBuf = [env!("CARGO_MANIFEST_DIR"), relative].iter().collect();
    serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
}

fn prune(value: Value) -> Value {
    match value {
        Value::Array(items) => Value::Array(items.into_iter().map(prune).collect()),
        Value::Object(members) => Value::Object(members.into_iter().filter(|(_, item)| !item.is_null() && item.as_array().is_none_or(|items| !items.is_empty())).map(|(key, item)| (key, prune(item))).collect()),
        other => other,
    }
}

fn texts(items: &[&str]) -> Value {
    json!(items)
}

fn expression(node: &ExpressExpression) -> Value {
    json!({
        "kind": format!("{:?}", node.kind).to_lowercase(),
        "offset": node.offset,
        "text": node.text,
        "operator": node.operator,
        "name": node.name,
        "operands": node.operands.iter().map(expression).collect::<Vec<_>>(),
        "entries": node.entries.iter().map(|entry| json!({"value": expression(entry.value), "repeat": entry.repeat.map(expression)})).collect::<Vec<_>>(),
    })
}

fn statement(node: &ExpressStatement) -> Value {
    json!({
        "kind": format!("{:?}", node.kind).to_lowercase(),
        "offset": node.offset,
        "name": node.name,
        "expressions": node.expressions.iter().map(expression).collect::<Vec<_>>(),
        "body": node.body.iter().map(statement).collect::<Vec<_>>(),
        "otherwise": node.otherwise.iter().map(statement).collect::<Vec<_>>(),
        "cases": node.cases.iter().map(|case| json!({"labels": case.labels.iter().map(expression).collect::<Vec<_>>(), "body": case.body.iter().map(statement).collect::<Vec<_>>()})).collect::<Vec<_>>(),
        "while": node.while_condition.map(expression),
        "until": node.until_condition.map(expression),
    })
}

fn variable(node: &ExpressVariable) -> Value {
    json!({"names": texts(node.names), "type": node.type_source, "initial": node.initial.map(expression)})
}

fn rule(node: &ExpressRule) -> Value {
    json!({"owner": node.owner, "label": node.label, "expression": expression(node.expression)})
}

fn program_json(program: &ast::ExpressProgram) -> Value {
    json!({
        "schema": program.schema,
        "functions": program.functions.iter().map(|node| json!({"name": node.name, "parameters": node.parameters.iter().map(variable).collect::<Vec<_>>(), "resultType": node.result_type, "locals": node.locals.iter().map(variable).collect::<Vec<_>>(), "body": node.body.iter().map(statement).collect::<Vec<_>>()})).collect::<Vec<_>>(),
        "globalRules": program.global_rules.iter().map(|node| json!({"name": node.name, "populations": texts(node.populations), "locals": node.locals.iter().map(variable).collect::<Vec<_>>(), "body": node.body.iter().map(statement).collect::<Vec<_>>(), "rules": node.rules.iter().map(rule).collect::<Vec<_>>()})).collect::<Vec<_>>(),
        "rules": program.rules.iter().map(rule).collect::<Vec<_>>(),
        "derived": program.derived.iter().map(rule).collect::<Vec<_>>(),
        "unique": program.unique.iter().map(rule).collect::<Vec<_>>(),
        "bounds": program.bounds.iter().map(rule).collect::<Vec<_>>(),
        "inverse": program.inverse.iter().map(|node| json!({"owner": node.owner, "name": node.name, "type": node.entity_type, "attribute": node.attribute, "aggregate": node.aggregate, "lower": node.lower.map(expression), "upper": node.upper.map(expression)})).collect::<Vec<_>>(),
    })
}

fn type_expression(node: &metadata::StaticTypeExpression) -> Value {
    if node.kind == "named" {
        return json!({"kind": node.kind, "name": node.name, "width": node.width, "fixed": node.fixed});
    }
    json!({
        "kind": node.kind, "aggregate": node.aggregate, "lower": node.lower, "upper": node.upper, "lowerExpression": node.lower_expression, "upperExpression": node.upper_expression,
        "unique": node.unique, "optional": node.optional, "item": node.item.map(type_expression),
    })
}

fn attribute(node: &metadata::StaticAttribute) -> Value {
    json!({"name": node.name, "typeExpr": type_expression(&node.type_expr), "optional": node.optional, "derived": node.derived, "declaredBy": node.declared_by})
}

#[test]
fn decoded_program_equals_the_neutral_program_json() {
    let expected = prune(neutral("../../program.json"));
    assert_eq!(prune(program_json(ifc4x3_express_program())), expected);
}

#[test]
fn projected_metadata_equals_the_neutral_released_catalog() {
    let released = neutral("../../../../📚️catalog/🔣️.json");
    let mut entities = Map::new();
    for entity in metadata::entities() {
        entities.insert(
            entity.key.to_owned(),
            json!({
                "name": entity.name, "abstractEntity": entity.abstract_entity, "supertype": entity.supertype, "supertypeExpression": entity.supertype_expression,
                "attributes": entity.attributes.iter().map(attribute).collect::<Vec<_>>(),
                "allAttributes": entity.all_attributes.iter().map(attribute).collect::<Vec<_>>(),
                "derived": entity.derived.iter().map(|node| json!({"name": node.name, "typeExpr": type_expression(&node.type_expr), "redeclares": node.redeclares, "expression": node.expression})).collect::<Vec<_>>(),
                "inverse": texts(entity.inverse), "whereRules": texts(entity.where_rules), "uniqueRules": texts(entity.unique_rules),
            }),
        );
    }
    let mut types = Map::new();
    for declaration in metadata::types() {
        types.insert(declaration.key.to_owned(), json!({"name": declaration.name, "kind": declaration.kind, "members": texts(declaration.members), "typeExpr": declaration.type_expr.map(type_expression), "whereRules": texts(declaration.where_rules)}));
    }
    assert_eq!(prune(Value::Object(entities)), prune(released["entities"].clone()));
    assert_eq!(prune(Value::Object(types)), prune(released["types"].clone()));
}

#[test]
fn hostile_program_resources_are_refused() {
    let resource = include_bytes!("../program.ifcj");
    assert!(decode_program(resource).is_ok());
    assert!(matches!(decode_program(&resource[..resource.len() / 2]), Err(JsonPackError::Inflate(_) | JsonPackError::Truncated)));
    assert!(decode_program(&[]).is_err());
    let foreign = include_bytes!("../../../../../../../../📚️catalog/🧾️json-pack/🧪️tests/🧫️fixtures/minimal.ifcj");
    assert_eq!(decode_program(foreign).unwrap_err(), JsonPackError::Shape("member"));
}
