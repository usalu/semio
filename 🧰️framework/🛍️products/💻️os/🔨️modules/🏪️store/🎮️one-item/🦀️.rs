//! 🎮️ One-item publication admission vocabulary: the fixed one-item ceilings, the exact footprint a lane declares
//! before preparation, the five-currency scheduler grant, the replay checkpoint and the step every retained one-item
//! owner reports, plus the declared-rows law that proves an aggregate's leaves against that ceiling. It names protocol,
//! version-graph and retained-value vocabulary only, never the Store engine, so every facet that prepares one item
//! (canonical edit, history, composition, the Store) depends on it in one direction.
//!
//! See `📓️2026-10-13-wave-e1-os-kernel.md` (ticket BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT) for the layering.

use protocol::Mutation;
use semio_framework_os_vcs::VcsError;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_value::ValueError;

pub const ARTIFACT_STORE_ONE_ITEM_MAXIMUM_WORK_ITEMS: usize = 65_536;
pub const ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES: usize = 1_048_576;
pub const ARTIFACT_STORE_ONE_ITEM_MAXIMUM_RETRIES: u8 = 2;
pub const ARTIFACT_STORE_ONE_ITEM_ID_BYTES: usize = 256;

/// 🧮 Exact capacity declaration checked before a retained one-item preparation receives
/// the immutable base root or mutation owner.
///
/// `work_items` counts **staged edit ROWS**, never mutations: one durable item always contributes
/// its single `forwards` row PLUS every row its `inverse` yields, and
/// `ArtifactStore::fold_batch_item` / the `PreflightingCommit` gate both compare
/// `forwards.len() + inverse.len()` against the gesture-wide merged declaration. A point-invertible
/// mutation therefore costs **2**, not 1 — declaring 1 fail-closes EVERY single-mutation durable
/// gesture with `batched item candidate failed its exact fixed fold contract` and every multi-item
/// one with `batched prepared candidate failed its exact fixed commit contract`. Never write the
/// number at a call site: a durable lane declares through [`ArtifactStoreOneItemFootprint::for_leaf`], whose inverse rows
/// the leaf payload schema declares (`x-semio-inverse-rows`, design §20.5), and an ephemeral lane through
/// [`ArtifactStoreOneItemFootprint::for_ephemeral_item`] (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ArtifactStoreOneItemFootprint {
    pub work_items: usize,
    pub retained_bytes: usize,
}

/// 🧾️ Staged edit rows ONE point-invertible durable item folds: its single forward row plus the
/// single inverse row `Mutation::inverse` yields for it.
pub const ARTIFACT_STORE_ONE_ITEM_INVERTIBLE_WORK_ITEMS: usize = 2;

impl ArtifactStoreOneItemFootprint {
    pub fn is_admissible(self) -> bool {
        self.work_items != 0 && self.work_items <= ARTIFACT_STORE_ONE_ITEM_MAXIMUM_WORK_ITEMS && self.retained_bytes <= ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES
    }

    /// 🧺️ The ONE declaration a durable lane makes for `mutation`: its forward row plus the inverse rows its leaf payload
    /// schema declares — the upper bound the lane can prove from the mutation alone, since `preflight` never sees the base.
    pub fn for_leaf<P, M: Mutation<P>>(mutation: &M, retained_bytes: usize) -> Self {
        Self { work_items: mutation.inverse_rows().saturating_add(1), retained_bytes }
    }

    /// 🐘️ The rows a whole durable gesture declares — every mutation's [`Self::for_leaf`] rows merged — refused as
    /// [`VcsError::TooLarge`] (`mutation.too-large`, a framework notice) above [`ARTIFACT_STORE_ONE_ITEM_MAXIMUM_WORK_ITEMS`],
    /// before any preparation, so the runtime tells the person instead of the batch admission failing closed.
    pub fn for_gesture<P, M: Mutation<P>>(mutations: &[M]) -> Result<Self, VcsError> {
        let rows = mutations.iter().fold(0usize, |rows, mutation| rows.saturating_add(Self::for_leaf::<P, M>(mutation, 0).work_items));
        if rows > ARTIFACT_STORE_ONE_ITEM_MAXIMUM_WORK_ITEMS {
            return Err(VcsError::TooLarge { rows, capacity: ARTIFACT_STORE_ONE_ITEM_MAXIMUM_WORK_ITEMS });
        }
        Ok(Self { work_items: rows, retained_bytes: 0 })
    }

    /// 🔁️ The declaration of one whole-record replacement: its forward row plus the single inverse row that restores the previous record by construction.
    pub fn for_point_invertible_item(retained_bytes: usize) -> Self {
        Self { work_items: ARTIFACT_STORE_ONE_ITEM_INVERTIBLE_WORK_ITEMS, retained_bytes }
    }

