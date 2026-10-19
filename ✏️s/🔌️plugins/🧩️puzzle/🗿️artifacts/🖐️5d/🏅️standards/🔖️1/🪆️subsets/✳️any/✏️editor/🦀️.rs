//! (dispatch-only), the structural-twin document model its command/panel/window nodes mutate and
//! 👯️ Puzzle 5d play app — the plugin's unified 2d+3d play app: its `ArtifactApp` impl
//! render, the shared scene/engine/brush helpers those nodes reach for, and the manifest that
//! stitches them together.
//!
//! 🧭️ Every behavioural arm lives in `🎮️commands/<group>/🦀️.rs`; every rendered surface in
//! `📌️panels/<panel>` or `🎭️modes/✏️edit/🪟️windows/{◻2d,🧊️3d}`. This file dispatches and stitches.
//!
//! 🌉️ `ArtifactApp::Snapshot` is the `Puzzle5dPlaySnapshot` newtype over a bare
//! `Value` document (see `crate::standards::v1::subsets::any::io::text::mutations`'s `🔖️ValueBridge`), not the
//! typed `Puzzle5dSnapshot` — the `Puzzle5dDocument` model below is this app's own structural twin
//! of it, and each action emits the concrete typed mutation kinds of the gesture it carries
//! (`🎮️commands/<group>`), never a delta of two documents.
use crate::editor::puzzle5d::entities::puzzle5d_grip_full_id;

use crate::Puzzle5dPlaySnapshot;

use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;

use crate::Puzzle5dSnapshot;
use crate::editor::puzzle5d::commands::apply_sun;
use crate::editor::puzzle5d::commands::cycle_brush_candidate;
use crate::editor::puzzle5d::commands::delete_selection;
use crate::editor::puzzle5d::commands::duplicate_selection;
use crate::editor::puzzle5d::commands::engagement_abort;
use crate::editor::puzzle5d::commands::engagement_control_select;
use crate::editor::puzzle5d::commands::engagement_input;
use crate::editor::puzzle5d::commands::engagement_repeat_last;
use crate::editor::puzzle5d::commands::engagement_submit;
use crate::editor::puzzle5d::commands::register_brush_mesh;
use crate::editor::puzzle5d::commands::rotate_selection;
use crate::editor::puzzle5d::commands::scale_selection;
use crate::editor::puzzle5d::commands::select_same_kind;
use crate::editor::puzzle5d::commands::set_brush_placement_contact_tolerance;
use crate::editor::puzzle5d::commands::set_camera;
use crate::editor::puzzle5d::commands::set_camera_2d;
use crate::editor::puzzle5d::commands::set_camera_3d;
use crate::editor::puzzle5d::commands::set_fill_count;
use crate::editor::puzzle5d::commands::set_grid_factor;
use crate::editor::puzzle5d::commands::set_grid_snap_enabled;
use crate::editor::puzzle5d::commands::set_kind_weight;
use crate::editor::puzzle5d::commands::set_lod_mode;
use crate::editor::puzzle5d::commands::set_selection_flag;
use crate::editor::puzzle5d::commands::set_suggestion_offset;
use crate::editor::puzzle5d::commands::target_brush_suggestions;
use crate::editor::puzzle5d::commands::translate_selection;
use crate::editor::puzzle5d::commands::world_relocate;
use crate::editor::puzzle5d::commands::set_grid_spacing;
use crate::editor::puzzle5d::commands::set_grid_visible;
use crate::editor::puzzle5d::commands::set_grip_direction;
use crate::editor::puzzle5d::commands::set_grip_show;
use crate::editor::puzzle5d::commands::set_lod_automatic;
use crate::editor::puzzle5d::commands::set_lod_depth_variable;
use crate::editor::puzzle5d::commands::set_lod_manual;
use crate::editor::puzzle5d::commands::set_projection;
use crate::editor::puzzle5d::commands::set_selectable_kind;
use crate::editor::puzzle5d::commands::set_transform_gumball_flag;
use crate::editor::puzzle5d::commands::add_target_volume;
use crate::editor::puzzle5d::commands::delete_target_volume;
use crate::editor::puzzle5d::commands::relocate_target_volume;
use crate::editor::puzzle5d::commands::set_target_volume_flag;
use crate::editor::puzzle5d::commands::set_voxel_dims;
use crate::editor::puzzle5d::commands::set_chunk_size;
use crate::editor::puzzle5d::commands::set_proximity_radius;
use crate::editor::puzzle5d::commands::accept_suggestion;
use crate::editor::puzzle5d::commands::close_vortex_suggestions;
use crate::editor::puzzle5d::commands::hover_suggestion;
use crate::editor::puzzle5d::commands::open_vortex_suggestions;
use crate::editor::puzzle5d::commands::export_snapshot;
use crate::editor::puzzle5d::commands::import_snapshot;
use crate::editor::puzzle5d::commands::open_add_part_dialog;
use crate::editor::puzzle5d::commands::open_import_snapshot;
use crate::editor::puzzle5d::config::Puzzle5dCamera2d;
use crate::editor::puzzle5d::config::Puzzle5dConfig;
pub use crate::editor::puzzle5d::config::Puzzle5dKindWeights;
use crate::editor::puzzle5d::config::Puzzle5dConfigMutation;
use crate::editor::puzzle5d::config::Puzzle5dRuntime;
use crate::editor::puzzle5d::config::Puzzle5dConfigSetObjectKindWeights;
use crate::editor::puzzle5d::config::Puzzle5dConfigSetVortexKindWeights;
use crate::editor::puzzle5d::modes::edit;
use crate::editor::puzzle5d::modes::edit::tools::fill as fill_tool;
use crate::editor::puzzle5d::modes::edit::windows::board2d;
use crate::editor::puzzle5d::modes::edit::windows::world3d;
use crate::editor::puzzle5d::panels::catalogue;
use crate::editor::puzzle5d::panels::artifact as artifact_panel;
use crate::editor::puzzle5d::panels::inspection;
use crate::editor::puzzle5d::panels::settings as settings_panel;
use semio_s_artifact_puzzle_3d_editor::Puzzle3dInstanceOperationOwner;
use crate::editor::puzzle5d::presence::Puzzle5dPresence;
use crate::editor::puzzle5d::presence::Puzzle5dPresenceMutation;
use crate::editor::puzzle5d::terminology::puzzle5d_labels;
use crate::editor::puzzle5d::terminology::puzzle5d_localized;
use crate::editor::puzzle5d::terminology::Puzzle5dLabels;
use crate::editor::puzzle5d::window as window_ownership;
use semio_framework_plugin::kernel::ClipboardError;
use semio_framework_plugin::kernel::ClipboardFragment;
use semio_framework_plugin::kernel::Effect;
use semio_framework_plugin::kernel::PasteAnchor;
use semio_framework_plugin::kernel::PastePlacement;
use semio_framework_plugin::kernel::UiDirtyScope;
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionDescriptor;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::AppIo;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactPresentation;
use semio_framework_plugin::ArtifactReservedJob;
use semio_framework_plugin::ArtifactReservedToolInput;
use semio_framework_plugin::ArtifactReservedToolJob;
use semio_framework_plugin::ArtifactReservedToolJobRequest;
use semio_framework_plugin::ArtifactToolCompletion;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::EphemeralEmit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::DialogDefinition;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_plugin::InteractionRef;
use semio_framework_plugin::InteractionTarget;
use semio_framework_plugin::InteractionWrite;
use semio_framework_plugin::InteractiveJobClassification;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::Media;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaError;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaPortDirection;
use semio_framework_plugin::MediaPortSpec;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::PortMultiplicity;
use semio_framework_plugin::ToolRef;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_plugin::ToolExecutionContract;
use semio_framework_plugin::ToolFactoryKey;
use semio_framework_plugin::ToolJobFactory;
use semio_framework_plugin::ToolJobFactoryError;
use semio_framework_plugin::WindowEngagement;
use semio_framework_plugin::WindowMeasure;
use semio_framework_plugin::INTERACTION_SELECT_ACTION_ID;
// 🕹️ `InteractionView` — see 🧊️3d/🦀️.rs's identical import comment (missing top-level
// re-export from `semio_framework_plugin`, flagged to the coordinator, not fixed here).
use semio_framework_job::InteractiveJob;
use semio_framework_job::Operation;
use semio_framework_job::StepContext;
use semio_framework_plugin::app::InteractionView;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_pack_json::parse;
use semio_framework_pack_json::Value;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::LazyLock;
use semio_framework_2d_compute::EngineHandles;

pub use semio_s_artifact_puzzle_5d_mut_any_8::editor::puzzle5d::entities;
pub use entities::*;

pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::view;
pub use view::*;

pub use semio_s_artifact_puzzle_5d_catalog::editor::puzzle5d::catalog;
pub use catalog::*;

pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::interaction;
pub use interaction::*;

#[path = "🖌️session/🦀️.rs"]
pub mod session;
use session::*;

#[path = "🧰️kit/🦀️.rs"]
mod kit;
use kit::*;










//#endregion 🔖️Constants

//#region 🔖️Document














#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dPart {
    pub id: String,
    #[value(rename = "partKind")]
    pub part_kind: String,
    #[value(default)]
    pub anchor: Puzzle5dPartAnchor,
    #[value(default, rename = "2d")]
    pub part_2d: Puzzle5dPart2d,
    #[value(default, rename = "3d")]
    pub part_3d: Puzzle5dPart3d,
    #[value(default)]
    pub grips: Vec<Puzzle5dGrip>,
}



























































pub use crate::standards::v1::subsets::any::schema::mutations::quat_from_axis_angle;

pub use crate::standards::v1::subsets::any::schema::mutations::quat_mul;




















pub use crate::standards::v1::subsets::any::schema::mutations::PUZZLE5D_FLAT_TO_WORLD;
//#endregion 🔖️Document

//#region 🔖️Scene























//#endregion 🔖️Scene

//#region 🔖️Engine

//#endregion 🔖️Engine

//#region 🏷️Labels













//#endregion 🏷️Labels

//#region 🧬️InferredKinds



//#endregion 🧬️InferredKinds

//#region 🙈️SelectionFlags
//#endregion 🙈️SelectionFlags

//#region 🔖️Distribution









//#endregion 🔖️Distribution

//#region 🔖️CopyPaste











//#endregion 🔖️CopyPaste

//#region 🧵️ReservedJobs

















































