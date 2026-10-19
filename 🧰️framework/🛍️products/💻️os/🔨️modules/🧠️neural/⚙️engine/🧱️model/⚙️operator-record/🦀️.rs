//! ⚙️ Operator record: the evaluation errors, the operator and job traits, the cardinality and the channel specification.
//!
//! Holds the `🔖️OperatorRecord`, `🔖️Cardinality` regions of the module; the root re-exports its items.

use super::*;

// #region 🔖️OperatorRecord
/// ⚙️ Eval error from an operator.
#[derive(Clone, Debug, PartialEq,semio_framework_value::RetireOwned)]
pub enum EvalError {
    Retained(ValueError),
    UnknownKind(String),
    MissingInput(String),
    InvalidInput(String),
    CardinalityViolation(String),
    HeterogeneousList(String),
    CycleDetected,
    PendingExtension { extension_id: String, operator_id: String, node_hash: u64 },
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalError::Retained(error)=>std::fmt::Display::fmt(error,f),
            EvalError::UnknownKind(k) => write!(f, "unknown kind: {k}"),
            EvalError::MissingInput(k) => write!(f, "missing input: {k}"),
            EvalError::InvalidInput(m) => write!(f, "invalid input: {m}"),
            EvalError::CardinalityViolation(m) => write!(f, "cardinality violation: {m}"),
            EvalError::HeterogeneousList(m) => write!(f, "heterogeneous list: {m}"),
            EvalError::CycleDetected => write!(f, "cycle detected"),
            EvalError::PendingExtension { extension_id, operator_id, .. } => write!(f, "pending extension {extension_id} operator {operator_id}"),
        }
    }
}

impl std::error::Error for EvalError {}
impl From<ValueError> for EvalError{fn from(error:ValueError)->Self{Self::Retained(error)}}

/// 🪢️ Keeps the complete evaluation refusal or original native turn failure with its paid receipt.
#[derive(Debug,semio_framework_value::RetireOwned)]
pub enum OperatorWorkFailure{
 Evaluation{primary:EvalError,retained:RetainedCloneProgress,receipt_refusal:Option<ValueError>},
 Original(semio_framework_job::OriginalTurnFailure),
}
impl OperatorWorkFailure{
 /// 🧾️ Posts one genuine failed evaluation event without formatting or replacing its original primary.
 pub fn debit_evaluation(primary:EvalError,retained:RetainedCloneProgress,cx:&mut semio_framework_job::StepContext<'_>)->Self{Self::Evaluation{primary,retained,receipt_refusal:cx.consume_retained(retained).err()}}
 /// 📏️ Borrows the same failed producer's actual physical effects.
 pub fn retained_progress(&self)->RetainedCloneProgress{match self{Self::Evaluation{retained,..}=>*retained,Self::Original(failure)=>failure.primary.retained_progress()}}
}
impl From<EvalError> for OperatorWorkFailure{fn from(primary:EvalError)->Self{let retained=match &primary{EvalError::Retained(primary)=>primary.retained_progress(),_=>Default::default()};Self::Evaluation{primary,retained,receipt_refusal:None}}}
impl From<semio_framework_job::OriginalTurnFailure> for OperatorWorkFailure{fn from(primary:semio_framework_job::OriginalTurnFailure)->Self{Self::Original(primary)}}
impl From<ValueError> for OperatorWorkFailure{fn from(primary:ValueError)->Self{Self::Original(primary.into())}}

/// 📥️ Returns the same original input when an implementation declares no resumable plan.
#[derive(semio_framework_value::RetireOwned)]
pub struct PendingOperatorMetadata{pub extension_id:String,pub operator_id:String}
#[derive(semio_framework_value::RetireOwned)]
pub struct OperatorPendingPlan{pub metadata:protocol::value::ordered::SharedOwner<PendingOperatorMetadata>,pub input:Dictionary}
pub enum OperatorPlanAdmission {Job(Box<dyn OperatorJob>),Immediate(Dictionary),Pending(OperatorPendingPlan)}
impl OperatorPlanAdmission {
    /// 🌊️ Leases original contribution metadata and moves its input without payload work.
    pub fn pending(metadata:&protocol::value::ordered::SharedOwner<PendingOperatorMetadata>,input:Dictionary,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),(EvalError,Dictionary)>{if grant.maximum_items==0{return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original pending plan requires an admitted item").into(),input))}if grant.maximum_depth==0{return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original pending plan requires admitted depth").into(),input))}Ok((Self::Pending(OperatorPendingPlan{metadata:metadata.clone(),input}),RetainedCloneProgress{copied_items:1,..Default::default()}))}

    /// 📤️ Hands back the exact original no-plan input with one admitted ownership event and no physical effects.
    pub fn immediate(input:Dictionary,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),(EvalError,Dictionary)>{if grant.maximum_items==0{return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original immediate plan requires one item").into(),input))}if grant.maximum_depth==0{return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original immediate plan requires admitted depth").into(),input))}Ok((Self::Immediate(input),RetainedCloneProgress{copied_items:1,..Default::default()}))}
}