    /// 🫧️ The declaration of one ephemeral (presence, transient, window-transient) item: its single forward row, which no
    /// inverse ever accompanies.
    pub fn for_ephemeral_item(retained_bytes: usize) -> Self {
        Self { work_items: 1, retained_bytes }
    }

    /// ➕️ Folds one more admitted item into the gesture-wide declaration a batched publication is
    /// admitted against.
    pub fn merged(self, item: Self) -> Self {
        Self { work_items: self.work_items.saturating_add(item.work_items), retained_bytes: self.retained_bytes.max(item.retained_bytes) }
    }
}

/// 🎟️ One scheduler grant. A preparation owner must consume at most one semantic unit even
/// when both limits are larger than one.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ArtifactStoreOneItemGrant {
    pub maximum_items: usize,
    pub maximum_copy_bytes: usize,
    pub maximum_capacity_bytes: usize,
    pub maximum_release_bytes: usize,
    pub maximum_depth: usize,
}

impl ArtifactStoreOneItemGrant {
    pub fn permits_one(self) -> bool {
        self.maximum_items != 0 && self.maximum_depth != 0
    }
    pub fn retained_grant(self) -> RetainedCloneGrant {
        RetainedCloneGrant {
            maximum_items: self.maximum_items,
            maximum_copy_bytes: self.maximum_copy_bytes,
            maximum_capacity_bytes: self.maximum_capacity_bytes,
            maximum_release_bytes: self.maximum_release_bytes,
            maximum_depth: self.maximum_depth,
        }
    }
    /// 🔁️ Preserves all five currencies of the caller's original independent authority.
    pub fn from_retained(original:RetainedCloneGrant)->Self {
        Self{maximum_items:original.maximum_items,maximum_copy_bytes:original.maximum_copy_bytes,maximum_capacity_bytes:original.maximum_capacity_bytes,maximum_release_bytes:original.maximum_release_bytes,maximum_depth:original.maximum_depth}
    }
}

/// 📍 Fixed-width replay checkpoint emitted after every accepted preparation unit.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ArtifactStoreOneItemCheckpoint {
    pub cursor: u32,
    pub completed_items: u32,
    pub completed_bytes: u64,
    pub digest: [u8; 32],
}

/// 🛠️ Result of one domain-owned preparation turn.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactStoreOneItemPreparationStep {
    Progress(ArtifactStoreOneItemCheckpoint, RetainedCloneProgress),
    Prepared(ArtifactStoreOneItemCheckpoint, RetainedCloneProgress),
    Blocked,
}

impl ArtifactStoreOneItemPreparationStep {
    /// 🧾️ The genuine consumed currencies survive independently of the displayed checkpoint.
    pub fn ownership_progress(&self) -> RetainedCloneProgress {
        match self {Self::Progress(_,progress)|Self::Prepared(_,progress)=>*progress,Self::Blocked=>Default::default()}
    }
}

/// 🎟️ The Store validates the original child's exact receipt and prepared-root witness.
pub fn validate_original_preparation_step(step:ArtifactStoreOneItemPreparationStep,grant:ArtifactStoreOneItemGrant,prepared:bool)->Result<ArtifactStoreOneItemPreparationStep,ValueError>{
    let progress=step.ownership_progress();
    if !progress.fits(grant.retained_grant())||grant.maximum_depth==0&&progress!=RetainedCloneProgress::default(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"preparation exceeded its original ownership grant").with_retained_progress(progress))}
    if matches!(step,ArtifactStoreOneItemPreparationStep::Prepared(_, _))&&!prepared{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"preparation reported Prepared without its original root").with_retained_progress(progress))}
    Ok(step)
}