impl Puzzle5dPasteJob {
    fn new(request: ArtifactReservedToolJobRequest<EditorApp<Puzzle5dPlayApp>>, args: Option<Value>) -> Self {
        Self {
            raw: request.raw_wire,
            raw_cursor: 0,
            raw_page: [0; PUZZLE5D_RESERVED_PAGE_BYTES],
            raw_page_len: 0,
            progress: 0,
            stage: Puzzle5dPasteStage::Envelope,
            snapshot: Some(request.snapshot),
            args,
            fragment_value: None,
            fragment_parts: Vec::new(),
            cursor: 0,
            source_sum: (0.0, 0.0),
            target_sum: (0.0, 0.0),
            target_count: 0,
            placement: PastePlacement::default(),
            delta: (0.0, 0.0),
            id_map: HashMap::new(),
            fresh_ids: Puzzle5dFreshIds::default(),
            mutations: Vec::new(),
            completion: Some(request.completion),
            pending_completion_rejection: None,
            completed: false,
            outbox: Default::default(),
            closing: false,
            owners: Default::default(),
        }
    }

    fn checkpoint(&self) -> crate::puzzle_job::JobTurn {
        puzzle5d_job_checkpoint(self.stage as u8, self.cursor, self.progress)
    }
}

impl Puzzle5dPasteJob {
    fn turn(&mut self, cx: &mut StepContext<'_>) -> crate::puzzle_job::JobTurn {
        if self.closing || cx.is_cancelled() {
            return crate::puzzle_job::JobTurn::Cancelled;
        }
        if self.pending_completion_rejection.is_some() {
            return puzzle5d_job_fault("puzzle5d paste completion remains rejected");
        }
        match self.stage {
            Puzzle5dPasteStage::Envelope => {
                if let Some(outcome) = puzzle5d_step_envelope(&self.raw, &mut self.raw_cursor, &mut self.raw_page, &mut self.raw_page_len, &mut self.progress, cx) {
                    return outcome;
                }
                self.stage = Puzzle5dPasteStage::Decode;
            }
            Puzzle5dPasteStage::Decode => {
                let Some(args) = self.args.as_ref() else {
                    self.stage = Puzzle5dPasteStage::Complete;
                    return self.checkpoint();
                };
                let Some(fragment_value) = args.get("fragment").cloned() else {
                    self.stage = Puzzle5dPasteStage::Complete;
                    return self.checkpoint();
                };
                let fragment: ClipboardFragment = match puzzle5d_record_from_projection(fragment_value) {
                    Ok(fragment) => fragment,
                    Err(error) => return puzzle5d_job_fault(error.to_string()),
                };
                if fragment.media_type != (MediaType { class: MediaClass::Kit, form: MediaForm::Design }) {
                    return puzzle5d_job_fault("puzzle5d paste received an incompatible media type");
                }
                if fragment.dsl_text.len() > PUZZLE5D_RESERVED_RAW_BYTES {
                    return puzzle5d_job_fault("puzzle5d paste fragment exceeds its predecode cap");
                }
                self.fragment_value = match parse(&fragment.dsl_text, semio_framework_pack_json::JsonMemberPolicy::Reject) {
                    Ok(value) => Some(value),
                    Err(error) => return puzzle5d_job_fault(error.to_string()),
                };
                self.placement = match puzzle5d_paste_placement(args) {
                    Ok(placement) => placement,
                    Err(error) => return puzzle5d_job_fault(error.to_string()),
                };
                self.stage = Puzzle5dPasteStage::FragmentParts;
                self.cursor = 0;
            }
            Puzzle5dPasteStage::FragmentParts => {
                let rows = self.fragment_value.as_ref().and_then(|value| value.get("parts")).and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[]);
                if let Some(row) = rows.get(self.cursor).cloned() {
                    self.cursor += 1;
                    let part: Puzzle5dPart = match puzzle5d_record_from_projection(row) {
                        Ok(part) => part,
                        Err(error) => return puzzle5d_job_fault(error.to_string()),
                    };
                    self.source_sum.0 += part.part_2d.x;
                    self.source_sum.1 += part.part_2d.y;
                    self.fragment_parts.push(part);
                } else {
                    self.stage = Puzzle5dPasteStage::TargetParts;
                    self.cursor = 0;
                }
            }
            Puzzle5dPasteStage::TargetParts => {
                let rows = self.snapshot.as_ref().map(|snapshot| &snapshot.typed().parts).map(Vec::as_slice).unwrap_or(&[]);
                if let Some(row) = rows.get(self.cursor) {
                    self.cursor += 1;
                    self.fresh_ids.observe_part(&row.id);
                    self.target_sum.0 += row.part_2d.x;
                    self.target_sum.1 += row.part_2d.y;
                    self.target_count += 1;
                } else {
                    let offset = self.placement.position.map_or((0.0, 0.0), |position| (position[0], position[1]));
                    self.delta = if matches!(self.placement.anchor, PasteAnchor::Original) || self.fragment_parts.is_empty() || self.target_count == 0 {
                        offset
                    } else {
                        (self.target_sum.0 / self.target_count as f64 - self.source_sum.0 / self.fragment_parts.len() as f64 + offset.0, self.target_sum.1 / self.target_count as f64 - self.source_sum.1 / self.fragment_parts.len() as f64 + offset.1)
                    };
                    self.stage = Puzzle5dPasteStage::TargetFasteners;
                    self.cursor = 0;
                }
            }
            Puzzle5dPasteStage::TargetFasteners => {
                let rows = self.snapshot.as_ref().map(|snapshot| &snapshot.typed().fasteners).map(Vec::as_slice).unwrap_or(&[]);
                if let Some(row) = rows.get(self.cursor) {
                    self.cursor += 1;
                    self.fresh_ids.observe_fastener(&row.id);
                } else {
                    self.stage = Puzzle5dPasteStage::MaterializeParts;
                    self.cursor = 0;
                }
            }
            Puzzle5dPasteStage::MaterializeParts => {
                if let Some(part) = self.fragment_parts.get(self.cursor).cloned() {
                    self.cursor += 1;
                    let fresh_id = self.fresh_ids.next_part();
                    self.id_map.insert(part.id.clone(), fresh_id.clone());
                    let mut next = part;
                    next.id = fresh_id;
                    next.part_2d.x += self.delta.0;
                    next.part_2d.y += self.delta.1;
                    next.part_3d.origin[0] += self.delta.0;
                    next.part_3d.origin[1] += self.delta.1;
                    let typed = match <crate::Puzzle5dPart as FromValue>::from_value(next.to_value()) {
                        Ok(typed) => typed,
                        Err(error) => return puzzle5d_job_fault(error.to_string()),
                    };
                    self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::create_part(typed, None));
                } else {
                    self.stage = Puzzle5dPasteStage::MaterializeFasteners;
                    self.cursor = 0;
                }
            }
            Puzzle5dPasteStage::MaterializeFasteners => {
                let rows = self.fragment_value.as_ref().and_then(|value| value.get("fasteners")).and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[]);
                if let Some(row) = rows.get(self.cursor).cloned() {
                    self.cursor += 1;
                    let fastener: Puzzle5dFastener = match puzzle5d_record_from_projection(row) {
                        Ok(fastener) => fastener,
                        Err(error) => return puzzle5d_job_fault(error.to_string()),
                    };
                    self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::connect_grips(
                        self.fresh_ids.next_fastener(),
                        rewrite_grip_ref_local(&fastener.source, &self.id_map),
                        rewrite_grip_ref_local(&fastener.target, &self.id_map),
                        fastener.fastener_kind,
                        fastener.gap,
                        fastener.shift,
                        fastener.rise,
                        fastener.rotation,
                        fastener.turn,
                        fastener.tilt,
                        fastener.x + self.delta.0,
                        fastener.y + self.delta.1, None,
                    ));
                } else {
                    self.stage = Puzzle5dPasteStage::Complete;
                }
            }
            Puzzle5dPasteStage::Complete => {
                if !self.raw.is_empty() {
                    return crate::puzzle_job::JobTurn::Prepare(std::mem::take(&mut self.raw));
                }
                if !self.completed {
                    // 🕹️ The pasted parts BECOME the selection: a paste whose result is invisible until the
                    // user hunts for it is indistinguishable from one that landed nowhere, and the fresh ids
                    // are the only handle on it. Sorted, so one fragment always re-selects in one order.
                    let mut fresh: Vec<String> = self.id_map.values().cloned().collect();
                    fresh.sort();
                    let emit = Emit { artifact_mutations: std::mem::take(&mut self.mutations), interaction_writes: vec![InteractionWrite::replace(PUZZLE5D_INTERACTION_DOMAIN, PUZZLE5D_GRANULARITY_PART, fresh)], ..Default::default() };
                    let Some(completion) = self.completion.as_ref() else { return puzzle5d_job_fault("puzzle5d paste lost its completion authority") };
                    if let Err(rejected) = completion.complete(Ok(emit), EphemeralEmit::default()) {
                        let message = rejected.fault.message.clone();
                        self.pending_completion_rejection = Some(rejected);
                        return puzzle5d_job_fault(message);
                    }
                    self.completed = true;
                }
                return crate::puzzle_job::JobTurn::Complete;
            }
        }
        self.progress = self.progress.saturating_add(1);
        cx.consume_fuel(1);
        self.checkpoint()
    }
}
















impl Puzzle5dImportJob {
    fn new(request: ArtifactReservedToolJobRequest<EditorApp<Puzzle5dPlayApp>>, port: String, media: Media) -> Self {
        let media_json = match media.payload {
            semio_framework_plugin::MediaPayload::Structured { json, .. } => Some(json),
            semio_framework_plugin::MediaPayload::Intrinsic { value, .. } => Some(semio_framework_pack_json::to_json_string(&value)),
            semio_framework_plugin::MediaPayload::Binary { .. } => None,
        };
        Self {
            raw: request.raw_wire,
            raw_cursor: 0,
            raw_page: [0; PUZZLE5D_RESERVED_PAGE_BYTES],
            raw_page_len: 0,
            progress: 0,
            stage: Puzzle5dImportStage::Envelope,
            port,
            media_json,
            snapshot: Some(request.snapshot),
            fragment: None,
            catalogs: Default::default(),
            had_catalogs: false,
            catalog_changed: false,
            compatibility: Vec::new(),
            part_index: Vec::new(),
            grip_index: Vec::new(),
            compatibility_index: Vec::new(),
            mutation_pages: std::array::from_fn(|_| Vec::new()),
            cursor: 0,
            nested_cursor: 0,
            decoded_items: 0,
            current_part: None,
            completion: Some(request.completion),
            pending_completion_rejection: None,
            completed: false,
            outbox: Default::default(),
            closing: false,
            owners: Default::default(),
        }
    }

