//! 🧠️ Precompute command work: the staged precompute commands that decode and page the scene, its catalogs and the mesh positions and indices into the engine session before they publish.

use super::*;
use semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dCommand;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle3dPrecomputeCommandStage {
    Decode,
    Objects,
    Vortices,
    Attractions,
    CatalogObjects,
    CatalogVortices,
    Positions,
    Indices,
    PrologueScene,
    PrologueSync,
    Publish,
    Complete,
    Closing,
}

/// ♻️ The owners one `Puzzle3dPrecomputeCommandWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
pub(super) struct Puzzle3dPrecomputeWorkOwners {
    pub(super) prologue: Puzzle3dActionPrologue,
    pub(super) session: Option<(u32, String)>,
    pub(super) instance_owner: Option<ArtifactInstanceOperationOwnerHandle>,
}

pub struct Puzzle3dPrecomputeCommandWork {
    pub(super) tool_id: &'static str,
    pub(super) stage: Puzzle3dPrecomputeCommandStage,
    pub(super) object_cursor: usize,
    child_cursor: usize,
    pub(super) attraction_cursor: usize,
    catalog_object_cursor: usize,
    catalog_vortex_cursor: usize,
    payload_cursor: usize,
    requested_count: u32,
    pub(super) delta: isize,
    candidate_count: usize,
    processed_units: usize,
    /// 🪪️ Session key of the admission that built this work — bound by `build_tool_job` on every
    /// admission and every worker-hop resume, so the precompute-gated actions reach the same slot.
    pub(super) session: Option<(u32, String)>,
    pub(super) prologue: Puzzle3dActionPrologue,
    pub(super) instance_owner: Option<ArtifactInstanceOperationOwnerHandle>,
    pub(super) close_owners: semio_s_plugin_puzzle_jobs::WorkClosing<Puzzle3dPrecomputeWorkOwners>,
}

impl Puzzle3dPrecomputeCommandWork {
    /// 🪪️ Binds the admission's instance identity and retained operation owner.
    pub fn bound(mut self, instance: &Puzzle3dInstanceBinding) -> Self {
        self.session = Some((instance.app_instance_id, instance.parent_document_id.clone()));
        self.instance_owner = Some(instance.owner.clone());
        self
    }

    pub(super) fn session(&self) -> Option<(u32, Option<String>)> {
        self.session.as_ref().map(|(app_instance_id, parent_artifact_id)| (*app_instance_id, Some(parent_artifact_id.clone())))
    }

    pub fn new(tool_id: &'static str) -> Self {
        Self {
            tool_id,
            close_owners: Default::default(),
            stage: Puzzle3dPrecomputeCommandStage::Decode,
            object_cursor: 0,
            child_cursor: 0,
            attraction_cursor: 0,
            catalog_object_cursor: 0,
            catalog_vortex_cursor: 0,
            payload_cursor: 0,
            requested_count: 0,
            delta: if tool_id == "cycleBrushCandidateBack" { -1 } else { 1 },
            candidate_count: 0,
            processed_units: 0,
            session: None,
            prologue: Puzzle3dActionPrologue::default(),
            instance_owner: None,
        }
    }

    /// 🔤️ Validates one bounded slice of a `registerBrushMesh` page's base64 payload — the alphabet and,
    /// once the stream ends, its four-character grouping. `payload_cursor` returns to zero exactly when
    /// the stream is fully scanned, which is what advances the stage.
    fn scan_mesh_page(&mut self, command: &Puzzle3dCommand, key: &str) -> bool {
        let payload = command.args().and_then(|args| args.get(key)).and_then(Value::as_str).unwrap_or_default().as_bytes();
        let start = self.payload_cursor.min(payload.len());
        let end = payload.len().min(start.saturating_add(PUZZLE3D_MESH_PAGE_SCAN_CHARS));
        let admissible = payload[start..end].iter().all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'+' | b'/' | b'='));
        self.payload_cursor = if end >= payload.len() { 0 } else { end };
        admissible && payload.len().is_multiple_of(4)
    }
}

/// 🔤️ Base64 characters one interactive step of a `registerBrushMesh` page validates. A whole page is
/// at most [`crate::editor::puzzle3d::precompute::PUZZLE3D_MESH_PAGE_BASE64_CHARS`] characters, so a
/// page costs at most eleven steps per stream and no step approaches the lane's own budget.
const PUZZLE3D_MESH_PAGE_SCAN_CHARS: usize = 512;

