//! 🏗️ Each source coordinate, merge comparison, selected scalar and carrier has an explicit planning frontier.
use super::super::receive::{fixed, invalid};
use super::super::*;
use semio_framework_value::{RetirementDemand, ValueError};
use std::mem::size_of;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    Allocate,
    Index,
    Merge,
    Prepare,
    Controls,
    Attachments,
    TypeSets,
    Enumerations,
    EnumLiteral,
    Keys,
    IdentitySet,
    Identities,
    Guids,
    Values,
    Enumeration,
    Entry,
    Set,
    TypeCopy,
    Attach,
    Done,
}
pub(crate) struct Planner {
    pub application: Option<IfcTemplateApplication>,
    pub mutations: Option<Vec<IfcMutation>>,
    pub order: Option<Vec<usize>>,
    pub scratch: Option<Vec<usize>>,
    pub histories: Option<Vec<bool>>,
    pub properties: Option<Vec<IfcValue>>,
    pub enumeration_values: Option<Vec<IfcValue>>,
    pub garbage: Option<IfcValue>,
    prepared: Option<Prepared>,
    phase: Phase,
    index: usize,
    member: usize,
    selected: usize,
    literal: usize,
    lane: usize,
    matched: bool,
    width: usize,
    start: usize,
    left: usize,
    right: usize,
    end: usize,
    keys: [Option<usize>; MAXIMUM_IDENTITIES],
    key_count: usize,
    set: u64,
}
impl Default for Planner {
    fn default() -> Self {
        Self {
            application: None,
            mutations: None,
            order: None,
            scratch: None,
            histories: None,
            properties: None,
            enumeration_values: None,
            garbage: None,
            prepared: None,
            phase: Phase::Allocate,
            index: 0,
            member: 0,
            selected: 0,
            literal: 0,
            lane: 0,
            matched: false,
            width: 1,
            start: 0,
            left: 0,
            right: 1,
            end: 0,
            keys: [None; MAXIMUM_IDENTITIES],
            key_count: 0,
            set: 0,
        }
    }
}
fn capacity(count: usize, item: usize) -> Result<usize, ValueError> {
    count.checked_mul(item).ok_or_else(|| invalid("Template planning capacity overflow"))
}
fn entity_capacity(name: &str) -> Result<usize, ValueError> {
    capacity(attributes(name).ok_or_else(|| invalid("Template carrier declaration is missing"))?.len(), size_of::<IfcValue>())?.checked_add(name.len()).ok_or_else(|| invalid("Template entity capacity overflow"))
}
fn scalar(value: &IfcValue, domain: &str) -> TemplateResult<()> {
    match value {
        IfcValue::Unset | IfcValue::Derived | IfcValue::Reference(_) => Err(error("invalidDomain", "A selected scalar must be explicit")),
        IfcValue::TypedValue(value) if !value.name.eq_ignore_ascii_case(domain) || value.items.len() != 1 => Err(error("invalidDomain", "Typed scalar must match the published domain exactly")),
        _ => Ok(()),
    }
}
fn number(value: &IfcValue) -> Option<(i64, Option<f64>)> {
    let value = match value {
        IfcValue::TypedValue(value) if value.items.len() == 1 => &value.items[0],
        value => value,
    };
    match value {
        IfcValue::Integer(value) => Some((*value, None)),
        IfcValue::Real(value) if value.is_finite() => Some((0, Some(*value))),
        _ => None,
    }
}
fn compare(a: &IfcValue, b: &IfcValue) -> Option<std::cmp::Ordering> {
    let (a, ar) = number(a)?;
    let (b, br) = number(b)?;
    Some(match (ar, br) {
        (None, None) => a.cmp(&b),
        (Some(a), Some(b)) => a.partial_cmp(&b)?,
        (None, Some(b)) => compare_integer_real(a, b),
        (Some(a), None) => compare_integer_real(b, a).reverse(),
    })
}
fn compare_integer_real(integer: i64, real: f64) -> std::cmp::Ordering {
    if real >= 9_223_372_036_854_775_808.0 {
        return std::cmp::Ordering::Less;
    }
    if real < -9_223_372_036_854_775_808.0 {
        return std::cmp::Ordering::Greater;
    }
    let truncated = real as i64;
    match integer.cmp(&truncated) {
        std::cmp::Ordering::Equal if real.fract() > 0.0 => std::cmp::Ordering::Less,
        std::cmp::Ordering::Equal if real.fract() < 0.0 => std::cmp::Ordering::Greater,
        order => order,
    }
}
fn reference_container(value: Option<&IfcValue>) -> Option<&IfcValue> {
    match value? {
        IfcValue::TypedValue(value) if value.name.eq_ignore_ascii_case("IFCPROPERTYSETDEFINITIONSET") && value.items.len() == 1 => value.items.first(),
        value => Some(value),
    }
}
fn immediate(value: Option<&IfcValue>, index: usize) -> Option<u64> {
    match reference_container(value)? {
        IfcValue::Reference(id) if index == 0 => Some(*id),
        IfcValue::Aggregate(values) => values.get(index).and_then(IfcValue::as_reference),
        _ => None,
    }
}
fn immediate_len(value: Option<&IfcValue>) -> usize {
    match reference_container(value) {
        Some(IfcValue::Reference(_)) => 1,
        Some(IfcValue::Aggregate(values)) => values.len(),
        _ => 0,
    }
}
fn unit(form: &IfcTemplateForm) -> Option<u64> {
    match form {
        IfcTemplateForm::Enumerated { unit, .. } => *unit,
        _ => None,
    }
}
fn guid(value: &str) -> bool {
    value.len() == 22 && matches!(value.as_bytes()[0], b'0'..=b'3') && value.as_bytes()[1..].iter().all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'$'))
}
impl Planner {
    fn entry_declaration(&self) -> &'static str {
        match &self.app().entries[self.selected].form {
            IfcTemplateForm::Single { .. } => "IFCPROPERTYSINGLEVALUE",
            IfcTemplateForm::Reference { .. } => "IFCPROPERTYREFERENCEVALUE",
            IfcTemplateForm::Enumerated { .. } => "IFCPROPERTYENUMERATEDVALUE",
            IfcTemplateForm::List { .. } => "IFCPROPERTYLISTVALUE",
            IfcTemplateForm::Bounded { .. } => "IFCPROPERTYBOUNDEDVALUE",
            IfcTemplateForm::Table { .. } => "IFCPROPERTYTABLEVALUE",
            IfcTemplateForm::Quantity { kind, .. } => kind.declaration().0,
        }
    }
    fn set_declaration(&self) -> &'static str {
        match self.plan().template.kind {
            "PSET_MATERIALDRIVEN" => "IFCMATERIALPROPERTIES",
            "PSET_PROFILEDRIVEN" => "IFCPROFILEPROPERTIES",
            kind if kind.starts_with("QTO_") => "IFCELEMENTQUANTITY",
            _ => "IFCPROPERTYSET",
        }
    }
    pub fn done(&self) -> bool {
        self.phase == Phase::Done
    }
    pub fn identity_frontier(&self)->bool{self.phase==Phase::IdentitySet}
    pub fn identity_count(&self)->Option<usize>{self.identity_frontier().then_some(self.key_count)}
    pub fn identity_requirement(&self,index:usize)->Option<(usize,bool)>{
        if !self.identity_frontier()||index>=self.key_count{return None;}let key=self.keys[index]?;
        let rooted=key>=MAXIMUM_ENTRIES*2&&!(key==MAXIMUM_ENTRIES*2&&["PSET_MATERIALDRIVEN","PSET_PROFILEDRIVEN"].contains(&self.plan().template.kind));Some((key,rooted))
    }
    fn app(&self) -> &IfcTemplateApplication {
        self.application.as_ref().unwrap()
    }
    fn plan(&self) -> &Prepared {
        self.prepared.as_ref().unwrap()
    }
    fn key(&mut self, key: usize) {
        self.keys[self.key_count] = Some(key);
        self.key_count += 1;
    }
    fn identity_id(&self, key: usize) -> u64 {
        self.app().identities.iter().find(|identity| key_matches(key, &identity.key)).unwrap().id
    }
    pub fn demand(&self, snapshot: &IfcSnapshot) -> Result<RetirementDemand, ValueError> {
        let mut demand = fixed(size_of::<Self>() + 4 * size_of::<IfcValue>());
        demand.capacity_bytes = match self.phase {
            Phase::Allocate => capacity(snapshot.entities.len(), 2 * size_of::<usize>() + size_of::<bool>())?.checked_add(capacity(MAXIMUM_IDENTITIES, size_of::<IfcMutation>())?).ok_or_else(|| invalid("Template capacity overflow"))?,
            Phase::Values => {
                let selected = &self.app().entries[self.selected];
                let declaration = self.plan().entries[self.selected].unwrap().declaration;
                let domain = if self.lane == 1 { declaration.secondary_type.unwrap_or(declaration.data_type) } else { declaration.data_type };
                if !matches!(selected.form, IfcTemplateForm::Quantity { .. }) && self.current_value(selected).is_some_and(|value| !matches!(value, IfcValue::TypedValue(_))) {
                    size_of::<IfcValue>() + domain.len()
                } else {
                    0
                }
            }
            Phase::Enumeration => {
                let enumeration = self.plan().entries[self.selected].unwrap().declaration.enumeration.unwrap();
                if self.enumeration_values.is_none() {
                    capacity(enumeration.literals.len(), size_of::<IfcValue>())?
                } else if self.literal < enumeration.literals.len() {
                    size_of::<IfcValue>() + enumeration.literals[self.literal].len() + self.plan().entries[self.selected].unwrap().declaration.data_type.len()
                } else {
                    entity_capacity("IFCPROPERTYENUMERATION")?.checked_add(enumeration.name.len()).ok_or_else(|| invalid("Template enumeration capacity overflow"))?
                }
            }
            Phase::TypeCopy => {
                if self.properties.is_none() {
                    let target = &snapshot.entities[self.plan().target];
                    capacity(immediate_len(field(target, "HasPropertySets")) + 1, size_of::<IfcValue>())?
                } else {
                    0
                }
            }
            Phase::Entry => entity_capacity(self.entry_declaration())?,
            Phase::Set => entity_capacity(self.set_declaration())?
                .checked_add(self.plan().template.name.len())
                .and_then(|bytes| self.app().entries.len().checked_mul(size_of::<IfcValue>()).and_then(|values| bytes.checked_add(values)))
                .ok_or_else(|| invalid("Template property-set capacity overflow"))?,
            Phase::Attach => {
                if self.plan().performance && self.member == 0 {
                    entity_capacity("IFCPERFORMANCEHISTORY")?
                } else {
                    entity_capacity(if self.plan().performance && self.member == 1 { "IFCRELASSIGNSTOCONTROL" } else { "IFCRELDEFINESBYPROPERTIES" })?.checked_add(size_of::<IfcValue>()).ok_or_else(|| invalid("Template relation capacity overflow"))?
                }
            }
            _ => 0,
        };
        Ok(demand)
    }
    fn current_value<'a>(&self, selection: &'a IfcTemplateSelection) -> Option<&'a IfcValue> {
        match &selection.form {
            IfcTemplateForm::Single { value, .. } | IfcTemplateForm::Quantity { value, .. } if self.member == 0 => Some(value),
            IfcTemplateForm::Enumerated { values, .. } | IfcTemplateForm::List { values, .. } => values.get(self.member),
            IfcTemplateForm::Bounded { lower, upper, set_point, .. } => [lower.as_ref(), upper.as_ref(), set_point.as_ref()].get(self.member).copied().flatten(),
            IfcTemplateForm::Table { defining, defined, .. } => {
                if self.lane == 0 {
                    defining.get(self.member)
                } else {
                    defined.get(self.member)
                }
            }
            _ => None,
        }
    }
    fn next_value(&mut self) {
        self.member += 1;
    }
    pub fn advance(&mut self, snapshot: &IfcSnapshot) -> TemplateResult<()> {
        match self.phase {
            Phase::Allocate => {
                self.order = Some(Vec::with_capacity(snapshot.entities.len()));
                self.scratch = Some(Vec::with_capacity(snapshot.entities.len()));
                self.histories = Some(Vec::with_capacity(snapshot.entities.len()));
                self.mutations = Some(Vec::with_capacity(MAXIMUM_IDENTITIES));
                self.phase = Phase::Index;
            }
            Phase::Index => {
                if self.index == snapshot.entities.len() {
                    self.index = 0;
                    self.end = (2 * self.width).min(snapshot.entities.len());
                    self.phase = Phase::Merge;
                } else {
                    let entity = &snapshot.entities[self.index];
                    if entity.id == 0 || entity.name.len() > 100 {
                        return Err(error("invalidDomain", "Source graph has an invalid entity identity or declaration"));
                    }
                    self.order.as_mut().unwrap().push(self.index);
                    self.histories.as_mut().unwrap().push(false);
                    self.index += 1;
                }
            }
            Phase::Merge => {
                let count = snapshot.entities.len();
                if self.width >= count {
                    self.phase = Phase::Prepare;
                    self.index = 0;
                } else if self.start >= count {
                    std::mem::swap(&mut self.order, &mut self.scratch);
                    self.scratch.as_mut().unwrap().clear();
                    self.width = self.width.checked_mul(2).ok_or_else(|| error("invalidDomain", "Source index width overflow"))?;
                    self.start = 0;
                    self.left = 0;
                    self.right = self.width.min(count);
                    self.end = (self.width.saturating_mul(2)).min(count);
                } else if self.left == self.right.min(self.start + self.width) && self.right == self.end {
                    self.start = self.end;
                    self.left = self.start;
                    self.right = self.start.saturating_add(self.width).min(count);
                    self.end = self.start.saturating_add(self.width.saturating_mul(2)).min(count);
                } else {
                    let middle = self.start.saturating_add(self.width).min(count);
                    let order = self.order.as_ref().unwrap();
                    let take_left = self.left < middle && (self.right == self.end || snapshot.entities[order[self.left]].id <= snapshot.entities[order[self.right]].id);
                    let index = if take_left {
                        let index = self.left;
                        self.left += 1;
                        index
                    } else {
                        let index = self.right;
                        self.right += 1;
                        index
                    };
                    self.scratch.as_mut().unwrap().push(order[index]);
                }
            }
            Phase::Prepare => {
                if self.index > 0 && self.index < snapshot.entities.len() && snapshot.entities[self.order.as_ref().unwrap()[self.index - 1]].id == snapshot.entities[self.order.as_ref().unwrap()[self.index]].id {
                    return Err(error("identityCollision", "Source graph repeats an entity identity"));
                }
                if self.index < snapshot.entities.len() {
                    self.index += 1;
                } else {
                    let reader = Reader { snapshot, order: self.order.as_deref().unwrap() };
                    self.prepared = Some(prepare(&reader, self.app())?);
                    self.index = 0;
                    self.phase = Phase::Controls;
                }
            }
            Phase::Controls => {
                if !self.plan().performance || self.index == snapshot.entities.len() {
                    self.index = 0;
                    self.member = 0;
                    self.matched = false;
                    self.phase = Phase::Attachments;
                } else {
                    let entity = &snapshot.entities[self.index];
                    if entity.name.eq_ignore_ascii_case("IFCRELASSIGNSTOCONTROL") && self.member < immediate_len(field(entity, "RelatedObjects")) {
                        if immediate(field(entity, "RelatedObjects"), self.member) == Some(self.app().target) {
                            if let Some(id) = field(entity, "RelatingControl").and_then(IfcValue::as_reference) {
                                let reader = Reader { snapshot, order: self.order.as_deref().unwrap() };
                                let position = reader.position(id)?;
                                self.histories.as_mut().unwrap()[position] = true;
                            }
                        }
                        self.member += 1;
                    } else {
                        self.index += 1;
                        self.member = 0;
                    }
                }
            }
            Phase::Attachments => self.attachments(snapshot)?,
            Phase::TypeSets => {
                let target = &snapshot.entities[self.plan().target];
                let values = field(target, "HasPropertySets");
                if self.plan().type_target && self.member < immediate_len(values) {
                    if let Some(id) = immediate(values, self.member) {
                        self.reject_existing(snapshot, id)?;
                    }
                    self.member += 1;
                } else {
                    self.member = 0;
                    self.selected = 0;
                    self.index = 0;
                    self.phase = Phase::Enumerations;
                }
            }
            Phase::Enumerations => self.enumerations(snapshot)?,
            Phase::EnumLiteral => {
                let row = self.plan().entries[self.selected].unwrap();
                let enumeration = row.declaration.enumeration.unwrap();
                let entity = &snapshot.entities[self.index];
                let values = field(entity, "EnumerationValues").and_then(IfcValue::as_aggregate).unwrap();
                if self.literal == enumeration.literals.len() {
                    let entry = self.prepared.as_mut().unwrap().entries[self.selected].as_mut().unwrap();
                    if entry.enumeration.is_none() {
                        entry.enumeration = Some(entity.id);
                    }
                    self.index += 1;
                    self.phase = Phase::Enumerations;
                } else {
                    let expected = enumeration.literals[self.literal];
                    let IfcValue::TypedValue(actual) = &values[self.literal] else {
                        return Err(error("enumConflict", "Existing enumeration differs from its complete released domain"));
                    };
                    if actual.name.bytes().ne(row.declaration.data_type.bytes().map(|byte| byte.to_ascii_uppercase())) || actual.items.len() != 1 || actual.items[0].as_str() != Some(expected) {
                        return Err(error("enumConflict", "Existing enumeration differs from its complete released domain"));
                    }
                    self.literal += 1;
                }
            }
            Phase::Keys => self.keys()?,
            Phase::IdentitySet=>{if self.app().identities.len()!=self.key_count{return Err(error("identityMissing","Application must retain exactly every planned identity"));}self.phase=Phase::Identities;},
            Phase::Identities => self.identities(snapshot)?,
            Phase::Guids => {
                if self.index == snapshot.entities.len() {
                    self.selected = 0;
                    self.member = 0;
                    self.lane = 0;
                    self.phase = Phase::Values;
                } else {
                    if let Some(guid) = text(&snapshot.entities[self.index], "GlobalId") {
                        if self.app().identities.iter().any(|identity| identity.guid.as_deref() == Some(guid)) {
                            return Err(error("duplicateGuid", "Retained GlobalId already exists in source"));
                        }
                    }
                    self.index += 1;
                }
            }
            Phase::Values => self.values()?,
            Phase::Enumeration => self.build_enumeration(snapshot)?,
            Phase::Entry => self.build_entry(snapshot)?,
            Phase::Set => self.build_set(snapshot)?,
            Phase::TypeCopy => {
                let target = &snapshot.entities[self.plan().target];
                let old = field(target, "HasPropertySets");
                if self.properties.is_none() {
                    if !matches!(old, None | Some(IfcValue::Unset | IfcValue::Aggregate(_))) {
                        return Err(error("invalidDomain", "Type HasPropertySets requires an aggregate"));
                    }
                    self.properties = Some(Vec::with_capacity(immediate_len(old) + 1));
                    self.member = 0;
                } else if self.member < immediate_len(old) {
                    let id = immediate(old, self.member).ok_or_else(|| error("invalidDomain", "HasPropertySets requires entity references"))?;
                    self.properties.as_mut().unwrap().push(IfcValue::Reference(id));
                    self.member += 1;
                } else {
                    let properties = self.properties.take().unwrap();
                    self.mutations.as_mut().unwrap().push(type_attachment(target, self.set, properties)?);
                    self.phase = Phase::Done;
                }
            }
            Phase::Attach => self.attach(snapshot)?,
            Phase::Done => {}
        }
        Ok(())
    }
    fn reject_existing(&self, snapshot: &IfcSnapshot, id: u64) -> TemplateResult<()> {
        let reader = Reader { snapshot, order: self.order.as_deref().unwrap() };
        if text(reader.entity(id)?, "Name") == Some(self.plan().template.name) {
            Err(error("existingTemplate", "This released template already exists on the selected carrier"))
        } else {
            Ok(())
        }
    }
    fn attachments(&mut self, snapshot: &IfcSnapshot) -> TemplateResult<()> {
        if self.index == snapshot.entities.len() {
            self.member = 0;
            self.phase = Phase::TypeSets;
            return Ok(());
        }
        let entity = &snapshot.entities[self.index];
        let material = self.plan().template.kind == "PSET_MATERIALDRIVEN";
        let profile = self.plan().template.kind == "PSET_PROFILEDRIVEN";
        if ((material && entity.name.eq_ignore_ascii_case("IFCMATERIALPROPERTIES") && field(entity, "Material").and_then(IfcValue::as_reference) == Some(self.app().target))
            || (profile && entity.name.eq_ignore_ascii_case("IFCPROFILEPROPERTIES") && field(entity, "ProfileDefinition").and_then(IfcValue::as_reference) == Some(self.app().target)))
            && text(entity, "Name") == Some(self.plan().template.name)
        {
            return Err(error("existingTemplate", "This released resource template already exists on target"));
        }
        if entity.name.eq_ignore_ascii_case("IFCRELDEFINESBYPROPERTIES") {
            if !self.matched && self.member < immediate_len(field(entity, "RelatedObjects")) {
                if let Some(id) = immediate(field(entity, "RelatedObjects"), self.member) {
                    let related = if self.plan().performance {
                        let reader = Reader { snapshot, order: self.order.as_deref().unwrap() };
                        reader.position(id).ok().is_some_and(|position| self.histories.as_ref().unwrap()[position])
                    } else {
                        id == self.app().target
                    };
                    if related {
                        self.matched = true;
                        self.member = 0;
                        return Ok(());
                    }
                }
                self.member += 1;
                return Ok(());
            }
            if self.matched && self.member < immediate_len(field(entity, "RelatingPropertyDefinition")) {
                if let Some(id) = immediate(field(entity, "RelatingPropertyDefinition"), self.member) {
                    self.reject_existing(snapshot, id)?;
                }
                self.member += 1;
                return Ok(());
            }
        }
        self.index += 1;
        self.member = 0;
        self.matched = false;
        Ok(())
    }
    fn enumerations(&mut self, snapshot: &IfcSnapshot) -> TemplateResult<()> {
        if self.selected == self.app().entries.len() {
            self.selected = 0;
            self.phase = Phase::Keys;
            return Ok(());
        }
        let row = self.plan().entries[self.selected].unwrap();
        if row.declaration.kind != Kind::EnumeratedValue {
            self.selected += 1;
            self.index = 0;
            return Ok(());
        }
        let enumeration = row.declaration.enumeration.ok_or_else(|| error("invalidDomain", "Published enumeration metadata missing"))?;
        if self.index == snapshot.entities.len() {
            if row.enumeration.is_none() {
                let mut previous = None;
                for index in 0..self.selected {
                    let other = self.plan().entries[index].unwrap();
                    if other.declaration.enumeration.is_some_and(|value| value.name == enumeration.name) {
                        if other.declaration.data_type != row.declaration.data_type || other.declaration.enumeration.unwrap().literals != enumeration.literals || unit(&self.app().entries[index].form) != unit(&self.app().entries[self.selected].form) {
                            return Err(error("enumConflict", "Selected entries require conflicting complete named enumeration domains"));
                        }
                        previous = other.enum_key;
                        break;
                    }
                }
                self.prepared.as_mut().unwrap().entries[self.selected].as_mut().unwrap().enum_key = Some(previous.unwrap_or(self.selected));
            }
            self.selected += 1;
            self.index = 0;
            return Ok(());
        }
        let entity = &snapshot.entities[self.index];
        if entity.name.eq_ignore_ascii_case("IFCPROPERTYENUMERATION") && text(entity, "Name") == Some(enumeration.name) {
            if field(entity, "Unit").and_then(IfcValue::as_reference) != unit(&self.app().entries[self.selected].form) || !matches!(field(entity, "Unit"), Some(IfcValue::Unset | IfcValue::Reference(_))) {
                return Err(error("enumConflict", "Existing named enumeration has a different unit"));
            }
            let values = field(entity, "EnumerationValues").and_then(IfcValue::as_aggregate).ok_or_else(|| error("enumConflict", "Existing enumeration lacks complete values"))?;
            if values.len() != enumeration.literals.len() {
                return Err(error("enumConflict", "Existing enumeration has different complete values"));
            }
            self.literal = 0;
            self.phase = Phase::EnumLiteral;
        } else {
            self.index += 1;
        }
        Ok(())
    }
    fn keys(&mut self) -> TemplateResult<()> {
        if self.selected < self.app().entries.len() {
            self.key(self.selected);
            let entry = self.plan().entries[self.selected].unwrap();
            if entry.enum_key == Some(self.selected) {
                self.key(MAXIMUM_ENTRIES + self.selected);
            }
            self.selected += 1;
        } else {
            self.key(MAXIMUM_ENTRIES * 2);
            if !self.plan().type_target && !["PSET_MATERIALDRIVEN", "PSET_PROFILEDRIVEN"].contains(&self.plan().template.kind) {
                self.key(MAXIMUM_ENTRIES * 2 + 1);
            }
            if self.plan().performance {
                self.key(MAXIMUM_ENTRIES * 2 + 2);
                self.key(MAXIMUM_ENTRIES * 2 + 3);
            }
            self.index = 0;
            self.phase = Phase::IdentitySet;
        }
        Ok(())
    }
    fn identities(&mut self, snapshot: &IfcSnapshot) -> TemplateResult<()> {
        if self.index == self.app().identities.len() {
            self.index = 0;
            self.phase = Phase::Guids;
            return Ok(());
        }
        let identity = &self.app().identities[self.index];
        let key = self.keys[..self.key_count].iter().flatten().find(|key| key_matches(**key, &identity.key)).copied().ok_or_else(|| error("identityUnexpected", "Unexpected retained identity key"))?;
        if identity.id == 0 || self.app().identities[..self.index].iter().any(|other| other.key == identity.key || other.id == identity.id) || (Reader { snapshot, order: self.order.as_deref().unwrap() }).contains(identity.id) {
            return Err(error("identityCollision", "Retained identity is repeated or already exists"));
        }
        let rooted = key >= MAXIMUM_ENTRIES * 2 && !(key == MAXIMUM_ENTRIES * 2 && ["PSET_MATERIALDRIVEN", "PSET_PROFILEDRIVEN"].contains(&self.plan().template.kind));
        if rooted != identity.guid.is_some() {
            return Err(error("identityUnexpected", "Retain GlobalId exactly for rooted planned carriers"));
        }
        if let Some(value) = &identity.guid {
            if !guid(value) {
                return Err(error("invalidDomain", "GlobalId requires canonical IFC encoding"));
            }
            if self.app().identities[..self.index].iter().any(|other| other.guid.as_deref() == Some(value)) {
                return Err(error("duplicateGuid", "Retained GlobalId is repeated"));
            }
        }
        self.index += 1;
        Ok(())
    }
    fn values(&mut self) -> TemplateResult<()> {
        let row = self.plan().entries[self.selected].unwrap();
        let selection = &self.app().entries[self.selected];
        if let IfcTemplateForm::Bounded { lower, upper, set_point, .. } = &selection.form {
            if self.member == 0 && [(lower.as_ref(), upper.as_ref()), (lower.as_ref(), set_point.as_ref()), (set_point.as_ref(), upper.as_ref())].iter().any(|(a, b)| a.zip(*b).and_then(|(a, b)| compare(a, b)).is_some_and(|order| order.is_gt())) {
                return Err(error("invalidDomain", "Bounds must be ordered and contain the set point"));
            }
        }
        let value = self.current_value(selection);
        if value.is_none() {
            let advance_bound = matches!(selection.form, IfcTemplateForm::Bounded { .. }) && self.member < 2;
            let advance_table = matches!(selection.form, IfcTemplateForm::Table { .. }) && self.lane == 0;
            if advance_bound {
                self.member += 1;
            } else if advance_table {
                self.member = 0;
                self.lane = 1;
            } else {
                self.literal = 0;
                self.phase = if row.enum_key == Some(self.selected) { Phase::Enumeration } else { Phase::Entry };
            }
            return Ok(());
        }
        let domain = if self.lane == 1 { row.declaration.secondary_type.unwrap() } else { row.declaration.data_type };
        scalar(value.unwrap(), domain)?;
        if let Some(enumeration) = row.declaration.enumeration {
            let value = match value.unwrap() {
                IfcValue::TypedValue(value) => &value.items[0],
                value => value,
            };
            if !value.as_str().is_some_and(|literal| enumeration.literals.contains(&literal)) {
                return Err(error("invalidDomain", "Selected enum literal is not published"));
            }
        }
        let selection = &mut self.application.as_mut().unwrap().entries[self.selected];
        let quantity = matches!(selection.form, IfcTemplateForm::Quantity { .. });
        let value = match &mut selection.form {
            IfcTemplateForm::Single { value, .. } | IfcTemplateForm::Quantity { value, .. } => value,
            IfcTemplateForm::Enumerated { values, .. } | IfcTemplateForm::List { values, .. } => &mut values[self.member],
            IfcTemplateForm::Bounded { lower, upper, set_point, .. } => [lower, upper, set_point].into_iter().nth(self.member).unwrap().as_mut().unwrap(),
            IfcTemplateForm::Table { defining, defined, .. } => {
                if self.lane == 0 {
                    &mut defining[self.member]
                } else {
                    &mut defined[self.member]
                }
            }
            _ => unreachable!(),
        };
        if quantity {
            if matches!(value, IfcValue::TypedValue(_)) {
                let IfcValue::TypedValue(mut previous) = std::mem::take(value) else { unreachable!() };
                *value = previous.items.pop().unwrap();
                self.garbage = Some(IfcValue::TypedValue(previous));
            }
        } else if !matches!(value, IfcValue::TypedValue(_)) {
            *value = typed(std::mem::take(value), domain)?;
        } else if let IfcValue::TypedValue(value) = value {
            value.name.make_ascii_uppercase();
        }
        self.next_value();
        Ok(())
    }
    fn build_enumeration(&mut self, snapshot: &IfcSnapshot) -> TemplateResult<()> {
        let declaration = self.plan().entries[self.selected].unwrap().declaration;
        let enumeration = declaration.enumeration.unwrap();
        if self.enumeration_values.is_none() {
            self.enumeration_values = Some(Vec::with_capacity(enumeration.literals.len()));
        } else if self.literal < enumeration.literals.len() {
            self.enumeration_values.as_mut().unwrap().push(typed(IfcValue::String(enumeration.literals[self.literal].into()), declaration.data_type)?);
            self.literal += 1;
        } else {
            let values = self.enumeration_values.take().unwrap();
            let unit = unit(&self.app().entries[self.selected].form);
            let entity = entity(
                self.application.as_mut().unwrap(),
                MAXIMUM_ENTRIES + self.selected,
                "IFCPROPERTYENUMERATION",
                [("Name", IfcValue::String(enumeration.name.into())), ("EnumerationValues", IfcValue::Aggregate(values)), ("Unit", reference(unit))],
            )?;
            insert(self.mutations.as_mut().unwrap(), snapshot.entities.len(), entity);
            self.phase = Phase::Entry;
        }
        Ok(())
    }
    fn build_entry(&mut self, snapshot: &IfcSnapshot) -> TemplateResult<()> {
        let row = self.plan().entries[self.selected].unwrap();
        let enumeration = row.enumeration.or_else(|| row.enum_key.map(|key| self.identity_id(MAXIMUM_ENTRIES + key)));
        let selection = &mut self.application.as_mut().unwrap().entries[self.selected];
        let name = IfcValue::String(std::mem::take(&mut selection.name));
        let description = optional_text(selection.description.take());
        let (declaration, fields) = match &mut selection.form {
            IfcTemplateForm::Single { value, unit } => (
                "IFCPROPERTYSINGLEVALUE",
                [("Name", name), ("Specification", description), ("NominalValue", std::mem::take(value)), ("Unit", reference(*unit)), ("", IfcValue::Unset), ("", IfcValue::Unset), ("", IfcValue::Unset), ("", IfcValue::Unset)],
            ),
            IfcTemplateForm::Reference { value, usage_name } => (
                "IFCPROPERTYREFERENCEVALUE",
                [
                    ("Name", name),
                    ("Specification", description),
                    ("PropertyReference", IfcValue::Reference(*value)),
                    ("UsageName", optional_text(usage_name.take())),
                    ("", IfcValue::Unset),
                    ("", IfcValue::Unset),
                    ("", IfcValue::Unset),
                    ("", IfcValue::Unset),
                ],
            ),
            IfcTemplateForm::List { values, unit } => (
                "IFCPROPERTYLISTVALUE",
                [("Name", name), ("Specification", description), ("ListValues", IfcValue::Aggregate(std::mem::take(values))), ("Unit", reference(*unit)), ("", IfcValue::Unset), ("", IfcValue::Unset), ("", IfcValue::Unset), ("", IfcValue::Unset)],
            ),
            IfcTemplateForm::Enumerated { values, .. } => (
                "IFCPROPERTYENUMERATEDVALUE",
                [
                    ("Name", name),
                    ("Specification", description),
                    ("EnumerationValues", IfcValue::Aggregate(std::mem::take(values))),
                    ("EnumerationReference", reference(enumeration)),
                    ("", IfcValue::Unset),
                    ("", IfcValue::Unset),
                    ("", IfcValue::Unset),
                    ("", IfcValue::Unset),
                ],
            ),
            IfcTemplateForm::Bounded { lower, upper, set_point, unit } => (
                "IFCPROPERTYBOUNDEDVALUE",
                [
                    ("Name", name),
                    ("Specification", description),
                    ("LowerBoundValue", lower.take().unwrap_or_default()),
                    ("UpperBoundValue", upper.take().unwrap_or_default()),
                    ("SetPointValue", set_point.take().unwrap_or_default()),
                    ("Unit", reference(*unit)),
                    ("", IfcValue::Unset),
                    ("", IfcValue::Unset),
                ],
            ),
            IfcTemplateForm::Table { defining, defined, defining_unit, defined_unit, expression, interpolation } => (
                "IFCPROPERTYTABLEVALUE",
                [
                    ("Name", name),
                    ("Specification", description),
                    ("DefiningValues", IfcValue::Aggregate(std::mem::take(defining))),
                    ("DefinedValues", IfcValue::Aggregate(std::mem::take(defined))),
                    ("DefiningUnit", reference(*defining_unit)),
                    ("DefinedUnit", reference(*defined_unit)),
                    ("Expression", optional_text(expression.take())),
                    ("CurveInterpolation", interpolation.take().map_or(IfcValue::Unset, IfcValue::Enum)),
                ],
            ),
            IfcTemplateForm::Quantity { kind, value, unit, formula } => {
                let (name_decl, attribute) = kind.declaration();
                (
                    name_decl,
                    [("Name", name), ("Description", description), (attribute, std::mem::take(value)), ("Unit", reference(*unit)), ("Formula", optional_text(formula.take())), ("", IfcValue::Unset), ("", IfcValue::Unset), ("", IfcValue::Unset)],
                )
            }
        };
        let entity = entity(self.application.as_mut().unwrap(), self.selected, declaration, fields)?;
        insert(self.mutations.as_mut().unwrap(), snapshot.entities.len(), entity);
        self.selected += 1;
        self.member = 0;
        self.lane = 0;
        self.phase = if self.selected == self.app().entries.len() { Phase::Set } else { Phase::Values };
        Ok(())
    }
    fn build_set(&mut self, snapshot: &IfcSnapshot) -> TemplateResult<()> {
        let template = self.plan().template;
        let material = template.kind == "PSET_MATERIALDRIVEN";
        let profile = template.kind == "PSET_PROFILEDRIVEN";
        let quantity = template.kind.starts_with("QTO_");
        let mut values = Vec::with_capacity(self.app().entries.len());
        for index in 0..self.app().entries.len() {
            values.push(IfcValue::Reference(self.identity_id(index)));
        }
        let target = self.app().target;
        let application = self.application.as_mut().unwrap();
        let description = optional_text(application.description.take());
        let method = optional_text(application.method_of_measurement.take());
        let entity = entity(
            application,
            MAXIMUM_ENTRIES * 2,
            if material {
                "IFCMATERIALPROPERTIES"
            } else if profile {
                "IFCPROFILEPROPERTIES"
            } else if quantity {
                "IFCELEMENTQUANTITY"
            } else {
                "IFCPROPERTYSET"
            },
            [
                ("Name", IfcValue::String(template.name.into())),
                ("Description", description),
                (
                    if material || profile {
                        "Properties"
                    } else if quantity {
                        "Quantities"
                    } else {
                        "HasProperties"
                    },
                    IfcValue::Aggregate(values),
                ),
                (if material { "Material" } else { "ProfileDefinition" }, IfcValue::Reference(target)),
                ("MethodOfMeasurement", method),
            ],
        )?;
        self.set = entity.id;
        insert(self.mutations.as_mut().unwrap(), snapshot.entities.len(), entity);
        self.phase = if self.plan().type_target {
            Phase::TypeCopy
        } else if material || profile {
            Phase::Done
        } else {
            self.member = 0;
            Phase::Attach
        };
        Ok(())
    }
    fn attach(&mut self, snapshot: &IfcSnapshot) -> TemplateResult<()> {
        if self.plan().performance && self.member == 0 {
            let application = self.application.as_mut().unwrap();
            let performance = application.performance.as_mut().unwrap();
            let fields = [
                ("Name", optional_text(performance.name.take())),
                ("Description", optional_text(performance.description.take())),
                ("ObjectType", optional_text(performance.object_type.take())),
                ("Identification", optional_text(performance.identification.take())),
                ("LifeCyclePhase", IfcValue::String(std::mem::take(&mut performance.life_cycle_phase))),
                ("PredefinedType", IfcValue::Enum(std::mem::take(&mut performance.predefined_type))),
            ];
            let entity = entity(application, MAXIMUM_ENTRIES * 2 + 2, "IFCPERFORMANCEHISTORY", fields)?;
            insert(self.mutations.as_mut().unwrap(), snapshot.entities.len(), entity);
            self.member += 1;
        } else if self.plan().performance && self.member == 1 {
            let target = self.app().target;
            let history = self.identity_id(MAXIMUM_ENTRIES * 2 + 2);
            let entity = entity(self.application.as_mut().unwrap(), MAXIMUM_ENTRIES * 2 + 3, "IFCRELASSIGNSTOCONTROL", [("RelatedObjects", IfcValue::Aggregate(vec![IfcValue::Reference(target)])), ("RelatingControl", IfcValue::Reference(history))])?;
            insert(self.mutations.as_mut().unwrap(), snapshot.entities.len(), entity);
            self.member += 1;
        } else {
            let target = if self.plan().performance { self.identity_id(MAXIMUM_ENTRIES * 2 + 2) } else { self.app().target };
            let entity = entity(
                self.application.as_mut().unwrap(),
                MAXIMUM_ENTRIES * 2 + 1,
                "IFCRELDEFINESBYPROPERTIES",
                [("RelatedObjects", IfcValue::Aggregate(vec![IfcValue::Reference(target)])), ("RelatingPropertyDefinition", IfcValue::Reference(self.set))],
            )?;
            insert(self.mutations.as_mut().unwrap(), snapshot.entities.len(), entity);
            self.phase = Phase::Done;
        }
        Ok(())
    }
}