    fn checkpoint(&self) -> crate::puzzle_job::JobTurn {
        puzzle5d_import_checkpoint(self.stage as u8, self.cursor, self.nested_cursor, self.decoded_items, self.progress)
    }

    fn rows(&self, key: &str) -> &[Value] {
        self.fragment.as_ref().and_then(|value| value.get(key)).and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[])
    }

    fn snapshot_rows(&self, parent: &str, key: &str) -> Vec<Value> {
        self.snapshot.as_ref().map(|snapshot| puzzle5d_editor_projection(snapshot)).and_then(|projection| projection.get(parent).and_then(|value| value.get(key)).and_then(Value::as_array).cloned()).unwrap_or_default()
    }

    fn snapshot_kind_compatibility_rows(&self) -> Vec<Value> {
        self.snapshot.as_ref().map(|snapshot| puzzle5d_editor_projection(snapshot)).and_then(|projection| projection.get("kindCompatibility").and_then(Value::as_array).cloned()).unwrap_or_default()
    }

    fn push_mutation(&mut self, mutation: Puzzle5dMutation) -> Result<(), &'static str> {
        let mutation_count = self.mutation_pages.iter().map(Vec::len).sum::<usize>();
        if mutation_count >= PUZZLE5D_IMPORT_MUTATION_ITEMS {
            return Err("puzzle5d kit:in mutation limit exceeded");
        }
        let page_index = mutation_count / PUZZLE5D_IMPORT_MUTATIONS_PER_PAGE;
        let Some(page) = self.mutation_pages.get_mut(page_index) else {
            return Err("puzzle5d kit:in mutation page limit exceeded");
        };
        if page.len() == page.capacity() {
            return Err("puzzle5d kit:in mutation page reserve exhausted");
        }
        page.push(mutation);
        Ok(())
    }
}

