//! 📨️ The original decimal application is consumed into the native command under retained grants.
use super::*;
use crate::details::{IfcValueReceiveMode, IfcValueReceiver};
use semio_framework_job::{OriginalTurnFailure, StepContext};
use semio_framework_value::retained_clone::{RetainedCloneClose, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::{DslValue, RetirementDemand, ValueError, ValueRefusalKind};
use std::mem::size_of;

pub(crate) fn invalid(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, message)
}
pub(crate) fn fits(grant: RetainedCloneGrant, demand: RetirementDemand) -> bool {
    grant.maximum_items > 0 && grant.maximum_copy_bytes >= demand.copy_bytes && grant.maximum_capacity_bytes >= demand.capacity_bytes && grant.maximum_release_bytes >= demand.release_bytes && grant.maximum_depth >= demand.depth
}
pub(crate) fn receipt(demand: RetirementDemand) -> RetainedCloneProgress {
    RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: demand.capacity_bytes, released_bytes: demand.release_bytes }
}
pub(crate) fn fixed(copy_bytes: usize) -> RetirementDemand {
    RetirementDemand { copy_bytes, depth: 1, ..Default::default() }
}
fn object(value: &DslValue) -> Result<&[(String, DslValue)], ValueError> {
    match value {
        DslValue::Object(fields) => Ok(fields),
        _ => Err(invalid("Template application requires a closed object")),
    }
}
fn get<'a>(value: &'a DslValue, name: &str) -> Option<&'a DslValue> {
    object(value).ok()?.iter().find(|(key, _)| key == name).map(|(_, value)| value)
}
fn get_mut<'a>(value: &'a mut DslValue, name: &str) -> Result<&'a mut DslValue, ValueError> {
    let DslValue::Object(fields) = value else {
        return Err(invalid("Template field requires an object"));
    };
    fields.iter_mut().find(|(key, _)| key == name).map(|(_, value)| value).ok_or_else(|| invalid("Required template field is missing"))
}
fn shape(value: &DslValue, allowed: &[&str], required: &[&str]) -> Result<(), ValueError> {
    let fields = object(value)?;
    if fields.len() > allowed.len() {
        return Err(invalid("Template input contains unknown or repeated fields"));
    }
    for (index, (name, _)) in fields.iter().enumerate() {
        if !allowed.contains(&name.as_str()) || fields[..index].iter().any(|(previous, _)| previous == name) {
            return Err(invalid("Template input contains unknown or repeated fields"));
        }
    }
    for name in required {
        if get(value, name).is_none() {
            return Err(invalid("Template input omits a required field"));
        }
    }
    Ok(())
}
fn string(value: &DslValue) -> Result<&str, ValueError> {
    match value {
        DslValue::String(value) => Ok(value),
        _ => Err(invalid("Template text field requires a string")),
    }
}
fn take_text(value: &mut DslValue, name: &str) -> Result<String, ValueError> {
    match get_mut(value, name)? {
        DslValue::String(text) => Ok(std::mem::take(text)),
        _ => Err(invalid("Template text field requires a string")),
    }
}
fn take_optional(value: &mut DslValue, name: &str) -> Result<Option<String>, ValueError> {
    if get(value, name).is_none() {
        Ok(None)
    } else {
        take_text(value, name).map(Some)
    }
}
fn id(value: &DslValue) -> Result<u64, ValueError> {
    let text = string(value)?;
    if text.is_empty() || text.len() > 20 || text.starts_with('0') || !text.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(invalid("Template identities require canonical positive decimal u64 strings"));
    }
    text.parse().map_err(|_| invalid("Template identity exceeds u64"))
}
fn ref_id(value: Option<&DslValue>) -> Result<Option<u64>, ValueError> {
    let Some(value) = value else {
        return Ok(None);
    };
    shape(value, &["kind", "value"], &["kind", "value"])?;
    if string(get(value, "kind").unwrap())? != "reference" {
        return Err(invalid("Template units require canonical graph references"));
    }
    id(get(value, "value").unwrap()).map(Some)
}
fn array(value: &DslValue) -> Result<&[DslValue], ValueError> {
    match value {
        DslValue::Array(values) => Ok(values),
        _ => Err(invalid("Template input requires an array")),
    }
}
fn texts(value: &DslValue, names: &[&str]) -> Result<(), ValueError> {
    for name in names {
        if let Some(value) = get(value, name) {
            string(value)?;
        }
    }
    Ok(())
}
fn initial_shape(source: &DslValue) -> Result<(), ValueError> {
    shape(source, &["template", "target", "entries", "identities", "ownerHistory", "description", "methodOfMeasurement", "performance"], &["template", "target", "entries", "identities"])?;
    texts(source, &["template", "description", "methodOfMeasurement"])?;
    id(get(source, "target").unwrap())?;
    ref_id(get(source, "ownerHistory"))?;
    if let Some(value) = get(source, "performance") {
        shape(value, &["lifeCyclePhase", "predefinedType", "objectType", "identification", "name", "description"], &["lifeCyclePhase", "predefinedType"])?;
        texts(value, &["lifeCyclePhase", "predefinedType", "objectType", "identification", "name", "description"])?;
    }
    Ok(())
}
fn row_mut<'a>(value: &'a mut DslValue, lane: &str, index: usize) -> Result<&'a mut DslValue, ValueError> {
    let DslValue::Array(rows) = get_mut(value, lane)? else {
        return Err(invalid("Template input requires an array"));
    };
    rows.get_mut(index).ok_or_else(|| invalid("Template row is missing"))
}
fn kind(value: &DslValue) -> Result<Kind, ValueError> {
    Ok(match string(get(value, "kind").ok_or_else(|| invalid("Template form is required"))?)? {
        "P_SINGLEVALUE" => Kind::SingleValue,
        "P_REFERENCEVALUE" => Kind::ReferenceValue,
        "P_ENUMERATEDVALUE" => Kind::EnumeratedValue,
        "P_LISTVALUE" => Kind::ListValue,
        "P_BOUNDEDVALUE" => Kind::BoundedValue,
        "P_TABLEVALUE" => Kind::TableValue,
        "Q_LENGTH" => Kind::Length,
        "Q_AREA" => Kind::Area,
        "Q_VOLUME" => Kind::Volume,
        "Q_WEIGHT" => Kind::Weight,
        "Q_COUNT" => Kind::Count,
        "Q_TIME" => Kind::Time,
        _ => return Err(invalid("Unknown template form")),
    })
}
fn leaf_fields(kind: Kind) -> &'static [(&'static str, bool)] {
    match kind {
        Kind::SingleValue | Kind::Weight | Kind::Length | Kind::Area | Kind::Volume | Kind::Count | Kind::Time => &[("value", false)],
        Kind::ReferenceValue => &[],
        Kind::EnumeratedValue | Kind::ListValue => &[("values", true)],
        Kind::BoundedValue => &[("lower", false), ("upper", false), ("setPoint", false)],
        Kind::TableValue => &[("defining", true), ("defined", true)],
    }
}
fn entry_shape(value: &DslValue, kind: Kind) -> Result<(), ValueError> {
    match kind {
        Kind::SingleValue => shape(value, &["name", "kind", "description", "value", "unit"], &["name", "kind", "value"]),
        Kind::ReferenceValue => shape(value, &["name", "kind", "description", "value", "usageName"], &["name", "kind", "value"]),
        Kind::EnumeratedValue | Kind::ListValue => shape(value, &["name", "kind", "description", "values", "unit"], &["name", "kind", "values"]),
        Kind::BoundedValue => shape(value, &["name", "kind", "description", "lower", "upper", "setPoint", "unit"], &["name", "kind"]),
        Kind::TableValue => shape(value, &["name", "kind", "description", "defining", "defined", "definingUnit", "definedUnit", "expression", "interpolation"], &["name", "kind", "defining", "defined"]),
        _ => shape(value, &["name", "kind", "description", "value", "unit", "formula"], &["name", "kind", "value"]),
    }
}
#[derive(Clone, Copy)]
enum Phase {
    Empty,
    Initialize,
    Identities,
    Entry,
    Leaf,
    ReceiveLeaf,
    TakeLeaf,
    DeliverLeaf,
    CloseLeaf,
    FinishEntry,
    Finish,
    Ready,
    Spent,
}
pub struct ApplicationReceiver {
    source: Option<DslValue>,
    output: Option<IfcTemplateApplication>,
    pending: Option<IfcTemplateSelection>,
    input: Option<DslValue>,
    value: Option<IfcValue>,
    receiver: IfcValueReceiver,
    retirement: RetainedCloneClose,
    phase: Phase,
    identity: usize,
    entry: usize,
    entry_count: usize,
    leaf: usize,
    item: usize,
    closing: bool,
}
impl Default for ApplicationReceiver {
    fn default() -> Self {
        Self {
            source: None,
            output: None,
            pending: None,
            input: None,
            value: None,
            receiver: IfcValueReceiver::new(IfcValueReceiveMode::Decimal),
            retirement: Default::default(),
            phase: Phase::Empty,
            identity: 0,
            entry: 0,
            entry_count: 0,
            leaf: 0,
            item: 0,
            closing: false,
        }
    }
}
impl ApplicationReceiver {
    pub fn receive_owned(&mut self, original: &mut Option<DslValue>, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        let demand = fixed(2 * size_of::<Option<DslValue>>() + size_of::<Phase>());
        if !fits(cx.retained_grant(), demand) {
            return Ok(false);
        }
        if !matches!(self.phase, Phase::Empty) || self.closing {
            return Err(invalid("Template receiver already owns its original input").into());
        }
        self.source = original.take();
        if self.source.is_none() {
            return Err(invalid("Template original input is missing").into());
        }
        self.phase = Phase::Initialize;
        cx.consume_retained(receipt(demand))?;
        cx.consume_fuel(1);
        Ok(true)
    }
    pub fn demands(&self) -> Result<RetirementDemand, ValueError> {
        if self.closing {
            return self.close_demands(4096);
        }
        let mut demand = fixed(2048);
        let source = self.source.as_ref();
        match self.phase {
            Phase::Initialize => {
                let source = source.ok_or_else(|| invalid("Template original missing"))?;
                initial_shape(source)?;
                let entries = array(get(source, "entries").unwrap())?.len();
                let identities = array(get(source, "identities").unwrap())?.len();
                if entries == 0 || entries > MAXIMUM_ENTRIES || identities > MAXIMUM_IDENTITIES {
                    return Err(invalid("Template selected rows exceed the published release"));
                }
                demand.capacity_bytes = entries
                    .checked_mul(size_of::<IfcTemplateSelection>())
                    .and_then(|bytes| identities.checked_mul(size_of::<IfcTemplateIdentity>()).and_then(|other| bytes.checked_add(other)))
                    .ok_or_else(|| invalid("Template receiving capacity overflow"))?;
            }
            Phase::Entry => {
                let source = source.ok_or_else(|| invalid("Template original missing"))?;
                let row = array(get(source, "entries").unwrap())?.get(self.entry).ok_or_else(|| invalid("Template row missing"))?;
                let kind = kind(row)?;
                entry_shape(row, kind)?;
                let count = leaf_fields(kind)
                    .iter()
                    .filter(|(_, sequence)| *sequence)
                    .try_fold(0usize, |count, (key, _)| array(get(row, key).ok_or_else(|| invalid("Template column missing"))?).and_then(|values| count.checked_add(values.len()).ok_or_else(|| invalid("Template receiving capacity overflow"))))?;
                demand.capacity_bytes = count.checked_mul(size_of::<IfcValue>()).ok_or_else(|| invalid("Template receiving capacity overflow"))?;
            }
            Phase::ReceiveLeaf => return if self.input.is_some() { Ok(fixed(size_of::<DslValue>())) } else { self.receiver.demands() },
            Phase::TakeLeaf => return Ok(fixed(size_of::<IfcValue>())),
            Phase::CloseLeaf => return if self.receiver.terminal_is_empty() { Ok(fixed(size_of::<IfcValueReceiver>() + size_of::<Phase>())) } else { self.receiver.close_demands(4096) },
            Phase::Finish => return if self.source.is_none() && self.retirement.is_empty() { Ok(fixed(size_of::<Phase>())) } else { self.close_source_demand(4096) },
            _ => {}
        }
        Ok(demand)
    }
    fn initialize(&mut self) -> Result<(), ValueError> {
        let source = self.source.as_mut().unwrap();
        let count = array(get(source, "entries").unwrap())?.len();
        let identities = array(get(source, "identities").unwrap())?.len();
        self.entry_count = count;
        let mut application = IfcTemplateApplication {
            template: take_text(source, "template")?,
            target: id(get(source, "target").unwrap())?,
            entries: Vec::with_capacity(count),
            identities: Vec::with_capacity(identities),
            owner_history: ref_id(get(source, "ownerHistory"))?,
            description: take_optional(source, "description")?,
            method_of_measurement: take_optional(source, "methodOfMeasurement")?,
            performance: None,
        };
        if get(source, "performance").is_some() {
            let value = get_mut(source, "performance")?;
            shape(value, &["lifeCyclePhase", "predefinedType", "objectType", "identification", "name", "description"], &["lifeCyclePhase", "predefinedType"])?;
            application.performance = Some(IfcTemplatePerformance {
                life_cycle_phase: take_text(value, "lifeCyclePhase")?,
                predefined_type: take_text(value, "predefinedType")?,
                object_type: take_optional(value, "objectType")?,
                identification: take_optional(value, "identification")?,
                name: take_optional(value, "name")?,
                description: take_optional(value, "description")?,
            });
        }
        self.output = Some(application);
        self.phase = Phase::Identities;
        Ok(())
    }
    fn entry(&mut self) -> Result<(), ValueError> {
        let row = row_mut(self.source.as_mut().unwrap(), "entries", self.entry)?;
        let kind = kind(row)?;
        let unit = ref_id(get(row, "unit"))?;
        texts(row, &["name", "description", "usageName", "expression", "interpolation", "formula"])?;
        ref_id(get(row, "definingUnit"))?;
        ref_id(get(row, "definedUnit"))?;
        let form = match kind {
            Kind::SingleValue => IfcTemplateForm::Single { value: IfcValue::Unset, unit },
            Kind::ReferenceValue => IfcTemplateForm::Reference { value: ref_id(get(row, "value"))?.ok_or_else(|| invalid("Reference property requires its reference"))?, usage_name: take_optional(row, "usageName")? },
            Kind::EnumeratedValue => IfcTemplateForm::Enumerated { values: Vec::with_capacity(array(get(row, "values").unwrap())?.len()), unit },
            Kind::ListValue => IfcTemplateForm::List { values: Vec::with_capacity(array(get(row, "values").unwrap())?.len()), unit },
            Kind::BoundedValue => IfcTemplateForm::Bounded { lower: None, upper: None, set_point: None, unit },
            Kind::TableValue => IfcTemplateForm::Table {
                defining: Vec::with_capacity(array(get(row, "defining").unwrap())?.len()),
                defined: Vec::with_capacity(array(get(row, "defined").unwrap())?.len()),
                defining_unit: ref_id(get(row, "definingUnit"))?,
                defined_unit: ref_id(get(row, "definedUnit"))?,
                expression: take_optional(row, "expression")?,
                interpolation: take_optional(row, "interpolation")?,
            },
            quantity => IfcTemplateForm::Quantity {
                kind: match quantity {
                    Kind::Length => IfcTemplateQuantityKind::Length,
                    Kind::Area => IfcTemplateQuantityKind::Area,
                    Kind::Volume => IfcTemplateQuantityKind::Volume,
                    Kind::Weight => IfcTemplateQuantityKind::Weight,
                    Kind::Count => IfcTemplateQuantityKind::Count,
                    _ => IfcTemplateQuantityKind::Time,
                },
                value: IfcValue::Unset,
                unit,
                formula: take_optional(row, "formula")?,
            },
        };
        self.pending = Some(IfcTemplateSelection { name: take_text(row, "name")?, description: take_optional(row, "description")?, form });
        self.leaf = 0;
        self.item = 0;
        self.phase = Phase::Leaf;
        Ok(())
    }
    pub fn advance(&mut self, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        if cx.is_cancelled() {
            return Err(ValueError::literal(ValueRefusalKind::Canceled, "Template receiving cancelled").into());
        }
        if cx.should_yield() {
            return Ok(false);
        }
        let demand = self.demands()?;
        if !fits(cx.retained_grant(), demand) {
            return Ok(false);
        }
        match self.phase {
            Phase::Initialize => self.initialize()?,
            Phase::Identities => {
                let source = self.source.as_mut().unwrap();
                if self.identity == array(get(source, "identities").unwrap())?.len() {
                    self.phase = Phase::Entry;
                } else {
                    let row = row_mut(source, "identities", self.identity)?;
                    shape(row, &["key", "id", "guid"], &["key", "id"])?;
                    texts(row, &["key", "guid"])?;
                    let id = id(get(row, "id").unwrap())?;
                    self.output.as_mut().unwrap().identities.push(IfcTemplateIdentity { key: take_text(row, "key")?, id, guid: take_optional(row, "guid")? });
                    self.identity += 1;
                }
            }
            Phase::Entry => self.entry()?,
            Phase::Leaf => {
                let rules = leaf_fields(self.pending.as_ref().unwrap().form.kind());
                if self.leaf == rules.len() {
                    self.phase = Phase::FinishEntry;
                } else {
                    let (key, sequence) = rules[self.leaf];
                    let row = row_mut(self.source.as_mut().unwrap(), "entries", self.entry)?;
                    if get(row, key).is_none() {
                        self.leaf += 1;
                        self.item = 0;
                    } else {
                        let value = get_mut(row, key)?;
                        if sequence {
                            let DslValue::Array(values) = value else {
                                return Err(invalid("Template column requires an array").into());
                            };
                            if self.item == values.len() {
                                self.leaf += 1;
                                self.item = 0;
                            } else {
                                self.input = Some(std::mem::replace(&mut values[self.item], DslValue::Null));
                                self.phase = Phase::ReceiveLeaf;
                            }
                        } else {
                            self.input = Some(std::mem::replace(value, DslValue::Null));
                            self.phase = Phase::ReceiveLeaf;
                        }
                    }
                }
            }
            Phase::ReceiveLeaf => {
                if self.input.is_some() {
                    self.receiver.receive_owned(&mut self.input, cx)?;
                } else if self.receiver.advance(cx)? {
                    self.phase = Phase::TakeLeaf;
                }
                return Ok(false);
            }
            Phase::TakeLeaf => {
                if self.receiver.take(&mut self.value, cx)? {
                    self.phase = Phase::DeliverLeaf;
                }
                return Ok(false);
            }
            Phase::DeliverLeaf => {
                let (key, sequence) = leaf_fields(self.pending.as_ref().unwrap().form.kind())[self.leaf];
                let value = self.value.take().unwrap();
                match &mut self.pending.as_mut().unwrap().form {
                    IfcTemplateForm::Single { value: output, .. } | IfcTemplateForm::Quantity { value: output, .. } => *output = value,
                    IfcTemplateForm::Enumerated { values, .. } | IfcTemplateForm::List { values, .. } => values.push(value),
                    IfcTemplateForm::Bounded { lower, upper, set_point, .. } => {
                        *match key {
                            "lower" => lower,
                            "upper" => upper,
                            _ => set_point,
                        } = Some(value)
                    }
                    IfcTemplateForm::Table { defining, defined, .. } => {
                        if key == "defining" {
                            defining.push(value);
                        } else {
                            defined.push(value);
                        }
                    }
                    _ => unreachable!(),
                }
                if sequence {
                    self.item += 1;
                } else {
                    self.leaf += 1;
                }
                self.receiver.begin_close();
                self.phase = Phase::CloseLeaf;
            }
            Phase::CloseLeaf => {
                if self.receiver.terminal_is_empty() {
                    self.receiver = IfcValueReceiver::new(IfcValueReceiveMode::Decimal);
                    self.phase = Phase::Leaf;
                } else {
                    semio_framework_job::close_original_retained_step(cx, demand, |grant| self.receiver.close_step(grant))?;
                    return Ok(false);
                }
            }
            Phase::FinishEntry => {
                self.output.as_mut().unwrap().entries.push(self.pending.take().unwrap());
                self.entry += 1;
                self.phase = if self.entry == self.entry_count { Phase::Finish } else { Phase::Entry };
            }
            Phase::Finish => {
                if self.source.is_none() && self.retirement.is_empty() {
                    self.phase = Phase::Ready;
                    cx.consume_retained(receipt(demand))?;
                    cx.consume_fuel(1);
                } else {
                    semio_framework_job::close_original_retained_step(cx, demand, |grant| self.close_source(grant))?;
                }
                return Ok(matches!(self.phase, Phase::Ready));
            }
            Phase::Ready => return Ok(true),
            _ => return Err(invalid("Template receiving phase is invalid").into()),
        }
        cx.consume_retained(receipt(demand))?;
        cx.consume_fuel(1);
        Ok(false)
    }
    pub fn take(&mut self, output: &mut Option<IfcTemplateApplication>, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        let demand = fixed(2 * size_of::<Option<IfcTemplateApplication>>() + size_of::<Phase>());
        if !fits(cx.retained_grant(), demand) {
            return Ok(false);
        }
        if !matches!(self.phase, Phase::Ready) || output.is_some() {
            return Err(invalid("Template output is unavailable").into());
        }
        *output = self.output.take();
        self.phase = Phase::Spent;
        cx.consume_retained(receipt(demand))?;
        cx.consume_fuel(1);
        Ok(true)
    }
    fn close_source_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        self.retirement.next_owner_demands::<DslValue>(self.source.is_some(), &None, body)
    }
    fn close_source(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.retirement.is_empty() {
            self.retirement.step_granted(grant)
        } else {
            self.retirement.begin_granted(&mut self.source, grant)?.ok_or_else(|| invalid("Template source close omitted its owner"))
        }
    }
    pub fn begin_close(&mut self) {
        self.closing = true;
        self.receiver.begin_close();
    }
    pub fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if !self.receiver.terminal_is_empty() {
            return self.receiver.close_demands(body);
        }
        if !self.retirement.is_empty() {
            return self.retirement.next_owner_demands::<DslValue>(false, &None, body);
        }
        if self.input.is_some() {
            return self.retirement.next_owner_demands::<DslValue>(true, &None, body);
        }
        if self.value.is_some() {
            return self.retirement.next_owner_demands::<IfcValue>(true, &None, body);
        }
        if self.pending.is_some() {
            return self.retirement.next_owner_demands::<IfcTemplateSelection>(true, &None, body);
        }
        if self.output.is_some() {
            return self.retirement.next_owner_demands::<IfcTemplateApplication>(true, &None, body);
        }
        self.close_source_demand(body)
    }
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.receiver.terminal_is_empty() {
            return self.receiver.close_step(grant);
        }
        if !self.retirement.is_empty() {
            return self.retirement.step_granted(grant);
        }
        if self.input.is_some() {
            return self.retirement.begin_granted(&mut self.input, grant)?.ok_or_else(|| invalid("Template input close omitted its owner"));
        }
        if self.value.is_some() {
            return self.retirement.begin_granted(&mut self.value, grant)?.ok_or_else(|| invalid("Template value close omitted its owner"));
        }
        if self.pending.is_some() {
            return self.retirement.begin_granted(&mut self.pending, grant)?.ok_or_else(|| invalid("Template entry close omitted its owner"));
        }
        if self.output.is_some() {
            return self.retirement.begin_granted(&mut self.output, grant)?.ok_or_else(|| invalid("Template output close omitted its owner"));
        }
        self.close_source(grant)
    }
    pub fn terminal_is_empty(&self) -> bool {
        self.source.is_none() && self.output.is_none() && self.pending.is_none() && self.input.is_none() && self.value.is_none() && self.retirement.is_empty() && self.receiver.terminal_is_empty()
    }
}
