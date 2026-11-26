//! 🏗️ Executes registered schema projections against immutable input and result roots.

use super::super::{produced_channel_id, schema_errors_list, Atom, Dictionary, EvalError, NeuralEngineError as Error, Schema, Value, ValueType};
use semio_framework_value::{OriginalRetirementFailure, RetainedAllocationEffect, RetainedCloneProgress, RetainedWorkOutcome, RetainedWorkPoll, RetirementDemand, ValueError};
use semio_os_kernel_neural_engine_model::{BudgetedOwnedEntry, OriginalDictionaryLookup};

fn text(value: impl Into<String>) -> Value {
    Value::Atom(Atom::String(value.into()))
}
fn object(value: Dictionary) -> Value {
    Value::Dictionary(value)
}
fn string<'a>(dict: &'a Dictionary, key: &str) -> &'a str {
    dict.get(key).unwrap().as_atom().unwrap().as_str().unwrap()
}
fn child<'a>(dict: &'a Dictionary, key: &str) -> &'a Dictionary {
    dict.get(key).unwrap().as_dictionary().unwrap()
}

pub(in super::super) fn compile(schema: &Schema) -> Dictionary {
    let out = produced_channel_id(&schema.id);
    let mut nulls = Dictionary::new().insert(&out, Value::null());
    for field in &schema.fields {
        nulls = nulls.insert(produced_channel_id(&field.key), Value::null());
    }
    let nulls = super::super::ColdOwner::new(nulls);
    let failure = |error: Error| object((*nulls).clone().insert("errors", object(schema_errors_list(&[error.to_string()]))));
    let mut fields = Dictionary::new();
    for (index, field) in schema.fields.iter().enumerate() {
        let (kind, wrapper, reason) = match &field.value {
            ValueType::Decimal => ("decimal", "number", "numeric"),
            ValueType::Integer => ("integer", "number", "integral"),
            ValueType::Boolean => ("boolean", "boolean", "boolean"),
            ValueType::Text => ("text", "text", "text"),
            ValueType::List(_) => ("list", "", ""),
            ValueType::Schema(_) => ("schema", "", ""),
            ValueType::Any => ("any", "", ""),
        };
        let id = field.value.id();
        let missing = EvalError::MissingInput(field.key.clone());
        let wrong = EvalError::InvalidInput(format!("field {} does not match {id}", field.key));
        let descriptor = Dictionary::new()
            .insert("key", text(&field.key))
            .insert("out", text(produced_channel_id(&field.key)))
            .insert("kind", text(kind))
            .insert("id", text(id))
            .insert("absent", failure(Error::MissingField(field.key.clone())))
            .insert("missing", failure(missing.into()))
            .insert("wrong", failure(wrong.into()))
            .insert("channel", failure(if wrapper.is_empty() { Error::ChannelNotDictionary } else { Error::ChannelTypeMismatch { kind } }))
            .insert("payload", failure(Error::ChannelMissingValue { kind }))
            .insert("projection", failure(if wrapper.is_empty() { Error::FieldNotDictionary } else { Error::FieldTypeMismatch { kind, reason } }))
            .insert("wrapper", object(if wrapper.is_empty() { Dictionary::new() } else { Dictionary::with_schema(wrapper) }));
        fields = fields.insert(format!("{index:016x}"), object(descriptor));
    }
    Dictionary::new()
        .insert("key", text(&schema.id))
        .insert("out", text(out))
        .insert("fields", object(fields))
        .insert("defaults", object(schema.default_dictionary()))
        .insert("output", object(Dictionary::new().insert("errors", object(schema_errors_list(&[])))))
        .insert("empty", failure(Error::NoInputProvided))
        .insert("missing", failure(Error::Missing(schema.id.clone())))
        .insert("invalid", failure(Error::Invalid(schema.id.clone())))
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Stage {
    Instance,
    Apply,
    Presence,
    Validate,
    Project,
    Wrap,
    PublishField,
    PublishInstance,
    Done,
}
semio_framework_value::artifact_retire_leaf!(Stage);

#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
pub(super) struct Projection {
    program: Dictionary,
    input: Dictionary,
    instance: Dictionary,
    output: Dictionary,
    wrapped: Dictionary,
    field: usize,
    tag_byte: usize,
    stage: Stage,
    supplied: bool,
    modifying: bool,
    entry: Option<BudgetedOwnedEntry>,
    closing_entry: bool,
    lookup: OriginalDictionaryLookup,
    progress: RetainedCloneProgress,
}

impl Projection {
    pub(super) fn new(program: Dictionary, input: Dictionary) -> Self {
        Self {
            program,
            input,
            instance: Dictionary::new(),
            output: Dictionary::new(),
            wrapped: Dictionary::new(),
            field: 0,
            tag_byte: 0,
            stage: Stage::Instance,
            supplied: false,
            modifying: false,
            entry: None,
            closing_entry: false,
            lookup: Default::default(),
            progress: Default::default(),
        }
    }

    fn field(&self) -> Option<&Dictionary> {
        child(&self.program, "fields").entry_at_rank(self.field).and_then(|(_, v)| v.as_dictionary())
    }

    fn key(&self) -> &str {
        match self.stage {
            Stage::Instance => string(&self.program, "key"),
            Stage::PublishInstance => string(&self.program, "out"),
            Stage::PublishField => string(self.field().unwrap(), "out"),
            Stage::Wrap => "value",
            _ => self.field().map_or("", |f| string(f, "key")),
        }
    }

    pub(super) fn next_demands(&self, copy: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(entry) = &self.entry {
            return if self.closing_entry { entry.next_close_demands(copy) } else { entry.next_demands(copy) };
        }
        let scanning = matches!(self.stage, Stage::Instance | Stage::Apply | Stage::Presence | Stage::Validate | Stage::Project) && !self.lookup.complete();
        if scanning {
            return Ok(RetirementDemand { copy_bytes: usize::from(self.lookup.comparison_pending()), capacity_bytes: 0, release_bytes: 0, depth: 1 });
        }
        let key = match self.stage {
            Stage::Apply | Stage::PublishInstance | Stage::PublishField => self.key().len(),
            Stage::Project => 5,
            _ => 0,
        };
        Ok(RetirementDemand { copy_bytes: key.max(1), capacity_bytes: key, release_bytes: 0, depth: 1 })
    }

    pub(super) fn phase(&self) -> &'static str {
        match self.stage {
            Stage::Instance => "schema-instance",
            Stage::Apply => "schema-apply",
            Stage::Presence | Stage::Validate => "schema-validate",
            Stage::Project | Stage::Wrap | Stage::PublishField | Stage::PublishInstance => "schema-output",
            Stage::Done => "schema-done",
        }
    }

    pub(super) fn step_progress(&self) -> RetainedCloneProgress {
        self.progress
    }

    fn next_field(&mut self, stage: Stage) {
        self.field += 1;
        self.tag_byte = 0;
        self.stage = stage;
        self.lookup.reset();
    }

    fn accept(&mut self, dict: Dictionary) {
        match self.stage {
            Stage::Apply => {
                self.instance = dict;
                self.next_field(Stage::Apply);
            }
            Stage::Wrap => {
                self.wrapped = dict;
                self.stage = Stage::PublishField;
            }
            Stage::PublishField => {
                self.output = dict;
                self.next_field(Stage::Project);
            }
            Stage::PublishInstance => {
                self.output = dict;
                self.stage = Stage::Done;
            }
            _ => unreachable!(),
        }
    }

    pub(super) fn advance(&mut self, copy: usize, effect: &mut RetainedAllocationEffect<'_>) -> RetainedWorkPoll<Option<Dictionary>, OriginalRetirementFailure> {
        self.progress = Default::default();
        if let Some(entry) = self.entry.as_mut() {
            if self.closing_entry {
                let turn = entry.close_step(copy, effect);
                self.progress = entry.step_progress();
                if entry.terminal_is_empty() {
                    self.entry = None;
                    self.closing_entry = false;
                }
                return match turn {
                    RetainedWorkPoll::Complete(turn) => RetainedWorkPoll::Complete(RetainedWorkOutcome { result: turn.result.map(|_| None), work: turn.work }),
                    RetainedWorkPoll::Pending { work } => RetainedWorkPoll::Pending { work },
                };
            }
            let turn = entry.step(copy, effect);
            self.progress = entry.step_progress();
            return match turn {
                RetainedWorkPoll::Complete(mut result) => {
                    if let Ok(Some(dict)) = &mut result.result {
                        let dict = std::mem::take(dict);
                        self.closing_entry = true;
                        self.accept(dict);
                        result.result = Ok(None);
                    }
                    RetainedWorkPoll::Complete(result)
                }
                pending => pending,
            };
        }
        let before = effect.performed_work();
        let result = (|| {
            let grant = effect.grant();
            let demand = self.next_demands(copy)?;
            if grant.maximum_items == 0 || grant.maximum_depth < demand.depth || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || !effect.performed(1)? {
                return Ok(None);
            }
            self.progress.copied_items = 1;
            self.advance_field(grant)
        })();
        RetainedWorkPoll::Complete(RetainedWorkOutcome { result, work: effect.performed_work() - before })
    }

    fn advance_field(&mut self, grant: super::RetainedCloneGrant) -> Result<Option<Dictionary>, OriginalRetirementFailure> {
        let fields = child(&self.program, "fields");
        if matches!(self.stage, Stage::Apply | Stage::Presence | Stage::Validate | Stage::Project) && self.field == fields.len() {
            self.stage = match self.stage {
                Stage::Apply if !self.supplied && !self.modifying => return Ok(Some(child(&self.program, "empty").clone())),
                Stage::Apply if !self.modifying => Stage::Presence,
                Stage::Presence | Stage::Apply => Stage::Validate,
                Stage::Validate => Stage::Project,
                _ => Stage::PublishInstance,
            };
            self.field = 0;
            return Ok(None);
        }
        if self.stage == Stage::Done {
            return Ok(Some(std::mem::take(&mut self.output)));
        }
        if self.stage == Stage::PublishInstance {
            let key = string(&self.program, "out").to_owned();
            self.charge_key(&key);
            self.entry = Some(BudgetedOwnedEntry::new(std::mem::take(&mut self.output), key, object(std::mem::take(&mut self.instance))));
            return Ok(None);
        }
        if self.stage == Stage::PublishField {
            let key = string(self.field().unwrap(), "out").to_owned();
            self.charge_key(&key);
            self.entry = Some(BudgetedOwnedEntry::new(std::mem::take(&mut self.output), key, object(std::mem::take(&mut self.wrapped))));
            return Ok(None);
        }
        if self.stage == Stage::Instance {
            let key = string(&self.program, "key");
            if !self.lookup.complete() {
                let (_, receipt) = self.lookup.step(&self.input, key, grant)?;
                self.progress = receipt;
                return Ok(None);
            }
            let value = self.lookup.result(&self.input)?.filter(|v| !v.is_null());
            self.instance = match value {
                None => child(&self.program, "defaults").clone(),
                Some(value) => match value.as_dictionary() {
                    None => return Ok(Some(child(&self.program, "missing").clone())),
                    Some(dict) => match compare_tag(key, dict.schema(), &mut self.tag_byte, &mut self.progress) {
                        None => return Ok(None),
                        Some(false) => return Ok(Some(child(&self.program, "invalid").clone())),
                        Some(true) => {
                            self.modifying = true;
                            dict.clone()
                        }
                    },
                },
            };
            self.output = child(&self.program, "output").clone();
            self.lookup.reset();
            self.stage = Stage::Apply;
            self.tag_byte = 0;
            return Ok(None);
        }
        let field = fields.entry_at_rank(self.field).unwrap().1.as_dictionary().unwrap();
        let key = string(field, "key");
        let kind = string(field, "kind");
        let source = if self.stage == Stage::Apply { &self.input } else { &self.instance };
        if !self.lookup.complete() {
            let (_, receipt) = self.lookup.step(source, key, grant)?;
            self.progress = receipt;
            return Ok(None);
        }
        let value = self.lookup.result(source)?;
        let rank = self.lookup.matched_rank();
        if self.stage == Stage::Apply {
            let Some(value) = value.filter(|v| !v.is_null()) else {
                self.next_field(Stage::Apply);
                return Ok(None);
            };
            self.supplied = true;
            let Some(dict) = value.as_dictionary() else { return Ok(Some(child(field, "channel").clone())) };
            let key_bytes = key.len();
            let input = match kind {
                "decimal" | "integer" | "text" | "boolean" => {
                    let Some(payload) = dict.get("value") else { return Ok(Some(child(field, "payload").clone())) };
                    if kind == "text" {
                        if payload.as_atom().and_then(Atom::as_str).is_none() {
                            return Ok(Some(child(field, "payload").clone()));
                        }
                        let value = dict.value_shared_at_rank(short_key_rank(dict, "value").unwrap()).unwrap();
                        BudgetedOwnedEntry::from_shared_value(std::mem::take(&mut self.instance), key.to_owned(), value)
                    } else {
                        let Some(atom) = scalar(kind, payload) else { return Ok(Some(child(field, "payload").clone())) };
                        BudgetedOwnedEntry::new(std::mem::take(&mut self.instance), key.to_owned(), Value::Atom(atom))
                    }
                }
                _ => BudgetedOwnedEntry::from_shared_value(std::mem::take(&mut self.instance), key.to_owned(), self.input.value_shared_at_rank(rank.unwrap()).unwrap()),
            };
            self.progress.copied_bytes += key_bytes;
            self.progress.retained_capacity_bytes += key_bytes;
            self.entry = Some(input);
            return Ok(None);
        }
        let Some(value) = value else { return Ok(Some(child(field, if self.modifying { "missing" } else { "absent" }).clone())) };
        if self.stage == Stage::Presence {
            self.next_field(Stage::Presence);
            return Ok(None);
        }
        if self.stage == Stage::Validate {
            use semio_framework_value::ValueKind as K;
            let valid = match (kind, value.kind()) {
                (_, K::Null) => false,
                ("any", _) => true,
                ("integer", K::Integer) | ("boolean", K::Boolean) | ("text", K::Text) => true,
                ("decimal", K::Integer | K::Boolean | K::Decimal) => true,
                ("list", K::Dictionary(Some("list"))) => true,
                ("schema", K::Dictionary(id)) => match compare_tag(string(field, "id"), id, &mut self.tag_byte, &mut self.progress) {
                    None => return Ok(None),
                    Some(valid) => valid,
                },
                _ => false,
            };
            if !valid {
                return Ok(Some(child(field, "wrong").clone()));
            }
            self.next_field(Stage::Validate);
            return Ok(None);
        }
        self.stage = Stage::Wrap;
        let wrapper = child(field, "wrapper");
        if wrapper.is_empty() {
            let Some(dict) = value.as_dictionary() else { return Ok(Some(child(field, "projection").clone())) };
            self.wrapped = dict.clone();
            self.stage = Stage::PublishField;
        } else {
            let entry = if kind == "text" {
                BudgetedOwnedEntry::from_shared_value(wrapper.clone(), "value".into(), self.instance.value_shared_at_rank(rank.unwrap()).unwrap())
            } else {
                let Some(atom) = scalar(kind, value) else { return Ok(Some(child(field, "projection").clone())) };
                BudgetedOwnedEntry::new(wrapper.clone(), "value".into(), Value::Atom(atom))
            };
            self.progress.copied_bytes += 5;
            self.progress.retained_capacity_bytes += 5;
            self.entry = Some(entry);
        }
        self.lookup.reset();
        Ok(None)
    }

    fn charge_key(&mut self, key: &str) {
        self.progress.copied_bytes += key.len();
        self.progress.retained_capacity_bytes += key.len();
    }
}