impl Puzzle5dImportJob {
    fn turn(&mut self, cx: &mut StepContext<'_>) -> crate::puzzle_job::JobTurn {
        if self.closing || cx.is_cancelled() {
            return crate::puzzle_job::JobTurn::Cancelled;
        }
        if self.pending_completion_rejection.is_some() {
            return puzzle5d_job_fault("puzzle5d import completion remains rejected");
        }
        match self.stage {
            Puzzle5dImportStage::Envelope => {
                if let Some(outcome) = puzzle5d_step_envelope(&self.raw, &mut self.raw_cursor, &mut self.raw_page, &mut self.raw_page_len, &mut self.progress, cx) {
                    return outcome;
                }
                self.stage = Puzzle5dImportStage::Decode;
            }
            Puzzle5dImportStage::Decode => {
                if self.port != "kit:in" {
                    return puzzle5d_job_fault("puzzle5d import only implements kit:in");
                }
                let Some(media_json) = self.media_json.as_ref() else {
                    return puzzle5d_job_fault("puzzle5d kit:in requires a structured payload");
                };
                let fragment = match puzzle5d_decode_import_fragment(media_json) {
                    Ok(fragment) => fragment,
                    Err(error) => return puzzle5d_job_fault(error),
                };
                let fragment_items = ["objectKinds", "vortexKinds", "cableKinds", "attractionKinds", "kindCompatibility"].into_iter().try_fold(0usize, |total, key| total.checked_add(fragment.get(key).and_then(Value::as_array).map_or(0, Vec::len)));
                let snapshot_items = ["parts", "grips", "fasteners", "ropes"]
                    .into_iter()
                    .try_fold(0usize, |total, key| total.checked_add(self.snapshot_rows("kindCatalogs", key).len()))
                    .and_then(|total| total.checked_add(self.snapshot_kind_compatibility_rows().len()));
                self.decoded_items = match fragment_items.and_then(|fragment_items| snapshot_items.and_then(|snapshot_items| fragment_items.checked_add(snapshot_items))) {
                    Some(items) if items <= PUZZLE5D_IMPORT_DECODED_ITEMS => items,
                    _ => return puzzle5d_job_fault("puzzle5d kit:in decoded item limit exceeded"),
                };
                if fragment.as_object().is_none() {
                    return puzzle5d_job_fault("puzzle5d kit:in root must be an object");
                }
                if !puzzle5d_import_keys_are(&fragment, &["schema", "objectKinds", "vortexKinds", "cableKinds", "attractionKinds", "kindCompatibility"]) {
                    return puzzle5d_job_fault("puzzle5d kit:in root contains an unknown field");
                }
                if fragment.get("schema").is_some_and(|value| value.as_str() != Some("manifest")) {
                    return puzzle5d_job_fault("puzzle5d kit:in schema must be manifest when present");
                }
                if ["objectKinds", "vortexKinds", "cableKinds", "attractionKinds", "kindCompatibility"].into_iter().any(|key| fragment.get(key).is_some_and(|value| value.as_array().is_none())) {
                    return puzzle5d_job_fault("puzzle5d kit:in collection must be an array");
                }
                if ["objectKinds", "vortexKinds", "cableKinds", "attractionKinds", "kindCompatibility"].into_iter().any(|key| fragment.get(key).and_then(Value::as_array).is_some_and(|rows| rows.len() > PUZZLE5D_IMPORT_SEMANTIC_ITEMS)) {
                    return puzzle5d_job_fault("puzzle5d kit:in collection exceeds its fixed-page descriptor cap");
                }
                if ["cableKinds", "attractionKinds"].into_iter().any(|key| fragment.get(key).and_then(Value::as_array).is_some_and(|rows| !rows.is_empty())) {
                    return puzzle5d_job_fault("puzzle5d kit:in cannot silently discard unmapped cable or attraction kinds");
                }
                self.had_catalogs = self.snapshot.as_ref().is_some_and(|snapshot| snapshot.typed().kind_catalogs.is_some());
                self.catalog_changed = !self.had_catalogs;
                self.fragment = Some(fragment);
                self.stage = Puzzle5dImportStage::CensusParts;
                self.cursor = 0;
            }
            Puzzle5dImportStage::CensusParts => {
                if let Some(row) = self.rows("objectKinds").get(self.cursor) {
                    if !puzzle5d_import_keys_are(row, &["id", "name", "label", "meshUrl", "vortices"]) {
                        return puzzle5d_job_fault("puzzle5d kit:in object kind contains an unknown field");
                    }
                    if row.get("vortices").is_some_and(|value| value.as_array().is_none()) {
                        return puzzle5d_job_fault("puzzle5d kit:in object-kind vortices must be an array");
                    }
                    let vortices = row.get("vortices").and_then(Value::as_array).map_or(0, Vec::len);
                    if vortices > PUZZLE5D_IMPORT_SEMANTIC_ITEMS {
                        return puzzle5d_job_fault("puzzle5d kit:in vortex collection exceeds its fixed-page descriptor cap");
                    }
                    self.decoded_items = match self.decoded_items.checked_add(vortices) {
                        Some(items) if items <= PUZZLE5D_IMPORT_DECODED_ITEMS => items,
                        _ => return puzzle5d_job_fault("puzzle5d kit:in nested vortex item limit exceeded"),
                    };
                    self.cursor += 1;
                } else {
                    self.stage = Puzzle5dImportStage::ReserveCatalogParts;
                    self.cursor = 0;
                }
            }
            Puzzle5dImportStage::ReserveCatalogParts => {
                let capacity = self.snapshot_rows("kindCatalogs", "parts").len().saturating_add(self.rows("objectKinds").len());
                if capacity > PUZZLE5D_IMPORT_SEMANTIC_ITEMS || self.catalogs.parts.try_reserve_exact(capacity).is_err() {
                    return puzzle5d_job_fault("puzzle5d kit:in part catalog reserve rejected");
                }
                self.stage = Puzzle5dImportStage::ReserveCatalogGrips;
            }
            Puzzle5dImportStage::ReserveCatalogGrips => {
                let capacity = self.snapshot_rows("kindCatalogs", "grips").len().saturating_add(self.rows("vortexKinds").len());
                if capacity > PUZZLE5D_IMPORT_SEMANTIC_ITEMS || self.catalogs.grips.try_reserve_exact(capacity).is_err() {
                    return puzzle5d_job_fault("puzzle5d kit:in grip catalog reserve rejected");
                }
                self.stage = Puzzle5dImportStage::ReserveCatalogFasteners;
            }
            Puzzle5dImportStage::ReserveCatalogFasteners => {
                let capacity = self.snapshot_rows("kindCatalogs", "fasteners").len();
                if capacity > PUZZLE5D_IMPORT_SEMANTIC_ITEMS || self.catalogs.fasteners.try_reserve_exact(capacity).is_err() {
                    return puzzle5d_job_fault("puzzle5d kit:in fastener catalog reserve rejected");
                }
                self.stage = Puzzle5dImportStage::ReserveCatalogRopes;
            }
            Puzzle5dImportStage::ReserveCatalogRopes => {
                let capacity = self.snapshot_rows("kindCatalogs", "ropes").len();
                if capacity > PUZZLE5D_IMPORT_SEMANTIC_ITEMS || self.catalogs.ropes.try_reserve_exact(capacity).is_err() {
                    return puzzle5d_job_fault("puzzle5d kit:in rope catalog reserve rejected");
                }
                self.stage = Puzzle5dImportStage::ReserveCompatibility;
            }
            Puzzle5dImportStage::ReserveCompatibility => {
                let capacity = self.snapshot_kind_compatibility_rows().len().saturating_add(self.rows("kindCompatibility").len());
                if capacity > PUZZLE5D_IMPORT_SEMANTIC_ITEMS || self.compatibility.try_reserve_exact(capacity).is_err() {
                    return puzzle5d_job_fault("puzzle5d kit:in compatibility reserve rejected");
                }
                self.stage = Puzzle5dImportStage::ReservePartIndex;
            }
            Puzzle5dImportStage::ReservePartIndex => {
                let capacity = self.snapshot_rows("kindCatalogs", "parts").len().saturating_add(self.rows("objectKinds").len());
                if capacity > PUZZLE5D_IMPORT_SEMANTIC_ITEMS || self.part_index.try_reserve_exact(capacity).is_err() {
                    return puzzle5d_job_fault("puzzle5d kit:in part index reserve rejected");
                }
                self.stage = Puzzle5dImportStage::ReserveGripIndex;
            }
            Puzzle5dImportStage::ReserveGripIndex => {
                let capacity = self.snapshot_rows("kindCatalogs", "grips").len().saturating_add(self.rows("vortexKinds").len());
                if capacity > PUZZLE5D_IMPORT_SEMANTIC_ITEMS || self.grip_index.try_reserve_exact(capacity).is_err() {
                    return puzzle5d_job_fault("puzzle5d kit:in grip index reserve rejected");
                }
                self.stage = Puzzle5dImportStage::ReserveCompatibilityIndex;
            }
            Puzzle5dImportStage::ReserveCompatibilityIndex => {
                let capacity = self.snapshot_kind_compatibility_rows().len().saturating_add(self.rows("kindCompatibility").len());
                if capacity > PUZZLE5D_IMPORT_SEMANTIC_ITEMS || self.compatibility_index.try_reserve_exact(capacity).is_err() {
                    return puzzle5d_job_fault("puzzle5d kit:in compatibility index reserve rejected");
                }
                self.stage = Puzzle5dImportStage::ReserveMutations;
                self.nested_cursor = 0;
            }
            Puzzle5dImportStage::ReserveMutations => {
                let capacity = self.rows("kindCompatibility").len().saturating_mul(2).saturating_add(1);
                if capacity > PUZZLE5D_IMPORT_MUTATION_ITEMS {
                    return puzzle5d_job_fault("puzzle5d kit:in mutation reserve rejected");
                }
                let page_start = self.nested_cursor.saturating_mul(PUZZLE5D_IMPORT_MUTATIONS_PER_PAGE);
                if page_start < capacity {
                    let page_items = capacity.saturating_sub(page_start).min(PUZZLE5D_IMPORT_MUTATIONS_PER_PAGE);
                    let Some(page) = self.mutation_pages.get_mut(self.nested_cursor) else {
                        return puzzle5d_job_fault("puzzle5d kit:in mutation page reserve rejected");
                    };
                    if page.try_reserve_exact(page_items).is_err() {
                        return puzzle5d_job_fault("puzzle5d kit:in mutation page reserve rejected");
                    }
                    self.nested_cursor += 1;
                    return self.checkpoint();
                }
                self.stage = Puzzle5dImportStage::LoadCatalogParts;
                self.cursor = 0;
                self.nested_cursor = 0;
            }
            Puzzle5dImportStage::LoadCatalogParts => {
                if let Some(row) = self.snapshot_rows("kindCatalogs", "parts").get(self.cursor) {
                    let parsed = match <crate::Puzzle5dCatalogPartKind as FromValue>::from_value(semio_framework_pack_json::to_dsl_value(row)) {
                        Ok(parsed) => parsed,
                        Err(error) => return puzzle5d_job_fault(error.to_string()),
                    };
                    self.catalogs.parts.push(parsed);
                    self.cursor += 1;
                } else {
                    self.stage = Puzzle5dImportStage::LoadCatalogGrips;
                    self.cursor = 0;
                }
            }
            Puzzle5dImportStage::LoadCatalogGrips => {
                if let Some(row) = self.snapshot_rows("kindCatalogs", "grips").get(self.cursor) {
                    let parsed = match <crate::Puzzle5dCatalogGripKind as FromValue>::from_value(semio_framework_pack_json::to_dsl_value(row)) {
                        Ok(parsed) => parsed,
                        Err(error) => return puzzle5d_job_fault(error.to_string()),
                    };
                    self.catalogs.grips.push(parsed);
                    self.cursor += 1;
                } else {
                    self.stage = Puzzle5dImportStage::LoadCatalogFasteners;
                    self.cursor = 0;
                }
            }
            Puzzle5dImportStage::LoadCatalogFasteners => {
                if let Some(row) = self.snapshot_rows("kindCatalogs", "fasteners").get(self.cursor) {
                    let parsed = match <crate::Puzzle5dCatalogFastenerKind as FromValue>::from_value(semio_framework_pack_json::to_dsl_value(row)) {
                        Ok(parsed) => parsed,
                        Err(error) => return puzzle5d_job_fault(error.to_string()),
                    };
                    self.catalogs.fasteners.push(parsed);
                    self.cursor += 1;
                } else {
                    self.stage = Puzzle5dImportStage::LoadCatalogRopes;
                    self.cursor = 0;
                }
            }
            Puzzle5dImportStage::LoadCatalogRopes => {
                if let Some(row) = self.snapshot_rows("kindCatalogs", "ropes").get(self.cursor) {
                    let parsed = match <crate::Puzzle5dCatalogRopeKind as FromValue>::from_value(semio_framework_pack_json::to_dsl_value(row)) {
                        Ok(parsed) => parsed,
                        Err(error) => return puzzle5d_job_fault(error.to_string()),
                    };
                    self.catalogs.ropes.push(parsed);
                    self.cursor += 1;
                } else {
                    self.stage = Puzzle5dImportStage::LoadCompatibility;
                    self.cursor = 0;
                }
            }
            Puzzle5dImportStage::LoadCompatibility => {
                let rows = self.snapshot_kind_compatibility_rows();
                if let Some(row) = rows.get(self.cursor) {
                    let parsed = match <crate::Puzzle5dKindCompatibility as FromValue>::from_value(semio_framework_pack_json::to_dsl_value(row)) {
                        Ok(parsed) => parsed,
                        Err(error) => return puzzle5d_job_fault(error.to_string()),
                    };
                    self.compatibility.push(parsed);
                    self.cursor += 1;
                } else {
                    self.stage = Puzzle5dImportStage::IndexParts;
                    self.cursor = 0;
                }
            }
            Puzzle5dImportStage::IndexParts => {
                if let Some(row) = self.catalogs.parts.get(self.cursor) {
                    self.part_index.push((row.id.clone(), self.cursor));
                    self.cursor += 1;
                } else {
                    self.stage = Puzzle5dImportStage::IndexGrips;
                    self.cursor = 0;
                }
            }
            Puzzle5dImportStage::IndexGrips => {
                if let Some(row) = self.catalogs.grips.get(self.cursor) {
                    self.grip_index.push((row.id.clone(), self.cursor));
                    self.cursor += 1;
                } else {
                    self.stage = Puzzle5dImportStage::IndexCompatibility;
                    self.cursor = 0;
                }
            }
            Puzzle5dImportStage::IndexCompatibility => {
                if let Some(row) = self.compatibility.get(self.cursor) {
                    self.compatibility_index.push(((row.source.clone(), row.target.clone()), self.cursor));
                    self.cursor += 1;
                } else {
                    self.stage = Puzzle5dImportStage::Parts;
                    self.cursor = 0;
                }
            }
            Puzzle5dImportStage::Parts => {
                if let Some(row) = self.rows("objectKinds").get(self.cursor) {
                    let id = match row.get("id").and_then(Value::as_str) {
                        Some(value) => value.to_string(),
                        None => return puzzle5d_job_fault("puzzle5d kit:in object kind lacks id"),
                    };
                    let name = match row.get("name").and_then(Value::as_str) {
                        Some(value) => value.to_string(),
                        None => return puzzle5d_job_fault("puzzle5d kit:in object kind lacks name"),
                    };
                    let label = match row.get("label").and_then(Value::as_str) {
                        Some(value) => value.to_string(),
                        None => return puzzle5d_job_fault("puzzle5d kit:in object kind lacks label"),
                    };
                    let mesh_url = row.get("meshUrl").and_then(Value::as_str).map(str::to_string);
                    self.current_part = Some(crate::Puzzle5dCatalogPartKind {
                        id,
                        name,
                        label,
                        representations: mesh_url.map(|url| vec![crate::Puzzle5dRepresentation { id: "mesh".into(), name: "mesh".into(), url, mime: "model/gltf-binary".into(), ..Default::default() }]).unwrap_or_default(),
                        grips: Vec::new(),
                        ..Default::default()
                    });
                    self.nested_cursor = 0;
                    self.stage = Puzzle5dImportStage::PartReserve;
                } else {
                    self.stage = Puzzle5dImportStage::Grips;
                    self.cursor = 0;
                }
            }
            Puzzle5dImportStage::PartReserve => {
                let grip_count = self.rows("objectKinds").get(self.cursor).and_then(|row| row.get("vortices")).and_then(Value::as_array).map_or(0, Vec::len);
                let Some(part) = self.current_part.as_mut() else {
                    return puzzle5d_job_fault("puzzle5d kit:in lost its part before nested reserve");
                };
                if grip_count > PUZZLE5D_IMPORT_SEMANTIC_ITEMS || part.grips.try_reserve_exact(grip_count).is_err() {
                    return puzzle5d_job_fault("puzzle5d kit:in nested grip reserve rejected");
                }
                self.stage = Puzzle5dImportStage::PartVortices;
            }
            Puzzle5dImportStage::PartVortices => {
                let vortex = self.rows("objectKinds").get(self.cursor).and_then(|row| row.get("vortices")).and_then(Value::as_array).and_then(|rows| rows.get(self.nested_cursor));
                if let Some(vortex) = vortex {
                    if !puzzle5d_import_keys_are(vortex, &["id", "vortexKind", "position", "direction", "radius"]) {
                        return puzzle5d_job_fault("puzzle5d kit:in vortex contains an unknown field");
                    }
                    let vortex_kind = match vortex.get("vortexKind").and_then(Value::as_str) {
                        Some(value) => value.to_string(),
                        None => return puzzle5d_job_fault("puzzle5d kit:in vortex lacks kind"),
                    };
                    let point = match puzzle5d_import_vec3(vortex.get("position")) {
                        Ok(value) => value,
                        Err(error) => return puzzle5d_job_fault(error),
                    };
                    let direction = match puzzle5d_import_vec3(vortex.get("direction")) {
                        Ok(value) => value,
                        Err(error) => return puzzle5d_job_fault(error),
                    };
                    let radius = match vortex.get("radius").and_then(Value::as_f64).filter(|value| value.is_finite()) {
                        Some(value) => value,
                        None => return puzzle5d_job_fault("puzzle5d kit:in vortex lacks finite radius"),
                    };
                    let Some(part) = self.current_part.as_mut() else {
                        return puzzle5d_job_fault("puzzle5d kit:in lost its current part owner");
                    };
                    part.grips.push(crate::Puzzle5dGripTemplate {
                        id: format!("g{}", self.nested_cursor),
                        name: vortex_kind.clone(),
                        label: vortex_kind.clone(),
                        grip_kind: Some(vortex_kind),
                        point,
                        direction,
                        radius: Some(radius),
                        ..Default::default()
                    });
                    self.nested_cursor += 1;
                } else {
                    self.stage = Puzzle5dImportStage::PartPublish;
                }
            }
            Puzzle5dImportStage::PartPublish => {
                let Some(next) = self.current_part.take() else {
                    return puzzle5d_job_fault("puzzle5d kit:in lost its completed part owner");
                };
                let id = next.id.clone();
                match self.part_index.iter().find_map(|(candidate, index)| (candidate == &id).then_some(*index)) {
                    Some(index) => self.catalogs.parts[index] = next,
                    None => {
                        self.part_index.push((id, self.catalogs.parts.len()));
                        self.catalogs.parts.push(next);
                    }
                }
                self.catalog_changed = true;
                self.cursor += 1;
                self.stage = Puzzle5dImportStage::Parts;
            }
            Puzzle5dImportStage::Grips => {
                if let Some(row) = self.rows("vortexKinds").get(self.cursor) {
                    if !puzzle5d_import_keys_are(row, &["id", "name", "label", "color", "defaultCableKind"]) {
                        return puzzle5d_job_fault("puzzle5d kit:in vortex kind contains an unknown field");
                    }
                    let id = match row.get("id").and_then(Value::as_str) {
                        Some(value) => value.to_string(),
                        None => return puzzle5d_job_fault("puzzle5d kit:in vortex kind lacks id"),
                    };
                    let next = crate::Puzzle5dCatalogGripKind {
                        id: id.clone(),
                        code: row.get("name").and_then(Value::as_str).map(str::to_string),
                        label: row.get("label").and_then(Value::as_str).map(str::to_string),
                        color: row.get("color").and_then(Value::as_str).unwrap_or_default().to_string(),
                        default_rope_kind: row.get("defaultCableKind").and_then(Value::as_str).unwrap_or_default().to_string(),
                        ..Default::default()
                    };
                    match self.grip_index.iter().find_map(|(candidate, index)| (candidate == &id).then_some(*index)) {
                        Some(index) => self.catalogs.grips[index] = next,
                        None => {
                            self.grip_index.push((id, self.catalogs.grips.len()));
                            self.catalogs.grips.push(next);
                        }
                    }
                    self.catalog_changed = true;
                    self.cursor += 1;
                } else {
                    self.stage = Puzzle5dImportStage::Compatibility;
                    self.cursor = 0;
                }
            }
            Puzzle5dImportStage::Compatibility => {
                if let Some(row) = self.rows("kindCompatibility").get(self.cursor) {
                    if !puzzle5d_import_keys_are(row, &["source", "target", "bidirectional", "important", "specificity"]) {
                        return puzzle5d_job_fault("puzzle5d kit:in compatibility contains an unknown field");
                    }
                    let parsed = match <crate::Puzzle5dKindCompatibility as FromValue>::from_value(semio_framework_pack_json::to_dsl_value(row)) {
                        Ok(parsed) => parsed,
                        Err(error) => return puzzle5d_job_fault(error.to_string()),
                    };
                    let key = (parsed.source.clone(), parsed.target.clone());
                    match self.compatibility_index.iter().find_map(|(candidate, index)| (candidate == &key).then_some(*index)) {
                        Some(index) if self.compatibility[index] == parsed => {}
                        Some(index) => {
                            if let Err(error) = self.push_mutation(crate::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility(parsed.source.clone(), parsed.target.clone())) {
                                return puzzle5d_job_fault(error);
                            }
                            if let Err(error) = self.push_mutation(crate::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility(parsed.source.clone(), parsed.target.clone(), parsed.bidirectional, parsed.important, parsed.specificity, None)) {
                                return puzzle5d_job_fault(error);
                            }
                            self.compatibility[index] = parsed;
                        }
                        None => {
                            if let Err(error) = self.push_mutation(crate::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility(parsed.source.clone(), parsed.target.clone(), parsed.bidirectional, parsed.important, parsed.specificity, None)) {
                                return puzzle5d_job_fault(error);
                            }
                            self.compatibility_index.push((key, self.compatibility.len()));
                            self.compatibility.push(parsed);
                        }
                    }
                    self.cursor += 1;
                } else {
                    self.stage = Puzzle5dImportStage::CatalogMutation;
                }
            }
            Puzzle5dImportStage::CatalogMutation => {
                if self.catalog_changed {
                    let mutation = crate::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs(Some(std::mem::take(&mut self.catalogs)));
                    if let Err(error) = self.push_mutation(mutation) {
                        return puzzle5d_job_fault(error);
                    }
                    self.catalog_changed = false;
                }
                self.stage = Puzzle5dImportStage::Complete;
            }
            Puzzle5dImportStage::Complete => {
                if !self.raw.is_empty() {
                    return crate::puzzle_job::JobTurn::Prepare(std::mem::take(&mut self.raw));
                }
                if !self.completed {
                    let mut mutation_pages = std::mem::take(&mut self.mutation_pages);
                    let mut mutations = std::mem::take(&mut mutation_pages[0]);
                    for page in mutation_pages.iter_mut().skip(1) {
                        mutations.append(page);
                    }
                    let Some(completion) = self.completion.as_ref() else { return puzzle5d_job_fault("puzzle5d import lost its completion authority") };
                    if let Err(rejected) = completion.complete(Ok(Emit::mutations(mutations)), EphemeralEmit::default()) {
                        let message = rejected.fault.message.clone();
                        self.pending_completion_rejection = Some(rejected);
                        return puzzle5d_job_fault(message);
                    }
                    self.completed = true;
                }
                return crate::puzzle_job::JobTurn::Complete;
            }
        }
        self.progress = self.progress.saturating_add(1);
        cx.consume_fuel(1);
        self.checkpoint()
    }
}




