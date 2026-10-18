//! 🧮️ Surface reconciler: the `SurfaceReconciler` that drives a surface document reconcile and its drop. See [`super`].

use super::*;

//#region 🔖️Reconciler
/// ♻️ Keyed differ for one render surface: the retained [`SurfaceShadow`] it diffs against, composed with the
/// handback reservation it owes the process-wide registry. The shadow owns every retained owner and its
/// retirement; this owner only adds the registry hand-off that outlives it.
#[derive(Debug)]
pub struct SurfaceReconciler {
    pub(super) shadow: SurfaceShadow,
    pub(super) handback: Option<SurfaceReconcileHandbackReservation>,
    retirement_armed: bool,
}

impl SurfaceReconciler {
    /// 🌱️ A reconciler for `surface` with no retained state yet — the next reconcile necessarily emits a full
    /// `SetRoot` plus one `Upsert` per node.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new(surface: ui_contract::SurfaceId) -> Self {
        Self::from_shadow(SurfaceShadow::new(surface))
    }

    pub(super) fn from_shadow(shadow: SurfaceShadow) -> Self {
        Self { shadow, handback: None, retirement_armed: true }
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn from_text(surface: impl AsRef<str>) -> Self {
        Self::from_shadow(SurfaceShadow::from_text(surface))
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn reconcile(&mut self, tree: &ComponentTree) -> Option<ui_contract::UiPatch> {
        self.shadow.reconcile(tree)
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn snapshot(&self) -> ui_contract::UiSnapshot {
        self.shadow.snapshot()
    }

    /// 🧬️ Returns the retained scalar revision without cloning the retained document.
    pub fn revision(&self) -> ui_contract::UiRevision {
        self.shadow.revision()
    }

    /// 🪪️ Returns the reconciler's surface identity without allocating a second owner.
    pub fn surface(&self) -> &ui_contract::SurfaceId {
        self.shadow.surface()
    }

    /// 📖️ Captures the canonical root without a second payload tree or reservation.
    pub fn capture_document(&self, target: &mut Option<ui_contract::UiDocumentLease>, admitted_bytes: usize) -> Result<bool, ui_contract::UiDocumentLeaseError> {
        self.shadow.capture_document(target, admitted_bytes)
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn transaction_reader(&self) -> Self {
        Self::from_shadow(self.shadow.transaction_reader())
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn close_transaction_oracle(&mut self) {
        while !self.retire_one() {}
    }

    pub(super) fn seal_step(&mut self, usage: SurfaceReconcileUsage, output: &mut Option<ui_contract::UiResidentPermit>, has_output: bool) -> Result<bool, ui_contract::UiDocumentAssemblyError> {
        self.shadow.seal_step(size_of::<Self>(), usage, output, has_output)
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn mark_rejected(&mut self) {
        self.shadow.mark_rejected();
    }

    pub fn native_resources(&self) -> Option<&UiNativeResources> { self.shadow.native_resources() }

    pub fn native_resource_loans_terminal_is_empty(&self)->bool{self.shadow.native_resource_loans_terminal_is_empty()}

    pub fn with_native_resource_loans_on_original<R>(&mut self,cx:&mut semio_framework_job::StepContext<'_>,receive:impl FnOnce(&mut UiNativeResourceLoans,&mut semio_framework_job::StepContext<'_>)->Result<R,semio_framework_job::OriginalTurnFailure>)->Result<Option<R>,semio_framework_job::OriginalTurnFailure>{
        self.shadow.with_native_resource_loans_on_original(cx,receive)
    }

    pub fn close_native_resources_on_original(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<bool>, semio_framework_job::OriginalTurnFailure> {
        self.shadow.close_native_resources_on_original(cx)
    }

    pub(super) fn retire_one(&mut self) -> bool {
        self.shadow.retire_one(&mut self.handback, release_surface_reconcile_handback)
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn install_fixture_record(&mut self, record: ui_contract::UiNodeRecord) {
        self.shadow.install_fixture_record(record);
    }
}

impl Drop for SurfaceReconciler {
    fn drop(&mut self) {
        if !self.retirement_armed {
            return;
        }
        self.retirement_armed = false;
        if self.shadow.terminal_is_empty() {
            if let Some(handback) = self.handback.take() {
                release_surface_reconcile_handback(handback);
            }
            return;
        }
        let generation = self.handback.as_ref().map_or(0, |reservation| reservation.key.generation);
        let owner = SurfaceReconciler { shadow: self.shadow.take_all(), handback: None, retirement_armed: false };
        let state = Box::new(SurfaceReconcileRetained {
            output_handback: None,
            generation,
            phase: SurfaceReconcileJobPhase::Closing,
            current: Some(owner),
            source: None,
            cursor: None,
            candidate: None,
            patch: ui_contract::UiPendingPatch::default(),
            retire_tree: SurfaceTreeRetireCursor::default(),
            fault: None,
            usage: SurfaceReconcileUsage::default(),
            credit: None,
            handback: self.handback.take(),
        });
        handback_surface_reconcile(state);
    }
}
//#endregion 🔖️Reconciler
