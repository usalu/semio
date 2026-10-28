//! 🔗️ Original reference input and native declaration owners reduce without normalized event graphs.
use super::*;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, retained_clone_grant_admits};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use std::mem::size_of;
#[cfg(test)]
#[path = "reference_cursor_tests.rs"]
mod tests;
fn invalid(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, message)
}
fn upper<T>(name: &str, read: impl FnOnce(&str) -> T) -> Result<T, ValueError> {
    if name.len() > 256 {
        return Err(invalid("IFC reference declaration name exceeds its native extent"));
    }
    let mut bytes = [0; 256];
    bytes[..name.len()].copy_from_slice(name.as_bytes());
    bytes[..name.len()].make_ascii_uppercase();
    Ok(read(std::str::from_utf8(&bytes[..name.len()]).map_err(|_| invalid("IFC reference declaration name is invalid UTF-8"))?))
}
fn entity<C: IfcDetailsProfile>(name: &str) -> Result<&'static super::super::IfcEntityDeclaration, ValueError> {
    upper(name, |key| C::catalog().entities.get(key))?.ok_or_else(|| invalid("IFC reference entity declaration is absent"))
}
fn canonical<C: IfcDetailsProfile>(name: &str) -> Result<Option<&'static str>, ValueError> {
    upper(name, |key| C::catalog().entities.get_key_value(key).map(|(key, _)| key.as_str()).or_else(|| C::catalog().types.get_key_value(key).map(|(key, _)| key.as_str())))
}
#[derive(Clone, Copy, Debug)]
enum Expr {
    Declared(&'static IfcTypeExpression),
    Named(&'static str),
}
impl Expr {
    fn name(self) -> Option<&'static str> {
        match self {
            Self::Declared(expression) => expression.name.as_deref(),
            Self::Named(name) => Some(name),
        }
    }
}
#[derive(Clone, Copy, Debug)]
struct SearchFrame {
    expression: Expr,
    index: usize,
    actual: Option<&'static str>,
    phase: u8,
    ancestors: usize,
}
#[derive(Debug)]
struct TypeSearch {
    frames: [Option<SearchFrame>; 128],
    depth: usize,
    actual: Option<&'static str>,
    done: Option<bool>,
    typed: bool,
}
impl Default for TypeSearch {
    fn default() -> Self {
        Self { frames: [None; 128], depth: 0, actual: None, done: None, typed: false }
    }
}
impl TypeSearch {
    fn start(&mut self, expression: Expr, actual: Option<&'static str>) {
        self.depth = 1;
        self.actual = actual;
        self.done = None;
        self.typed = false;
        self.frames[0] = Some(SearchFrame { expression, index: 0, actual, phase: 0, ancestors: 0 });
    }
    fn start_typed(&mut self, expression: Expr, actual: &'static str) {
        self.start(expression, Some(actual));
        self.typed = true;
    }
    fn child(&mut self, expression: Expr) -> Result<(), ValueError> {
        if self.depth == 128 {
            return Err(invalid("IFC reference type search exceeds its declared depth"));
        }
        if expression.name().is_some_and(|name| self.frames[..self.depth].iter().flatten().any(|frame| frame.expression.name() == Some(name))) {
            return Err(invalid("IFC reference type declaration is cyclic"));
        }
        self.frames[self.depth] = Some(SearchFrame { expression, index: 0, actual: self.actual, phase: 0, ancestors: 0 });
        self.depth += 1;
        Ok(())
    }
    fn pop(&mut self) {
        self.depth -= 1;
        self.frames[self.depth] = None;
        if self.depth == 0 {
            self.done = Some(false);
        }
    }
    fn advance<C: IfcDetailsProfile>(&mut self) -> Result<Option<bool>, ValueError> {
        if self.done.is_some() {
            return Ok(self.done);
        }
        let top = self.depth.checked_sub(1).ok_or_else(|| invalid("IFC reference type search is absent"))?;
        let mut frame = self.frames[top].ok_or_else(|| invalid("IFC reference type frame is absent"))?;
        let name = frame.expression.name();
        match frame.phase {
            0 => {
                if self.typed && name == self.actual && name.is_some_and(|name| C::catalog().types.contains_key(name)) {
                    self.done = Some(true);
                } else if matches!(frame.expression,Expr::Declared(expression)if expression.kind=="aggregate") {
                    self.pop();
                } else if let Some(name) = name {
                    if C::catalog().entities.contains_key(name) {
                        if self.actual.is_none() {
                            self.done = Some(true);
                        } else {
                            frame.phase = 2;
                            self.frames[top] = Some(frame);
                        }
                    } else if let Some(declaration) = C::catalog().types.get(name) {
                        if declaration.kind == "defined" {
                            if self.typed {
                                self.pop();
                            } else {
                                self.child(Expr::Declared(declaration.type_expr.as_ref().ok_or_else(|| invalid("IFC defined reference type has no expression"))?))?;
                                frame.phase = 3;
                                self.frames[top] = Some(frame);
                            }
                        } else if declaration.kind == "select" {
                            frame.phase = 1;
                            self.frames[top] = Some(frame);
                        } else {
                            self.pop();
                        }
                    } else {
                        self.pop();
                    }
                } else {
                    self.pop();
                }
            }
            1 => {
                let declaration = C::catalog().types.get(name.ok_or_else(|| invalid("IFC SELECT reference name is absent"))?).ok_or_else(|| invalid("IFC SELECT declaration is absent"))?;
                if let Some(member) = declaration.members.get(frame.index) {
                    frame.index += 1;
                    self.frames[top] = Some(frame);
                    self.child(Expr::Named(member))?;
                } else {
                    self.pop();
                }
            }
            2 => {
                if let Some(actual) = frame.actual {
                    if Some(actual) == name {
                        self.done = Some(true);
                    } else {
                        if frame.ancestors == 128 {
                            return Err(invalid("IFC reference subtype chain exceeds its declared depth"));
                        }
                        frame.ancestors += 1;
                        frame.actual = C::catalog().entities.get(actual).and_then(|declaration| declaration.supertype.as_deref());
                        self.frames[top] = Some(frame);
                    }
                } else {
                    self.pop();
                }
            }
            _ => self.pop(),
        }
        Ok(self.done)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ReferenceKind {
    Whole,
    Payload,
    Virtual,
    Append,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct ReferenceRequest {
    pub kind: ReferenceKind,
    pub owner_end: usize,
    pub id: Option<u64>,
}
impl ReferenceRequest {
    pub(super) fn value(self) -> IfcValue {
        self.id.map_or(IfcValue::Unset, IfcValue::Reference)
    }
}
#[derive(Clone, Copy, Default, Debug)]
#[repr(u8)]
enum Phase {
    #[default]
    Init,
    Derived,
    Resolve,
    Route,
    Search,
    TypedMembership,
    Input,
    Digits,
    Unique,
    Targets,
    TargetType,
    Done,
}
#[derive(Debug)]
pub(super) struct ReferenceReduction {
    binding: Option<(usize, usize, usize)>,
    key_copied: usize,
    address: Option<pointer::NativeAddress>,
    attribute: Option<&'static IfcAttributeDeclaration>,
    expression: Option<Expr>,
    owner_expression: Option<Expr>,
    typed_name: Option<&'static str>,
    typed_index: usize,
    typed_next: usize,
    indices: [usize; 128],
    depth: usize,
    field: usize,
    owner_end: usize,
    derive: usize,
    derive_attribute: usize,
    derived: bool,
    optional: bool,
    kind: Option<ReferenceKind>,
    search: TypeSearch,
    id: Option<u64>,
    digits: usize,
    number: u64,
    unique_parent: Option<usize>,
    selected: Option<usize>,
    unique: bool,
    scan: usize,
    target: usize,
    part: usize,
    matches: usize,
    accepted: bool,
    pub(super) request: Option<ReferenceRequest>,
    phase: Phase,
    done: bool,
    closing: bool,
}
impl Default for ReferenceReduction {
    fn default() -> Self {
        Self {
            binding: None,
            key_copied: 0,
            address: None,
            attribute: None,
            expression: None,
            owner_expression: None,
            typed_name: None,
            typed_index: 0,
            typed_next: 0,
            indices: [0; 128],
            depth: 0,
            field: 0,
            owner_end: 0,
            derive: 0,
            derive_attribute: 0,
            derived: false,
            optional: false,
            kind: None,
            search: Default::default(),
            id: None,
            digits: 0,
            number: 0,
            unique_parent: None,
            selected: None,
            unique: false,
            scan: 0,
            target: 0,
            part: 0,
            matches: 0,
            accepted: false,
            request: None,
            phase: Phase::Init,
            done: false,
            closing: false,
        }
    }
}
impl ReferenceReduction {
    pub(super) fn complete(&self) -> bool {
        self.done
    }
    pub(super) fn demands(&self) -> RetirementDemand {
        RetirementDemand { copy_bytes: self.metadata_bytes() + if matches!(self.phase, Phase::Init | Phase::Derived | Phase::Route | Phase::Targets) { 256 } else { 0 }, depth: 1, ..Default::default() }
    }
    fn metadata_bytes(&self) -> usize {
        2 * size_of::<SearchFrame>() + 12 * size_of::<usize>() + 2 * size_of::<Option<Expr>>() + size_of::<Option<pointer::NativeAddress>>() + size_of::<ReferenceRequest>() + size_of::<Option<(usize, usize, usize)>>() + size_of::<Phase>()
    }
    fn copy_key(&mut self, name: &str) {
        if name.len() <= 256 {
            self.key_copied += name.len();
        }
    }
    fn original<'a>(&self, snapshot: &'a IfcSnapshot, depth: usize) -> Result<&'a IfcValue, ValueError> {
        let address = self.address.ok_or_else(|| invalid("IFC reference native address is absent"))?;
        let entity = snapshot.entities.get(address.position).ok_or_else(|| invalid("IFC reference source entity is absent"))?;
        let args = if let Some(part) = address.part { &entity.complex.get(part).ok_or_else(|| invalid("IFC reference source constituent is absent"))?.args } else { &entity.args };
        let mut value = args.get(address.argument).ok_or_else(|| invalid("IFC reference source argument is absent"))?;
        for index in &self.indices[..depth] {
            value = match value {
                IfcValue::Aggregate(items) => items.get(*index),
                IfcValue::TypedValue(typed) => typed.items.get(*index),
                _ => None,
            }
            .ok_or_else(|| invalid("IFC reference source child is absent"))?;
        }
        Ok(value)
    }
    fn resolve<C: IfcDetailsProfile>(&mut self) -> Result<bool, ValueError> {
        let expression = self.expression.ok_or_else(|| invalid("IFC reference expression is absent"))?;
        if let Some(declaration) = expression.name().and_then(|name| C::catalog().types.get(name)).filter(|declaration| declaration.kind == "defined") {
            if self.scan == 128 {
                return Err(invalid("IFC reference aliases exceed their declared depth"));
            }
            self.scan += 1;
            self.expression = Some(Expr::Declared(declaration.type_expr.as_ref().ok_or_else(|| invalid("IFC reference alias expression is absent"))?));
            return Ok(false);
        }
        Ok(true)
    }
    fn begin_search(&mut self) {
        self.search.start(self.expression.unwrap(), None);
        self.phase = Phase::Search;
    }
    fn inner<C: IfcDetailsProfile>(&mut self, event: &editing::SnapshotEditEvent, snapshot: &IfcSnapshot, addressing: &pointer::PathAdmission) -> Result<(), ValueError> {
        let editing::SnapshotEditEvent::SetValue { path, value } = event else {
            self.done = true;
            self.phase = Phase::Done;
            return Ok(());
        };
        match self.phase {
            Phase::Init => {
                let Some(address) = addressing.native_address(event) else {
                    self.done = true;
                    self.phase = Phase::Done;
                    return Ok(());
                };
                let Some(source) = snapshot.entities.get(address.position) else {
                    self.done = true;
                    self.phase = Phase::Done;
                    return Ok(());
                };
                let (name, args, direct) = if let Some(part) = address.part {
                    let Some(part) = source.complex.get(part) else {
                        self.done = true;
                        self.phase = Phase::Done;
                        return Ok(());
                    };
                    (part.name.as_str(), part.args.as_slice(), true)
                } else {
                    (source.name.as_str(), source.args.as_slice(), !source.complex.is_empty())
                };
                if args.get(address.argument).is_none() {
                    self.done = true;
                    self.phase = Phase::Done;
                    return Ok(());
                }
                self.copy_key(name);
                let declaration = entity::<C>(name)?;
                let attributes = if direct { &declaration.attributes } else { &declaration.all_attributes };
                let Some(attribute) = attributes.get(address.argument) else {
                    self.done = true;
                    self.phase = Phase::Done;
                    return Ok(());
                };
                self.address = Some(address);
                self.field = address.field;
                self.owner_end = address.prefix;
                self.attribute = Some(attribute);
                self.expression = Some(Expr::Declared(&attribute.type_expr));
                self.owner_expression = self.expression;
                self.optional = attribute.optional || attribute.type_expr.optional;
                self.derived = attribute.derived;
                self.phase = Phase::Derived;
            }
            Phase::Derived => {
                let source = &snapshot.entities[self.address.unwrap().position];
                if self.derive > source.complex.len() {
                    self.phase = Phase::Resolve;
                    self.scan = 0;
                } else {
                    let name = if self.derive == 0 { &source.name } else { &source.complex[self.derive - 1].name };
                    self.copy_key(name);
                    let declaration = entity::<C>(name)?;
                    if let Some(other) = declaration.all_attributes.get(self.derive_attribute) {
                        let attribute = self.attribute.unwrap();
                        self.derived |= other.derived && other.name == attribute.name && other.declared_by == attribute.declared_by;
                        self.derive_attribute += 1;
                    } else {
                        self.derive += 1;
                        self.derive_attribute = 0;
                    }
                }
            }
            Phase::Resolve => {
                if self.resolve::<C>()? {
                    self.phase = Phase::Route;
                }
            }
            Phase::Route => {
                if self.kind == Some(ReferenceKind::Append) {
                    self.begin_search();
                    return Ok(());
                }
                let current = self.original(snapshot, self.depth)?;
                let count = addressing.field_count();
                if self.field == count {
                    self.kind = Some(ReferenceKind::Whole);
                    self.begin_search();
                } else if self.field + 1 == count && addressing.field_is(path, self.field, "reference") {
                    self.kind = Some(ReferenceKind::Virtual);
                    self.begin_search();
                } else if self.field + 1 == count && addressing.field_is(path, self.field, "appendReference") {
                    let IfcValue::Aggregate(items) = current else {
                        return Err(invalid("IFC reference append requires its native aggregate"));
                    };
                    let Expr::Declared(domain) = self.expression.unwrap() else {
                        return Err(invalid("IFC reference append requires its aggregate declaration"));
                    };
                    if domain.kind != "aggregate" || domain.aggregate.as_deref() == Some("ARRAY") || domain.upper.is_some_and(|upper| items.len() >= upper.max(0) as usize) {
                        return Err(invalid("IFC reference append exceeds its declared collection capacity"));
                    }
                    self.unique = domain.unique || domain.aggregate.as_deref() == Some("SET");
                    self.unique_parent = Some(self.depth);
                    self.selected = None;
                    self.optional = false;
                    self.expression = Some(Expr::Declared(domain.item.as_deref().ok_or_else(|| invalid("IFC reference aggregate item declaration is absent"))?));
                    self.kind = Some(ReferenceKind::Append);
                    self.scan = 0;
                    self.phase = Phase::Resolve;
                } else if !addressing.field_is(path, self.field, "value") {
                    self.done = true;
                    self.phase = Phase::Done;
                } else if self.field + 1 == count {
                    if matches!(current, IfcValue::Reference(_)) {
                        self.kind = Some(ReferenceKind::Payload);
                        self.begin_search();
                    } else {
                        self.done = true;
                        self.phase = Phase::Done;
                    }
                } else {
                    let (domain, index, next, optional) = match current {
                        IfcValue::Aggregate(_) => {
                            let Expr::Declared(domain) = self.expression.unwrap() else {
                                return Err(invalid("IFC reference aggregate expression is absent"));
                            };
                            if domain.kind != "aggregate" {
                                self.done = true;
                                self.phase = Phase::Done;
                                return Ok(());
                            }
                            let index = addressing.field_index(path, self.field + 1).ok_or_else(|| invalid("IFC reference item index is invalid"))?;
                            self.unique_parent = Some(self.depth);
                            self.selected = Some(index);
                            self.unique = domain.unique || domain.aggregate.as_deref() == Some("SET");
                            let item = domain.item.as_deref().ok_or_else(|| invalid("IFC reference item declaration is absent"))?;
                            (Expr::Declared(item), index, self.field + 2, item.optional || domain.optional)
                        }
                        IfcValue::TypedValue(typed) if addressing.field_is(path, self.field + 1, "items") && self.field + 2 < count => {
                            if typed.items.len() != 1 {
                                return Err(invalid("IFC typed reference constructor requires one item"));
                            }
                            let index = addressing.field_index(path, self.field + 2).ok_or_else(|| invalid("IFC typed reference item index is invalid"))?;
                            self.copy_key(&typed.name);
                            let Some(name) = canonical::<C>(&typed.name)? else {
                                return Err(invalid("IFC typed reference declaration is absent"));
                            };
                            if !C::catalog().types.contains_key(name) {
                                return Err(invalid("IFC typed reference constructor cannot use an entity name"));
                            }
                            self.typed_name = Some(name);
                            self.typed_index = index;
                            self.typed_next = self.field + 3;
                            self.search.start_typed(self.owner_expression.unwrap(), name);
                            self.phase = Phase::TypedMembership;
                            return Ok(());
                        }
                        _ => {
                            self.done = true;
                            self.phase = Phase::Done;
                            return Ok(());
                        }
                    };
                    if self.depth == 128 {
                        return Err(invalid("IFC reference route exceeds its declared depth"));
                    }
                    self.indices[self.depth] = index;
                    self.depth += 1;
                    self.field = next;
                    self.owner_end = addressing.field_end(next - 1).ok_or_else(|| invalid("IFC reference owner extent is absent"))?;
                    self.expression = Some(domain);
                    self.owner_expression = self.expression;
                    self.optional = optional;
                    self.scan = 0;
                    self.phase = Phase::Resolve;
                }
            }
            Phase::TypedMembership => {
                if let Some(accepted) = self.search.advance::<C>()? {
                    if !accepted {
                        return Err(invalid("IFC typed reference constructor is outside its declared SELECT or defined domain"));
                    }
                    if self.depth == 128 {
                        return Err(invalid("IFC typed reference route exceeds its declared depth"));
                    }
                    self.indices[self.depth] = self.typed_index;
                    self.depth += 1;
                    self.field = self.typed_next;
                    self.owner_end = addressing.field_end(self.field - 1).ok_or_else(|| invalid("IFC typed reference owner extent is absent"))?;
                    self.expression = Some(Expr::Named(self.typed_name.unwrap()));
                    self.owner_expression = self.expression;
                    self.optional = false;
                    self.unique_parent = None;
                    self.selected = None;
                    self.unique = false;
                    self.scan = 0;
                    self.phase = Phase::Resolve;
                }
            }
            Phase::Search => {
                if let Some(accepted) = self.search.advance::<C>()? {
                    if !accepted {
                        if matches!(self.kind, Some(ReferenceKind::Virtual | ReferenceKind::Append)) {
                            return Err(invalid("IFC virtual reference requires an entity reference declaration"));
                        }
                        self.done = true;
                        self.phase = Phase::Done;
                    } else {
                        if self.derived {
                            return Err(invalid("IFC derived reference is read only"));
                        }
                        self.scan = 0;
                        self.phase = Phase::Input;
                    }
                }
            }
            Phase::Input => {
                if self.kind == Some(ReferenceKind::Whole) {
                    let DslValue::Object(fields) = value else {
                        return Err(invalid("IFC reference slot requires a closed native reference wrapper"));
                    };
                    if fields.len() > 2 {
                        return Err(invalid("IFC reference wrapper has unknown fields"));
                    }
                    match value.get("kind").and_then(DslValue::as_str) {
                        Some("unset") if fields.len() == 1 => self.id = None,
                        Some("reference") if fields.len() == 2 => self.id = Some(value.get("value").and_then(DslValue::as_u64).filter(|id| *id > 0).ok_or_else(|| invalid("IFC native reference requires an exact positive integer"))?),
                        _ => return Err(invalid("IFC reference wrapper requires reference or optional unset")),
                    }
                    self.phase = Phase::Unique;
                } else {
                    let DslValue::String(text) = value else {
                        return Err(invalid("IFC reference input requires its original exact decimal text"));
                    };
                    if text.is_empty() {
                        if self.kind != Some(ReferenceKind::Virtual) {
                            return Err(invalid("IFC required reference input cannot be empty"));
                        }
                        self.id = None;
                        self.phase = Phase::Unique;
                    } else {
                        if text.len() > 20 || text.starts_with('0') {
                            return Err(invalid("IFC reference decimal is outside its canonical positive extent"));
                        }
                        self.phase = Phase::Digits;
                    }
                }
            }
            Phase::Digits => {
                let DslValue::String(text) = value else {
                    return Err(invalid("IFC reference decimal owner changed"));
                };
                if let Some(byte) = text.as_bytes().get(self.digits) {
                    if !byte.is_ascii_digit() {
                        return Err(invalid("IFC reference decimal has a nonnumeric byte"));
                    }
                    self.number = self.number.checked_mul(10).and_then(|number| number.checked_add(u64::from(*byte - b'0'))).ok_or_else(|| invalid("IFC reference decimal exceeds u64"))?;
                    self.digits += 1;
                } else {
                    self.id = Some(self.number);
                    self.phase = Phase::Unique;
                }
            }
            Phase::Unique => {
                if self.id.is_none() {
                    if !self.optional {
                        return Err(invalid("IFC required reference cannot be unset"));
                    }
                    self.request = Some(ReferenceRequest { kind: self.kind.unwrap(), owner_end: self.owner_end, id: None });
                    self.done = true;
                    self.phase = Phase::Done;
                } else if self.unique {
                    let Some(depth) = self.unique_parent else {
                        return Err(invalid("IFC reference uniqueness owner is absent"));
                    };
                    let IfcValue::Aggregate(items) = self.original(snapshot, depth)? else {
                        return Err(invalid("IFC reference uniqueness aggregate is absent"));
                    };
                    if let Some(item) = items.get(self.scan) {
                        if self.selected != Some(self.scan) && item.as_reference() == self.id && (self.kind == Some(ReferenceKind::Append) || self.original(snapshot, self.depth)?.as_reference() != self.id) {
                            return Err(invalid("IFC SET or UNIQUE reference duplicates an existing target"));
                        }
                        self.scan += 1;
                    } else {
                        self.phase = Phase::Targets;
                    }
                } else {
                    self.phase = Phase::Targets;
                }
            }
            Phase::Targets => {
                if self.target == snapshot.entities.len() {
                    if self.matches != 1 || !self.accepted {
                        return Err(invalid("IFC reference requires exactly one existing target of an admitted type"));
                    }
                    self.request = Some(ReferenceRequest { kind: self.kind.unwrap(), owner_end: self.owner_end, id: self.id });
                    self.done = true;
                    self.phase = Phase::Done;
                } else {
                    let target = &snapshot.entities[self.target];
                    if Some(target.id) != self.id {
                        self.target += 1;
                        self.part = 0;
                    } else if self.part > target.complex.len() {
                        self.matches += 1;
                        self.target += 1;
                        self.part = 0;
                    } else {
                        let name = if self.part == 0 { &target.name } else { &target.complex[self.part - 1].name };
                        self.copy_key(name);
                        let actual = canonical::<C>(name)?.ok_or_else(|| invalid("IFC reference target type declaration is absent"))?;
                        self.search.start(self.expression.unwrap(), Some(actual));
                        self.phase = Phase::TargetType;
                    }
                }
            }
            Phase::TargetType => {
                if let Some(accepted) = self.search.advance::<C>()? {
                    self.accepted |= accepted;
                    self.part += 1;
                    self.phase = Phase::Targets;
                }
            }
            Phase::Done => {}
        }
        Ok(())
    }
    pub(super) fn advance<C: IfcDetailsProfile>(
        &mut self,
        event: &editing::SnapshotEditEvent,
        snapshot: &IfcSnapshot,
        addressing: &pointer::PathAdmission,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if cx.is_cancelled() {
            return Err(ValueError::literal(ValueRefusalKind::Canceled, "IFC original reference reduction cancelled").into());
        }
        if self.closing {
            return Err(invalid("IFC reference reduction is closing").into());
        }
        let value = match event {
            editing::SnapshotEditEvent::SetValue { value, .. } => value as *const DslValue as usize,
            _ => 0,
        };
        let binding = (event as *const _ as usize, snapshot as *const _ as usize, value);
        if self.binding.is_some_and(|original| original != binding) {
            return Err(invalid("IFC reference reduction original owners changed").into());
        }
        let demand = self.demands();
        if cx.should_yield() || !retained_clone_grant_admits(cx.retained_grant(), demand) {
            return Ok(false);
        }
        if self.done {
            return Ok(true);
        }
        self.binding = Some(binding);
        self.key_copied = 0;
        let result = self.inner::<C>(event, snapshot, addressing);
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: self.metadata_bytes() + self.key_copied, ..Default::default() })?;
        cx.consume_fuel(1);
        result?;
        Ok(self.done)
    }
    pub(super) fn begin_close(&mut self) {
        self.closing = true;
    }
    pub(super) fn terminal_is_empty(&self) -> bool {
        self.binding.is_none()
    }
    pub(super) fn close_demands(&self) -> RetirementDemand {
        RetirementDemand { copy_bytes: size_of::<Option<(usize, usize, usize)>>(), depth: 1, ..Default::default() }
    }
    pub(super) fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.closing {
            return Err(invalid("IFC reference close was not started"));
        }
        let demand = self.close_demands();
        if !retained_clone_grant_admits(grant, demand) {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        self.binding = None;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))
    }
}
