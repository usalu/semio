//! 🧭️ Shared bounded native MVD cursors borrow the original retained graph and static declarations.
use crate::validation::metadata;
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::{IfcEntity, IfcSnapshot, IfcValue};
use std::mem::size_of;
fn invalid(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, message)
}
#[derive(Clone, Copy, Debug)]
pub(super) struct Expression(pub &'static metadata::StaticTypeExpression);
impl std::ops::Deref for Expression {
    type Target = metadata::StaticTypeExpression;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.0, other.0)
    }
}
impl Eq for Expression {}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Owner {
    pub entity: usize,
    pub part: Option<usize>,
    pub argument: usize,
}
impl Owner {
    pub fn value(self, source: &IfcSnapshot) -> Option<&IfcValue> {
        let entity = source.entities.get(self.entity)?;
        self.part.and_then(|part| entity.complex.get(part).map(|part| part.args.as_slice())).or_else(|| self.part.is_none().then_some(entity.args.as_slice()))?.get(self.argument)
    }
}
pub(super) fn positive_id(value: &str) -> Result<u64, ValueError> {
    if value.is_empty() || value.len() > 20 || value.starts_with('0') {
        return Err(invalid("mvd.request"));
    }
    let mut id = 0u64;
    for byte in value.bytes() {
        if !byte.is_ascii_digit() {
            return Err(invalid("mvd.request"));
        }
        id = id.checked_mul(10).and_then(|id| id.checked_add((byte - b'0') as u64)).ok_or_else(|| invalid("mvd.request"))?;
    }
    Ok(id)
}
pub(super) fn component(entity: &IfcEntity, index: usize) -> (Option<usize>, &str, &[IfcValue]) {
    if index == 0 {
        (None, &entity.name, &entity.args)
    } else {
        let part = &entity.complex[index - 1];
        (Some(index - 1), &part.name, &part.args)
    }
}
pub(super) fn label_value(mut value: &IfcValue, wrappers: usize) -> Option<&IfcValue> {
    for _ in 0..wrappers {
        let IfcValue::TypedValue(typed) = value else {
            return None;
        };
        value = typed.items.first()?;
    }
    Some(value)
}
pub(super) fn valid_name_equal(value: &str, expected: &str) -> bool {
    value.len() == expected.len() && value.is_ascii() && value.eq_ignore_ascii_case(expected)
}
pub(super) fn owned(text: &str) -> Result<String, ValueError> {
    let mut value = String::new();
    value.try_reserve_exact(text.len()).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "mvd.allocation"))?;
    value.push_str(text);
    Ok(value)
}
pub(super) fn vector_growth<T>(length: usize, capacity: usize) -> Result<RetirementDemand, ValueError> {
    let grow = length == capacity;
    let next = if grow { capacity.checked_mul(2).unwrap_or(usize::MAX).max(8) } else { capacity };
    let bytes = next.checked_mul(size_of::<T>()).ok_or_else(|| invalid("mvd.capacity"))?;
    Ok(RetirementDemand {
        copy_bytes: 2 * size_of::<T>() + if grow { length * size_of::<T>() } else { 0 },
        capacity_bytes: if grow { bytes } else { 0 },
        release_bytes: if grow { capacity * size_of::<T>() } else { 0 },
        depth: 1,
        ..Default::default()
    })
}
pub(super) fn reserve<T>(values: &mut Vec<T>, bytes: usize) -> Result<(), ValueError> {
    if bytes > 0 {
        values.try_reserve_exact(bytes / size_of::<T>() - values.len()).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "mvd.allocation"))?;
    }
    Ok(())
}
pub(super) fn digit_length(mut value: usize) -> usize {
    let mut length = 1;
    while value >= 10 {
        value /= 10;
        length += 1;
    }
    length
}
pub(super) fn append_number(output: &mut String, mut value: usize) {
    let mut bytes = [0; 20];
    let mut offset = bytes.len();
    loop {
        offset -= 1;
        bytes[offset] = b'0' + (value % 10) as u8;
        value /= 10;
        if value == 0 {
            break;
        }
    }
    output.push_str(std::str::from_utf8(&bytes[offset..]).unwrap());
}
pub(super) fn number(value: usize) -> Result<String, ValueError> {
    let mut result = String::new();
    result.try_reserve_exact(digit_length(value)).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "mvd.allocation"))?;
    append_number(&mut result, value);
    Ok(result)
}
#[derive(Default)]
pub(super) struct Membership {
    position: usize,
    expected: &'static str,
    component: usize,
    ancestor: Option<&'static str>,
    ancestors: usize,
    accepted: bool,
    unknown: bool,
    done: bool,
    walking: bool,
}
impl Membership {
    pub fn matched(&self) -> bool {
        self.accepted
    }
    pub fn start(&mut self, position: usize, expected: &'static str) {
        self.position = position;
        self.expected = expected;
        self.component = 0;
        self.ancestor = None;
        self.ancestors = 0;
        self.accepted = false;
        self.unknown = false;
        self.done = false;
        self.walking = false;
    }
    pub fn copy_bound(&self) -> usize {
        4 * size_of::<Self>() + 256
    }
    pub fn result(&self) -> Option<bool> {
        if self.unknown { None } else { Some(self.accepted) }
    }
    pub fn advance(&mut self, source: &IfcSnapshot) -> bool {
        if self.done {
            return true;
        }
        let entity = &source.entities[self.position];
        if self.walking {
            if let Some(actual) = self.ancestor {
                if actual == self.expected {
                    self.accepted = true;
                    self.walking = false;
                    self.component += 1;
                } else if self.ancestors == 128 {
                    self.unknown = true;
                    self.walking = false;
                    self.component += 1;
                } else {
                    self.ancestors += 1;
                    self.ancestor = metadata::entity(actual).and_then(|declaration| declaration.supertype);
                }
            } else {
                self.walking = false;
                self.component += 1;
            }
        } else if self.component > entity.complex.len() {
            self.done = true;
        } else {
            let (_, name, _) = component(entity, self.component);
            if let Some(declaration) = metadata::entity(name) {
                self.ancestor = Some(declaration.key);
                self.ancestors = 0;
                self.walking = true;
            } else {
                self.unknown = true;
                self.component += 1;
            }
        }
        self.done
    }
}
#[derive(Clone, Copy)]
enum Root {
    Header(usize, usize),
    Argument(Owner),
}
pub(super) struct ShapeCursor {
    column: usize,
    entity: usize,
    component: usize,
    argument: usize,
    checked: bool,
    root: Option<Root>,
    path: [usize; 256],
    next: [usize; 257],
    depth: usize,
    values: usize,
    done: bool,
    invalid: bool,
}
impl Default for ShapeCursor {
    fn default() -> Self {
        Self { column: 0, entity: 0, component: 0, argument: 0, checked: false, root: None, path: [0; 256], next: [0; 257], depth: 0, values: 0, done: false, invalid: false }
    }
}
impl ShapeCursor {
    pub fn copy_bound(&self) -> usize {
        2 * size_of::<Self>() + 256 * size_of::<usize>()
    }
    pub fn valid(&self) -> bool {
        !self.invalid
    }
    pub fn complete(&self) -> bool {
        self.done
    }
    fn value<'a>(&self, source: &'a IfcSnapshot) -> Option<&'a IfcValue> {
        let mut value = match self.root? {
            Root::Header(column, index) => {
                let values = match column {
                    0 => &source.header.file_description,
                    1 => &source.header.file_name,
                    _ => &source.header.file_schema,
                };
                values.get(index)?
            }
            Root::Argument(owner) => owner.value(source)?,
        };
        for index in &self.path[..self.depth] {
            value = match value {
                IfcValue::Aggregate(items) => items.get(*index)?,
                IfcValue::TypedValue(typed) => typed.items.get(*index)?,
                _ => return None,
            };
        }
        Some(value)
    }
    pub fn advance(&mut self, source: &IfcSnapshot) -> bool {
        if self.done {
            return true;
        }
        if self.root.is_none() {
            if self.column < 3 {
                let values = match self.column {
                    0 => &source.header.file_description,
                    1 => &source.header.file_name,
                    _ => &source.header.file_schema,
                };
                if self.argument < values.len() {
                    self.root = Some(Root::Header(self.column, self.argument));
                    self.depth = 0;
                    self.next[0] = 0;
                } else {
                    self.column += 1;
                    self.argument = 0;
                }
            } else if let Some(entity) = source.entities.get(self.entity) {
                if !self.checked {
                    self.checked = true;
                    if entity.id == 0 {
                        self.invalid = true;
                        self.done = true;
                    }
                } else if self.component > entity.complex.len() {
                    self.entity += 1;
                    self.component = 0;
                    self.argument = 0;
                    self.checked = false;
                } else {
                    let (part, _, args) = component(entity, self.component);
                    if self.argument < args.len() {
                        self.root = Some(Root::Argument(Owner { entity: self.entity, part, argument: self.argument }));
                        self.depth = 0;
                        self.next[0] = 0;
                    } else {
                        self.component += 1;
                        self.argument = 0;
                    }
                }
            } else {
                self.done = true;
            }
        } else {
            if self.next[self.depth] == 0 {
                self.values += 1;
                if self.values > 10_000_000 {
                    self.invalid = true;
                    self.done = true;
                    return true;
                }
            }
            let value = self.value(source).unwrap();
            if matches!(value, IfcValue::Reference(0)) {
                self.invalid = true;
                self.done = true;
                return true;
            }
            let length = match value {
                IfcValue::Aggregate(items) => items.len(),
                IfcValue::TypedValue(typed) => typed.items.len(),
                _ => 0,
            };
            if self.next[self.depth] < length {
                if self.depth == self.path.len() {
                    self.invalid = true;
                    self.done = true;
                } else {
                    self.path[self.depth] = self.next[self.depth];
                    self.next[self.depth] += 1;
                    self.depth += 1;
                    self.next[self.depth] = 0;
                }
            } else if self.depth > 0 {
                self.depth -= 1;
            } else {
                self.root = None;
                self.argument += 1;
            }
        }
        self.done
    }
}
semio_framework_value::artifact_retire_leaf!(ShapeCursor, Membership);

