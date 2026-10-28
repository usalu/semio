//! 🔀️ Original source constituents cross one funded roster slot before central complex publication.
use super::*;
use semio_framework_value::retained_clone::{RetainedClone, RetainedCloneClose, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, retained_clone_grant_admits};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use std::mem::size_of;
type PartsCursor = <Vec<IfcComplexType> as RetainedClone>::Cursor;
type ValueCursor = <IfcValue as RetainedClone>::Cursor;
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct State {
    binding: Option<(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize)>,
    authority: Option<semio_framework_job::OriginalSourceBinding>,
    position: usize,
    origin: usize,
    destination: usize,
    id: u64,
    row: usize,
    matches: usize,
    next: usize,
    phase: u8,
    removal: bool,
    renaming: bool,
    primary: bool,
    insertion: bool,
    arity: usize,
    target_arity: usize,
    reshaping: bool,
    mapping: bool,
    repair: bool,
    rootless_profile: Option<usize>,
    input_profile: Option<[usize;6]>,
    primitive_total: usize,
    slots_ready: bool,
    published: Option<(usize, usize)>,
    guard_part: usize,
    guard_slot: usize,
    guard_values: bool,
    guard_direct: bool,
    closing: bool,
}
#[derive(Default)]
pub(super) struct ConstituentRoster {
    state: State,
    rebuilt: Option<Vec<IfcComplexType>>,
    retyped_args: Option<Vec<IfcValue>>,
    root_slots: Option<Vec<usize>>,
    inputs: super::type_input::Inputs,
    remap: super::type_remap::Plan,
    retirement: RetainedCloneClose,
}
pub(super) struct RetainedTypeAdmission<'a> {
    owner: &'a ConstituentRoster,
}
impl RetainedTypeAdmission<'_> {
    pub(super) fn same_name_repair(&self) -> bool {
        self.owner.state.repair && self.owner.state.primary
    }
    pub(super) fn position(&self) -> usize {
        self.owner.state.position
    }
    pub(super) fn primary_name<'a>(&self, entity: &'a IfcEntity, mutations: &'a [IfcMutation]) -> Option<&'a str> {
        if !self.owner.state.primary {
            return None;
        }
        if self.owner.state.repair {
            return Some(&entity.name);
        }
        match mutations.first()? {
            IfcMutation::SetEntityName(value) if value.id == entity.id => Some(&value.name),
            _ => None,
        }
    }
    pub(super) fn primary_arity(&self) -> Option<usize> {
        self.owner.state.primary.then_some(self.owner.state.target_arity)
    }
    pub(super) fn primary_argument<'a>(&self, entity: &'a IfcEntity, mutations: &'a [IfcMutation], index: usize) -> Option<&'a IfcValue> {
        let state = &self.owner.state;
        if !state.primary || index >= state.target_arity {
            return None;
        }
        if state.mapping {
            let ordinal = *self.owner.root_slots.as_ref()?.get(index)?;
            if ordinal == usize::MAX {
                return entity.args.get(index);
            }
            return match mutations.get(ordinal)? {
                IfcMutation::SetEntityArg(value) if value.id == state.id && value.index == index => Some(&value.value),
                IfcMutation::InsertEntityArg(value) if value.id == state.id && value.index == index => Some(&value.value),
                _ => None,
            };
        }
        if !state.reshaping {
            return entity.args.get(index);
        }
        let IfcMutation::InsertEntityArg(value) = mutations.get(index.checked_add(1)?)? else {
            return None;
        };
        (value.id == state.id && value.index == index).then_some(&value.value)
    }
    pub(super) fn validate<C: IfcDetailsProfile>(&self, event: &editing::SnapshotEditEvent, snapshot: &IfcSnapshot, address: &pointer::PathAdmission, mutations: &[IfcMutation], cx: &semio_framework_job::StepContext<'_>) -> Result<usize, ValueError> {
        let state = &self.owner.state;
        let Some((original_event, original_snapshot, original_address, _, _, _, _, _, _, normalization, count)) = state.binding else {
            return Err(invalid("IFC retained type original binding missing"));
        };
        if state.closing
            || state.phase != 6
            || !state.renaming
            || state.arity == 0 && !state.reshaping
            || original_event != event as *const _ as usize
            || original_snapshot != snapshot as *const _ as usize
            || original_address != address as *const _ as usize
            || normalization != C::normalization_attribute as usize
            || count != C::direct_attribute_count as usize
            || state.input_profile != Some(input_profile::<C>())
            || state.authority.is_none_or(|authority| !authority.matches(cx))
            || state.published != Some((mutations.as_ptr() as usize, mutations.len()))
        {
            return Err(invalid("IFC retained type domain owner changed"));
        }
        let entity = snapshot.entities.get(state.position).ok_or_else(|| invalid("IFC retained type source entity missing"))?;
        let shape = match (state.primary, mutations.first()) {
            (true, _) if state.repair => {
                state.mapping
                    && state.arity == state.target_arity
                    && entity.args.len() == state.arity
                    && self.owner.primitive_count() == Some(mutations.len())
                    && self.owner.root_slots.as_ref().is_some_and(|slots| slots.len() == state.target_arity)
                    && ConstituentRoster::type_name(event, address).is_some_and(|name| name.eq_ignore_ascii_case(&entity.name))
                    && (!entity.complex.is_empty() || state.rootless_profile == Some(C::rootless_attribute_count as usize) && C::rootless_attribute_count(&entity.name) == Some(state.arity))
                    && matches!(mutations.first(), Some(IfcMutation::SetEntityArg(value)) if value.id == state.id)
            }
            (true, Some(IfcMutation::SetEntityName(candidate))) => {
                candidate.id == state.id
                    && entity.args.len() == state.arity
                    && !entity.complex.is_empty()
                    && self.owner.primitive_count() == Some(mutations.len())
                    && (!state.mapping || self.owner.root_slots.as_ref().is_some_and(|slots| slots.len() == state.target_arity))
            }
            (false, Some(IfcMutation::SetEntityComplex(candidate))) => {
                candidate.id == state.id
                    && mutations.len() == 1
                    && candidate.complex.len() == entity.complex.len()
                    && candidate.complex.get(state.origin).is_some_and(|part| part.args.len() == state.target_arity)
                    && entity.complex.get(state.origin).is_some_and(|part| part.args.len() == state.arity)
            }
            _ => false,
        };
        if entity.id != state.id || !shape {
            return Err(invalid("IFC retained type candidate shape changed"));
        }
        Ok(self.owner as *const _ as usize)
    }
}
fn input_profile<C:IfcDetailsProfile>()->[usize;6]{[C::enumeration_count as usize,C::enumeration_member as usize,C::rootless_attribute_count as usize,C::defined_type_expression as usize,C::select_count as usize,C::select_member as usize]}
fn invalid(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, message)
}
impl ConstituentRoster {
    #[cfg(test)]
    pub(super) fn profile_probe<C:IfcDetailsProfile>(&mut self,hook:usize)->State{assert!((3..6).contains(&hook));let original=self.state;let captured=original.input_profile.unwrap();let mut profile=input_profile::<C>();assert_ne!(profile[hook],captured[hook]);profile[hook]=captured[hook];self.state.input_profile=Some(profile);let binding=self.state.binding.as_mut().unwrap();binding.9=C::normalization_attribute as usize;binding.10=C::direct_attribute_count as usize;original}
    #[cfg(test)]
    pub(super) fn restore_profile_probe(&mut self,original:State){self.state=original;}

