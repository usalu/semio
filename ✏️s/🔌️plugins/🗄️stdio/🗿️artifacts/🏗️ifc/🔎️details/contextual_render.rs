//! 🪟️ Original details render work resolves contextual metadata before bounded provider control construction.
use super::contextual::{IfcEnumerationAddress, IfcEnumerationEntry};
use super::*;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, retained_clone_grant_admits};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use std::mem::size_of;
#[cfg(test)]
#[path = "contextual_render_tests.rs"]
mod tests;
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
struct RenderState {
    binding: Option<(usize, u64)>,
    source_len: usize,
    index_ready: bool,
    row: usize,
    component: usize,
    address: Option<IfcEnumerationAddress>,
    phase: u8,
    done: bool,
}
semio_framework_value::artifact_retire_leaf!(RenderState);
#[derive(Default, semio_framework_value::RetireOwned)]
pub struct IfcDetailsRenderWork {
    query: IfcEnumerationQuery,
    entries: Vec<IfcEnumerationEntry>,
    index: source_index::IfcSourceIndex,
    state: RenderState,
}
pub(super) fn choice_request(definition: u64, choice: usize, list: bool) -> DslValue {
    DslValue::Object(vec![(
        "$ifcEnumerationChoice".into(),
        DslValue::Object(vec![("definition".into(), DslValue::Number(semio_framework_value::Number::UInt(definition))), ("choice".into(), DslValue::String(choice.to_string())), ("list".into(), DslValue::Bool(list))]),
    )])
}
fn invalid(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, message)
}
pub(super) fn choice_label(value: &IfcValue) -> String {
    let (domain, value) = if let IfcValue::TypedValue(typed) = value { (Some(typed.name.as_str()), typed.items.first().unwrap_or(value)) } else { (None, value) };
    let brief = |text: &str| {
        let mut chars = text.chars();
        let mut result: String = chars.by_ref().take(16).collect();
        if chars.next().is_some() {
            result.push('…');
        }
        result
    };
    let payload = match value {
        IfcValue::String(text) | IfcValue::Enum(text) | IfcValue::Binary(text) => brief(text),
        IfcValue::Integer(value) => value.to_string(),
        IfcValue::Real(value) => value.to_string(),
        IfcValue::Reference(value) => format!("#{value}"),
        IfcValue::Aggregate(values) => format!("[{}]", values.len()),
        IfcValue::TypedValue(_) => "(…)".into(),
        IfcValue::Unset => "$".into(),
        IfcValue::Derived => "*".into(),
    };
    domain.map_or_else(|| payload.clone(), |domain| format!("{} {payload}", brief(domain)))
}
impl IfcDetailsRenderWork {
    fn entry_demand(&self) -> Result<RetirementDemand, ValueError> {
        let old = self.entries.capacity();
        let capacity = if old == self.entries.len() { old.checked_mul(2).unwrap_or(usize::MAX).max(8) } else { old };
        let bytes = capacity.checked_mul(size_of::<IfcEnumerationEntry>()).ok_or_else(|| invalid("IFC contextual metadata capacity overflow"))?;
        Ok(RetirementDemand {
            copy_bytes: 2 * size_of::<IfcEnumerationEntry>() + size_of::<RenderState>() + if capacity != old { self.entries.len() * size_of::<IfcEnumerationEntry>() } else { 0 },
            capacity_bytes: if capacity != old { bytes } else { 0 },
            release_bytes: if capacity != old { old * size_of::<IfcEnumerationEntry>() } else { 0 },
            depth: 1,
            ..Default::default()
        })
    }
    pub fn demands(&self) -> Result<RetirementDemand, ValueError> {
        if self.state.binding.is_some() && !self.state.index_ready {
            let mut demand = self.index.demands_len(self.state.source_len)?;
            demand.copy_bytes += size_of::<bool>();
            return Ok(demand);
        }
        Ok(match self.state.phase {
            1 => {
                let mut demand = self.query.demands();
                demand.copy_bytes += size_of::<u8>();
                demand
            }
            2 => self.entry_demand()?,
            3 => {
                let mut demand = self.query.close_demands();
                demand.copy_bytes += size_of::<u8>();
                demand
            }
            _ => RetirementDemand { copy_bytes: 2 * size_of::<RenderState>() + size_of::<IfcEnumerationQuery>(), depth: 1, ..Default::default() },
        })
    }
    pub fn advance<C: IfcDetailsProfile>(&mut self, snapshot: &IfcSnapshot, revision: u64, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if cx.is_cancelled() {
            return Err(ValueError::literal(ValueRefusalKind::Canceled, "IFC original contextual details render cancelled").into());
        }
        let binding = (snapshot as *const _ as usize, revision);
        if self.state.binding.is_some_and(|original| original != binding) {
            return Err(invalid("IFC original contextual details source revision changed").into());
        }
        let demand = self.demands()?;
        if cx.should_yield() || !retained_clone_grant_admits(cx.retained_grant(), demand) {
            return Ok(false);
        }
        if self.state.done {
            return Ok(true);
        }
        if self.state.binding.is_none() {
            self.state.binding = Some(binding);
            self.state.source_len = snapshot.entities.len();
            cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })?;
            cx.consume_fuel(1);
            return Ok(false);
        }
        if !self.state.index_ready {
            if self.index.advance(snapshot, revision, cx)? {
                self.state.index_ready = true;
                cx.consume_retained(RetainedCloneProgress { copied_bytes: size_of::<bool>(), ..Default::default() })?;
            }
            return Ok(false);
        }
        match self.state.phase {
            0 => {
                if let Some(entity) = snapshot.entities.get(self.state.row) {
                    if self.state.component > entity.complex.len() {
                        self.state.row += 1;
                        self.state.component = 0;
                    } else {
                        let (part, name) = if self.state.component == 0 { (None, &entity.name) } else { (Some(self.state.component - 1), &entity.complex[self.state.component - 1].name) };
                        self.state.component += 1;
                        if name.eq_ignore_ascii_case("IFCPROPERTYENUMERATEDVALUE") {
                            self.state.address = Some(IfcEnumerationAddress { entity: self.state.row, part, argument: None });
                            self.state.phase = 1;
                        }
                    }
                } else {
                    self.state.done = true;
                }
                cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })?;
                cx.consume_fuel(1);
            }
            1 => {
                if self.query.advance_indexed::<C>(snapshot, self.state.address.unwrap(), revision, &self.index, cx)? {
                    self.state.phase = 2;
                    cx.consume_retained(RetainedCloneProgress { copied_bytes: size_of::<u8>(), ..Default::default() })?;
                }
            }
            2 => {
                let old = self.entries.capacity();
                if demand.capacity_bytes > 0 {
                    self.entries.try_reserve_exact(demand.capacity_bytes / size_of::<IfcEnumerationEntry>() - self.entries.len()).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "IFC contextual metadata allocation failed"))?;
                }
                self.entries.push(self.query.state_entry(snapshot, revision)?);
                self.query.begin_close();
                self.state.phase = 3;
                cx.consume_retained(RetainedCloneProgress {
                    copied_items: 1,
                    copied_bytes: demand.copy_bytes,
                    retained_capacity_bytes: if self.entries.capacity() != old { self.entries.capacity() * size_of::<IfcEnumerationEntry>() } else { 0 },
                    released_bytes: if self.entries.capacity() != old { old * size_of::<IfcEnumerationEntry>() } else { 0 },
                })?;
                cx.consume_fuel(1);
            }
            3 => {
                let step = self.query.close_step(cx.retained_grant())?;
                cx.consume_retained(step.progress())?;
                cx.consume_fuel(step.progress().copied_items as u64);
                if self.query.terminal_is_empty() {
                    self.state.phase = 4;
                    cx.consume_retained(RetainedCloneProgress { copied_bytes: size_of::<u8>(), ..Default::default() })?;
                }
            }
            _ => {
                self.query = Default::default();
                self.state.address = None;
                self.state.phase = 0;
                cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })?;
                cx.consume_fuel(1);
            }
        };
        Ok(self.state.done)
    }
    pub fn validate(&self, snapshot: &IfcSnapshot, revision: u64) -> Result<(), ValueError> {
        if !self.state.done || self.state.binding != Some((snapshot as *const _ as usize, revision)) {
            return Err(invalid("IFC contextual details have no completed original source revision"));
        }
        Ok(())
    }
    pub(super) fn at(&self, snapshot: &IfcSnapshot, revision: u64, path: &[P]) -> Option<(&IfcEnumerationEntry, usize)> {
        self.validate(snapshot, revision).ok()?;
        if path.len() < 4 || !key(&path[0], "entities") {
            return None;
        }
        let P::Index(entity) = path[1] else {
            return None;
        };
        let (part, start) = if key(&path[2], "complex") {
            if path.len() < 6 || !key(&path[4], "args") {
                return None;
            }
            let P::Index(part) = path[3] else {
                return None;
            };
            (Some(part), 5)
        } else {
            if !key(&path[2], "args") {
                return None;
            }
            (None, 3)
        };
        let P::Index(argument) = path[start] else {
            return None;
        };
        let entry = &self.entries[self
            .entries
            .binary_search_by_key(&(entity, part), |entry| {
                let address = entry.address();
                (address.entity, address.part)
            })
            .ok()?];
        (entry.address().argument == Some(argument)).then_some((entry, start + 1))
    }
}
