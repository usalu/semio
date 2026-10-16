//! 🧮️ Tool run progress: verdicts, reasoned steps, the bounded step ring, counters and the progress snapshot.

use super::*;

//#region 🔖️Progress
/// 🚥️ Verdict of one traced attempt; colours are framework semantic tokens.
#[derive(semio_framework_value::RetireOwned)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum ToolRunVerdict {
    Testing,
    Success,
    Warning,
    Danger,
}

impl ToolRunVerdict {
    pub const ALL: [Self; 4] = [Self::Testing, Self::Success, Self::Warning, Self::Danger];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Testing => "testing",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Danger => "danger",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|verdict| verdict.as_str() == text)
    }

    pub fn ordinal(self) -> u8 {
        self as u8
    }

    pub fn from_ordinal(ordinal: u8) -> Option<Self> {
        Self::ALL.get(usize::from(ordinal)).copied()
    }

    /// 🗑️ `warning` and `danger` records are the only ones residency may evict.
    pub fn is_rejected(self) -> bool {
        matches!(self, Self::Warning | Self::Danger)
    }
}

/// 🪧️ Severity of one step-log entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum ToolRunStepKind {
    Info,
    Success,
    Warning,
    Danger,
}

impl ToolRunStepKind {
    pub const ALL: [Self; 4] = [Self::Info, Self::Success, Self::Warning, Self::Danger];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Danger => "danger",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|kind| kind.as_str() == text)
    }

    pub fn ordinal(self) -> u8 {
        self as u8
    }

    pub fn from_ordinal(ordinal: u64) -> Option<Self> {
        Self::ALL.get(usize::try_from(ordinal).ok()?).copied()
    }
}

/// 🪝️ One positional template argument.
#[derive(Clone, Copy, Debug, semio_framework_value::RetireOwned)]
pub enum ToolRunStepArg {
    Unsigned(u64),
    Float(f64),
}

impl PartialEq for ToolRunStepArg {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Unsigned(left), Self::Unsigned(right)) => left == right,
            (Self::Float(left), Self::Float(right)) => left.to_bits() == right.to_bits(),
            _ => false,
        }
    }
}

impl ToolRunStepArg {
    /// 🔤️ Locale-neutral text used for template substitution.
    pub fn to_plain_string(self) -> String {
        match self {
            Self::Unsigned(value) => value.to_string(),
            Self::Float(value) => value.to_string(),
        }
    }
}

/// 📝️ One step-log entry; plugins send reason codes and arguments, never prose.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct ToolRunStep {
    pub sequence: u64,
    pub kind: ToolRunStepKind,
    pub stage: u16,
    pub reason: u16,
    pub subject: Option<u64>,
    pub repeat: u32,
    pub args: Vec<ToolRunStepArg>,
}

impl ToolRunStep {
    /// 🧩️ A single (repeat 1) step without subject or arguments.
    pub fn new(sequence: u64, kind: ToolRunStepKind, stage: u16, reason: u16) -> Self {
        Self { sequence, kind, stage, reason, subject: None, repeat: 1, args: Vec::new() }
    }

    /// 🧲️ Identical apart from `sequence` and `repeat`, so consecutive pushes coalesce.
    pub fn coalesces_with(&self, other: &Self) -> bool {
        self.kind == other.kind && self.stage == other.stage && self.reason == other.reason && self.subject == other.subject && self.args == other.args
    }
}

/// 🎡️ Newest ≤ 64 steps, overwrite-oldest, identical consecutive steps coalesced.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::RetireOwned)]
pub struct ToolRunStepRing {
    #[cfg(feature = "testkit")] #[doc(hidden)] pub steps: VecDeque<ToolRunStep>,
    #[cfg(not(feature = "testkit"))] pub(crate) steps: VecDeque<ToolRunStep>,
}

impl ToolRunStepRing {
    pub fn new() -> Self {
        Self { steps: VecDeque::new() }
    }

    /// 📥️ Coalesces into the newest entry or appends, evicting the oldest when full.
    pub fn push(&mut self, step: ToolRunStep) {
        if let Some(newest) = self.steps.back_mut().filter(|newest| newest.coalesces_with(&step)) {
            newest.repeat = newest.repeat.saturating_add(step.repeat);
            newest.sequence = step.sequence;
            return;
        }
        if self.steps.len() == TOOL_RUN_STEP_RING_CAPACITY {
            self.steps.pop_front();
        }
        self.steps.push_back(step);
    }

    /// 🧾️ Rebuilds a ring from wire order without coalescing.
    pub fn from_steps(steps: Vec<ToolRunStep>) -> Result<Self, ToolRunCodecError> {
        if steps.len() > TOOL_RUN_STEP_RING_CAPACITY {
            return Err(ToolRunCodecError::Limit("step ring capacity"));
        }
        Ok(Self { steps: steps.into() })
    }

    pub fn len(&self) -> usize {
        self.steps.len()
    }

    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &ToolRunStep> {
        self.steps.iter()
    }

    pub fn oldest(&self) -> Option<&ToolRunStep> {
        self.steps.front()
    }

    pub fn newest(&self) -> Option<&ToolRunStep> {
        self.steps.back()
    }

    pub fn clear(&mut self) {
        self.steps.clear();
    }
}

/// 📟️ One counter value by index into `ToolRunDefinition.counters`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, semio_framework_value::RetireOwned)]
pub struct ToolRunCounter {
    pub counter: u16,
    pub value: u64,
}

/// 📊️ Progress snapshot; status text and percentage are derived by the renderer.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct ToolRunProgress {
    pub identity: ToolRunIdentity,
    pub sequence: u64,
    pub state: ToolRunState,
    pub stage: u16,
    pub completed: u64,
    pub total: Option<u64>,
    pub counters: Vec<ToolRunCounter>,
    pub units_per_second: f32,
    pub conflicts: u32,
    pub steps: ToolRunStepRing,
}

impl ToolRunProgress {
    /// ➗️ `completed / total`; `None` while indeterminate.
    pub fn fraction(&self) -> Option<f64> {
        self.total.filter(|total| *total > 0).map(|total| self.completed as f64 / total as f64)
    }
}
//#endregion 🔖️Progress
