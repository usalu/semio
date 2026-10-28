//! 📐️ Tool run definition: the static `ToolRunDefinition` vocabulary and the ToolRun panel group keys.

use super::*;

//#region 🔖️Definition
/// 🧵️ Id of a job kind registered in the artifact tool factory registry.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(transparent)]
#[value(transparent)]
pub struct JobKindId(pub String);

impl JobKindId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// 🔄️ How a mutating run reacts to head changes (§2.7.5).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum ToolRunRebasePolicy {
    Revalidate,
    Restart,
    Freeze,
}

/// 🎚️ How a run reacts to settings changes (§3.3).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum ToolRunReconfigurePolicy {
    Resume,
    Restart,
}

/// 🫥️ Trace subject kind a run emits.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum ToolRunTraceKind {
    Instance3d,
    Placement2d,
    Entity,
    None,
}

/// 🪜️ One algorithm stage.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ToolRunStageDefinition {
    pub id: String,
    pub label: LocalizedLabel,
}

/// 🔟️ One progress counter.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ToolRunCounterDefinition {
    pub id: String,
    pub label: LocalizedLabel,
}

/// 🗯️ One reason code with its verdict and EN/DE template (`{0}`..`{3}` take step arguments).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ToolRunReasonDefinition {
    pub code: u16,
    pub id: String,
    pub verdict: ToolRunVerdict,
    pub template: LocalizedLabel,
}

/// 🎚️ The settings a run's jobs read (§3.3): RFC 6901 JSON Pointers into the app config document and, per
/// window kind id, into that kind's window config documents. `settingsChanged` fires only when a value
/// behind one of them changes; an empty declaration reads no settings, so no settings publication ever
/// reconfigures the run.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ToolRunSettingsReads {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub config: Vec<String>,
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    #[value(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub window_config: std::collections::BTreeMap<String, Vec<String>>,
}

impl ToolRunSettingsReads {
    pub fn is_empty(&self) -> bool {
        self.config.is_empty() && self.window_config.values().all(Vec::is_empty)
    }

    /// 🔎️ Every declared pointer, config first, then window config in window kind order.
    pub fn pointers(&self) -> impl Iterator<Item = &str> {
        self.config.iter().chain(self.window_config.values().flatten()).map(String::as_str)
    }
}

/// 🧭️ The reference tokens of an RFC 6901 JSON Pointer (`~1` → `/`, `~0` → `~`); `None` for a malformed one.
/// The empty pointer names the whole document.
pub fn tool_run_pointer_tokens(pointer: &str) -> Option<Vec<String>> {
    if pointer.is_empty() {
        return Some(Vec::new());
    }
    pointer.strip_prefix('/')?.split('/').map(|token| (!token.replace("~0", "").replace("~1", "").contains('~')).then(|| token.replace("~1", "/").replace("~0", "~"))).collect()
}

/// 📍️ The value an RFC 6901 JSON Pointer names inside `document`; `None` when it is malformed or names nothing.
pub fn tool_run_pointer_value<'a>(document: &'a semio_framework_value::DslValue, pointer: &str) -> Option<&'a semio_framework_value::DslValue> {
    tool_run_pointer_tokens(pointer)?.iter().try_fold(document, |value, token| match value {
        semio_framework_value::DslValue::Object(fields) => fields.iter().find(|(key, _)| key == token).map(|(_, value)| value),
        semio_framework_value::DslValue::Array(items) => (token.bytes().all(|byte| byte.is_ascii_digit()) && (token == "0" || !token.starts_with('0'))).then(|| token.parse::<usize>().ok()).flatten().and_then(|index| items.get(index)),
        _ => None,
    })
}

/// 📜️ Static run declaration attached to tool and utility definitions (§2.4).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ToolRunDefinition {
    pub receiving: ToolRunReceivingLimits,
    pub mutating: bool,
    pub rebase: ToolRunRebasePolicy,
    pub reconfigure: ToolRunReconfigurePolicy,
    pub unit: LocalizedLabel,
    pub stages: Vec<ToolRunStageDefinition>,
    pub counters: Vec<ToolRunCounterDefinition>,
    pub reasons: Vec<ToolRunReasonDefinition>,
    pub trace: ToolRunTraceKind,
    pub run_job: JobKindId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub revalidate_job: Option<JobKindId>,
    #[serde(default, skip_serializing_if = "ToolRunSettingsReads::is_empty")]
    #[value(default, skip_serializing_if = "ToolRunSettingsReads::is_empty")]
    pub settings: ToolRunSettingsReads,
    /// 🪟️ Window kind ids whose bodies render this run's state (`ArtifactView::tool_run()`): every tick refreshes them
    /// next to the ToolRun panel, and no other window.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub windows: Vec<String>,
    /// 🧩️ The composed-child slot whose owned member store a mutating run edits (design §12, §20.15): its provisional ops are
    /// that member's ops, the renderers read them composed over the live children, and finalize publishes one member edit.
    /// Absent: the document's own store.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub member: Option<String>,
}

/// 🚫️ Why a `ToolRunDefinition` violates the contract.
#[derive(Clone, Debug, PartialEq, Eq, Hash, semio_framework_value::RetireOwned)]
pub enum ToolRunDefinitionError {
    InvalidReceivingLimits,
    NoStages,
    TooManyStages,
    TooManyCounters,
    DuplicateStageId(String),
    DuplicateCounterId(String),
    DuplicateReasonId(String),
    DuplicateReasonCode(u16),
    ReservedReasonCode(u16),
    InvalidSettingsPointer(String),
    EmptyMember,
    ReadOnlyMember,
}

