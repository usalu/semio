//! 🫎️ Staged slice of the `tool-run` band of the mounted artifact instance: its `instance-children` members (WP6 step S0 of the instance family).
//!
//! ⏯️ The mounted instance's tool-run turns (ledger driving, tick folding and finalize publication) and the
//! [`ToolRunExtension`] that registers them with the instance through [`InstanceExtension`].
use semio_framework_plugin_tool_run::ToolRunMemberFold;
use semio_framework_plugin_tool_run::ToolRunMemberGeneration;
use semio_framework_plugin_tool_run::ToolRunMemberRun;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use store::Mutation;
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use crate::children_components::ChildComposition;
use semio_framework_plugin_child::ChildContentRetirement;
use semio_framework_plugin_decl_views::child_view::ChildContentView;
use semio_framework::{Fault, FaultCode, FaultOrigin};
use std::future::Future;
use semio_framework_ui_contract::*;
use semio_framework_plugin_tool_run::*;
use semio_framework_plugin_decl_views::child_view::MemberKeyRef;
use semio_framework_os_kernel::{MemberFactory, ReferenceSpaceMember};
use semio_framework_value::RetainedCloneGrant;

impl<M: ReferenceSpaceMember + MemberFactory + 'static> ChildComposition<M> {
    /// 🧩️ The member a run declaring `ToolRunDefinition::member` edits: the one live owned child composed under `slot`, with
    /// the member store generation the run rests on. No such child, or several, refuse the start by name.
    pub fn tool_run_member_target(&self, slot: &str) -> Result<ToolRunMemberRun, Fault> {
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
    /// `None` when the overlay already shows every op; `receipt` carries the retained progress the fold paid for, to be retained on the run ledger.
    pub fn refresh_member_overlay(&mut self, member: &mut ToolRunMemberRun, deadline: u64,grant:RetainedCloneGrant,receiving:&mut semio_framework_value::NativeDecodeControl<'_>,receipt:&mut semio_framework_value::RetainedCloneProgress) -> Result<Option<(bool, u32, bool)>, Fault> {
        let stale = std::mem::take(&mut member.stale);
        self.retire_tool_run_views(stale, &mut member.stale)?;
        if !member.rebase && member.shown == member.ops.len() {
            return Ok(None);
        }
        let child = self.children.get(&(member.slot.clone(), member.child_id.clone())).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("toolRun.member-gone"), format!("the member {}/{} a tool run edits is gone", member.slot, member.child_id)))?;
        let rebased = member.rebase;
        let folded = match child.member.visit_member(ToolRunMemberFold { owners: &mut member.owners, ops: &member.ops, shown: member.shown, rebase: member.rebase, deadline_us: deadline,grant,receiving }){Ok(folded)=>folded,Err(ToolRunMemberFoldRefusal::Native(error))=>{*receipt=error.retained_progress();return Err(error.into_fault())},Err(ToolRunMemberFoldRefusal::Semantic(error))=>return Err(error)};
        *receipt=folded.progress;
        member.rebase = false;
        member.shown = folded.shown;
        let pending = folded.pending;
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
    pub fn retire_tool_run_views(&mut self, views: Vec<ChildContentView>, unretired: &mut Vec<ChildContentView>) -> Result<(), Fault> {
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
