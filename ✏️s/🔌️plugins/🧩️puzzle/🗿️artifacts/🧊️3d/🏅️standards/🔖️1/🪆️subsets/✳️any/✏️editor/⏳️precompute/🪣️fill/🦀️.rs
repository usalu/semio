//! 🪣️ Puzzle 3d play app — the precompute fill planner's own state: the running `FillBuilder` (base
//! scene, the growing plan sequence and its appended objects/attractions, the placed collision
//! entries the next step tests against, the per-session RNG stream) plus its progress readout. The
//! stepping itself lives in the sibling `⏳️precompute/🦀️.rs`, which owns the two precompute
//! lanes. Rehomed from the former `⚙️engine/🪣️fill` (ticket
//! 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): this is interactive fill-tool session state,
//! so it lives with the app, not the artifact.

use crate::editor::puzzle3d::precompute::brush::{
    brush_fill_candidate_at, brush_object_id, brush_preview_from_candidate, brush_stack_mate_pair, fill_candidate_diversity_score, fill_rng, resolve_object_kind_mesh_url, vortex_world_from_object, AttractionVortexContext, BrushCatalogView,
    BrushFillVortexTarget, BrushFixtureView, TargetVortexWorld,
};
use crate::editor::puzzle3d::precompute::geometry::{
    pose_isometry, world_bounds, world_volumes_contain_aabb, CollisionAabb, CollisionBody, CollisionIndexMutation, CollisionIndexOwner, CollisionIndexOwnerCensusCursor, CollisionIndexOwnerCensusStep, CollisionIndexRejectedOwner,
    CollisionIndexRemoval, CollisionMutationStep, CollisionOverlapState, CollisionQueryCursor, CollisionQueryStep, CollisionSpatialIndex, CollisionStepResult, FixedOwnerMap, FixedOwnerMapInsert, FixedOwnerSet, FixedOwnerSetInsert, FixedOwnerVec,
    Pose3d, DOCUMENT_ATTRACTION_SLOTS, DOCUMENT_CANDIDATE_SLOTS, DOCUMENT_KIND_SLOTS, DOCUMENT_OBJECT_SLOTS, DOCUMENT_OWNER_PAGE_BYTES, DOCUMENT_VOLUME_SLOTS, DOCUMENT_VORTEX_SLOTS,
};
use crate::editor::puzzle3d::precompute::FILL_COUNT_MAX;
use crate::standards::v1::subsets::any::schema::{
    puzzle3d_vortex_full_id, AttractionProps, BrushCompatibleCandidate, BrushHostRules, BrushPlacePayload, BrushPreviewState, CableKindCatalog, FillBuildPreview, FillBuildProgress, Fixture, FixtureObject, KindCompatEntry, ObjectKind, SceneConfig,
    VortexKindCatalog, VortexProps, WorldVolumeProps,
};
use semio_framework_job::{CommitCandidate, InteractiveJob, JobFault, Operation, StepContext, StepOutcome};
use std::collections::HashMap;
use std::sync::Arc;

//#region 🔭️RetainedPreviewJson
pub(crate) const FILL_PREVIEW_JSON_MAX_BYTES: usize = 4 * 1024;
pub(crate) const FILL_PREVIEW_JSON_MAX_COLOR_BYTES: usize = 128;
pub(crate) const FILL_PREVIEW_JSON_MAX_STATUS_LABEL_BYTES: usize = 256;
pub(crate) const FILL_PREVIEW_JSON_MAX_SOURCE_VORTEX_INDEX: u64 = 9_007_199_254_740_991;
pub(crate) const FILL_PREVIEW_JSON_MAX_DIAGNOSTIC_INTEGER: u64 = 9_007_199_254_740_991;

fn preview_json_wire_u64(value: u64, minimum: u64) -> Result<u64, ()> {
    (value >= minimum && value <= FILL_PREVIEW_JSON_MAX_DIAGNOSTIC_INTEGER).then_some(value).ok_or(())
}