/// 📈️ Monotone progress of one [`OperatorJob`]. `units_done` never decreases; `units_total` is the
/// plan known so far and may be revised upward by a job whose later stages are only plannable once
/// an earlier one has run (a boolean cannot count its result's validation units before it has a
/// result). `phase` is the job's own domain vocabulary — the framework never interprets it, it only
/// carries it to the surface that labels it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct OperatorProgress {
    pub units_done: usize,
    pub units_total: usize,
    pub phase: &'static str,
}

/// ⏱️ Outcome of one budgeted [`OperatorJob::step`].
#[derive(Clone, Debug, PartialEq)]
pub enum OperatorJobStep {
    /// 🔁 Budget spent, work remains — call `step` again.
    Working(OperatorProgress),
    /// ✅ Terminal: the operator's out dictionary.
    Done(Dictionary),
    /// 🛑 Terminal: [`OperatorJob::cancel`] retired the job; nothing is produced.
    Cancelled(OperatorProgress),
}

/// ⏱️ A single operator evaluation split into budgetable units so a host can drive it across many
/// turns inside an interactive step ceiling, paint progress, and cancel it — the operator twin of
/// the kernel's resumable tessellation job.
///
/// Domain-NEUTRAL by construction: the framework knows only units, a phase tag and the three
/// outcomes. Which units an operator has (a face pair, a validation entity, a solver iteration) is
/// entirely the domain extension's business, and an operator that has no sub-structure simply does
/// not offer a job (see [`Operator::step_plan`]'s default) and is evaluated in one call as before
/// (ticket `26/09/09/PROCEDURAL-3D-END-TO-END`).
pub trait OperatorJob: Send {
    /// ⏱️ Advances by at most `budget` units under independently supplied physical credits, preserving the actual receipt through terminal output.
    fn step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<(OperatorJobStep,RetainedCloneProgress),OperatorWorkFailure>;
    /// 🧾️ Preserves the actual last normal receipt through successful and failed child turns.
    fn normal_step_progress(&self)->RetainedCloneProgress;
    fn next_step_copy_byte_demand(&self) -> Result<usize, ValueError>;
    fn next_step_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, ValueError>;
    fn next_step_release_byte_demand(&self) -> Result<usize, ValueError>;
    fn next_step_depth_demand(&self) -> Result<usize, ValueError>;
    /// 📈️ Progress right now — safe to read between steps and after termination.
    fn progress(&self) -> OperatorProgress;
    /// 🛑️ Retires the job at the next observable boundary. A job that already produced its output
    /// is never retired: supersession may only stop work still in flight.
    fn cancel(&mut self);
    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>;
    fn next_close_capacity_byte_demand(&self,maximum_copy_bytes:usize)->Result<usize,ValueError>;
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>;
    fn next_close_depth_demand(&self)->Result<usize,ValueError>;
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
    fn terminal_is_empty(&self)->bool;

}

