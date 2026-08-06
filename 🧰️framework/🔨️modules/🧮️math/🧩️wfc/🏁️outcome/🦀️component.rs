//! 🏁️ What a solve attempt concludes with. `Contradiction`/`Unsatisfiable` are ordinary outcomes,
//! never [`crate::wfc::error`] variants — a search finding no solution is not a bug.

use crate::wfc::bitset::PatternSet;
use crate::wfc::diag::{Event, Metrics};
use crate::wfc::ids::{NodeId, PatternId};

// #region 🔖️Report
/// 🏁️ Bookkeeping attached to every [`SolveOutcome`] variant.
#[derive(Clone, Debug)]
pub struct RunReport {
    pub metrics: Metrics,
    pub model_fingerprint: u64,
    pub seed: u64,
    pub events: Vec<Event>,
}
// #endregion 🔖️Report

// #region 🔖️Solution
/// 🏁️ A complete, hard-constraint-satisfying assignment (index = `NodeId`).
#[derive(Clone, Debug)]
pub struct Solution {
    pub assignment: Vec<PatternId>,
    pub report: RunReport,
}

/// 🏁️ A proof of unsatisfiability. `proven = true` only when the entire search tree was exhausted
/// without finding a solution — restart-only search that simply gives up never sets this.
#[derive(Clone, Debug)]
pub struct UnsatReport {
    pub proven: bool,
    pub report: RunReport,
}

/// 🏁️ A search branch that hit an empty domain and (for restart-only search) exhausted its
/// restart budget before finding a solution.
#[derive(Clone, Debug)]
pub struct ContradictionReport {
    pub node: NodeId,
    pub report: RunReport,
}

/// 🏁️ Domains and decided assignments at the moment a solve stopped without concluding.
#[derive(Clone, Debug)]
pub struct PartialState {
    pub domains: Vec<PatternSet>,
    pub decided: Vec<Option<PatternId>>,
}
// #endregion 🔖️Solution

// #region 🔖️Outcome
/// 🏁️ The five ways a solve attempt can end.
#[derive(Clone, Debug)]
pub enum SolveOutcome {
    Solved(Solution),
    Unsatisfiable(UnsatReport),
    Contradiction(ContradictionReport),
    BudgetExceeded {
        partial: PartialState,
        report: RunReport,
    },
    /// 🏁️ A caller-supplied [`crate::wfc::search::CancelToken`] was set before the attempt concluded.
    Cancelled {
        partial: PartialState,
        report: RunReport,
    },
}
// #endregion 🔖️Outcome