//#endregion 🧵️ReservedJobs

//#region 🔖️ContextMenu







//#endregion 🔖️ContextMenu

//#region 🔖️Puzzle5dCommand
/// 🎯️ B1: `Puzzle5dPlayApp::Command` — the SOLE dispatch surface, one variant per declared
/// action (mirrors every `.mutation(...)`/`.view_action(...)` id `create_puzzle5d_app` registers,
/// plus the framework-injected `SET_ACTIVE_UTILITY_ACTION_ID`). Each variant carries `window_id` (was
/// host-pushed `view_state.window_id`) plus `args` (the action's original `{...}` JSON payload,
/// unchanged) — `handle` reconstructs the exact `(action, args, window_id)` triple every
/// `🎮️commands/*` arm expects, so each arm's internal `args.get("field")` extraction stays
/// byte-for-byte identical to the pre-migration implementation.
///
/// ⚠️ `OpBinary` is a plain JSON-bytes bridge (NOT `#[derive(dsl::DslOps)]`, and NOT the framework's
/// `app_commands!` macro): a generic `args: Value` field is not representable in the DSL grammar those
/// target, so adopting them would silently rewrite this app's wire format. Keep this macro's variant
/// list, its order and its action-id literals byte-for-byte stable.
macro_rules! puzzle5d_command_variants {
    ($($Variant:ident = $id:tt),* $(,)?) => {
        #[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
        pub enum Puzzle5dCommand {
            $($Variant { window_id: Option<String>, args: Option<semio_framework_pack_json::Value> }),*
        }

        impl Puzzle5dCommand {
            /// 🏷️ The action id this variant was declared under — used both for `command_id()`
            /// (command-log labeling / registry kind-discipline) and to reconstruct the exact
            /// `action: &str` `handle_action_impl` dispatches on.
            fn action_id(&self) -> &'static str {
                match self {
                    $(Puzzle5dCommand::$Variant { .. } => $id),*
                }
            }

            fn window_id(&self) -> Option<&str> {
                match self {
                    $(Puzzle5dCommand::$Variant { window_id, .. } => window_id.as_deref()),*
                }
            }

            fn args(&self) -> Option<&semio_framework_pack_json::Value> {
                match self {
                    $(Puzzle5dCommand::$Variant { args, .. } => args.as_ref()),*
                }
            }

            fn try_from_action(action: &str, args: Option<semio_framework_pack_json::Value>, window_id: Option<String>) -> Option<Self> {
                match action {
                    $($id => Some(Puzzle5dCommand::$Variant { window_id, args })),*,
                    _ => None,
                }
            }

            #[cfg(test)]
            fn from_action(action: &str, args: Option<semio_framework_pack_json::Value>, window_id: Option<String>) -> Self {
                Self::try_from_action(action, args, window_id)
                    .unwrap_or_else(|| panic!("unknown puzzle5d action id in test: {action}"))
            }

            /// 🪶️ Hand-written JSON bridge (see this macro's own doc comment on why `OpBinary` here
            /// is a plain JSON-bytes bridge, not a derive): reproduces serde's default externally
            /// tagged struct-variant shape (`{"VariantName": {"window_id": ..., "args": ...}}`) so
            /// `encode_op`/`decode_op` stay byte-for-byte compatible with the pre-migration wire.
            fn to_json(&self) -> semio_framework_pack_json::Value {
                match self {
                    $(Puzzle5dCommand::$Variant { window_id, args } => semio_framework_pack_json::object([(
                        stringify!($Variant).to_string(),
                        semio_framework_pack_json::object([("window_id".to_string(), semio_framework_pack_json::Value::from(window_id.clone())), ("args".to_string(), args.clone().unwrap_or(semio_framework_pack_json::Value::Null))]),
                    )])),*
                }
            }

            fn from_json(value: &semio_framework_pack_json::Value) -> Option<Self> {
                let entries = value.as_object()?;
                if entries.len() != 1 {
                    return None;
                }
                let (tag, payload) = entries.iter().next()?;
                let window_id = payload.get("window_id").and_then(semio_framework_pack_json::Value::as_str).map(str::to_string);
                let args = payload.get("args").cloned().filter(|value| !value.is_null());
                match tag {
                    $(stringify!($Variant) => Some(Puzzle5dCommand::$Variant { window_id, args }),)*
                    _ => None,
                }
            }
        }
    };
}