/// 🧮️ Computational unit: one dictionary to another.
pub trait Operator: Send + Sync {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError>;
    /// 🧊️ The cold-boundary form of [`Operator::evaluate`], mirroring
    /// [`Registry::dispatch_cold`]: takes EXACT ownership of `input` and answers inside a cold
    /// boundary, on the refusal path too. A batch caller (a test harness, an export bridge) that
    /// evaluates an operator directly therefore never becomes the final owner of either
    /// dictionary, and never has to open-code [`Dictionary`]'s drop law
    /// (`final Dictionary ownership must be explicitly retired or owned by a cold boundary`).
    // 🚫️async: E1 pure operator evaluation mirroring `evaluate` (no I/O) — see R9
    fn evaluate_cold(&self, input: Dictionary) -> Result<ColdOwner<Dictionary>, EvalError> {
        let input = ColdOwner::new(input);
        self.evaluate(&input).map(ColdOwner::new)
    }
    /// ⏱️ Admits a concrete plan by moving the exact original input under the caller's independent physical currencies.
    fn step_plan(&self,input:Dictionary,grant:RetainedCloneGrant)->Result<(OperatorPlanAdmission,RetainedCloneProgress),(EvalError,Dictionary)>;
    fn next_plan_copy_byte_demand(&self,input:&Dictionary)->Result<usize,ValueError>;
    fn next_plan_capacity_byte_demand(&self,input:&Dictionary,maximum_copy_bytes:usize)->Result<usize,ValueError>;
    fn next_plan_release_byte_demand(&self,input:&Dictionary)->Result<usize,ValueError>;
    fn next_plan_depth_demand(&self,input:&Dictionary)->Result<usize,ValueError>;
    /// 🪶️ Only compiler-proven trivial operators are terminal without domain-specific field retirement.
    fn retirement_is_empty(&self) -> bool { !std::mem::needs_drop::<Self>() }
    fn next_retire_copy_byte_demand(&self)->Result<usize,ValueError> {if self.retirement_is_empty(){Ok(0)}else{Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"operator copy ownership is not declared"))}}
    fn next_retire_capacity_byte_demand(&self,_maximum_copy_bytes:usize)->Result<usize,ValueError> {if self.retirement_is_empty(){Ok(0)}else{Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"operator capacity ownership is not declared"))}}
    fn next_retire_release_byte_demand(&self)->Result<usize,ValueError> {if self.retirement_is_empty(){Ok(0)}else{Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"operator release ownership is not declared"))}}
    fn next_retire_depth_demand(&self)->Result<usize,ValueError> {if self.retirement_is_empty(){Ok(0)}else{Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"operator depth ownership is not declared"))}}
    /// 🧹️ Retires or transfers actual fields while preserving the original dynamic operator shell.
    fn retire_step(&mut self,_grant:RetainedCloneGrant,_values:&mut ValueRetirement)->Result<RetainedCloneStep,ValueError> {
        if self.retirement_is_empty(){Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()))}else{Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"operator retained ownership is not declared"))}
    }
    /// 🧊️ Explicit cold registry teardown; implementations owning neural domains retire those fields here.
    fn retire_cold(self: Box<Self>) { drop(self); }
}
// #region 🔖️Cardinality
/// 🔢️ Channel multiplicity: exactly one, optional, or homogeneous list collections.
#[derive(Clone, Debug, PartialEq, Default)]
pub enum Cardinality {
    #[default]
    ExactlyOne,
    ZeroOrOne,
    ZeroOrMore,
    OneOrMore,
    Exactly(usize),
}

impl Cardinality {
    pub fn symbol(&self) -> String {
        match self {
            Self::ExactlyOne => "!".into(),
            Self::ZeroOrOne => "?".into(),
            Self::ZeroOrMore => "*".into(),
            Self::OneOrMore => "+".into(),
            Self::Exactly(count) => count.to_string(),
        }
    }