    fn primitive_count(&self) -> Option<usize> {
        if self.state.primary && self.state.mapping {
            Some(self.state.primitive_total)
        } else if self.state.primary && self.state.reshaping {
            self.state.arity.max(self.state.target_arity).checked_add(1)
        } else {
            Some(1)
        }
    }
    pub(super) fn receiving_primitives(&self) -> bool {
        matches!(self.state.phase, 14 | 19 | 20 | 21)
    }
    pub(super) fn type_admission(&self) -> Option<RetainedTypeAdmission<'_>> {
        (!self.state.closing
            && self.state.binding.is_some()
            && self.state.phase == 6
            && self.state.renaming
            && (self.state.arity > 0 || self.state.reshaping)
            && self.state.published.is_some_and(|(_, length)| length > 0 && Some(length) == self.primitive_count()))
        .then_some(RetainedTypeAdmission { owner: self })
    }
    fn insertion_name<'a>(event: &'a editing::SnapshotEditEvent, address: &pointer::PathAdmission) -> Option<(&'a str, bool)> {
        let editing::SnapshotEditEvent::InsertValue { path, value } = event else {
            return None;
        };
        if address.field_count() != 4 || !address.field_is(path, 0, "entities") || !address.field_is(path, 2, "complex") {
            return None;
        }
        let DslValue::Object(fields) = value else {
            return None;
        };
        if fields.len() != 2 {
            return None;
        }
        let name = value.get("name")?.as_str()?;
        let DslValue::Array(args) = value.get("args")? else {
            return None;
        };
        Some((name, args.is_empty()))
    }
    fn insertion_name_mut<'a>(event: &'a mut editing::SnapshotEditEvent) -> Option<&'a mut String> {
        let editing::SnapshotEditEvent::InsertValue { value: DslValue::Object(fields), .. } = event else {
            return None;
        };
        if fields.len() != 2 {
            return None;
        }
        let (_, DslValue::String(name)) = fields.iter_mut().find(|(key, _)| key.as_str() == "name")? else {
            return None;
        };
        Some(name)
    }
    fn type_name<'a>(event: &'a editing::SnapshotEditEvent, address: &pointer::PathAdmission) -> Option<&'a str> {
        let editing::SnapshotEditEvent::SetValue { path, value } = event else {
            return None;
        };
        if !address.field_is(path, 0, "entities") {
            return None;
        }
        if address.field_count() == 2 {
            return requested_type(value);
        }
        if !address.field_is(path, 2, "complex") {
            return None;
        }
        if address.field_count() == 4 {
            return requested_type(value);
        }
        if address.field_count() == 5 && address.field_is(path, 4, "name") {
            return value.as_str();
        }
        None
    }
    fn type_name_mut<'a>(event: &'a mut editing::SnapshotEditEvent, address: &pointer::PathAdmission) -> Option<&'a mut String> {
        let editing::SnapshotEditEvent::SetValue { path, value } = event else {
            return None;
        };
        if address.field_count() == 5 && address.field_is(path, 4, "name") {
            if let DslValue::String(name) = value {
                return Some(name);
            }
            return None;
        }
        if address.field_count() != 4 && address.field_count() != 2 {
            return None;
        }
        super::type_input::request_name_mut(value)
    }
    pub(super) fn applies(event: &editing::SnapshotEditEvent, address: &pointer::PathAdmission) -> bool {
        Self::type_name(event, address).is_some()
            || Self::insertion_name(event, address).is_some()
            || matches!(event,editing::SnapshotEditEvent::MoveValue{path,..}|editing::SnapshotEditEvent::RemoveValue{path}if address.field_count()==4&&address.field_is(path,0,"entities")&&address.field_is(path,2,"complex"))
    }
    pub(super) fn fallback(&self) -> bool {
        self.state.phase == 8
    }
    fn length(&self, snapshot: &IfcSnapshot) -> usize {
        snapshot.entities[self.state.position].complex.len() + usize::from(self.state.insertion) - usize::from(self.state.removal)
    }
    fn base() -> RetirementDemand {
        RetirementDemand { copy_bytes: 4 * size_of::<State>() + 4 * size_of::<IfcComplexType>() + 2 * size_of::<IfcMutation>() + 4 * size_of::<Vec<IfcComplexType>>() + 2 * size_of::<Vec<IfcValue>>() + 4 * 256, depth: 1, ..Default::default() }
    }
    pub(super) fn demands(&self, event: &editing::SnapshotEditEvent, source: Option<&wire::SnapshotSource>, cursor: &PartsCursor, value_cursor: &ValueCursor, snapshot: &IfcSnapshot, body: usize) -> Result<RetirementDemand, ValueError> {
        let mut demand = Self::base();
        match self.state.phase {
            22 => {
                let child = self.inputs.demands(event, body)?;
                demand.copy_bytes = demand.copy_bytes.checked_add(child.copy_bytes).ok_or_else(|| invalid("IFC type input wrapper copy overflow"))?;
                demand.capacity_bytes = child.capacity_bytes;
                demand.release_bytes = child.release_bytes;
                demand.depth = demand.depth.max(child.depth);
            }
            15 => {
                demand.copy_bytes = demand.copy_bytes.checked_add(super::type_remap::Plan::copy_bytes()).ok_or_else(|| invalid("IFC retype metadata copy overflow"))?;
                demand.capacity_bytes = self.remap.capacity_bytes(self.state.target_arity)?;
            }
            16 => demand.capacity_bytes = self.state.target_arity.checked_mul(size_of::<IfcValue>()).ok_or_else(|| invalid("IFC retype tuple capacity overflow"))?,
            2 => {
                let source = source.ok_or_else(|| invalid("IFC constituent move source missing"))?;
                let child = cursor.advance_demands(source.borrow().project(self.state.position, |snapshot| &snapshot.entities[self.state.position].complex), body.min(4096).saturating_sub(Self::base().copy_bytes))?;
                demand.copy_bytes = demand.copy_bytes.checked_add(child.copy_bytes).ok_or_else(|| invalid("IFC constituent move copy overflow"))?;
                demand.capacity_bytes = child.capacity_bytes;
                demand.release_bytes = child.release_bytes;
                demand.depth = demand.depth.max(child.depth);
            }
            18 if !self.state.slots_ready => demand.capacity_bytes = self.state.target_arity.checked_mul(size_of::<usize>()).ok_or_else(|| invalid("IFC primary ordinal capacity overflow"))?,
            19 | 20 => {
                let child = if self.state.phase == 19 {
                    let source = source.ok_or_else(|| invalid("IFC primary value source missing"))?;
                    let index = *self.remap.map.as_ref().and_then(|map| map.get(self.state.next)).ok_or_else(|| invalid("IFC primary source ordinal missing"))?;
                    value_cursor.advance_demands(source.borrow().project(self.state.position, |snapshot| &snapshot.entities[self.state.position]).project(index, |entity| &entity.args[index]), body.min(4096).saturating_sub(Self::base().copy_bytes))?
                } else if value_cursor.terminal_is_empty() {
                    RetirementDemand { copy_bytes: size_of::<ValueCursor>(), depth: 1, ..Default::default() }
                } else {
                    value_cursor.close_demands(body.min(4096).saturating_sub(Self::base().copy_bytes))?
                };
                demand.copy_bytes = demand.copy_bytes.checked_add(child.copy_bytes).ok_or_else(|| invalid("IFC primary value copy overflow"))?;
                demand.capacity_bytes = child.capacity_bytes;
                demand.release_bytes = child.release_bytes;
                demand.depth = demand.depth.max(child.depth);
            }
            3 => demand.capacity_bytes = self.length(snapshot).checked_mul(size_of::<IfcComplexType>()).ok_or_else(|| invalid("IFC constituent roster capacity overflow"))?,
            12 => demand.capacity_bytes = self.state.target_arity.checked_mul(size_of::<IfcValue>()).ok_or_else(|| invalid("IFC constituent default tuple capacity overflow"))?,
            5 => demand.capacity_bytes = size_of::<IfcMutation>(),
            10 => demand.capacity_bytes = self.primitive_count().and_then(|count| count.checked_mul(size_of::<IfcMutation>())).ok_or_else(|| invalid("IFC primary primitive capacity overflow"))?,
            _ => {}
        }
        Ok(demand)
    }
    pub(super) fn advance<C: IfcDetailsProfile>(
        &mut self,
        event: &mut editing::SnapshotEditEvent,
        snapshot: &IfcSnapshot,
        address: &pointer::PathAdmission,
        source: Option<&wire::SnapshotSource>,
        cursor: &mut PartsCursor,
        parts: &mut Option<Vec<IfcComplexType>>,
        pending: &mut Option<Vec<IfcMutation>>,
        value_cursor: &mut ValueCursor,
        value: &mut Option<IfcValue>,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if cx.is_cancelled() {
            return Err(ValueError::literal(ValueRefusalKind::Canceled, "IFC original constituent move cancelled").into());
        }
        let source = source.ok_or_else(|| invalid("IFC constituent move source missing"))?;
        let binding = (
            event as *const _ as usize,
            snapshot as *const _ as usize,
            address as *const _ as usize,
            source as *const _ as usize,
            cursor as *const _ as usize,
            parts as *const _ as usize,
            pending as *const _ as usize,
            value_cursor as *const _ as usize,
            value as *const _ as usize,
            C::normalization_attribute as usize,
            C::direct_attribute_count as usize,
        );
        if self.state.input_profile.is_some_and(|profile| profile != input_profile::<C>())
            || self.state.closing
            || self.state.binding.is_some_and(|original| original != binding)
            || self.state.authority.is_some_and(|authority| !authority.matches(cx))
        {
            return Err(invalid("IFC constituent move original owner changed or closed").into());
        }
        let body = cx.retained_grant().maximum_copy_bytes.min(4096);
        let demand = self.demands(event, Some(source), cursor, value_cursor, snapshot, body)?;
        if cx.should_yield() || !retained_clone_grant_admits(cx.retained_grant(), demand) {
            return Ok(false);
        }
        if self.state.phase == 6 {
            return Ok(true);
        }
        if self.state.phase == 22 {
            cx.consume_retained(RetainedCloneProgress { copied_bytes: Self::base().copy_bytes, ..Default::default() })?;
            let mut bytes = [0; 256];
            let name = Self::type_name(event, address).filter(|name| name.len() <= 256 && name.is_ascii()).ok_or_else(|| invalid("IFC type input target changed"))?;
            let length = name.len();
            for (index, byte) in name.bytes().enumerate() {
                bytes[index] = byte.to_ascii_uppercase();
            }
            let name = std::str::from_utf8(&bytes[..length]).map_err(|_| invalid("IFC type input target invalid"))?;
            if self.inputs.advance::<C>(event, name, self.state.target_arity, cx)? {
                self.state.phase = 15;
            }
            return Ok(false);
        }
        if self.state.phase == 19 || self.state.phase == 20 && !value_cursor.terminal_is_empty() {
            cx.consume_retained(RetainedCloneProgress { copied_bytes: Self::base().copy_bytes, ..Default::default() })?;
            let child_grant = RetainedCloneGrant { maximum_copy_bytes: cx.retained_grant().maximum_copy_bytes.min(body.saturating_sub(Self::base().copy_bytes)), ..cx.retained_grant() };
            let step = if self.state.phase == 19 {
                let index = *self.remap.map.as_ref().and_then(|map| map.get(self.state.next)).ok_or_else(|| invalid("IFC primary source ordinal missing"))?;
                value_cursor.advance(source.borrow().project(self.state.position, |snapshot| &snapshot.entities[self.state.position]).project(index, |entity| &entity.args[index]), body.saturating_sub(Self::base().copy_bytes), child_grant)
            } else {
                value_cursor.close_step(child_grant)
            }
            .map_err(|error| semio_framework_job::OriginalTurnFailure::debit_original(error, cx))?;
            cx.consume_retained(step.progress())?;
            cx.consume_fuel(1);
            if self.state.phase == 19 && matches!(step, RetainedCloneStep::Complete(_)) {
                if value.is_some() {
                    return Err(invalid("IFC primary value receiver occupied").into());
                }
                *value = Some(value_cursor.take().ok_or_else(|| invalid("IFC primary moved payload missing"))?);
                value_cursor.begin_close();
                self.state.phase = 20;
            }
            return Ok(false);
        }
        if self.state.phase == 2 {
            cx.consume_retained(RetainedCloneProgress { copied_bytes: Self::base().copy_bytes, ..Default::default() })?;
            let step = cursor
                .advance(source.borrow().project(self.state.position, |snapshot| &snapshot.entities[self.state.position].complex), body.saturating_sub(Self::base().copy_bytes), cx.retained_grant())
                .map_err(|error| semio_framework_job::OriginalTurnFailure::debit_original(error, cx))?;
            cx.consume_retained(step.progress())?;
            cx.consume_fuel(1);
            if matches!(step, RetainedCloneStep::Complete(_)) {
                *parts = cursor.take();
                self.state.phase = if self.state.mapping { 16 } else { 3 };
            }
            return Ok(false);
        }
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })?;
        cx.consume_fuel(1);
        match self.state.phase {
            0 => {
                let renaming = Self::type_name(event, address).is_some();
                let insertion = Self::insertion_name(event, address).is_some();
                let primary = renaming && address.field_count() == 2;
                let removal = matches!(event, editing::SnapshotEditEvent::RemoveValue { .. });
                let (path, from) = match &*event {
                    editing::SnapshotEditEvent::MoveValue { path, from } => (path, Some(from)),
                    editing::SnapshotEditEvent::RemoveValue { path } | editing::SnapshotEditEvent::SetValue { path, .. } | editing::SnapshotEditEvent::InsertValue { path, .. } => (path, None),
                    _ => return Err(invalid("IFC constituent roster command missing").into()),
                };
                if !Self::applies(event, address) {
                    return Err(invalid("IFC constituent roster path invalid").into());
                }
                let position = address.field_index(path, 1).ok_or_else(|| invalid("IFC constituent move position missing"))?;
                if let Some(from) = from {
                    if address.from_field_count() != 4 || !address.from_field_is(from, 0, "entities") || !address.from_field_is(from, 2, "complex") || address.from_field_index(from, 1) != Some(position) {
                        return Err(invalid("IFC constituent move leaves its owner").into());
                    }
                }
                let entity = snapshot.entities.get(position).ok_or_else(|| invalid("IFC constituent roster entity missing"))?;
                let length = entity.complex.len();
                let origin = if primary || insertion {
                    Some(0)
                } else if let Some(from) = from {
                    address.from_field_index(from, 3)
                } else {
                    address.field_index(path, 3)
                }
                .ok_or_else(|| invalid("IFC constituent roster origin missing"))?;
                let destination = if insertion {
                    if address.field_is(path, 3, "-") { length } else { address.field_index(path, 3).ok_or_else(|| invalid("IFC constituent insertion destination missing"))? }
                } else if removal || renaming {
                    origin
                } else if address.field_is(path, 3, "-") {
                    length.saturating_sub(1)
                } else {
                    address.field_index(path, 3).ok_or_else(|| invalid("IFC constituent move destination missing"))?
                };
                if (if insertion { destination > length } else { !primary && (origin >= length || destination >= length) }) || entity.id == 0 || parts.is_some() || pending.is_some() {
                    return Err(invalid("IFC constituent roster bounds or owner invalid").into());
                }
                self.state.binding = Some(binding);
                self.state.input_profile = Some(input_profile::<C>());
                self.state.authority = Some(semio_framework_job::OriginalSourceBinding::borrow(cx));
                self.state.position = position;
                self.state.origin = origin;
                self.state.destination = destination;
                self.state.id = entity.id;
                self.state.removal = removal;
                self.state.renaming = renaming;
                self.state.primary = primary;
                self.state.insertion = insertion;
                self.state.phase = 1;
            }
            1 => {
                if let Some(entity) = snapshot.entities.get(self.state.row) {
                    self.state.matches += usize::from(entity.id == self.state.id);
                    self.state.row += 1;
                } else if self.state.matches != 1 {
                    return Err(invalid("IFC constituent move requires unique identity").into());
                } else {
                    self.state.phase = if self.state.renaming || self.state.insertion {
                        9
                    } else if self.state.removal {
                        7
                    } else {
                        2
                    };
                }
            }
            9 => {
                let entity = &snapshot.entities[self.state.position];
                if self.state.insertion {
                    let mut bytes = [0; 256];
                    let requested = Self::insertion_name(event, address).ok_or_else(|| invalid("IFC constituent original insertion payload changed"))?;
                    let count = super::domain_cursor::canonical_name(requested.0, &mut bytes).and_then(C::direct_attribute_count);
                    self.state.phase = if count == Some(0) && requested.1 && !entity.complex.is_empty() { 7 } else { 8 };
                } else {
                    let (name, args) = if self.state.primary {
                        (entity.name.as_str(), entity.args.as_slice())
                    } else {
                        let part = &entity.complex[self.state.origin];
                        (part.name.as_str(), part.args.as_slice())
                    };
                    let mut previous = [0; 256];
                    let mut requested = [0; 256];
                    let old = super::domain_cursor::canonical_name(name, &mut previous).and_then(C::direct_attribute_count);
                    let next = Self::type_name(event, address).and_then(|name| super::domain_cursor::canonical_name(name, &mut requested)).and_then(C::direct_attribute_count);
                    let complete = old.is_some() && next.is_some() && old == Some(args.len());
                    let matches = complete && old == next && (!self.state.primary || !entity.complex.is_empty());
                    let reshaping = complete && old != next && (next == Some(0) || old == Some(0)) && (!self.state.primary || !entity.complex.is_empty());
                    let supplied = super::type_input::rows(event).is_some();
                    let repair = complete
                        && self.state.primary
                        && supplied
                        && old == next
                        && old != Some(0)
                        && Self::type_name(event, address).is_some_and(|requested| requested.eq_ignore_ascii_case(name))
                        && (!entity.complex.is_empty() || super::domain_cursor::canonical_name(name, &mut previous).and_then(C::rootless_attribute_count) == old);
                    let mapping = repair || complete && (!self.state.primary || !entity.complex.is_empty()) && old != next && old != Some(0) && next != Some(0);
                    if supplied && !(self.state.primary && mapping) {
                        return Err(invalid("IFC type inputs require an original complete primary remap").into());
                    }
                    if matches || reshaping || mapping {
                        self.state.arity = args.len();
                        self.state.target_arity = next.unwrap();
                        self.state.reshaping = reshaping || mapping;
                        self.state.mapping = mapping;
                        self.state.repair = repair;
                        self.state.rootless_profile = (repair && entity.complex.is_empty()).then_some(C::rootless_attribute_count as usize);
                    }
                    self.state.phase = if mapping {
                        if supplied { 22 } else { 15 }
                    } else if reshaping {
                        11
                    } else if matches {
                        7
                    } else {
                        8
                    };
                }
            }
            11 => {
                if self.state.guard_slot == self.state.target_arity {
                    self.state.guard_slot = 0;
                    self.state.phase = 7;
                } else {
                    let mut bytes = [0; 256];
                    let name = Self::type_name(event, address).and_then(|name| super::domain_cursor::canonical_name(name, &mut bytes)).ok_or_else(|| invalid("IFC constituent default target changed"))?;
                    match C::normalization_attribute(name, self.state.guard_slot, true) {
                        Ok(Some(attribute)) if attribute.optional && !attribute.derived && !attribute.global_id => self.state.guard_slot += 1,
                        _ => self.state.phase = 8,
                    }
                }
            }
            15 => {
                let entity = &snapshot.entities[self.state.position];
                let (previous_name, args) = if self.state.primary {
                    (entity.name.as_str(), entity.args.as_slice())
                } else {
                    let part = &entity.complex[self.state.origin];
                    (part.name.as_str(), part.args.as_slice())
                };
                let mut previous = [0; 256];
                let mut target = [0; 256];
                let previous = super::domain_cursor::canonical_name(previous_name, &mut previous).ok_or_else(|| invalid("IFC retype source name invalid"))?;
                let target = Self::type_name(event, address).and_then(|name| super::domain_cursor::canonical_name(name, &mut target)).ok_or_else(|| invalid("IFC retype target name invalid"))?;
                let (complete, capacity) = self.remap.advance::<C>(previous, target, args, self.state.target_arity, self.inputs.values())?;
                if capacity > 0 {
                    cx.consume_retained(RetainedCloneProgress { retained_capacity_bytes: capacity, ..Default::default() })?;
                }
                if complete {
                    self.state.phase = if self.state.primary { 7 } else { 2 };
                }
            }
            16 => {
                let values = self.retyped_args.get_or_insert_with(Vec::new);
                values.try_reserve_exact(self.state.target_arity).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "IFC retype target tuple allocation failed"))?;
                cx.consume_retained(RetainedCloneProgress { retained_capacity_bytes: values.capacity() * size_of::<IfcValue>(), ..Default::default() })?;
                self.state.phase = 17;
            }
            17 => {
                if self.state.next == self.state.target_arity {
                    self.state.next = 0;
                    self.state.phase = 7;
                } else {
                    let previous = *self.remap.map.as_ref().and_then(|map| map.get(self.state.next)).ok_or_else(|| invalid("IFC retype ordinal map missing"))?;
                    let value = if previous == usize::MAX {
                        IfcValue::Unset
                    } else {
                        let old = parts.as_mut().and_then(|parts| parts.get_mut(self.state.origin)).and_then(|part| part.args.get_mut(previous)).ok_or_else(|| invalid("IFC retype original argument missing"))?;
                        std::mem::replace(old, IfcValue::Unset)
                    };
                    self.retyped_args.as_mut().ok_or_else(|| invalid("IFC retype target tuple owner missing"))?.push(value);
                    self.state.next += 1;
                }
            }
            7 => {
                let entity = &snapshot.entities[self.state.position];
                if self.state.removal && entity.complex.len() <= 1 {
                    self.state.phase = 8;
                } else if self.state.guard_part > entity.complex.len() + usize::from(self.state.insertion) {
                    self.state.phase = if self.state.primary && self.state.mapping {
                        18
                    } else if self.state.primary {
                        10
                    } else if self.state.mapping {
                        3
                    } else if self.state.reshaping {
                        12
                    } else {
                        2
                    };
                } else if self.state.removal && self.state.guard_part == self.state.origin + 1 {
                    self.state.guard_part += 1;
                } else {
                    let (name, values) = if self.state.guard_part == 0 {
                        (if self.state.primary { Self::type_name(event, address).ok_or_else(|| invalid("IFC primary original type request changed"))? } else { entity.name.as_str() }, entity.args.as_slice())
                    } else if self.state.insertion && self.state.guard_part == entity.complex.len() + 1 {
                        (Self::insertion_name(event, address).ok_or_else(|| invalid("IFC constituent original insertion payload changed"))?.0, &[][..])
                    } else {
                        let part = &entity.complex[self.state.guard_part - 1];
                        (
                            if self.state.renaming && !self.state.primary && self.state.guard_part == self.state.origin + 1 {
                                Self::type_name(event, address).ok_or_else(|| invalid("IFC constituent original type request changed"))?
                            } else {
                                part.name.as_str()
                            },
                            if self.state.mapping && self.state.guard_part == self.state.origin + 1 { self.retyped_args.as_deref().ok_or_else(|| invalid("IFC retype prepared tuple missing"))? } else { part.args.as_slice() },
                        )
                    };
                    let mapped = self.state.primary && self.state.mapping && self.state.guard_part == 0;
                    let value_at = |index: usize| -> Option<&IfcValue> {
                        if mapped {
                            let previous = *self.remap.map.as_ref()?.get(index)?;
                            if previous == usize::MAX {
                                Some(&IfcValue::Unset)
                            } else if let Some(ordinal) = super::type_input::input_ordinal(previous) {
                                self.inputs.argument(ordinal)
                            } else {
                                entity.args.get(previous)
                            }
                        } else {
                            values.get(index)
                        }
                    };
                    if self.state.guard_values {
                        if let Some(value) = value_at(self.state.guard_slot) {
                            if matches!(value, IfcValue::Derived) {
                                self.state.phase = 8;
                            } else {
                                self.state.guard_slot += 1;
                            }
                        } else {
                            self.state.guard_part += 1;
                            self.state.guard_slot = 0;
                            self.state.guard_values = false;
                            self.state.guard_direct = false;
                        }
                    } else {
                        let mut bytes = [0; 256];
                        let canonical = super::domain_cursor::canonical_name(name, &mut bytes);
                        if canonical.is_none()
                            || self.state.guard_part > 0
                                && !(self.state.renaming && !self.state.primary && self.state.guard_part == self.state.origin + 1)
                                && !(self.state.insertion && self.state.guard_part == entity.complex.len() + 1)
                                && canonical != Some(name)
                        {
                            self.state.phase = 8;
                        } else {
                            match C::normalization_attribute(canonical.unwrap(), self.state.guard_slot, self.state.guard_direct) {
                                Ok(Some(attribute)) => {
                                    if attribute.derived || self.state.guard_direct && attribute.global_id && !matches!(value_at(self.state.guard_slot),Some(IfcValue::String(value))if !value.is_empty()) {
                                        self.state.phase = 8;
                                    } else {
                                        self.state.guard_slot += 1;
                                    }
                                }
                                Ok(None) => {
                                    self.state.guard_slot = 0;
                                    if self.state.guard_direct {
                                        self.state.guard_values = true;
                                    } else {
                                        self.state.guard_direct = true;
                                    }
                                }
                                _ => self.state.phase = 8,
                            }
                        }
                    }
                }
            }
            3 => {
                let length = self.length(snapshot);
                let values = self.rebuilt.get_or_insert_with(Vec::new);
                values.try_reserve_exact(length).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "IFC constituent roster allocation failed"))?;
                cx.consume_retained(RetainedCloneProgress { retained_capacity_bytes: values.capacity() * size_of::<IfcComplexType>(), ..Default::default() })?;
                self.state.phase = 4;
            }
            12 => {
                let values = self.retyped_args.get_or_insert_with(Vec::new);
                values.try_reserve_exact(self.state.target_arity).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "IFC constituent default tuple allocation failed"))?;
                cx.consume_retained(RetainedCloneProgress { retained_capacity_bytes: values.capacity() * size_of::<IfcValue>(), ..Default::default() })?;
                self.state.phase = 13;
            }
            13 => {
                if self.state.next == self.state.target_arity {
                    self.state.next = 0;
                    self.state.phase = 2;
                } else {
                    self.retyped_args.as_mut().ok_or_else(|| invalid("IFC constituent default tuple missing"))?.push(IfcValue::Unset);
                    self.state.next += 1;
                }
            }
            10 => {
                if pending.is_some() {
                    return Err(invalid("IFC primary type receiver occupied").into());
                }
                let count = self.primitive_count().ok_or_else(|| invalid("IFC primary primitive count overflow"))?;
                if self.state.repair && count == 0 {
                    return Err(invalid("IFC unchanged type has no required argument changes").into());
                }
                let name = Self::type_name_mut(event, address).ok_or_else(|| invalid("IFC primary original type name owner missing"))?;
                let values = pending.get_or_insert_with(Vec::new);
                if self.state.repair || !name.eq_ignore_ascii_case(&snapshot.entities[self.state.position].name) {
                    values.try_reserve_exact(count).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "IFC primary type primitive allocation failed"))?;
                    cx.consume_retained(RetainedCloneProgress { retained_capacity_bytes: values.capacity() * size_of::<IfcMutation>(), ..Default::default() })?;
                    if !self.state.repair {
                        name.make_ascii_uppercase();
                        values.push(IfcMutation::SetEntityName(semio_s_artifact_stdio_ifc_4_mutations::standards::v4::subsets::any::schema::mutations::set_entity_name::SetEntityName { id: self.state.id, name: std::mem::take(name) }));
                    }
                }
                if self.state.reshaping {
                    self.state.phase = 14;
                } else {
                    self.state.published = Some((values.as_ptr() as usize, values.len()));
                    self.state.phase = 6;
                }
            }
            18 => {
                if !self.state.slots_ready {
                    let slots = self.root_slots.get_or_insert_with(Vec::new);
                    slots.try_reserve_exact(self.state.target_arity).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "IFC primary ordinal allocation failed"))?;
                    cx.consume_retained(RetainedCloneProgress { retained_capacity_bytes: slots.capacity() * size_of::<usize>(), ..Default::default() })?;
                    self.state.primitive_total = self.state.arity.saturating_sub(self.state.target_arity).checked_add(usize::from(!self.state.repair)).ok_or_else(|| invalid("IFC primary primitive count overflow"))?;
                    self.state.slots_ready = true;
                } else if self.state.next == self.state.target_arity {
                    self.state.next = 0;
                    self.state.phase = 10;
                } else {
                    let index = self.state.next;
                    let previous = *self.remap.map.as_ref().and_then(|map| map.get(index)).ok_or_else(|| invalid("IFC primary target ordinal missing"))?;
                    let unchanged = previous == index || previous == usize::MAX && matches!(snapshot.entities[self.state.position].args.get(index), Some(IfcValue::Unset));
                    let ordinal = if unchanged {
                        usize::MAX
                    } else {
                        let ordinal = self.state.primitive_total;
                        self.state.primitive_total = ordinal.checked_add(1).ok_or_else(|| invalid("IFC primary primitive count overflow"))?;
                        ordinal
                    };
                    self.root_slots.as_mut().ok_or_else(|| invalid("IFC primary ordinal owner missing"))?.push(ordinal);
                    self.state.next += 1;
                }
            }
            20 => {
                *value_cursor = IfcValue::retained_clone_cursor();
                self.state.phase = 21;
            }
            21 => {
                use semio_s_artifact_stdio_ifc_4_mutations::standards::v4::subsets::any::schema::mutations::{insert_entity_arg::InsertEntityArg, set_entity_arg::SetEntityArg};
                let index = self.state.next;
                let value = value.take().ok_or_else(|| invalid("IFC primary prepared payload missing"))?;
                let values = pending.as_mut().ok_or_else(|| invalid("IFC primary argument receiver missing"))?;
                values.push(if index < self.state.arity { IfcMutation::SetEntityArg(SetEntityArg { id: self.state.id, index, value }) } else { IfcMutation::InsertEntityArg(InsertEntityArg { id: self.state.id, index, value }) });
                self.state.next += 1;
                self.state.phase = 14;
            }
            14 if self.state.mapping => {
                use semio_s_artifact_stdio_ifc_4_mutations::standards::v4::subsets::any::schema::mutations::{insert_entity_arg::InsertEntityArg, remove_entity_arg::RemoveEntityArg, set_entity_arg::SetEntityArg};
                let values = pending.as_mut().ok_or_else(|| invalid("IFC primary argument receiver missing"))?;
                let removals = self.state.arity.saturating_sub(self.state.target_arity);
                let removed = values.len().saturating_sub(usize::from(!self.state.repair));
                if removed < removals {
                    values.push(IfcMutation::RemoveEntityArg(RemoveEntityArg { id: self.state.id, index: self.state.arity - removed - 1 }));
                } else if self.state.next == self.state.target_arity {
                    if values.len() != self.state.primitive_total {
                        return Err(invalid("IFC primary sparse batch count changed").into());
                    }
                    self.state.published = Some((values.as_ptr() as usize, values.len()));
                    self.state.phase = 6;
                } else {
                    let index = self.state.next;
                    let ordinal = *self.root_slots.as_ref().and_then(|slots| slots.get(index)).ok_or_else(|| invalid("IFC primary argument ordinal missing"))?;
                    if ordinal == usize::MAX {
                        self.state.next += 1;
                    } else if ordinal != values.len() {
                        return Err(invalid("IFC primary prepared ordinal changed").into());
                    } else if self.remap.map.as_ref().and_then(|map| map.get(index)) == Some(&usize::MAX) {
                        values.push(if index < self.state.arity {
                            IfcMutation::SetEntityArg(SetEntityArg { id: self.state.id, index, value: IfcValue::Unset })
                        } else {
                            IfcMutation::InsertEntityArg(InsertEntityArg { id: self.state.id, index, value: IfcValue::Unset })
                        });
                        self.state.next += 1;
                    } else if let Some(ordinal) = self.remap.map.as_ref().and_then(|map| map.get(index)).and_then(|value| super::type_input::input_ordinal(*value)) {
                        if value.is_some() {
                            return Err(invalid("IFC supplied type value receiver occupied").into());
                        }
                        *value = Some(self.inputs.take_argument(ordinal).ok_or_else(|| invalid("IFC supplied type argument missing"))?);
                        self.state.phase = 21;
                    } else {
                        if value.is_some() || !value_cursor.terminal_is_empty() {
                            return Err(invalid("IFC primary payload owner occupied").into());
                        }
                        self.state.phase = 19;
                    }
                }
            }
            14 => {
                let values = pending.as_mut().ok_or_else(|| invalid("IFC primary argument primitive receiver missing"))?;
                if self.state.next == self.state.arity.max(self.state.target_arity) {
                    self.state.published = Some((values.as_ptr() as usize, values.len()));
                    self.state.phase = 6;
                } else {
                    use semio_s_artifact_stdio_ifc_4_mutations::standards::v4::subsets::any::schema::mutations::{insert_entity_arg::InsertEntityArg, remove_entity_arg::RemoveEntityArg};
                    values.push(if self.state.arity == 0 {
                        IfcMutation::InsertEntityArg(InsertEntityArg { id: self.state.id, index: self.state.next, value: IfcValue::Unset })
                    } else {
                        IfcMutation::RemoveEntityArg(RemoveEntityArg { id: self.state.id, index: self.state.arity - 1 - self.state.next })
                    });
                    self.state.next += 1;
                }
            }
            4 => {
                let length = self.length(snapshot);
                let old = parts.as_mut().ok_or_else(|| invalid("IFC constituent roster copied roster missing"))?;
                if self.state.next == length {
                    self.state.phase = 5;
                } else {
                    let index = self.state.next;
                    let start = self.state.origin;
                    let end = self.state.destination;
                    if self.state.insertion && index == self.state.destination {
                        let name = Self::insertion_name_mut(event).ok_or_else(|| invalid("IFC constituent original insertion name owner missing"))?;
                        name.make_ascii_uppercase();
                        self.rebuilt.as_mut().unwrap().push(IfcComplexType { name: std::mem::take(name), args: Vec::new() });
                    } else {
                        let origin = if self.state.insertion {
                            index - usize::from(index > self.state.destination)
                        } else if self.state.removal {
                            index + usize::from(index >= start)
                        } else if index == end {
                            start
                        } else if start < end && (start..end).contains(&index) {
                            index + 1
                        } else if start > end && (end + 1..=start).contains(&index) {
                            index - 1
                        } else {
                            index
                        };
                        if self.state.renaming && index == self.state.origin {
                            if self.state.reshaping {
                                std::mem::swap(&mut old[origin].args, self.retyped_args.as_mut().ok_or_else(|| invalid("IFC constituent default tuple owner missing"))?);
                            }
                            let name = Self::type_name_mut(event, address).ok_or_else(|| invalid("IFC constituent original type name owner missing"))?;
                            name.make_ascii_uppercase();
                            std::mem::swap(&mut old[origin].name, name);
                        }
                        self.rebuilt.as_mut().unwrap().push(std::mem::take(&mut old[origin]));
                    }
                    self.state.next += 1;
                }
            }
            5 => {
                if pending.is_some() {
                    return Err(invalid("IFC constituent move receiver occupied").into());
                }
                let values = pending.get_or_insert_with(Vec::new);
                values.try_reserve_exact(1).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "IFC constituent move primitive allocation failed"))?;
                cx.consume_retained(RetainedCloneProgress { retained_capacity_bytes: values.capacity() * size_of::<IfcMutation>(), ..Default::default() })?;
                values.push(IfcMutation::SetEntityComplex(semio_s_artifact_stdio_ifc_4_mutations::standards::v4::subsets::any::schema::mutations::set_entity_complex::SetEntityComplex { id: self.state.id, complex: self.rebuilt.take().unwrap() }));
                self.state.published = Some((values.as_ptr() as usize, values.len()));
                self.state.phase = 6;
            }
            _ => return Err(invalid("IFC constituent move phase invalid").into()),
        }
        if self.state.mapping && self.state.phase == 8 {
            return Err(invalid("IFC retype candidate needs unsupported normalization").into());
        }
        Ok(self.state.phase == 6)
    }
    pub(super) fn begin_close(&mut self) {
        self.state.closing = true;
        self.inputs.begin_close();
    }
    pub(super) fn terminal_is_empty(&self) -> bool {
        self.state.binding.is_none() && self.rebuilt.is_none() && self.retyped_args.is_none() && self.remap.map.is_none() && self.root_slots.is_none() && self.inputs.terminal_is_empty() && self.retirement.is_empty()
    }
    pub(super) fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if !self.inputs.terminal_is_empty() {
            return self.inputs.close_demands(body);
        }
        if !self.retirement.is_empty() {
            self.retirement.next_owner_demands::<Vec<IfcValue>>(false, &None, body)
        } else if self.root_slots.is_some() {
            self.retirement.next_owner_demands::<Vec<usize>>(true, &None, body)
        } else if self.remap.map.is_some() {
            self.retirement.next_owner_demands::<Vec<usize>>(true, &None, body)
        } else if self.retyped_args.is_some() {
            self.retirement.next_owner_demands::<Vec<IfcValue>>(true, &None, body)
        } else if self.rebuilt.is_some() {
            self.retirement.next_owner_demands::<Vec<IfcComplexType>>(self.rebuilt.is_some(), &None, body)
        } else {
            Ok(RetirementDemand { copy_bytes: 2 * size_of::<State>() + size_of::<super::type_remap::Plan>(), depth: 1, ..Default::default() })
        }
    }
    pub(super) fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.state.closing {
            return Err(invalid("IFC constituent move close not started"));
        }
        if !self.inputs.terminal_is_empty() {
            return self.inputs.close_step(grant);
        }
        if !self.retirement.is_empty() {
            return self.retirement.step_granted(grant);
        }
        if self.root_slots.is_some() {
            return self.retirement.begin_granted(&mut self.root_slots, grant)?.ok_or_else(|| invalid("IFC primary ordinal retirement missing"));
        }
        if self.remap.map.is_some() {
            return self.retirement.begin_granted(&mut self.remap.map, grant)?.ok_or_else(|| invalid("IFC retype ordinal map retirement missing"));
        }
        if self.retyped_args.is_some() {
            return self.retirement.begin_granted(&mut self.retyped_args, grant)?.ok_or_else(|| invalid("IFC constituent tuple retirement missing"));
        }
        if self.rebuilt.is_some() {
            return self.retirement.begin_granted(&mut self.rebuilt, grant)?.ok_or_else(|| invalid("IFC constituent move retirement missing"));
        }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if !retained_clone_grant_admits(grant, demand) {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        self.state.binding = None;
        self.state.authority = None;
        self.remap.clear_metadata();
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))
    }
    #[cfg(test)]
    pub(super) fn metadata(&self) -> (State, usize, usize, usize, usize, [usize; 18], usize, usize, (super::type_input::State, usize, usize, bool, bool)) {
        (
            self.state,
            self.rebuilt.as_ref().map_or(0, Vec::len),
            self.rebuilt.as_ref().map_or(0, Vec::capacity),
            self.retyped_args.as_ref().map_or(0, Vec::len),
            self.retyped_args.as_ref().map_or(0, Vec::capacity),
            self.remap.metadata(),
            self.root_slots.as_ref().map_or(0, Vec::len),
            self.root_slots.as_ref().map_or(0, Vec::capacity),
            self.inputs.metadata(),
        )
    }
    #[cfg(test)]
    pub(super) fn phase(&self) -> u8 {
        self.state.phase
    }
}
