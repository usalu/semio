//! 🧵️ Original-source declaration admission advances borrowed mutation arguments one unit per turn.
use super::domain_cursor::{DomainCursor, DomainPopulation, canonical_name};
use super::*;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, retained_clone_grant_admits};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use std::mem::size_of;
#[cfg(test)]
#[path = "domain_work_tests.rs"]
mod tests;
impl DomainPopulation for IfcSnapshot {
    fn len(&self) -> usize {
        self.entities.len()
    }
    fn id(&self, row: usize) -> u64 {
        self.entities[row].id
    }
    fn constituent_count(&self, row: usize) -> usize {
        self.entities[row].complex.len() + 1
    }
    fn constituent_name(&self, row: usize, part: usize) -> &str {
        if part == 0 { &self.entities[row].name } else { &self.entities[row].complex[part - 1].name }
    }
}
struct Population<'a> {
    snapshot: &'a IfcSnapshot,
    inserted: Option<&'a IfcEntity>,
    complex: Option<(usize, &'a [IfcComplexType])>,
    primary: Option<(usize, &'a str)>,
}
impl DomainPopulation for Population<'_> {
    fn len(&self) -> usize {
        self.snapshot.entities.len() + usize::from(self.inserted.is_some())
    }
    fn id(&self, row: usize) -> u64 {
        if row < self.snapshot.entities.len() { self.snapshot.entities[row].id } else { self.inserted.unwrap().id }
    }
    fn constituent_count(&self, row: usize) -> usize {
        if let Some((position, parts)) = self.complex {
            if position == row {
                return parts.len() + 1;
            }
        }
        if row < self.snapshot.entities.len() { self.snapshot.constituent_count(row) } else { self.inserted.unwrap().complex.len() + 1 }
    }
    fn constituent_name(&self, row: usize, part: usize) -> &str {
        if let Some((position, name)) = self.primary {
            if position == row && part == 0 {
                return name;
            }
        }
        if let Some((position, parts)) = self.complex {
            if position == row && part > 0 {
                return &parts[part - 1].name;
            }
        }
        if row < self.snapshot.entities.len() {
            self.snapshot.constituent_name(row, part)
        } else {
            let entity = self.inserted.unwrap();
            if part == 0 { &entity.name } else { &entity.complex[part - 1].name }
        }
    }
}
struct Task<'a> {
    before: Option<&'a IfcValue>,
    after: &'a IfcValue,
    attribute: &'static IfcAttributeDeclaration,
    index: usize,
    part: Option<usize>,
    creation: bool,
}
#[derive(Default)]
pub(super) struct DomainWork {
    binding: Option<(usize, usize, usize, usize, usize, usize, usize)>,
    closing: bool,
    cursor: DomainCursor,
    mutation: usize,
    part: usize,
    argument: usize,
    derive: usize,
    derive_attribute: usize,
    context: Option<usize>,
    context_scan: usize,
    context_checked: bool,
    readonly: bool,
    active: bool,
    initializing: bool,
    checked: bool,
    done: bool,
    #[cfg(test)]
    turns: usize,
    #[cfg(test)]
    cancel_at: Option<usize>,
}
impl DomainWork {
    pub(super) fn demands(&self) -> RetirementDemand {
        RetirementDemand { copy_bytes: 4 * size_of::<Self>() + 4 * 256 + 256 * size_of::<usize>(), depth: 1, ..Default::default() }
    }
    pub(super) fn begin_close(&mut self) {
        self.closing = true;
    }
    pub(super) fn terminal_is_empty(&self) -> bool {
        self.binding.is_none()
    }
    pub(super) fn close_demands(&self) -> RetirementDemand {
        RetirementDemand { copy_bytes: size_of::<Option<(usize, usize, usize, usize, usize, usize, usize)>>() + 3 * size_of::<bool>(), depth: 1, ..Default::default() }
    }
    pub(super) fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.closing {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "IFC domain metadata close was not started"));
        }
        let demand = self.close_demands();
        if !retained_clone_grant_admits(grant, demand) {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        self.active = false;
        self.initializing = false;
        self.done = true;
        self.binding = None;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))
    }
    #[cfg(test)]
    fn metadata(&self) -> (Option<(usize, usize, usize, usize, usize, usize, usize)>, [usize; 14]) {
        (
            self.binding,
            [
                self.mutation,
                self.part,
                self.argument,
                self.derive,
                self.derive_attribute,
                self.context.unwrap_or(usize::MAX),
                self.context_scan,
                self.context_checked as usize,
                self.readonly as usize,
                self.active as usize,
                self.initializing as usize,
                self.checked as usize,
                self.done as usize,
                self.closing as usize,
            ],
        )
    }
    #[cfg(test)]
    pub(super) fn cancel_at_turn(&mut self, turn: usize) {
        self.cancel_at = Some(turn);
    }
    #[cfg(test)]
    pub(super) fn turns(&self) -> usize {
        self.turns
    }
    fn next(&mut self) {
        self.active = false;
        self.initializing = false;
        self.derive = 0;
        self.readonly = false;
        self.argument += 1;
    }
    fn attribute<C: IfcDetailsProfile>(name: &str, index: usize, direct: bool) -> Result<&'static IfcAttributeDeclaration, String> {
        if name.len() > 256 {
            return Err("ifc-domain-entity".into());
        }
        let mut bytes = [0; 256];
        let declaration = C::catalog().entities.get(canonical_name(name, &mut bytes).ok_or("ifc-domain-entity")?).ok_or("ifc-domain-entity")?;
        let attributes = if direct { &declaration.attributes } else { &declaration.all_attributes };
        attributes.get(index).ok_or_else(|| "ifc-domain-attribute".into())
    }
    fn task<'a, C: IfcDetailsProfile>(
        &mut self,
        event: &editing::SnapshotEditEvent,
        path: &str,
        structural: bool,
        entity: &'a IfcEntity,
        mutations: &'a [IfcMutation],
        direct: bool,
        creation: bool,
        strict_candidate: Option<&super::constituent_roster::RetainedTypeAdmission<'_>>,
    ) -> Result<Option<Task<'a>>, String> {
        if let Some(admission) = strict_candidate.filter(|admission| admission.primary_arity().is_some()) {
            let name = admission.primary_name(entity, mutations).ok_or("ifc-domain-primary-name")?;
            let mut bytes = [0; 256];
            let declaration = C::catalog().entities.get(canonical_name(name, &mut bytes).ok_or("ifc-domain-entity")?).ok_or("ifc-domain-entity")?;
            let count = admission.primary_arity().ok_or("ifc-domain-primary-role")?;
            if count != declaration.attributes.len() {
                return Err("ifc-domain-argument-arity".into());
            }
            if self.argument == count {
                self.done = true;
                return Ok(None);
            }
            return Ok(Some(Task {
                before: None,
                after: admission.primary_argument(entity, mutations, self.argument).ok_or("ifc-domain-primary-argument")?,
                attribute: Self::attribute::<C>(name, self.argument, true)?,
                index: self.argument,
                part: None,
                creation: false,
            }));
        }
        let Some(mutation) = mutations.get(self.mutation) else {
            self.done = true;
            return Ok(None);
        };
        match mutation {
            IfcMutation::InsertEntity(value) if creation => {
                let (part, name, values) = if self.part == 0 {
                    (None, value.entity.name.as_str(), value.entity.args.as_slice())
                } else if let Some(part) = value.entity.complex.get(self.part - 1) {
                    (Some(self.part - 1), part.name.as_str(), part.args.as_slice())
                } else {
                    self.mutation += 1;
                    self.part = 0;
                    self.argument = 0;
                    return Ok(None);
                };
                let direct = part.is_some() || !value.entity.complex.is_empty();
                let mut bytes = [0; 256];
                let declaration = C::catalog().entities.get(canonical_name(name, &mut bytes).ok_or("ifc-domain-entity")?).ok_or("ifc-domain-entity")?;
                let attributes = if direct { &declaration.attributes } else { &declaration.all_attributes };
                if values.len() != attributes.len() {
                    return Err("ifc-domain-argument-arity".into());
                }
                if self.argument == values.len() {
                    self.part += 1;
                    self.argument = 0;
                    return Ok(None);
                }
                Ok(Some(Task { before: None, after: &values[self.argument], attribute: Self::attribute::<C>(name, self.argument, direct)?, index: self.argument, part, creation: true }))
            }
            IfcMutation::SetEntityArg(value) => {
                if self.argument != 0 {
                    self.mutation += 1;
                    self.argument = 0;
                    return Ok(None);
                }
                if value.id != entity.id {
                    return Err("ifc-domain-identity".into());
                }
                Ok(Some(Task { before: entity.args.get(value.index), after: &value.value, attribute: Self::attribute::<C>(&entity.name, value.index, direct)?, index: value.index, part: None, creation: false }))
            }
            IfcMutation::InsertEntityArg(value) => {
                if self.argument != 0 {
                    self.mutation += 1;
                    self.argument = 0;
                    return Ok(None);
                }
                if value.id != entity.id || value.index < entity.args.len() {
                    return Err("ifc-domain-argument-arity".into());
                }
                Ok(Some(Task { before: None, after: &value.value, attribute: Self::attribute::<C>(&entity.name, value.index, direct)?, index: value.index, part: None, creation: false }))
            }
            IfcMutation::RemoveEntityArg(value) => {
                if entity.name.len() > 256 {
                    return Err("ifc-domain-entity".into());
                }
                let mut bytes = [0; 256];
                let declaration = C::catalog().entities.get(canonical_name(&entity.name, &mut bytes).ok_or("ifc-domain-entity")?).ok_or("ifc-domain-entity")?;
                let count = if entity.complex.is_empty() { declaration.all_attributes.len() } else { declaration.attributes.len() };
                if value.index < count {
                    return Err("ifc-domain-argument-arity".into());
                }
                self.mutation += 1;
                Ok(None)
            }
            IfcMutation::SetEntityComplex(value) => {
                if value.id != entity.id {
                    return Err("ifc-domain-identity".into());
                }
                if self.part == value.complex.len() {
                    self.mutation += 1;
                    self.part = 0;
                    self.argument = 0;
                    return Ok(None);
                }
                let next = &value.complex[self.part];
                let before = domain_edit::prior_component(event, path, self.part, entity.complex.len()).and_then(|index| entity.complex.get(index)).filter(|part| part.name.eq_ignore_ascii_case(&next.name));
                if next.name.len() > 256 {
                    return Err("ifc-domain-entity".into());
                }
                let mut bytes = [0; 256];
                let declaration = C::catalog().entities.get(canonical_name(&next.name, &mut bytes).ok_or("ifc-domain-entity")?).ok_or("ifc-domain-entity")?;
                if next.args.len() != declaration.attributes.len() && before.is_none_or(|before| before.args.len() != next.args.len()) {
                    return Err("ifc-domain-argument-arity".into());
                }
                if self.argument == next.args.len() {
                    self.part += 1;
                    self.argument = 0;
                    return Ok(None);
                }
                Ok(Some(Task {
                    before: before.and_then(|before| before.args.get(self.argument)),
                    after: &next.args[self.argument],
                    attribute: Self::attribute::<C>(&next.name, self.argument, true)?,
                    index: self.argument,
                    part: Some(self.part),
                    creation: structural && before.is_none() && strict_candidate.is_none(),
                }))
            }
            _ => {
                self.mutation += 1;
                Ok(None)
            }
        }
    }
    pub(super) fn advance<C: IfcDetailsProfile>(
        &mut self,
        event: &editing::SnapshotEditEvent,
        snapshot: &IfcSnapshot,
        mutations: &[IfcMutation],
        address: &pointer::PathAdmission,
        retained_type: Option<super::constituent_roster::RetainedTypeAdmission<'_>>,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if cx.is_cancelled() {
            return Err(Fault::from("details-cancelled").into());
        }
        let policy = retained_type.as_ref().map(|admission| admission.validate::<C>(event, snapshot, address, mutations, cx)).transpose()?;
        let binding = (event as *const _ as usize, snapshot as *const _ as usize, mutations.as_ptr() as usize, mutations.len(), address as *const _ as usize, C::catalog() as *const _ as usize, policy.unwrap_or(0));
        if self.binding.is_some_and(|original| original != binding) || self.closing {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "IFC domain original owner changed or closed").into());
        }
        let demand = self.demands();
        if cx.should_yield() || !retained_clone_grant_admits(cx.retained_grant(), demand) {
            return Ok(false);
        }
        if self.done {
            return Ok(true);
        }
        #[cfg(test)]
        {
            self.turns += 1;
            if self.cancel_at == Some(self.turns) {
                cx.original_cancel_token().cancel_now();
                return Err(ValueError::literal(ValueRefusalKind::Canceled, "IFC original domain metadata cancelled").into());
            }
        }
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })?;
        cx.consume_fuel(1);
        self.binding = Some(binding);
        if !self.checked {
            self.checked = true;
            if domain_edit::draft(event) && policy.is_none() {
                self.done = true;
                return Ok(true);
            }
            return Ok(false);
        }
        let Some((path, operation)) = domain_edit::event_path(event) else {
            self.done = true;
            return Ok(true);
        };
        let mut fields = path.split('/').skip(1);
        if fields.next() != Some("entities") {
            self.done = true;
            return Ok(true);
        }
        let Some(position) = fields.next().and_then(|position| if position == "-" { Some(snapshot.entities.len()) } else { position.parse::<usize>().ok() }) else {
            self.done = true;
            return Ok(true);
        };
        let inserted = domain_edit::inserted_entity(event, path, position, mutations).map_err(Fault::from)?;
        let entity = inserted.or_else(|| snapshot.entities.get(position)).ok_or_else(|| Fault::from("ifc-domain-entity"))?;
        if !self.context_checked {
            if let Some(mutation) = mutations.get(self.context_scan) {
                if let IfcMutation::SetEntityComplex(value) = mutation {
                    if value.id != entity.id {
                        return Err(Fault::from("ifc-domain-identity").into());
                    }
                    self.context = Some(self.context_scan);
                }
                self.context_scan += 1;
            } else {
                self.context_checked = true;
            }
            return Ok(false);
        }
        let complex = if let Some(index) = self.context {
            let IfcMutation::SetEntityComplex(value) = &mutations[index] else {
                return Err(Fault::from("ifc-domain-context").into());
            };
            &value.complex
        } else {
            &entity.complex
        };
        let structural = address.field_is(path, 2, "complex") && matches!(address.field_count(), 3 | 4);
        let primary = retained_type.as_ref().and_then(|admission| admission.primary_name(entity, mutations).map(|name| (admission.position(), name)));
        let Some(task) = self.task::<C>(event, path, structural, entity, mutations, !complex.is_empty(), inserted.is_some(), retained_type.as_ref()).map_err(Fault::from)? else { return Ok(self.done) };
        if self.active {
            if self
                .cursor
                .advance(C::catalog(), task.before.unwrap_or(task.after), task.after, &Population { snapshot, inserted, complex: retained_type.as_ref().map(|admission| (admission.position(), complex.as_slice())), primary })
                .map_err(Fault::from)?
            {
                self.next();
            }
            return Ok(false);
        }
        if !self.initializing {
            self.readonly = task.attribute.derived;
            self.derive = 0;
            self.derive_attribute = 0;
            self.initializing = true;
            return Ok(false);
        }
        if self.derive <= complex.len() {
            let name = if self.derive == 0 { primary.map(|(_, name)| name).unwrap_or(&entity.name) } else { &complex[self.derive - 1].name };
            if name.len() <= 256 {
                let mut bytes = [0; 256];
                if let Some(declaration) = canonical_name(name, &mut bytes).and_then(|name| C::catalog().entities.get(name)) {
                    if let Some(other) = declaration.all_attributes.get(self.derive_attribute) {
                        self.readonly |= other.derived && other.name == task.attribute.name && other.declared_by == task.attribute.declared_by;
                        self.derive_attribute += 1;
                        return Ok(false);
                    }
                }
            }
            self.derive += 1;
            self.derive_attribute = 0;
            return Ok(false);
        }
        if (structural || task.creation) && self.readonly && matches!(task.after, IfcValue::Derived) || !self.readonly && matches!(task.after, IfcValue::Unset) && (task.creation || structural && matches!(task.before, Some(IfcValue::Derived))) {
            self.next();
            return Ok(false);
        }
        let suffix = address.native_address(event).filter(|native| native.position == position && native.part == task.part && native.argument == task.index).and_then(|native| path.get(native.prefix..));
        let mut indices = [0; 128];
        let length = if !self.readonly { if let (Some(before), Some(suffix)) = (task.before, suffix) { domain_edit::route(before, suffix, &mut indices).map_err(Fault::from)? } else { 0 } } else { 0 };
        self.cursor.start(&task.attribute.type_expr, &indices[..length], operation, task.attribute.optional, task.before.is_none(), self.readonly).map_err(Fault::from)?;
        self.active = true;
        Ok(false)
    }
}
                                                                                                                                                                                                                                                                                                                                                                                                         