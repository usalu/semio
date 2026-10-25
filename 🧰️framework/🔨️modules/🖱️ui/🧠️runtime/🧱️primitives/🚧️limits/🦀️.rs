//! 🚧️ Admission bounds of one reconciliation — the limits a reconcile may spend, the usage it has observed, the typed faults that end it, and the page and work-byte constants every step is priced in.

use std::mem::size_of;
use crate::SURFACE_RECONCILE_FIXED_NODES;

//#region ⏭️ResumableReconcile

/// 🔏️ Fixed admission bounds for one retained reconciliation authority.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SurfaceReconcileLimits {
    pub max_nodes: usize,
    pub max_items: usize,
    pub max_bytes: usize,
    pub max_identifier_bytes: usize,
}

/// 🛑️ The item ceiling [`SurfaceReconcileLimits::default`] admits for one surface — the bound a producer that
/// windows its rows (the plugin SDK's `TreeWindows` ledger) stays under by charging [`surface_subtree_items`].
pub const SURFACE_RECONCILE_MAX_ITEMS: usize = 4_097;

impl Default for SurfaceReconcileLimits {
    fn default() -> Self {
        Self { max_nodes: SURFACE_RECONCILE_FIXED_NODES, max_items: SURFACE_RECONCILE_MAX_ITEMS, max_bytes: SURFACE_RECONCILE_SURFACE_BYTES, max_identifier_bytes: 256 }
    }
}

/// 📏️ Credits observed while advancing one retained reconciliation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SurfaceReconcileUsage {
    pub nodes: usize,
    pub items: usize,
    pub bytes: usize,
}

impl SurfaceReconcileUsage {
    pub fn include(&mut self, nodes: usize, items: usize, bytes: usize) -> bool {
        let Some(next_nodes) = self.nodes.checked_add(nodes) else { return false };
        let Some(next_items) = self.items.checked_add(items) else { return false };
        let Some(next_bytes) = self.bytes.checked_add(bytes) else { return false };
        self.nodes = next_nodes;
        self.items = next_items;
        self.bytes = next_bytes;
        true
    }

    pub fn fits(self, limits: SurfaceReconcileLimits) -> bool {
        self.nodes <= limits.max_nodes && self.items <= limits.max_items && self.bytes <= limits.max_bytes
    }
}

/// 🚫️ Typed terminal reason retaining the exact in-progress authority.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SurfaceReconcileFault {
    PublicationAuthority(&'static str),
    AliasCapacity,
    CounterOverflow,
    DuplicateSiblingKey,
    IdentifierBytes { actual: usize, max: usize },
    Credits { usage: SurfaceReconcileUsage, limits: SurfaceReconcileLimits },
    ResidentCredit { required: usize, aggregate: usize },
    PageBytes { actual: usize, max: usize },
    ValueDepth { actual: usize, max: usize },
    StaleGeneration { expected: u64, actual: u64 },
    Cancelled,
}

pub const SURFACE_RECONCILE_VALUE_DEPTH: usize = 64;
pub const SURFACE_RECONCILE_SEMANTIC_COPIES: usize = 3;

#[cfg(any(test, feature = "testkit"))]
pub fn admit_vec_backing<T>(owner: &mut Vec<T>, usage: &mut SurfaceReconcileUsage, limits: SurfaceReconcileLimits) -> Result<(), SurfaceReconcileFault> {
    if owner.len() < owner.capacity() {
        return Ok(());
    }
    let before = owner.capacity();
    if owner.try_reserve_exact(1).is_err() {
        return Err(SurfaceReconcileFault::Credits { usage: SurfaceReconcileUsage { bytes: limits.max_bytes.checked_add(1).unwrap_or(usize::MAX), ..*usage }, limits });
    }
    let slots = owner.capacity().checked_sub(before).ok_or(SurfaceReconcileFault::CounterOverflow)?;
    let mut projected = *usage;
    let bytes = slots.checked_mul(size_of::<T>()).ok_or(SurfaceReconcileFault::CounterOverflow)?;
    if !projected.include(0, slots, bytes) || !projected.fits(limits) {
        return Err(SurfaceReconcileFault::Credits { usage: projected, limits });
    }
    *usage = projected;
    Ok(())
}

//#region 🎟️RetainedAuthority

