//! 🧭️ Cursor state and construction: the reconcile stage and step, the `SurfaceReconcileCursor` state and its construction, usage and pending accessors. See [`super`].

use super::*;

//#region ⏭️ResumableReconcile
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceReconcileStage {
    TraversePresentation,
    AllocateIdentities,
    DiffRecords,
    RemoveStale,
    Finalize,
}

#[derive(Debug)]
#[expect(clippy::large_enum_variant, reason = "Completion transfers the admitted shadow and patch by value without allocating another owner.")]
pub enum SurfaceReconcileStep {
    Yield { nodes: usize, bytes: usize },
    Complete { shadow: SurfaceShadow, patch: Option<ui_contract::UiPatch> },
    Fault(SurfaceReconcileFault),
}

/// ⏭️ Persistent one-node-at-a-time traversal and keyed differ for a single presented surface.
/// The retained shadow is read-only until `Complete`; abandoning this cursor therefore abandons
/// every candidate identity, record, operation, and revision together.
pub struct SurfaceReconcileCursor {
    pub(super) stage: SurfaceReconcileStage,
    pub(super) surface: ui_contract::SurfaceId,
    pub(super) base_revision: ui_contract::UiRevision,
    pub(super) allocator: ui_contract::UiNodeIdAllocator,
    pub(super) old_root: Option<ui_contract::UiNodeId>,
    pub(super) pending_root: Option<TreeNode>,
    pub(super) native_resources: Option<UiNativeResources>,
    pub(super) traversal: SurfaceFixedVec<PresentationFrame, SURFACE_RECONCILE_VALUE_DEPTH>,
    pub(super) overflow_frame: Option<PresentationFrame>,
    pub(super) flat: SurfaceFixedVec<FlatPresentedNode, SURFACE_RECONCILE_FIXED_NODES>,
    pub(super) postorder: SurfaceFixedVec<usize, SURFACE_RECONCILE_FIXED_NODES>,
    pub(super) seen: SurfaceLinearSet<(Option<usize>, ui_contract::UiText), SURFACE_RECONCILE_FIXED_NODES>,
    pub(super) ids: SurfaceFixedVec<ui_contract::UiNodeId, SURFACE_RECONCILE_FIXED_NODES>,
    pub(super) allocate_index: usize,
    pub(super) diff_index: usize,
    pub(super) record_diff: Option<RecordDiffCursor>,
    pub(super) assembly: ui_contract::UiDocumentAssembly,
    pub(super) assembly_open: bool,
    pub(super) assembly_surface: Option<ui_contract::SurfaceId>,
    pub(super) assembly_credit: Option<ui_contract::UiResidentPermit>,
    pub(super) assembly_generation: u64,
    pub(super) new_ordinals: SurfaceLinearMap<ui_contract::UiNodeId, usize, SURFACE_RECONCILE_FIXED_NODES>,
    pub(super) new_key_index: SurfaceLinearMap<NodeIdentity, ui_contract::UiNodeId, SURFACE_RECONCILE_FIXED_NODES>,
    pub(super) remove_next: Option<ui_contract::UiNodeId>,
    pub(super) removal: SurfaceFixedVec<RemovalFrame, SURFACE_RECONCILE_FIXED_NODES>,
    pub(super) ops: ui_contract::UiPatchOps,
    pub(super) pending_op: ui_contract::UiPendingPatchOp,
    pub(super) root_patch_prepared: bool,
    pub(super) limits: SurfaceReconcileLimits,
    pub(super) usage: SurfaceReconcileUsage,
    pub(super) held_node: Option<(Option<usize>, TreeNode)>,
    pub(super) semantic_census: Option<SurfaceSemanticCensusCursor>,
    pub(super) semantic_usage: SurfaceSemanticUsage,
    pub(super) fault: Option<SurfaceReconcileFault>,
    pub(super) retire_tree: SurfaceTreeRetireCursor,
    pub(super) retire_fresh_field: u8,
    pub(super) retire_record_field: u8,
    #[cfg(any(test, feature = "testkit"))]
    pub(super) fail_binding_step: bool,
    #[cfg(any(test, feature = "testkit"))]
    pub(super) fail_component_step: bool,
}

impl SurfaceReconcileCursor {
    #[cfg(any(test, feature = "testkit"))]
    pub fn new(tree: ComponentTree, current: &SurfaceShadow) -> Self {
        Self::new_with_limits(tree, current, SurfaceReconcileLimits::default())
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn new_with_limits(tree: ComponentTree, current: &SurfaceShadow, limits: SurfaceReconcileLimits) -> Self {
        let credit = surface_reconcile_limits_admissible(limits).then(|| reserve_surface_reconcile_floor(limits)).flatten();
        Self::new_admitted(tree, current, limits, current.revision.0.checked_add(1).unwrap_or(0), credit)
    }

    pub fn new_admitted(tree: ComponentTree, current: &SurfaceShadow, limits: SurfaceReconcileLimits, generation: u64, credit: Option<ui_contract::UiResidentPermit>) -> Self {
        Self {
            stage: SurfaceReconcileStage::TraversePresentation,
            surface: current.surface.clone(),
            base_revision: current.revision,
            allocator: current.allocator.clone(),
            old_root: current.root,
            pending_root: Some(tree.root),
            native_resources: tree.native_resources,
            traversal: SurfaceFixedVec::default(),
            overflow_frame: None,
            flat: SurfaceFixedVec::default(),
            postorder: SurfaceFixedVec::default(),
            seen: SurfaceLinearSet::default(),
            ids: SurfaceFixedVec::default(),
            allocate_index: 0,
            diff_index: 0,
            record_diff: None,
            assembly: ui_contract::UiDocumentAssembly::default(),
            assembly_open: false,
            assembly_surface: Some(current.surface.clone()),
            assembly_credit: credit,
            assembly_generation: generation,
            new_ordinals: SurfaceLinearMap::default(),
            new_key_index: SurfaceLinearMap::default(),
            remove_next: None,
            removal: SurfaceFixedVec::default(),
            ops: ui_contract::UiPatchOps::default(),
            pending_op: ui_contract::UiPendingPatchOp::default(),
            root_patch_prepared: false,
            limits,
            usage: SurfaceReconcileUsage { nodes: 0, items: 1, bytes: 0 },
            held_node: None,
            semantic_census: None,
            semantic_usage: SurfaceSemanticUsage::default(),
            fault: None,
            retire_tree: SurfaceTreeRetireCursor::default(),
            retire_fresh_field: 0,
            retire_record_field: 0,
            #[cfg(any(test, feature = "testkit"))]
            fail_binding_step: false,
            #[cfg(any(test, feature = "testkit"))]
            fail_component_step: false,
        }
    }

    pub fn usage(&self) -> SurfaceReconcileUsage {
        self.usage
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn assembly_generation(&self) -> u64 {
        self.assembly_generation
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn pending_root(&self) -> Option<&TreeNode> {
        self.pending_root.as_ref()
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn held_node(&self) -> Option<&(Option<usize>, TreeNode)> {
        self.held_node.as_ref()
    }

}
//#endregion ⏭️ResumableReconcile
