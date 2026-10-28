//! 🧠 Budgeted native EXPRESS interpreter over the canonical immutable syntax.
use super::ast::*;
use crate::catalog::{ifc4x3_catalog, IfcCatalog};
use std::borrow::Cow;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExpressLogical {
    True,
    False,
    Unknown,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExpressAggregateKind {
    List,
    Set,
    Bag,
    Array,
}
#[derive(Clone, Debug)]
pub enum ExpressData {
    Unknown,
    Logical(ExpressLogical),
    Integer(i128),
    Real(f64),
    String(String),
    Binary(String),
    Enum(String),
    Aggregate { items: Vec<ExpressValue>, kind: ExpressAggregateKind, lower: i128 },
    Entity(usize),
}
#[derive(Clone, Debug)]
pub struct ExpressValue {
    pub(crate) data: Arc<ExpressData>,
    pub(crate) types: Arc<Vec<String>>,
    pub(crate) qualifier: Option<String>,
    pub(crate) context: Option<Arc<()>>,
    pub(crate) entity_id: Option<u64>,
}
impl ExpressValue {
    pub fn entity_id(&self) -> Option<u64> {
        self.entity_id
    }
    pub fn data(&self) -> &ExpressData {
        self.data.as_ref()
    }
    pub fn types(&self) -> &[String] {
        &self.types
    }
    pub fn kind(&self) -> &'static str {
        match self.data.as_ref() {
            ExpressData::Unknown => "unknown",
            ExpressData::Logical(_) => "logical",
            ExpressData::Integer(_) => "integer",
            ExpressData::Real(_) => "real",
            ExpressData::String(_) => "string",
            ExpressData::Binary(_) => "binary",
            ExpressData::Enum(_) => "enum",
            ExpressData::Aggregate { .. } => "aggregate",
            ExpressData::Entity(_) => "entity",
        }
    }
    pub fn scalar_text(&self) -> Option<Cow<'_, str>> {
        match self.data.as_ref() {
            ExpressData::Unknown => None,
            ExpressData::Logical(value) => Some(
                match value {
                    ExpressLogical::True => "true",
                    ExpressLogical::False => "false",
                    ExpressLogical::Unknown => "unknown",
                }
                .into(),
            ),
            ExpressData::Integer(value) => Some(Cow::Owned(value.to_string())),
            ExpressData::Real(value) => Some(Cow::Owned(value.to_string())),
            ExpressData::String(value) | ExpressData::Binary(value) | ExpressData::Enum(value) => Some(Cow::Borrowed(value.as_str())),
            _ => None,
        }
    }
    pub fn truth(&self) -> ExpressLogical {
        if let ExpressData::Logical(value) = self.data.as_ref() {
            *value
        } else {
            ExpressLogical::Unknown
        }
    }
    pub fn missing(&self) -> bool {
        matches!(self.data.as_ref(), ExpressData::Unknown)
    }
    pub fn items(&self) -> &[ExpressValue] {
        if let ExpressData::Aggregate { items, .. } = self.data.as_ref() {
            items
        } else {
            &[]
        }
    }
    pub fn integer(&self) -> Option<i128> {
        if let ExpressData::Integer(value) = self.data.as_ref() {
            Some(*value)
        } else {
            None
        }
    }
    pub fn aggregate_kind(&self) -> ExpressAggregateKind {
        if let ExpressData::Aggregate { kind, .. } = self.data.as_ref() {
            *kind
        } else {
            ExpressAggregateKind::List
        }
    }
    pub fn lower(&self) -> i128 {
        if let ExpressData::Aggregate { lower, .. } = self.data.as_ref() {
            *lower
        } else {
            1
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct ExpressScope {
    pub values: BTreeMap<String, ExpressValue>,
    pub declarations: BTreeMap<String, String>,
}
#[derive(Clone, Debug)]
pub struct ExpressEvaluationError {
    pub code: String,
    pub message: String,
    pub offset: Option<usize>,
    pub entity_id: Option<u64>,
    pub owner: Option<String>,
    pub attribute: Option<String>,
}
impl std::fmt::Display for ExpressEvaluationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}
impl std::error::Error for ExpressEvaluationError {}
pub type ExpressResult<T> = Result<T, ExpressEvaluationError>;
pub(crate) fn error(code: &str, message: impl Into<String>) -> ExpressEvaluationError {
    ExpressEvaluationError { code: code.into(), message: message.into(), offset: None, entity_id: None, owner: None, attribute: None }
}
#[derive(Clone, Copy, Debug)]
pub struct ExpressProgress {
    pub phase: &'static str,
    pub completed: u64,
    pub maximum: u64,
}
pub struct ExpressControl<'a> {
    pub maximum_work: u64,
    pub maximum_nodes: usize,
    pub maximum_depth: usize,
    pub maximum_iterations: usize,
    pub batch_size: u64,
    pub cancelled: Option<&'a dyn Fn() -> bool>,
    pub progress: Option<&'a mut dyn FnMut(ExpressProgress)>,
}
impl Default for ExpressControl<'_> {
    fn default() -> Self {
        Self { maximum_work: 10_000_000, maximum_nodes: 10_000_000, maximum_depth: 256, maximum_iterations: 1_000_000, batch_size: 2048, cancelled: None, progress: None }
    }
}
#[derive(Clone, Debug)]
pub(crate) struct ExpressEntity {
    pub identity: Option<u64>,
    pub types: Vec<String>,
    pub fields: BTreeMap<String, ExpressValue>,
}
#[derive(Clone, Debug)]
pub(crate) enum ExpressFlow {
    Return(ExpressValue),
    Escape,
    Skip,
}
pub struct ExpressEvaluator<'a> {
    pub program: &'static ExpressProgram,
    pub control: ExpressControl<'a>,
    pub work: u64,
    pub nodes: usize,
    pub(crate) depth: usize,
    pub(crate) catalog: &'static IfcCatalog,
    pub(crate) entities: Vec<ExpressEntity>,
    pub(crate) identities: BTreeMap<u64, usize>,
    pub(crate) incoming: BTreeMap<usize, Vec<(usize, String)>>,
    pub(crate) pending: BTreeSet<(usize, String, String)>,
    pub(crate) context: Arc<()>,
}
impl<'a> ExpressEvaluator<'a> {
    pub fn new(program: &'static ExpressProgram, control: ExpressControl<'a>) -> Self {
        Self { program, control, work: 0, nodes: 0, depth: 0, catalog: ifc4x3_catalog(), entities: Vec::new(), identities: BTreeMap::new(), incoming: BTreeMap::new(), pending: BTreeSet::new(), context: Arc::new(()) }
    }
    pub fn tick(&mut self) -> ExpressResult<()> {
        if self.control.cancelled.is_some_and(|cancelled| cancelled()) {
            return Err(error("cancelled", "EXPRESS evaluation cancelled"));
        }
        self.work = self.work.checked_add(1).ok_or_else(|| error("work-budget", "EXPRESS work counter overflow"))?;
        if self.work > self.control.maximum_work {
            return Err(error("work-budget", "EXPRESS work budget exceeded"));
        }
        if self.work % self.control.batch_size.max(1) == 0 {
            if let Some(progress) = self.control.progress.as_mut() {
                progress(ExpressProgress { phase: "evaluate", completed: self.work, maximum: self.control.maximum_work });
            }
            if self.control.cancelled.is_some_and(|cancelled| cancelled()) {
                return Err(error("cancelled", "EXPRESS evaluation cancelled after progress"));
            }
        }
        Ok(())
    }
    pub fn make(&mut self, data: ExpressData) -> ExpressResult<ExpressValue> {
        if matches!(&data, ExpressData::Entity(_)) {
            self.tick()?;
            return Err(error("entity-domain", "Entity handles require an owned arena identity query or constructor"));
        }
        self.own_value(data)
    }
    pub(crate) fn make_entity(&mut self, index: usize) -> ExpressResult<ExpressValue> {
        self.own_value(ExpressData::Entity(index))
    }
    fn own_value(&mut self, data: ExpressData) -> ExpressResult<ExpressValue> {
        self.tick()?;
        let charge = 1usize
            .checked_add(match &data {
                ExpressData::Aggregate { items, .. } => items.len(),
                ExpressData::String(text) | ExpressData::Binary(text) | ExpressData::Enum(text) => text.len(),
                _ => 0,
            })
            .ok_or_else(|| error("node-budget", "Owned node counter overflow"))?;
        if charge > self.control.maximum_nodes.saturating_sub(self.nodes) {
            return Err(error("node-budget", "EXPRESS owned node budget exceeded"));
        }
        if matches!(&data,ExpressData::Entity(index) if *index>=self.entities.len()) {
            return Err(error("entity-domain", "Entity does not belong to this evaluator"));
        }
        let mut context = if matches!(&data, ExpressData::Entity(_)) { Some(self.context.clone()) } else { None };
        if let ExpressData::Aggregate { items, .. } = &data {
            for value in items {
                self.tick()?;
                self.check_value_context(value)?;
                if value.context.is_some() {
                    context = Some(self.context.clone());
                }
            }
        }
        self.nodes += charge;
        let entity_id = if let ExpressData::Entity(index) = &data { self.entities[*index].identity } else { None };
        Ok(ExpressValue { data: Arc::new(data), types: Arc::new(Vec::new()), qualifier: None, context, entity_id })
    }
    pub(crate) fn checked_entity(&self, value: &ExpressValue) -> ExpressResult<usize> {
        let ExpressData::Entity(index) = value.data.as_ref() else { return Err(error("entity-domain", "Entity value required")) };
        if *index >= self.entities.len() || !value.context.as_ref().is_some_and(|context| Arc::ptr_eq(context, &self.context)) {
            return Err(error("entity-domain", "Entity value belongs to another evaluation context"));
        }
        Ok(*index)
    }
    pub(crate) fn check_value_context(&self, value: &ExpressValue) -> ExpressResult<()> {
        if value.context.as_ref().is_some_and(|context| !Arc::ptr_eq(context, &self.context)) {
            let mut failure = error("entity-domain", "Value contains an entity from another evaluation context");
            failure.entity_id = value.entity_id;
            return Err(failure);
        }
        if matches!(value.data.as_ref(), ExpressData::Entity(_)) {
            self.checked_entity(value)?;
        }
        Ok(())
    }
    pub(crate) fn charge_nodes(&mut self, count: usize) -> ExpressResult<()> {
        self.tick()?;
        if count > self.control.maximum_nodes.saturating_sub(self.nodes) {
            return Err(error("node-budget", "EXPRESS arena or index exceeds owned admission"));
        }
        self.nodes += count;
        Ok(())
    }
    pub(crate) fn push_value(&mut self, values: &mut Vec<ExpressValue>, value: ExpressValue) -> ExpressResult<()> {
        self.tick()?;
        if values.len().saturating_add(2) > self.control.maximum_nodes.saturating_sub(self.nodes) {
            return Err(error("node-budget", "Aggregate construction exceeds owned node admission"));
        }
        self.check_value_context(&value)?;
        values.push(value);
        Ok(())
    }
    pub(crate) fn text(&mut self, source: &str) -> ExpressResult<String> {
        let mut result = String::new();
        for item in source.chars() {
            self.tick()?;
            if result.len().saturating_add(item.len_utf8()) > self.control.maximum_nodes.saturating_sub(self.nodes) {
                return Err(error("node-budget", "Text construction exceeds owned node admission"));
            }
            result.push(item);
        }
        Ok(result)
    }
    pub(crate) fn text_order(&mut self, left: &str, right: &str) -> ExpressResult<Ordering> {
        let mut left = left.chars();
        let mut right = right.chars();
        loop {
            self.tick()?;
            match (left.next(), right.next()) {
                (None, None) => return Ok(Ordering::Equal),
                (None, Some(_)) => return Ok(Ordering::Less),
                (Some(_), None) => return Ok(Ordering::Greater),
                (Some(a), Some(b)) if a != b => return Ok(a.cmp(&b)),
                _ => {}
            }
        }
    }
    fn clone_scope(&mut self, source: &ExpressScope) -> ExpressResult<ExpressScope> {
        let mut result = ExpressScope::default();
        for (name, value) in &source.values {
            self.tick()?;
            if result.values.len() >= self.control.maximum_nodes.saturating_sub(self.nodes) {
                return Err(error("node-budget", "Scope capture exceeds owned admission"));
            }
            let name = self.text(name)?;
            result.values.insert(name, value.clone());
        }
        for (name, value) in &source.declarations {
            self.tick()?;
            let name = self.text(name)?;
            let value = self.text(value)?;
            result.declarations.insert(name, value);
        }
        Ok(result)
    }
    pub(crate) fn fields_owned(&mut self, index: usize) -> ExpressResult<BTreeMap<String, ExpressValue>> {
        use std::ops::Bound::{Excluded, Unbounded};
        let mut result = BTreeMap::new();
        let mut previous: Option<String> = None;
        loop {
            self.tick()?;
            let next = if let Some(previous) = &previous { self.entities[index].fields.range((Excluded(previous.clone()), Unbounded)).next() } else { self.entities[index].fields.first_key_value() }.map(|(name, value)| (name.clone(), value.clone()));
            let Some((name, value)) = next else { break };
            if result.len() >= self.control.maximum_nodes.saturating_sub(self.nodes) {
                return Err(error("node-budget", "Entity field frontier exceeds owned admission"));
            }
            previous = Some(name.clone());
            result.insert(name, value);
        }
        Ok(result)
    }
    fn types_owned(&mut self, index: usize) -> ExpressResult<Vec<String>> {
        let length = self.entities[index].types.len();
        let mut result = Vec::new();
        for at in 0..length {
            self.tick()?;
            result.push(self.entities[index].types[at].clone());
        }
        Ok(result)
    }
    pub(crate) fn unknown(&mut self) -> ExpressResult<ExpressValue> {
        self.make(ExpressData::Unknown)
    }
    pub(crate) fn logical(&mut self, value: ExpressLogical) -> ExpressResult<ExpressValue> {
        self.make(ExpressData::Logical(value))
    }
    pub(crate) fn boolean(&mut self, value: bool) -> ExpressResult<ExpressValue> {
        self.logical(if value { ExpressLogical::True } else { ExpressLogical::False })
    }
    pub(crate) fn integer_value(&mut self, value: i128) -> ExpressResult<ExpressValue> {
        self.make(ExpressData::Integer(value))
    }
    pub(crate) fn real(&mut self, value: f64) -> ExpressResult<ExpressValue> {
        if !value.is_finite() {
            return Err(error("numeric-overflow", "REAL exceeds finite evaluation precision"));
        }
        self.make(ExpressData::Real(value))
    }
    pub(crate) fn aggregate(&mut self, items: Vec<ExpressValue>, kind: ExpressAggregateKind, lower: i128) -> ExpressResult<ExpressValue> {
        self.make(ExpressData::Aggregate { items, kind, lower })
    }
    pub(crate) fn scoped(&mut self, entity: &ExpressValue) -> ExpressResult<ExpressScope> {
        self.tick()?;
        self.check_value_context(entity)?;
        let mut scope = ExpressScope::default();
        scope.values.insert("SELF".into(), entity.clone());
        Ok(scope)
    }
    fn operand(&mut self, e: &ExpressExpression, scope: &mut ExpressScope, index: usize) -> ExpressResult<ExpressValue> {
        self.evaluate(e.operands.get(index).ok_or_else(|| error("ast", "Missing expression operand"))?, scope)
    }
    pub fn evaluate(&mut self, e: &ExpressExpression, scope: &mut ExpressScope) -> ExpressResult<ExpressValue> {
        self.tick()?;
        if e.name.is_some_and(|name| name.len() > 1024) || e.operator.is_some_and(|operator| operator.len() > 16) {
            return Err(error("ast-budget", "AST identifier or operator exceeds admission"));
        }
        if matches!(e.kind, ExpressExpressionKind::Integer | ExpressExpressionKind::Real) {
            let text = e.text.unwrap_or("");
            if text.len() > 128 {
                return Err(error("numeric-budget", "Numeric literal token exceeds admission"));
            }
            for _ in text.bytes() {
                self.tick()?;
            }
        }
        if self.depth >= self.control.maximum_depth.min(256) {
            return Err(error("depth", "EXPRESS expression depth exceeded"));
        }
        self.depth += 1;
        let result = self.expression(e, scope);
        self.depth -= 1;
        result.map_err(|mut failure| {
            failure.offset.get_or_insert(e.offset);
            failure
        })
    }
    fn expression(&mut self, e: &ExpressExpression, scope: &mut ExpressScope) -> ExpressResult<ExpressValue> {
        let catalog = self.catalog;
        let program = self.program;

        use ExpressExpressionKind::*;
        match e.kind {
            Unknown => self.unknown(),
            Integer => self.integer_value(e.text.unwrap_or("").parse().map_err(|_| error("integer-overflow", "Integer literal exceeds checked i128 domain"))?),
            Real => self.real(e.text.unwrap_or("").parse().map_err(|_| error("numeric-literal", "Invalid REAL literal"))?),
            String => {
                let text = self.text(e.text.unwrap_or(""))?;
                self.make(ExpressData::String(text))
            }
            Logical => self.logical(match e.text.unwrap_or("") {
                "true" => ExpressLogical::True,
                "false" => ExpressLogical::False,
                "unknown" => ExpressLogical::Unknown,
                _ => return Err(error("ast", "Logical literal requires true false or unknown")),
            }),
            Identifier => {
                let name = e.name.unwrap_or("");
                if let Some(value) = scope.values.get(name) {
                    self.check_value_context(value)?;
                    return Ok(value.clone());
                }
                if let Some(value) = scope.values.get("SELF").cloned() {
                    self.check_value_context(&value)?;
                    if matches!(value.data.as_ref(), ExpressData::Entity(_)) {
                        let index = self.checked_entity(&value)?;
                        let declared = self.entities[index].fields.contains_key(name)
                            || program.derived.iter().any(|rule| rule.label == name && self.entities[index].types.iter().any(|owner| owner == rule.owner))
                            || program.inverse.iter().any(|rule| rule.name == name && self.entities[index].types.iter().any(|owner| owner == rule.owner));
                        if declared {
                            return self.attribute(&value, name);
                        }
                    }
                }
                if name == "PI" {
                    return self.real(std::f64::consts::PI);
                }
                if catalog.entities.contains_key(name) || catalog.types.contains_key(name) || catalog.types.values().any(|declaration| declaration.kind == "enum" && declaration.members.iter().any(|value| value == name)) {
                    self.make(ExpressData::Enum(name.into()))
                } else {
                    Err(error("unbound", format!("Unbound EXPRESS identifier {name}")))
                }
            }
            Aggregate => {
                let mut result = Vec::new();
                for entry in e.entries {
                    let value = self.evaluate(entry.value, scope)?;
                    let repeat = if let Some(repeat) = entry.repeat { self.evaluate(repeat, scope)? } else { self.integer_value(1)? };
                    if repeat.missing() {
                        return self.unknown();
                    }
                    let count = repeat.integer().filter(|value| *value >= 0).ok_or_else(|| error("repeat-count", "Aggregate repetition requires nonnegative integer"))?;
                    for _ in 0..count {
                        self.tick()?;
                        self.push_value(&mut result, value.clone())?;
                    }
                }
                self.aggregate(result, ExpressAggregateKind::List, 1)
            }
            Qualify => {
                let mut value = self.operand(e, scope, 0)?;
                if matches!(value.data.as_ref(), ExpressData::Entity(_)) {
                    value.qualifier = Some(e.name.unwrap_or("").into());
                    value.types = Arc::new(vec![e.name.unwrap_or("").into()]);
                }
                Ok(value)
            }
            Attribute => {
                let value = self.operand(e, scope, 0)?;
                self.attribute(&value, e.name.unwrap_or(""))
            }
            Index => {
                let value = self.operand(e, scope, 0)?;
                let index = self.operand(e, scope, 1)?;
                let Some(index) = index.integer() else { return self.unknown() };
                let Some(at) = index.checked_sub(value.lower()).and_then(|value| usize::try_from(value).ok()) else { return self.unknown() };
                match value.data.as_ref() {
                    ExpressData::Aggregate { items, .. } => items.get(at).cloned().map(Ok).unwrap_or_else(|| self.unknown()),
                    ExpressData::String(text) => {
                        for (index, item) in text.chars().enumerate() {
                            self.tick()?;
                            if index == at {
                                return self.make(ExpressData::String(item.to_string()));
                            }
                        }
                        self.unknown()
                    }
                    _ => self.unknown(),
                }
            }
            Slice => {
                let value = self.operand(e, scope, 0)?;
                let low = self.operand(e, scope, 1)?.integer();
                let high = self.operand(e, scope, 2)?.integer();
                let (Some(low), Some(high)) = (low, high) else { return self.unknown() };
                let lower = value.lower();
                let length = match value.data.as_ref() {
                    ExpressData::Aggregate { items, .. } => items.len(),
                    ExpressData::String(text) => {
                        let mut count = 0usize;
                        for _ in text.chars() {
                            self.tick()?;
                            count += 1;
                        }
                        count
                    }
                    _ => return self.unknown(),
                };
                let upper = lower.checked_add(length as i128).and_then(|value| value.checked_sub(1)).ok_or_else(|| error("integer-overflow", "Slice upper bound overflow"))?;
                if low < lower || high < low || high > upper {
                    return self.unknown();
                }
                let mut result = Vec::new();
                match value.data.as_ref() {
                    ExpressData::Aggregate { items, .. } => {
                        for (index, item) in items.iter().enumerate() {
                            self.tick()?;
                            let coordinate = lower + index as i128;
                            if coordinate >= low && coordinate <= high {
                                self.push_value(&mut result, item.clone())?;
                            }
                        }
                        self.aggregate(result, ExpressAggregateKind::List, 1)
                    }
                    ExpressData::String(text) => {
                        let mut result = std::string::String::new();
                        for (index, item) in text.chars().enumerate() {
                            self.tick()?;
                            let coordinate = index as i128 + 1;
                            if coordinate >= low && coordinate <= high {
                                result.push(item);
                            }
                        }
                        self.make(ExpressData::String(result))
                    }
                    _ => self.unknown(),
                }
            }
            Query => {
                let source = self.operand(e, scope, 0)?;
                if source.missing() {
                    return self.unknown();
                }
                let mut result = Vec::new();
                for value in source.items() {
                    self.tick()?;
                    let mut local = self.clone_scope(scope)?;
                    local.values.insert(e.name.unwrap_or("").into(), value.clone());
                    if self.evaluate(e.operands.get(1).ok_or_else(|| error("ast", "Missing QUERY predicate"))?, &mut local)?.truth() == ExpressLogical::True {
                        self.push_value(&mut result, value.clone())?;
                    }
                }
                self.aggregate(result, source.aggregate_kind(), source.lower())
            }
            Interval => {
                let value = self.operand(e, scope, 1)?;
                let low = self.operand(e, scope, 0)?;
                let high = self.operand(e, scope, 2)?;
                let mut operators = e.operator.unwrap_or("").split(',');
                let first = self.binary(operators.next().unwrap_or(""), &low, &value)?;
                let last = self.binary(operators.next().unwrap_or(""), &value, &high)?;
                self.logical_binary("AND", &first, &last)
            }
            Unary => {
                let value = self.operand(e, scope, 0)?;
                match e.operator.unwrap_or("") {
                    "NOT" => self.logical(match value.truth() {
                        ExpressLogical::True => ExpressLogical::False,
                        ExpressLogical::False => ExpressLogical::True,
                        _ => ExpressLogical::Unknown,
                    }),
                    "+" if value.missing() || matches!(value.data.as_ref(), ExpressData::Integer(_) | ExpressData::Real(_)) => Ok(value),
                    "-" => match value.data.as_ref() {
                        ExpressData::Unknown => self.unknown(),
                        ExpressData::Integer(value) => self.integer_value(value.checked_neg().ok_or_else(|| error("integer-overflow", "Integer negation exceeds checked domain"))?),
                        ExpressData::Real(value) => self.real(-value),
                        _ => Err(error("operand", "Numeric unary operand required")),
                    },
                    _ => Err(error("operator", "Unsupported unary operator")),
                }
            }
            Binary => {
                let left = self.operand(e, scope, 0)?;
                let operator = e.operator.unwrap_or("");
                if operator == "AND" && left.truth() == ExpressLogical::False {
                    return self.boolean(false);
                }
                if operator == "OR" && left.truth() == ExpressLogical::True {
                    return self.boolean(true);
                }
                let right = self.operand(e, scope, 1)?;
                self.binary(operator, &left, &right)
            }
            Call => {
                let name = e.operands.first().and_then(|value| value.name).ok_or_else(|| error("call", "Named callable required"))?;
                if name == "NVL" {
                    if e.operands.len() != 3 {
                        return Err(error("call-arity", "NVL requires two arguments"));
                    }
                    let value = self.operand(e, scope, 1)?;
                    return if value.missing() { self.operand(e, scope, 2) } else { Ok(value) };
                }
                let mut arguments = Vec::new();
                for operand in e.operands.iter().skip(1) {
                    let value = self.evaluate(operand, scope)?;
                    self.push_value(&mut arguments, value)?;
                }
                self.call(name, arguments, scope)
            }
        }
    }
    fn logical_binary(&mut self, operator: &str, left: &ExpressValue, right: &ExpressValue) -> ExpressResult<ExpressValue> {
        use ExpressLogical::*;
        let (a, b) = (left.truth(), right.truth());
        self.logical(match operator {
            "AND" => {
                if a == False || b == False {
                    False
                } else if a == Unknown || b == Unknown {
                    Unknown
                } else {
                    True
                }
            }
            "OR" => {
                if a == True || b == True {
                    True
                } else if a == Unknown || b == Unknown {
                    Unknown
                } else {
                    False
                }
            }
            _ => {
                if a == Unknown || b == Unknown {
                    Unknown
                } else if a != b {
                    True
                } else {
                    False
                }
            }
        })
    }
    pub(crate) fn numeric_order(left: &ExpressData, right: &ExpressData) -> ExpressResult<Ordering> {
        match (left, right) {
            (ExpressData::Integer(a), ExpressData::Integer(b)) => Ok(a.cmp(b)),
            (ExpressData::Real(a), ExpressData::Real(b)) => a.partial_cmp(b).ok_or_else(|| error("numeric-overflow", "Nonfinite comparison")),
            (ExpressData::Integer(a), ExpressData::Real(b)) => {
                if !b.is_finite() {
                    return Err(error("numeric-overflow", "Nonfinite comparison"));
                }
                if *b >= 2f64.powi(127) {
                    return Ok(Ordering::Less);
                }
                if *b < -2f64.powi(127) {
                    return Ok(Ordering::Greater);
                }
                let integral = b.trunc() as i128;
                let order = a.cmp(&integral);
                Ok(if order != Ordering::Equal {
                    order
                } else if b.fract() > 0.0 {
                    Ordering::Less
                } else if b.fract() < 0.0 {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                })
            }
            (ExpressData::Real(_), ExpressData::Integer(_)) => Ok(Self::numeric_order(right, left)?.reverse()),
            _ => Err(error("numeric-operand", "Numeric operands required")),
        }
    }
    fn number(value: &ExpressValue) -> ExpressResult<f64> {
        match value.data.as_ref() {
            ExpressData::Real(value) if value.is_finite() => Ok(*value),
            ExpressData::Integer(integer) => {
                let value = *integer as f64;
                if value >= 2f64.powi(127) || value < -2f64.powi(127) || value as i128 != *integer {
                    return Err(error("integer-precision", "Integer cannot be represented exactly as REAL"));
                }
                Ok(value)
            }
            _ => Err(error("numeric-operand", "Numeric operand required")),
        }
    }
    pub fn equal(&mut self, left: &ExpressValue, right: &ExpressValue, identity: bool) -> ExpressResult<ExpressValue> {
        let result = self.equal_inner(left, right, identity, 0)?;
        self.logical(result)
    }
    fn equal_inner(&mut self, left: &ExpressValue, right: &ExpressValue, identity: bool, depth: usize) -> ExpressResult<ExpressLogical> {
        self.tick()?;
        self.check_value_context(left)?;
        self.check_value_context(right)?;
        if depth >= self.control.maximum_depth.min(256) {
            return Err(error("depth", "Equality depth exceeded"));
        }
        use ExpressData::*;
        use ExpressLogical::{False, True, Unknown as Indeterminate};
        if left.missing() || right.missing() {
            return Ok(Indeterminate);
        }
        let boolean = |value| if value { True } else { False };
        Ok(match (left.data.as_ref(), right.data.as_ref()) {
            (Integer(_) | Real(_), Integer(_) | Real(_)) => boolean(Self::numeric_order(left.data.as_ref(), right.data.as_ref())? == Ordering::Equal),
            (Entity(a), Entity(b)) => {
                self.checked_entity(left)?;
                self.checked_entity(right)?;
                if a == b {
                    True
                } else if identity {
                    boolean(a == b)
                } else {
                    let left_types = self.types_owned(*a)?;
                    let right_types = self.types_owned(*b)?;
                    let left_fields = self.fields_owned(*a)?;
                    let right_fields = self.fields_owned(*b)?;
                    if left_types.iter().any(|name| !right_types.contains(name)) || right_types.iter().any(|name| !left_types.contains(name)) || left_fields.len() != right_fields.len() {
                        False
                    } else {
                        let mut result = True;
                        for (key, value) in &left_fields {
                            let Some(other) = right_fields.get(key) else { return Ok(False) };
                            match self.equal_inner(value, other, false, depth + 1)? {
                                False => return Ok(False),
                                Indeterminate => result = Indeterminate,
                                _ => {}
                            }
                        }
                        result
                    }
                }
            }
            (Aggregate { items: a, kind: ak, .. }, Aggregate { items: b, kind: bk, .. }) => {
                if a.len() != b.len() {
                    False
                } else {
                    let unordered = matches!(ak, ExpressAggregateKind::Set | ExpressAggregateKind::Bag) || matches!(bk, ExpressAggregateKind::Set | ExpressAggregateKind::Bag);
                    if unordered {
                        if self.bag_cover(a, b, false, identity, depth + 1)? {
                            True
                        } else if self.bag_cover(a, b, true, identity, depth + 1)? {
                            Indeterminate
                        } else {
                            False
                        }
                    } else {
                        let mut result = True;
                        for (index, value) in a.iter().enumerate() {
                            match self.equal_inner(value, &b[index], identity, depth + 1)? {
                                False => return Ok(False),
                                Indeterminate => result = Indeterminate,
                                _ => {}
                            }
                        }
                        result
                    }
                }
            }
            (Logical(a), Logical(b)) => boolean(a == b),
            (String(a), String(b)) | (Binary(a), Binary(b)) | (Enum(a), Enum(b)) => boolean(self.text_order(a, b)? == Ordering::Equal),
            _ => False,
        })
    }
    fn contains(&mut self, items: &[ExpressValue], value: &ExpressValue) -> ExpressResult<ExpressLogical> {
        let mut result = ExpressLogical::False;
        for item in items {
            match self.equal_inner(item, value, true, 0)? {
                ExpressLogical::True => return Ok(ExpressLogical::True),
                ExpressLogical::Unknown => result = ExpressLogical::Unknown,
                _ => {}
            }
        }
        Ok(result)
    }
    fn bag_augment(&mut self, left: &[ExpressValue], right: &[ExpressValue], index: usize, potential: bool, identity: bool, equality_depth: usize, seen: &mut [bool], matched: &mut [Option<usize>], depth: usize) -> ExpressResult<bool> {
        self.tick()?;
        if depth >= self.control.maximum_depth.min(256) {
            return Err(error("depth", "BAG matching path exceeds depth admission"));
        }
        for candidate in 0..right.len() {
            self.tick()?;
            if seen[candidate] {
                continue;
            }
            let equal = self.equal_inner(&left[index], &right[candidate], identity, equality_depth)?;
            if equal != ExpressLogical::True && !(potential && equal == ExpressLogical::Unknown) {
                continue;
            }
            seen[candidate] = true;
            if matched[candidate].is_none() || self.bag_augment(left, right, matched[candidate].unwrap(), potential, identity, equality_depth, seen, matched, depth + 1)? {
                matched[candidate] = Some(index);
                return Ok(true);
            }
        }
        Ok(false)
    }
    fn bag_cover(&mut self, left: &[ExpressValue], right: &[ExpressValue], potential: bool, identity: bool, equality_depth: usize) -> ExpressResult<bool> {
        if left.len() > right.len() {
            return Ok(false);
        }
        if right.len().saturating_mul(2) > self.control.maximum_nodes.saturating_sub(self.nodes) {
            return Err(error("node-budget", "BAG matching frontier exceeds owned admission"));
        }
        let mut seen = Vec::new();
        let mut matched = Vec::new();
        for _ in right {
            self.tick()?;
            seen.push(false);
            matched.push(None);
        }
        for index in 0..left.len() {
            for value in &mut seen {
                self.tick()?;
                *value = false;
            }
            if !self.bag_augment(left, right, index, potential, identity, equality_depth, &mut seen, &mut matched, 0)? {
                return Ok(false);
            }
        }
        Ok(true)
    }
    pub fn binary(&mut self, operator: &str, left: &ExpressValue, right: &ExpressValue) -> ExpressResult<ExpressValue> {
        self.tick()?;
        self.check_value_context(left)?;
        self.check_value_context(right)?;
        if operator.len() > 16 {
            return Err(error("ast-budget", "Operator exceeds native admission"));
        }
        if !["AND", "OR", "XOR", "=", "<>", ":=:", ":<>:", "IN", "||", "+", "-", "*", "/", "**", "DIV", "MOD", "<", "<=", ">", ">="].contains(&operator) {
            return Err(error("unsupported-operator", format!("Unsupported EXPRESS operator {operator}")));
        }
        if ["AND", "OR", "XOR"].contains(&operator) {
            return self.logical_binary(operator, left, right);
        }
        if ["=", "<>", ":=:", ":<>:"].contains(&operator) {
            let mut result = self.equal_inner(left, right, operator.contains(':'), 0)?;
            if ["<>", ":<>:"].contains(&operator) {
                result = match result {
                    ExpressLogical::True => ExpressLogical::False,
                    ExpressLogical::False => ExpressLogical::True,
                    _ => ExpressLogical::Unknown,
                };
            }
            return self.logical(result);
        }
        if operator == "IN" && !right.missing() && !matches!(right.data.as_ref(), ExpressData::Aggregate { .. }) {
            return Err(error("aggregate-operand", "IN requires an aggregate right operand"));
        }
        if left.missing() || right.missing() {
            return if ["IN", "<", "<=", ">", ">="].contains(&operator) { self.logical(ExpressLogical::Unknown) } else { self.unknown() };
        }
        if operator == "IN" {
            let mut result = ExpressLogical::False;
            for value in right.items() {
                match self.equal_inner(left, value, true, 0)? {
                    ExpressLogical::True => return self.boolean(true),
                    ExpressLogical::Unknown => result = ExpressLogical::Unknown,
                    _ => {}
                }
            }
            return self.logical(result);
        }
        if operator == "||" {
            if let (ExpressData::Entity(a), ExpressData::Entity(b)) = (left.data.as_ref(), right.data.as_ref()) {
                self.checked_entity(left)?;
                self.checked_entity(right)?;
                let mut fields = BTreeMap::new();
                let mut types = Vec::new();
                for index in [*a, *b] {
                    let owned_fields = self.fields_owned(index)?;
                    let owned_types = self.types_owned(index)?;
                    for (name, value) in owned_fields {
                        self.tick()?;
                        fields.insert(name, value);
                    }
                    for name in owned_types {
                        self.tick()?;
                        if !types.contains(&name) {
                            types.push(name);
                        }
                    }
                }
                return self.construct_entity(types, fields);
            }
        }
        if matches!(left.data.as_ref(), ExpressData::Aggregate { .. }) || matches!(right.data.as_ref(), ExpressData::Aggregate { .. }) {
            let a = if matches!(left.data.as_ref(), ExpressData::Aggregate { .. }) { left.items() } else { std::slice::from_ref(left) };
            let b = if matches!(right.data.as_ref(), ExpressData::Aggregate { .. }) { right.items() } else { std::slice::from_ref(right) };
            let mut result = Vec::new();
            match operator {
                "+" => {
                    for value in a {
                        self.tick()?;
                        self.push_value(&mut result, value.clone())?;
                    }
                    for value in b {
                        self.tick()?;
                        if left.aggregate_kind() == ExpressAggregateKind::Set {
                            match self.contains(&result, value)? {
                                ExpressLogical::True => continue,
                                ExpressLogical::Unknown => return Err(error("aggregate-indeterminate", "SET union membership is indeterminate")),
                                _ => {}
                            }
                        }
                        {
                            self.push_value(&mut result, value.clone())?;
                        }
                    }
                }
                "-" => {
                    for value in a {
                        self.tick()?;
                        self.push_value(&mut result, value.clone())?;
                    }
                    for value in b {
                        let mut at = None;
                        for (index, candidate) in result.iter().enumerate() {
                            let equal = self.equal_inner(value, candidate, true, 0)?;
                            if equal == ExpressLogical::Unknown {
                                return Err(error("aggregate-indeterminate", "Aggregate difference membership is indeterminate"));
                            }
                            if equal == ExpressLogical::True {
                                at = Some(index);
                                break;
                            }
                        }
                        if let Some(at) = at {
                            if left.aggregate_kind() == ExpressAggregateKind::Bag || left.aggregate_kind() == ExpressAggregateKind::Set {
                                result.swap_remove(at);
                            } else {
                                let mut retained = Vec::new();
                                for (index, value) in result.iter().enumerate() {
                                    if index != at {
                                        self.push_value(&mut retained, value.clone())?;
                                    }
                                }
                                result = retained;
                            }
                        }
                    }
                }
                "*" => {
                    let bag = left.aggregate_kind() == ExpressAggregateKind::Bag || right.aggregate_kind() == ExpressAggregateKind::Bag;
                    let mut available = Vec::new();
                    if bag {
                        for value in b {
                            self.push_value(&mut available, value.clone())?;
                        }
                    }
                    for value in a {
                        if bag {
                            let mut at = None;
                            for (index, candidate) in available.iter().enumerate() {
                                match self.equal_inner(value, candidate, true, 0)? {
                                    ExpressLogical::True => {
                                        at = Some(index);
                                        break;
                                    }
                                    ExpressLogical::Unknown => return Err(error("aggregate-indeterminate", "BAG intersection membership is indeterminate")),
                                    _ => {}
                                }
                            }
                            if let Some(index) = at {
                                available.swap_remove(index);
                                self.push_value(&mut result, value.clone())?;
                            }
                        } else {
                            match self.contains(b, value)? {
                                ExpressLogical::True => self.push_value(&mut result, value.clone())?,
                                ExpressLogical::Unknown => return Err(error("aggregate-indeterminate", "Aggregate intersection membership is indeterminate")),
                                _ => {}
                            }
                        }
                    }
                }
                "<=" | ">=" => {
                    let (a, b) = if operator == "<=" { (a, b) } else { (b, a) };
                    if left.aggregate_kind() == ExpressAggregateKind::Bag || right.aggregate_kind() == ExpressAggregateKind::Bag {
                        if self.bag_cover(a, b, false, true, 0)? {
                            return self.boolean(true);
                        }
                        let possible = self.bag_cover(a, b, true, true, 0)?;
                        return self.logical(if possible { ExpressLogical::Unknown } else { ExpressLogical::False });
                    }
                    let mut result = ExpressLogical::True;
                    for value in a {
                        match self.contains(b, value)? {
                            ExpressLogical::False => return self.boolean(false),
                            ExpressLogical::Unknown => result = ExpressLogical::Unknown,
                            _ => {}
                        }
                    }
                    return self.logical(result);
                }
                _ => return Err(error("aggregate-operator", format!("Unsupported aggregate operator {operator}"))),
            }
            let kind = if matches!(left.data.as_ref(), ExpressData::Aggregate { .. }) { left.aggregate_kind() } else { right.aggregate_kind() };
            return self.aggregate(result, kind, 1);
        }
        if ["<", "<=", ">", ">="].contains(&operator) {
            let order = match (left.data.as_ref(), right.data.as_ref()) {
                (ExpressData::Logical(a), ExpressData::Logical(b)) => {
                    let rank = |value: &ExpressLogical| match value {
                        ExpressLogical::False => 0,
                        ExpressLogical::Unknown => 1,
                        ExpressLogical::True => 2,
                    };
                    rank(a).cmp(&rank(b))
                }
                (ExpressData::String(a), ExpressData::String(b)) | (ExpressData::Enum(a), ExpressData::Enum(b)) => self.text_order(a, b)?,
                _ => Self::numeric_order(left.data.as_ref(), right.data.as_ref())?,
            };
            return self.boolean(match operator {
                "<" => order == Ordering::Less,
                "<=" => order != Ordering::Greater,
                ">" => order == Ordering::Greater,
                _ => order != Ordering::Less,
            });
        }
        if let (ExpressData::String(a), ExpressData::String(b)) = (left.data.as_ref(), right.data.as_ref()) {
            if operator == "+" {
                let mut result = self.text(a)?;
                for item in b.chars() {
                    self.tick()?;
                    if result.len().saturating_add(item.len_utf8()) > self.control.maximum_nodes.saturating_sub(self.nodes) {
                        return Err(error("node-budget", "String concatenation exceeds admission"));
                    }
                    result.push(item);
                }
                return self.make(ExpressData::String(result));
            }
        }
        if let (ExpressData::Integer(a), ExpressData::Integer(b)) = (left.data.as_ref(), right.data.as_ref()) {
            if ["/", "DIV", "MOD"].contains(&operator) && *b == 0 {
                return self.unknown();
            }
            let result = match operator {
                "+" => Some(a.checked_add(*b)),
                "-" => Some(a.checked_sub(*b)),
                "*" => Some(a.checked_mul(*b)),
                "DIV" => Some(a.checked_div(*b)),
                "MOD" => Some(a.checked_rem(*b)),
                "**" if *b >= 0 => {
                    let exponent = u32::try_from(*b).map_err(|_| error("integer-overflow", "Integer exponent exceeds checked domain"))?;
                    Some(a.checked_pow(exponent))
                }
                _ => None,
            };
            if let Some(value) = result {
                return self.integer_value(value.ok_or_else(|| error("integer-overflow", "Integer arithmetic exceeds checked i128 domain"))?);
            }
        }
        let a = Self::number(left)?;
        let b = Self::number(right)?;
        if operator == "/" && b == 0.0 {
            return self.unknown();
        }
        self.real(match operator {
            "+" => a + b,
            "-" => a - b,
            "*" => a * b,
            "/" => a / b,
            "**" => a.powf(b),
            _ => return Err(error("unsupported-operator", format!("Unsupported numeric operator {operator}"))),
        })
    }
    pub(crate) fn construct_entity(&mut self, types: Vec<String>, fields: BTreeMap<String, ExpressValue>) -> ExpressResult<ExpressValue> {
        self.tick()?;
        for value in fields.values() {
            self.tick()?;
            self.check_value_context(value)?;
        }
        if self.entities.len() >= self.control.maximum_nodes {
            return Err(error("node-budget", "Entity arena budget exceeded"));
        }
        let catalog = self.catalog;
        let mut complete = Vec::new();
        for name in types {
            let mut current = Some(name);
            while let Some(name) = current {
                self.tick()?;
                if !complete.contains(&name) {
                    complete.push(name.clone());
                }
                current = catalog.entities.get(&name).and_then(|declaration| declaration.supertype.clone());
            }
        }
        let types = complete;
        self.charge_nodes(1)?;
        let index = self.entities.len();
        self.entities.push(ExpressEntity { identity: None, types: types.clone(), fields });
        let mut value = self.make_entity(index)?;
        value.types = Arc::new(types);
        Ok(value)
    }
    pub fn primitive_types(value: &ExpressValue) -> Vec<String> {
        match value.data.as_ref() {
            ExpressData::Integer(_) => vec!["INTEGER".into(), "REAL".into(), "NUMBER".into()],
            ExpressData::Real(_) => vec!["REAL".into(), "NUMBER".into()],
            ExpressData::Logical(ExpressLogical::Unknown) => vec!["LOGICAL".into()],
            ExpressData::Logical(_) => vec!["LOGICAL".into(), "BOOLEAN".into()],
            ExpressData::Aggregate { kind, .. } => vec![format!("{kind:?}").to_ascii_uppercase()],
            ExpressData::Unknown => vec![],
            _ => vec![value.kind().to_ascii_uppercase()],
        }
    }
    fn coerce(&mut self, value: ExpressValue, type_source: &str, scope: &ExpressScope) -> ExpressResult<ExpressValue> {
        self.tick()?;
        if !matches!(value.data.as_ref(), ExpressData::Aggregate { .. }) {
            return Ok(value);
        }
        let source = type_source.trim().to_ascii_uppercase();
        let kind = if source.starts_with("LIST") {
            ExpressAggregateKind::List
        } else if source.starts_with("SET") {
            ExpressAggregateKind::Set
        } else if source.starts_with("BAG") {
            ExpressAggregateKind::Bag
        } else if source.starts_with("ARRAY") {
            ExpressAggregateKind::Array
        } else {
            return Ok(value);
        };
        let mut lower = value.lower();
        if kind == ExpressAggregateKind::Array {
            if let Some(bounds) = source.split('[').nth(1).and_then(|part| part.split(']').next()) {
                let bound = bounds.split(':').next().unwrap_or("").trim();
                lower = if let Ok(value) = bound.parse::<i128>() { value } else { scope.values.get(bound).and_then(ExpressValue::integer).ok_or_else(|| error("bound", "Unresolved ARRAY declaration lower bound"))? };
            }
        }
        let mut items = Vec::new();
        for item in value.items() {
            self.tick()?;
            self.push_value(&mut items, item.clone())?;
        }
        let mut result = self.aggregate(items, kind, lower)?;
        result.types = value.types;
        Ok(result)
    }
    pub fn declare(&mut self, declarations: &[ExpressVariable], scope: &mut ExpressScope) -> ExpressResult<()> {
        for declaration in declarations {
            let value = if let Some(initial) = declaration.initial { self.evaluate(initial, scope)? } else { self.unknown()? };
            for name in declaration.names {
                self.tick()?;
                scope.declarations.insert((*name).into(), declaration.type_source.into());
                let value = self.coerce(value.clone(), declaration.type_source, scope)?;
                scope.values.insert((*name).into(), value);
            }
        }
        Ok(())
    }
    pub fn call(&mut self, name: &str, args: Vec<ExpressValue>, scope: &mut ExpressScope) -> ExpressResult<ExpressValue> {
        self.tick()?;
        if self.depth >= self.control.maximum_depth.min(256) {
            return Err(error("depth", "Function recursion depth exceeded"));
        }
        self.depth += 1;
        let result = self.call_inner(name, args, scope);
        self.depth -= 1;
        result
    }
    fn call_inner(&mut self, name: &str, args: Vec<ExpressValue>, scope: &mut ExpressScope) -> ExpressResult<ExpressValue> {
        let catalog = self.catalog;
        let program = self.program;
        if name.len() > 1024 {
            return Err(error("ast-budget", "Callable identifier exceeds native admission"));
        }
        if args.len() > self.control.maximum_nodes.saturating_sub(self.nodes) {
            return Err(error("node-budget", "Callable argument frontier exceeds owned admission"));
        }

        let name = name.to_ascii_uppercase();
        let expected = match name.as_str() {
            "NVL" | "USEDIN" => Some(2),
            "EXISTS" | "TYPEOF" | "SIZEOF" | "HIINDEX" | "LOINDEX" | "LENGTH" | "BLENGTH" | "ABS" | "SQRT" => Some(1),
            _ => None,
        };
        if expected.is_some_and(|expected| expected != args.len()) {
            return Err(error("call-arity", "Builtin arguments do not match their declared arity"));
        }
        let a = if let Some(value) = args.first() { value.clone() } else { self.unknown()? };
        let b = if let Some(value) = args.get(1) { value.clone() } else { self.unknown()? };
        for value in &args {
            self.tick()?;
            self.check_value_context(value)?;
        }
        match name.as_str() {
            "NVL" => return Ok(if a.missing() { b } else { a }),
            "EXISTS" => return self.boolean(!a.missing()),
            "TYPEOF" => {
                const PREFIX: &str = "IFC4X3_ADD2.";
                let primitive = if a.types.is_empty() { Self::primitive_types(&a) } else { Vec::new() };
                let types = if a.types.is_empty() { primitive.as_slice() } else { a.types.as_slice() };
                let mut result: Vec<ExpressValue> = Vec::new();
                for name in types {
                    self.tick()?;
                    let key = name.strip_prefix(PREFIX).unwrap_or(name);
                    let mut duplicate = false;
                    for value in &result {
                        self.tick()?;
                        if value.scalar_text().as_deref().is_some_and(|text| text.strip_prefix(PREFIX).unwrap_or(text) == key) {
                            duplicate = true;
                            break;
                        }
                    }
                    if duplicate {
                        continue;
                    }
                    let name = name.clone();
                    let value = self.make(ExpressData::String(if name.starts_with("IFC") && !name.contains('.') { format!("IFC4X3_ADD2.{name}") } else { name }))?;
                    self.push_value(&mut result, value)?;
                }
                let mut at = 0;
                while at < result.len() {
                    self.tick()?;
                    if !result[at].scalar_text().as_deref().is_some_and(|text| text.starts_with(PREFIX)) {
                        at += 1;
                        continue;
                    }
                    let parents = {
                        let name = result[at].scalar_text().unwrap();
                        super::metadata::select_parents(name.strip_prefix(PREFIX).unwrap_or(&name))
                    };
                    for parent in parents {
                        self.tick()?;
                        let mut present = false;
                        for value in &result {
                            self.tick()?;
                            if value.scalar_text().as_deref().is_some_and(|text| text.strip_prefix(PREFIX).unwrap_or(text) == *parent) {
                                present = true;
                                break;
                            }
                        }
                        if !present {
                            let value = self.make(ExpressData::String(format!("{PREFIX}{parent}")))?;
                            self.push_value(&mut result, value)?;
                        }
                    }
                    at += 1;
                }
                return self.aggregate(result, ExpressAggregateKind::Set, 1);
            }
            "USEDIN" => return self.used_in(&a, if let ExpressData::String(value) = b.data.as_ref() { value } else { "" }),
            _ => {}
        }
        if a.missing() && ["SIZEOF", "HIINDEX", "LOINDEX", "LENGTH", "BLENGTH", "ABS", "SQRT"].contains(&name.as_str()) {
            return self.unknown();
        }
        if ["SIZEOF", "HIINDEX", "LOINDEX"].contains(&name.as_str()) && !matches!(a.data.as_ref(), ExpressData::Aggregate { .. }) {
            return Err(error("aggregate-operand", "Aggregate builtin requires an aggregate argument"));
        }
        match name.as_str() {
            "SIZEOF" => return self.integer_value(a.items().len() as i128),
            "HIINDEX" => return self.integer_value(a.lower().checked_add(a.items().len() as i128).and_then(|value| value.checked_sub(1)).ok_or_else(|| error("integer-overflow", "Aggregate upper bound overflow"))?),
            "LOINDEX" => return self.integer_value(a.lower()),
            "LENGTH" => {
                let ExpressData::String(text) = a.data.as_ref() else { return Err(error("operand", "String LENGTH operand required")) };
                let mut count = 0i128;
                for _ in text.chars() {
                    self.tick()?;
                    count += 1;
                }
                return self.integer_value(count);
            }
            "BLENGTH" => {
                let ExpressData::Binary(text) = a.data.as_ref() else { return Err(error("operand", "Binary BLENGTH operand required")) };
                return self.integer_value((text.len().saturating_sub(1) as i128) * 4 - text.as_bytes().first().map_or(0, |value| value.saturating_sub(b'0')) as i128);
            }
            "ABS" => {
                return match a.data.as_ref() {
                    ExpressData::Integer(value) => self.integer_value(value.checked_abs().ok_or_else(|| error("integer-overflow", "ABS integer overflow"))?),
                    _ => self.real(Self::number(&a)?.abs()),
                }
            }
            "SQRT" => {
                let value = Self::number(&a)?;
                return if value < 0.0 { self.unknown() } else { self.real(value.sqrt()) };
            }
            _ => {}
        }
        if let Some(function) = program.functions.iter().find(|function| function.name == name) {
            if args.len() != function.parameters.iter().map(|parameter| parameter.names.len()).sum::<usize>() {
                return Err(error("call-arity", "Function arguments do not match the canonical declaration"));
            }
            let mut local = ExpressScope::default();
            let mut index = 0;
            for parameter in function.parameters {
                for key in parameter.names {
                    let value = if let Some(value) = args.get(index) { value.clone() } else { self.unknown()? };
                    index += 1;
                    let value = self.coerce(value, parameter.type_source, scope)?;
                    local.values.insert((*key).into(), value);
                }
            }
            self.declare(function.locals, &mut local)?;
            let value = match self.statements(function.body, &mut local)? {
                Some(ExpressFlow::Return(value)) => value,
                _ => self.unknown()?,
            };
            return self.coerce(value, function.result_type, &local);
        }
        if let Some(declaration) = catalog.entities.get(&name) {
            if args.len() != declaration.attributes.len() {
                return Err(error("call-arity", "Entity constructor arguments do not match direct attributes"));
            }
            let names: Vec<_> = declaration.attributes.iter().map(|attribute| attribute.name.to_ascii_uppercase()).collect();
            let mut fields = BTreeMap::new();
            for (index, name) in names.into_iter().enumerate() {
                self.tick()?;
                fields.insert(name, if let Some(value) = args.get(index) { value.clone() } else { self.unknown()? });
            }
            return self.construct_entity(vec![name], fields);
        }
        if catalog.types.contains_key(&name) {
            if args.len() != 1 {
                return Err(error("call-arity", "Defined type constructor requires one argument"));
            }
            let mut value = a;
            let mut types = self.type_names(&name)?;
            for name in if value.types.is_empty() { Self::primitive_types(&value) } else { value.types.as_ref().clone() } {
                self.tick()?;
                if !types.contains(&name) {
                    types.push(name);
                }
            }
            value.types = Arc::new(types);
            return Ok(value);
        }
        Err(error("unsupported-call", format!("Unsupported EXPRESS callable {name}")))
    }
    pub(crate) fn statements(&mut self, body: &[ExpressStatement], scope: &mut ExpressScope) -> ExpressResult<Option<ExpressFlow>> {
        if self.depth >= self.control.maximum_depth.min(256) {
            return Err(error("depth", "Statement nesting depth exceeded"));
        }
        self.depth += 1;
        let result = self.statement_body(body, scope);
        self.depth -= 1;
        result
    }
    fn statement_body(&mut self, body: &[ExpressStatement], scope: &mut ExpressScope) -> ExpressResult<Option<ExpressFlow>> {
        use ExpressStatementKind::*;
        for statement in body {
            self.tick()?;
            let expression = |index: usize| statement.expressions.get(index).ok_or_else(|| error("ast", "Missing statement expression"));
            match statement.kind {
                Return => return Ok(Some(ExpressFlow::Return(self.evaluate(expression(0)?, scope)?))),
                Escape => return Ok(Some(ExpressFlow::Escape)),
                Skip => return Ok(Some(ExpressFlow::Skip)),
                Null => {}
                Call => {
                    self.evaluate(expression(0)?, scope)?;
                }
                Assign => {
                    let value = self.evaluate(expression(1)?, scope)?;
                    self.assign(expression(0)?, value, scope)?;
                }
                Block => {
                    if let Some(flow) = self.statements(statement.body, scope)? {
                        return Ok(Some(flow));
                    }
                }
                If => {
                    let branch = if self.evaluate(expression(0)?, scope)?.truth() == ExpressLogical::True { statement.body } else { statement.otherwise };
                    if let Some(flow) = self.statements(branch, scope)? {
                        return Ok(Some(flow));
                    }
                }
                Case => {
                    let selector = self.evaluate(expression(0)?, scope)?;
                    let mut selected = statement.otherwise;
                    'outer: for entry in statement.cases {
                        for label in entry.labels {
                            let value = self.evaluate(label, scope)?;
                            if self.equal_inner(&selector, &value, false, 0)? == ExpressLogical::True {
                                selected = entry.body;
                                break 'outer;
                            }
                        }
                    }
                    if let Some(flow) = self.statements(selected, scope)? {
                        return Ok(Some(flow));
                    }
                }
                Repeat => {
                    let (mut current, last, step) = if statement.name.is_some() {
                        let current = self.evaluate(expression(0)?, scope)?.integer();
                        let last = self.evaluate(expression(1)?, scope)?.integer();
                        let step = self.evaluate(expression(2)?, scope)?.integer();
                        match (current, last, step) {
                            (Some(current), Some(last), Some(step)) if step != 0 => (current, Some(last), step),
                            _ => continue,
                        }
                    } else {
                        (1, None, 1)
                    };
                    let mut iteration = 0usize;
                    loop {
                        if iteration >= self.control.maximum_iterations {
                            return Err(error("iteration-budget", "EXPRESS repeat iteration budget exceeded"));
                        }
                        iteration += 1;
                        self.tick()?;
                        if let Some(name) = statement.name {
                            let last = last.unwrap();
                            if if step > 0 { current > last } else { current < last } {
                                break;
                            }
                            let value = self.integer_value(current)?;
                            scope.values.insert(name.into(), value);
                        }
                        if let Some(condition) = statement.while_condition {
                            if self.evaluate(condition, scope)?.truth() != ExpressLogical::True {
                                break;
                            }
                        }
                        match self.statements(statement.body, scope)? {
                            Some(ExpressFlow::Return(value)) => return Ok(Some(ExpressFlow::Return(value))),
                            Some(ExpressFlow::Escape) => break,
                            _ => {}
                        }
                        if let Some(condition) = statement.until_condition {
                            if self.evaluate(condition, scope)?.truth() == ExpressLogical::True {
                                break;
                            }
                        }
                        if statement.name.is_some() {
                            current = current.checked_add(step).ok_or_else(|| error("integer-overflow", "Repeat index overflow"))?;
                        }
                    }
                }
            }
        }
        Ok(None)
    }
    fn assign(&mut self, target: &ExpressExpression, value: ExpressValue, scope: &mut ExpressScope) -> ExpressResult<()> {
        self.tick()?;
        match target.kind {
            ExpressExpressionKind::Identifier => {
                let name = target.name.unwrap_or("");
                let value = if let Some(declaration) = scope.declarations.get(name).cloned() { self.coerce(value, &declaration, scope)? } else { value };
                scope.values.insert(name.into(), value);
                Ok(())
            }
            ExpressExpressionKind::Attribute => {
                let owner = self.evaluate(target.operands.first().ok_or_else(|| error("ast", "Missing assignment owner"))?, scope)?;
                let ExpressData::Entity(index) = owner.data.as_ref() else { return Err(error("assignment", "Entity assignment target required")) };
                self.checked_entity(&owner)?;
                if self.entities[*index].identity.is_some() {
                    return Err(error("assignment-identity", "Admitted population entities are immutable during semantic evaluation"));
                }
                self.entities[*index].fields.insert(target.name.unwrap_or("").into(), value);
                Ok(())
            }
            ExpressExpressionKind::Index => {
                let owner_expression = target.operands.first().ok_or_else(|| error("ast", "Missing indexed assignment owner"))?;
                let owner = self.evaluate(owner_expression, scope)?;
                let coordinate = self.evaluate(target.operands.get(1).ok_or_else(|| error("ast", "Missing assignment index"))?, scope)?.integer().ok_or_else(|| error("assignment", "Integer assignment index required"))?;
                let at = coordinate.checked_sub(owner.lower()).and_then(|value| usize::try_from(value).ok()).filter(|index| *index < owner.items().len()).ok_or_else(|| error("assignment-index", "Aggregate assignment outside bounds"))?;
                let mut items = Vec::new();
                for (index, item) in owner.items().iter().enumerate() {
                    self.tick()?;
                    self.push_value(&mut items, if index == at { value.clone() } else { item.clone() })?;
                }
                let replacement = self.aggregate(items, owner.aggregate_kind(), owner.lower())?;
                self.assign(owner_expression, replacement, scope)
            }
            _ => Err(error("assignment", "Unsupported assignment target")),
        }
    }
}
