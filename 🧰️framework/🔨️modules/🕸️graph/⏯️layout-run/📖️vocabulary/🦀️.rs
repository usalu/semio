//! 📖️ Layout run vocabulary: stages, counters, reasons, stop causes and the static `ToolRunDefinition` of the run.

use super::*;

//#region 🔖️Vocabulary
/// 🪜️ Run stages, indices into the `ToolRunDefinition` stages.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LayoutRunStage {
    Initialize,
    Iterate,
    Settle,
}

impl LayoutRunStage {
    pub const ALL: [Self; 3] = [Self::Initialize, Self::Iterate, Self::Settle];

    pub fn index(self) -> u16 {
        self as u16
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::Initialize => "initialize",
            Self::Iterate => "iterate",
            Self::Settle => "settle",
        }
    }

    pub fn label(self) -> LocalizedLabel {
        match self {
            Self::Initialize => LocalizedLabel::native("Initialize", "Initialisieren"),
            Self::Iterate => LocalizedLabel::native("Iterate", "Iterieren"),
            Self::Settle => LocalizedLabel::native("Settle", "Stabilisieren"),
        }
    }
}

/// 🧮️ Run counters; `Energy` and `MaxDisplacement` are fixed point in thousandths.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LayoutRunCounter {
    Iterations,
    Energy,
    MaxDisplacement,
    Settled,
}

impl LayoutRunCounter {
    pub const ALL: [Self; 4] = [Self::Iterations, Self::Energy, Self::MaxDisplacement, Self::Settled];

    pub fn index(self) -> u16 {
        self as u16
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::Iterations => "iterations",
            Self::Energy => "energy",
            Self::MaxDisplacement => "maxDisplacement",
            Self::Settled => "settled",
        }
    }

    pub fn fixed_point(self) -> u64 {
        match self {
            Self::Energy | Self::MaxDisplacement => 1000,
            Self::Iterations | Self::Settled => 1,
        }
    }

    pub fn label(self) -> LocalizedLabel {
        match self {
            Self::Iterations => LocalizedLabel::native("Iterations", "Iterationen"),
            Self::Energy => LocalizedLabel::native("Kinetic energy (thousandths)", "Kinetische Energie (Tausendstel)"),
            Self::MaxDisplacement => LocalizedLabel::native("Largest displacement (thousandths)", "Größte Verschiebung (Tausendstel)"),
            Self::Settled => LocalizedLabel::native("Settled nodes", "Ruhende Knoten"),
        }
    }
}

/// 🗯️ Trace and step reasons; `code()` is the ordinal.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LayoutRunReason {
    Moving,
    Settled,
    Pinned,
    Diverged,
    Unsettled,
    Initialized,
    Converged,
    IterationLimit,
    Resumed,
}

impl LayoutRunReason {
    pub const ALL: [Self; 9] = [Self::Moving, Self::Settled, Self::Pinned, Self::Diverged, Self::Unsettled, Self::Initialized, Self::Converged, Self::IterationLimit, Self::Resumed];

    pub fn code(self) -> u16 {
        self as u16
    }

    pub fn from_code(code: u16) -> Option<Self> {
        Self::ALL.get(usize::from(code)).copied()
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::Moving => "moving",
            Self::Settled => "settled",
            Self::Pinned => "pinned",
            Self::Diverged => "diverged",
            Self::Unsettled => "unsettled",
            Self::Initialized => "initialized",
            Self::Converged => "converged",
            Self::IterationLimit => "iterationLimit",
            Self::Resumed => "resumed",
        }
    }

    pub fn verdict(self) -> ToolRunVerdict {
        match self {
            Self::Moving | Self::Initialized | Self::Resumed => ToolRunVerdict::Testing,
            Self::Settled | Self::Pinned | Self::Converged => ToolRunVerdict::Success,
            Self::Unsettled | Self::IterationLimit => ToolRunVerdict::Warning,
            Self::Diverged => ToolRunVerdict::Danger,
        }
    }

    pub fn template(self) -> LocalizedLabel {
        match self {
            Self::Moving => LocalizedLabel::native("Moving", "In Bewegung"),
            Self::Settled => LocalizedLabel::native("Settled", "Zur Ruhe gekommen"),
            Self::Pinned => LocalizedLabel::native("Pinned", "Fixiert"),
            Self::Diverged => LocalizedLabel::native("{0} nodes diverged and were reset to their last finite position", "{0} Knoten divergierten und wurden auf ihre letzte endliche Position zurückgesetzt"),
            Self::Unsettled => LocalizedLabel::native("Still moving when the iteration limit was reached", "Beim Erreichen der Iterationsgrenze noch in Bewegung"),
            Self::Initialized => LocalizedLabel::native("{0} nodes and {1} edges prepared", "{0} Knoten und {1} Kanten vorbereitet"),
            Self::Converged => LocalizedLabel::native("Converged after {0} iterations (largest displacement {1}, energy {2})", "Nach {0} Iterationen konvergiert (größte Verschiebung {1}, Energie {2})"),
            Self::IterationLimit => LocalizedLabel::native("Iteration limit {0} reached (largest displacement {1}, energy {2})", "Iterationsgrenze {0} erreicht (größte Verschiebung {1}, Energie {2})"),
            Self::Resumed => LocalizedLabel::native("Resumed at iteration {0}", "Bei Iteration {0} fortgesetzt"),
        }
    }
}

/// 🏁️ Why a run settled.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum LayoutRunStop {
    Converged,
    IterationLimit,
}

/// 📜️ The `ToolRunDefinition` every consumer declares for its layout tool: mutating, `rebase: restart` (a changed
/// node set invalidates the graph), `reconfigure: resume` (settings continue from the last checkpoint), entity trace.
pub fn layout_run_definition(run_job: JobKindId) -> ToolRunDefinition {
    ToolRunDefinition { receiving: semio_framework_tool_run::ToolRunReceivingLimits::REPORTS,
        mutating: true,
        rebase: ToolRunRebasePolicy::Restart,
        reconfigure: ToolRunReconfigurePolicy::Resume,
        unit: LocalizedLabel::native("iteration", "Iteration"),
        stages: LayoutRunStage::ALL.iter().map(|stage| ToolRunStageDefinition { id: stage.id().to_string(), label: stage.label() }).collect(),
        counters: LayoutRunCounter::ALL.iter().map(|counter| ToolRunCounterDefinition { id: counter.id().to_string(), label: counter.label() }).collect(),
        reasons: LayoutRunReason::ALL.iter().map(|reason| ToolRunReasonDefinition { code: reason.code(), id: reason.id().to_string(), verdict: reason.verdict(), template: reason.template() }).collect(),
        trace: ToolRunTraceKind::Entity,
        run_job,
        revalidate_job: None,
        settings: ToolRunSettingsReads::default(),
        windows: Vec::new(),
        member: None,
    }
}
//#endregion 🔖️Vocabulary
