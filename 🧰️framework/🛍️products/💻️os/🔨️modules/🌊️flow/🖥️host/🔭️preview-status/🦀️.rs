//! 🔭️ Preview tessellation and evaluation phases, progress, status and outcome values.

use super::*;

/// ⏱️ Where one preview tessellation currently is — the wire-stable mirror of the kernel's own
/// `TessellationPhase`, widened by the two boundary-only outcomes (`invalid`, `failed`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PreviewTessellatePhase {
    #[default]
    Idle,
    AwaitingGrant,
    AdmittingSource,
    Validating,
    SamplingEdges,
    MeshingFaces,
    PackingEdges,
    Transferring,
    Complete,
    Cancelled,
    Invalid,
    /// 💥️ The kernel itself answered with a failure.
    Failed,
    /// 🪪️ No kernel could even be addressed — the geometry extension this preview needs is not
    /// contributed by any loaded plugin, so no invocation was ever emitted. Boundary-only, like
    /// `Invalid`/`Failed`: a kernel never reports it, the producing app raises it from its own
    /// `flow_extension_invocation_address` miss (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    Faulted,
}

impl PreviewTessellatePhase {
    /// 🏷️ The stable wire tag, shared by the extension envelope, the status object and the UI.
    pub fn tag(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::AwaitingGrant => "awaitingGrant",
            Self::AdmittingSource => "admittingSource",
            Self::Validating => "validating",
            Self::SamplingEdges => "samplingEdges",
            Self::MeshingFaces => "meshingFaces",
            Self::PackingEdges => "packingEdges",
            Self::Transferring => "transferring",
            Self::Complete => "complete",
            Self::Cancelled => "cancelled",
            Self::Invalid => "invalid",
            Self::Failed => "failed",
            Self::Faulted => "faulted",
        }
    }

    /// 🏷️ Inverse of [`PreviewTessellatePhase::tag`]; an unknown tag reads as `Idle`.
    pub fn from_tag(tag: &str) -> Self {
        match tag {
            "awaitingGrant" => Self::AwaitingGrant,
            "admittingSource" => Self::AdmittingSource,
            "validating" => Self::Validating,
            "samplingEdges" => Self::SamplingEdges,
            "meshingFaces" => Self::MeshingFaces,
            "packingEdges" => Self::PackingEdges,
            "transferring" => Self::Transferring,
            "complete" => Self::Complete,
            "cancelled" => Self::Cancelled,
            "invalid" => Self::Invalid,
            "failed" => Self::Failed,
            "faulted" => Self::Faulted,
            _ => Self::Idle,
        }
    }

    /// 🛑 True while a cancel can still retire the job.
    pub fn is_cancellable(self) -> bool {
        matches!(self, Self::AwaitingGrant | Self::AdmittingSource | Self::Validating | Self::SamplingEdges | Self::MeshingFaces | Self::PackingEdges | Self::Transferring)
    }

    /// 🌍 English and German labels — the UI carries both, with no default language.
    pub fn labels(self) -> (&'static str, &'static str) {
        match self {
            Self::Idle => ("Idle", "Bereit"),
            Self::AwaitingGrant => ("Awaiting admission", "Warten auf Zulassung"),
            Self::AdmittingSource => ("Admitting geometry", "Geometrie wird zugelassen"),
            Self::Validating => ("Validating geometry", "Geometrie wird geprüft"),
            Self::SamplingEdges => ("Sampling edges", "Kanten werden abgetastet"),
            Self::MeshingFaces => ("Meshing faces", "Flächen werden vernetzt"),
            Self::PackingEdges => ("Packing edges", "Kanten werden gepackt"),
            Self::Transferring => ("Transferring mesh", "Netz wird übertragen"),
            Self::Complete => ("Complete", "Fertig"),
            Self::Cancelled => ("Cancelled", "Abgebrochen"),
            Self::Invalid => ("Invalid geometry", "Ungültige Geometrie"),
            Self::Failed => ("Failed", "Fehlgeschlagen"),
            Self::Faulted => ("Geometry extension unavailable", "Geometrie-Erweiterung nicht verfügbar"),
        }
    }
}

/// ⏱️ Where one BUDGETED `evaluate` round trip currently is. The extension's job reports its own
/// domain phase tag (`imprint`, `classifyA`, `validate`, …); this enum is the wire-stable,
/// localizable vocabulary the surface paints, and `Computing` is the honest answer for any tag a
/// future operator invents (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PreviewEvalPhase {
    #[default]
    Idle,
    /// 🧮️ A stepped operator is working and named a phase this vocabulary does not know.
    Computing,
    /// ✂️ Imprinting intersection curves onto the operands' faces.
    Imprinting,
    /// 🧩 Splitting imprinted faces into pieces.
    Splitting,
    /// 🎯 Classifying each piece against the other operand.
    Classifying,
    /// 🧵 Stitching the kept faces into shells and solids.
    Stitching,
    /// 🩺 Validating the result.
    Validating,
    Complete,
    Cancelled,
}