impl<A: Puzzle3dEditor> semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>> for Puzzle3dPrecomputeCommandWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    /// 📏️ `registerBrushMesh` carries one page of a mesh upload run as two base64 payload strings, never
    /// as JSON number arrays — a whole document-scale mesh is 64 KB of them and the shared retained wire
    /// admits 8 192 bytes. The extent a page claims is the characters it must validate, so a page that
    /// declares more than [`crate::editor::puzzle3d::precompute::PUZZLE3D_MESH_PAGE_BASE64_CHARS`] per
    /// stream is refused before a single character is read.
    fn extent(&self, command: &Puzzle3dCommand, _snapshot: &Puzzle3dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<A>>>) -> Option<usize> {
        let payload = |key: &str| command.args().and_then(|args| args.get(key)).and_then(Value::as_str).map_or(0, str::len);
        let (positions, indices) = (payload("positionsB64"), payload("indicesB64"));
        let maximum = semio_s_artifact_puzzle_3d_precompute::PUZZLE3D_MESH_PAGE_BASE64_CHARS;
        let items = positions.max(indices).div_ceil(PUZZLE3D_MESH_PAGE_SCAN_CHARS).checked_mul(2)?.checked_add(1)?;
        (positions <= maximum && indices <= maximum && items <= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        semio_s_plugin_puzzle_retained::step_demands::<EditorApp<A>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config, interaction, hover: hover, context, .. } = *input;
        let view_state = context.and_then(|context| context.view_state.as_ref());
        let window_config_snapshot = context.and_then(|context| context.window_config.as_ref());
        let window_transient_snapshot = context.and_then(|context| context.window_transient.as_ref());
        if self.processed_units >= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS {
            return Err(Fault::from("puzzle3d-precompute-work-capacity"));
        }
        self.processed_units += 1;
        let document = snapshot.typed();
        match self.stage {
            Puzzle3dPrecomputeCommandStage::Decode => {
                self.requested_count = set_fill_count::parse_count(command.args());
                self.delta = command.args().and_then(|args| args.get("delta")).and_then(Value::as_i64).map_or(self.delta, |value| value.clamp(isize::MIN as i64, isize::MAX as i64) as isize);
                self.stage = match self.tool_id {
                    "registerBrushMesh" => Puzzle3dPrecomputeCommandStage::Positions,
                    "setFillCount" => Puzzle3dPrecomputeCommandStage::Publish,
                    _ => Puzzle3dPrecomputeCommandStage::Objects,
                };
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-precompute-decode", "Reading precompute command", "Vorberechnungsbefehl wird gelesen"))
            }
            Puzzle3dPrecomputeCommandStage::Objects => {
                let Some(object) = document.objects.get(self.object_cursor) else {
                    self.stage = Puzzle3dPrecomputeCommandStage::Attractions;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-precompute-attractions", "Scanning attraction owner", "Anziehungsinhaber wird geprüft"));
                };
                if object.vortices.len() > PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT {
                    return Err(Fault::from("puzzle3d-precompute-vortex-capacity"));
                }
                self.stage = Puzzle3dPrecomputeCommandStage::Vortices;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-precompute-object", "Scanning object owner", "Objektinhaber wird geprüft"))
            }
            Puzzle3dPrecomputeCommandStage::Vortices => {
                let object = document.objects.get(self.object_cursor).ok_or_else(|| Fault::from("puzzle3d-precompute-object-cursor"))?;
                if object.vortices.get(self.child_cursor).is_some() {
                    self.child_cursor += 1;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-precompute-vortex", "Scanning one vortex owner", "Ein Vortexinhaber wird geprüft"));
                }
                self.object_cursor += 1;
                self.child_cursor = 0;
                self.stage = Puzzle3dPrecomputeCommandStage::Objects;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-precompute-object", "Advancing object cursor", "Objektzeiger wird fortgesetzt"))
            }
            Puzzle3dPrecomputeCommandStage::Attractions => {
                if document.attractions.get(self.attraction_cursor).is_some() {
                    self.attraction_cursor += 1;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-precompute-attraction", "Scanning one attraction owner", "Ein Anziehungsinhaber wird geprüft"));
                }
                self.stage = Puzzle3dPrecomputeCommandStage::CatalogObjects;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-precompute-catalog-object", "Scanning object kind owner", "Objektartinhaber wird geprüft"))
            }
            Puzzle3dPrecomputeCommandStage::CatalogObjects => {
                let entries = document.meta.kind_catalogs.as_ref().map(|catalogs| catalogs.objects.as_slice()).unwrap_or_default();
                if entries.get(self.catalog_object_cursor).is_some() {
                    self.catalog_object_cursor += 1;
                    self.candidate_count += 1;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-precompute-catalog-object", "Scanning one object kind", "Eine Objektart wird geprüft"));
                }
                self.stage = Puzzle3dPrecomputeCommandStage::CatalogVortices;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-precompute-catalog-vortex", "Scanning vortex kind owner", "Vortexartinhaber wird geprüft"))
            }
            Puzzle3dPrecomputeCommandStage::CatalogVortices => {
                let entries = document.meta.kind_catalogs.as_ref().map(|catalogs| catalogs.vortices.as_slice()).unwrap_or_default();
                if entries.get(self.catalog_vortex_cursor).is_some() {
                    self.catalog_vortex_cursor += 1;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-precompute-catalog-vortex", "Scanning one vortex kind", "Eine Vortexart wird geprüft"));
                }
                self.stage = Puzzle3dPrecomputeCommandStage::PrologueScene;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-precompute-transfer", "Transferring precompute census", "Vorberechnungszensus wird übertragen"))
            }
            Puzzle3dPrecomputeCommandStage::Positions => {
                if !self.scan_mesh_page(command, "positionsB64") {
                    return Err(Fault::from("puzzle3d-register-mesh-position-malformed"));
                }
                if self.payload_cursor > 0 {
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-register-mesh-position", "Reading one mesh position page", "Eine Mesh-Positionsseite wird gelesen"));
                }
                self.stage = Puzzle3dPrecomputeCommandStage::Indices;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-register-mesh-index", "Reading mesh indices", "Mesh-Indizes werden gelesen"))
            }
            Puzzle3dPrecomputeCommandStage::Indices => {
                if !self.scan_mesh_page(command, "indicesB64") {
                    return Err(Fault::from("puzzle3d-register-mesh-index-malformed"));
                }
                if self.payload_cursor > 0 {
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-register-mesh-index", "Reading one mesh index page", "Eine Mesh-Indexseite wird gelesen"));
                }
                self.stage = Puzzle3dPrecomputeCommandStage::PrologueScene;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-register-mesh-transfer", "Transferring validated mesh owner", "Geprüfter Mesh-Inhaber wird übertragen"))
            }
            Puzzle3dPrecomputeCommandStage::PrologueScene => {
                let view = view_state.ok_or_else(|| Fault::from("puzzle3d-precompute-window-context-required"))?;
                let window_config = window_ownership::config_from_snapshot(window_config_snapshot);
                let window_transient = window_ownership::transient_from_snapshot(window_transient_snapshot);
                let runtime = window_ownership::runtime(config, &window_config, &window_transient, Some(view));
                let window_id = puzzle3d_addressed_window_id(Some(view), None, command.window_id(), &runtime.window_ids);
                self.prologue.scene_step(snapshot, &runtime, Some(view), Some(window_id));
                self.stage = Puzzle3dPrecomputeCommandStage::PrologueSync;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-action-scene", "Reading the artifact", "Artefakt wird gelesen"))
            }
            Puzzle3dPrecomputeCommandStage::PrologueSync => {
                let view = view_state.ok_or_else(|| Fault::from("puzzle3d-precompute-window-context-required"))?;
                let window_config = window_ownership::config_from_snapshot(window_config_snapshot);
                let window_transient = window_ownership::transient_from_snapshot(window_transient_snapshot);
                let runtime = window_ownership::runtime(config, &window_config, &window_transient, Some(view));
                let window_id = puzzle3d_addressed_window_id(Some(view), None, command.window_id(), &runtime.window_ids);
                let session = self.session();
                let owed = with_puzzle3d_session_for(session, &runtime, |app| self.prologue.sync_step(app, command.action_id(), &runtime, Some(view), Some(window_id)));
                if !owed {
                    self.stage = Puzzle3dPrecomputeCommandStage::Publish;
                }
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-action-sync", "Preparing the placement session", "Platzierungssitzung wird vorbereitet"))
            }
            Puzzle3dPrecomputeCommandStage::Publish => {
                if self.tool_id == "setFillCount" {
                    let config_mutations = (self.requested_count != config.fill_count).then_some(Puzzle3dConfigMutation::SetFillCount(Puzzle3dConfigSetFillCount{ count: self.requested_count })).into_iter().collect();
                    self.stage = Puzzle3dPrecomputeCommandStage::Complete;
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { config_mutations, ui_scope: puzzle3d_scope(puzzle3d_command_scope_class("setFillCount")), ..Default::default() }));
                }
                let view = view_state.ok_or_else(|| Fault::from("puzzle3d-precompute-window-context-required"))?;
                let window_config = window_ownership::config_from_snapshot(window_config_snapshot);
                let window_transient = window_ownership::transient_from_snapshot(window_transient_snapshot);
                let runtime = window_ownership::runtime(config, &window_config, &window_transient, Some(view));
                let snapshot_interaction = Puzzle3dInteractionSnapshot::from_state(interaction, hover);
                let window_id = puzzle3d_addressed_window_id(Some(view), None, command.window_id(), &runtime.window_ids);
                let session = self.session();
                let instance_owner = self.instance_owner.as_ref();
                let (emit, ephemeral) = with_puzzle3d_session_for(session, &runtime, |app| self.prologue.dispatch_step(app, instance_owner, "", command, Some(window_id), &runtime, Some(view), &snapshot_interaction));
                self.stage = Puzzle3dPrecomputeCommandStage::Complete;
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral })
            }
            Puzzle3dPrecomputeCommandStage::Complete => Err(Fault::from("puzzle3d-precompute-complete-repolled")),
            Puzzle3dPrecomputeCommandStage::Closing => Err(Fault::from("puzzle3d-precompute-closing")),
        }
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle3dPrecomputeCommandStage::Closing;
        self.close_owners.stage(Puzzle3dPrecomputeWorkOwners { prologue: std::mem::take(&mut self.prologue), session: self.session.take(), instance_owner: self.instance_owner.take() });
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        semio_framework_job::close_original_granted_step(cx,|grant|{
            self.close_owners.close_step(grant)
        })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle3dPrecomputeCommandStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}