/// 🔢️ Original consumers fund this bounded borrowed numeric cursor and retain the immutable source.
#[derive(Default)]
pub(super) struct PrimitiveZero {
    owner: Option<Owner>,
    expression: Option<Expression>,
    depth: usize,
    wrappers: usize,
    done: bool,
    result: Option<bool>,
}
impl PrimitiveZero {
    pub fn start(&mut self, owner: Owner, expression: Expression, wrappers: usize) {
        *self = Self { owner: Some(owner), expression: Some(expression), wrappers, ..Default::default() };
    }
    pub fn copy_bound(&self) -> usize {
        4 * size_of::<Self>() + 64 * size_of::<usize>() + 4 * 256
    }
    pub fn result(&self) -> Option<bool> {
        self.result
    }
    pub fn advance(&mut self, source: &IfcSnapshot) -> bool {
        if self.done {
            return true;
        }
        if self.depth == 64 || self.wrappers > 64 {
            self.done = true;
            return true;
        }
        let value = self.owner.and_then(|owner| owner.value(source)).and_then(|value| label_value(value, self.wrappers));
        let Some(expression) = self.expression else {
            self.done = true;
            return true;
        };
        let name = expression.name.as_deref().unwrap_or("");
        if expression.kind != "named" {
            self.done = true;
        } else if name == "REAL" {
            self.result = match value {
                Some(IfcValue::Integer(value)) => Some(*value == 0),
                Some(IfcValue::Real(value)) if value.is_finite() => Some(*value == 0.0),
                _ => None,
            };
            self.done = true;
        } else if let Some(declaration) = metadata::defined_type(name) {
            if declaration.kind != "defined" || declaration.type_expr.is_none() {
                self.done = true;
            } else if let Some(IfcValue::TypedValue(typed)) = value {
                if !valid_name_equal(&typed.name, name) || typed.items.len() != 1 {
                    self.done = true;
                } else {
                    self.wrappers += 1;
                    self.expression = declaration.type_expr.map(Expression);
                }
            } else {
                self.expression = declaration.type_expr.map(Expression);
            }
        } else {
            self.done = true;
        }
        self.depth += 1;
        self.done
    }
}
semio_framework_value::artifact_retire_leaf!(PrimitiveZero);