    pub fn from_symbol(raw: &str) -> Result<Self, NeuralEngineError> {
        match raw.trim() {
            "!" => Ok(Self::ExactlyOne),
            "?" => Ok(Self::ZeroOrOne),
            "*" => Ok(Self::ZeroOrMore),
            "+" => Ok(Self::OneOrMore),
            digits if digits.chars().all(|ch| ch.is_ascii_digit()) && !digits.is_empty() => digits.parse::<usize>().map(Self::Exactly).map_err(|_| NeuralEngineError::InvalidCardinality(raw.to_string())),
            other => Err(NeuralEngineError::InvalidCardinality(other.to_string())),
        }
    }

    pub fn is_collection(&self) -> bool {
        match self {
            Self::ZeroOrMore | Self::OneOrMore => true,
            Self::Exactly(count) => *count != 1,
            _ => false,
        }
    }

    pub fn accepts(&self, count: usize) -> bool {
        match self {
            Self::ExactlyOne => count == 1,
            Self::ZeroOrOne => count <= 1,
            Self::ZeroOrMore => true,
            Self::OneOrMore => count >= 1,
            Self::Exactly(expected) => count == *expected,
        }
    }

    pub fn count_range(&self) -> (usize, Option<usize>) {
        match self {
            Self::ExactlyOne => (1, Some(1)),
            Self::ZeroOrOne => (0, Some(1)),
            Self::ZeroOrMore => (0, None),
            Self::OneOrMore => (1, None),
            Self::Exactly(count) => (*count, Some(*count)),
        }
    }

    pub fn range_contains(&self, other: &Self) -> bool {
        let (min, max) = self.count_range();
        let (other_min, other_max) = other.count_range();
        if other_min < min {
            return false;
        }
        match (max, other_max) {
            (Some(limit), Some(other_limit)) => other_limit <= limit,
            (Some(_limit), None) => false,
            (None, Some(_other_limit)) => other_min >= min,
            (None, None) => true,
        }
    }
}

/// 🔮️ Test-only — `ChannelSpec.cardinality` was the only production caller and moved to `ToValue`/
/// `FromValue` below (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/01, tenth-seam pass).
#[cfg(any(test, feature = "testkit"))]
impl Serialize for Cardinality {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.symbol())
    }
}

#[cfg(any(test, feature = "testkit"))]
impl<'de> Deserialize<'de> for Cardinality {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        Self::from_symbol(&raw).map_err(serde::de::Error::custom)
    }
}

impl ToValue for Cardinality {
    fn to_value(&self) -> DslValue {
        DslValue::String(self.symbol())
    }
}

impl FromValue for Cardinality {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::String(raw) = value else { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected a string for Cardinality")) };
        Self::from_symbol(&raw).map_err(|error| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string()))
    }
}
// #endregion 🔖️Cardinality

/// ➕️ Variadic input or output slot specification. `serde` is TEST-ONLY — see `Dictionary`'s docstring.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(any(test, feature = "testkit"), derive(Serialize, Deserialize))]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
pub struct VariadicSpec {
    pub slot_key: String,
    pub min: usize,
    #[cfg_attr(any(test, feature = "testkit"), serde(skip_serializing_if = "Option::is_none"))]
    pub max: Option<usize>,
}

impl ToValue for VariadicSpec {
    fn to_value(&self) -> DslValue {
        DslValue::Object(vec![("slotKey".into(), self.slot_key.to_value()), ("min".into(), self.min.to_value()), ("max".into(), self.max.to_value())])
    }
}

impl FromValue for VariadicSpec {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Object(_) = &value else { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an object for VariadicSpec")) };
        Ok(Self {
            slot_key: value.get("slotKey").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "slotKey"))?,
            min: value.get("min").cloned().map(usize::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "min"))?,
            max: value.get("max").cloned().map(Option::<usize>::from_value).transpose()?.unwrap_or_default(),
        })
    }
}