fn preview_json_wire_usize(value: usize) -> Result<u64, ()> {
    preview_json_wire_u64(u64::try_from(value).map_err(|_| ())?, 0)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct FillPreviewJsonIdentity {
    operation: u64,
    base_revision: u64,
    registry_generation: u64,
    generation: u64,
    sequence: u64,
}

impl FillPreviewJsonIdentity {
    fn read(preview: &FillBuildPreview) -> Option<Self> {
        (preview_json_wire_u64(preview.operation, 1).is_ok()
            && preview_json_wire_u64(preview.base_revision, 1).is_ok()
            && preview_json_wire_u64(preview.registry_generation, 1).is_ok()
            && preview_json_wire_u64(preview.generation, 1).is_ok()
            && preview_json_wire_u64(preview.sequence, 0).is_ok())
        .then_some(Self { operation: preview.operation, base_revision: preview.base_revision, registry_generation: preview.registry_generation, generation: preview.generation, sequence: preview.sequence })
    }
}

#[derive(Clone, Copy)]
struct FillPreviewJsonSourceAuthority {
    root: u64,
    candidate_ghost: u64,
}

impl FillPreviewJsonSourceAuthority {
    fn field(value: usize) -> Result<u64, ()> {
        let value = preview_json_wire_usize(value)?;
        (value <= FILL_PREVIEW_JSON_MAX_SOURCE_VORTEX_INDEX).then_some(value).ok_or(())
    }

    fn read(preview: &FillBuildPreview) -> Result<Option<Self>, ()> {
        let Some(ghost) = preview.candidate_ghost.as_ref() else {
            return Ok(None);
        };
        Ok(Some(Self { root: Self::field(ghost.source_vortex_index)?, candidate_ghost: Self::field(ghost.source_vortex_index)? }))
    }
}

struct FillPreviewJsonDiagnosticAuthority;

impl FillPreviewJsonDiagnosticAuthority {
    fn read(preview: &FillBuildPreview) -> Result<Self, ()> {
        preview_json_wire_u64(preview.operation, 1)?;
        preview_json_wire_u64(preview.base_revision, 1)?;
        preview_json_wire_u64(preview.registry_generation, 1)?;
        preview_json_wire_u64(preview.sequence, 0)?;
        preview_json_wire_u64(preview.generation, 1)?;
        preview_json_wire_usize(preview.collision_count)?;
        preview_json_wire_usize(preview.sample_cursor)?;
        preview_json_wire_usize(preview.inside_both)?;
        preview_json_wire_usize(preview.target_cursor)?;
        preview_json_wire_usize(preview.candidate_cursor)?;
        preview_json_wire_usize(preview.accepted_count)?;
        preview_json_wire_usize(preview.total_count)?;
        preview_json_wire_u64(preview.search_count, 0)?;
        preview_json_wire_u64(preview.rejected_count, 0)?;
        Ok(Self)
    }
}

struct FillPreviewJsonAdmission;

impl FillPreviewJsonAdmission {
    fn read(preview: &FillBuildPreview, color: &str, status_label: &str) -> Result<Self, ()> {
        FillPreviewJsonSourceAuthority::read(preview)?;
        FillPreviewJsonDiagnosticAuthority::read(preview)?;
        if color.len() > FILL_PREVIEW_JSON_MAX_COLOR_BYTES || status_label.is_empty() || status_label.len() > FILL_PREVIEW_JSON_MAX_STATUS_LABEL_BYTES {
            return Err(());
        }
        if preview.candidate_ghost.as_ref().is_some_and(|ghost| ghost.origin.into_iter().chain(ghost.orientation).any(|value| !value.is_finite())) || preview.last_sample.is_some_and(|sample| sample.into_iter().any(|value| !value.is_finite())) {
            return Err(());
        }
        fill_preview_json_wire_bytes(preview, color, status_label)?;
        Ok(Self)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FillPreviewJsonPhase {
    Idle,
    RetireSuperseded,
    Census,
    Reserve,
    Encode,
    Validate,
    Ready,
    Rejected,
    Closing,
    Terminal,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum FillPreviewJsonStep {
    Pending { progress: u64, checkpoint: [u64; 6] },
    Ready,
    Rejected,
    Cancelled,
    Terminal,
}

#[derive(Clone, Copy)]
enum FillPreviewString {
    RootTarget,
    RootKind,
    RootMesh,
    Color,
    Stage,
    StatusLabel,
    Target,
    Candidate,
    GhostTarget,
    GhostKind,
    GhostMesh,
    CurrentPair,
    CandidatePage(usize),
    Rejection,
}

#[derive(Default)]
struct FillPreviewJsonPass {
    field: u8,
    subfield: u8,
    item: usize,
    string_phase: u8,
    string_byte: usize,
}

struct FillPreviewJsonUnit {
    bytes: [u8; 128],
    len: usize,
}

impl FillPreviewJsonUnit {
    fn empty() -> Self {
        Self { bytes: [0; 128], len: 0 }
    }

    fn extend(&mut self, source: &[u8]) -> Result<(), ()> {
        self.bytes.get_mut(self.len..self.len.checked_add(source.len()).ok_or(())?).ok_or(())?.copy_from_slice(source);
        self.len += source.len();
        Ok(())
    }

    fn bytes(source: &[u8]) -> Result<Self, ()> {
        let mut unit = Self::empty();
        unit.extend(source)?;
        Ok(unit)
    }

    fn formatted(arguments: std::fmt::Arguments<'_>) -> Result<Self, ()> {
        let mut unit = Self::empty();
        std::fmt::write(&mut unit, arguments).map_err(|_| ())?;
        Ok(unit)
    }

    fn escaped(byte: u8) -> Self {
        let mut unit = Self { bytes: [0; 128], len: 1 };
        match byte {
            b'"' => {
                unit.bytes[..2].copy_from_slice(b"\\\"");
                unit.len = 2;
            }
            b'\\' => {
                unit.bytes[..2].copy_from_slice(b"\\\\");
                unit.len = 2;
            }
            0x08 => {
                unit.bytes[..2].copy_from_slice(b"\\b");
                unit.len = 2;
            }
            0x0c => {
                unit.bytes[..2].copy_from_slice(b"\\f");
                unit.len = 2;
            }
            b'\n' => {
                unit.bytes[..2].copy_from_slice(b"\\n");
                unit.len = 2;
            }
            b'\r' => {
                unit.bytes[..2].copy_from_slice(b"\\r");
                unit.len = 2;
            }
            b'\t' => {
                unit.bytes[..2].copy_from_slice(b"\\t");
                unit.len = 2;
            }
            value @ 0x00..=0x1f => {
                const HEX: &[u8; 16] = b"0123456789abcdef";
                unit.bytes[..6].copy_from_slice(&[b'\\', b'u', b'0', b'0', HEX[(value >> 4) as usize], HEX[(value & 0x0f) as usize]]);
                unit.len = 6;
            }
            _ => unit.bytes[0] = byte,
        }
        unit
    }
}

impl std::fmt::Write for FillPreviewJsonUnit {
    fn write_str(&mut self, source: &str) -> std::fmt::Result {
        self.extend(source.as_bytes()).map_err(|_| std::fmt::Error)
    }
}

struct FillPreviewQuotedField {
    prefix: &'static [u8],
    source: FillPreviewString,
    optional: bool,
    advance: bool,
}

fn preview_json_float(unit: &mut FillPreviewJsonUnit, value: f64) -> Result<(), ()> {
    if !value.is_finite() {
        return Err(());
    }
    let start = unit.len;
    std::fmt::write(unit, format_args!("{value}")).map_err(|_| ())?;
    if !unit.bytes[start..unit.len].iter().any(|byte| matches!(*byte, b'.' | b'e' | b'E')) {
        unit.extend(b".0")?;
    }
    Ok(())
}

fn preview_json_vec3(prefix: &str, value: [f64; 3]) -> Result<FillPreviewJsonUnit, ()> {
    let mut unit = FillPreviewJsonUnit::empty();
    unit.extend(prefix.as_bytes())?;
    unit.extend(b"[")?;
    preview_json_float(&mut unit, value[0])?;
    unit.extend(b",")?;
    preview_json_float(&mut unit, value[1])?;
    unit.extend(b",")?;
    preview_json_float(&mut unit, value[2])?;
    unit.extend(b"]")?;
    Ok(unit)
}

fn preview_json_quat(prefix: &str, value: [f64; 4]) -> Result<FillPreviewJsonUnit, ()> {
    let mut unit = FillPreviewJsonUnit::empty();
    unit.extend(prefix.as_bytes())?;
    unit.extend(b"[")?;
    preview_json_float(&mut unit, value[0])?;
    unit.extend(b",")?;
    preview_json_float(&mut unit, value[1])?;
    unit.extend(b",")?;
    preview_json_float(&mut unit, value[2])?;
    unit.extend(b",")?;
    preview_json_float(&mut unit, value[3])?;
    unit.extend(b"]")?;
    Ok(unit)
}

impl FillPreviewJsonPass {
    fn advance_field(&mut self) {
        self.field = self.field.saturating_add(1);
        self.subfield = 0;
        self.item = 0;
        self.string_phase = 0;
        self.string_byte = 0;
    }

    fn string<'a>(&self, preview: &'a FillBuildPreview, color: &'a str, status_label: &'a str, source: FillPreviewString) -> Option<&'a str> {
        let ghost = preview.candidate_ghost.as_ref();
        match source {
            FillPreviewString::RootTarget | FillPreviewString::GhostTarget => ghost.map(|value| value.target_vortex_full_id.as_str()),
            FillPreviewString::RootKind | FillPreviewString::GhostKind => ghost.map(|value| value.object_kind_id.as_str()),
            FillPreviewString::RootMesh | FillPreviewString::GhostMesh => ghost.map(|value| value.mesh_url.as_str()),
            FillPreviewString::Color => Some(color),
            FillPreviewString::Stage => Some(preview.stage.as_str()),
            FillPreviewString::StatusLabel => Some(status_label),
            FillPreviewString::Target => preview.target_vortex_full_id.as_deref(),
            FillPreviewString::Candidate => preview.candidate_object_kind_id.as_deref(),
            FillPreviewString::CurrentPair => preview.current_pair_object_id.as_deref(),
            FillPreviewString::CandidatePage(index) => preview.candidate_page.get(index).and_then(Option::as_deref),
            FillPreviewString::Rejection => preview.rejection_reason.as_deref(),
        }
    }

    fn quoted(&mut self, preview: &FillBuildPreview, color: &str, status_label: &str, field: FillPreviewQuotedField) -> Result<FillPreviewJsonUnit, ()> {
        let FillPreviewQuotedField { prefix, source, optional, advance } = field;
        let value = self.string(preview, color, status_label, source);
        match self.string_phase {
            0 => {
                self.string_phase = 1;
                FillPreviewJsonUnit::bytes(prefix)
            }
            1 if optional && value.is_none() => {
                if advance {
                    self.advance_field();
                } else {
                    self.string_phase = 0;
                }
                FillPreviewJsonUnit::bytes(b"null")
            }
            1 => {
                self.string_phase = 2;
                FillPreviewJsonUnit::bytes(b"\"")
            }
            2 => {
                let bytes = value.ok_or(())?.as_bytes();
                if let Some(byte) = bytes.get(self.string_byte).copied() {
                    self.string_byte += 1;
                    Ok(FillPreviewJsonUnit::escaped(byte))
                } else {
                    if advance {
                        self.advance_field();
                    } else {
                        self.string_phase = 0;
                        self.string_byte = 0;
                    }
                    FillPreviewJsonUnit::bytes(b"\"")
                }
            }
            _ => Err(()),
        }
    }

    fn candidate_ghost(&mut self, preview: &FillBuildPreview, color: &str, status_label: &str) -> Result<FillPreviewJsonUnit, ()> {
        let Some(ghost) = preview.candidate_ghost.as_ref() else {
            self.advance_field();
            return FillPreviewJsonUnit::bytes(b",\"candidateGhost\":null");
        };
        let active_subfield = self.subfield;
        let unit = match active_subfield {
            0 => {
                self.subfield = 1;
                FillPreviewJsonUnit::bytes(b",\"candidateGhost\":{")
            }
            1 => self.quoted(preview, color, status_label, FillPreviewQuotedField { prefix: b"\"targetVortexFullId\":", source: FillPreviewString::GhostTarget, optional: false, advance: false }),
            2 => self.quoted(preview, color, status_label, FillPreviewQuotedField { prefix: b",\"objectKindId\":", source: FillPreviewString::GhostKind, optional: false, advance: false }),
            3 => {
                self.subfield = 4;
                let source_vortex_index = FillPreviewJsonSourceAuthority::read(preview)?.ok_or(())?.candidate_ghost;
                FillPreviewJsonUnit::formatted(format_args!(",\"sourceVortexIndex\":{source_vortex_index}"))
            }
            4 => self.quoted(preview, color, status_label, FillPreviewQuotedField { prefix: b",\"meshUrl\":", source: FillPreviewString::GhostMesh, optional: false, advance: false }),
            5 => {
                self.subfield = 6;
                preview_json_vec3(",\"origin\":", ghost.origin)
            }
            6 => {
                self.subfield = 7;
                preview_json_quat(",\"orientation\":", ghost.orientation)
            }
            _ => {
                self.advance_field();
                FillPreviewJsonUnit::bytes(b"}")
            }
        }?;
        if self.string_phase == 0 && matches!(active_subfield, 1 | 2 | 4) {
            self.subfield += 1;
        }
        Ok(unit)
    }

    fn candidate_page(&mut self, preview: &FillBuildPreview, color: &str, status_label: &str) -> Result<FillPreviewJsonUnit, ()> {
        if self.subfield == 0 {
            self.subfield = 1;
            return FillPreviewJsonUnit::bytes(b",\"candidatePage\":[");
        }
        if self.item == preview.candidate_page.len() {
            self.advance_field();
            return FillPreviewJsonUnit::bytes(b"]");
        }
        let prefix = if self.item == 0 { b"".as_slice() } else { b",".as_slice() };
        let item = self.item;
        let unit = self.quoted(preview, color, status_label, FillPreviewQuotedField { prefix, source: FillPreviewString::CandidatePage(item), optional: true, advance: false })?;
        if self.string_phase == 0 {
            self.item += 1;
        }
        Ok(unit)
    }

    fn next_unit(&mut self, preview: &FillBuildPreview, color: &str, status_label: &str) -> Result<Option<FillPreviewJsonUnit>, ()> {
        let ghost = preview.candidate_ghost.as_ref();
        let unit = match self.field {
            0 => {
                self.field = if ghost.is_some() { 1 } else { 9 };
                FillPreviewJsonUnit::bytes(b"{")?
            }
            1 => self.quoted(preview, color, status_label, FillPreviewQuotedField { prefix: b"\"targetVortexFullId\":", source: FillPreviewString::RootTarget, optional: false, advance: true })?,
            2 => self.quoted(preview, color, status_label, FillPreviewQuotedField { prefix: b",\"objectKindId\":", source: FillPreviewString::RootKind, optional: false, advance: true })?,
            3 => {
                self.advance_field();
                let source_vortex_index = FillPreviewJsonSourceAuthority::read(preview)?.ok_or(())?.root;
                FillPreviewJsonUnit::formatted(format_args!(",\"sourceVortexIndex\":{source_vortex_index}"))?
            }
            4 => self.quoted(preview, color, status_label, FillPreviewQuotedField { prefix: b",\"meshUrl\":", source: FillPreviewString::RootMesh, optional: false, advance: true })?,
            5 => {
                self.advance_field();
                preview_json_vec3(",\"origin\":", ghost.ok_or(())?.origin)?
            }
            6 => {
                self.advance_field();
                preview_json_quat(",\"orientation\":", ghost.ok_or(())?.orientation)?
            }
            7 => self.quoted(preview, color, status_label, FillPreviewQuotedField { prefix: b",\"color\":", source: FillPreviewString::Color, optional: false, advance: true })?,
            8 => {
                self.advance_field();
                FillPreviewJsonUnit::bytes(b",\"opacity\":0.35")?
            }
            9 => {
                self.advance_field();
                FillPreviewJsonUnit::bytes(if ghost.is_some() { b",\"fillBuildPreview\":{" } else { b"\"fillBuildPreview\":{" })?
            }
            10 => {
                self.advance_field();
                FillPreviewJsonUnit::formatted(format_args!("\"operation\":{}", preview.operation))?
            }
            11 => {
                self.advance_field();
                FillPreviewJsonUnit::formatted(format_args!(",\"baseRevision\":{}", preview.base_revision))?
            }
            12 => {
                self.advance_field();
                FillPreviewJsonUnit::formatted(format_args!(",\"registryGeneration\":{}", preview.registry_generation))?
            }
            13 => {
                self.advance_field();
                FillPreviewJsonUnit::formatted(format_args!(",\"sequence\":{}", preview.sequence))?
            }
            14 => {
                self.advance_field();
                FillPreviewJsonUnit::formatted(format_args!(",\"generation\":{}", preview.generation))?
            }
            15 => self.quoted(preview, color, status_label, FillPreviewQuotedField { prefix: b",\"stage\":", source: FillPreviewString::Stage, optional: false, advance: true })?,
            16 => self.quoted(preview, color, status_label, FillPreviewQuotedField { prefix: b",\"statusLabel\":", source: FillPreviewString::StatusLabel, optional: false, advance: true })?,
            17 => self.quoted(preview, color, status_label, FillPreviewQuotedField { prefix: b",\"targetVortexFullId\":", source: FillPreviewString::Target, optional: true, advance: true })?,
            18 => self.quoted(preview, color, status_label, FillPreviewQuotedField { prefix: b",\"candidateObjectKindId\":", source: FillPreviewString::Candidate, optional: true, advance: true })?,
            19 => self.candidate_ghost(preview, color, status_label)?,
            20 => self.quoted(preview, color, status_label, FillPreviewQuotedField { prefix: b",\"currentPairObjectId\":", source: FillPreviewString::CurrentPair, optional: true, advance: true })?,
            21 => {
                self.advance_field();
                FillPreviewJsonUnit::formatted(format_args!(",\"collisionCount\":{}", preview.collision_count))?
            }
            22 => {
                self.advance_field();
                FillPreviewJsonUnit::formatted(format_args!(",\"sampleCursor\":{}", preview.sample_cursor))?
            }
            23 => {
                self.advance_field();
                FillPreviewJsonUnit::formatted(format_args!(",\"insideBoth\":{}", preview.inside_both))?
            }
            24 => {
                self.advance_field();
                match preview.last_sample {
                    Some(value) => preview_json_vec3(",\"lastSample\":", value.map(f64::from))?,
                    None => FillPreviewJsonUnit::bytes(b",\"lastSample\":null")?,
                }
            }
            25 => self.candidate_page(preview, color, status_label)?,
            26 => {
                self.advance_field();
                FillPreviewJsonUnit::bytes(if preview.truncated { b",\"truncated\":true" } else { b",\"truncated\":false" })?
            }
            27 => self.quoted(preview, color, status_label, FillPreviewQuotedField { prefix: b",\"rejectionReason\":", source: FillPreviewString::Rejection, optional: true, advance: true })?,
            28 => {
                self.advance_field();
                FillPreviewJsonUnit::formatted(format_args!(",\"targetCursor\":{}", preview.target_cursor))?
            }
            29 => {
                self.advance_field();
                FillPreviewJsonUnit::formatted(format_args!(",\"candidateCursor\":{}", preview.candidate_cursor))?
            }
            30 => {
                self.advance_field();
                FillPreviewJsonUnit::formatted(format_args!(",\"acceptedCount\":{}", preview.accepted_count))?
            }
            31 => {
                self.advance_field();
                FillPreviewJsonUnit::formatted(format_args!(",\"totalCount\":{}", preview.total_count))?
            }
            32 => {
                self.advance_field();
                FillPreviewJsonUnit::formatted(format_args!(",\"searchCount\":{}", preview.search_count))?
            }
            33 => {
                self.advance_field();
                FillPreviewJsonUnit::formatted(format_args!(",\"rejectedCount\":{}}}}}", preview.rejected_count))?
            }
            _ => return Ok(None),
        };
        Ok(Some(unit))
    }
}

fn fill_preview_json_wire_bytes(preview: &FillBuildPreview, color: &str, status_label: &str) -> Result<usize, ()> {
    let mut pass = FillPreviewJsonPass::default();
    let mut exact_bytes = 0usize;
    while let Some(unit) = pass.next_unit(preview, color, status_label)? {
        exact_bytes = exact_bytes.checked_add(unit.len).filter(|bytes| *bytes <= FILL_PREVIEW_JSON_MAX_BYTES).ok_or(())?;
    }
    Ok(exact_bytes)
}

pub(crate) struct FillPreviewJsonCursor {
    phase: FillPreviewJsonPhase,
    identity: Option<FillPreviewJsonIdentity>,
    color: String,
    status_label: String,
    census: FillPreviewJsonPass,
    encode: FillPreviewJsonPass,
    exact_bytes: usize,
    output: Option<Vec<u8>>,
    ready: Option<String>,
    ready_identity: Option<FillPreviewJsonIdentity>,
    retiring_bytes: Option<Vec<u8>>,
    retiring_ready: Option<String>,
    retiring_color: Option<String>,
    retiring_status_label: Option<String>,
    progress: u64,
}

impl Default for FillPreviewJsonCursor {
    fn default() -> Self {
        Self {
            phase: FillPreviewJsonPhase::Idle,
            identity: None,
            color: String::new(),
            status_label: String::new(),
            census: FillPreviewJsonPass::default(),
            encode: FillPreviewJsonPass::default(),
            exact_bytes: 0,
            output: None,
            ready: None,
            ready_identity: None,
            retiring_bytes: None,
            retiring_ready: None,
            retiring_color: None,
            retiring_status_label: None,
            progress: 0,
        }
    }
}

impl FillPreviewJsonCursor {
    fn checkpoint(&self) -> [u64; 6] {
        let identity = self.identity.unwrap_or(FillPreviewJsonIdentity { operation: 0, base_revision: 0, registry_generation: 0, generation: 0, sequence: 0 });
        [identity.operation, identity.base_revision, identity.registry_generation, identity.generation, identity.sequence, self.progress]
    }

    fn pending(&self) -> FillPreviewJsonStep {
        FillPreviewJsonStep::Pending { progress: self.progress, checkpoint: self.checkpoint() }
    }

    fn begin(&mut self, identity: FillPreviewJsonIdentity, color: &str, status_label: &str) -> FillPreviewJsonStep {
        if color.len() > FILL_PREVIEW_JSON_MAX_COLOR_BYTES || status_label.is_empty() || status_label.len() > FILL_PREVIEW_JSON_MAX_STATUS_LABEL_BYTES {
            self.phase = FillPreviewJsonPhase::Rejected;
            return FillPreviewJsonStep::Rejected;
        }
        self.identity = Some(identity);
        self.census = FillPreviewJsonPass::default();
        self.encode = FillPreviewJsonPass::default();
        self.exact_bytes = 0;
        self.progress = 0;
        if let Some(output) = self.output.take() {
            self.retiring_bytes = Some(output);
        }
        if self.color != color {
            let retiring = std::mem::take(&mut self.color);
            self.retiring_color = (retiring.capacity() != 0).then_some(retiring);
            if self.color.try_reserve_exact(color.len()).is_err() {
                self.phase = FillPreviewJsonPhase::Rejected;
                return FillPreviewJsonStep::Rejected;
            }
            self.color.push_str(color);
        }
        if self.status_label != status_label {
            let retiring = std::mem::take(&mut self.status_label);
            self.retiring_status_label = (retiring.capacity() != 0).then_some(retiring);
            if self.status_label.try_reserve_exact(status_label.len()).is_err() {
                self.phase = FillPreviewJsonPhase::Rejected;
                return FillPreviewJsonStep::Rejected;
            }
            self.status_label.push_str(status_label);
        }
        self.phase = if self.retiring_bytes.is_some() || self.retiring_color.as_ref().is_some_and(|value| value.capacity() != 0) || self.retiring_status_label.as_ref().is_some_and(|value| value.capacity() != 0) {
            FillPreviewJsonPhase::RetireSuperseded
        } else {
            FillPreviewJsonPhase::Census
        };
        self.pending()
    }

    pub(crate) fn step(&mut self, preview: &FillBuildPreview, color: &str, status_label: &str, fuel: &mut u32, cancelled: bool, deadline_reached: bool) -> FillPreviewJsonStep {
        if matches!(self.phase, FillPreviewJsonPhase::Closing | FillPreviewJsonPhase::Terminal) {
            return if self.phase == FillPreviewJsonPhase::Terminal { FillPreviewJsonStep::Terminal } else { self.pending() };
        }
        if FillPreviewJsonAdmission::read(preview, color, status_label).is_err() {
            return FillPreviewJsonStep::Rejected;
        }
        if cancelled {
            if let Some(output) = self.output.take() {
                self.retiring_bytes = Some(output);
            }
            self.phase = FillPreviewJsonPhase::RetireSuperseded;
            return FillPreviewJsonStep::Cancelled;
        }
        if deadline_reached {
            return self.pending();
        }
        let Some(next_fuel) = fuel.checked_sub(1) else {
            return self.pending();
        };
        *fuel = next_fuel;
        let Some(identity) = FillPreviewJsonIdentity::read(preview) else {
            self.phase = FillPreviewJsonPhase::Rejected;
            return FillPreviewJsonStep::Rejected;
        };
        if self.identity != Some(identity) || self.color != color || self.status_label != status_label {
            let result = self.begin(identity, color, status_label);
            self.progress = self.progress.saturating_add(1);
            return result;
        }
        let result = match self.phase {
            FillPreviewJsonPhase::Idle => self.begin(identity, color, status_label),
            FillPreviewJsonPhase::RetireSuperseded => {
                if self.retiring_bytes.take().is_none() && self.retiring_color.take().is_none() {
                    self.retiring_status_label.take();
                }
                if self.retiring_bytes.is_none() && self.retiring_color.is_none() && self.retiring_status_label.is_none() {
                    self.phase = FillPreviewJsonPhase::Census;
                }
                self.pending()
            }
            FillPreviewJsonPhase::Census => match self.census.next_unit(preview, &self.color, &self.status_label) {
                Ok(Some(unit)) => match self.exact_bytes.checked_add(unit.len) {
                    Some(bytes) if bytes <= FILL_PREVIEW_JSON_MAX_BYTES => {
                        self.exact_bytes = bytes;
                        self.pending()
                    }
                    _ => {
                        self.phase = FillPreviewJsonPhase::Rejected;
                        FillPreviewJsonStep::Rejected
                    }
                },
                Ok(None) => {
                    self.phase = FillPreviewJsonPhase::Reserve;
                    self.pending()
                }
                Err(()) => {
                    self.phase = FillPreviewJsonPhase::Rejected;
                    FillPreviewJsonStep::Rejected
                }
            },
            FillPreviewJsonPhase::Reserve => {
                let mut output = Vec::new();
                if output.try_reserve_exact(self.exact_bytes).is_err() {
                    self.phase = FillPreviewJsonPhase::Rejected;
                    FillPreviewJsonStep::Rejected
                } else {
                    self.output = Some(output);
                    self.phase = FillPreviewJsonPhase::Encode;
                    self.pending()
                }
            }
            FillPreviewJsonPhase::Encode => match self.encode.next_unit(preview, &self.color, &self.status_label) {
                Ok(Some(unit)) => {
                    let Some(output) = self.output.as_mut() else {
                        self.phase = FillPreviewJsonPhase::Rejected;
                        return FillPreviewJsonStep::Rejected;
                    };
                    if output.len().saturating_add(unit.len) > self.exact_bytes {
                        self.phase = FillPreviewJsonPhase::Rejected;
                        return FillPreviewJsonStep::Rejected;
                    }
                    output.extend_from_slice(&unit.bytes[..unit.len]);
                    self.pending()
                }
                Ok(None) if self.output.as_ref().is_some_and(|output| output.len() == self.exact_bytes) => {
                    self.phase = FillPreviewJsonPhase::Validate;
                    self.pending()
                }
                _ => {
                    self.phase = FillPreviewJsonPhase::Rejected;
                    FillPreviewJsonStep::Rejected
                }
            },
            FillPreviewJsonPhase::Validate => {
                if self.identity != FillPreviewJsonIdentity::read(preview) {
                    return self.begin(identity, color, status_label);
                }
                let Some(output) = self.output.take() else {
                    self.phase = FillPreviewJsonPhase::Rejected;
                    return FillPreviewJsonStep::Rejected;
                };
                let Ok(text) = String::from_utf8(output) else {
                    self.phase = FillPreviewJsonPhase::Rejected;
                    return FillPreviewJsonStep::Rejected;
                };
                if let Some(ready) = self.ready.replace(text) {
                    self.retiring_ready = Some(ready);
                }
                self.ready_identity = self.identity;
                self.phase = FillPreviewJsonPhase::Ready;
                FillPreviewJsonStep::Ready
            }
            FillPreviewJsonPhase::Ready => {
                self.retiring_ready.take();
                FillPreviewJsonStep::Ready
            }
            FillPreviewJsonPhase::Rejected => FillPreviewJsonStep::Rejected,
            FillPreviewJsonPhase::Closing | FillPreviewJsonPhase::Terminal => unreachable!(),
        };
        self.progress = self.progress.saturating_add(1);
        result
    }

    pub(crate) fn ready(&self) -> Option<&str> {
        self.ready.as_deref()
    }

    #[cfg(test)]
    pub(crate) fn ready_identity(&self) -> Option<[u64; 5]> {
        self.ready_identity.map(|identity| [identity.operation, identity.base_revision, identity.registry_generation, identity.generation, identity.sequence])
    }

    pub(crate) fn close_step(&mut self) -> bool {
        self.phase = FillPreviewJsonPhase::Closing;
        if self.output.take().is_some() {
            return false;
        }
        if self.retiring_bytes.take().is_some() {
            return false;
        }
        if self.ready.take().is_some() {
            return false;
        }
        if self.retiring_ready.take().is_some() {
            return false;
        }
        if self.retiring_color.take().is_some() {
            return false;
        }
        if self.retiring_status_label.take().is_some() {
            return false;
        }
        if !self.color.is_empty() {
            drop(std::mem::take(&mut self.color));
            return false;
        }
        if !self.status_label.is_empty() {
            drop(std::mem::take(&mut self.status_label));
            return false;
        }
        self.identity = None;
        self.ready_identity = None;
        self.phase = FillPreviewJsonPhase::Terminal;
        true
    }

    fn terminal_owners_empty(&self) -> bool {
        self.output.is_none()
            && self.ready.is_none()
            && self.retiring_bytes.is_none()
            && self.retiring_ready.is_none()
            && self.retiring_color.is_none()
            && self.retiring_status_label.is_none()
            && self.color.capacity() == 0
            && self.status_label.capacity() == 0
    }
}
//#endregion 🔭️RetainedPreviewJson

/// 🧱️ One already-placed object's collision footprint, kept alongside the plan so each new fill step
/// only has to test the candidate against bodies it can actually hit.
#[derive(Clone)]
pub(crate) struct PlacedCollisionEntry {
    pub(crate) object_id: String,
    pub(crate) mesh_url: String,
    pub(crate) world: Pose3d,
}

fn fenwick_add(tree: &mut [f64], index: usize, delta: f64) {
    let mut cursor = index + 1;
    while cursor < tree.len() {
        tree[cursor] += delta;
        cursor += cursor & cursor.wrapping_neg();
    }
}

fn fenwick_total(tree: &[f64]) -> f64 {
    let mut cursor = tree.len().saturating_sub(1);
    let mut total = 0.0;
    while cursor > 0 {
        total += tree[cursor];
        cursor &= cursor - 1;
    }
    total
}

fn fenwick_pick(tree: &[f64], target: f64) -> usize {
    let mut index = 0;
    let mut prefix = 0.0;
    let mut bit = 1usize;
    while bit < tree.len() {
        bit <<= 1;
    }
    let mut step = bit >> 1;
    while step > 0 {
        let next = index + step;
        if next < tree.len() && prefix + tree[next] < target {
            prefix += tree[next];
            index = next;
        }
        step >>= 1;
    }
    index.min(tree.len().saturating_sub(2))
}

fn weighted_pick(weights: &mut [f64], tree: &mut [f64], remaining: usize, rng_state: &mut u32) -> Option<usize> {
    if remaining == 0 {
        return None;
    }
    let total = fenwick_total(tree);
    if total <= 0.0 {
        return None;
    }
    let target = if weights.len() == 1 { f64::MIN_POSITIVE } else { (fill_rng(rng_state) * total).max(f64::MIN_POSITIVE) };
    let index = fenwick_pick(tree, target);
    let weight = std::mem::replace(&mut weights[index], 0.0);
    fenwick_add(tree, index, -weight);
    Some(index)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum FillJobStage {
    DiscardTail,
    PrepareFixture,
    PrepareCatalogs,
    PrepareMeshes,
    PrepareEntries,
    PrepareSpatial,
    PrepareLookup,
    PrepareConfiguration,
    PrepareTargets,
    SelectTarget,
    PrepareCandidates,
    SelectCandidate,
    ConstructPreview,
    QueryBroadPhase,
    TestCollision,
    AcceptCandidate,
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TargetPreparePhase {
    Reset,
    Blocked,
    Enumerate,
    BuildSeedWeights,
    BuildFrontierWeights,
    OrderSeed,
    OrderFrontier,
    Finish,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CandidatePreparePhase {
    Reset,
    Enumerate,
    Classify,
    DrainCross,
    DrainSame,
    BuildSameWeights,
    OrderSame,
    Finish,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AcceptPhase {
    Validate,
    CheckAttractions,
    BuildVortices,
    BeginSpatial,
    StepSpatial,
    InstallLookup,
    Commit,
}

pub(crate) struct FillPreparationRoots {
    scene: Arc<SceneConfig>,
    meshes: Arc<HashMap<String, CollisionBody>>,
}

impl FillPreparationRoots {
    pub(crate) fn new(scene: Arc<SceneConfig>, meshes: Arc<HashMap<String, CollisionBody>>) -> Self {
        Self { scene, meshes }
    }
}

#[derive(Debug)]
pub(crate) struct FixedFixtureOwner {
    pub(crate) objects: FixedOwnerVec<FixtureObject, DOCUMENT_OBJECT_SLOTS>,
    pub(crate) attractions: FixedOwnerVec<AttractionProps, DOCUMENT_ATTRACTION_SLOTS>,
    pub(crate) target_volumes: FixedOwnerVec<WorldVolumeProps, DOCUMENT_VOLUME_SLOTS>,
}

impl FixedFixtureOwner {
    fn new() -> Self {
        Self { objects: FixedOwnerVec::new(), attractions: FixedOwnerVec::new(), target_volumes: FixedOwnerVec::new() }
    }

    pub(crate) fn snapshot(&self) -> Fixture {
        Fixture { objects: self.objects.iter().cloned().collect(), attractions: self.attractions.iter().cloned().collect(), target_volumes: self.target_volumes.iter().cloned().collect() }
    }
}

#[derive(Debug)]
struct FixedCatalogOwner {
    objects: FixedOwnerVec<ObjectKind, DOCUMENT_KIND_SLOTS>,
    vortices: FixedOwnerVec<VortexKindCatalog, DOCUMENT_KIND_SLOTS>,
    cables: FixedOwnerVec<CableKindCatalog, DOCUMENT_KIND_SLOTS>,
}

impl FixedCatalogOwner {
    fn new() -> Self {
        Self { objects: FixedOwnerVec::new(), vortices: FixedOwnerVec::new(), cables: FixedOwnerVec::new() }
    }
}

impl BrushCatalogView for FixedCatalogOwner {
    fn objects(&self) -> &[ObjectKind] {
        self.objects.as_slice()
    }

    fn vortices(&self) -> &[VortexKindCatalog] {
        self.vortices.as_slice()
    }

    fn cables(&self) -> &[CableKindCatalog] {
        self.cables.as_slice()
    }
}

struct FillFixtureView<'a> {
    base: &'a FixedFixtureOwner,
    appended: &'a [FixtureObject],
}

impl BrushFixtureView for FillFixtureView<'_> {
    fn object_count(&self) -> usize {
        self.base.objects.len() + self.appended.len()
    }

    fn find_object_kind(&self, kind_id: &str) -> Option<&FixtureObject> {
        self.base.objects.iter().chain(self.appended).find(|object| object.object_kind.as_deref() == Some(kind_id))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PreparationCapacityBranch {
    FixtureObjects,
    FixtureAttractions,
    FixtureTargetVolumes,
    Meshes,
    CatalogObjects,
    CatalogVortices,
    CatalogCables,
    KindCompatibility,
    ObjectWeights,
    VortexWeights,
}

impl PreparationCapacityBranch {
    fn label(self) -> &'static str {
        match self {
            Self::FixtureObjects => "fixture-objects",
            Self::FixtureAttractions => "fixture-attractions",
            Self::FixtureTargetVolumes => "fixture-target-volumes",
            Self::Meshes => "meshes",
            Self::CatalogObjects => "catalog-objects",
            Self::CatalogVortices => "catalog-vortices",
            Self::CatalogCables => "catalog-cables",
            Self::KindCompatibility => "kind-compatibility",
            Self::ObjectWeights => "object-weights",
            Self::VortexWeights => "vortex-weights",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct PreparationCapacityRefusal {
    branch: PreparationCapacityBranch,
    omitted_index: usize,
    diagnostic_published: bool,
}

impl PreparationCapacityRefusal {
    /// 🏷️ `preparation-capacity:<branch>:<limit>` — the refused document-scale branch and the exact
    /// capacity its fixed page declares, so the published diagnostic never hides which limit bound.
    fn diagnostic(self) -> String {
        format!("preparation-capacity:{}:{}", self.branch.label(), self.omitted_index)
    }
}

/// 🚧️ Preflights every document-scale owner the preparation stages install into, each against the
/// capacity its own fixed page declares — objects and attractions at `DOCUMENT_OBJECT_SLOTS` /
/// `DOCUMENT_ATTRACTION_SLOTS` (scene plus a full `FILL_COUNT_MAX` plan), catalogs, compatibility
/// rows, mesh urls and kind weights at `DOCUMENT_KIND_SLOTS`. The refusal carries that capacity so
/// the published diagnostic names the branch and the limit it exceeded.
fn preparation_capacity_refusal(roots: &FillPreparationRoots) -> Option<PreparationCapacityRefusal> {
    let catalogs = roots.scene.kind_catalogs.as_ref();
    let (branch, capacity) = [
        (PreparationCapacityBranch::FixtureObjects, roots.scene.fixture.objects.len(), DOCUMENT_OBJECT_SLOTS),
        (PreparationCapacityBranch::FixtureAttractions, roots.scene.fixture.attractions.len(), DOCUMENT_ATTRACTION_SLOTS),
        (PreparationCapacityBranch::FixtureTargetVolumes, roots.scene.fixture.target_volumes.len(), DOCUMENT_VOLUME_SLOTS),
        (PreparationCapacityBranch::Meshes, roots.meshes.len(), DOCUMENT_KIND_SLOTS),
        (PreparationCapacityBranch::CatalogObjects, catalogs.map_or(0, |value| value.objects.len()), DOCUMENT_KIND_SLOTS),
        (PreparationCapacityBranch::CatalogVortices, catalogs.map_or(0, |value| value.vortices.len()), DOCUMENT_KIND_SLOTS),
        (PreparationCapacityBranch::CatalogCables, catalogs.map_or(0, |value| value.cables.len()), DOCUMENT_KIND_SLOTS),
        (PreparationCapacityBranch::KindCompatibility, roots.scene.kind_compatibility.len(), DOCUMENT_KIND_SLOTS),
        (PreparationCapacityBranch::ObjectWeights, roots.scene.weights.object_weights.len(), DOCUMENT_KIND_SLOTS),
        (PreparationCapacityBranch::VortexWeights, roots.scene.weights.vortex_weights.len(), DOCUMENT_KIND_SLOTS),
    ]
    .into_iter()
    .find_map(|(branch, len, capacity)| (len > capacity).then_some((branch, capacity)))?;
    Some(PreparationCapacityRefusal { branch, omitted_index: capacity, diagnostic_published: false })
}

pub(crate) struct FillBuilder {
    pub(crate) base: FixedFixtureOwner,
    preparation_roots: Option<FillPreparationRoots>,
    preparation_cursor: usize,
    preparation_inner_cursor: usize,
    preparation_spatial: Option<CollisionIndexMutation>,
    /// 🎚️ Live withdrawal of ONE unapplied placement during a weight-only replan — the same
    /// resumable index mutation shape as `preparation_spatial`, so the discard never scans.
    tail_removal: Option<CollisionIndexRemoval>,
    preparation_capacity_refusal: Option<PreparationCapacityRefusal>,
    pub(crate) applied_count: usize,
    pub(crate) sequence: Vec<BrushPlacePayload>,
    pub(crate) appended_objects: Vec<FixtureObject>,
    pub(crate) appended_attractions: Vec<AttractionProps>,
    pub(crate) placed: Vec<PlacedCollisionEntry>,
    placed_lookup: FixedOwnerMap<String, usize, DOCUMENT_OBJECT_SLOTS>,
    /// 🗄️ Keeps the bookkeeping page width: no stage installs a per-target-vortex candidate list
    /// yet, so this owner exists for the census/retirement walk only and never reaches document
    /// scale. Wiring a real cache means widening it to `DOCUMENT_VORTEX_SLOTS` in the same move.
    pub(crate) candidate_cache: FixedOwnerMap<String, Vec<BrushCompatibleCandidate>>,
    pub(crate) seed_object_ids: FixedOwnerSet<String, DOCUMENT_OBJECT_SLOTS>,
    pub(crate) rng_state: u32,
    pub(crate) stalled: bool,
    pub(crate) max_count: usize,
    pub(crate) operation: Operation,
    pub(crate) stage: FillJobStage,
    pub(crate) preview: FillBuildPreview,
    preview_json: FillPreviewJsonCursor,
    catalogs: FixedCatalogOwner,
    weights: RetainedBrushKindWeights,
    kind_compatibility: FixedOwnerVec<KindCompatEntry, DOCUMENT_KIND_SLOTS>,
    host_rules: BrushHostRules,
    overlap_budget: f64,
    meshes: FixedOwnerMap<String, CollisionBody, DOCUMENT_KIND_SLOTS>,
    spatial_index: CollisionSpatialIndex,
    targets: Vec<BrushFillVortexTarget>,
    target_cursor: usize,
    target_rotation: usize,
    target_prepare_phase: TargetPreparePhase,
    blocked_vortex_ids: FixedOwnerSet<String, DOCUMENT_VORTEX_SLOTS>,
    target_attraction_cursor: usize,
    target_object_cursor: usize,
    target_vortex_cursor: usize,
    seed_targets: Vec<BrushFillVortexTarget>,
    frontier_targets: Vec<BrushFillVortexTarget>,
    seed_target_weights: Vec<f64>,
    frontier_target_weights: Vec<f64>,
    seed_target_tree: Vec<f64>,
    frontier_target_tree: Vec<f64>,
    target_prepare_cursor: usize,
    seed_target_remaining: usize,
    frontier_target_remaining: usize,
    current_target: Option<BrushFillVortexTarget>,
    candidates: Vec<BrushCompatibleCandidate>,
    candidate_cursor: usize,
    candidate_prepare_phase: CandidatePreparePhase,
    candidate_kind_cursor: usize,
    candidate_vortex_cursor: usize,
    candidate_prepare_cursor: usize,
    candidate_seen: FixedOwnerSet<String, DOCUMENT_CANDIDATE_SLOTS>,
    candidate_raw: Vec<BrushCompatibleCandidate>,
    candidate_cross: FixedOwnerMap<String, BrushCompatibleCandidate, DOCUMENT_CANDIDATE_SLOTS>,
    candidate_same: FixedOwnerMap<String, BrushCompatibleCandidate, DOCUMENT_CANDIDATE_SLOTS>,
    candidate_same_sorted: Vec<BrushCompatibleCandidate>,
    candidate_same_weights: Vec<f64>,
    candidate_same_tree: Vec<f64>,
    candidate_same_remaining: usize,
    current_preview: Option<BrushPreviewState>,
    broad_phase_query: Option<CollisionQueryCursor>,
    broad_phase_cursor: usize,
    broad_phase_bounds: Option<CollisionAabb>,
    collision: Option<CollisionOverlapState>,
    accept_phase: AcceptPhase,
    accept_attraction_cursor: usize,
    accept_vortex_cursor: usize,
    pending_payload: Option<BrushPlacePayload>,
    pending_object: Option<FixtureObject>,
    pending_attraction: Option<AttractionProps>,
    pending_spatial: Option<CollisionIndexMutation>,
    last_rejection: Option<String>,
    fixed_rejection: Option<FillRetiredOwner>,
    collection_over_capacity: bool,
    pub(crate) transition_count: u64,
    rejected_count: u64,
    close_field: u8,
    close_current: Option<FillRetiredOwner>,
    closing: bool,
}

pub(crate) const FILL_BUILDER_OWNER_PAGE_BYTES: usize = 16 * 1024;
const FILL_BUILDER_NESTED_ITEMS: usize = 32;
const FILL_BUILDER_STD_COLLECTIONS: usize = 10;

struct RetainedBrushKindWeights {
    object_weights: FixedOwnerMap<String, f64, DOCUMENT_KIND_SLOTS>,
    vortex_weights: FixedOwnerMap<String, f64, DOCUMENT_KIND_SLOTS>,
}

impl RetainedBrushKindWeights {
    fn new() -> Self {
        Self { object_weights: FixedOwnerMap::new(), vortex_weights: FixedOwnerMap::new() }
    }

    fn object_value(&self, id: &str) -> f64 {
        self.object_weights.get(id).copied().unwrap_or(1.0)
    }

    fn vortex_value(&self, id: &str) -> f64 {
        self.vortex_weights.get(id).copied().unwrap_or(1.0)
    }
}

fn retained_fill_vortex_target_weight(target: &BrushFillVortexTarget, weights: &RetainedBrushKindWeights) -> f64 {
    weights.vortex_value(target.vortex_kind.as_deref().unwrap_or(""))
}

fn retained_candidate_suggestion_weight(candidate: &BrushCompatibleCandidate, weights: &RetainedBrushKindWeights, catalogs: &impl BrushCatalogView) -> f64 {
    let vortex_kind = catalogs.objects().iter().find(|kind| kind.id == candidate.object_kind_id).and_then(|kind| kind.vortices.get(candidate.source_vortex_index)).and_then(|template| template.vortex_kind.as_deref()).unwrap_or("");
    weights.object_value(&candidate.object_kind_id) * weights.vortex_value(vortex_kind)
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct FillBuilderOwnerCredit {
    pub(crate) items: usize,
    pub(crate) bytes: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum FillBuilderOwnerCensusStep {
    Pending,
    Complete(FillBuilderOwnerCredit),
    Rejected,
}

#[derive(Default)]
pub(crate) struct FillBuilderOwnerCensusCursor {
    field: u8,
    section: u8,
    phase: u8,
    index: usize,
    inner: usize,
    leaf: usize,
    dsl: Option<FillDslOwnerCensusCursor>,
    spatial: CollisionIndexOwnerCensusCursor,
    credit: FillBuilderOwnerCredit,
}

#[derive(Clone, Copy)]
enum FillOwnerCensusUnit {
    Credit(FillBuilderOwnerCredit),
    Advance,
    Rejected,
}

#[derive(Clone, Copy)]
enum FillDslOwnerRoot {
    FixtureObject { fixture: u8, index: usize },
    FixtureVolume { fixture: u8, index: usize },
    SequencePayload(usize),
    AppendedObject(usize),
    CatalogObject(usize),
    CurrentPreview,
    PendingPayload,
    PendingObject,
    PreviewGhost,
}

struct FillDslOwnerCensusCursor {
    root: FillDslOwnerRoot,
    depth: usize,
    path: [usize; 16],
    phase: [u8; 17],
    child: [usize; 17],
}

impl FillDslOwnerCensusCursor {
    fn new(root: FillDslOwnerRoot) -> Self {
        Self { root, depth: 0, path: [0; 16], phase: [0; 17], child: [0; 17] }
    }

    fn root<'a>(&self, fill: &'a FillBuilder) -> Option<&'a dsl::DslValue> {
        match self.root {
            FillDslOwnerRoot::FixtureObject { fixture, index } => {
                let value = if fixture == 0 { fill.base.objects.get(index) } else { fill.appended_objects.get(index) }?;
                value.scale.as_ref()
            }
            FillDslOwnerRoot::FixtureVolume { fixture, index } => {
                let value = (fixture == 0).then(|| fill.base.target_volumes.get(index)).flatten()?;
                value.scale.as_ref()
            }
            FillDslOwnerRoot::SequencePayload(index) => fill.sequence.get(index)?.scale.as_ref(),
            FillDslOwnerRoot::AppendedObject(index) => fill.appended_objects.get(index)?.scale.as_ref(),
            FillDslOwnerRoot::CatalogObject(index) => fill.catalogs.objects.get(index)?.scale.as_ref(),
            FillDslOwnerRoot::CurrentPreview => fill.current_preview.as_ref()?.scale.as_ref(),
            FillDslOwnerRoot::PendingPayload => fill.pending_payload.as_ref()?.scale.as_ref(),
            FillDslOwnerRoot::PendingObject => fill.pending_object.as_ref()?.scale.as_ref(),
            FillDslOwnerRoot::PreviewGhost => fill.preview.candidate_ghost.as_ref()?.scale.as_ref(),
        }
    }

    fn value<'a>(&self, fill: &'a FillBuilder) -> Option<&'a dsl::DslValue> {
        let mut value = self.root(fill)?;
        for depth in 0..self.depth {
            value = match value {
                dsl::DslValue::Array(values) => values.get(self.path[depth])?,
                dsl::DslValue::Object(values) => &values.get(self.path[depth])?.1,
                _ => return None,
            };
        }
        Some(value)
    }

    fn step(&mut self, fill: &FillBuilder) -> Result<Option<FillBuilderOwnerCredit>, ()> {
        let Some(value) = self.value(fill) else { return Err(()) };
        if self.phase[self.depth] == 0 {
            self.phase[self.depth] = 1;
            let bytes = match value {
                dsl::DslValue::String(value) => value.capacity(),
                dsl::DslValue::Array(values) => values.capacity().checked_mul(size_of::<dsl::DslValue>()).ok_or(())?,
                dsl::DslValue::Object(values) => values.capacity().checked_mul(size_of::<(String, dsl::DslValue)>()).ok_or(())?,
                _ => 0,
            };
            if bytes > FILL_BUILDER_OWNER_PAGE_BYTES {
                return Err(());
            }
            return Ok(Some(FillBuilderOwnerCredit { items: usize::from(bytes != 0), bytes }));
        }
        match value {
            dsl::DslValue::Array(values) if self.child[self.depth] < values.len() => {
                if self.depth == 16 {
                    return Err(());
                }
                self.path[self.depth] = self.child[self.depth];
                self.depth += 1;
                return Ok(None);
            }
            dsl::DslValue::Object(values) if self.child[self.depth] < values.len() => {
                if self.phase[self.depth] == 1 {
                    self.phase[self.depth] = 2;
                    let bytes = values[self.child[self.depth]].0.capacity();
                    if bytes > FILL_BUILDER_OWNER_PAGE_BYTES {
                        return Err(());
                    }
                    return Ok(Some(FillBuilderOwnerCredit { items: usize::from(bytes != 0), bytes }));
                }
                if self.depth == 16 {
                    return Err(());
                }
                self.path[self.depth] = self.child[self.depth];
                self.depth += 1;
                return Ok(None);
            }
            _ => {}
        }
        if self.depth == 0 {
            return Ok(None);
        }
        self.phase[self.depth] = 0;
        self.child[self.depth] = 0;
        self.depth -= 1;
        self.child[self.depth] += 1;
        if matches!(self.value(fill), Some(dsl::DslValue::Object(_))) {
            self.phase[self.depth] = 1;
        }
        Ok(None)
    }

    fn complete(&self, fill: &FillBuilder) -> bool {
        let Some(value) = self.value(fill) else { return true };
        self.depth == 0
            && self.phase[0] != 0
            && match value {
                dsl::DslValue::Array(values) => self.child[0] >= values.len(),
                dsl::DslValue::Object(values) => self.child[0] >= values.len(),
                _ => true,
            }
    }
}

fn fill_owner_strings<const N: usize>(values: [Option<&String>; N]) -> Option<FillBuilderOwnerCredit> {
    let mut credit = FillBuilderOwnerCredit::default();
    for value in values.into_iter().flatten() {
        if value.capacity() > FILL_BUILDER_OWNER_PAGE_BYTES {
            return None;
        }
        credit.items = credit.items.checked_add(usize::from(value.capacity() != 0))?;
        credit.bytes = credit.bytes.checked_add(value.capacity())?;
        if credit.bytes > FILL_BUILDER_OWNER_PAGE_BYTES {
            return None;
        }
    }
    Some(credit)
}

fn fill_owner_vec<T>(capacity: usize) -> Option<FillBuilderOwnerCredit> {
    let bytes = capacity.checked_mul(size_of::<T>())?;
    (capacity <= FILL_BUILDER_NESTED_ITEMS && bytes <= FILL_BUILDER_OWNER_PAGE_BYTES).then_some(FillBuilderOwnerCredit { items: usize::from(bytes != 0), bytes })
}

fn fill_owner_collection(occupied: usize) -> Option<FillBuilderOwnerCredit> {
    (occupied <= FILL_BUILDER_NESTED_ITEMS).then_some(FillBuilderOwnerCredit::default())
}

fn fill_fixed_vec_backing_credit<T, const N: usize>(values: &FixedOwnerVec<T, N>) -> Option<FillBuilderOwnerCredit> {
    let credit = values.backing_credit()?;
    (credit.1 <= DOCUMENT_OWNER_PAGE_BYTES).then_some(FillBuilderOwnerCredit { items: credit.0, bytes: credit.1 })
}

fn fill_collection_backing_credit(fill: &FillBuilder, index: usize) -> Option<FillBuilderOwnerCredit> {
    let credit = match index {
        0 => fill.placed_lookup.backing_credit(),
        1 => fill.candidate_cache.backing_credit(),
        2 => fill.seed_object_ids.backing_credit(),
        3 => fill.weights.object_weights.backing_credit(),
        4 => fill.weights.vortex_weights.backing_credit(),
        5 => fill.meshes.backing_credit(),
        6 => fill.blocked_vortex_ids.backing_credit(),
        7 => fill.candidate_seen.backing_credit(),
        8 => fill.candidate_cross.backing_credit(),
        9 => fill.candidate_same.backing_credit(),
        _ => return None,
    }?;
    (credit.1 <= DOCUMENT_OWNER_PAGE_BYTES).then_some(FillBuilderOwnerCredit { items: credit.0, bytes: credit.1 })
}

impl FillBuilderOwnerCensusCursor {
    fn finish_field(&mut self) -> FillOwnerCensusUnit {
        self.field += 1;
        self.section = 0;
        self.phase = 0;
        self.index = 0;
        self.inner = 0;
        self.leaf = 0;
        FillOwnerCensusUnit::Advance
    }

    pub(crate) fn step(&mut self, fill: &FillBuilder, max_items: usize, max_bytes: usize) -> FillBuilderOwnerCensusStep {
        if fill.collection_over_capacity || fill.fixed_rejection.is_some() {
            return FillBuilderOwnerCensusStep::Rejected;
        }
        if self.field > 13 {
            return FillBuilderOwnerCensusStep::Complete(self.credit);
        }
        if let Some(dsl) = self.dsl.as_mut() {
            if dsl.complete(fill) {
                self.dsl = None;
                return FillBuilderOwnerCensusStep::Pending;
            }
            let unit = match dsl.step(fill) {
                Ok(Some(credit)) => FillOwnerCensusUnit::Credit(credit),
                Ok(None) => FillOwnerCensusUnit::Advance,
                Err(()) => FillOwnerCensusUnit::Rejected,
            };
            return self.apply_unit(unit, max_items, max_bytes);
        }
        let unit = self.next_unit(fill);
        self.apply_unit(unit, max_items, max_bytes)
    }

    fn apply_unit(&mut self, unit: FillOwnerCensusUnit, max_items: usize, max_bytes: usize) -> FillBuilderOwnerCensusStep {
        let FillOwnerCensusUnit::Credit(credit) = unit else {
            return if matches!(unit, FillOwnerCensusUnit::Rejected) { FillBuilderOwnerCensusStep::Rejected } else { FillBuilderOwnerCensusStep::Pending };
        };
        let Some(items) = self.credit.items.checked_add(credit.items) else { return FillBuilderOwnerCensusStep::Rejected };
        let Some(bytes) = self.credit.bytes.checked_add(credit.bytes) else { return FillBuilderOwnerCensusStep::Rejected };
        if items > max_items || bytes > max_bytes {
            return FillBuilderOwnerCensusStep::Rejected;
        }
        self.credit = FillBuilderOwnerCredit { items, bytes };
        FillBuilderOwnerCensusStep::Pending
    }

    fn next_unit(&mut self, fill: &FillBuilder) -> FillOwnerCensusUnit {
        match self.field {
            0 => {
                if self.section == 0 {
                    self.section = 1;
                    return FillOwnerCensusUnit::Credit(FillBuilderOwnerCredit { items: 1, bytes: size_of::<FillBuilder>() });
                }
                if self.index < FILL_BUILDER_STD_COLLECTIONS {
                    let Some(credit) = fill_collection_backing_credit(fill, self.index) else { return FillOwnerCensusUnit::Rejected };
                    self.index += 1;
                    return FillOwnerCensusUnit::Credit(credit);
                }
                self.finish_field()
            }
            1 | 2 => self.fixture_unit(fill, self.field - 1),
            3 => self.sequence_unit(fill),
            4 => self.lookup_unit(fill),
            5 => self.catalog_unit(fill),
            6 => self.weight_mesh_unit(fill),
            7 => self.target_unit(fill),
            8 => self.target_weight_unit(fill),
            9 => self.candidate_unit(fill),
            10 => self.candidate_order_unit(fill),
            11 => self.pending_unit(fill),
            12 => self.preview_unit(fill),
            13 => self.final_unit(fill),
            _ => FillOwnerCensusUnit::Advance,
        }
    }

    fn credit(value: Option<FillBuilderOwnerCredit>) -> FillOwnerCensusUnit {
        value.map_or(FillOwnerCensusUnit::Rejected, FillOwnerCensusUnit::Credit)
    }

    fn start_dsl(&mut self, root: FillDslOwnerRoot) -> FillOwnerCensusUnit {
        self.dsl = Some(FillDslOwnerCensusCursor::new(root));
        self.phase += 1;
        FillOwnerCensusUnit::Advance
    }

    fn fixture_object_unit(&mut self, value: &FixtureObject, root: FillDslOwnerRoot) -> Option<FillOwnerCensusUnit> {
        let unit = match self.phase {
            0 => {
                self.phase = 1;
                Self::credit(fill_owner_strings([Some(&value.id), value.object_kind.as_ref(), value.mesh_url.as_ref()]))
            }
            1 if value.scale.is_some() => self.start_dsl(root),
            1 => {
                self.phase = 2;
                FillOwnerCensusUnit::Advance
            }
            2 => {
                self.phase = 3;
                Self::credit(fill_owner_vec::<VortexProps>(value.vortices.capacity()))
            }
            _ => match value.vortices.get(self.inner) {
                Some(vortex) => {
                    self.inner += 1;
                    Self::credit(fill_owner_strings([Some(&vortex.id), vortex.vortex_kind.as_ref()]))
                }
                None => {
                    self.phase = 0;
                    self.inner = 0;
                    return None;
                }
            },
        };
        Some(unit)
    }

    fn world_volume_unit(&mut self, value: &WorldVolumeProps, root: FillDslOwnerRoot) -> Option<FillOwnerCensusUnit> {
        match self.phase {
            0 => {
                self.phase = 1;
                Some(Self::credit(fill_owner_strings([Some(&value.id)])))
            }
            1 if value.scale.is_some() => Some(self.start_dsl(root)),
            _ => {
                self.phase = 0;
                None
            }
        }
    }

    fn payload_unit(&mut self, value: &BrushPlacePayload, root: FillDslOwnerRoot) -> Option<FillOwnerCensusUnit> {
        match self.phase {
            0 => {
                self.phase = 1;
                Some(Self::credit(fill_owner_strings([Some(&value.target_vortex_full_id), Some(&value.object_kind_id)])))
            }
            1 if value.scale.is_some() => Some(self.start_dsl(root)),
            _ => {
                self.phase = 0;
                None
            }
        }
    }

    fn fixture_unit(&mut self, fill: &FillBuilder, fixture_id: u8) -> FillOwnerCensusUnit {
        if fixture_id != 0 {
            return self.finish_field();
        }
        let fixture = &fill.base;
        match self.section {
            0 => {
                self.section = 1;
                Self::credit(fill_fixed_vec_backing_credit(&fixture.objects))
            }
            1 => match fixture.objects.get(self.index) {
                Some(value) => match self.fixture_object_unit(value, FillDslOwnerRoot::FixtureObject { fixture: fixture_id, index: self.index }) {
                    Some(unit) => unit,
                    None => {
                        self.index += 1;
                        FillOwnerCensusUnit::Advance
                    }
                },
                None => {
                    self.section = 2;
                    self.index = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            2 => {
                self.section = 3;
                Self::credit(fill_fixed_vec_backing_credit(&fixture.attractions))
            }
            3 => match fixture.attractions.get(self.index) {
                Some(value) => {
                    self.index += 1;
                    Self::credit(fill_owner_strings([Some(&value.id), Some(&value.attracting), Some(&value.attracted)]))
                }
                None => {
                    self.section = 4;
                    self.index = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            4 => {
                self.section = 5;
                Self::credit(fill_fixed_vec_backing_credit(&fixture.target_volumes))
            }
            5 => match fixture.target_volumes.get(self.index) {
                Some(value) => match self.world_volume_unit(value, FillDslOwnerRoot::FixtureVolume { fixture: fixture_id, index: self.index }) {
                    Some(unit) => unit,
                    None => {
                        self.index += 1;
                        FillOwnerCensusUnit::Advance
                    }
                },
                None => self.finish_field(),
            },
            _ => self.finish_field(),
        }
    }

    fn sequence_unit(&mut self, fill: &FillBuilder) -> FillOwnerCensusUnit {
        match self.section {
            0 => {
                self.section = 1;
                Self::credit(fill_owner_vec::<BrushPlacePayload>(fill.sequence.capacity()))
            }
            1 => match fill.sequence.get(self.index) {
                Some(value) => match self.payload_unit(value, FillDslOwnerRoot::SequencePayload(self.index)) {
                    Some(unit) => unit,
                    None => {
                        self.index += 1;
                        FillOwnerCensusUnit::Advance
                    }
                },
                None => {
                    self.section = 2;
                    self.index = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            2 => {
                self.section = 3;
                Self::credit(fill_owner_vec::<FixtureObject>(fill.appended_objects.capacity()))
            }
            3 => match fill.appended_objects.get(self.index) {
                Some(value) => match self.fixture_object_unit(value, FillDslOwnerRoot::AppendedObject(self.index)) {
                    Some(unit) => unit,
                    None => {
                        self.index += 1;
                        FillOwnerCensusUnit::Advance
                    }
                },
                None => {
                    self.section = 4;
                    self.index = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            4 => {
                self.section = 5;
                Self::credit(fill_owner_vec::<AttractionProps>(fill.appended_attractions.capacity()))
            }
            5 => match fill.appended_attractions.get(self.index) {
                Some(value) => {
                    self.index += 1;
                    Self::credit(fill_owner_strings([Some(&value.id), Some(&value.attracting), Some(&value.attracted)]))
                }
                None => {
                    self.section = 6;
                    self.index = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            6 => {
                self.section = 7;
                Self::credit(fill_owner_vec::<PlacedCollisionEntry>(fill.placed.capacity()))
            }
            7 => match fill.placed.get(self.index) {
                Some(value) => {
                    self.index += 1;
                    Self::credit(fill_owner_strings([Some(&value.object_id), Some(&value.mesh_url)]))
                }
                None => self.finish_field(),
            },
            _ => self.finish_field(),
        }
    }

    fn lookup_unit(&mut self, fill: &FillBuilder) -> FillOwnerCensusUnit {
        match self.section {
            0 => {
                self.section = 1;
                Self::credit(fill_owner_collection(fill.placed_lookup.len()))
            }
            1 => match fill.placed_lookup.keys().nth(self.index) {
                Some(key) => {
                    self.index += 1;
                    let Some(mut credit) = fill_owner_strings([Some(key)]) else { return FillOwnerCensusUnit::Rejected };
                    credit.items += 1;
                    Self::credit(Some(credit))
                }
                None => {
                    self.section = 2;
                    self.index = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            2 => {
                self.section = 3;
                Self::credit(fill_owner_collection(fill.candidate_cache.len()))
            }
            3 => match fill.candidate_cache.iter().nth(self.index) {
                Some((key, values)) if self.phase == 0 => {
                    self.phase = 1;
                    let Some(mut credit) = fill_owner_strings([Some(key)]) else { return FillOwnerCensusUnit::Rejected };
                    let Some(backing) = fill_owner_vec::<BrushCompatibleCandidate>(values.capacity()) else { return FillOwnerCensusUnit::Rejected };
                    credit.items = credit.items.saturating_add(backing.items).saturating_add(1);
                    credit.bytes = credit.bytes.saturating_add(backing.bytes);
                    FillOwnerCensusUnit::Credit(credit)
                }
                Some((_, values)) => match values.get(self.inner) {
                    Some(value) => {
                        self.inner += 1;
                        Self::credit(fill_owner_strings([Some(&value.object_kind_id)]))
                    }
                    None => {
                        self.index += 1;
                        self.inner = 0;
                        self.phase = 0;
                        FillOwnerCensusUnit::Advance
                    }
                },
                None => {
                    self.section = 4;
                    self.index = 0;
                    self.phase = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            4 => {
                self.section = 5;
                Self::credit(fill_owner_collection(fill.seed_object_ids.len()))
            }
            5 => match fill.seed_object_ids.iter().nth(self.index) {
                Some(value) => {
                    self.index += 1;
                    let Some(mut credit) = fill_owner_strings([Some(value)]) else { return FillOwnerCensusUnit::Rejected };
                    credit.items += 1;
                    Self::credit(Some(credit))
                }
                None => self.finish_field(),
            },
            _ => self.finish_field(),
        }
    }

    fn catalog_unit(&mut self, fill: &FillBuilder) -> FillOwnerCensusUnit {
        match self.section {
            0 => {
                self.section = 1;
                Self::credit(fill_fixed_vec_backing_credit(&fill.catalogs.objects))
            }
            1 => match fill.catalogs.objects.get(self.index) {
                Some(value) => match self.phase {
                    0 => {
                        self.phase = 1;
                        Self::credit(fill_owner_strings([Some(&value.id)]))
                    }
                    1 if value.scale.is_some() => self.start_dsl(FillDslOwnerRoot::CatalogObject(self.index)),
                    1 => {
                        self.phase = 2;
                        FillOwnerCensusUnit::Advance
                    }
                    2 => {
                        self.phase = 3;
                        Self::credit(fill_owner_vec::<crate::standards::v1::subsets::any::schema::ObjectKindRepresentation>(value.representations.capacity()))
                    }
                    3 => match value.representations.get(self.inner) {
                        Some(representation) if self.leaf == 0 => {
                            self.leaf = 1;
                            Self::credit(fill_owner_strings([Some(&representation.id), Some(&representation.name), Some(&representation.url), Some(&representation.mime), representation.lod.as_ref(), Some(&representation.description)]))
                        }
                        Some(representation) if self.leaf == 1 => {
                            self.leaf = 2;
                            Self::credit(fill_owner_vec::<String>(representation.tags.capacity()))
                        }
                        Some(representation) => match representation.tags.get(self.leaf - 2) {
                            Some(tag) => {
                                self.leaf += 1;
                                Self::credit(fill_owner_strings([Some(tag)]))
                            }
                            None => {
                                self.inner += 1;
                                self.leaf = 0;
                                FillOwnerCensusUnit::Advance
                            }
                        },
                        None => {
                            self.phase = 4;
                            self.inner = 0;
                            self.leaf = 0;
                            FillOwnerCensusUnit::Advance
                        }
                    },
                    4 => {
                        self.phase = 5;
                        Self::credit(fill_owner_vec::<crate::standards::v1::subsets::any::schema::ObjectKindVortexTemplate>(value.vortices.capacity()))
                    }
                    _ => match value.vortices.get(self.inner) {
                        Some(vortex) => {
                            self.inner += 1;
                            Self::credit(fill_owner_strings([Some(&vortex.id), Some(&vortex.name), Some(&vortex.label), Some(&vortex.description), Some(&vortex.icon), vortex.vortex_kind.as_ref()]))
                        }
                        None => {
                            self.index += 1;
                            self.inner = 0;
                            self.phase = 0;
                            FillOwnerCensusUnit::Advance
                        }
                    },
                },
                None => {
                    self.section = 2;
                    self.index = 0;
                    self.phase = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            2 => {
                self.section = 3;
                Self::credit(fill_fixed_vec_backing_credit(&fill.catalogs.vortices))
            }
            3 => match fill.catalogs.vortices.get(self.index) {
                Some(value) if self.phase == 0 => {
                    self.phase = 1;
                    Self::credit(fill_owner_strings([Some(&value.id), value.code.as_ref(), value.label.as_ref(), Some(&value.description), Some(&value.icon), Some(&value.color), value.default_cable_kind.as_ref()]))
                }
                Some(value) if self.phase == 1 => {
                    self.phase = 2;
                    Self::credit(fill_owner_vec::<String>(value.compatible_with.capacity()))
                }
                Some(value) => match value.compatible_with.get(self.inner) {
                    Some(entry) => {
                        self.inner += 1;
                        Self::credit(fill_owner_strings([Some(entry)]))
                    }
                    None => {
                        self.index += 1;
                        self.inner = 0;
                        self.phase = 0;
                        FillOwnerCensusUnit::Advance
                    }
                },
                None => {
                    self.section = 4;
                    self.index = 0;
                    self.phase = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            4 => {
                self.section = 5;
                Self::credit(fill_fixed_vec_backing_credit(&fill.catalogs.cables))
            }
            5 => match fill.catalogs.cables.get(self.index) {
                Some(value) => {
                    self.index += 1;
                    Self::credit(fill_owner_strings([Some(&value.id), value.default_attraction_kind.as_ref()]))
                }
                None => {
                    self.section = 6;
                    self.index = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            6 => {
                self.section = 7;
                Self::credit(fill_fixed_vec_backing_credit(&fill.kind_compatibility))
            }
            7 => match fill.kind_compatibility.get(self.index) {
                Some(value) => {
                    self.index += 1;
                    Self::credit(fill_owner_strings([Some(&value.source), Some(&value.target), value.specificity.as_ref()]))
                }
                None => self.finish_field(),
            },
            _ => self.finish_field(),
        }
    }

    fn weight_mesh_unit(&mut self, fill: &FillBuilder) -> FillOwnerCensusUnit {
        match self.section {
            0 => {
                self.section = 1;
                Self::credit(fill_owner_collection(fill.weights.object_weights.len()))
            }
            1 => match fill.weights.object_weights.keys().nth(self.index) {
                Some(key) => {
                    self.index += 1;
                    let Some(mut credit) = fill_owner_strings([Some(key)]) else { return FillOwnerCensusUnit::Rejected };
                    credit.items += 1;
                    FillOwnerCensusUnit::Credit(credit)
                }
                None => {
                    self.section = 2;
                    self.index = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            2 => {
                self.section = 3;
                Self::credit(fill_owner_collection(fill.weights.vortex_weights.len()))
            }
            3 => match fill.weights.vortex_weights.keys().nth(self.index) {
                Some(key) => {
                    self.index += 1;
                    let Some(mut credit) = fill_owner_strings([Some(key)]) else { return FillOwnerCensusUnit::Rejected };
                    credit.items += 1;
                    FillOwnerCensusUnit::Credit(credit)
                }
                None => {
                    self.section = 4;
                    self.index = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            4 => {
                self.section = 6;
                FillOwnerCensusUnit::Advance
            }
            6 => {
                self.section = 7;
                Self::credit(fill_owner_collection(fill.meshes.len()))
            }
            7 => match fill.meshes.iter().nth(self.index) {
                Some((key, body)) if self.phase == 0 => {
                    self.phase = 1;
                    let Some(mut credit) = fill_owner_strings([Some(key)]) else { return FillOwnerCensusUnit::Rejected };
                    let Some((items, bytes)) = body.retained_parts_backing_credit() else { return FillOwnerCensusUnit::Rejected };
                    credit.items = credit.items.saturating_add(items).saturating_add(1);
                    credit.bytes = credit.bytes.saturating_add(bytes);
                    FillOwnerCensusUnit::Credit(credit)
                }
                Some((_, body)) => match body.retained_part_credit(self.inner) {
                    Some((items, bytes)) => {
                        self.inner += 1;
                        FillOwnerCensusUnit::Credit(FillBuilderOwnerCredit { items, bytes })
                    }
                    None if self.inner < body.parts.len() => FillOwnerCensusUnit::Rejected,
                    None => {
                        self.index += 1;
                        self.inner = 0;
                        self.phase = 0;
                        FillOwnerCensusUnit::Advance
                    }
                },
                None => self.finish_field(),
            },
            _ => self.finish_field(),
        }
    }

    fn target_credit(value: &BrushFillVortexTarget) -> Option<FillBuilderOwnerCredit> {
        fill_owner_strings([Some(&value.full_id), Some(&value.object_id), value.object_kind.as_ref(), value.vortex_kind.as_ref()])
    }

    fn target_unit(&mut self, fill: &FillBuilder) -> FillOwnerCensusUnit {
        match self.section {
            0 => {
                self.section = 1;
                Self::credit(fill_owner_vec::<BrushFillVortexTarget>(fill.targets.capacity()))
            }
            1 => match fill.targets.get(self.index) {
                Some(value) => {
                    self.index += 1;
                    Self::credit(Self::target_credit(value))
                }
                None => {
                    self.section = 2;
                    self.index = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            2 => {
                self.section = 3;
                Self::credit(fill_owner_collection(fill.blocked_vortex_ids.len()))
            }
            3 => match fill.blocked_vortex_ids.iter().nth(self.index) {
                Some(value) => {
                    self.index += 1;
                    let Some(mut credit) = fill_owner_strings([Some(value)]) else { return FillOwnerCensusUnit::Rejected };
                    credit.items += 1;
                    FillOwnerCensusUnit::Credit(credit)
                }
                None => {
                    self.section = 4;
                    self.index = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            4 => {
                self.section = 5;
                Self::credit(fill_owner_vec::<BrushFillVortexTarget>(fill.seed_targets.capacity()))
            }
            5 => match fill.seed_targets.get(self.index) {
                Some(value) => {
                    self.index += 1;
                    Self::credit(Self::target_credit(value))
                }
                None => {
                    self.section = 6;
                    self.index = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            6 => {
                self.section = 7;
                Self::credit(fill_owner_vec::<BrushFillVortexTarget>(fill.frontier_targets.capacity()))
            }
            7 => match fill.frontier_targets.get(self.index) {
                Some(value) => {
                    self.index += 1;
                    Self::credit(Self::target_credit(value))
                }
                None => self.finish_field(),
            },
            _ => self.finish_field(),
        }
    }

    fn target_weight_unit(&mut self, fill: &FillBuilder) -> FillOwnerCensusUnit {
        let credit = match self.section {
            0 => fill_owner_vec::<f64>(fill.seed_target_weights.capacity()),
            1 => fill_owner_vec::<f64>(fill.frontier_target_weights.capacity()),
            2 => fill_owner_vec::<f64>(fill.seed_target_tree.capacity()),
            3 => fill_owner_vec::<f64>(fill.frontier_target_tree.capacity()),
            _ => return self.finish_field(),
        };
        self.section += 1;
        Self::credit(credit)
    }

    fn candidate_unit(&mut self, fill: &FillBuilder) -> FillOwnerCensusUnit {
        match self.section {
            0 => {
                self.section = 1;
                Self::credit(fill_owner_vec::<BrushCompatibleCandidate>(fill.candidates.capacity()))
            }
            1 => match fill.candidates.get(self.index) {
                Some(value) => {
                    self.index += 1;
                    Self::credit(fill_owner_strings([Some(&value.object_kind_id)]))
                }
                None => {
                    self.section = 2;
                    self.index = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            2 => {
                self.section = 3;
                Self::credit(fill_owner_collection(fill.candidate_seen.len()))
            }
            3 => match fill.candidate_seen.iter().nth(self.index) {
                Some(value) => {
                    self.index += 1;
                    let Some(mut credit) = fill_owner_strings([Some(value)]) else { return FillOwnerCensusUnit::Rejected };
                    credit.items += 1;
                    FillOwnerCensusUnit::Credit(credit)
                }
                None => {
                    self.section = 4;
                    self.index = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            4 => {
                self.section = 5;
                Self::credit(fill_owner_vec::<BrushCompatibleCandidate>(fill.candidate_raw.capacity()))
            }
            5 => match fill.candidate_raw.get(self.index) {
                Some(value) => {
                    self.index += 1;
                    Self::credit(fill_owner_strings([Some(&value.object_kind_id)]))
                }
                None => {
                    self.section = 6;
                    self.index = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            6 => {
                self.section = 7;
                Self::credit(fill_owner_collection(fill.candidate_cross.len()))
            }
            7 => match fill.candidate_cross.iter().nth(self.index) {
                Some((key, value)) => {
                    self.index += 1;
                    let Some(mut credit) = fill_owner_strings([Some(key), Some(&value.object_kind_id)]) else { return FillOwnerCensusUnit::Rejected };
                    credit.items += 1;
                    FillOwnerCensusUnit::Credit(credit)
                }
                None => self.finish_field(),
            },
            _ => self.finish_field(),
        }
    }

    fn candidate_order_unit(&mut self, fill: &FillBuilder) -> FillOwnerCensusUnit {
        match self.section {
            0 => {
                self.section = 1;
                Self::credit(fill_owner_collection(fill.candidate_same.len()))
            }
            1 => match fill.candidate_same.iter().nth(self.index) {
                Some((key, value)) => {
                    self.index += 1;
                    let Some(mut credit) = fill_owner_strings([Some(key), Some(&value.object_kind_id)]) else { return FillOwnerCensusUnit::Rejected };
                    credit.items += 1;
                    FillOwnerCensusUnit::Credit(credit)
                }
                None => {
                    self.section = 2;
                    self.index = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            2 => {
                self.section = 3;
                Self::credit(fill_owner_vec::<BrushCompatibleCandidate>(fill.candidate_same_sorted.capacity()))
            }
            3 => match fill.candidate_same_sorted.get(self.index) {
                Some(value) => {
                    self.index += 1;
                    Self::credit(fill_owner_strings([Some(&value.object_kind_id)]))
                }
                None => {
                    self.section = 4;
                    self.index = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            4 => {
                self.section = 5;
                Self::credit(fill_owner_vec::<f64>(fill.candidate_same_weights.capacity()))
            }
            5 => {
                self.section = 6;
                Self::credit(fill_owner_vec::<f64>(fill.candidate_same_tree.capacity()))
            }
            _ => self.finish_field(),
        }
    }

    fn preview_state_unit(&mut self, value: &BrushPreviewState, root: FillDslOwnerRoot) -> Option<FillOwnerCensusUnit> {
        match self.phase {
            0 => {
                self.phase = 1;
                Some(Self::credit(fill_owner_strings([Some(&value.target_vortex_full_id), Some(&value.object_kind_id), Some(&value.mesh_url)])))
            }
            1 if value.scale.is_some() => Some(self.start_dsl(root)),
            _ => {
                self.phase = 0;
                None
            }
        }
    }

    fn pending_unit(&mut self, fill: &FillBuilder) -> FillOwnerCensusUnit {
        match self.section {
            0 => {
                self.section = 1;
                fill.current_target.as_ref().map_or(FillOwnerCensusUnit::Advance, |value| Self::credit(Self::target_credit(value)))
            }
            1 => match fill.current_preview.as_ref() {
                Some(value) => match self.preview_state_unit(value, FillDslOwnerRoot::CurrentPreview) {
                    Some(unit) => unit,
                    None => {
                        self.section = 2;
                        FillOwnerCensusUnit::Advance
                    }
                },
                None => {
                    self.section = 2;
                    FillOwnerCensusUnit::Advance
                }
            },
            2 => match fill.pending_payload.as_ref() {
                Some(value) => match self.payload_unit(value, FillDslOwnerRoot::PendingPayload) {
                    Some(unit) => unit,
                    None => {
                        self.section = 3;
                        FillOwnerCensusUnit::Advance
                    }
                },
                None => {
                    self.section = 3;
                    FillOwnerCensusUnit::Advance
                }
            },
            3 => match fill.pending_object.as_ref() {
                Some(value) => match self.fixture_object_unit(value, FillDslOwnerRoot::PendingObject) {
                    Some(unit) => unit,
                    None => {
                        self.section = 4;
                        FillOwnerCensusUnit::Advance
                    }
                },
                None => {
                    self.section = 4;
                    FillOwnerCensusUnit::Advance
                }
            },
            4 => {
                self.section = 5;
                fill.pending_attraction.as_ref().map_or(FillOwnerCensusUnit::Advance, |value| Self::credit(fill_owner_strings([Some(&value.id), Some(&value.attracting), Some(&value.attracted)])))
            }
            _ => self.finish_field(),
        }
    }

    fn preview_unit(&mut self, fill: &FillBuilder) -> FillOwnerCensusUnit {
        let preview = &fill.preview;
        match self.section {
            0 => {
                self.section = 1;
                Self::credit(fill_owner_strings([Some(&preview.stage), preview.target_vortex_full_id.as_ref(), preview.candidate_object_kind_id.as_ref(), preview.current_pair_object_id.as_ref(), preview.rejection_reason.as_ref()]))
            }
            1 => match preview.candidate_ghost.as_ref() {
                Some(value) => match self.preview_state_unit(value, FillDslOwnerRoot::PreviewGhost) {
                    Some(unit) => unit,
                    None => {
                        self.section = 2;
                        FillOwnerCensusUnit::Advance
                    }
                },
                None => {
                    self.section = 2;
                    FillOwnerCensusUnit::Advance
                }
            },
            2 => match preview.candidate_page.get(self.index).and_then(Option::as_ref) {
                Some(value) => {
                    self.index += 1;
                    Self::credit(fill_owner_strings([Some(value)]))
                }
                None => {
                    self.section = 3;
                    self.index = 0;
                    FillOwnerCensusUnit::Advance
                }
            },
            3 => self.finish_field(),
            _ => self.finish_field(),
        }
    }

    fn final_unit(&mut self, fill: &FillBuilder) -> FillOwnerCensusUnit {
        if self.section == 0 {
            self.section = 1;
            return Self::credit(fill_owner_strings([fill.last_rejection.as_ref()]));
        }
        match fill.spatial_index.census_one_owner(&mut self.spatial) {
            CollisionIndexOwnerCensusStep::Pending { items, bytes } => FillOwnerCensusUnit::Credit(FillBuilderOwnerCredit { items, bytes }),
            CollisionIndexOwnerCensusStep::Complete => self.finish_field(),
            CollisionIndexOwnerCensusStep::Rejected => FillOwnerCensusUnit::Rejected,
        }
    }
}

pub(crate) struct FillBuilderRetirementCursor {
    fill: Option<FillBuilder>,
    field: u8,
    current: Option<FillRetiredOwner>,
}

enum FillRetiredOwner {
    String(String),
    FixtureObject(FixtureObject),
    Attraction(AttractionProps),
    WorldVolume(WorldVolumeProps),
    Payload(BrushPlacePayload),
    Placed(PlacedCollisionEntry),
    Candidate(BrushCompatibleCandidate),
    Target(BrushFillVortexTarget),
    PreviewState(BrushPreviewState),
    ObjectKind(ObjectKind),
    VortexKind(VortexKindCatalog),
    CableKind(CableKindCatalog),
    Compat(KindCompatEntry),
    CandidateCache(String, Vec<BrushCompatibleCandidate>),
    CandidateMap(String, BrushCompatibleCandidate),
    Mesh(String, CollisionBody),
    Spatial(CollisionIndexRejectedOwner),
}

fn retire_string(value: &mut String) -> bool {
    if value.capacity() == 0 {
        return true;
    }
    drop(std::mem::take(value));
    false
}

fn retire_option_string(value: &mut Option<String>) -> bool {
    let Some(string) = value.as_mut() else { return true };
    if !retire_string(string) {
        return false;
    }
    value.take();
    false
}

fn retire_dsl_one(value: &mut dsl::DslValue, depth: usize) -> bool {
    if depth > 16 {
        return false;
    }
    match value {
        dsl::DslValue::String(string) => {
            if !retire_string(string) {
                return false;
            }
            *value = dsl::DslValue::Null;
            false
        }
        dsl::DslValue::Array(values) => {
            if let Some(child) = values.last_mut() {
                if !retire_dsl_one(child, depth + 1) {
                    return false;
                }
                values.pop();
                return false;
            }
            if values.capacity() != 0 {
                drop(std::mem::take(values));
                return false;
            }
            *value = dsl::DslValue::Null;
            false
        }
        dsl::DslValue::Object(values) => {
            if let Some((key, child)) = values.last_mut() {
                if !retire_string(key) || !retire_dsl_one(child, depth + 1) {
                    return false;
                }
                values.pop();
                return false;
            }
            if values.capacity() != 0 {
                drop(std::mem::take(values));
                return false;
            }
            *value = dsl::DslValue::Null;
            false
        }
        dsl::DslValue::Null | dsl::DslValue::Bool(_) | dsl::DslValue::Number(_) => true,
    }
}

fn retire_option_dsl(value: &mut Option<dsl::DslValue>) -> bool {
    let Some(dsl) = value.as_mut() else { return true };
    if !retire_dsl_one(dsl, 0) {
        return false;
    }
    value.take();
    false
}

fn retire_fixture_object(value: &mut FixtureObject) -> bool {
    if !retire_string(&mut value.id) || !retire_option_string(&mut value.object_kind) || !retire_option_string(&mut value.mesh_url) || !retire_option_dsl(&mut value.scale) {
        return false;
    }
    if let Some(vortex) = value.vortices.last_mut() {
        if !retire_string(&mut vortex.id) || !retire_option_string(&mut vortex.vortex_kind) {
            return false;
        }
        value.vortices.pop();
        return false;
    }
    if value.vortices.capacity() != 0 {
        drop(std::mem::take(&mut value.vortices));
        return false;
    }
    true
}

fn retire_attraction(value: &mut AttractionProps) -> bool {
    retire_string(&mut value.id) && retire_string(&mut value.attracting) && retire_string(&mut value.attracted)
}

fn retire_world_volume(value: &mut WorldVolumeProps) -> bool {
    retire_string(&mut value.id) && retire_option_dsl(&mut value.scale)
}

fn retire_payload(value: &mut BrushPlacePayload) -> bool {
    retire_string(&mut value.target_vortex_full_id) && retire_string(&mut value.object_kind_id) && retire_option_dsl(&mut value.scale)
}

fn retire_candidate(value: &mut BrushCompatibleCandidate) -> bool {
    retire_string(&mut value.object_kind_id)
}

fn retire_target(value: &mut BrushFillVortexTarget) -> bool {
    retire_string(&mut value.full_id) && retire_string(&mut value.object_id) && retire_option_string(&mut value.object_kind) && retire_option_string(&mut value.vortex_kind)
}

fn retire_preview_state(value: &mut BrushPreviewState) -> bool {
    retire_string(&mut value.target_vortex_full_id) && retire_string(&mut value.object_kind_id) && retire_string(&mut value.mesh_url) && retire_option_dsl(&mut value.scale)
}

fn retire_fill_preview(value: &mut FillBuildPreview) -> bool {
    if !retire_string(&mut value.stage) || !retire_option_string(&mut value.target_vortex_full_id) || !retire_option_string(&mut value.candidate_object_kind_id) || value.candidate_ghost.as_mut().is_some_and(|preview| !retire_preview_state(preview)) {
        return false;
    }
    if value.candidate_ghost.is_some() {
        value.candidate_ghost.take();
        return false;
    }
    for candidate in &mut value.candidate_page {
        if let Some(string) = candidate.as_mut() {
            if !retire_string(string) {
                return false;
            }
            candidate.take();
            return false;
        }
    }
    if !retire_option_string(&mut value.current_pair_object_id) || !retire_option_string(&mut value.rejection_reason) {
        return false;
    }
    true
}

fn retire_object_kind(value: &mut ObjectKind) -> bool {
    if !retire_string(&mut value.id) || !retire_option_dsl(&mut value.scale) {
        return false;
    }
    if let Some(representation) = value.representations.last_mut() {
        if !retire_string(&mut representation.id)
            || !retire_string(&mut representation.name)
            || !retire_string(&mut representation.url)
            || !retire_string(&mut representation.mime)
            || !retire_option_string(&mut representation.lod)
            || !retire_string(&mut representation.description)
        {
            return false;
        }
        if let Some(tag) = representation.tags.last_mut() {
            if !retire_string(tag) {
                return false;
            }
            representation.tags.pop();
            return false;
        }
        if representation.tags.capacity() != 0 {
            drop(std::mem::take(&mut representation.tags));
            return false;
        }
        value.representations.pop();
        return false;
    }
    if value.representations.capacity() != 0 {
        drop(std::mem::take(&mut value.representations));
        return false;
    }
    if let Some(vortex) = value.vortices.last_mut() {
        if !retire_string(&mut vortex.id) || !retire_string(&mut vortex.name) || !retire_string(&mut vortex.label) || !retire_string(&mut vortex.description) || !retire_string(&mut vortex.icon) || !retire_option_string(&mut vortex.vortex_kind) {
            return false;
        }
        value.vortices.pop();
        return false;
    }
    if value.vortices.capacity() != 0 {
        drop(std::mem::take(&mut value.vortices));
        return false;
    }
    true
}

fn retire_retained_owner(owner: &mut FillRetiredOwner) -> bool {
    match owner {
        FillRetiredOwner::String(value) => retire_string(value),
        FillRetiredOwner::FixtureObject(value) => retire_fixture_object(value),
        FillRetiredOwner::Attraction(value) => retire_attraction(value),
        FillRetiredOwner::WorldVolume(value) => retire_world_volume(value),
        FillRetiredOwner::Payload(value) => retire_payload(value),
        FillRetiredOwner::Placed(value) => retire_string(&mut value.object_id) && retire_string(&mut value.mesh_url),
        FillRetiredOwner::Candidate(value) => retire_candidate(value),
        FillRetiredOwner::Target(value) => retire_target(value),
        FillRetiredOwner::PreviewState(value) => retire_preview_state(value),
        FillRetiredOwner::ObjectKind(value) => retire_object_kind(value),
        FillRetiredOwner::VortexKind(value) => {
            if !retire_string(&mut value.id)
                || !retire_option_string(&mut value.code)
                || !retire_option_string(&mut value.label)
                || !retire_string(&mut value.description)
                || !retire_string(&mut value.icon)
                || !retire_string(&mut value.color)
                || !retire_option_string(&mut value.default_cable_kind)
            {
                return false;
            }
            if let Some(entry) = value.compatible_with.last_mut() {
                if !retire_string(entry) {
                    return false;
                }
                value.compatible_with.pop();
                return false;
            }
            if value.compatible_with.capacity() != 0 {
                drop(std::mem::take(&mut value.compatible_with));
                return false;
            }
            true
        }
        FillRetiredOwner::CableKind(value) => retire_string(&mut value.id) && retire_option_string(&mut value.default_attraction_kind),
        FillRetiredOwner::Compat(value) => retire_string(&mut value.source) && retire_string(&mut value.target) && retire_option_string(&mut value.specificity),
        FillRetiredOwner::CandidateCache(key, values) => {
            if !retire_string(key) {
                return false;
            }
            if let Some(value) = values.last_mut() {
                if !retire_candidate(value) {
                    return false;
                }
                values.pop();
                return false;
            }
            if values.capacity() != 0 {
                drop(std::mem::take(values));
                return false;
            }
            true
        }
        FillRetiredOwner::CandidateMap(key, value) => retire_string(key) && retire_candidate(value),
        FillRetiredOwner::Mesh(key, body) => {
            if !retire_string(key) {
                return false;
            }
            if body.parts.pop().is_some() {
                return false;
            }
            if body.parts.capacity() != 0 {
                drop(std::mem::take(&mut body.parts));
                return false;
            }
            true
        }
        FillRetiredOwner::Spatial(owner) => owner.retire_one(),
    }
}

fn release_vec_backing<T>(values: &mut Vec<T>) -> bool {
    if values.capacity() == 0 {
        return false;
    }
    debug_assert!(values.is_empty());
    drop(std::mem::take(values));
    true
}

fn take_fixture_owner(value: &mut FixedFixtureOwner, current: &mut Option<FillRetiredOwner>) -> bool {
    if let Some(value) = value.objects.pop() {
        *current = Some(FillRetiredOwner::FixtureObject(value));
        return true;
    }
    if value.objects.retire_backing() {
        return true;
    }
    if let Some(value) = value.attractions.pop() {
        *current = Some(FillRetiredOwner::Attraction(value));
        return true;
    }
    if value.attractions.retire_backing() {
        return true;
    }
    if let Some(value) = value.target_volumes.pop() {
        *current = Some(FillRetiredOwner::WorldVolume(value));
        return true;
    }
    value.target_volumes.retire_backing()
}

fn take_sequence_owner(fill: &mut FillBuilder, current: &mut Option<FillRetiredOwner>) -> bool {
    if let Some(value) = fill.sequence.pop() {
        *current = Some(FillRetiredOwner::Payload(value));
        return true;
    }
    if release_vec_backing(&mut fill.sequence) {
        return true;
    }
    if let Some(value) = fill.appended_objects.pop() {
        *current = Some(FillRetiredOwner::FixtureObject(value));
        return true;
    }
    if release_vec_backing(&mut fill.appended_objects) {
        return true;
    }
    if let Some(value) = fill.appended_attractions.pop() {
        *current = Some(FillRetiredOwner::Attraction(value));
        return true;
    }
    if release_vec_backing(&mut fill.appended_attractions) {
        return true;
    }
    if let Some(value) = fill.placed.pop() {
        *current = Some(FillRetiredOwner::Placed(value));
        return true;
    }
    release_vec_backing(&mut fill.placed)
}

fn take_lookup_owner(fill: &mut FillBuilder, current: &mut Option<FillRetiredOwner>) -> bool {
    if let Some((key, _)) = fill.placed_lookup.pop_first() {
        *current = Some(FillRetiredOwner::String(key));
        return true;
    }
    if let Some((key, values)) = fill.candidate_cache.pop_first() {
        *current = Some(FillRetiredOwner::CandidateCache(key, values));
        return true;
    }
    if let Some(value) = fill.seed_object_ids.pop_first() {
        *current = Some(FillRetiredOwner::String(value));
        return true;
    }
    false
}

fn take_catalog_owner(fill: &mut FillBuilder, current: &mut Option<FillRetiredOwner>) -> bool {
    if let Some(value) = fill.catalogs.objects.pop() {
        *current = Some(FillRetiredOwner::ObjectKind(value));
        return true;
    }
    if fill.catalogs.objects.retire_backing() {
        return true;
    }
    if let Some(value) = fill.catalogs.vortices.pop() {
        *current = Some(FillRetiredOwner::VortexKind(value));
        return true;
    }
    if fill.catalogs.vortices.retire_backing() {
        return true;
    }
    if let Some(value) = fill.catalogs.cables.pop() {
        *current = Some(FillRetiredOwner::CableKind(value));
        return true;
    }
    if fill.catalogs.cables.retire_backing() {
        return true;
    }
    if let Some(value) = fill.kind_compatibility.pop() {
        *current = Some(FillRetiredOwner::Compat(value));
        return true;
    }
    fill.kind_compatibility.retire_backing()
}

fn take_weight_map_owner<const N: usize>(values: &mut FixedOwnerMap<String, f64, N>, current: &mut Option<FillRetiredOwner>) -> bool {
    if let Some((key, _)) = values.pop_first() {
        *current = Some(FillRetiredOwner::String(key));
        return true;
    }
    false
}

fn take_weight_mesh_owner(fill: &mut FillBuilder, current: &mut Option<FillRetiredOwner>) -> bool {
    if take_weight_map_owner(&mut fill.weights.object_weights, current) || take_weight_map_owner(&mut fill.weights.vortex_weights, current) {
        return true;
    }
    if let Some((key, body)) = fill.meshes.pop_first() {
        *current = Some(FillRetiredOwner::Mesh(key, body));
        return true;
    }
    false
}

fn take_target_owner(fill: &mut FillBuilder, current: &mut Option<FillRetiredOwner>) -> bool {
    if let Some(value) = fill.targets.pop() {
        *current = Some(FillRetiredOwner::Target(value));
        return true;
    }
    if release_vec_backing(&mut fill.targets) {
        return true;
    }
    if let Some(value) = fill.blocked_vortex_ids.pop_first() {
        *current = Some(FillRetiredOwner::String(value));
        return true;
    }
    if let Some(value) = fill.seed_targets.pop() {
        *current = Some(FillRetiredOwner::Target(value));
        return true;
    }
    if release_vec_backing(&mut fill.seed_targets) {
        return true;
    }
    if let Some(value) = fill.frontier_targets.pop() {
        *current = Some(FillRetiredOwner::Target(value));
        return true;
    }
    release_vec_backing(&mut fill.frontier_targets)
}

fn take_candidate_owner(fill: &mut FillBuilder, current: &mut Option<FillRetiredOwner>) -> bool {
    if let Some(value) = fill.candidates.pop() {
        *current = Some(FillRetiredOwner::Candidate(value));
        return true;
    }
    if release_vec_backing(&mut fill.candidates) {
        return true;
    }
    if let Some(value) = fill.candidate_seen.pop_first() {
        *current = Some(FillRetiredOwner::String(value));
        return true;
    }
    if let Some(value) = fill.candidate_raw.pop() {
        *current = Some(FillRetiredOwner::Candidate(value));
        return true;
    }
    if release_vec_backing(&mut fill.candidate_raw) {
        return true;
    }
    if let Some((key, value)) = fill.candidate_cross.pop_first() {
        *current = Some(FillRetiredOwner::CandidateMap(key, value));
        return true;
    }
    false
}

fn take_candidate_order_owner(fill: &mut FillBuilder, current: &mut Option<FillRetiredOwner>) -> bool {
    if let Some((key, value)) = fill.candidate_same.pop_first() {
        *current = Some(FillRetiredOwner::CandidateMap(key, value));
        return true;
    }
    if let Some(value) = fill.candidate_same_sorted.pop() {
        *current = Some(FillRetiredOwner::Candidate(value));
        return true;
    }
    if release_vec_backing(&mut fill.candidate_same_sorted) {
        return true;
    }
    for values in [&mut fill.candidate_same_weights, &mut fill.candidate_same_tree] {
        if values.pop().is_some() || release_vec_backing(values) {
            return true;
        }
    }
    false
}

fn retire_fixed_collection_backing(fill: &mut FillBuilder) -> bool {
    fill.placed_lookup.retire_backing()
        || fill.candidate_cache.retire_backing()
        || fill.seed_object_ids.retire_backing()
        || fill.weights.object_weights.retire_backing()
        || fill.weights.vortex_weights.retire_backing()
        || fill.meshes.retire_backing()
        || fill.blocked_vortex_ids.retire_backing()
        || fill.candidate_seen.retire_backing()
        || fill.candidate_cross.retire_backing()
        || fill.candidate_same.retire_backing()
}

fn take_target_weight_owner(fill: &mut FillBuilder) -> bool {
    for values in [&mut fill.seed_target_weights, &mut fill.frontier_target_weights, &mut fill.seed_target_tree, &mut fill.frontier_target_tree] {
        if values.pop().is_some() || release_vec_backing(values) {
            return true;
        }
    }
    false
}

fn fixture_terminal_owners_empty(value: &FixedFixtureOwner) -> bool {
    value.objects.terminal_owners_empty() && value.attractions.terminal_owners_empty() && value.target_volumes.terminal_owners_empty()
}

fn preview_terminal_owners_empty(value: &FillBuildPreview) -> bool {
    value.stage.capacity() == 0
        && value.target_vortex_full_id.is_none()
        && value.candidate_object_kind_id.is_none()
        && value.candidate_ghost.is_none()
        && value.current_pair_object_id.is_none()
        && value.rejection_reason.is_none()
        && value.candidate_page.iter().all(Option::is_none)
}

impl FillBuilderRetirementCursor {
    pub(crate) fn new(fill: FillBuilder) -> Self {
        Self { fill: Some(fill), field: 0, current: None }
    }

    pub(crate) fn retire_one(&mut self) -> bool {
        if let Some(current) = self.current.as_mut() {
            if retire_retained_owner(current) {
                self.current = None;
            }
            return false;
        }
        let Some(fill) = self.fill.as_mut() else {
            return true;
        };
        let retired = match self.field {
            0 => take_fixture_owner(&mut fill.base, &mut self.current),
            1 => false,
            2 => take_sequence_owner(fill, &mut self.current),
            3 => take_lookup_owner(fill, &mut self.current),
            4 => take_catalog_owner(fill, &mut self.current),
            5 => take_weight_mesh_owner(fill, &mut self.current),
            6 => take_target_owner(fill, &mut self.current),
            7 => take_target_weight_owner(fill),
            8 => take_candidate_owner(fill, &mut self.current),
            9 => take_candidate_order_owner(fill, &mut self.current),
            10 => match fill.broad_phase_query.as_mut() {
                Some(query) => {
                    if query.retire_one_owner() {
                        fill.broad_phase_query.take();
                    }
                    true
                }
                None => false,
            },
            11 => fill.pending_payload.take().is_some_and(|value| {
                self.current = Some(FillRetiredOwner::Payload(value));
                true
            }),
            12 => fill.pending_object.take().is_some_and(|value| {
                self.current = Some(FillRetiredOwner::FixtureObject(value));
                true
            }),
            13 => fill.pending_attraction.take().is_some_and(|value| {
                self.current = Some(FillRetiredOwner::Attraction(value));
                true
            }),
            14 => fill.current_target.take().is_some_and(|value| {
                self.current = Some(FillRetiredOwner::Target(value));
                true
            }),
            15 => fill.current_preview.take().is_some_and(|value| {
                self.current = Some(FillRetiredOwner::PreviewState(value));
                true
            }),
            16 => fill.last_rejection.take().is_some_and(|value| {
                self.current = Some(FillRetiredOwner::String(value));
                true
            }),
            17 => fill.collision.take().is_some(),
            18 => !fill.preview_json.close_step(),
            19 => !retire_fill_preview(&mut fill.preview),
            20 => match fill.fixed_rejection.as_mut() {
                Some(rejected) => {
                    if retire_retained_owner(rejected) {
                        fill.fixed_rejection.take();
                    }
                    true
                }
                None => false,
            },
            21 => retire_fixed_collection_backing(fill),
            22 => !fill.spatial_index.retire_one_owner(),
            23 if fill.collection_over_capacity => {
                fill.collection_over_capacity = false;
                true
            }
            23 => false,
            24 => match fill.preparation_spatial.as_mut() {
                Some(mutation) => {
                    if mutation.retire_one_owner() {
                        fill.preparation_spatial.take();
                    }
                    true
                }
                None => false,
            },
            25 => match fill.pending_spatial.as_mut() {
                Some(mutation) => {
                    if mutation.retire_one_owner() {
                        fill.pending_spatial.take();
                    }
                    true
                }
                None => false,
            },
            26 => match fill.tail_removal.as_mut() {
                Some(removal) => {
                    if removal.retire_one_owner() {
                        fill.tail_removal.take();
                    }
                    true
                }
                None => false,
            },
            27 if fill.preparation_roots.take().is_some() => true,
            27 => false,
            28 if fill.preparation_capacity_refusal.take().is_some() => true,
            28 => false,
            _ => {
                if !fill.terminal_owners_empty() {
                    return false;
                }
                let shell = self.fill.take().expect("terminal-empty builder shell");
                drop(shell);
                return self.fill.is_none() && self.current.is_none();
            }
        };
        if !retired {
            self.field += 1;
        }
        false
    }
}

impl FillBuilder {
    #[cfg(test)]
    fn preparation_refusal_owner_for_test(&self) -> Option<(&'static str, usize, String, Option<f64>)> {
        let refusal = self.preparation_capacity_refusal?;
        let roots = self.preparation_roots.as_ref()?;
        let (owner, weight) = match refusal.branch {
            PreparationCapacityBranch::FixtureObjects => (roots.scene.fixture.objects.get(refusal.omitted_index)?.id.clone(), None),
            PreparationCapacityBranch::FixtureAttractions => (roots.scene.fixture.attractions.get(refusal.omitted_index)?.id.clone(), None),
            PreparationCapacityBranch::FixtureTargetVolumes => (roots.scene.fixture.target_volumes.get(refusal.omitted_index)?.id.clone(), None),
            PreparationCapacityBranch::Meshes => (roots.meshes.keys().nth(refusal.omitted_index)?.clone(), None),
            PreparationCapacityBranch::CatalogObjects => (roots.scene.kind_catalogs.as_ref()?.objects.get(refusal.omitted_index)?.id.clone(), None),
            PreparationCapacityBranch::CatalogVortices => (roots.scene.kind_catalogs.as_ref()?.vortices.get(refusal.omitted_index)?.id.clone(), None),
            PreparationCapacityBranch::CatalogCables => (roots.scene.kind_catalogs.as_ref()?.cables.get(refusal.omitted_index)?.id.clone(), None),
            PreparationCapacityBranch::KindCompatibility => (roots.scene.kind_compatibility.get(refusal.omitted_index)?.source.clone(), None),
            PreparationCapacityBranch::ObjectWeights => {
                let (key, value) = roots.scene.weights.object_weights.iter().nth(refusal.omitted_index)?;
                (key.clone(), Some(*value))
            }
            PreparationCapacityBranch::VortexWeights => {
                let (key, value) = roots.scene.weights.vortex_weights.iter().nth(refusal.omitted_index)?;
                (key.clone(), Some(*value))
            }
        };
        Some((refusal.branch.label(), refusal.omitted_index, owner, weight))
    }

    #[cfg(test)]
    pub(crate) fn inject_nested_owner_page_plus_one_for_test(&mut self) {
        let mut owner = String::with_capacity(FILL_BUILDER_OWNER_PAGE_BYTES + 1);
        owner.push_str("nested-owner");
        self.catalogs.objects[0].representations[0].tags.push(owner);
    }

    #[cfg(test)]
    pub(crate) fn fixed_backing_witness_for_test(&self) -> [(usize, usize, usize); 13] {
        let mut witness = [
            (self.placed_lookup.backing_ptr().map_or(0, |pointer| pointer.cast::<()>() as usize), FixedOwnerMap::<String, usize, DOCUMENT_OBJECT_SLOTS>::page_bytes(), self.placed_lookup.len()),
            (self.candidate_cache.backing_ptr().map_or(0, |pointer| pointer.cast::<()>() as usize), FixedOwnerMap::<String, Vec<BrushCompatibleCandidate>>::page_bytes(), self.candidate_cache.len()),
            (self.seed_object_ids.backing_ptr().map_or(0, |pointer| pointer.cast::<()>() as usize), FixedOwnerMap::<String, (), DOCUMENT_OBJECT_SLOTS>::page_bytes(), self.seed_object_ids.len()),
            (self.weights.object_weights.backing_ptr().map_or(0, |pointer| pointer.cast::<()>() as usize), FixedOwnerMap::<String, f64, DOCUMENT_KIND_SLOTS>::page_bytes(), self.weights.object_weights.len()),
            (self.weights.vortex_weights.backing_ptr().map_or(0, |pointer| pointer.cast::<()>() as usize), FixedOwnerMap::<String, f64, DOCUMENT_KIND_SLOTS>::page_bytes(), self.weights.vortex_weights.len()),
            (self.meshes.backing_ptr().map_or(0, |pointer| pointer.cast::<()>() as usize), FixedOwnerMap::<String, CollisionBody, DOCUMENT_KIND_SLOTS>::page_bytes(), self.meshes.len()),
            (self.blocked_vortex_ids.backing_ptr().map_or(0, |pointer| pointer.cast::<()>() as usize), FixedOwnerMap::<String, (), DOCUMENT_VORTEX_SLOTS>::page_bytes(), self.blocked_vortex_ids.len()),
            (self.candidate_seen.backing_ptr().map_or(0, |pointer| pointer.cast::<()>() as usize), FixedOwnerMap::<String, (), DOCUMENT_CANDIDATE_SLOTS>::page_bytes(), self.candidate_seen.len()),
            (self.candidate_cross.backing_ptr().map_or(0, |pointer| pointer.cast::<()>() as usize), FixedOwnerMap::<String, BrushCompatibleCandidate, DOCUMENT_CANDIDATE_SLOTS>::page_bytes(), self.candidate_cross.len()),
            (self.candidate_same.backing_ptr().map_or(0, |pointer| pointer.cast::<()>() as usize), FixedOwnerMap::<String, BrushCompatibleCandidate, DOCUMENT_CANDIDATE_SLOTS>::page_bytes(), self.candidate_same.len()),
            (0, 0, 0),
            (0, 0, 0),
            (0, 0, 0),
        ];
        witness[10..].copy_from_slice(&self.spatial_index.fixed_backing_witness_for_test());
        witness
    }

    fn terminal_owners_empty(&self) -> bool {
        fixture_terminal_owners_empty(&self.base)
            && self.sequence.is_empty()
            && self.sequence.capacity() == 0
            && self.appended_objects.is_empty()
            && self.appended_objects.capacity() == 0
            && self.appended_attractions.is_empty()
            && self.appended_attractions.capacity() == 0
            && self.placed.is_empty()
            && self.placed.capacity() == 0
            && self.placed_lookup.is_empty()
            && self.candidate_cache.is_empty()
            && self.seed_object_ids.is_empty()
            && self.catalogs.objects.terminal_owners_empty()
            && self.catalogs.vortices.terminal_owners_empty()
            && self.catalogs.cables.terminal_owners_empty()
            && self.weights.object_weights.is_empty()
            && self.weights.vortex_weights.is_empty()
            && self.kind_compatibility.terminal_owners_empty()
            && self.meshes.is_empty()
            && self.spatial_index.terminal_owners_empty()
            && self.targets.is_empty()
            && self.targets.capacity() == 0
            && self.blocked_vortex_ids.is_empty()
            && self.seed_targets.is_empty()
            && self.seed_targets.capacity() == 0
            && self.frontier_targets.is_empty()
            && self.frontier_targets.capacity() == 0
            && self.seed_target_weights.is_empty()
            && self.seed_target_weights.capacity() == 0
            && self.frontier_target_weights.is_empty()
            && self.frontier_target_weights.capacity() == 0
            && self.seed_target_tree.is_empty()
            && self.seed_target_tree.capacity() == 0
            && self.frontier_target_tree.is_empty()
            && self.frontier_target_tree.capacity() == 0
            && self.current_target.is_none()
            && self.candidates.is_empty()
            && self.candidates.capacity() == 0
            && self.candidate_seen.is_empty()
            && self.candidate_raw.is_empty()
            && self.candidate_raw.capacity() == 0
            && self.candidate_cross.is_empty()
            && self.candidate_same.is_empty()
            && self.candidate_same_sorted.is_empty()
            && self.candidate_same_sorted.capacity() == 0
            && self.candidate_same_weights.is_empty()
            && self.candidate_same_weights.capacity() == 0
            && self.candidate_same_tree.is_empty()
            && self.candidate_same_tree.capacity() == 0
            && self.current_preview.is_none()
            && self.broad_phase_query.as_ref().is_none_or(CollisionQueryCursor::terminal_owners_empty)
            && self.collision.is_none()
            && self.pending_payload.is_none()
            && self.pending_object.is_none()
            && self.pending_attraction.is_none()
            && self.pending_spatial.is_none()
            && self.preparation_spatial.is_none()
            && self.tail_removal.is_none()
            && self.preparation_roots.is_none()
            && self.preparation_capacity_refusal.is_none()
            && self.last_rejection.is_none()
            && self.fixed_rejection.is_none()
            && !self.collection_over_capacity
            && self.placed_lookup.terminal_owners_empty()
            && self.candidate_cache.terminal_owners_empty()
            && self.seed_object_ids.terminal_owners_empty()
            && self.weights.object_weights.terminal_owners_empty()
            && self.weights.vortex_weights.terminal_owners_empty()
            && self.meshes.terminal_owners_empty()
            && self.blocked_vortex_ids.terminal_owners_empty()
            && self.candidate_seen.terminal_owners_empty()
            && self.candidate_cross.terminal_owners_empty()
            && self.candidate_same.terminal_owners_empty()
            && self.preview_json.terminal_owners_empty()
            && preview_terminal_owners_empty(&self.preview)
    }

    pub(crate) fn begin_preparation(roots: FillPreparationRoots, operation: Operation) -> Self {
        let seed = roots.scene.seed;
        let preparation_capacity_refusal = preparation_capacity_refusal(&roots);
        let rejection_reason = preparation_capacity_refusal.map(PreparationCapacityRefusal::diagnostic);
        Self {
            base: FixedFixtureOwner::new(),
            preparation_roots: Some(roots),
            preparation_cursor: 0,
            preparation_inner_cursor: 0,
            preparation_spatial: None,
            tail_removal: None,
            preparation_capacity_refusal,
            applied_count: 0,
            sequence: Vec::new(),
            appended_objects: Vec::new(),
            appended_attractions: Vec::new(),
            placed: Vec::new(),
            placed_lookup: FixedOwnerMap::new(),
            candidate_cache: FixedOwnerMap::new(),
            seed_object_ids: FixedOwnerSet::new(),
            rng_state: seed,
            stalled: false,
            max_count: FILL_COUNT_MAX,
            operation,
            stage: FillJobStage::PrepareFixture,
            preview: FillBuildPreview {
                operation: operation.operation.0,
                base_revision: operation.base_revision.0,
                registry_generation: operation.generation.0,
                sequence: 0,
                generation: operation.generation.0,
                stage: "prepare-fixture".into(),
                target_vortex_full_id: None,
                candidate_object_kind_id: None,
                candidate_ghost: None,
                current_pair_object_id: None,
                collision_count: 0,
                sample_cursor: 0,
                inside_both: 0,
                last_sample: None,
                candidate_page: std::array::from_fn(|_| None),
                truncated: false,
                rejection_reason,
                target_cursor: 0,
                candidate_cursor: 0,
                accepted_count: 0,
                total_count: FILL_COUNT_MAX,
                search_count: 0,
                rejected_count: 0,
            },
            preview_json: FillPreviewJsonCursor::default(),
            catalogs: FixedCatalogOwner::new(),
            weights: RetainedBrushKindWeights::new(),
            kind_compatibility: FixedOwnerVec::new(),
            host_rules: BrushHostRules::default(),
            overlap_budget: 0.0,
            meshes: FixedOwnerMap::new(),
            spatial_index: CollisionSpatialIndex::new(8.0),
            targets: Vec::new(),
            target_cursor: 0,
            target_rotation: 0,
            target_prepare_phase: TargetPreparePhase::Reset,
            blocked_vortex_ids: FixedOwnerSet::new(),
            target_attraction_cursor: 0,
            target_object_cursor: 0,
            target_vortex_cursor: 0,
            seed_targets: Vec::new(),
            frontier_targets: Vec::new(),
            seed_target_weights: Vec::new(),
            frontier_target_weights: Vec::new(),
            seed_target_tree: vec![0.0],
            frontier_target_tree: vec![0.0],
            target_prepare_cursor: 0,
            seed_target_remaining: 0,
            frontier_target_remaining: 0,
            current_target: None,
            candidates: Vec::new(),
            candidate_cursor: 0,
            candidate_prepare_phase: CandidatePreparePhase::Reset,
            candidate_kind_cursor: 0,
            candidate_vortex_cursor: 0,
            candidate_prepare_cursor: 0,
            candidate_seen: FixedOwnerSet::new(),
            candidate_raw: Vec::new(),
            candidate_cross: FixedOwnerMap::new(),
            candidate_same: FixedOwnerMap::new(),
            candidate_same_sorted: Vec::new(),
            candidate_same_weights: Vec::new(),
            candidate_same_tree: vec![0.0],
            candidate_same_remaining: 0,
            current_preview: None,
            broad_phase_query: None,
            broad_phase_cursor: 0,
            broad_phase_bounds: None,
            collision: None,
            accept_phase: AcceptPhase::Validate,
            accept_attraction_cursor: 0,
            accept_vortex_cursor: 0,
            pending_payload: None,
            pending_object: None,
            pending_attraction: None,
            pending_spatial: None,
            last_rejection: None,
            fixed_rejection: None,
            collection_over_capacity: false,
            transition_count: 0,
            rejected_count: 0,
            close_field: 0,
            close_current: None,
            closing: false,
        }
    }

    /// 🧊️ Fixture the fill projection layers its applied prefix on. Until preparation releases them,
    /// the authority is the still-owned preparation roots — the fixed `base` page is only partially
    /// copied, and projecting from it would hand the document a fixture with its own objects missing.
    pub(crate) fn base_fixture(&self) -> Fixture {
        match self.preparation_roots.as_ref() {
            Some(roots) => roots.scene.fixture.clone(),
            None => self.base.snapshot(),
        }
    }

    pub(crate) fn progress(&self) -> FillBuildProgress {
        FillBuildProgress {
            count: self.sequence.len(),
            applied_count: self.applied_count,
            max_count: self.max_count,
            done: self.stalled || self.sequence.len() >= self.max_count,
            appended_objects: Vec::new(),
            appended_attractions: Vec::new(),
            sequence: Vec::new(),
            preview: Some(self.preview.clone()),
        }
    }

    pub(crate) fn preview_json_step(&mut self, color: &str, status_label: &str, fuel: &mut u32, cancelled: bool, deadline_reached: bool) -> FillPreviewJsonStep {
        self.preview_json.step(&self.preview, color, status_label, fuel, cancelled, deadline_reached)
    }

    pub(crate) fn preview_json_ready(&self) -> Option<&str> {
        self.preview_json.ready()
    }

    #[cfg(test)]
    pub(crate) fn preview_json_ready_identity(&self) -> Option<[u64; 5]> {
        self.preview_json.ready_identity()
    }

    fn retire_one_close_owner(&mut self) -> bool {
        if let Some(current) = self.close_current.as_mut() {
            if retire_retained_owner(current) {
                self.close_current = None;
            }
            return false;
        }
        let mut current = None;
        let retired = match self.close_field {
            0 => take_fixture_owner(&mut self.base, &mut current),
            1 => false,
            2 => take_sequence_owner(self, &mut current),
            3 => take_lookup_owner(self, &mut current),
            4 => take_catalog_owner(self, &mut current),
            5 => take_weight_mesh_owner(self, &mut current),
            6 => take_target_owner(self, &mut current),
            7 => take_target_weight_owner(self),
            8 => take_candidate_owner(self, &mut current),
            9 => take_candidate_order_owner(self, &mut current),
            10 => match self.broad_phase_query.as_mut() {
                Some(query) => {
                    if query.retire_one_owner() {
                        self.broad_phase_query.take();
                    }
                    true
                }
                None => false,
            },
            11 => self.pending_payload.take().is_some_and(|value| {
                current = Some(FillRetiredOwner::Payload(value));
                true
            }),
            12 => self.pending_object.take().is_some_and(|value| {
                current = Some(FillRetiredOwner::FixtureObject(value));
                true
            }),
            13 => self.pending_attraction.take().is_some_and(|value| {
                current = Some(FillRetiredOwner::Attraction(value));
                true
            }),
            14 => self.current_target.take().is_some_and(|value| {
                current = Some(FillRetiredOwner::Target(value));
                true
            }),
            15 => self.current_preview.take().is_some_and(|value| {
                current = Some(FillRetiredOwner::PreviewState(value));
                true
            }),
            16 => self.last_rejection.take().is_some_and(|value| {
                current = Some(FillRetiredOwner::String(value));
                true
            }),
            17 => self.collision.take().is_some(),
            18 => !self.preview_json.close_step(),
            19 => !retire_fill_preview(&mut self.preview),
            20 => match self.fixed_rejection.as_mut() {
                Some(rejected) => {
                    if retire_retained_owner(rejected) {
                        self.fixed_rejection.take();
                    }
                    true
                }
                None => false,
            },
            21 => retire_fixed_collection_backing(self),
            22 => !self.spatial_index.retire_one_owner(),
            23 if self.collection_over_capacity => {
                self.collection_over_capacity = false;
                true
            }
            23 => false,
            24 => match self.preparation_spatial.as_mut() {
                Some(mutation) => {
                    if mutation.retire_one_owner() {
                        self.preparation_spatial.take();
                    }
                    true
                }
                None => false,
            },
            25 => match self.pending_spatial.as_mut() {
                Some(mutation) => {
                    if mutation.retire_one_owner() {
                        self.pending_spatial.take();
                    }
                    true
                }
                None => false,
            },
            26 => match self.tail_removal.as_mut() {
                Some(removal) => {
                    if removal.retire_one_owner() {
                        self.tail_removal.take();
                    }
                    true
                }
                None => false,
            },
            27 if self.preparation_roots.take().is_some() => true,
            27 => false,
            28 if self.preparation_capacity_refusal.take().is_some() => true,
            28 => false,
            _ => return self.terminal_owners_empty() && self.close_current.is_none(),
        };
        self.close_current = current;
        if !retired {
            self.close_field = self.close_field.saturating_add(1);
        }
        false
    }

    fn collision_owner(&self) -> CollisionIndexOwner {
        CollisionIndexOwner { operation: self.operation.operation.0, generation: self.operation.generation.0 }
    }

    fn fixture_object(&self, index: usize) -> Option<&FixtureObject> {
        self.base.objects.get(index).or_else(|| self.appended_objects.get(index.saturating_sub(self.base.objects.len())))
    }

    fn fixture_attraction(&self, index: usize) -> Option<&AttractionProps> {
        self.base.attractions.get(index).or_else(|| self.appended_attractions.get(index.saturating_sub(self.base.attractions.len())))
    }

    fn fixture_view(&self) -> FillFixtureView<'_> {
        FillFixtureView { base: &self.base, appended: &self.appended_objects }
    }

    /// 🎚️ Weight-only replan: the applied prefix, the prepared base/catalog/mesh pages and the
    /// spatial index all stay; only the unapplied planning tail is withdrawn — one placement per
    /// turn through [`FillJobStage::DiscardTail`] — before the planner rewinds to target selection
    /// under the new distribution. Rebuilding the whole builder instead is what dropped every
    /// already-applied fill object the moment a distribution slider moved.
    pub(crate) fn begin_soft_replan(&mut self, object_weights: &std::collections::BTreeMap<String, f64>, vortex_weights: &std::collections::BTreeMap<String, f64>) {
        self.weights = RetainedBrushKindWeights::new();
        for (id, weight) in object_weights {
            let _ = self.weights.object_weights.try_insert(id.clone(), *weight);
        }
        for (id, weight) in vortex_weights {
            let _ = self.weights.vortex_weights.try_insert(id.clone(), *weight);
        }
        self.applied_count = self.applied_count.min(self.sequence.len());
        self.stalled = false;
        self.last_rejection = None;
        self.preview.rejection_reason = None;
        self.stage = FillJobStage::DiscardTail;
    }

    /// ♻️ One unapplied placement per turn: withdraw its spatial owner, drop its lookup and placement
    /// rows, then pop the plan row itself. Reaching the applied prefix rewinds the planner.
    fn discard_tail_one(&mut self) {
        let owner = self.collision_owner();
        if let Some(removal) = self.tail_removal.as_mut() {
            match self.spatial_index.step_removal(removal, owner) {
                CollisionMutationStep::Pending => {}
                CollisionMutationStep::Complete => self.tail_removal = None,
                CollisionMutationStep::Rejected(rejected) => {
                    self.fixed_rejection = Some(FillRetiredOwner::Spatial(rejected));
                    self.collection_over_capacity = true;
                }
                CollisionMutationStep::Stale => self.stalled = true,
            }
            return;
        }
        if self.appended_objects.len() <= self.applied_count {
            self.targets.clear();
            self.target_cursor = 0;
            self.target_rotation = 0;
            self.target_prepare_phase = TargetPreparePhase::Reset;
            self.reset_candidate_preparation();
            self.reset_candidate();
            self.reset_collision(true);
            self.reset_acceptance();
            self.preview.accepted_count = self.sequence.len();
            self.stage = FillJobStage::PrepareTargets;
            return;
        }
        let Some(object) = self.appended_objects.pop() else {
            self.stage = FillJobStage::PrepareTargets;
            return;
        };
        self.sequence.pop();
        self.appended_attractions.pop();
        if let Some((id, index)) = self.placed_lookup.remove_entry(object.id.as_str()) {
            if index + 1 == self.placed.len() {
                self.placed.pop();
            }
            drop(id);
        }
        self.tail_removal = self.spatial_index.begin_removal(owner, object.id.clone());
        drop(object);
    }

    pub(crate) fn prepare_one(&mut self) {
        if self.collection_over_capacity {
            self.last_rejection = Some("preparation-capacity".into());
            self.preview.rejection_reason = self.last_rejection.clone();
            self.stalled = true;
            self.stage = FillJobStage::Complete;
            return;
        }
        match self.stage {
            FillJobStage::PrepareFixture => self.prepare_fixture_one(),
            FillJobStage::PrepareCatalogs => self.prepare_catalog_one(),
            FillJobStage::PrepareMeshes => self.prepare_mesh_one(),
            FillJobStage::PrepareEntries => self.prepare_entry_one(),
            FillJobStage::PrepareSpatial => self.prepare_spatial_one(),
            FillJobStage::PrepareLookup => self.prepare_lookup_one(),
            FillJobStage::PrepareConfiguration => self.prepare_configuration_one(),
            _ => {}
        }
    }

    fn prepare_fixture_one(&mut self) {
        let roots = self.preparation_roots.as_ref().expect("preparation roots");
        let fixture = &roots.scene.fixture;
        let value = match self.preparation_inner_cursor {
            0 => fixture.attractions.get(self.preparation_cursor).map(|value| {
                if let Err(owner) = self.base.attractions.try_push(value.clone()) {
                    self.fixed_rejection = Some(FillRetiredOwner::Attraction(owner));
                }
            }),
            1 => fixture.objects.get(self.preparation_cursor).map(|value| {
                if let Err(owner) = self.base.objects.try_push(value.clone()) {
                    self.fixed_rejection = Some(FillRetiredOwner::FixtureObject(owner));
                }
            }),
            _ => fixture.target_volumes.get(self.preparation_cursor).map(|value| {
                if let Err(owner) = self.base.target_volumes.try_push(value.clone()) {
                    self.fixed_rejection = Some(FillRetiredOwner::WorldVolume(owner));
                }
            }),
        };
        if value.is_some() {
            self.preparation_cursor += 1;
            return;
        }
        self.preparation_cursor = 0;
        self.preparation_inner_cursor += 1;
        if self.preparation_inner_cursor == 3 {
            self.preparation_inner_cursor = 0;
            self.stage = FillJobStage::PrepareCatalogs;
        }
    }

    fn prepare_catalog_one(&mut self) {
        let roots = self.preparation_roots.as_ref().expect("preparation roots");
        let catalogs = roots.scene.kind_catalogs.as_ref();
        let value = match self.preparation_inner_cursor {
            0 => catalogs.and_then(|value| value.objects.get(self.preparation_cursor)).map(|value| {
                if let Err(owner) = self.catalogs.objects.try_push(value.clone()) {
                    self.fixed_rejection = Some(FillRetiredOwner::ObjectKind(owner));
                }
            }),
            1 => catalogs.and_then(|value| value.vortices.get(self.preparation_cursor)).map(|value| {
                if let Err(owner) = self.catalogs.vortices.try_push(value.clone()) {
                    self.fixed_rejection = Some(FillRetiredOwner::VortexKind(owner));
                }
            }),
            _ => catalogs.and_then(|value| value.cables.get(self.preparation_cursor)).map(|value| {
                if let Err(owner) = self.catalogs.cables.try_push(value.clone()) {
                    self.fixed_rejection = Some(FillRetiredOwner::CableKind(owner));
                }
            }),
        };
        if value.is_some() {
            self.preparation_cursor += 1;
            return;
        }
        self.preparation_cursor = 0;
        self.preparation_inner_cursor += 1;
        if self.preparation_inner_cursor == 3 {
            self.preparation_inner_cursor = 0;
            self.stage = FillJobStage::PrepareMeshes;
        }
    }

    fn prepare_mesh_one(&mut self) {
        let roots = self.preparation_roots.as_ref().expect("preparation roots");
        let Some((url, body)) = roots.meshes.iter().nth(self.preparation_cursor) else {
            self.preparation_cursor = 0;
            self.stage = FillJobStage::PrepareEntries;
            return;
        };
        self.preparation_cursor += 1;
        match self.meshes.try_insert(url.clone(), body.clone()) {
            Ok(FixedOwnerMapInsert::Inserted) => {}
            Ok(FixedOwnerMapInsert::Occupied { input_key: url, input_value: body }) | Err((url, body)) => {
                self.fixed_rejection = Some(FillRetiredOwner::Mesh(url, body));
                self.collection_over_capacity = true;
            }
        }
    }

    fn prepare_entry_one(&mut self) {
        let Some(object) = self.base.objects.get(self.preparation_cursor) else {
            self.preparation_cursor = 0;
            self.stage = FillJobStage::PrepareSpatial;
            return;
        };
        self.preparation_cursor += 1;
        match self.seed_object_ids.try_insert(object.id.clone()) {
            Ok(FixedOwnerSetInsert::Inserted) => {}
            Ok(FixedOwnerSetInsert::Present { input }) => drop(input),
            Err(input) => {
                self.fixed_rejection = Some(FillRetiredOwner::String(input));
                self.collection_over_capacity = true;
                return;
            }
        }
        let fixture = FillFixtureView { base: &self.base, appended: &self.appended_objects };
        let Some(mesh_url) = resolve_object_kind_mesh_url(object.object_kind.as_deref().unwrap_or(""), &self.catalogs, &fixture) else {
            return;
        };
        if self.meshes.get(&mesh_url).is_none() {
            return;
        }
        self.placed.push(PlacedCollisionEntry { object_id: object.id.clone(), mesh_url, world: pose_isometry(object.origin, object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &object.scale) });
    }

    fn prepare_spatial_one(&mut self) {
        let owner = self.collision_owner();
        if let Some(mutation) = self.preparation_spatial.as_mut() {
            match self.spatial_index.step_replacement(mutation, owner) {
                CollisionMutationStep::Pending => return,
                CollisionMutationStep::Complete => {
                    self.preparation_spatial = None;
                    self.preparation_cursor += 1;
                    return;
                }
                CollisionMutationStep::Rejected(rejected) => {
                    self.fixed_rejection = Some(FillRetiredOwner::Spatial(rejected));
                    self.collection_over_capacity = true;
                    return;
                }
                CollisionMutationStep::Stale => {
                    self.stalled = true;
                    return;
                }
            }
        }
        let Some(entry) = self.placed.get(self.preparation_cursor) else {
            self.preparation_cursor = 0;
            self.stage = FillJobStage::PrepareLookup;
            return;
        };
        let Some(body) = self.meshes.get(&entry.mesh_url) else {
            self.preparation_cursor += 1;
            return;
        };
        self.preparation_spatial = Some(self.spatial_index.begin_replacement(owner, entry.object_id.clone(), CollisionAabb::from_body(body, &entry.world)));
    }

    fn prepare_lookup_one(&mut self) {
        let Some(entry) = self.placed.get(self.preparation_cursor) else {
            self.preparation_cursor = 0;
            self.stage = FillJobStage::PrepareConfiguration;
            return;
        };
        let index = self.preparation_cursor;
        self.preparation_cursor += 1;
        match self.placed_lookup.try_insert(entry.object_id.clone(), index) {
            Ok(FixedOwnerMapInsert::Inserted) => {}
            Ok(FixedOwnerMapInsert::Occupied { input_key, input_value: _ }) | Err((input_key, _)) => {
                self.fixed_rejection = Some(FillRetiredOwner::String(input_key));
                self.collection_over_capacity = true;
            }
        }
    }

    fn prepare_configuration_one(&mut self) {
        let roots = self.preparation_roots.as_ref().expect("preparation roots");
        let value = match self.preparation_inner_cursor {
            0 => roots.scene.weights.object_weights.iter().nth(self.preparation_cursor).map(|(id, weight)| {
                let _ = self.weights.object_weights.try_insert(id.clone(), *weight);
            }),
            1 => roots.scene.weights.vortex_weights.iter().nth(self.preparation_cursor).map(|(id, weight)| {
                let _ = self.weights.vortex_weights.try_insert(id.clone(), *weight);
            }),
            _ => roots.scene.kind_compatibility.get(self.preparation_cursor).map(|value| {
                if let Err(owner) = self.kind_compatibility.try_push(value.clone()) {
                    self.fixed_rejection = Some(FillRetiredOwner::Compat(owner));
                }
            }),
        };
        if value.is_some() {
            self.preparation_cursor += 1;
            return;
        }
        self.preparation_cursor = 0;
        self.preparation_inner_cursor += 1;
        if self.preparation_inner_cursor == 3 {
            self.host_rules = roots.scene.host_rules.clone();
            self.overlap_budget = roots.scene.overlap_budget;
            self.preparation_roots = None;
            self.preparation_inner_cursor = 0;
            self.stage = FillJobStage::PrepareTargets;
        }
    }
}

//#region 🧵️InteractiveFillJob
impl FillBuilder {
    fn prepare_targets(&mut self) {
        match self.target_prepare_phase {
            TargetPreparePhase::Reset => {
                if let Some(value) = self.blocked_vortex_ids.pop_first() {
                    drop(value);
                } else {
                    self.target_prepare_phase = TargetPreparePhase::Blocked;
                }
            }
            TargetPreparePhase::Blocked => {
                if let Some((attracting, attracted)) = self.fixture_attraction(self.target_attraction_cursor).map(|attraction| (attraction.attracting.clone(), attraction.attracted.clone())) {
                    match self.blocked_vortex_ids.try_insert(attracting) {
                        Ok(FixedOwnerSetInsert::Inserted) => {}
                        Ok(FixedOwnerSetInsert::Present { input }) => drop(input),
                        Err(value) => {
                            self.fixed_rejection = Some(FillRetiredOwner::String(value));
                            return;
                        }
                    }
                    match self.blocked_vortex_ids.try_insert(attracted) {
                        Ok(FixedOwnerSetInsert::Inserted) => {}
                        Ok(FixedOwnerSetInsert::Present { input }) => drop(input),
                        Err(value) => {
                            self.fixed_rejection = Some(FillRetiredOwner::String(value));
                            return;
                        }
                    }
                    self.target_attraction_cursor += 1;
                } else {
                    self.target_prepare_phase = TargetPreparePhase::Enumerate;
                }
            }
            TargetPreparePhase::Enumerate => {
                let Some(object) = self.fixture_object(self.target_object_cursor) else {
                    self.seed_target_tree = vec![0.0; self.seed_target_weights.len() + 1];
                    self.frontier_target_tree = vec![0.0; self.frontier_target_weights.len() + 1];
                    self.seed_target_remaining = self.seed_targets.len();
                    self.frontier_target_remaining = self.frontier_targets.len();
                    self.target_prepare_cursor = 0;
                    self.target_prepare_phase = TargetPreparePhase::BuildSeedWeights;
                    return;
                };
                let Some(vortex) = object.vortices.get(self.target_vortex_cursor) else {
                    self.target_object_cursor += 1;
                    self.target_vortex_cursor = 0;
                    return;
                };
                let object_id = object.id.clone();
                let object_kind = object.object_kind.clone();
                let vortex_id = vortex.id.clone();
                let vortex_kind = vortex.vortex_kind.clone();
                let vortex_index = self.target_vortex_cursor;
                self.target_vortex_cursor += 1;
                let full_id = puzzle3d_vortex_full_id(&object_id, &vortex_id);
                if self.blocked_vortex_ids.contains(&full_id) {
                    return;
                }
                let target = BrushFillVortexTarget { full_id, object_id, object_kind, vortex_kind, vortex_index };
                let weight = retained_fill_vortex_target_weight(&target, &self.weights);
                if weight <= 0.0 {
                    return;
                }
                if self.seed_object_ids.contains(&target.object_id) {
                    self.seed_targets.push(target);
                    self.seed_target_weights.push(weight);
                } else {
                    self.frontier_targets.push(target);
                    self.frontier_target_weights.push(weight);
                }
            }
            TargetPreparePhase::BuildSeedWeights => {
                if let Some(weight) = self.seed_target_weights.get(self.target_prepare_cursor).copied() {
                    fenwick_add(&mut self.seed_target_tree, self.target_prepare_cursor, weight);
                    self.target_prepare_cursor += 1;
                } else {
                    self.target_prepare_cursor = 0;
                    self.target_prepare_phase = TargetPreparePhase::BuildFrontierWeights;
                }
            }
            TargetPreparePhase::BuildFrontierWeights => {
                if let Some(weight) = self.frontier_target_weights.get(self.target_prepare_cursor).copied() {
                    fenwick_add(&mut self.frontier_target_tree, self.target_prepare_cursor, weight);
                    self.target_prepare_cursor += 1;
                } else {
                    self.target_prepare_phase = TargetPreparePhase::OrderSeed;
                }
            }
            TargetPreparePhase::OrderSeed => {
                if let Some(index) = weighted_pick(&mut self.seed_target_weights, &mut self.seed_target_tree, self.seed_target_remaining, &mut self.rng_state) {
                    self.targets.push(self.seed_targets[index].clone());
                    self.seed_target_remaining -= 1;
                } else {
                    self.target_prepare_phase = TargetPreparePhase::OrderFrontier;
                }
            }
            TargetPreparePhase::OrderFrontier => {
                if let Some(index) = weighted_pick(&mut self.frontier_target_weights, &mut self.frontier_target_tree, self.frontier_target_remaining, &mut self.rng_state) {
                    self.targets.push(self.frontier_targets[index].clone());
                    self.frontier_target_remaining -= 1;
                } else {
                    self.target_prepare_phase = TargetPreparePhase::Finish;
                }
            }
            TargetPreparePhase::Finish => {
                if self.targets.is_empty() {
                    self.stalled = true;
                    self.stage = FillJobStage::Complete;
                    return;
                }
                self.target_rotation = self.sequence.len() % self.targets.len();
                self.target_cursor = 0;
                self.stage = FillJobStage::SelectTarget;
            }
        }
    }

    fn select_target(&mut self) {
        if self.target_cursor >= self.targets.len() {
            self.stalled = true;
            self.stage = FillJobStage::Complete;
            return;
        }
        let index = self.target_rotation.checked_add(self.target_cursor).map(|value| value % self.targets.len().max(1));
        let Some(target) = index.and_then(|index| self.targets.get(index)).cloned() else {
            self.stalled = true;
            self.stage = FillJobStage::Complete;
            return;
        };
        self.preview.target_vortex_full_id = Some(target.full_id.clone());
        self.preview.target_cursor = self.target_cursor;
        self.current_target = Some(target);
        self.reset_candidate_preparation();
        self.stage = FillJobStage::PrepareCandidates;
    }

    fn prepare_candidates(&mut self) {
        let Some(target) = self.current_target.clone() else {
            self.reject_target("missing-target");
            return;
        };
        let target_context = AttractionVortexContext { object_kind: target.object_kind.clone(), vortex_kind: target.vortex_kind.clone() };
        match self.candidate_prepare_phase {
            CandidatePreparePhase::Reset => {
                if let Some(value) = self.candidate_seen.pop_first() {
                    drop(value);
                } else if let Some(value) = self.candidate_cross.pop_first() {
                    drop(value);
                } else if let Some(value) = self.candidate_same.pop_first() {
                    drop(value);
                } else {
                    self.candidate_prepare_phase = CandidatePreparePhase::Enumerate;
                }
            }
            CandidatePreparePhase::Enumerate => {
                let Some(kind) = self.catalogs.objects.get(self.candidate_kind_cursor) else {
                    self.candidate_prepare_cursor = 0;
                    self.candidate_prepare_phase = CandidatePreparePhase::Classify;
                    return;
                };
                if self.candidate_vortex_cursor >= kind.vortices.len() {
                    self.candidate_kind_cursor += 1;
                    self.candidate_vortex_cursor = 0;
                    return;
                }
                let vortex_index = self.candidate_vortex_cursor;
                self.candidate_vortex_cursor += 1;
                let Some((candidate, _)) = brush_fill_candidate_at(&target_context, &self.catalogs, &self.kind_compatibility, &self.host_rules, self.candidate_kind_cursor, vortex_index) else { return };
                let key = format!("{}\u{1}{}", candidate.object_kind_id, candidate.source_vortex_index);
                match self.candidate_seen.try_insert(key) {
                    Ok(FixedOwnerSetInsert::Inserted) => self.candidate_raw.push(candidate),
                    Ok(FixedOwnerSetInsert::Present { input }) => drop(input),
                    Err(key) => self.fixed_rejection = Some(FillRetiredOwner::String(key)),
                }
            }
            CandidatePreparePhase::Classify => {
                let Some(candidate) = self.candidate_raw.get(self.candidate_prepare_cursor).cloned() else {
                    self.candidate_prepare_phase = CandidatePreparePhase::DrainCross;
                    return;
                };
                self.candidate_prepare_cursor += 1;
                if retained_candidate_suggestion_weight(&candidate, &self.weights, &self.catalogs) <= 0.0 {
                    return;
                }
                let source_vortex = self.catalogs.objects.iter().find(|kind| kind.id == candidate.object_kind_id).and_then(|kind| kind.vortices.get(candidate.source_vortex_index)).and_then(|vortex| vortex.vortex_kind.as_deref()).unwrap_or("");
                let target_vortex = target.vortex_kind.as_deref().unwrap_or("");
                if source_vortex != target_vortex || brush_stack_mate_pair(source_vortex, target_vortex) {
                    let score = fill_candidate_diversity_score(&candidate, target.vortex_index, target.object_kind.as_deref()).max(0) as u64;
                    let key = format!("{:016x}\u{1}{}\u{1}{:016x}", u64::MAX - score, candidate.object_kind_id, candidate.source_vortex_index);
                    match self.candidate_cross.try_insert(key, candidate) {
                        Ok(FixedOwnerMapInsert::Inserted) => {}
                        Ok(FixedOwnerMapInsert::Occupied { input_key: key, input_value: candidate }) | Err((key, candidate)) => {
                            self.fixed_rejection = Some(FillRetiredOwner::CandidateMap(key, candidate));
                        }
                    }
                } else {
                    let key = format!("{}\u{1}{:016x}", candidate.object_kind_id, candidate.source_vortex_index);
                    match self.candidate_same.try_insert(key, candidate) {
                        Ok(FixedOwnerMapInsert::Inserted) => {}
                        Ok(FixedOwnerMapInsert::Occupied { input_key: key, input_value: candidate }) | Err((key, candidate)) => {
                            self.fixed_rejection = Some(FillRetiredOwner::CandidateMap(key, candidate));
                        }
                    }
                }
            }
            CandidatePreparePhase::DrainCross => {
                if let Some((_, candidate)) = self.candidate_cross.pop_first() {
                    self.candidates.push(candidate);
                } else {
                    self.candidate_prepare_phase = CandidatePreparePhase::DrainSame;
                }
            }
            CandidatePreparePhase::DrainSame => {
                if let Some((_, candidate)) = self.candidate_same.pop_first() {
                    self.candidate_same_weights.push(retained_candidate_suggestion_weight(&candidate, &self.weights, &self.catalogs));
                    self.candidate_same_sorted.push(candidate);
                } else {
                    self.candidate_same_remaining = self.candidate_same_sorted.len();
                    self.candidate_same_tree = vec![0.0; self.candidate_same_weights.len() + 1];
                    self.candidate_prepare_cursor = 0;
                    self.candidate_prepare_phase = CandidatePreparePhase::BuildSameWeights;
                }
            }
            CandidatePreparePhase::BuildSameWeights => {
                if let Some(weight) = self.candidate_same_weights.get(self.candidate_prepare_cursor).copied() {
                    fenwick_add(&mut self.candidate_same_tree, self.candidate_prepare_cursor, weight);
                    self.candidate_prepare_cursor += 1;
                } else {
                    self.candidate_prepare_phase = CandidatePreparePhase::OrderSame;
                }
            }
            CandidatePreparePhase::OrderSame => {
                if let Some(index) = weighted_pick(&mut self.candidate_same_weights, &mut self.candidate_same_tree, self.candidate_same_remaining, &mut self.rng_state) {
                    self.candidates.push(self.candidate_same_sorted[index].clone());
                    self.candidate_same_remaining -= 1;
                } else {
                    self.candidate_prepare_phase = CandidatePreparePhase::Finish;
                }
            }
            CandidatePreparePhase::Finish => {
                self.candidate_cursor = 0;
                if self.candidates.is_empty() {
                    self.reject_target("no-compatible-candidate");
                } else {
                    self.stage = FillJobStage::SelectCandidate;
                }
            }
        }
    }

    fn select_candidate(&mut self) {
        let Some(candidate) = self.candidates.get(self.candidate_cursor) else {
            self.reject_target("candidates-exhausted");
            return;
        };
        self.preview.candidate_cursor = self.candidate_cursor;
        self.preview.candidate_object_kind_id = Some(candidate.object_kind_id.clone());
        self.stage = FillJobStage::ConstructPreview;
    }

    fn construct_preview(&mut self) {
        let Some(target) = &self.current_target else {
            self.reject_target("missing-target");
            return;
        };
        let Some(candidate) = self.candidates.get(self.candidate_cursor) else {
            self.reject_target("missing-candidate");
            return;
        };
        let Some(host) = self.base.objects.iter().chain(&self.appended_objects).find(|object| object.id == target.object_id) else {
            self.reject_target("missing-host");
            return;
        };
        let Some((position, direction)) = vortex_world_from_object(host, target.vortex_index) else {
            self.reject_candidate("invalid-target-pose");
            return;
        };
        let context = AttractionVortexContext { object_kind: target.object_kind.clone(), vortex_kind: target.vortex_kind.clone() };
        let world = TargetVortexWorld { position, direction, reference_orientation: host.orientation };
        let fixture = self.fixture_view();
        let Some(preview) = brush_preview_from_candidate(&target.full_id, candidate, &context, world, &self.catalogs, &fixture) else {
            self.reject_candidate("preview-unavailable");
            return;
        };
        let Some(body) = self.meshes.get(&preview.mesh_url) else {
            self.reject_candidate("mesh-unavailable");
            return;
        };
        let preview_world = pose_isometry(preview.origin, preview.orientation, &preview.scale);
        let (min, max) = world_bounds(body, &preview_world);
        if !world_volumes_contain_aabb(self.base.target_volumes.as_slice(), min, max) {
            self.reject_candidate("outside-target-volume");
            return;
        }
        self.current_preview = Some(preview);
        self.last_rejection = None;
        self.preview.rejection_reason = None;
        self.preview.candidate_ghost = self.current_preview.clone();
        self.preview.current_pair_object_id = None;
        self.preview.collision_count = 0;
        self.preview.candidate_page = std::array::from_fn(|_| None);
        self.preview.truncated = false;
        self.stage = FillJobStage::QueryBroadPhase;
    }

    fn query_broad_phase(&mut self) {
        let Some(_target) = &self.current_target else {
            self.reject_target("missing-target");
            return;
        };
        let Some(preview) = &self.current_preview else {
            self.reject_candidate("missing-preview");
            return;
        };
        let Some(body) = self.meshes.get(&preview.mesh_url) else {
            self.reject_candidate("mesh-unavailable");
            return;
        };
        if self.broad_phase_query.is_none() {
            let world = pose_isometry(preview.origin, preview.orientation, &preview.scale);
            let bounds = CollisionAabb::from_body(body, &world);
            self.broad_phase_bounds = Some(bounds);
            self.broad_phase_query = Some(self.spatial_index.begin_query(self.collision_owner(), bounds));
            self.broad_phase_cursor = 0;
            return;
        }
        let owner = self.collision_owner();
        let query = self.broad_phase_query.as_mut().expect("broad phase query");
        match self.spatial_index.step_query(query, owner) {
            CollisionQueryStep::Pending => return,
            CollisionQueryStep::Stale => {
                self.reject_candidate("stale-spatial-query");
                return;
            }
            CollisionQueryStep::Complete => {}
        }
        self.broad_phase_cursor = 0;
        self.collision = None;
        self.preview.candidate_page = std::array::from_fn(|index| query.candidate(index).cloned());
        self.preview.truncated = query.truncated() || query.len() > self.preview.candidate_page.len();
        self.preview.collision_count = 0;
        self.stage = FillJobStage::TestCollision;
    }

    fn test_collision(&mut self, context: &mut StepContext<'_>) -> Option<StepOutcome> {
        let Some(pair_id) = self.broad_phase_query.as_ref().and_then(|query| query.candidate(self.broad_phase_cursor)).cloned() else {
            self.preview.current_pair_object_id = None;
            self.stage = FillJobStage::AcceptCandidate;
            return None;
        };
        if self.current_target.as_ref().is_some_and(|target| target.object_id == pair_id) {
            self.broad_phase_cursor += 1;
            return None;
        }
        self.preview.current_pair_object_id = Some(pair_id.clone());
        let Some(preview) = &self.current_preview else {
            self.reject_candidate("missing-preview");
            return None;
        };
        let Some(preview_body) = self.meshes.get(&preview.mesh_url) else {
            self.reject_candidate("mesh-unavailable");
            return None;
        };
        let Some(entry) = self.placed_lookup.get(&pair_id).and_then(|index| self.placed.get(*index)) else {
            self.reject_candidate("broad-phase-entry-missing");
            return None;
        };
        let Some(other) = self.meshes.get(&entry.mesh_url) else {
            self.reject_candidate("placed-mesh-unavailable");
            return None;
        };
        let preview_world = pose_isometry(preview.origin, preview.orientation, &preview.scale);
        let collision = self.collision.get_or_insert_with(|| CollisionOverlapState::new(512, 8, self.overlap_budget));
        let result = collision.step(context, preview_body, &preview_world, other, &entry.world);
        self.preview.sample_cursor = collision.sample_cursor;
        self.preview.inside_both = collision.inside_both;
        self.preview.last_sample = collision.last_sample;
        match result {
            CollisionStepResult::Pending => {}
            CollisionStepResult::Cancelled => return Some(StepOutcome::Cancelled),
            CollisionStepResult::Complete { overlap, .. } if overlap > self.overlap_budget => {
                self.preview.collision_count += 1;
                self.reject_candidate("solid-overlap");
            }
            CollisionStepResult::Complete { .. } => {
                self.broad_phase_cursor += 1;
                self.collision = None;
                self.preview.sample_cursor = 0;
                self.preview.inside_both = 0;
                self.preview.last_sample = None;
            }
        }
        None
    }

    fn accept_candidate(&mut self) -> StepOutcome {
        match self.accept_phase {
            AcceptPhase::Validate => {
                let Some(preview) = self.current_preview.clone() else {
                    self.reject_candidate("missing-preview");
                    return StepOutcome::Yield;
                };
                let payload = BrushPlacePayload {
                    target_vortex_full_id: preview.target_vortex_full_id.clone(),
                    object_kind_id: preview.object_kind_id.clone(),
                    source_vortex_index: preview.source_vortex_index,
                    origin: preview.origin,
                    orientation: preview.orientation,
                    scale: preview.scale,
                };
                let Some(kind) = self.catalogs.objects.iter().find(|kind| kind.id == payload.object_kind_id) else {
                    self.reject_candidate("placement-kind-missing");
                    return StepOutcome::Yield;
                };
                if kind.vortices.get(payload.source_vortex_index).is_none() {
                    self.reject_candidate("placement-vortex-missing");
                    return StepOutcome::Yield;
                }
                let fixture = self.fixture_view();
                let Some(mesh_url) = resolve_object_kind_mesh_url(&payload.object_kind_id, &self.catalogs, &fixture) else {
                    self.reject_candidate("placement-mesh-missing");
                    return StepOutcome::Yield;
                };
                let object_id = brush_object_id(&fixture, &payload);
                let source_vortex_id = format!("{object_id}:v{}", payload.source_vortex_index);
                let attracted = puzzle3d_vortex_full_id(&object_id, &source_vortex_id);
                self.pending_attraction = Some(AttractionProps {
                    id: format!("attraction-{}-{attracted}", payload.target_vortex_full_id),
                    attracting: payload.target_vortex_full_id.clone(),
                    attracted,
                    gap: 0.0,
                    shift: 0.0,
                    rise: 0.0,
                    rotation: 0.0,
                    turn: 0.0,
                    tilt: 0.0,
                    x: 0.0,
                    y: 0.0,
                });
                self.pending_object = Some(FixtureObject {
                    id: object_id,
                    object_kind: Some(kind.id.clone()),
                    anchor: Default::default(),
                    mesh_url: Some(mesh_url),
                    origin: payload.origin,
                    orientation: Some(payload.orientation),
                    scale: payload.scale.clone().or(kind.scale.clone()),
                    vortices: Vec::new(),
                    reveal_index: None,
                });
                self.pending_payload = Some(payload);
                self.accept_attraction_cursor = 0;
                self.accept_vortex_cursor = 0;
                self.accept_phase = AcceptPhase::CheckAttractions;
                StepOutcome::Yield
            }
            AcceptPhase::CheckAttractions => {
                let Some((pending_attracting, pending_attracted)) = self.pending_attraction.as_ref().map(|pending| (pending.attracting.clone(), pending.attracted.clone())) else {
                    self.reject_candidate("placement-state-missing");
                    return StepOutcome::Yield;
                };
                if let Some(rejected) = self.fixture_attraction(self.accept_attraction_cursor).map(|attraction| attraction.attracting == pending_attracting || attraction.attracted == pending_attracted) {
                    self.accept_attraction_cursor += 1;
                    if rejected {
                        self.reject_candidate("placement-rejected");
                    }
                    return StepOutcome::Yield;
                }
                self.accept_phase = AcceptPhase::BuildVortices;
                StepOutcome::Yield
            }
            AcceptPhase::BuildVortices => {
                let Some(payload) = self.pending_payload.as_ref() else {
                    self.reject_candidate("placement-state-missing");
                    return StepOutcome::Yield;
                };
                let Some(kind) = self.catalogs.objects.iter().find(|kind| kind.id == payload.object_kind_id) else {
                    self.reject_candidate("placement-kind-missing");
                    return StepOutcome::Yield;
                };
                if let Some(template) = kind.vortices.get(self.accept_vortex_cursor) {
                    let object_id = self.pending_object.as_ref().expect("pending object").id.clone();
                    let index = self.accept_vortex_cursor;
                    self.accept_vortex_cursor += 1;
                    self.pending_object.as_mut().expect("pending object").vortices.push(VortexProps { id: format!("{object_id}:v{index}"), vortex_kind: template.vortex_kind.clone(), position: template.point, direction: template.direction });
                    return StepOutcome::Yield;
                }
                self.accept_phase = AcceptPhase::BeginSpatial;
                StepOutcome::Yield
            }
            AcceptPhase::BeginSpatial => {
                let Some(object) = self.pending_object.as_ref() else {
                    self.reject_candidate("placement-state-missing");
                    return StepOutcome::Yield;
                };
                let Some(mesh_url) = object.mesh_url.as_ref() else {
                    self.accept_phase = AcceptPhase::Commit;
                    return StepOutcome::Yield;
                };
                let Some(body) = self.meshes.get(mesh_url) else {
                    self.reject_candidate("placement-mesh-missing");
                    return StepOutcome::Yield;
                };
                let world = pose_isometry(object.origin, object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &object.scale);
                self.pending_spatial = Some(self.spatial_index.begin_replacement(self.collision_owner(), object.id.clone(), CollisionAabb::from_body(body, &world)));
                self.accept_phase = AcceptPhase::StepSpatial;
                StepOutcome::Yield
            }
            AcceptPhase::StepSpatial => {
                let owner = self.collision_owner();
                let Some(mutation) = self.pending_spatial.as_mut() else {
                    self.reject_candidate("placement-spatial-state-missing");
                    return StepOutcome::Yield;
                };
                match self.spatial_index.step_replacement(mutation, owner) {
                    CollisionMutationStep::Pending => {}
                    CollisionMutationStep::Complete => {
                        self.pending_spatial = None;
                        self.accept_phase = AcceptPhase::InstallLookup;
                    }
                    CollisionMutationStep::Rejected(rejected) => self.fixed_rejection = Some(FillRetiredOwner::Spatial(rejected)),
                    CollisionMutationStep::Stale => self.reject_candidate("stale-spatial-mutation"),
                }
                StepOutcome::Yield
            }
            AcceptPhase::InstallLookup => {
                let Some(object) = self.pending_object.as_ref() else {
                    self.reject_candidate("placement-state-missing");
                    return StepOutcome::Yield;
                };
                let Some(mesh_url) = object.mesh_url.as_ref() else {
                    self.accept_phase = AcceptPhase::Commit;
                    return StepOutcome::Yield;
                };
                let index = self.placed.len();
                match self.placed_lookup.try_insert(object.id.clone(), index) {
                    Ok(FixedOwnerMapInsert::Inserted) => {}
                    Ok(FixedOwnerMapInsert::Occupied { input_key, input_value: _ }) | Err((input_key, _)) => {
                        self.fixed_rejection = Some(FillRetiredOwner::String(input_key));
                        return StepOutcome::Yield;
                    }
                }
                self.placed.push(PlacedCollisionEntry { object_id: object.id.clone(), mesh_url: mesh_url.clone(), world: pose_isometry(object.origin, object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &object.scale) });
                self.accept_phase = AcceptPhase::Commit;
                StepOutcome::Yield
            }
            AcceptPhase::Commit => {
                let Some(payload) = self.pending_payload.take() else {
                    self.reject_candidate("placement-state-missing");
                    return StepOutcome::Yield;
                };
                let Some(mut placed_object) = self.pending_object.take() else {
                    self.reject_candidate("placement-state-missing");
                    return StepOutcome::Yield;
                };
                let Some(attraction) = self.pending_attraction.take() else {
                    self.reject_candidate("placement-state-missing");
                    return StepOutcome::Yield;
                };
                self.sequence.push(payload);
                placed_object.reveal_index = Some(self.appended_objects.len());
                self.appended_objects.push(placed_object);
                self.appended_attractions.push(attraction);
                self.preview.accepted_count = self.sequence.len();
                self.reset_candidate();
                self.stage = if self.sequence.len() >= self.max_count { FillJobStage::Complete } else { FillJobStage::PrepareTargets };
                if self.stage == FillJobStage::Complete {
                    return self.complete();
                }
                StepOutcome::CheckpointReady(semio_framework_job::Checkpoint { state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CheckpointState), applied_progress: self.applied_count as u64 })
            }
        }
    }

    fn reject_candidate(&mut self, reason: &str) {
        self.last_rejection = Some(reason.to_string());
        self.preview.rejection_reason = self.last_rejection.clone();
        self.rejected_count += 1;
        self.preview.rejected_count = self.rejected_count;
        self.candidate_cursor += 1;
        self.reset_acceptance();
        self.reset_collision(false);
        self.stage = FillJobStage::SelectCandidate;
    }

    fn reject_target(&mut self, reason: &str) {
        self.last_rejection = Some(reason.to_string());
        self.preview.rejection_reason = self.last_rejection.clone();
        self.rejected_count += 1;
        self.preview.rejected_count = self.rejected_count;
        self.target_cursor += 1;
        self.current_target = None;
        self.reset_candidate_preparation();
        self.reset_acceptance();
        self.reset_collision(false);
        self.stage = FillJobStage::SelectTarget;
    }

    fn reset_collision(&mut self, clear_preview: bool) {
        self.current_preview = None;
        self.broad_phase_query = None;
        self.broad_phase_cursor = 0;
        self.broad_phase_bounds = None;
        self.collision = None;
        if clear_preview {
            self.preview.candidate_ghost = None;
            self.preview.current_pair_object_id = None;
            self.preview.collision_count = 0;
            self.preview.sample_cursor = 0;
            self.preview.inside_both = 0;
            self.preview.last_sample = None;
            self.preview.candidate_page = std::array::from_fn(|_| None);
            self.preview.truncated = false;
        }
    }

    fn reset_candidate(&mut self) {
        self.targets.clear();
        self.target_cursor = 0;
        self.target_rotation = 0;
        self.target_prepare_phase = TargetPreparePhase::Reset;
        self.target_attraction_cursor = 0;
        self.target_object_cursor = 0;
        self.target_vortex_cursor = 0;
        self.seed_targets.clear();
        self.frontier_targets.clear();
        self.seed_target_weights.clear();
        self.frontier_target_weights.clear();
        self.seed_target_tree = vec![0.0];
        self.frontier_target_tree = vec![0.0];
        self.target_prepare_cursor = 0;
        self.seed_target_remaining = 0;
        self.frontier_target_remaining = 0;
        self.current_target = None;
        self.reset_candidate_preparation();
        self.reset_acceptance();
        self.last_rejection = None;
        self.preview.rejection_reason = None;
        self.reset_collision(true);
    }

    fn reset_candidate_preparation(&mut self) {
        self.candidates.clear();
        self.candidate_cursor = 0;
        self.candidate_prepare_phase = CandidatePreparePhase::Reset;
        self.candidate_kind_cursor = 0;
        self.candidate_vortex_cursor = 0;
        self.candidate_prepare_cursor = 0;
        self.candidate_raw.clear();
        self.candidate_same_sorted.clear();
        self.candidate_same_weights.clear();
        self.candidate_same_tree = vec![0.0];
        self.candidate_same_remaining = 0;
    }

    fn reset_acceptance(&mut self) {
        self.accept_phase = AcceptPhase::Validate;
        self.accept_attraction_cursor = 0;
        self.accept_vortex_cursor = 0;
        self.pending_payload = None;
        self.pending_object = None;
        self.pending_attraction = None;
        self.pending_spatial = None;
    }

    fn publish_preview(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        self.preview.sequence = match context.next_preview_sequence() {
            Ok(sequence) => sequence,
            Err(_) => return StepOutcome::Fault(JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) }),
        };
        self.preview.operation = self.operation.operation.0;
        self.preview.base_revision = self.operation.base_revision.0;
        self.preview.generation = self.operation.generation.0;
        self.preview.stage = self.stage_label().to_string();
        self.preview.target_cursor = self.target_cursor;
        self.preview.candidate_cursor = self.candidate_cursor;
        self.preview.search_count = self.transition_count;
        self.preview.rejected_count = self.rejected_count;
        StepOutcome::PreviewReady(semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Preview))
    }

    fn complete(&self) -> StepOutcome {
        StepOutcome::Complete(CommitCandidate {
            state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
            output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
        })
    }

    fn stage_label(&self) -> &'static str {
        match self.stage {
            FillJobStage::DiscardTail => "discard-tail",
            FillJobStage::PrepareFixture => "prepare-fixture",
            FillJobStage::PrepareCatalogs => "prepare-catalogs",
            FillJobStage::PrepareMeshes => "prepare-meshes",
            FillJobStage::PrepareEntries => "prepare-entries",
            FillJobStage::PrepareSpatial => "prepare-spatial",
            FillJobStage::PrepareLookup => "prepare-lookup",
            FillJobStage::PrepareConfiguration => "prepare-configuration",
            FillJobStage::PrepareTargets => "prepare-targets",
            FillJobStage::SelectTarget => "select-target",
            FillJobStage::PrepareCandidates => "prepare-candidates",
            FillJobStage::SelectCandidate => "select-candidate",
            FillJobStage::ConstructPreview => "construct-preview",
            FillJobStage::QueryBroadPhase => "query-broad-phase",
            FillJobStage::TestCollision => "test-collision",
            FillJobStage::AcceptCandidate => "accept-candidate",
            FillJobStage::Complete => "complete",
        }
    }
}

impl InteractiveJob for FillBuilder {
    fn step(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        if context.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            let detail = context.payload_from_bytes(semio_framework_job::JobPayloadStream::Fault, b"stale-fill-operation").unwrap_or_else(|_| semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault));
            return StepOutcome::Fault(JobFault { detail });
        }
        if let Some(refusal) = self.preparation_capacity_refusal.as_mut() {
            if !refusal.diagnostic_published {
                refusal.diagnostic_published = true;
                self.preview.candidate_ghost = None;
                self.preview.rejection_reason = Some(refusal.diagnostic());
                return self.publish_preview(context);
            }
            let detail = context.payload_from_bytes(semio_framework_job::JobPayloadStream::Fault, b"fill-preparation-capacity").unwrap_or_else(|_| semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault));
            return StepOutcome::Fault(JobFault { detail });
        }
        if self.collection_over_capacity || self.fixed_rejection.is_some() {
            let detail = context.payload_from_bytes(semio_framework_job::JobPayloadStream::Fault, b"fill-fixed-collection-capacity").unwrap_or_else(|_| semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault));
            return StepOutcome::Fault(JobFault { detail });
        }
        if context.should_yield() {
            return StepOutcome::Yield;
        }
        context.set_stage(self.stage_label());
        let stage = self.stage;
        let outcome = match stage {
            FillJobStage::DiscardTail => {
                self.discard_tail_one();
                None
            }
            FillJobStage::PrepareFixture | FillJobStage::PrepareCatalogs | FillJobStage::PrepareMeshes | FillJobStage::PrepareEntries | FillJobStage::PrepareSpatial | FillJobStage::PrepareLookup | FillJobStage::PrepareConfiguration => {
                self.prepare_one();
                None
            }
            FillJobStage::PrepareTargets => {
                self.prepare_targets();
                None
            }
            FillJobStage::SelectTarget => {
                self.select_target();
                None
            }
            FillJobStage::PrepareCandidates => {
                self.prepare_candidates();
                None
            }
            FillJobStage::SelectCandidate => {
                self.select_candidate();
                None
            }
            FillJobStage::ConstructPreview => {
                self.construct_preview();
                None
            }
            FillJobStage::QueryBroadPhase => {
                self.query_broad_phase();
                None
            }
            FillJobStage::TestCollision => self.test_collision(context),
            FillJobStage::AcceptCandidate => Some(self.accept_candidate()),
            FillJobStage::Complete => return self.complete(),
        };
        self.transition_count += 1;
        if stage != FillJobStage::TestCollision {
            context.consume_fuel(1);
        }
        if context.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if stage == self.stage
            && matches!(
                stage,
                FillJobStage::DiscardTail
                    | FillJobStage::PrepareFixture
                    | FillJobStage::PrepareCatalogs
                    | FillJobStage::PrepareMeshes
                    | FillJobStage::PrepareEntries
                    | FillJobStage::PrepareSpatial
                    | FillJobStage::PrepareLookup
                    | FillJobStage::PrepareConfiguration
                    | FillJobStage::PrepareTargets
                    | FillJobStage::PrepareCandidates
                    | FillJobStage::QueryBroadPhase
            )
        {
            return StepOutcome::Yield;
        }
        outcome.unwrap_or_else(|| self.publish_preview(context))
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        self.closing = true;
        if maximum_items == 0 || maximum_bytes < FILL_BUILDER_OWNER_PAGE_BYTES {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if self.retire_one_close_owner() {
            semio_framework_job::InteractiveJobCloseStep::Complete
        } else {
            semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: FILL_BUILDER_OWNER_PAGE_BYTES }
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.close_current.is_none() && self.terminal_owners_empty()
    }
}
//#endregion 🧵️InteractiveFillJob

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