impl PreviewEvalPhase {
    /// 🏷️ The stable wire tag the status object publishes.
    pub fn tag(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Computing => "computing",
            Self::Imprinting => "imprinting",
            Self::Splitting => "splitting",
            Self::Classifying => "classifying",
            Self::Stitching => "stitching",
            Self::Validating => "validating",
            Self::Complete => "complete",
            Self::Cancelled => "cancelled",
        }
    }

    /// 🏷️ Reads the extension job's own phase tag. An unknown tag is `Computing`, never `Idle`: the
    /// envelope that carried it said the job is still working, and publishing `idle` for live work
    /// is the exact defect this ledger exists to close.
    pub fn from_job_tag(tag: &str) -> Self {
        match tag {
            "imprint" => Self::Imprinting,
            "applyA" | "applyB" => Self::Splitting,
            "classifyA" | "classifyB" => Self::Classifying,
            "stitch" => Self::Stitching,
            "validate" => Self::Validating,
            "complete" => Self::Complete,
            "cancelled" => Self::Cancelled,
            "idle" => Self::Idle,
            _ => Self::Computing,
        }
    }

    /// 🛑 True while a cancel can still retire the evaluation.
    pub fn is_cancellable(self) -> bool {
        matches!(self, Self::Computing | Self::Imprinting | Self::Splitting | Self::Classifying | Self::Stitching | Self::Validating)
    }

    /// 🌍 English and German labels — the UI carries both, with no default language.
    pub fn labels(self) -> (&'static str, &'static str) {
        match self {
            Self::Idle => ("Idle", "Bereit"),
            Self::Computing => ("Computing", "Berechnen"),
            Self::Imprinting => ("Imprinting intersections", "Schnittkurven werden eingeprägt"),
            Self::Splitting => ("Splitting faces", "Flächen werden geteilt"),
            Self::Classifying => ("Classifying faces", "Flächen werden klassifiziert"),
            Self::Stitching => ("Stitching shells", "Schalen werden vernäht"),
            Self::Validating => ("Validating solid", "Körper wird geprüft"),
            Self::Complete => ("Complete", "Fertig"),
            Self::Cancelled => ("Cancelled", "Abgebrochen"),
        }
    }
}

/// 📈 One budgeted evaluation's monotone progress.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PreviewEvalProgress {
    pub units_done: u64,
    pub units_total: u64,
    pub phase: PreviewEvalPhase,
}

semio_framework_value::artifact_retire_leaf!(PreviewEvalProgress);

/// 📈 The whole session's budgeted-evaluation state, as the status object reports it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PreviewEvalStatus {
    pub units_done: u64,
    pub units_total: u64,
    pub in_flight: u32,
    pub phase: PreviewEvalPhase,
}

impl PreviewEvalStatus {
    /// 📈 Fraction of the admitted evaluation work already done, in `[0, 1]`.
    pub fn ratio(&self) -> f64 {
        if self.units_total == 0 {
            return if self.in_flight == 0 { 1.0 } else { 0.0 };
        }
        (self.units_done as f64 / self.units_total as f64).clamp(0.0, 1.0)
    }

    /// 🛑 True while an explicit cancel would still retire something.
    pub fn is_cancellable(&self) -> bool {
        self.in_flight > 0 || self.phase.is_cancellable()
    }
}

/// ⛓️ The whole preview evaluation chain's state, as the status object reports it when no finer
/// ledger has anything to say — see [`FlowEvalSession::preview_chain_status`] for why the finer two
/// are structurally silent at every hop boundary.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PreviewChainStatus {
    /// 📈 Nodes this chain has settled (anything but `queued`/`computing`/`stale`).
    pub nodes_done: u32,
    /// 📈 Nodes the published census knows about at all.
    pub nodes_total: u32,
    /// ⏳️ Extension answers outstanding across every attached preview window.
    pub in_flight: u32,
    /// ⏳️ Whether ANY window still owes, awaits or is running a hop.
    pub working: bool,
}

impl PreviewChainStatus {
    /// 📈 The units this chain owes, as `(done, total)` — the pair a surface publishes and a pill
    /// prints as `n/m`.
    ///
    /// ⚖️ The denominator is the node census, raised to `done + 1` while the chain is still working:
    /// a census that has seen every node settle can still be waiting on the answer that will produce
    /// the mesh, and `4/4` there would say "done" about work a cancel could still stop — the same
    /// rule [`PreviewTessellateStatus::ratio`] applies throughout original receiving.
    ///
    /// 🩸️ The outstanding round trips are deliberately NOT in the denominator. Counting them made it
    /// oscillate with every park and fold — `2/3` at a parked request, `2/4` the moment it settled —
    /// so the published ratio went BACKWARDS mid-evaluation, which is worse than no progress at all.
    /// The census is the one quantity that only ever grows inside one chain.
    pub fn units(&self) -> (u32, u32) {
        if !self.working {
            return (self.nodes_total, self.nodes_total);
        }
        (self.nodes_done, self.nodes_total.max(self.nodes_done.saturating_add(1)))
    }