/// 🔌️ Declared operator channel with required/provided operator capabilities. `serde` is TEST-ONLY
/// — see `Dictionary`'s docstring; `default: Option<Value>` fans in `Value` transitively.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(any(test, feature = "testkit"), derive(Serialize, Deserialize))]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
pub struct ChannelSpec {
    pub code: String,
    pub abbreviation: String,
    pub name: String,
    pub full_name: String,
    #[cfg_attr(any(test, feature = "testkit"), serde(default, skip_serializing_if = "Vec::is_empty"))]
    pub operators: Vec<String>,
    #[cfg_attr(any(test, feature = "testkit"), serde(default, skip_serializing_if = "Vec::is_empty"))]
    pub value_types: Vec<String>,
    #[cfg_attr(any(test, feature = "testkit"), serde(default, skip_serializing_if = "Vec::is_empty"))]
    pub item_types: Vec<String>,
    #[cfg_attr(any(test, feature = "testkit"), serde(skip_serializing_if = "Option::is_none"))]
    pub default: Option<Value>,
    #[cfg_attr(any(test, feature = "testkit"), serde(skip_serializing_if = "Option::is_none"))]
    pub label: Option<String>,
    #[cfg_attr(any(test, feature = "testkit"), serde(default))]
    pub cardinality: Cardinality,
}

impl ToValue for ChannelSpec {
    fn to_value(&self) -> DslValue {
        let mut fields = vec![
            ("code".into(), self.code.to_value()),
            ("abbreviation".into(), self.abbreviation.to_value()),
            ("name".into(), self.name.to_value()),
            ("fullName".into(), self.full_name.to_value()),
            ("operators".into(), self.operators.to_value()),
            ("valueTypes".into(), self.value_types.to_value()),
            ("default".into(), self.default.to_value()),
            ("label".into(), self.label.to_value()),
            ("cardinality".into(), self.cardinality.to_value()),
        ];
        if !self.item_types.is_empty() { fields.push(("itemTypes".into(), self.item_types.to_value())); }
        DslValue::Object(fields)
    }
}

impl FromValue for ChannelSpec {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Object(_) = &value else { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an object for ChannelSpec")) };
        Ok(Self {
            code: value.get("code").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "code"))?,
            abbreviation: value.get("abbreviation").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "abbreviation"))?,
            name: value.get("name").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "name"))?,
            full_name: value.get("fullName").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "fullName"))?,
            operators: value.get("operators").cloned().map(Vec::<String>::from_value).transpose()?.unwrap_or_default(),
            value_types: value.get("valueTypes").cloned().map(Vec::<String>::from_value).transpose()?.unwrap_or_default(),
            item_types: value.get("itemTypes").cloned().map(Vec::<String>::from_value).transpose()?.unwrap_or_default(),
            default: value.get("default").cloned().map(Option::<Value>::from_value).transpose()?.unwrap_or_default(),
            label: value.get("label").cloned().map(Option::<String>::from_value).transpose()?.unwrap_or_default(),
            cardinality: value.get("cardinality").cloned().map(Cardinality::from_value).transpose()?.unwrap_or_default(),
        })
    }
}

pub fn derive_channel_names(name: &str) -> (String, String, String) {
    let code = if name.len() <= 2 { name.to_uppercase() } else { name.chars().take(2).collect::<String>().to_uppercase() };
    let abbreviation = if name.len() <= 3 { name.to_string() } else { name.chars().take(3).collect() };
    let mut full = String::new();
    let mut capitalize = true;
    for ch in name.chars() {
        if ch == '_' {
            capitalize = true;
            continue;
        }
        if capitalize {
            full.extend(ch.to_uppercase());
            capitalize = false;
        } else {
            full.push(ch);
        }
    }
    if full.is_empty() {
        full = name.to_string();
    }
    (code, abbreviation, full)
}

impl ChannelSpec {
    pub fn named(code: impl Into<String>, abbreviation: impl Into<String>, name: impl Into<String>, full_name: impl Into<String>) -> Self {
        Self { code: code.into(), abbreviation: abbreviation.into(), name: name.into(), full_name: full_name.into(), operators: Vec::new(), value_types: Vec::new(), item_types: Vec::new(), default: None, label: None, cardinality: Cardinality::ExactlyOne }
    }