pub const SURFACE_RECONCILE_PAGE_BYTES: usize = 32 * 1_024;
pub const SURFACE_COMPONENT_COPY_WORK_BYTES: usize = if size_of::<ui_contract::Component>() > 4096 { size_of::<ui_contract::Component>() } else { 4096 };
const _: () = assert!(size_of::<ui_contract::Component>() <= SURFACE_COMPONENT_COPY_WORK_BYTES);
const _: () = assert!(size_of::<ui_contract::UiComponentCopy>() <= SURFACE_RECONCILE_PAGE_BYTES);
const _: () = assert!(2 * size_of::<ui_contract::Component>() + SURFACE_COMPONENT_COPY_WORK_BYTES <= SURFACE_RECONCILE_PAGE_BYTES);
const _: () = assert!(size_of::<ui_contract::UiPatchOp>() <= SURFACE_RECONCILE_PAGE_BYTES);
const _: () = assert!(size_of::<ui_contract::UiBindingsCopy>() <= SURFACE_RECONCILE_PAGE_BYTES);
pub const SURFACE_RECONCILE_SURFACE_BYTES: usize = ui_contract::UI_RESIDENT_SURFACE_BYTES;

/// 🎟️ What one reconcile reservation costs BEFORE its tree has been measured: the fixed assembly root plus
/// four work pages, which covers the largest charge any single reconcile step can make (`PageBytes` faults
/// anything wider than one page) with room for the fixed census structures the seal prices in.
///
/// 🐛️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B58: the reservation used to ask for the per-surface
/// CEILING [`SURFACE_RECONCILE_SURFACE_BYTES`] (8 MiB) of a 32 MiB aggregate, so exactly THREE surfaces
/// could reconcile process-wide while one puzzle3d session mounts THIRTEEN. The ceiling is a maximum a
/// surface may reach, never a price.
pub const SURFACE_RECONCILE_FLOOR_BYTES: usize = ui_contract::UiDocumentAssembly::required_open_bytes() + 4 * SURFACE_RECONCILE_PAGE_BYTES;
pub const SURFACE_RECONCILE_FLOOR_ITEMS: usize = 256;
const _: () = assert!(SURFACE_RECONCILE_FLOOR_BYTES <= SURFACE_RECONCILE_SURFACE_BYTES);
const _: () = assert!(SURFACE_RECONCILE_FLOOR_ITEMS <= ui_contract::UI_RESIDENT_SURFACE_ITEMS);

pub fn surface_reconcile_limits_admissible(limits: SurfaceReconcileLimits) -> bool {
    limits.max_nodes <= SurfaceReconcileLimits::default().max_nodes
        && limits.max_items <= SurfaceReconcileLimits::default().max_items
        && limits.max_bytes <= SurfaceReconcileLimits::default().max_bytes
        && limits.max_identifier_bytes <= SurfaceReconcileLimits::default().max_identifier_bytes
}

pub fn reserve_surface_reconcile_floor(limits: SurfaceReconcileLimits) -> Option<ui_contract::UiResidentPermit> {
    let mut permit = None;
    let floor = ui_contract::UiResidentLimits { items: SURFACE_RECONCILE_FLOOR_ITEMS.min(limits.max_items), bytes: SURFACE_RECONCILE_FLOOR_BYTES.min(limits.max_bytes) };
    ui_contract::UiResidentPermit::try_reserve(floor, &mut permit, SURFACE_RECONCILE_PAGE_BYTES).ok()?;
    permit
}

pub fn release_surface_reconcile(mut credit: ui_contract::UiResidentPermit) {
    if let Err(error) = credit.close_step(1) {
        if !std::thread::panicking() { panic!("{}", error.reason()); }
    }
}

pub fn release_surface_reconcile_one(owner: &mut Option<ui_contract::UiResidentPermit>) -> Result<ui_contract::UiValueRetirementStep, &'static str> {
    let Some(credit) = owner.as_mut() else { return Ok(ui_contract::UiValueRetirementStep { complete: true, ..Default::default() }); };
    let step = credit.close_step(1).map_err(|error| error.reason())?;
    if step.complete { owner.take(); }
    Ok(ui_contract::UiValueRetirementStep { complete: step.complete, progressed: step.progressed, released_items: step.released_permits, released_bytes: 0 })
}

//#endregion 🎟️RetainedAuthority

//#endregion ⏭️ResumableReconcile