    /// 🏁️ Whether this chain has SETTLED: it owes, awaits and runs nothing, and its own census
    /// accounts for every node it published.
    ///
    /// ⚖️ This is the guest's OWN answer to "is the evaluation over", and it outranks the run view on
    /// that question. A `ToolRunView` is a host artifact that reaches a surface on a render, so it
    /// LAGS — and a run whose job has finished but whose terminal state has not been rendered back
    /// yet kept `computing` and the `toolRunAbort` affordance raised for good: a spinner that
    /// outlives its work and a Cancel button for an evaluation with nothing left to cancel
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️flow-inline-continuation-2026-09-14.md`).
    ///
    /// 🚦️ A census of ZERO nodes is deliberately NOT settled. Before the first hop publishes one
    /// there is nothing to be done with, and that is exactly the window in which the run legitimately
    /// knows more than the chain — it was started by a gesture whose first tick has not run yet.
    pub fn settled(&self) -> bool {
        !self.working && self.nodes_total > 0 && self.nodes_done >= self.nodes_total
    }

    /// 📈 Fraction of the chain's units already done, in `[0, 1]`. A working chain never reports `1`:
    /// "done" is the one answer a live evaluation may not give.
    pub fn ratio(&self) -> f64 {
        let (done, total) = self.units();
        if total == 0 {
            return 1.0;
        }
        (f64::from(done) / f64::from(total)).clamp(0.0, 1.0)
    }
}

/// ✅️ What folding one budgeted `evaluate` envelope achieved.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PreviewEvalOutcome {
    Pending(neural::PendingExtensionEval),
    /// 🔁 The extension's job is still working — arm another round trip for the SAME request.
    Working,
    /// ✅ The operator answered; `output_json` is the out dictionary to seed.
    Complete { output_json: String },
    /// 🛑 The job was retired; nothing is produced and nothing is owed.
    Cancelled,
}

impl PreviewEvalOutcome {
    /// 🔁 Whether the chain owes another identical `evaluate` round trip.
    pub fn needs_another_round_trip(&self) -> bool {
        matches!(self, Self::Working|Self::Pending(_))
    }
}

/// 📈 One tessellation's monotone progress plus its mesh-body chunk cursor.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PreviewTessellateProgress {
    pub units_done: u64,
    pub units_total: u64,
    pub faces_done: u64,
    pub faces_total: u64,
    pub phase: PreviewTessellatePhase,
}

semio_framework_value::artifact_retire_leaf!(PreviewTessellateProgress);

/// 📈 The whole session's preview tessellation state, as the status object reports it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PreviewTessellateStatus {
    pub units_done: u64,
    pub units_total: u64,
    pub faces_done: u64,
    pub faces_total: u64,
    /// ⏳️ Original requests remain in flight through canonical receiving and cache admission.
    pub in_flight: u32,
    pub diagnostics: u32,
    pub phase: PreviewTessellatePhase,
}

impl PreviewTessellateStatus {
    /// 📈 Fraction of the admitted work already done, in `[0, 1]`.
    pub fn ratio(&self) -> f64 {
        let done = u64::from(self.units_done);
        let total = u64::from(self.units_total);
        if total == 0 {
            return if self.in_flight == 0 { 1.0 } else { 0.0 };
        }
        ((done as f64) / (total as f64)).clamp(0.0, 1.0)
    }

    /// 🛑 True while an explicit cancel would still retire something.
    pub fn is_cancellable(&self) -> bool {
        self.in_flight > 0 || self.phase.is_cancellable()
    }
}

/// ✅ What one folded `tessellate` response did to the session.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PreviewTessellateOutcome {
    /// 🔁️ Original kernel or receiving work remains.
    Working,
    /// ✅ The complete mesh landed under its handle.
    Ready,
    /// 🩺 The validate gate rejected the topology; a typed diagnostic is stored for the handle.
    Invalid,
    /// 🛑 The job was cancelled.
    Cancelled,
    /// 💥 The extension faulted or sent an undecodable body.
    Failed,
    /// ❓ No pending request carried this `nodeHash` — a stale or superseded response.
    Unknown,
}

impl PreviewTessellateOutcome {
    /// 🔁 True when the caller must dispatch another tick to continue this tessellation.
    pub fn needs_another_round_trip(self) -> bool {
        matches!(self, Self::Working)
    }
}
