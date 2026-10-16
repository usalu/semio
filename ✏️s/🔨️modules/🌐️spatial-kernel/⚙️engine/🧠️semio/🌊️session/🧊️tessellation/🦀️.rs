//! 🧊️ Tessellation: the retained job registry, mesh cache and authority claims, and the failed-envelope codec.

use super::*;

// #region 🔖️Tessellation
/// 🧹️ Retains only geometry handles referenced by the current evaluation outputs.

/// 🩺️ One blocking finding from the pre-tessellation validate gate, in the typed shape the preview
/// status object carries — never a re-parsed prose string.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PreviewDiagnostic {
    pub entity: String,
    pub code: String,
    pub message: String,
}

/// ⏱️ What one budgeted [`tessellate_step`] call achieved.
#[derive(Clone, Debug, PartialEq)]
pub enum TessellationStepOutcome {
    /// 🔁 Budget spent, work remains — step again next turn.
    Working { units_done: usize, units_total: usize, faces_done: usize, faces_total: usize, phase: &'static str },
    /// ✅ The mesh is ready (freshly tessellated, or served from the LOD cache).
    Ready { mesh: semio_framework::MeshData, units_total: usize, faces_total: usize },
    /// 🛑 A cancel retired the job; nothing is produced and the slot is free.
    Cancelled,
    /// 🩺️ The validate gate rejected the topology before any triangle was produced.
    Invalid { issues: Vec<PreviewDiagnostic> },
    /// 💥 The kernel refused the handle or a unit faulted.
    Failed { message: String },
}

/// ⏱️ Retained resumable tessellations keyed by `(handle, tolerance bits)`. Bounded: a new job past
/// the ceiling evicts the least recently stepped one rather than growing without limit.
pub(super) const TESSELLATION_JOB_CAPACITY: usize = 32;
pub(super) type TessellationJobKey=(String,u64,u64,u64);

pub(super) struct RetainedTessellation {
    pub(super) identity:semio_framework_os_flow::geometry::GeometryWorkIdentity,
    pub(super) source:usize,
    pub(super) length:usize,
    pub(super) source_units:usize,
    pub(super) geometry:Option<GeometryHandle>,
    pub(super) admission_claim:String,
    pub(super) validation_units:usize,
    pub(super) validation:Option<semio_framework_3d_brep_validation::BodyValidationJob>,
    pub(super) validated:bool,
    pub(super) job:TessellationJob,
    pub(super) last_step:u64,
    pub(super) retired:bool,
    pub(super) output:Option<semio_framework_3d_brep_representation::contract::MeshTransfer>,
    pub(super) previous_mesh:Option<CachedMesh>,
}
semio_framework_value::artifact_retire_struct!(RetainedTessellation {identity,source,length,source_units,geometry,admission_claim,validation_units,validation,validated,job,last_step,retired,output,previous_mesh});
#[derive(Default)]
pub(super) struct TessellationJobRegistry {
    pub(super) admission:Option<TessellationAdmission>,
    pub(super) admission_closing:Option<ControlledRetirement<TessellationAdmission>>,
    pub(super) source_cancellation:Option<tessellation_admission::TessellationSourceCancellation>,
    pub(super) jobs:HistoryFoldIndex<TessellationJobKey,RetainedTessellation>,
    pub(super) clock:u64,
    pub(super) retire_slot:usize,
    pub(super) closing:Option<ControlledRetirement<(TessellationJobKey,RetainedTessellation)>>,
    pub(super) cancellation:Option<TessellationCancellation>,
}
#[derive(Clone,Copy)]
pub(super) struct TessellationCancellation {pub(super) slot:usize}
semio_framework_value::artifact_retire_leaf!(TessellationCancellation);
semio_framework_value::artifact_retire_struct!(TessellationJobRegistry {admission,admission_closing,source_cancellation,jobs,clock,retire_slot,closing,cancellation});
pub(super) struct CachedMesh {pub(super) mesh:semio_framework::MeshData,pub(super) valid:bool}
semio_framework_value::artifact_retire_struct!(CachedMesh {mesh,valid});
#[derive(Default)]
pub(super) struct AuthorityClaims {pub(super) handles:HistoryFoldIndex<String,bool>,pub(super) active:bool}
impl AuthorityClaims {
    pub(super) fn open()->Self {Self {active:true,..Self::default()}}
    pub(super) fn insert(&mut self,handle:String) {self.handles.insert(handle,true);}
    pub(super) fn contains(&self,handle:&str)->bool {self.active && self.handles.get(handle)==Some(&true)}
    pub(super) fn remove(&mut self,handle:&str) {if let Some(live)=self.handles.get_mut(handle) {*live=false;}}
    pub(super) fn retain(&mut self,handles:&[String]) {for (handle,live) in self.handles.slot_entries_mut() {*live=handles.iter().any(|item|item==handle);}for handle in handles {if self.handles.get(handle)==Some(&true) {continue;}self.insert(handle.clone());}}
    pub(super) fn extend(&mut self,handles:impl IntoIterator<Item=String>) {for handle in handles {self.insert(handle);}}
    pub(super) fn iter(&self)->impl Iterator<Item=&String> {self.handles.iter().filter(|(_,live)|self.active && **live).map(|(handle,_)|handle)}
}
impl FromIterator<String> for AuthorityClaims {fn from_iter<T:IntoIterator<Item=String>>(handles:T)->Self {let mut claims=Self::open();claims.extend(handles);claims}}
semio_framework_value::artifact_retire_struct!(AuthorityClaims {handles,active});
pub(super) type MeshCache=HistoryFoldIndex<(String,u64),CachedMesh>;
pub(super) type FamilyClaims=HistoryFoldIndex<u64,AuthorityClaims>;
pub(super) type FamilyRetirement=ControlledRetirement<(Brep,MeshCache,FamilyClaims)>;



/// 🧹️ Drops retained tessellations whose handle is no longer live. An empty `live` clears them all.

/// 🛑️ Retires the in-flight tessellation of `handle` at `tolerance`. Returns true when a job was
/// actually retired — a cancel for an already-finished or never-started job is a no-op, not a fault.

/// 🛑️ Retires every in-flight tessellation — the explicit "cancel preview evaluation" gesture.

/// 📈️ Progress of the in-flight tessellation of `handle` at `tolerance`, if one is retained.

/// 🎚️ The cached mesh for `handle` at `tolerance` OR at any FINER tolerance already computed — a
/// finer mesh is a valid, better-than-requested answer, so switching the LOD mode from fine to
/// coarse serves the existing mesh instead of retessellating.

/// ⏱️ Advances the tessellation of `handle` at `tolerance` by at most `budget` units and reports
/// what happened. This is the ONLY tessellation entry point that respects an interactive step
/// ceiling: it never runs more than `budget` face/edge units before returning, so a host can drive
/// it from inside its own maintenance budget, paint progress, and cancel it.
///
/// The validate gate runs once, on first admission of a `(handle, tolerance)` pair — a solid with a
/// blocking (`code` not prefixed `warning-`) validation issue never reaches the tessellator.

/// 🧊️ Tessellates a geometry handle owned by the in-process brep kernel into preview `MeshData`.
/// Unbudgeted convenience over [`tessellate_step`] for callers with no interactive ceiling (export
/// bridges, schema tests) — the incremental path is still the only algorithm underneath.

/// 🌐️ One budgeted tessellate ROUND TRIP as the extension-boundary JSON envelope: progress/phase
/// always, plus one base64 `pack` mesh-body chunk per continuation once the mesh is ready. `chunk`
/// selects which chunk to ship; `chunks` tells the caller how many there are in total.
///
/// ⏱️ `budget` bounds ONE step in face/edge units — the granularity at which a cancel can land —
/// and `wall_micros` bounds the whole round trip in wall time: the call keeps stepping while the
/// job is still working AND the deadline has not passed. Both bounds are needed because units are
/// not time: the cheapest step measured on this kernel is 29 µs and the dearest 5.9 s, so a unit
/// budget alone either wastes a whole interactive round trip on microseconds of work or overruns
/// any ceiling on a single face (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).

// 🚫️async: E1 pure codec helper (no I/O), consumed from sync envelope call sites — see R9
fn failed_envelope_json(code: &str, message: &str) -> String {
    use semio_framework_pack_json::{object, Value};
    semio_framework_pack_json::to_string(&object([
        ("done".to_string(), Value::Bool(true)),
        ("cancellable".to_string(), Value::Bool(false)),
        ("phase".to_string(), Value::String("failed".to_string())),
        ("error".to_string(), Value::String(message.to_string())),
        ("errorCode".to_string(), Value::String(code.to_string())),
    ]))
}


// #endregion 🔖️Tessellation
