//! 🫎️ Staged slice of the `tool-run` band of the mounted artifact instance: its `instance-children` members (WP6 step S0 of the instance family).
//!
//! ⏯️ The mounted instance's tool-run turns (ledger driving, tick folding and finalize publication) and the
//! [`ToolRunExtension`] that registers them with the instance through [`InstanceExtension`].
use super::*;
use crate::tool_jobs::tool_run::*;
use semio_framework_tool_run::{
    tool_run_pointer_value, ToolRunAction, ToolRunEffect, ToolRunEvent, ToolRunId, ToolRunIdentity, ToolRunLane, ToolRunMachine, ToolRunRebasePolicy, ToolRunReconfigurePolicy,
    ToolRunRejection, ToolRunSettingsReads, ToolRunState, ToolRunStepArg, ToolRunStepKind, ToolRunStepRing, ToolRunTick, ToolRunTraceStore, TOOL_RUN_ARG_GENERATION, TOOL_RUN_ARG_RUN_ID, TOOL_RUN_ARG_TOOL_ID, TOOL_RUN_ARG_WINDOW_ID, TOOL_RUN_REASON_CONFLICT, TOOL_RUN_REASON_REBASING, TOOL_RUN_TICK_BYTES_MAX,
};
use std::sync::Arc;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,admit_retained_clone_close}};
use crate::artifact_instance::InstanceExtension;
use crate::artifact_instance::VcsArtifactApp;
use crate::app_emission::ActionMeta;
use crate::child_view::MemberKeyRef;
use crate::tool_run_contract::ToolRunJobRequest;
use semio_framework::{ActionKind, ViewModel};
use store::{os_store::store_schema::HistoryLane, MemberFactory, SpaceMember};
use semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority;

impl<M: SpaceMember + MemberFactory + 'static> ChildComposition<M> {
    /// 🧩️ The member a run declaring `ToolRunDefinition::member` edits: the one live owned child composed under `slot`, with
    /// the member store generation the run rests on. No such child, or several, refuse the start by name.
    pub(crate) fn tool_run_member_target(&self, slot: &str) -> Result<ToolRunMemberRun, Fault> {
        let mut candidates = self.children.keyed_entries().filter(|(key, _)| key.owner.is_empty() && key.slot == slot).map(|(_, entry)| entry);
        let (Some(entry), None) = (candidates.next(), candidates.next()) else {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("toolRun.member-unavailable"), format!("a member tool run needs exactly one live owned child in slot '{slot}'")));
        };
        Ok(ToolRunMemberRun {
            slot: slot.to_string(),
            child_id: entry.owner.child_id.clone(),
            dialect: entry.reference.dialect.clone(),
            base_generation: entry.member.visit_member(ToolRunMemberGeneration),
            ops: Vec::new(),
            shown: 0,
            rebase: false,
            owners: None,
            children: None,
            stale: Vec::new(),
            emit:None,emission_owner:None,
            retired_emits:std::collections::VecDeque::new(),retired_emission_owners:std::collections::VecDeque::new(),retired_ops:std::collections::VecDeque::new(),
        })
    }

    /// 🔁️ [`Self::refresh_tool_run_member`] on a member taken out of its entry: `(fold pending, conflicts, rebased)`, or
    /// `None` when the overlay already shows every op.
    pub(crate) fn refresh_member_overlay(&mut self, member: &mut ToolRunMemberRun, deadline: u64) -> Result<Option<(bool, u32, bool)>, Fault> {
        let stale = std::mem::take(&mut member.stale);
        self.retire_tool_run_views(stale, &mut member.stale)?;
        if !member.rebase && member.shown == member.ops.len() {
            return Ok(None);
        }
        let child = self.children.get(&(member.slot.clone(), member.child_id.clone())).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("toolRun.member-gone"), format!("the member {}/{} a tool run edits is gone", member.slot, member.child_id)))?;
        let rebased = member.rebase;
        let folded = child.member.visit_member(ToolRunMemberFold { owners: &mut member.owners, ops: &member.ops, shown: member.shown, rebase: member.rebase, deadline_us: deadline })?;
        member.rebase = false;
        member.shown = folded.shown;
        let pending = folded.shown < member.ops.len();
        if !pending {
            let next = match folded.read {
                Some((read, revision)) => Some(self.child_content_root.with_member_read(MemberKeyRef::root(&member.slot, &member.child_id), &member.dialect, revision, read)?),
                None => None,
            };
            if let Some(previous) = std::mem::replace(&mut member.children, next) {
                match self.retire_tool_run_views(vec![previous], &mut member.stale) {
                    Err(fault) if !fault.retryable => return Err(fault),
                    _ => {}
                }
            }
        }
        Ok(Some((pending, folded.conflicts, rebased)))
    }

    /// 🧹️ Retires a member run's composed reads in order through the child-content retirements, one admitted generation
    /// each; a refused admission hands the view it could not retire and every later one back to `unretired`.
    pub(crate) fn retire_tool_run_views(&mut self, views: Vec<ChildContentView>, unretired: &mut Vec<ChildContentView>) -> Result<(), Fault> {
        let mut views = views.into_iter();
        while let Some(view) = views.next() {
            match self.admit_child_content_publication() {
                Ok(generation) => {
                    self.child_content_retirements.insert_admitted(generation, ChildContentRetirement::new(view, false));
                    self.child_content_generation = generation;
                }
                Err(fault) => {
                    unretired.push(view);
                    unretired.extend(views);
                    return Err(fault);
                }
            }
        }
        Ok(())
    }
}
