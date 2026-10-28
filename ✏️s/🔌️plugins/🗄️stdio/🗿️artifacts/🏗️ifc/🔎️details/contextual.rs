//! 🎨️ Original-funded enumeration metadata resolves complete borrowed typed choices before rendering.
use super::*;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, retained_clone_grant_admits};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use std::mem::size_of;
#[cfg(test)]
#[path = "contextual_tests.rs"]
mod tests;
const NONE: usize = usize::MAX;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IfcEnumerationAddress {
    pub entity: usize,
    pub part: Option<usize>,
    pub argument: Option<usize>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IfcEnumerationDiagnostic {
    Unset,
    Missing,
    WrongKind,
    Ambiguous,
    Empty,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct State {
    binding: Option<(usize, usize, usize, u64)>,
    index: Option<(usize, usize)>,
    owner: usize,
    part: Option<usize>,
    requested: usize,
    values: usize,
    reference: usize,
    attribute: usize,
    row: usize,
    matches: usize,
    owner_matches: usize,
    target: usize,
    component: usize,
    target_part: Option<usize>,
    components: usize,
    target_values: usize,
    phase: u8,
    diagnostic: Option<IfcEnumerationDiagnostic>,
}
impl Default for State {
    fn default() -> Self {
        Self {
            binding: None,
            index: None,
            owner: 0,
            part: None,
            requested: 0,
            values: NONE,
            reference: NONE,
            attribute: 0,
            row: 0,
            matches: 0,
            owner_matches: 0,
            target: NONE,
            component: 0,
            target_part: None,
            components: 0,
            target_values: NONE,
            phase: 0,
            diagnostic: None,
        }
    }
}
#[derive(Default, Debug)]
pub struct IfcEnumerationQuery {
    state: State,
    closing: bool,
}
pub struct IfcEnumerationView<'a> {
    pub property: u64,
    pub enumeration: Option<u64>,
    pub revision: u64,
    pub selection: &'a IfcValue,
    pub allowed: Option<&'a [IfcValue]>,
    pub diagnostic: Option<IfcEnumerationDiagnostic>,
}
#[derive(Clone, Copy)]
pub(super) struct IfcEnumerationEntry {
    binding: (usize, u64),
    address: IfcEnumerationAddress,
    target: usize,
    target_part: Option<usize>,
    target_values: usize,
    diagnostic: Option<IfcEnumerationDiagnostic>,
}
impl IfcEnumerationEntry {
    pub(super) fn address(&self) -> IfcEnumerationAddress {
        self.address
    }
    pub(super) fn view<'a>(&self, snapshot: &'a IfcSnapshot, revision: u64) -> Result<IfcEnumerationView<'a>, ValueError> {
        if self.binding != (snapshot as *const _ as usize, revision) {
            return Err(invalid("IFC enumeration entry original source changed"));
        }
        let selection = args(snapshot, self.address.entity, self.address.part)?.get(self.address.argument.unwrap()).ok_or_else(|| invalid("IFC enumeration entry selection is absent"))?;
        let allowed = if self.diagnostic.is_none() { args(snapshot, self.target, self.target_part)?.get(self.target_values).and_then(IfcValue::as_aggregate) } else { None };
        Ok(IfcEnumerationView { property: snapshot.entities[self.address.entity].id, enumeration: (self.target != NONE).then(|| snapshot.entities[self.target].id), revision, selection, allowed, diagnostic: self.diagnostic })
    }
}
semio_framework_value::artifact_retire_leaf!(IfcEnumerationQuery, IfcEnumerationEntry);
fn invalid(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, message)
}
fn args(snapshot: &IfcSnapshot, owner: usize, part: Option<usize>) -> Result<&[IfcValue], ValueError> {
    let entity = snapshot.entities.get(owner).ok_or_else(|| invalid("IFC enumeration original property is absent"))?;
    Ok(if let Some(part) = part { &entity.complex.get(part).ok_or_else(|| invalid("IFC enumeration original constituent is absent"))?.args } else { &entity.args })
}
fn attributes<C: IfcDetailsProfile>(name: &str, partial: bool, copied: &mut usize) -> Result<&'static [IfcAttributeDeclaration], ValueError> {
    if name.len() > 128 || !name.is_ascii() {
        return Err(invalid("IFC enumeration declaration name is outside its native extent"));
    }
    let mut bytes = [0u8; 128];
    bytes[..name.len()].copy_from_slice(name.as_bytes());
    bytes[..name.len()].make_ascii_uppercase();
    *copied += 128 + 2 * name.len();
    let name = std::str::from_utf8(&bytes[..name.len()]).map_err(|_| invalid("IFC enumeration declaration is invalid"))?;
    let declaration = C::catalog().entities.get(name).ok_or_else(|| invalid("IFC enumeration declaration is unavailable"))?;
    Ok(if partial { &declaration.attributes } else { &declaration.all_attributes })
}
fn unavailable(state: &mut State, diagnostic: IfcEnumerationDiagnostic) {
    state.diagnostic = Some(diagnostic);
    state.phase = 8;
}
impl IfcEnumerationQuery {
    pub fn demands(&self) -> RetirementDemand {
        RetirementDemand { copy_bytes: 3 * size_of::<State>() + 384, depth: 1, ..Default::default() }
    }
    fn step<C: IfcDetailsProfile>(state: &mut State, snapshot: &IfcSnapshot, address: IfcEnumerationAddress, copied: &mut usize, index: Option<&source_index::IfcSourceIndex>) -> Result<(), ValueError> {
        match state.phase {
            0 => {
                state.owner = address.entity;
                state.part = address.part;
                state.requested = address.argument.unwrap_or(NONE);
                let entity = snapshot.entities.get(address.entity).ok_or_else(|| invalid("IFC enumeration property is absent"))?;
                let (name, partial) = if let Some(part) = address.part { (&entity.complex.get(part).ok_or_else(|| invalid("IFC enumeration constituent is absent"))?.name, true) } else { (&entity.name, !entity.complex.is_empty()) };
                if !name.eq_ignore_ascii_case("IFCPROPERTYENUMERATEDVALUE") {
                    return Err(invalid("IFC enumeration requires an actual enumerated property"));
                }
                let _ = attributes::<C>(name, partial, copied)?;
                state.phase = 1;
            }
            1 => {
                let entity = &snapshot.entities[state.owner];
                let (name, partial) = state.part.map_or((&entity.name, !entity.complex.is_empty()), |part| (&entity.complex[part].name, true));
                let attributes = attributes::<C>(name, partial, copied)?;
                if let Some(attribute) = attributes.get(state.attribute) {
                    if attribute.name == "EnumerationValues" {
                        state.values = state.attribute;
                    }
                    if attribute.name == "EnumerationReference" {
                        state.reference = state.attribute;
                    }
                    state.attribute += 1;
                } else {
                    if state.requested != NONE && state.values != state.requested || state.reference == NONE || args(snapshot, state.owner, state.part)?.get(state.values).is_none() {
                        return Err(invalid("IFC enumeration does not address its original values"));
                    }
                    state.phase = 2;
                }
            }
            2 => match args(snapshot, state.owner, state.part)?.get(state.reference) {
                None | Some(IfcValue::Unset) => unavailable(state, IfcEnumerationDiagnostic::Unset),
                Some(IfcValue::Reference(id)) if *id > 0 => state.phase = 3,
                _ => unavailable(state, IfcEnumerationDiagnostic::WrongKind),
            },
            3 => {
                if let Some(index) = index {
                    use source_index::IdMatch;
                    let reference = args(snapshot, state.owner, state.part)?.get(state.reference).and_then(IfcValue::as_reference).ok_or_else(|| invalid("IFC original enumeration index reference is absent"))?;
                    if !matches!(index.find(snapshot.entities[state.owner].id), IdMatch::Unique(_)) {
                        unavailable(state, IfcEnumerationDiagnostic::Ambiguous);
                    } else {
                        match index.find(reference) {
                            IdMatch::Missing => unavailable(state, IfcEnumerationDiagnostic::Missing),
                            IdMatch::Ambiguous => unavailable(state, IfcEnumerationDiagnostic::Ambiguous),
                            IdMatch::Unique(target) => {
                                state.target = target;
                                state.phase = 4;
                            }
                        }
                    }
                    return Ok(());
                }
                if let Some(entity) = snapshot.entities.get(state.row) {
                    if entity.id == snapshot.entities[state.owner].id {
                        state.owner_matches += 1;
                    }
                    if let Some(IfcValue::Reference(id)) = args(snapshot, state.owner, state.part)?.get(state.reference) {
                        if entity.id == *id {
                            state.matches += 1;
                            state.target = state.row;
                        }
                    }
                    state.row += 1;
                } else if state.matches > 1 || state.owner_matches != 1 {
                    unavailable(state, IfcEnumerationDiagnostic::Ambiguous);
                } else if state.matches == 0 {
                    unavailable(state, IfcEnumerationDiagnostic::Missing);
                } else {
                    state.phase = 4;
                }
            }
            4 => {
                let target = &snapshot.entities[state.target];
                if state.component > target.complex.len() {
                    if state.components == 0 {
                        unavailable(state, IfcEnumerationDiagnostic::WrongKind);
                    } else if state.components > 1 {
                        unavailable(state, IfcEnumerationDiagnostic::Ambiguous);
                    } else {
                        state.attribute = 0;
                        state.phase = 5;
                    }
                } else {
                    let (part, name) = if state.component == 0 { (None, &target.name) } else { (Some(state.component - 1), &target.complex[state.component - 1].name) };
                    if name.eq_ignore_ascii_case("IFCPROPERTYENUMERATION") {
                        state.components += 1;
                        state.target_part = part;
                    }
                    state.component += 1;
                }
            }
            5 => {
                let target = &snapshot.entities[state.target];
                let (name, partial) = state.target_part.map_or((&target.name, !target.complex.is_empty()), |part| (&target.complex[part].name, true));
                let attributes = attributes::<C>(name, partial, copied)?;
                if let Some(attribute) = attributes.get(state.attribute) {
                    if attribute.name == "EnumerationValues" {
                        state.target_values = state.attribute;
                    }
                    state.attribute += 1;
                } else {
                    state.phase = 6;
                }
            }
            6 => match args(snapshot, state.target, state.target_part)?.get(state.target_values) {
                Some(IfcValue::Aggregate(values)) if !values.is_empty() => state.phase = 8,
                Some(IfcValue::Aggregate(_)) => unavailable(state, IfcEnumerationDiagnostic::Empty),
                _ => unavailable(state, IfcEnumerationDiagnostic::WrongKind),
            },
            _ => {}
        };
        Ok(())
    }
    pub fn advance<C: IfcDetailsProfile>(&mut self, snapshot: &IfcSnapshot, path: &[P], revision: u64, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if path.len() < 4 || path.len() > 128 || !key(&path[0], "entities") {
            return Err(invalid("IFC enumeration original path is invalid").into());
        }
        let P::Index(entity) = path[1] else {
            return Err(invalid("IFC enumeration original entity ordinal is absent").into());
        };
        let (part, argument) = if key(&path[2], "complex") {
            if path.len() < 6 || !key(&path[4], "args") {
                return Err(invalid("IFC enumeration original constituent path is invalid").into());
            }
            let (P::Index(part), P::Index(argument)) = (&path[3], &path[5]) else {
                return Err(invalid("IFC enumeration original constituent ordinal is absent").into());
            };
            (Some(*part), *argument)
        } else {
            if !key(&path[2], "args") {
                return Err(invalid("IFC enumeration original argument path is absent").into());
            }
            let P::Index(argument) = path[3] else {
                return Err(invalid("IFC enumeration original argument ordinal is absent").into());
            };
            (None, argument)
        };
        self.advance_bound::<C>(snapshot, IfcEnumerationAddress { entity, part, argument: Some(argument) }, (snapshot as *const _ as usize, path.as_ptr() as usize, path.len(), revision), None, cx)
    }
    pub fn advance_address<C: IfcDetailsProfile>(&mut self, snapshot: &IfcSnapshot, address: IfcEnumerationAddress, revision: u64, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if self.state.binding.is_some() && (self.state.owner != address.entity || self.state.part != address.part || self.state.requested != address.argument.unwrap_or(NONE)) {
            return Err(invalid("IFC enumeration source address changed").into());
        }
        self.advance_bound::<C>(snapshot, address, (snapshot as *const _ as usize, address.entity, address.argument.unwrap_or(NONE), revision), None, cx)
    }
    pub(super) fn advance_indexed<C: IfcDetailsProfile>(
        &mut self,
        snapshot: &IfcSnapshot,
        address: IfcEnumerationAddress,
        revision: u64,
        index: &source_index::IfcSourceIndex,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if self.state.binding.is_some() && (self.state.owner != address.entity || self.state.part != address.part || self.state.requested != address.argument.unwrap_or(NONE)) {
            return Err(invalid("IFC enumeration indexed source address changed").into());
        }
        self.advance_bound::<C>(snapshot, address, (snapshot as *const _ as usize, address.entity, address.argument.unwrap_or(NONE), revision), Some(index), cx)
    }
    fn advance_bound<C: IfcDetailsProfile>(
        &mut self,
        snapshot: &IfcSnapshot,
        address: IfcEnumerationAddress,
        binding: (usize, usize, usize, u64),
        index: Option<&source_index::IfcSourceIndex>,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if cx.is_cancelled() {
            return Err(ValueError::literal(ValueRefusalKind::Canceled, "IFC original enumeration query cancelled").into());
        }
        if self.closing {
            return Err(invalid("IFC enumeration query is closing").into());
        }
        if self.state.binding.is_some_and(|original| original != binding) {
            return Err(invalid("IFC enumeration original source or revision changed").into());
        }
        let index_binding = index.map(|index| index.identity(snapshot, binding.3)).transpose()?;
        if self.state.binding.is_some() && self.state.index != index_binding {
            return Err(invalid("IFC enumeration original source index changed").into());
        }
        if cx.should_yield() || !retained_clone_grant_admits(cx.retained_grant(), self.demands()) {
            return Ok(false);
        }
        if self.state.phase == 8 {
            return Ok(true);
        }
        let mut next = self.state;
        next.binding = Some(binding);
        next.index = index_binding;
        let mut copied = 0;
        let result = Self::step::<C>(&mut next, snapshot, address, &mut copied, index);
        self.state = next;
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: 3 * size_of::<State>() + copied, ..Default::default() })?;
        cx.consume_fuel(1);
        result?;
        Ok(self.state.phase == 8)
    }
    pub fn result<'a>(&self, snapshot: &'a IfcSnapshot, revision: u64) -> Result<IfcEnumerationView<'a>, ValueError> {
        if self.closing || self.state.phase != 8 || !self.state.binding.is_some_and(|binding| binding.0 == snapshot as *const _ as usize && binding.3 == revision) {
            return Err(invalid("IFC enumeration query result has no original source revision"));
        }
        let state = self.state;
        let selection = args(snapshot, state.owner, state.part)?.get(state.values).ok_or_else(|| invalid("IFC enumeration original selection is absent"))?;
        let allowed = if state.diagnostic.is_none() { args(snapshot, state.target, state.target_part)?.get(state.target_values).and_then(IfcValue::as_aggregate) } else { None };
        Ok(IfcEnumerationView { property: snapshot.entities[state.owner].id, enumeration: (state.target != NONE).then(|| snapshot.entities[state.target].id), revision, selection, allowed, diagnostic: state.diagnostic })
    }
    pub(super) fn state_entry(&self, snapshot: &IfcSnapshot, revision: u64) -> Result<IfcEnumerationEntry, ValueError> {
        let _ = self.result(snapshot, revision)?;
        Ok(IfcEnumerationEntry {
            binding: (snapshot as *const _ as usize, revision),
            address: IfcEnumerationAddress { entity: self.state.owner, part: self.state.part, argument: Some(self.state.values) },
            target: self.state.target,
            target_part: self.state.target_part,
            target_values: self.state.target_values,
            diagnostic: self.state.diagnostic,
        })
    }
    pub fn begin_close(&mut self) {
        self.closing = true;
    }
    pub fn terminal_is_empty(&self) -> bool {
        self.state.binding.is_none()
    }
    pub fn close_demands(&self) -> RetirementDemand {
        RetirementDemand { copy_bytes: size_of::<State>(), depth: 1, ..Default::default() }
    }
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.closing {
            return Err(invalid("IFC enumeration query close was not started"));
        }
        let demand = self.close_demands();
        if !retained_clone_grant_admits(grant, demand) {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        self.state = State::default();
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))
    }
}
