//! 🗜️ Staged slice of the `ingress` band of the mounted artifact instance: its `instance-ladders` members (WP6 step S0 of the instance family).
//!
//! 🚪️ Ingress of the mounted artifact instance: framework-reserved job driving, constructors, store-constructor units, envelope ingress and decode, document archive genesis and merge, child members and packs.
use super::*;
use crate::tool_jobs::time_travel;

impl<'ctx, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension> LadderCtx<'ctx, A, M, X> {
    /// 🌳️ Supplies the exact live and saved previews to the common original child disposer.
    pub(crate) fn child_root_retirement_unit(&mut self, grant: RetainedCloneGrant, closing: bool) -> Result<RetainedCloneStep, Fault> {
        let member = self.time_travel.member.as_ref().and_then(|member| member.children.as_ref());
        let review = self.time_travel.member.as_ref().and_then(|member| member.review_children.as_ref().map(|review|&review.view));
        advance_original_child_root_retirement(&mut self.child_composition.child_content_retirements, &mut self.child_composition.children, Some(&mut self.child_composition.child_member_retirements), &self.child_composition.child_content_root, member, review, &mut self.maintenance.maintenance_child_root_cursor, grant, closing)
    }

    /// 🎟️ Advances one child-member retirement under the caller's unchanged grant.
    pub(super) fn child_member_retirement_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, Fault> {
        self.child_composition.child_member_retirement_unit(grant, self.close.close_child_root_detached).map(|step| step.progress())
    }
}

impl<'v, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension> LadderRef<'v, A, M, X> {
    /// 🕰️ The fixed maintenance stage the NEXT [`PluginApp::maintenance_step`] call will run.
    /// A step-budget law reads this before each unit so an over-budget unit names its own stage
    /// instead of the stale round-robin cursor an idle early return leaves behind.
    pub fn next_maintenance_stage(&self) -> u8 {
        self.maintenance.maintenance_stage
    }

    /// 📐️ Quotes the original registry and its real child snapshot owners; an empty registry owes only its backing, and only once the close has detached the child root that still inserts its retirement.
    fn child_root_retirement_quote(&self, body: usize, closing: bool) -> Result<Option<RetirementDemand>, ValueError> {
        original_child_root_retirement_demands(&self.child_composition.child_content_retirements, &self.child_composition.children, Some(&self.child_composition.child_member_retirements), self.maintenance.maintenance_child_root_cursor, body, closing)
    }

    /// 📏️ Quotes the next child-root retirement turn; nothing is owed once the registry and its backing are gone.
    pub(super) fn child_root_retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        Ok(self.child_root_retirement_quote(body, self.close.close_child_root_detached)?.unwrap_or_default())
    }

    /// 📏️ Quotes the next child-member retirement turn; nothing is owed once the registry and its backing are gone.
    pub(super) fn child_member_retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        Ok(self.child_composition.child_member_retirement_quote(body, self.close.close_child_root_detached)?.unwrap_or_default())
    }
}

impl<'ctx, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension> LadderCtx<'ctx, A, M, X> {
    /// 🎟️ Advances one child-root retirement under the caller's unchanged grant.
    pub(super) fn child_root_retirement_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, Fault> {
        self.child_root_retirement_unit(grant, self.close.close_child_root_detached).map(|step| step.progress())
    }
}