fn short_key_rank(dict: &Dictionary, key: &str) -> Option<usize> {
    let mut low = 0;
    let mut high = dict.len();
    while low < high {
        let mid = low + (high - low) / 2;
        match dict.entry_at_rank(mid)?.0.as_str().cmp(key) {
            std::cmp::Ordering::Less => low = mid + 1,
            std::cmp::Ordering::Greater => high = mid,
            std::cmp::Ordering::Equal => return Some(mid),
        }
    }
    None
}

fn scalar(kind: &str, value: &Value) -> Option<Atom> {
    let atom = value.as_atom()?;
    match kind {
        "decimal" => Some(Atom::Decimal(atom.as_f64()?)),
        "boolean" => Some(Atom::Boolean(atom.as_bool()?)),
        "integer" => match atom {
            Atom::Integer(i) => Some(Atom::Integer(*i)),
            Atom::Decimal(d) => Some(Atom::Integer(d.round() as i64)),
            _ => None,
        },
        _ => None,
    }
}

fn compare_tag(expected: &str, actual: Option<&str>, at: &mut usize, receipt: &mut RetainedCloneProgress) -> Option<bool> {
    let Some(actual) = actual else { return Some(false) };
    if expected.len() != actual.len() {
        return Some(false);
    }
    if *at == actual.len() {
        return Some(true);
    }
    receipt.copied_bytes += 1;
    if actual.as_bytes()[*at] != expected.as_bytes()[*at] {
        return Some(false);
    }
    *at += 1;
    None
}