impl std::fmt::Display for ToolRunDefinitionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidReceivingLimits => formatter.write_str("toolRun.definition.invalidReceivingLimits"),
            Self::NoStages => formatter.write_str("toolRun.definition.noStages"),
            Self::TooManyStages => formatter.write_str("toolRun.definition.tooManyStages"),
            Self::TooManyCounters => formatter.write_str("toolRun.definition.tooManyCounters"),
            Self::DuplicateStageId(id) => write!(formatter, "toolRun.definition.duplicateStageId: {id}"),
            Self::DuplicateCounterId(id) => write!(formatter, "toolRun.definition.duplicateCounterId: {id}"),
            Self::DuplicateReasonId(id) => write!(formatter, "toolRun.definition.duplicateReasonId: {id}"),
            Self::DuplicateReasonCode(code) => write!(formatter, "toolRun.definition.duplicateReasonCode: {code}"),
            Self::ReservedReasonCode(code) => write!(formatter, "toolRun.definition.reservedReasonCode: {code}"),
            Self::InvalidSettingsPointer(pointer) => write!(formatter, "toolRun.definition.invalidSettingsPointer: {pointer}"),
            Self::EmptyMember => formatter.write_str("toolRun.definition.emptyMember"),
            Self::ReadOnlyMember => formatter.write_str("toolRun.definition.readOnlyMember"),
        }
    }
}

impl std::error::Error for ToolRunDefinitionError {}

impl ToolRunDefinition {
    /// ✅️ Checks stage, counter and reason tables against the contract limits, and that a member target names a slot of a
    /// mutating run.
    pub fn validate(&self) -> Result<(), ToolRunDefinitionError> {
        self.receiving.validate().map_err(|_|ToolRunDefinitionError::InvalidReceivingLimits)?;
        if self.stages.is_empty() {
            return Err(ToolRunDefinitionError::NoStages);
        }
        if self.stages.len() > usize::from(u16::MAX) + 1 {
            return Err(ToolRunDefinitionError::TooManyStages);
        }
        if self.counters.len() > TOOL_RUN_COUNTERS_MAX {
            return Err(ToolRunDefinitionError::TooManyCounters);
        }
        let mut stage_ids = std::collections::HashSet::new();
        if let Some(stage) = self.stages.iter().find(|stage| !stage_ids.insert(stage.id.as_str())) {
            return Err(ToolRunDefinitionError::DuplicateStageId(stage.id.clone()));
        }
        let mut counter_ids = std::collections::HashSet::new();
        if let Some(counter) = self.counters.iter().find(|counter| !counter_ids.insert(counter.id.as_str())) {
            return Err(ToolRunDefinitionError::DuplicateCounterId(counter.id.clone()));
        }
        let mut reason_ids = std::collections::HashSet::new();
        let mut reason_codes = std::collections::HashSet::new();
        for reason in &self.reasons {
            if reason.code >= TOOL_RUN_RESERVED_REASON_FLOOR {
                return Err(ToolRunDefinitionError::ReservedReasonCode(reason.code));
            }
            if !reason_codes.insert(reason.code) {
                return Err(ToolRunDefinitionError::DuplicateReasonCode(reason.code));
            }
            if !reason_ids.insert(reason.id.as_str()) {
                return Err(ToolRunDefinitionError::DuplicateReasonId(reason.id.clone()));
            }
        }
        if let Some(pointer) = self.settings.pointers().find(|pointer| tool_run_pointer_tokens(pointer).is_none()) {
            return Err(ToolRunDefinitionError::InvalidSettingsPointer(pointer.to_string()));
        }
        match self.member.as_deref() {
            Some("") => Err(ToolRunDefinitionError::EmptyMember),
            Some(_) if !self.mutating => Err(ToolRunDefinitionError::ReadOnlyMember),
            _ => Ok(()),
        }
    }

    pub fn stage(&self, index: u16) -> Option<&ToolRunStageDefinition> {
        self.stages.get(usize::from(index))
    }

    pub fn counter(&self, index: u16) -> Option<&ToolRunCounterDefinition> {
        self.counters.get(usize::from(index))
    }

    pub fn reason(&self, code: u16) -> Option<&ToolRunReasonDefinition> {
        self.reasons.iter().find(|reason| reason.code == code)
    }
}
//#endregion 🔖️Definition

//#region 🔖️Panel
/// 🪧️ Key of the framework ToolRun panel root; each run is one child group [`tool_run_panel_group_id`].
pub const TOOL_RUN_PANEL_ID: &str = "framework.toolRun";

/// 🔑️ The key of run `run`'s group in the ToolRun panel; every id inside the group is scoped under it.
pub fn tool_run_panel_group_id(run: u64) -> String {
    format!("{TOOL_RUN_PANEL_ID}.{run}")
}

/// 🏃️ The run a ToolRun panel group key names; `None` for any other key (a key inside a group included).
pub fn tool_run_panel_group_run(key: &str) -> Option<u64> {
    let digits = key.strip_prefix(TOOL_RUN_PANEL_ID)?.strip_prefix('.')?;
    (!digits.is_empty() && digits.bytes().all(|byte| byte.is_ascii_digit()) && (digits == "0" || !digits.starts_with('0'))).then(|| digits.parse().ok()).flatten()
}

/// 📣️ The runs a rendered ToolRun panel holds and those of them not in `previous` — the runs whose start makes a
/// host reveal the panel.
pub fn tool_run_panel_new_runs<'a>(previous: &std::collections::BTreeSet<u64>, keys: impl IntoIterator<Item = &'a str>) -> (std::collections::BTreeSet<u64>, Vec<u64>) {
    let current: std::collections::BTreeSet<u64> = keys.into_iter().filter_map(tool_run_panel_group_run).collect();
    let new = current.difference(previous).copied().collect();
    (current, new)
}
//#endregion 🔖️Panel