/// 🐘️ The declared-rows law of an aggregate (design §20.5, audit CLOSURE-4): every editable operation answers exactly the inverse
/// rows its leaf payload schema's `x-semio-inverse-rows` declares for its own payload — so a hand-written aggregate that forwards
/// leaf schemas but answers the default single row fails — and every `perTarget` leaf, grown along its first target field, is
/// admitted by [`ArtifactStoreOneItemFootprint::for_gesture`] at the one-item ceiling and refused one target past it as
/// `mutation.too-large` (a framework notice). One line per breach.
pub fn mutation_inverse_rows_declaration_failures<P, M: Mutation<P>>(ops: &[M]) -> Vec<String> {
    let ceiling = ARTIFACT_STORE_ONE_ITEM_MAXIMUM_WORK_ITEMS;
    let mut failures = Vec::new();
    let mut grown = std::collections::BTreeSet::new();
    for (index, op) in ops.iter().enumerate() {
        let Some(schema) = op.input_schema() else { continue };
        let kind = op.descriptor().semantic_kind;
        let rows = match semio_framework_pack_json::parse(schema, semio_framework_pack_json::JsonMemberPolicy::Reject) {
            Ok(json) => semio_framework_pack_json::to_dsl_value(&json).get("x-semio-inverse-rows").cloned(),
            Err(_) => {
                failures.push(format!("op {index} ({kind}): its payload schema is no JSON"));
                continue;
            }
        };
        let count = |key: &str| rows.as_ref().and_then(|rows| rows.get(key)).and_then(semio_framework_value::DslValue::as_u64).map(|count| count as usize);
        let fixed = if rows.is_none() { 1 } else { count("bounded").or_else(|| count("fixed")).unwrap_or(0) };
        let per_target: Vec<(String, usize)> = rows.as_ref().and_then(|rows| rows.get("perTarget")).and_then(semio_framework_value::DslValue::as_object).map(|fields| fields.iter().map(|(field, rows)| (field.clone(), rows.as_u64().unwrap_or(0) as usize)).collect()).unwrap_or_default();
        let payload = op.payload_value();
        let length = |field: &str| payload.get(field).and_then(semio_framework_value::DslValue::as_array).map_or(0, <[semio_framework_value::DslValue]>::len);
        let declared = fixed + per_target.iter().map(|(field, rows)| rows * length(field)).sum::<usize>();
        if op.inverse_rows() != declared {
            failures.push(format!("op {index} ({kind}): answers {} inverse row(s) where its leaf schema declares {declared}", op.inverse_rows()));
            continue;
        }
        let Some((field, per)) = per_target.first().cloned() else { continue };
        let items = payload.get(&field).and_then(semio_framework_value::DslValue::as_array).map(<[semio_framework_value::DslValue]>::to_vec).unwrap_or_default();
        let base = declared - per * items.len();
        if items.is_empty() || per == 0 || base + 1 > ceiling || grown.contains(kind) {
            continue;
        }
        grown.insert(kind);
        let admitted = (ceiling - 1 - base) / per;
        let with_targets = |targets: usize| {
            let entries = payload.as_object().unwrap_or_default().iter().map(|(key, value)| (key.clone(), if *key == field { semio_framework_value::DslValue::Array(items.iter().cycle().take(targets).cloned().collect()) } else { value.clone() }));
            op.with_payload_value(semio_framework_value::DslValue::object(entries))
        };
        match (with_targets(admitted), with_targets(admitted + 1)) {
            (Ok(at), Ok(past)) => {
                if at.inverse_rows() != base + per * admitted || ArtifactStoreOneItemFootprint::for_gesture::<P, M>(std::slice::from_ref(&at)).is_err() {
                    failures.push(format!("op {index} ({kind}): {admitted} {field} at the {ceiling}-row ceiling are refused"));
                }
                match ArtifactStoreOneItemFootprint::for_gesture::<P, M>(std::slice::from_ref(&past)) {
                    Err(error) if semio_framework_diagnostic::FaultFrom::fault_code(&error).0 == "mutation.too-large" => {}
                    _ => failures.push(format!("op {index} ({kind}): {} {field} past the {ceiling}-row ceiling are not refused as mutation.too-large", admitted + 1)),
                }
                Mutation::<P>::retire_cold(at);
                Mutation::<P>::retire_cold(past);
            }
            (at, past) => {
                failures.push(format!("op {index} ({kind}): a {field} array grown to the ceiling does not rebuild from its payload"));
                for op in [at, past].into_iter().flatten() {
                    Mutation::<P>::retire_cold(op);
                }
            }
        }
    }
    failures
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue, serde::Serialize, serde::Deserialize)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub struct LaneItemReceipt {
    pub generation_before: u64,
    pub generation_after: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub enum ArtifactStoreOneItemPublicationPhase {
    Preparing,
    PreparingCursor,
    PreflightingCommit,
    Publishing,
    AwaitingAck,
    Closing,
    Complete,
    Fault,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactStoreOneItemAdvance {
    Progress(ArtifactStoreOneItemCheckpoint),
    PreparationProgress(ArtifactStoreOneItemCheckpoint, RetainedCloneProgress),
    PreparationRefused,
    Published(LaneItemReceipt, RetainedCloneProgress),
    AwaitingAck(LaneItemReceipt),
    Complete,
    Blocked,
}

#[path="🧬️semantic/🦀️.rs"]
pub mod semantic;
pub use semantic::{PagedOneItemEdit,PagedOneItemEditStep,OriginalPostRead,PagedOneItemPostMode,PagedOneItemPostRead,PagedOneItemPost};