puzzle5d_command_variants! {
    ExportSnapshot = "exportSnapshot",
    ImportSnapshot = "importSnapshot",
    OpenImportSnapshot = "openImportSnapshot",
    OpenAddPartDialog = "openAddPartDialog",
    SetActiveExample = "setActiveExample",
    AddNode = "addNode",
    AddPartKind = "addPartKind",
    AddBrushPart = "addBrushPart",
    DeleteSelection = "deleteSelection",
    DuplicateSelection = "duplicateSelection",
    SetSelectionFlag = "setSelectionFlag",
    SetSelectionHidden = "setSelectionHidden",
    SetSelectionLocked = "setSelectionLocked",
    FocusSelection = "focusSelection",
    EngagementSubmit = "engagementSubmit",
    EngagementRepeatLast = "engagementRepeatLast",
    SetFillCount = "setFillCount",
    PatchPart = "patchPart",
    PatchGrip = "patchGrip",
    PatchFastener = "patchFastener",
    CreateFastener = "createFastener",
    DeleteFastener = "deleteFastener",
    RetargetFastener = "retargetFastener",
    EditFastener = "editFastener",
    ProximityConnect = "proximityConnect",
    TranslateSelection = "translateSelection",
    RotateSelection = "rotateSelection",
    ScaleSelection = "scaleSelection",
    WorldRelocate = "worldRelocate",
    ApplyBoardEvents = "applyBoardEvents",
    SetCamera = "setCamera",
    SetCamera2d = "setCamera2d",
    SetCamera3d = "setCamera3d",
    SelectSameKindSelection = "selectSameKindSelection",
    ToggleSun = "toggleSun",
    SetSunAzimuth = "setSunAzimuth",
    SetSunElevation = "setSunElevation",
    SetSunIntensity = "setSunIntensity",
    EngagementInput = "engagementInput",
    EngagementAbort = "engagementAbort",
    EngagementControlSelect = "engagementControlSelect",
    CycleBrushCandidate = "cycleBrushCandidate",
    CycleBrushCandidateBack = "cycleBrushCandidateBack",
    TargetBrushSuggestions = "targetBrushSuggestions",
    OpenVortexSuggestions = "openVortexSuggestions",
    CloseVortexSuggestions = "closeVortexSuggestions",
    HoverSuggestion = "hoverSuggestion",
    AcceptSuggestion = "acceptSuggestion",
    RegisterBrushMesh = "registerBrushMesh",
    SetBrushPlacementContactTolerance = "setBrushPlacementContactTolerance",
    SetProximityRadius = "setProximityRadius",
    SetChunkSize = "setChunkSize",
    SetPartKindWeight = "setPartKindWeight",
    SetGripKindWeight = "setGripKindWeight",
    SetLodMode = "setLodMode",
    SetSuggestionOffset = "setSuggestionOffset",
    SetGridSnapEnabled = "setGridSnapEnabled",
    SetGridFactor = "setGridFactor",
    SetGridVisible = "setGridVisible",
    SetGridSpacing = "setGridSpacing",
    SetProjection = "setProjection",
    SetProjectionParam = "setProjectionParam",
    SetGripShow = "setGripShow",
    SetGripDirection = "setGripDirection",
    SetSelectableKind = "setSelectableKind",
    SetLodAutomatic = "setLodAutomatic",
    SetLodDepthVariable = "setLodDepthVariable",
    SetLodManual = "setLodManual",
    SetTransformGumballFlag = "setTransformGumballFlag",
    WorldPointerDown = "worldPointerDown",
    CanvasPointerDown = "canvasPointerDown",
    AddTargetVolume = "addTargetVolume",
    DeleteTargetVolume = "deleteTargetVolume",
    RelocateTargetVolume = "relocateTargetVolume",
    SetTargetVolumeFlag = "setTargetVolumeFlag",
    SetTargetVolumeHidden = "setTargetVolumeHidden",
    SetTargetVolumeLocked = "setTargetVolumeLocked",
    SetVoxelDims = "setVoxelDims",
}




//#endregion 🔖️Puzzle5dCommand

//#region 🔖️ActionContext














//#endregion 🔖️ActionContext

//#region 🔖️PlayApp
// 🧩️ B1: Puzzle-5d play app. Stateless: the persisted document (bare `Puzzle5dDocument` json) lives in
// the wrapping `VcsArtifactApp`'s document store, the ephemeral view state in the wrapping store's real,
// VCS-tracked `Puzzle5dConfig` artifact (see `🦀️config.rs`) — every read comes from `cfg.snapshot`, every write
// flows out as a `Puzzle5dConfigMutation` in the returned `Emit` — and what an instance retains across turns
// (the brush suggestions link) in its instance operation owner.
// Each action mutates a transient {@link Puzzle5dScene}, then emits the granular operation delta.
// Undo/redo/checkpoints are handled by the wrapper.
fn with_puzzle5d_app<R>(f: impl FnOnce(&Puzzle5dPlayApp) -> R) -> R {
    let app = Puzzle5dPlayApp;
    f(&app)
}

#[derive(Default)]
pub struct Puzzle5dPlayApp;

impl Puzzle5dPlayApp {
    /// 🧩️ B1: the pure per-action core, dispatched into by `ArtifactApp::handle` with
    /// `action`/`args`/`window_id` reconstructed 1:1 from the typed `Puzzle5dCommand`. Everything past
    /// this adapter boundary reads/writes the passed-in `Puzzle5dConfig` snapshot and returns a real
    /// `Emit` (document + config operations) instead of mutating `self`.
    fn handle_action_impl(
        &self,
        authoring_seed: &str,
        action: &str,
        args: Option<&Value>,
        window_id: Option<&str>,
        snapshot: &Puzzle5dPlaySnapshot,
        config: &Puzzle5dRuntime,
        view_state: Option<&semio_framework_plugin::ViewModel>,
        selection: &protocol::DomainSelection,
        instance_owner: Option<&semio_framework_plugin::ArtifactInstanceOperationOwnerHandle>,
        tool_run: Option<&semio_framework_plugin::ToolRunView>,
    ) -> (Emit<Puzzle5dMutation, Puzzle5dConfigMutation>, EphemeralEmit<EditorApp<Puzzle5dPlayApp>>) {
        let projection = puzzle5d_editor_projection(snapshot);
        let shared_before = window_ownership::shared(config);
        let window_before = window_ownership::config_from_runtime(config);
        let transient_before = window_ownership::transient_from_runtime(config, window_id.unwrap_or(world3d::WINDOW_KIND_ID));
        let active_utility_initial = puzzle5d_scene_active_utility(view_state, window_id);
        let wid = window_id.map_or_else(|| world3d::WINDOW_KIND_ID.to_string(), str::to_string);
        let window_kind = view_state.and_then(window_ownership::kind_for_view).unwrap_or(world3d::WINDOW_KIND_ID);
        let mut scene = scene_from_projection(&projection, config.clone(), &active_utility_initial);
        let mut ctx = Puzzle5dActionCtx {
            scene: &mut scene,
            snapshot,
            instance_owner,
            window_id: &wid,
            window_kind,
            selection,
            view_state,
            tool_run,
            effects: Vec::new(),
            interaction_writes: Vec::new(),
            abort: false,
            authoring_seed,
            artifact_mutations: Vec::new(),
            transaction: None,
        };
        dispatch_puzzle5d_action(&mut ctx, action, args);
        let aborted = ctx.abort;
        let mut arm_effects = std::mem::take(&mut ctx.effects);
        let interaction_writes = std::mem::take(&mut ctx.interaction_writes);
        let tool_mutations = std::mem::take(&mut ctx.artifact_mutations);
        let transaction = ctx.transaction.take();
        if aborted {
            // 🧯️ An aborted arm emits no document/config delta — but the refusal NOTICE it pushed is the
            // user-visible half of that refusal and must survive, or "refused" and "silently did nothing"
            // look identical. Effects travel on their own lane, so keeping them costs no mutation and no
            // undo entry; the scope stays `None` because nothing was painted.
            return (Emit { effects: arm_effects, ui_scope: UiDirtyScope::None, ..Default::default() }, EphemeralEmit::default());
        }
        let next_active_utility = scene.active_utility.clone();
        let operations: Vec<Puzzle5dMutation> = tool_mutations;
        // 🛠️ A committed transform-tool transaction stamps every op of this ONE edit.
        let transaction = transaction.filter(|_| !operations.is_empty());
        // 🧰️🛠️ Programmatic tool/utility switches push the host session. Arming fill emits `SetActiveTool { fill }`
        // only — an empty tool effect here would bounce-disarm a just-armed run, so LEAVING fill is exclusively the
        // host's own `setActiveTool ""` (which `engagementAbort` raises itself). A real utility change still emits
        // `SetActiveUtility`; the host's mutual exclusion clears the tool when the utility is non-empty.
        let initial_is_fill_tool = active_utility_initial == fill_tool::TOOL_ID;
        let next_is_fill_tool = next_active_utility == fill_tool::TOOL_ID;
        if next_is_fill_tool && !initial_is_fill_tool {
            arm_effects.push(Effect::SetActiveTool { tool_id: fill_tool::TOOL_ID.into() });
        }
        if !next_is_fill_tool && next_active_utility != active_utility_initial {
            arm_effects.push(Effect::SetActiveUtility { window_id: wid.clone(), utility_id: next_active_utility.clone() });
        }
        let effects = arm_effects;
        let shared_after = window_ownership::shared(&scene.runtime);
        let config_mutations = shared_before.mutations_to(&shared_after);
        let window_after = window_ownership::config_from_runtime(&scene.runtime);
        let window_config_mutations = if window_after != window_before {
            view_state.and_then(|view| window_ownership::addressed_config(view, window_after).ok()).into_iter().collect()
        } else {
            Vec::new()
        };
        let transient_after = window_ownership::transient_from_runtime(&scene.runtime, window_id.unwrap_or(world3d::WINDOW_KIND_ID));
        let window_transient = if transient_after != transient_before {
            view_state.and_then(|view| window_ownership::addressed_transient(view, transient_after).ok()).into_iter().collect()
        } else {
            Vec::new()
        };
        (Emit { artifact_mutations: operations, config_mutations, window_config_mutations, transaction, effects, interaction_writes, ..Default::default() }, EphemeralEmit { window_transient, ..Default::default() })
    }
}

impl Puzzle5dPlayApp {
    /// 🖱️ The ONE context-menu implementation — `ArtifactEditor::context_menu` and
    /// `context_menu_with_request_context` funnel here, differing only in whether `interaction` carries a live
    /// `vortex`-domain read or the empty default.
    fn context_menu_body(
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Puzzle5dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle5dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        interaction: &Puzzle5dInteractionSnapshot,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        let projection = puzzle5d_editor_projection(&doc.snapshot);
        let Some(labels) = puzzle5d_labels(view_state) else { return Vec::new() };
        let window_id = view_state.window_id.as_deref().unwrap_or(world3d::WINDOW_KIND_ID);
        let active_utility = puzzle5d_scene_active_utility(Some(view_state), Some(window_id));
        let runtime = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), &window_ownership::Puzzle5dWindowTransient::default(), window_id);
        let envelope = scene_from_projection(&projection, runtime, &active_utility);
        let mut selection = Puzzle5dContextSelection::from_surface(request.surface.as_ref());
        selection.fill_from_interaction(interaction);
        puzzle5d_context_menu_items(&envelope, &selection, labels, view_state, registry)
    }
}

