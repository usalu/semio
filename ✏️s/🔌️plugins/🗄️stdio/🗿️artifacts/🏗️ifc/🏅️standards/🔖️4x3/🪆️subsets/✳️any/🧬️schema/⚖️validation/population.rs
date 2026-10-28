//! 🌐 Controlled IFC population construction and complete retained EXPRESS rule execution.
use super::ast::*;
use super::evaluator::*;
use crate::catalog::{ifc_is_subtype, IfcTypeExpression};
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::{IfcEntity, IfcSnapshot, IfcValue};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct ExpressDiagnostic {
    pub code: String,
    pub message: String,
    pub owner: Option<String>,
    pub rule: Option<String>,
    pub entity_id: Option<u64>,
    pub offset: Option<usize>,
    pub attribute: Option<String>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExpressRuleKind {
    Where,
    Unique,
    Inverse,
    Bound,
    Global,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExpressRuleOutcome {
    True,
    False,
    Unknown,
    Incomplete,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExpressPhysicalConformance {
    NotAssessed,
}
#[derive(Clone, Debug)]
pub struct ExpressRuleResult {
    pub owner: String,
    pub rule: String,
    pub entity_id: Option<u64>,
    pub result: ExpressRuleOutcome,
    pub kind: ExpressRuleKind,
}
#[derive(Clone, Debug)]
pub struct IfcExpressValidation {
    pub complete: bool,
    pub valid: bool,
    pub diagnostics: Vec<ExpressDiagnostic>,
    pub results: Vec<ExpressRuleResult>,
    pub work: u64,
    pub nodes: usize,
    pub physical_conformance: ExpressPhysicalConformance,
}
fn outcome(value: &ExpressValue) -> ExpressRuleOutcome {
    match value.truth() {
        ExpressLogical::True => ExpressRuleOutcome::True,
        ExpressLogical::False => ExpressRuleOutcome::False,
        _ => ExpressRuleOutcome::Unknown,
    }
}
fn entity_failure(mut failure: ExpressEvaluationError, id: u64, owner: &str, attribute: Option<&str>) -> ExpressEvaluationError {
    failure.entity_id = Some(id);
    failure.owner = Some(owner.into());
    failure.attribute = attribute.map(str::to_string);
    failure
}
impl ExpressEvaluator<'_> {
    fn canonical_identifier(&mut self, source: &str) -> ExpressResult<bool> {
        if source.is_empty() {
            return Ok(false);
        }
        for (index, item) in source.bytes().enumerate() {
            self.tick()?;
            if if index == 0 { !item.is_ascii_uppercase() } else { !item.is_ascii_uppercase() && !item.is_ascii_digit() && item != b'_' } {
                return Ok(false);
            }
        }
        Ok(true)
    }
    fn canonical_binary(&mut self, source: &str) -> ExpressResult<bool> {
        if source.is_empty() {
            return Ok(false);
        }
        for (index, item) in source.bytes().enumerate() {
            self.tick()?;
            if index == 0 && !matches!(item, b'0'..=b'3') || index > 0 && !matches!(item,b'0'..=b'9'|b'A'..=b'F') {
                return Ok(false);
            }
        }
        if source.len() == 1 {
            return Ok(source == "0");
        }
        Ok(u8::from_str_radix(&source[1..2], 16).unwrap_or(255) < 1u8 << (4 - (source.as_bytes()[0] - b'0')))
    }
    fn source_equal(&mut self, left: &IfcValue, right: &IfcValue, depth: usize) -> ExpressResult<bool> {
        self.tick()?;
        if depth >= self.control.maximum_depth.min(256) {
            return Err(error("depth", "Structural equality depth exceeded"));
        }
        if let IfcValue::TypedValue(typed) = left {
            if typed.items.len() == 1 {
                return self.source_equal(&typed.items[0], right, depth + 1);
            }
        }
        if let IfcValue::TypedValue(typed) = right {
            if typed.items.len() == 1 {
                return self.source_equal(left, &typed.items[0], depth + 1);
            }
        }
        Ok(match (left, right) {
            (IfcValue::Unset, IfcValue::Unset) | (IfcValue::Derived, IfcValue::Derived) => false,
            (IfcValue::Integer(a), IfcValue::Integer(b)) => a == b,
            (IfcValue::Real(a), IfcValue::Real(b)) => a == b,
            (IfcValue::Integer(a), IfcValue::Real(b)) => Self::numeric_order(&ExpressData::Integer(*a as i128), &ExpressData::Real(*b))? == std::cmp::Ordering::Equal,
            (IfcValue::Real(a), IfcValue::Integer(b)) => Self::numeric_order(&ExpressData::Real(*a), &ExpressData::Integer(*b as i128))? == std::cmp::Ordering::Equal,
            (IfcValue::Reference(a), IfcValue::Reference(b)) => a == b,
            (IfcValue::String(a), IfcValue::String(b)) | (IfcValue::Binary(a), IfcValue::Binary(b)) | (IfcValue::Enum(a), IfcValue::Enum(b)) => {
                let mut a = a.chars();
                let mut b = b.chars();
                loop {
                    self.tick()?;
                    match (a.next(), b.next()) {
                        (None, None) => break true,
                        (Some(a), Some(b)) if a == b => {}
                        _ => break false,
                    }
                }
            }
            (IfcValue::Aggregate(a), IfcValue::Aggregate(b)) => {
                if a.len() != b.len() {
                    false
                } else {
                    let mut equal = true;
                    for (a, b) in a.iter().zip(b) {
                        if !self.source_equal(a, b, depth + 1)? {
                            equal = false;
                            break;
                        }
                    }
                    equal
                }
            }
            (IfcValue::TypedValue(a), IfcValue::TypedValue(b)) => {
                if self.text_order(&a.name, &b.name)? != std::cmp::Ordering::Equal || a.items.len() != b.items.len() {
                    false
                } else {
                    let mut equal = true;
                    for (a, b) in a.items.iter().zip(&b.items) {
                        if !self.source_equal(a, b, depth + 1)? {
                            equal = false;
                            break;
                        }
                    }
                    equal
                }
            }
            _ => false,
        })
    }
    fn member_accepts(&mut self, member: &str, actual: &str, depth: usize) -> ExpressResult<bool> {
        self.tick()?;
        if depth >= self.control.maximum_depth.min(256) {
            return Err(error("depth", "SELECT domain recursion exceeded"));
        }
        if member == actual || ifc_is_subtype(actual, member) {
            return Ok(true);
        }
        let catalog = self.catalog;
        if let Some(declaration) = catalog.types.get(member).filter(|declaration| declaration.kind == "select") {
            for member in &declaration.members {
                if self.member_accepts(member, actual, depth + 1)? {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
    fn domain_accepts(&mut self, value: &IfcValue, expression: &IfcTypeExpression, depth: usize) -> ExpressResult<bool> {
        self.tick()?;
        if depth >= self.control.maximum_depth.min(256) {
            return Err(error("depth", "Structural domain depth exceeded"));
        }
        let catalog = self.catalog;
        if expression.kind == "aggregate" {
            let IfcValue::Aggregate(items) = value else { return Ok(false) };
            let size = items.len() as i128;
            let bounded = if expression.aggregate.as_deref() == Some("ARRAY") {
                match (expression.lower, expression.upper) {
                    (Some(lower), Some(upper)) => size == upper as i128 - lower as i128 + 1,
                    _ => true,
                }
            } else {
                expression.lower.is_none_or(|lower| size >= lower as i128) && expression.upper.is_none_or(|upper| size <= upper as i128)
            };
            if !bounded {
                return Err(error("domain.aggregate-bounds", "Aggregate cardinality violates its declared structural bounds"));
            }
            for (index, item) in items.iter().enumerate() {
                self.tick()?;
                if expression.unique || expression.aggregate.as_deref() == Some("SET") {
                    for other in &items[..index] {
                        if self.source_equal(item, other, depth + 1)? {
                            return Err(error("domain.aggregate-unique", "Aggregate values must be instance unique"));
                        }
                    }
                }
                if matches!(item, IfcValue::Unset) && expression.optional {
                    continue;
                }
                if let Some(inner) = expression.item.as_deref() {
                    if !self.domain_accepts(item, inner, depth + 1)? {
                        return Ok(false);
                    }
                } else {
                    return Ok(false);
                }
            }
            return Ok(true);
        }
        let name = expression.name.as_deref().unwrap_or("");
        if catalog.entities.contains_key(name) {
            return Ok(if let IfcValue::Reference(id) = value { self.identities.get(id).is_some_and(|index| self.entities[*index].types.iter().any(|owner| owner == name)) } else { false });
        }
        if let Some(declaration) = catalog.types.get(name) {
            if declaration.kind == "enum" {
                return if let IfcValue::Enum(literal) = value {
                    let literal = self.text(literal)?.to_ascii_uppercase();
                    Ok(declaration.members.contains(&literal))
                } else {
                    Ok(false)
                };
            }
            if declaration.kind == "select" {
                return match value {
                    IfcValue::Reference(id) => {
                        let Some(index) = self.identities.get(id) else { return Ok(false) };
                        let actual = self.entities[*index].types.first().cloned().unwrap_or_default();
                        for member in &declaration.members {
                            if self.member_accepts(member, &actual, depth + 1)? {
                                return Ok(true);
                            }
                        }
                        Ok(false)
                    }
                    IfcValue::TypedValue(typed) if typed.items.len() == 1 => {
                        let actual = self.text(&typed.name)?.to_ascii_uppercase();
                        let mut allowed = false;
                        for member in &declaration.members {
                            if self.member_accepts(member, &actual, depth + 1)? {
                                allowed = true;
                                break;
                            }
                        }
                        if !allowed {
                            return Ok(false);
                        }
                        let named = IfcTypeExpression { kind: "named".into(), name: Some(actual), ..Default::default() };
                        self.domain_accepts(&typed.items[0], &named, depth + 1)
                    }
                    _ => Ok(false),
                };
            }
            if let Some(inner) = &declaration.type_expr {
                return match value {
                    IfcValue::TypedValue(typed) if typed.name.eq_ignore_ascii_case(name) => {
                        if typed.items.len() == 1 {
                            self.domain_accepts(&typed.items[0], inner, depth + 1)
                        } else {
                            Ok(false)
                        }
                    }
                    _ => self.domain_accepts(value, inner, depth + 1),
                };
            }
        }
        Ok(match (name, value) {
            ("STRING", IfcValue::String(text)) => {
                let mut count = 0i128;
                for _ in text.chars() {
                    self.tick()?;
                    count += 1;
                }
                expression.width.is_none_or(|width| if expression.fixed { count == width as i128 } else { count <= width as i128 })
            }
            ("INTEGER" | "REAL" | "NUMBER", IfcValue::Integer(_)) => true,
            ("REAL" | "NUMBER", IfcValue::Real(value)) => value.is_finite(),
            ("BINARY", IfcValue::Binary(_)) => true,
            ("BOOLEAN", IfcValue::Enum(value)) => ["T", "F"].contains(&self.text(value)?.to_ascii_uppercase().as_str()),
            ("LOGICAL", IfcValue::Enum(value)) => ["T", "F", "U"].contains(&self.text(value)?.to_ascii_uppercase().as_str()),
            _ => false,
        })
    }
    fn population_types(&mut self, entity: &IfcEntity) -> ExpressResult<Vec<String>> {
        let catalog = self.catalog;
        let mut names = BTreeSet::new();
        for name in std::iter::once(entity.name.as_str()).chain(entity.complex.iter().map(|part| part.name.as_str())) {
            if !self.canonical_identifier(name)? {
                return Err(error("admission", "Entity names require canonical EXPRESS identifiers"));
            }
            let name = self.text(name)?.to_ascii_uppercase();
            if !catalog.entities.contains_key(&name) || !names.insert(name) {
                return Err(error("complex", "Unknown or duplicate entity constituent"));
            }
        }
        let mut leaves = Vec::new();
        for name in &names {
            let mut parent = false;
            for other in &names {
                self.tick()?;
                if name != other && ifc_is_subtype(other, name) {
                    parent = true;
                    break;
                }
            }
            if !parent {
                leaves.push(name.clone());
            }
        }
        if leaves.len() != 1 {
            return Err(error("complex", "Entity constituents require one concrete subtype chain"));
        }
        let mut chain = Vec::new();
        let mut current = leaves.pop();
        while let Some(name) = current {
            self.tick()?;
            if chain.contains(&name) {
                return Err(error("domain", "Cyclic entity inheritance"));
            }
            chain.push(name.clone());
            current = catalog.entities.get(&name).and_then(|declaration| declaration.supertype.clone());
        }
        if !entity.complex.is_empty() && (chain.len() != names.len() || chain.iter().any(|name| !names.contains(name))) {
            return Err(error("complex", "Complex constituents require every inherited entity owner"));
        }
        if catalog.entities[&chain[0]].abstract_entity {
            return Err(error("domain", "Abstract entity population"));
        }
        Ok(chain)
    }
    pub(crate) fn type_names(&mut self, name: &str) -> ExpressResult<Vec<String>> {
        let catalog = self.catalog;

        let mut result = Vec::new();
        let mut current = name.to_ascii_uppercase();
        loop {
            self.tick()?;
            if result.contains(&current) {
                return Err(error("type-cycle", "Cyclic named type declarations"));
            }
            result.push(current.clone());
            let next = catalog.types.get(&current).and_then(|declaration| declaration.type_expr.as_ref()).filter(|expression| expression.kind == "named").and_then(|expression| expression.name.as_ref());
            if let Some(next) = next {
                current = next.clone();
            } else {
                break;
            }
        }
        Ok(result)
    }
    fn convert(&mut self, value: &IfcValue, type_expression: Option<&IfcTypeExpression>, depth: usize) -> ExpressResult<ExpressValue> {
        let catalog = self.catalog;

        self.tick()?;
        if depth >= self.control.maximum_depth.min(256) {
            return Err(error("depth", "Graph value conversion depth exceeded"));
        }
        let mut converted = match value {
            IfcValue::Unset | IfcValue::Derived => return self.unknown(),
            IfcValue::Integer(value) => self.integer_value(*value as i128)?,
            IfcValue::Real(value) => self.real(*value)?,
            IfcValue::String(value) => {
                let text = self.text(value)?;
                self.make(ExpressData::String(text))?
            }
            IfcValue::Binary(value) => {
                if value.is_empty() {
                    return Err(error("admission", "Empty BINARY content"));
                }
                let mut text = String::new();
                for (index, item) in value.bytes().enumerate() {
                    self.tick()?;
                    if index == 0 && !matches!(item, b'0'..=b'3') || index > 0 && !matches!(item,b'0'..=b'9'|b'A'..=b'F') {
                        return Err(error("admission", "Noncanonical BINARY content"));
                    }
                    text.push(item as char);
                }
                if value.len() == 1 && value != "0" || value.len() > 1 && value.as_bytes()[1].to_ascii_uppercase().is_ascii_hexdigit() && u8::from_str_radix(&value[1..2], 16).unwrap_or(255) >= 1u8 << (4 - (value.as_bytes()[0] - b'0')) {
                    return Err(error("admission", "Invalid BINARY padding"));
                }
                self.make(ExpressData::Binary(text))?
            }
            IfcValue::Enum(value) => {
                if !self.canonical_identifier(value)? {
                    return Err(error("admission", "Enumeration requires a canonical EXPRESS identifier"));
                }
                let text = self.text(value)?.to_ascii_uppercase();
                if ["T", "F", "U"].contains(&text.as_str()) {
                    self.logical(match text.as_str() {
                        "T" => ExpressLogical::True,
                        "F" => ExpressLogical::False,
                        _ => ExpressLogical::Unknown,
                    })?
                } else {
                    self.make(ExpressData::Enum(text))?
                }
            }
            IfcValue::Reference(id) => {
                let index = *self.identities.get(id).ok_or_else(|| error("admission", format!("Dangling entity reference #{id}")))?;
                let mut result = self.make_entity(index)?;
                result.types = Arc::new(self.entities[index].types.clone());
                return Ok(result);
            }
            IfcValue::TypedValue(typed) => {
                if !self.canonical_identifier(&typed.name)? {
                    return Err(error("admission", "Typed values require canonical EXPRESS identifiers"));
                }
                if typed.items.len() != 1 {
                    return Err(error("admission", "A typed value requires one payload"));
                }
                let name = self.text(&typed.name)?.to_ascii_uppercase();
                let expression = IfcTypeExpression { kind: "named".into(), name: Some(name), ..Default::default() };
                return self.convert(&typed.items[0], Some(&expression), depth + 1);
            }
            IfcValue::Aggregate(items) => {
                let mut expression = type_expression;
                let mut seen = BTreeSet::new();
                while let Some(current) = expression.filter(|expression| expression.kind == "named") {
                    self.tick()?;
                    let name = current.name.as_deref().unwrap_or("");
                    if !seen.insert(name) {
                        return Err(error("type-cycle", "Cyclic aggregate alias"));
                    }
                    expression = catalog.types.get(name).and_then(|declaration| declaration.type_expr.as_ref());
                }
                let kind = match expression.and_then(|expression| expression.aggregate.as_deref()) {
                    Some("SET") => ExpressAggregateKind::Set,
                    Some("BAG") => ExpressAggregateKind::Bag,
                    Some("ARRAY") => ExpressAggregateKind::Array,
                    _ => ExpressAggregateKind::List,
                };
                let lower = if kind == ExpressAggregateKind::Array { expression.and_then(|expression| expression.lower).unwrap_or(1) as i128 } else { 1 };
                let mut converted = Vec::new();
                for item in items {
                    self.tick()?;
                    let item = self.convert(item, expression.and_then(|expression| expression.item.as_deref()), depth + 1)?;
                    self.push_value(&mut converted, item)?;
                }
                self.aggregate(converted, kind, lower)?
            }
        };
        let mut names = if let Some(expression) = type_expression.filter(|expression| expression.kind == "named") { self.type_names(expression.name.as_deref().unwrap_or(""))? } else { Vec::new() };
        for name in Self::primitive_types(&converted) {
            self.tick()?;
            if !names.contains(&name) {
                names.push(name);
            }
        }
        converted.types = Arc::new(names);
        Ok(converted)
    }
    pub fn initialize(&mut self, snapshot: &IfcSnapshot) -> ExpressResult<()> {
        let catalog = self.catalog;

        self.tick()?;
        if !self.entities.is_empty() || !self.identities.is_empty() {
            return Err(error("context", "Population initialization requires a fresh evaluator"));
        }
        if snapshot.schema != "stdio.ifc" {
            return Err(error("admission", "Unexpected IFC graph identity"));
        }
        let schema = &snapshot.header.file_schema;
        if !matches!(schema.as_slice(),[IfcValue::Aggregate(items)]if matches!(items.as_slice(),[IfcValue::String(name)]if name=="IFC4X3_ADD2")) {
            return Err(error("admission", "FILE_SCHEMA must identify IFC4X3_ADD2"));
        }
        for lane in [&snapshot.header.file_description, &snapshot.header.file_name, &snapshot.header.file_schema] {
            for value in lane {
                self.admit_header(value, 0)?;
            }
        }
        for entity in &snapshot.entities {
            self.tick()?;
            if entity.id == 0 || self.identities.contains_key(&entity.id) {
                return Err(error("admission", "Entity identities must be unique positive u64"));
            }
            let types = self.population_types(entity).map_err(|failure| entity_failure(failure, entity.id, &entity.name, None))?;
            if self.entities.len() >= self.control.maximum_nodes {
                return Err(error("node-budget", "Population entity budget exceeded"));
            }
            self.charge_nodes(1)?;
            self.identities.insert(entity.id, self.entities.len());
            self.entities.push(ExpressEntity { identity: Some(entity.id), types, fields: BTreeMap::new() });
        }
        for entity in &snapshot.entities {
            self.tick()?;
            let index = self.identities[&entity.id];
            let mut parts = Vec::new();
            parts.push((self.text(&entity.name)?.to_ascii_uppercase(), entity.args.as_slice()));
            for part in &entity.complex {
                self.tick()?;
                parts.push((self.text(&part.name)?.to_ascii_uppercase(), part.args.as_slice()));
            }
            let leaf = if parts.len() == 1 {
                parts[0].0.clone()
            } else {
                let mut names = BTreeSet::new();
                for (name, _) in &parts {
                    self.tick()?;
                    if !catalog.entities.contains_key(name) || !names.insert(name.clone()) {
                        return Err(error("complex", "Unknown or duplicated complex constituent"));
                    }
                }
                let mut leaves = Vec::new();
                for (name, _) in &parts {
                    let mut parent = false;
                    for (other, _) in &parts {
                        self.tick()?;
                        if name != other && ifc_is_subtype(other, name) {
                            parent = true;
                            break;
                        }
                    }
                    if !parent {
                        leaves.push(name.clone());
                    }
                }
                if leaves.len() != 1 {
                    return Err(error("complex", "Complex constituents must form one subtype chain"));
                }
                let leaf = leaves.pop().unwrap();
                let mut current = Some(leaf.clone());
                let mut chain = Vec::new();
                while let Some(name) = current {
                    self.tick()?;
                    chain.push(name.clone());
                    current = catalog.entities.get(&name).and_then(|declaration| declaration.supertype.clone());
                }
                if chain.len() != names.len() || chain.iter().any(|name| !names.contains(name)) {
                    return Err(error("complex", "Complex constituents require every inherited owner"));
                }
                self.entities[index].types = chain;
                leaf
            };
            let declaration = catalog.entities.get(&leaf).ok_or_else(|| error("domain", "Missing concrete declaration"))?;
            if declaration.abstract_entity {
                return Err(error("domain", "Abstract entity population"));
            }
            let mut direct = BTreeMap::new();
            if parts.len() > 1 {
                for (name, args) in &parts {
                    let owner = &catalog.entities[name];
                    if owner.attributes.len() != args.len() {
                        return Err(error("complex", format!("Invalid direct argument count for {name}")));
                    }
                    for (attribute, value) in owner.attributes.iter().zip(args.iter()) {
                        self.tick()?;
                        direct.insert((name.clone(), attribute.name.to_ascii_uppercase()), value);
                    }
                }
            } else if declaration.all_attributes.len() != entity.args.len() {
                return Err(error("domain", format!("Invalid inherited argument count for {leaf}")));
            }
            for (position, attribute) in declaration.all_attributes.iter().enumerate() {
                self.tick()?;
                let unset = IfcValue::Unset;
                let value = if parts.len() == 1 {
                    &entity.args[position]
                } else if attribute.derived {
                    if let Some(original) = direct.get(&(attribute.declared_by.to_ascii_uppercase(), attribute.name.to_ascii_uppercase())) {
                        if !matches!(original, IfcValue::Derived) {
                            return Err(error("domain", format!("{}.{} requires a derived marker", leaf, attribute.name)));
                        }
                    }
                    &IfcValue::Derived
                } else {
                    direct.get(&(attribute.declared_by.to_ascii_uppercase(), attribute.name.to_ascii_uppercase())).copied().unwrap_or(&unset)
                };
                if attribute.derived {
                    if !matches!(value, IfcValue::Derived) {
                        return Err(entity_failure(error("domain", format!("{}.{} requires a derived marker", leaf, attribute.name)), entity.id, &leaf, Some(&attribute.name)));
                    }
                } else if matches!(value, IfcValue::Unset) {
                    if !attribute.optional {
                        return Err(entity_failure(error("domain", format!("{}.{} is required", leaf, attribute.name)), entity.id, &leaf, Some(&attribute.name)));
                    }
                } else if !self.domain_accepts(value, &attribute.type_expr, 0).map_err(|failure| entity_failure(failure, entity.id, &leaf, Some(&attribute.name)))? {
                    return Err(entity_failure(error("domain", format!("{}.{} violates its structural EXPRESS domain", leaf, attribute.name)), entity.id, &leaf, Some(&attribute.name)));
                }
                let converted = self.convert(value, Some(&attribute.type_expr), 0).map_err(|failure| entity_failure(failure, entity.id, &leaf, Some(&attribute.name)))?;
                self.entities[index].fields.insert(attribute.name.to_ascii_uppercase(), converted);
            }
        }
        for index in 0..self.entities.len() {
            let fields = self.fields_owned(index)?;
            for (attribute, value) in fields {
                self.index_incoming(&value, index, &attribute, 0)?;
            }
        }
        Ok(())
    }
    fn admit_header(&mut self, value: &IfcValue, depth: usize) -> ExpressResult<()> {
        self.tick()?;
        if depth >= self.control.maximum_depth.min(256) {
            return Err(error("depth", "Header depth exceeded"));
        }
        match value {
            IfcValue::Aggregate(items) => {
                for item in items {
                    self.admit_header(item, depth + 1)?;
                }
            }
            IfcValue::TypedValue(typed) => {
                if !self.canonical_identifier(&typed.name)? {
                    return Err(error("admission", "Header typed values require canonical EXPRESS identifiers"));
                }
                for item in &typed.items {
                    self.admit_header(item, depth + 1)?;
                }
            }
            IfcValue::Real(value) if !value.is_finite() => return Err(error("admission", "Nonfinite header REAL")),
            IfcValue::Reference(_) => return Err(error("admission", "Header entity references are not admitted")),
            IfcValue::Binary(text) => {
                if !self.canonical_binary(text)? {
                    return Err(error("admission", "Header BINARY content is not canonical"));
                }
            }
            IfcValue::Enum(text) => {
                if !self.canonical_identifier(text)? {
                    return Err(error("admission", "Header enumeration is not canonical"));
                }
            }
            IfcValue::String(text) => {
                for _ in text.chars() {
                    self.tick()?;
                }
            }
            _ => {}
        }
        Ok(())
    }
    fn index_incoming(&mut self, value: &ExpressValue, owner: usize, attribute: &str, depth: usize) -> ExpressResult<()> {
        self.tick()?;
        if depth >= self.control.maximum_depth.min(256) {
            return Err(error("depth", "Incoming reference depth exceeded"));
        }
        match value.data.as_ref() {
            ExpressData::Entity(target) => {
                self.charge_nodes(1)?;
                self.incoming.entry(*target).or_default().push((owner, attribute.into()));
            }
            ExpressData::Aggregate { items, .. } => {
                for item in items {
                    self.index_incoming(item, owner, attribute, depth + 1)?;
                }
            }
            _ => {}
        }
        Ok(())
    }
    pub(crate) fn entity_value(&mut self, index: usize) -> ExpressResult<ExpressValue> {
        let mut value = self.make_entity(index)?;
        value.types = Arc::new(self.entities[index].types.clone());
        Ok(value)
    }
    pub fn entity_by_id(&mut self, id: u64) -> ExpressResult<ExpressValue> {
        self.tick()?;
        let index = *self.identities.get(&id).ok_or_else(|| error("entity-domain", format!("No admitted entity #{id}")))?;
        self.entity_value(index)
    }
    pub fn attribute_by_id(&mut self, id: u64, name: &str) -> ExpressResult<ExpressValue> {
        let value = self.entity_by_id(id)?;
        let owner = value.types.first().cloned().unwrap_or_default();
        self.attribute(&value, name).map_err(|failure| entity_failure(failure, id, &owner, Some(name)))
    }
    pub fn attribute(&mut self, value: &ExpressValue, name: &str) -> ExpressResult<ExpressValue> {
        let catalog = self.catalog;
        let program = self.program;

        self.tick()?;
        self.check_value_context(value)?;
        if name.len() > 1024 {
            return Err(error("ast-budget", "Attribute identifier exceeds native admission"));
        }
        if let ExpressData::Enum(owner) = value.data.as_ref() {
            if catalog.types.get(owner).is_some_and(|declaration| declaration.kind == "enum" && declaration.members.iter().any(|member| member == name)) {
                let mut result = self.make(ExpressData::Enum(name.into()))?;
                result.types = Arc::new(vec![owner.clone()]);
                return Ok(result);
            }
        }
        let ExpressData::Entity(index) = value.data.as_ref() else { return self.unknown() };
        self.checked_entity(value)?;
        let index = *index;
        let name = name.to_ascii_uppercase();
        let mut owners = Vec::new();
        if let Some(qualifier) = &value.qualifier {
            owners.push(qualifier.clone());
        }
        for owner in &self.entities[index].types {
            if !owners.contains(owner) {
                owners.push(owner.clone());
            }
        }
        for owner in owners {
            self.tick()?;
            if let Some(rule) = program.derived.iter().find(|rule| rule.owner == owner && rule.label == name) {
                let key = (index, owner.clone(), name.clone());
                if !self.pending.insert(key.clone()) {
                    return Err(error("derived-cycle", format!("Cyclic derived attribute {owner}.{name}")));
                }
                let result = (|| {
                    let actual = self.entity_value(index)?;
                    let mut scope = self.scoped(&actual)?;
                    let mut result = self.evaluate(rule.expression, &mut scope)?;
                    if let Some(expression) = catalog.entities.get(&owner).and_then(|declaration| declaration.derived.iter().find(|attribute| attribute.name.eq_ignore_ascii_case(&name))).map(|declaration| &declaration.type_expr) {
                        if expression.kind == "aggregate" {
                            let kind = match expression.aggregate.as_deref() {
                                Some("ARRAY") => ExpressAggregateKind::Array,
                                Some("SET") => ExpressAggregateKind::Set,
                                Some("BAG") => ExpressAggregateKind::Bag,
                                _ => ExpressAggregateKind::List,
                            };
                            if matches!(result.data.as_ref(), ExpressData::Aggregate { .. }) {
                                let lower = if kind == ExpressAggregateKind::Array { expression.lower.unwrap_or(1) as i128 } else { 1 };
                                let mut items = Vec::new();
                                for item in result.items() {
                                    self.tick()?;
                                    self.push_value(&mut items, item.clone())?;
                                }
                                result = self.aggregate(items, kind, lower)?;
                            }
                        }
                    }
                    Ok(result)
                })();
                self.pending.remove(&key);
                return result;
            }
        }
        if let Some(inverse) = program.inverse.iter().find(|inverse| inverse.name == name && self.entities[index].types.iter().any(|owner| owner == inverse.owner)) {
            let matching = self.inverse_matches(index, inverse)?;
            if !inverse.aggregate {
                return if let Some(index) = matching.first() { self.entity_value(*index) } else { self.unknown() };
            }
            let mut items = Vec::new();
            for index in matching {
                let value = self.entity_value(index)?;
                self.push_value(&mut items, value)?;
            }
            return self.aggregate(items, ExpressAggregateKind::Set, 1);
        }
        self.entities[index].fields.get(&name).cloned().map(Ok).unwrap_or_else(|| self.unknown())
    }
    fn inverse_matches(&mut self, index: usize, inverse: &ExpressInverse) -> ExpressResult<Vec<usize>> {
        let mut result = Vec::new();
        let mut seen = BTreeSet::new();
        let length = self.incoming.get(&index).map_or(0, Vec::len);
        for offset in 0..length {
            self.tick()?;
            let (owner, attribute) = self.incoming[&index][offset].clone();
            if attribute == inverse.attribute && self.entities[owner].types.iter().any(|name| name == inverse.entity_type) && seen.insert(owner) {
                result.push(owner);
            }
        }
        Ok(result)
    }
    pub(crate) fn used_in(&mut self, value: &ExpressValue, role: &str) -> ExpressResult<ExpressValue> {
        self.tick()?;
        let ExpressData::Entity(target) = value.data.as_ref() else { return self.aggregate(Vec::new(), ExpressAggregateKind::Bag, 1) };
        self.checked_entity(value)?;
        let target = *target;
        if role.len() > 1024 {
            return Err(error("role-budget", "USEDIN role exceeds the admitted identifier domain"));
        }
        let role = self.text(role)?.to_ascii_uppercase();
        let mut parts = role.rsplit('.');
        let attribute = parts.next().filter(|value| !value.is_empty());
        let owner = parts.next();
        let mut result = Vec::new();
        let length = self.incoming.get(&target).map_or(0, Vec::len);
        for offset in 0..length {
            self.tick()?;
            let (index, field) = self.incoming[&target][offset].clone();
            if owner.is_some_and(|owner| !self.entities[index].types.iter().any(|name| name == owner)) || attribute.is_some_and(|attribute| field != attribute) {
                continue;
            }
            let value = self.entity_value(index)?;
            self.push_value(&mut result, value)?;
        }
        self.aggregate(result, ExpressAggregateKind::Bag, 1)
    }
    fn run_rule(&mut self, rule: &ExpressRule, scope: &mut ExpressScope, kind: ExpressRuleKind, id: Option<u64>, report: &mut IfcExpressValidation) -> ExpressResult<()> {
        self.tick()?;
        match self.evaluate(rule.expression, scope).and_then(|value| if matches!(value.data.as_ref(), ExpressData::Logical(_) | ExpressData::Unknown) { Ok(value) } else { Err(error("rule-result", "WHERE rule did not evaluate to LOGICAL")) }) {
            Ok(value) => {
                let result = outcome(&value);
                report.results.push(ExpressRuleResult { owner: rule.owner.into(), rule: rule.label.into(), entity_id: id, result, kind });
                if result == ExpressRuleOutcome::False {
                    report.diagnostics.push(ExpressDiagnostic {
                        code: "express.rule".into(),
                        message: format!("{}.{} is FALSE", rule.owner, rule.label),
                        owner: Some(rule.owner.into()),
                        rule: Some(rule.label.into()),
                        entity_id: id,
                        offset: Some(rule.expression.offset),
                        attribute: None,
                    });
                }
            }
            Err(failure) => {
                report.complete = false;
                report.results.push(ExpressRuleResult { owner: rule.owner.into(), rule: rule.label.into(), entity_id: id, result: ExpressRuleOutcome::Incomplete, kind });
                report.diagnostics.push(ExpressDiagnostic {
                    code: format!("express.{}", failure.code),
                    message: failure.message.clone(),
                    owner: Some(rule.owner.into()),
                    rule: Some(rule.label.into()),
                    entity_id: id,
                    offset: Some(rule.expression.offset),
                    attribute: None,
                });
                if ["cancelled", "work-budget", "node-budget", "depth", "iteration-budget"].contains(&failure.code.as_str()) {
                    return Err(failure);
                }
            }
        }
        Ok(())
    }
    fn type_rules(&mut self, value: &ExpressValue, expression: &IfcTypeExpression, id: u64, seen: &mut BTreeSet<String>, report: &mut IfcExpressValidation, depth: usize) -> ExpressResult<()> {
        let catalog = self.catalog;
        let program = self.program;

        self.tick()?;
        if depth >= self.control.maximum_depth.min(256) {
            return Err(error("depth", "Type rule recursion exceeded"));
        }
        if value.missing() {
            return Ok(());
        }
        if expression.kind == "aggregate" {
            if let Some(item) = expression.item.as_deref() {
                for value in value.items() {
                    self.type_rules(value, item, id, &mut seen.clone(), report, depth + 1)?;
                }
            }
            return Ok(());
        }
        let mut name = expression.name.as_deref().unwrap_or("").to_ascii_uppercase();
        if catalog.entities.contains_key(&name) {
            return Ok(());
        }
        if catalog.types.get(&name).is_some_and(|declaration| declaration.kind == "select") {
            if let Some(actual) = value.types.first().filter(|name| catalog.types.contains_key(*name)) {
                name = actual.clone();
            }
        }
        if !seen.insert(name.clone()) {
            return Ok(());
        }
        for rule in program.rules {
            self.tick()?;
            if rule.owner == name {
                let mut scope = self.scoped(value)?;
                self.run_rule(rule, &mut scope, ExpressRuleKind::Where, Some(id), report)?;
            }
        }
        if let Some(inner) = catalog.types.get(&name).and_then(|declaration| declaration.type_expr.as_ref()) {
            self.type_rules(value, inner, id, seen, report, depth + 1)?;
        }
        Ok(())
    }
    fn checked_result(&mut self, owner: &str, rule: &str, kind: ExpressRuleKind, id: Option<u64>, value: ExpressValue, code: &str, report: &mut IfcExpressValidation) -> ExpressResult<()> {
        self.tick()?;
        let result = outcome(&value);
        report.results.push(ExpressRuleResult { owner: owner.into(), rule: rule.into(), entity_id: id, result, kind });
        if result == ExpressRuleOutcome::False {
            report.diagnostics.push(ExpressDiagnostic { code: code.into(), message: format!("{owner}.{rule} is FALSE"), owner: Some(owner.into()), rule: Some(rule.into()), entity_id: id, offset: None, attribute: None });
        }
        Ok(())
    }
    fn run_population(&mut self, snapshot: &IfcSnapshot, report: &mut IfcExpressValidation) -> ExpressResult<()> {
        let catalog = self.catalog;
        let program = self.program;

        self.initialize(snapshot)?;
        let population = self.entities.len();
        for index in 0..population {
            self.tick()?;
            let value = self.entity_value(index)?;
            let id = self.entities[index].identity.unwrap();
            let mut scope = self.scoped(&value)?;
            let types = self.entities[index].types.clone();
            for rule in program.rules {
                self.tick()?;
                if types.iter().any(|owner| owner == rule.owner) {
                    self.run_rule(rule, &mut scope, ExpressRuleKind::Where, Some(id), report)?;
                }
            }
            let name = types.first().ok_or_else(|| error("domain", "Missing entity leaf"))?;
            let declaration = &catalog.entities[name];
            for attribute in &declaration.all_attributes {
                let field = self.entities[index].fields.get(&attribute.name.to_ascii_uppercase()).cloned().unwrap_or(self.unknown()?);
                self.type_rules(&field, &attribute.type_expr, id, &mut BTreeSet::new(), report, 0)?;
            }
            for derived in program.derived {
                self.tick()?;
                if types.iter().any(|owner| owner == derived.owner) {
                    self.attribute(&value, derived.label).map_err(|failure| entity_failure(failure, id, derived.owner, Some(derived.label)))?;
                }
            }
            for bound in program.bounds {
                self.tick()?;
                if !types.iter().any(|owner| owner == bound.owner) {
                    continue;
                }
                let mut path = bound.label.split('.');
                let attribute = path.next().unwrap_or("");
                let mut values = vec![self.attribute(&value, attribute)?];
                let rest: Vec<_> = path.collect();
                let side = rest.last().copied().unwrap_or("");
                for segment in rest.iter().take(rest.len().saturating_sub(1)) {
                    if *segment == "ITEM" {
                        let mut next = Vec::new();
                        for value in values {
                            for item in value.items() {
                                self.tick()?;
                                self.push_value(&mut next, item.clone())?;
                            }
                        }
                        values = next;
                    }
                }
                let expected = self.evaluate(bound.expression, &mut scope)?;
                for aggregate in values {
                    let actual = if matches!(aggregate.data.as_ref(), ExpressData::Aggregate { .. }) {
                        let coordinate = if side == "UPPER" {
                            aggregate.lower().checked_add(aggregate.items().len() as i128).and_then(|value| value.checked_sub(1)).ok_or_else(|| error("integer-overflow", "Dynamic aggregate bound overflow"))?
                        } else {
                            aggregate.lower()
                        };
                        self.integer_value(coordinate)?
                    } else {
                        self.unknown()?
                    };
                    let comparison = self.binary("=", &actual, &expected)?;
                    self.checked_result(bound.owner, bound.label, ExpressRuleKind::Bound, Some(id), comparison, "express.bound", report)?;
                }
            }
            for inverse in program.inverse {
                self.tick()?;
                if !types.iter().any(|owner| owner == inverse.owner) {
                    continue;
                }
                let size = self.inverse_matches(index, inverse)?.len() as i128;
                let actual = self.integer_value(size)?;
                if !inverse.aggregate {
                    let one = self.integer_value(1)?;
                    let comparison = self.binary("=", &actual, &one)?;
                    self.checked_result(inverse.owner, inverse.name, ExpressRuleKind::Inverse, Some(id), comparison, "express.inverse", report)?;
                }
                for (bound, operator) in [(inverse.lower, ">="), (inverse.upper, "<=")] {
                    if let Some(bound) = bound.filter(|bound| bound.kind != ExpressExpressionKind::Unknown) {
                        let expected = self.evaluate(bound, &mut scope)?;
                        let comparison = self.binary(operator, &actual, &expected)?;
                        self.checked_result(inverse.owner, inverse.name, ExpressRuleKind::Inverse, Some(id), comparison, "express.inverse", report)?;
                    }
                }
            }
        }
        for rule in program.unique {
            let mut previous = Vec::new();
            for index in 0..population {
                self.tick()?;
                if !self.entities[index].types.iter().any(|owner| owner == rule.owner) {
                    continue;
                }
                let value = self.entity_value(index)?;
                let mut scope = self.scoped(&value)?;
                let tuple = self.evaluate(rule.expression, &mut scope)?;
                let mut result = ExpressLogical::True;
                for candidate in &previous {
                    match self.equal(&tuple, candidate, true)?.truth() {
                        ExpressLogical::True => {
                            result = ExpressLogical::False;
                            break;
                        }
                        ExpressLogical::Unknown => result = ExpressLogical::Unknown,
                        _ => {}
                    }
                }
                let comparison = self.logical(result)?;
                self.checked_result(rule.owner, rule.label, ExpressRuleKind::Unique, self.entities[index].identity, comparison, "express.unique", report)?;
                previous.push(tuple);
            }
        }
        for global in program.global_rules {
            let mut scope = ExpressScope::default();
            for name in global.populations {
                let mut values = Vec::new();
                for index in 0..population {
                    self.tick()?;
                    if self.entities[index].types.iter().any(|owner| owner == name) {
                        let value = self.entity_value(index)?;
                        self.push_value(&mut values, value)?;
                    }
                }
                let value = self.aggregate(values, ExpressAggregateKind::Set, 1)?;
                scope.values.insert((*name).into(), value);
            }
            self.declare(global.locals, &mut scope)?;
            self.statements(global.body, &mut scope)?;
            for rule in global.rules {
                self.run_rule(rule, &mut scope, ExpressRuleKind::Global, None, report)?;
            }
        }
        Ok(())
    }
}
/// ⚖️ Executes the normative retained program; any unexecuted or limited work makes validity false.
pub fn validate_ifc4x3_express(snapshot: &IfcSnapshot, control: ExpressControl<'_>) -> IfcExpressValidation {
    validate_ifc4x3_express_with_program(snapshot, super::ifc4x3_express_program(), control)
}
/// 🗂️ Prepares one owned controlled graph context for revision-cached computed-attribute queries.
pub fn prepare_ifc4x3_express<'a>(snapshot: &IfcSnapshot, control: ExpressControl<'a>) -> ExpressResult<ExpressEvaluator<'a>> {
    let mut evaluator = ExpressEvaluator::new(super::ifc4x3_express_program(), control);
    evaluator.initialize(snapshot)?;
    evaluator.tick()?;
    Ok(evaluator)
}
/// 🧪️ Runs the same native authority with a caller-selected owned canonical program.
pub fn validate_ifc4x3_express_with_program(snapshot: &IfcSnapshot, program: &'static ExpressProgram, control: ExpressControl<'_>) -> IfcExpressValidation {
    let mut evaluator = ExpressEvaluator::new(program, control);
    let mut report = IfcExpressValidation { complete: true, valid: false, diagnostics: Vec::new(), results: Vec::new(), work: 0, nodes: 0, physical_conformance: ExpressPhysicalConformance::NotAssessed };
    let execution = if program.schema != "IFC4X3_ADD2" { Err(error("program", "Canonical program schema must identify IFC4X3_ADD2")) } else { evaluator.run_population(snapshot, &mut report).and_then(|()| evaluator.tick()) };
    if let Err(failure) = execution {
        report.complete = false;
        report.diagnostics.push(ExpressDiagnostic { code: format!("express.{}", failure.code), message: failure.message, owner: failure.owner, rule: None, entity_id: failure.entity_id, offset: failure.offset, attribute: failure.attribute });
    }
    report.work = evaluator.work;
    report.nodes = evaluator.nodes;
    report.valid = report.complete && !report.results.iter().any(|row| row.result == ExpressRuleOutcome::False || row.result == ExpressRuleOutcome::Incomplete);
    report
}