    pub fn requires(name: impl Into<String>, operators: &[impl AsRef<str>]) -> Self {
        let name = name.into();
        let (code, abbreviation, full_name) = derive_channel_names(&name);
        Self { code, abbreviation, name, full_name, operators: operators.iter().map(|entry| entry.as_ref().to_string()).collect(), value_types: Vec::new(), item_types: Vec::new(), default: None, label: None, cardinality: Cardinality::ExactlyOne }
    }

    pub fn provides(name: impl Into<String>, operators: Vec<String>) -> Self {
        let name = name.into();
        let (code, abbreviation, full_name) = derive_channel_names(&name);
        Self { code, abbreviation, name, full_name, operators, value_types: Vec::new(), item_types: Vec::new(), default: None, label: None, cardinality: Cardinality::ExactlyOne }
    }

    pub fn with_operators(mut self, operators: Vec<String>) -> Self {
        self.operators = operators;
        self
    }

    /// 🔤️ Declares the VALUE SCHEMAS this channel carries — the `Dictionary::schema()` a value on
    /// this wire actually has (`"geometry"`, `"vector"`, `"point"`, `"number"`, `"text"`, `"list"`),
    /// not the operator ids `operators` holds. An input lists every schema it accepts, an output
    /// every schema it may produce, and `Registry::channel_compatible` refuses a pair whose declared
    /// sets are disjoint. Empty means undeclared, which stays connectable.
    ///
    /// @see `🧫️fixtures/🔌️port-types/🔣️.json` — the catalogue-wide law
    pub fn with_value_types(mut self, value_types: &[impl AsRef<str>]) -> Self {
        self.value_types = value_types.iter().map(|entry| entry.as_ref().to_string()).collect();
        self
    }

    pub fn with_default(mut self, default: Value) -> Self {
        self.default.replace(default).retire_cold();
        self
    }