/// 🎬️ Dispatch only: every arm's behaviour lives in its `🎮️commands/<group>/🦀️.rs` free
/// function. No behaviour lives in this match.
fn dispatch_puzzle5d_action(ctx: &mut Puzzle5dActionCtx<'_>, action: &str, args: Option<&Value>) {
    match action {
        "importSnapshot" => import_snapshot::import_snapshot(ctx, args),
        "openImportSnapshot" => open_import_snapshot::open_import_snapshot(ctx),
        "openAddPartDialog" => open_add_part_dialog::open_add_part_dialog(ctx),
        "selectSameKindSelection" => select_same_kind::select_same_kind(ctx),
        "deleteSelection" => delete_selection::delete_selection(ctx),
        "duplicateSelection" => duplicate_selection::duplicate_selection(ctx),
        "setSelectionFlag" => set_selection_flag::set_selection_flag(ctx, args),
        "setSelectionHidden" => set_selection_flag::set_selection_flag_value(ctx, args, "hidden"),
        "setSelectionLocked" => set_selection_flag::set_selection_flag_value(ctx, args, "locked"),
        "setCamera" => set_camera::set_camera(ctx, args),
        "setCamera2d" => set_camera_2d::set_camera_2d(ctx, args),
        "setCamera3d" => set_camera_3d::set_camera_3d(ctx, args),
        "focusSelection" => focus_selection::focus_selection(ctx),
        "toggleSun" | "setSunAzimuth" | "setSunElevation" | "setSunIntensity" => apply_sun::apply(ctx, action, args),
        "setLodMode" => set_lod_mode::set_lod_mode(ctx, args),
        "setGridSnapEnabled" => set_grid_snap_enabled::set_grid_snap_enabled(ctx, args),
        "setGridFactor" => set_grid_factor::set_grid_factor(ctx, args),
        "setGridVisible" => set_grid_visible::set_grid_visible(ctx, args),
        "setGridSpacing" => set_grid_spacing::set_grid_spacing(ctx, args),
        "setProjection" | "setProjectionParam" => set_projection::set_projection(ctx, action, args),
        "setGripShow" => set_grip_show::set_grip_show(ctx, args),
        "setGripDirection" => set_grip_direction::set_grip_direction(ctx, args),
        "setSelectableKind" => set_selectable_kind::set_selectable_kind(ctx, args),
        "setLodAutomatic" => set_lod_automatic::set_lod_automatic(ctx, args),
        "setLodDepthVariable" => set_lod_depth_variable::set_lod_depth_variable(ctx, args),
        "setLodManual" => set_lod_manual::set_lod_manual(ctx, args),
        "setTransformGumballFlag" => set_transform_gumball_flag::set_transform_gumball_flag(ctx, args),
        "cycleBrushCandidate" => cycle_brush_candidate::cycle_brush_candidate(ctx),
        "cycleBrushCandidateBack" => cycle_brush_candidate::cycle_brush_candidate_back(ctx),
        "targetBrushSuggestions" => target_brush_suggestions::target_brush_suggestions(ctx, args),
        "openVortexSuggestions" => open_vortex_suggestions::open_vortex_suggestions(ctx, args),
        "closeVortexSuggestions" => close_vortex_suggestions::close_vortex_suggestions(ctx, args),
        "hoverSuggestion" => hover_suggestion::hover_suggestion(ctx, args),
        "acceptSuggestion" => accept_suggestion::accept_suggestion(ctx, args),
        "registerBrushMesh" => register_brush_mesh::register_brush_mesh(ctx, args),
        "setBrushPlacementContactTolerance" => set_brush_placement_contact_tolerance::set_brush_placement_contact_tolerance(ctx, args),
        "setProximityRadius" => set_proximity_radius::set_proximity_radius(ctx, args),
        "setChunkSize" => set_chunk_size::set_chunk_size(ctx, args),
        "setPartKindWeight" | "setGripKindWeight" => set_kind_weight::set_kind_weight(ctx, action, args),
        "engagementControlSelect" => engagement_control_select::engagement_control_select(ctx, args),
        "setSuggestionOffset" => set_suggestion_offset::set_suggestion_offset(ctx, args),
        "setFillCount" => set_fill_count::set_fill_count(ctx, args),
        "engagementInput" => engagement_input::engagement_input(ctx, args),
        "engagementSubmit" => engagement_submit::engagement_submit(ctx, args),
        "engagementRepeatLast" => engagement_repeat_last::engagement_repeat_last(ctx),
        "engagementAbort" => engagement_abort::engagement_abort(ctx, args),
        "translateSelection" => translate_selection::translate_selection(ctx, args),
        "rotateSelection" => rotate_selection::rotate_selection(ctx, args),
        "scaleSelection" => scale_selection::scale_selection(ctx, args),
        "worldRelocate" => world_relocate::world_relocate(ctx, args),
        "addTargetVolume" => add_target_volume::add_target_volume(ctx, args),
        "deleteTargetVolume" => delete_target_volume::delete_target_volume(ctx, args),
        "relocateTargetVolume" => relocate_target_volume::relocate_target_volume(ctx, args),
        "setTargetVolumeFlag" => set_target_volume_flag::set_target_volume_flag(ctx, args),
        "setTargetVolumeHidden" => set_target_volume_flag::set_target_volume_flag_value(ctx, args, "hidden"),
        "setTargetVolumeLocked" => set_target_volume_flag::set_target_volume_flag_value(ctx, args, "locked"),
        "setVoxelDims" => set_voxel_dims::set_voxel_dims(ctx, args),
        // 🛑️ Pure pointer-down notifications: no scene mutation, no operations, no config snapshot —
        // the pre-migration code returned `Emit::default()` here, which `abort` reproduces exactly.
        "worldPointerDown" | "canvasPointerDown" => ctx.abort = true,
        _ => {}
    }
}

//#region 🧵️RetainedCommands
pub(crate) const PUZZLE5D_RETAINED_TOOL_IDS: &[&str] = &[
    "addTargetVolume",
    "deleteTargetVolume",
    "relocateTargetVolume",
    "setTargetVolumeFlag",
    "setTargetVolumeHidden",
    "setTargetVolumeLocked",
    "setVoxelDims",
    "canvasPointerDown",
    "cycleBrushCandidate",
    "cycleBrushCandidateBack",
    "worldPointerDown",
    "deleteSelection",
    "duplicateSelection",
    "engagementAbort",
    "engagementControlSelect",
    "engagementInput",
    "engagementRepeatLast",
    "engagementSubmit",
    "exportSnapshot",
    "importSnapshot",
    "openAddPartDialog",
    "openImportSnapshot",
    "selectSameKindSelection",
    "setFillCount",
    "setSelectionFlag",
    "setSelectionHidden",
    "setSelectionLocked",
    "targetBrushSuggestions",
    "openVortexSuggestions",
    "closeVortexSuggestions",
    "hoverSuggestion",
    "acceptSuggestion",
    "focusSelection",
    "addBrushPart",
    "addNode",
    "addPartKind",
    "applyBoardEvents",
    "createFastener",
    "deleteFastener",
    "editFastener",
    "patchFastener",
    "patchGrip",
    "patchPart",
    "proximityConnect",
    "registerBrushMesh",
    "retargetFastener",
    "rotateSelection",
    "scaleSelection",
    "setActiveExample",
    "translateSelection",
    "worldRelocate",
    "setCamera",
    "setCamera2d",
    "setCamera3d",
    "setGridFactor",
    "setGridSnapEnabled",
    "setLodMode",
    "setSuggestionOffset",
    "toggleSun",
    "setSunAzimuth",
    "setSunElevation",
    "setSunIntensity",
    "setBrushPlacementContactTolerance",
    "setProximityRadius",
    "setChunkSize",
    "setPartKindWeight",
    "setGripKindWeight",
    "setGridVisible",
    "setGridSpacing",
    "setProjection",
    "setProjectionParam",
    "setGripShow",
    "setGripDirection",
    "setSelectableKind",
    "setLodAutomatic",
    "setLodDepthVariable",
    "setLodManual",
    "setTransformGumballFlag",
];













































































































impl Puzzle5dProximityConnectWork {
    fn argument<'a>(command: &'a Puzzle5dCommand, key: &str) -> Option<&'a str> {
        command.args().and_then(|args| args.get(key)).and_then(Value::as_str).filter(|value| !value.is_empty())
    }

    fn grip_kind(grip: &Value) -> Option<String> {
        grip.get("gripKind").and_then(Value::as_str).filter(|kind| !kind.is_empty()).or_else(|| grip.get("2d").and_then(|value| value.get("gripKind")).and_then(Value::as_str).filter(|kind| !kind.is_empty())).map(str::to_string)
    }

    fn world_position(part: &Value, grip: &Value) -> [f64; 3] {
        let origin = part.get("3d").and_then(|part| part.get("origin")).and_then(puzzle5d_value_as_f64_3).unwrap_or_default();
        let orientation = part.get("3d").and_then(|part| part.get("orientation")).and_then(puzzle5d_value_as_f64_4).unwrap_or([0.0, 0.0, 0.0, 1.0]);
        let position = grip.get("3d").and_then(|grip| grip.get("position")).and_then(puzzle5d_value_as_f64_3).unwrap_or_default();
        let rotated = quat_rotate_vector(orientation, position);
        [origin[0] + rotated[0], origin[1] + rotated[1], origin[2] + rotated[2]]
    }

    fn clear_candidate(&mut self) {
        self.candidate_id = None;
        self.candidate_kind = None;
        self.fastener_cursor = 0;
        self.compatibility_cursor = 0;
        self.stage = Puzzle5dProximityConnectStage::Candidate;
    }

    fn complete(&mut self) -> semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle5dPlayApp>> {
        self.stage = Puzzle5dProximityConnectStage::Complete;
        semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), ui_scope: UiDirtyScope::Full, ..Default::default() })
    }
}

impl Puzzle5dProximityConnectWork {
    /// 🪪️ Binds the operation nonce its minted ids derive from.
    fn bound(mut self, operation_nonce: u64) -> Self {
        self.operation_nonce = operation_nonce;
        self
    }
}





























impl Puzzle5dAddNodeWork {
    fn part_kind(command: &Puzzle5dCommand) -> &str {
        command.args().and_then(|args| args.get("kind")).and_then(Value::as_str).unwrap_or("Part")
    }

    fn catalogs(snapshot: &Puzzle5dPlaySnapshot) -> Vec<Value> {
        let projection = puzzle5d_editor_projection(snapshot);
        projection.get("kindCatalogs").and_then(|catalogs| catalogs.get("parts")).and_then(Value::as_array).cloned().unwrap_or_default()
    }
}

impl Puzzle5dAddNodeWork {
    /// 🪪️ Binds the operation nonce its minted ids derive from.
    fn bound(mut self, operation_nonce: u64) -> Self {
        self.operation_nonce = operation_nonce;
        self
    }
}









impl Puzzle5dAddBrushPartWork {
    fn is_closing(&self) -> bool {
        self.stage == Puzzle5dAddBrushPartStage::Closing
    }

    fn new(tool_id: &'static str) -> Self {
        Self { close_owners: Default::default(),
            tool_id,
            payload: None,
            stage: Puzzle5dAddBrushPartStage::Catalog,
            catalog_cursor: 0,
            grip_cursor: 0,
            part_cursor: 0,
            target_grip_cursor: 0,
            processed_units: 0,
            catalog_index: None,
            mesh_url: None,
            target_id: None,
            target_position: None,
            target_direction: None,
            created_id: None,
            created_grip_id: None,
            grips: Vec::with_capacity(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS),
            mutations: Vec::with_capacity(2),
            operation_nonce: 0,
            fresh_cursor: 0,
        }
    }

    fn from_board_payload(payload: Value) -> Self {
        let mut work = Self::new("addBrushPart");
        work.payload = Some(payload);
        work
    }

    /// 🌉️ `self.payload` (materialized once via `take_payload`/`parse`) and `command.args()` are
    /// both this file's own first-party `Value` now, so no bridge is needed — an in-progress board
    /// payload wins, falling back to the dispatch args only when there is none.
    fn args(&self, command: &Puzzle5dCommand) -> Option<Value> {
        self.payload.clone().or_else(|| command.args().cloned())
    }

    /// 🗂️ The kind this run adds: the caller's own `partKind`, else THIS tool's own declared select
    /// default — never a literal.
    ///
    /// 🐛️ It used to fall back to the string `"Part"`, the very literal
    /// `PUZZLE5D_SHIPPED_PART_KINDS` replaced in the select, so a bare `addPartKind` (the arg form
    /// dispatched with no args, which is what an agent sends) added a part of a kind no catalog
    /// declares. Measured 2026-09-22 (slice PZ2): `add_part_kind_materializes_the_declared_kind_default`
    /// read back `"Part"` where the declared default is `"Hexagonal Cut Concrete Forest Left"`.
    /// `addBrushPart` declares a one-row `"Part"` select of its own, so reading each tool's OWN
    /// declared default keeps that verb byte-identical while fixing this one.
    fn owned_part_kind(&self, command: &Puzzle5dCommand) -> String {
        self.args(command)
            .and_then(|args| args.get("partKind").or_else(|| args.get("objectKindId")).or_else(|| args.get("nodeKind")).and_then(Value::as_str).map(str::to_string))
            .filter(|kind| !kind.is_empty())
            .unwrap_or_else(|| if self.tool_id == "addPartKind" { puzzle5d_default_part_kind(&puzzle5d_part_kind_options()) } else { "Part".to_string() })
    }

    fn catalogs(snapshot: &Puzzle5dPlaySnapshot) -> Vec<Value> {
        Puzzle5dAddNodeWork::catalogs(snapshot)
    }

    fn target(&self, command: &Puzzle5dCommand, interaction: &protocol::InteractionState) -> Option<String> {
        self.args(command)
            .and_then(|args| args.get("targetVortexFullId").or_else(|| args.get("targetGripFullId")).and_then(Value::as_str).map(str::to_string))
            .filter(|id| !id.is_empty())
            .or_else(|| interaction.selection.get(PUZZLE5D_INTERACTION_DOMAIN).filter(|selection| selection.granularity == PUZZLE5D_GRANULARITY_GRIP).and_then(|selection| selection.ids.first().cloned()))
    }

    fn world_direction(part: &Value, grip: &Value) -> [f64; 3] {
        let orientation = part.get("3d").and_then(|part| part.get("orientation")).and_then(puzzle5d_value_as_f64_4).unwrap_or([0.0, 0.0, 0.0, 1.0]);
        let direction = grip.get("3d").and_then(|grip| grip.get("direction")).and_then(puzzle5d_value_as_f64_3).unwrap_or([0.0, 0.0, -1.0]);
        quat_rotate_vector(orientation, direction)
    }
}

impl Puzzle5dAddBrushPartWork {
    /// 🪪️ Binds the operation nonce its minted ids derive from.
    fn bound(mut self, operation_nonce: u64) -> Self {
        self.operation_nonce = operation_nonce;
        self
    }
}



























impl Puzzle5dCreateFastenerWork {
    fn endpoint<'a>(command: &'a Puzzle5dCommand, primary: &str, alias: &str) -> &'a str {
        command.args().and_then(|args| args.get(primary).or_else(|| args.get(alias))).and_then(Value::as_str).filter(|id| !id.is_empty()).unwrap_or("")
    }

    fn scan_grip(&mut self, snapshot: &Puzzle5dPlaySnapshot, target: &str) -> Puzzle5dGripScan {
        let projection = puzzle5d_editor_projection(snapshot);
        let Some(part) = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
            return Puzzle5dGripScan::Exhausted;
        };
        let Some(grip) = part.get("grips").and_then(Value::as_array).and_then(|grips| grips.get(self.grip_cursor)) else {
            self.part_cursor += 1;
            self.grip_cursor = 0;
            return Puzzle5dGripScan::Progress;
        };
        self.grip_cursor += 1;
        let Some(part_id) = part.get("id").and_then(Value::as_str) else { return Puzzle5dGripScan::Progress };
        let Some(grip_id) = grip.get("id").and_then(Value::as_str) else { return Puzzle5dGripScan::Progress };
        if puzzle5d_grip_full_id(part_id, grip_id) != target {
            return Puzzle5dGripScan::Progress;
        }
        let kind = grip.get("gripKind").and_then(Value::as_str).filter(|kind| !kind.is_empty()).or_else(|| grip.get("2d").and_then(|value| value.get("gripKind")).and_then(Value::as_str).filter(|kind| !kind.is_empty())).map(str::to_string);
        Puzzle5dGripScan::Found(kind)
    }

    fn complete_empty(&mut self) -> semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle5dPlayApp>> {
        self.stage = Puzzle5dCreateFastenerStage::Complete;
        semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit::default())
    }

    fn arg_f64(command: &Puzzle5dCommand, key: &str) -> f64 {
        command.args().and_then(|args| args.get(key)).and_then(Value::as_f64).unwrap_or(0.0)
    }
}

impl Puzzle5dCreateFastenerWork {
    /// 🪪️ Binds the operation nonce its minted ids derive from.
    fn bound(mut self, operation_nonce: u64) -> Self {
        self.operation_nonce = operation_nonce;
        self
    }
}







pub use semio_s_artifact_puzzle_5d_retained::editor::puzzle5d::retained;
use retained::*;

pub use semio_s_artifact_puzzle_5d_mut_any_6::editor::puzzle5d::gestures;
use gestures::*;

pub use semio_s_artifact_puzzle_5d_retained::editor::puzzle5d::kit;
use kit::*;

pub use semio_s_artifact_puzzle_5d_dispatch::editor::puzzle5d::dispatch;
use dispatch::puzzle5d_handle_action;

use crate::editor::puzzle5d::modes::edit::windows::world3d::utilities::transform::puzzle5d_inspector_nudge;

#[cfg(any(test, feature = "testkit"))]
#[path = "🧪️tests/🧰️testkit/🦀️.rs"]
pub mod testkit;
#[cfg(any(test, feature = "testkit"))]
pub use testkit::puzzle5d_app_manifest_for_tests;

#[path = "🔧️play-app-retained-commands-and-tool-proofs/🦀️.rs"]
mod play_app_retained_commands_and_tool_proofs;
pub use play_app_retained_commands_and_tool_proofs::{
    Puzzle5dPlayApp, Puzzle5dRetainedCommandJobFactory,
};
use play_app_retained_commands_and_tool_proofs::{
    Puzzle5dHostConfigurationProofs, Puzzle5dRetainedCommandProofs,
};

#[path = "✍️artifact-editor/🦀️.rs"]
mod artifact_editor;

#[path = "🚀️app-assembly/🦀️.rs"]
mod app_assembly;
use app_assembly::{puzzle5d_fault_notices, puzzle5d_flag_value_argument};
pub use app_assembly::create_puzzle5d_app;

//#region 🔖️Manifest

// 🗂️ `Puzzle5dPlaySnapshot`'s pack<->dsl codec (so `framework/sync`'s `FolderEndpoint::Pack` can
// print/parse puzzle-5d play documents without depending on this crate's concrete
// `Projection`/`Mutation` types) is now declared via `.document_codec::<Puzzle5dPlayApp>()` on
// `crate::declaration()` (ticket `26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE`
// M1) — the old side-effecting `register_puzzle5d_exports()` wrapper (this app file's only caller of
// `register_document_codec_for_app`) is gone. The 5d mesh export/import OS-host registration
// (`register_mesh_io()`/`puzzle5d_document_from_mesh`) was never rewired to a real `.setup()` caller
// after the artifacts-only-plugin-architecture migration (`🧩️puzzle/🦀️.rs`'s `plugin()`
// builder chain has no `.setup()` call at all) and was deleted as dead code (ticket
// 26/08/17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS) — same fate as puzzle3d's
// sibling mesh bridge. Mesh export/import should be re-derived from `io_dispatch`'s real
// `ComposerEntry` chain if/when this bridge is needed again.
//#endregion 🔖️Manifest

//#region 🧪️UnitTests

//#endregion 🧪️UnitTests

//#region 🧩️ArtifactItems4
pub mod artifact_items_4 {
use crate::{PUZZLE_5D_SCHEMA};

//#region 🔖️ArtifactKind
/// 🗿️ The `5d.puzzle` artifact kind — lifted out of the pre-consolidation manifest builder chain so
/// the artifact, not the app, owns its own identity.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    semio_framework_plugin::ArtifactKindSpec {
        id: "5d.puzzle".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("5D Puzzle", "5D-Puzzle"),
        source_format: "puzzle.5d".into(),
        component_kind: "puzzle5d".into(),
        dimension: "5d".into(),
        media_capability: semio_framework_plugin::OsMediaCapability::MeshOnly,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Kit, form: semio_framework_plugin::MediaForm::Design },
        schema: PUZZLE_5D_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.json".into(), "stdio.png".into(), "stdio.zip".into()],
        import_stdio_kinds: vec!["stdio.json".into(), "stdio.zip".into()],
    }
}
//#endregion 🔖️ArtifactKind
}
//#endregion 🧩️ArtifactItems4