    /// 📃️ Declares homogeneous collection element schemas independently of the list's wire schema.
    pub fn with_item_types(mut self, item_types: &[impl AsRef<str>]) -> Self {
        self.item_types = item_types.iter().map(|entry| entry.as_ref().to_string()).collect();
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_cardinality(mut self, cardinality: Cardinality) -> Self {
        self.cardinality = cardinality;
        self
    }

    pub fn number(name: impl Into<String>, operators: &[impl AsRef<str>]) -> Self {
        Self::requires(name, operators).with_value_types(&[VALUE_TYPE_NUMBER])
    }

    pub fn number_default(name: impl Into<String>, default: f64, operators: &[impl AsRef<str>]) -> Self {
        Self::requires(name, operators).with_value_types(&[VALUE_TYPE_NUMBER]).with_default(Value::Dictionary(Dictionary::with_schema("number").insert("value", Value::Atom(Atom::Decimal(default)))))
    }

    pub fn integer_default(name: impl Into<String>, default: i64, operators: &[impl AsRef<str>]) -> Self {
        Self::requires(name, operators).with_value_types(&[VALUE_TYPE_NUMBER]).with_default(Value::Atom(Atom::Integer(default)))
    }

    pub fn boolean_default(name: impl Into<String>, default: bool, operators: &[impl AsRef<str>]) -> Self {
        Self::requires(name, operators).with_value_types(&[VALUE_TYPE_BOOLEAN]).with_default(Value::Dictionary(Dictionary::with_schema("boolean").insert("value", Value::Atom(Atom::Boolean(default)))))
    }

    pub fn text_default(name: impl Into<String>, default: impl Into<String>, operators: &[impl AsRef<str>]) -> Self {
        Self::requires(name, operators).with_value_types(&[VALUE_TYPE_TEXT]).with_default(Value::Dictionary(Dictionary::with_schema("text").insert("value", Value::Atom(Atom::String(default.into())))))
    }

    pub fn list(name: impl Into<String>, operators: &[impl AsRef<str>]) -> Self {
        Self::requires(name, operators).with_value_types(&[VALUE_TYPE_LIST]).with_cardinality(Cardinality::ZeroOrMore)
    }

    pub fn list_output(name: impl Into<String>, operators: Vec<String>) -> Self {
        Self::provides(name, operators).with_value_types(&[VALUE_TYPE_LIST]).with_cardinality(Cardinality::ZeroOrMore)
    }

    pub fn dictionary(name: impl Into<String>, operators: &[impl AsRef<str>]) -> Self {
        Self::requires(name, operators)
    }

    pub fn any(name: impl Into<String>) -> Self {
        Self::requires(name, &[] as &[&str])
    }

    pub fn wildcard() -> Self {
        Self::any("*")
    }
}

/// 📤️ Wraps a payload dictionary under a named output channel.
pub fn channel_output(name: &str, payload: Dictionary) -> Dictionary {
    Dictionary::new().insert(name, Value::Dictionary(payload))
}

/// 📇️ Catalogue metadata for an operator. `serde` is TEST-ONLY — see `Dictionary`'s docstring;
/// `inputs`/`outputs: Vec<ChannelSpec>` fan in `Value` transitively.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(any(test, feature = "testkit"), derive(Serialize, Deserialize))]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
pub struct OperatorInfo {
    pub id: String,
    pub extension: String,
    pub name: String,
    pub abbreviation: String,
    pub icon: String,
    pub summary: String,
    pub inputs: Vec<ChannelSpec>,
    pub outputs: Vec<ChannelSpec>,
    #[cfg_attr(any(test, feature = "testkit"), serde(default, skip_serializing_if = "Option::is_none"))]
    pub variadic_input: Option<VariadicSpec>,
    #[cfg_attr(any(test, feature = "testkit"), serde(default, skip_serializing_if = "Option::is_none"))]
    pub variadic_output: Option<VariadicSpec>,
    #[cfg_attr(any(test, feature = "testkit"), serde(default, skip_serializing_if = "Vec::is_empty"))]
    pub group: Vec<String>,
}

impl ToValue for OperatorInfo {
    fn to_value(&self) -> DslValue {
        DslValue::Object(vec![
            ("id".into(), self.id.to_value()),
            ("extension".into(), self.extension.to_value()),
            ("name".into(), self.name.to_value()),
            ("abbreviation".into(), self.abbreviation.to_value()),
            ("icon".into(), self.icon.to_value()),
            ("summary".into(), self.summary.to_value()),
            ("inputs".into(), self.inputs.to_value()),
            ("outputs".into(), self.outputs.to_value()),
            ("variadicInput".into(), self.variadic_input.to_value()),
            ("variadicOutput".into(), self.variadic_output.to_value()),
            ("group".into(), self.group.to_value()),
        ])
    }
}

impl FromValue for OperatorInfo {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Object(_) = &value else { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an object for OperatorInfo")) };
        Ok(Self {
            id: value.get("id").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "id"))?,
            extension: value.get("extension").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "extension"))?,
            name: value.get("name").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "name"))?,
            abbreviation: value.get("abbreviation").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "abbreviation"))?,
            icon: value.get("icon").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "icon"))?,
            summary: value.get("summary").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "summary"))?,
            inputs: value.get("inputs").cloned().map(Vec::<ChannelSpec>::from_value).transpose()?.unwrap_or_default(),
            outputs: value.get("outputs").cloned().map(Vec::<ChannelSpec>::from_value).transpose()?.unwrap_or_default(),
            variadic_input: value.get("variadicInput").cloned().map(Option::<VariadicSpec>::from_value).transpose()?.unwrap_or_default(),
            variadic_output: value.get("variadicOutput").cloned().map(Option::<VariadicSpec>::from_value).transpose()?.unwrap_or_default(),
            group: value.get("group").cloned().map(Vec::<String>::from_value).transpose()?.unwrap_or_default(),
        })
    }
}
// #endregion 🔖️OperatorRecord
