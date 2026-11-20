//! 📚️ Example and mesh work of the puzzle 5d editor kit: the retained set-active-example and register-brush-mesh work. See [`super`].

use crate::standards::v1::subsets::any::schema::play_snapshot::Puzzle5dPlaySnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use crate::editor::puzzle5d::commands::register_brush_mesh;
use crate::editor::puzzle5d::config::Puzzle5dConfig;
use semio_plugin::kernel::UiDirtyScope;
use semio_plugin::EditorApp;
use semio_plugin::Emit;
use semio_plugin::Fault;
use semio_plugin::InteractionTarget;
use semio_plugin::InteractionWrite;
use semio_ui_locale::Label;
use semio_plugin::MergeMode;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_pack_json::Value;
use crate::editor::puzzle5d::interaction::Puzzle5dEditor;
use crate::editor::puzzle5d::entities::{CAPSULE_DREAM_EXAMPLE_DOCUMENT, CONCRETE_FOREST_EXAMPLE_DOCUMENT, EMPTY_EXAMPLE_DOCUMENT, NAKAGIN_EXAMPLE_DOCUMENT, PUZZLE5D_EXAMPLE_CAPSULE_DREAM, PUZZLE5D_EXAMPLE_CONCRETE_FOREST, PUZZLE5D_EXAMPLE_NAKAGIN, PUZZLE5D_INTERACTION_DOMAIN, Puzzle5dDocument, Puzzle5dPart, puzzle5d_editor_projection};
use semio_framework_job::StepContext;
use semio_os_kernel::os_store::one_item_publication::ArtifactMutationPages;
use crate::editor::puzzle5d::interaction::{Puzzle5dCommand, puzzle5d_notice_emit};

//#region 🔖️PlayApp
//#region 🧵️RetainedCommands
/// 📦️ Rows one `setActiveExample` step replays. A declared unit is a CHUNK, not a row:
/// `🌙️capsule-dream` is 2 880 parts + 2 865 fasteners, so a row-per-unit extent (≈5 749, and ≈11 500 when
/// reloading capsule-dream over itself) overruns the shared `PUZZLE_COMMAND_WORK_ITEMS` band and the
/// switch could only ever refuse. Chunking makes the same replay fit without widening a constant three
/// fixtures pin. It is 8× puzzle 3d's `PUZZLE3D_SET_ACTIVE_EXAMPLE_CHUNK` on purpose: 3d steps over a
/// TYPED snapshot, while every 5d work re-derives `puzzle5d_projection_value` per step, so a step's cost
/// is dominated by that one O(document) projection rather than by the rows it then replays — fewer,
/// fatter steps is strictly cheaper here, and capsule-dream's switch drops from ≈720 steps to ≈100.
const PUZZLE5D_SET_ACTIVE_EXAMPLE_CHUNK: usize = 64;

/// 🔢️ Stage transitions and whole-document rows a switch always pays on top of its chunks (label, domain,
/// description, catalogs and one exhaustion step per cursored stage). 3d's own count, kept identical.
const PUZZLE5D_SET_ACTIVE_EXAMPLE_FIXED_STEPS: usize = 13;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Puzzle5dSetActiveExampleStage {
    ClearFasteners,
    ClearParts,
    Label,
    Domain,
    Description,
    ClearCompatibility,
    AddCompatibility,
    Catalogs,
    AddParts,
    AddFasteners,
    Complete,
    Closing,
}

/// ♻️ The owners one `Puzzle5dSetActiveExampleWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dSetActiveExampleWorkOwners {
    pub mutations: Vec<Puzzle5dMutation>,
    pub mutation_pages: Option<ArtifactMutationPages<Puzzle5dMutation>>,
    pub before: Option<semio_framework_value::retirement::arc_custody::ArcLease<Puzzle5dSetActiveExampleBefore>>,
}

pub struct Puzzle5dSetActiveExampleWork {
    pub stage: Puzzle5dSetActiveExampleStage,
    pub cursor: usize,
    pub admitted: bool,
    pub mutations: Vec<Puzzle5dMutation>,
    mutation_pages: Option<ArtifactMutationPages<Puzzle5dMutation>>,
    /// 🗂️ The BEFORE document's ids to clear — fastener ids, part ids and compatibility pairs —
    /// harvested ONCE and then merely indexed by the cursored clearing stages.
    ///
    /// 🐛️ `step` used to call `puzzle5d_projection_value` over the run's snapshot on entry, i.e. re-derive the
    /// whole document (`Value` → `DslValue` → os-pack `Value`) on each of the ~110 chunk
    /// steps one example switch takes, and then re-walk its arrays. The run's snapshot is an `Arc`
    /// the retained driver holds FIXED for the whole run (nothing publishes before `Complete`), so
    /// every one of those derivations produced the same bytes — and leaving `🌙️capsule-dream`
    /// (2 880 parts, ~3.5 MB of JSON) paid that whole projection ~110 times for one switch.
    pub before: Option<std::sync::Arc<Puzzle5dSetActiveExampleBefore>>,
    pub close_owners: crate::puzzle_job::WorkClosing<Puzzle5dSetActiveExampleWorkOwners>,
}

/// 🗂️ Everything the clearing stages need from the BEFORE document, harvested in one pass.
#[derive(Default, semio_framework_value::RetireOwned)]
struct Puzzle5dSetActiveExampleBefore {
    pub fastener_ids: Vec<String>,
    pub part_ids: Vec<String>,
    pub compatibility: Vec<(String, String)>,
}

impl Default for Puzzle5dSetActiveExampleWork {
    fn default() -> Self {
        Self { close_owners: Default::default(), stage: Puzzle5dSetActiveExampleStage::ClearFasteners, cursor: 0, admitted: false, mutations: Vec::new(), mutation_pages: None, before: None }
    }
}

impl Puzzle5dSetActiveExampleWork {
    /// 📏️ How many semantic units switching to `target` from `snapshot` costs — every old fastener and
    /// part cleared, every old compatibility row dropped, then the target's own compatibility rows, parts
    /// and fasteners, each counted in `PUZZLE5D_SET_ACTIVE_EXAMPLE_CHUNK`-sized steps, plus the fixed
    /// stage-transition and whole-document rows.
    pub fn units(command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot) -> Option<usize> {
        Self::row_counts(command, snapshot).and_then(Self::work_units)
    }

    fn row_counts(command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot) -> Option<[usize; 6]> {
        let target = Self::target(command)?;
        let current = snapshot.typed();
        Some([
            current.fasteners.len(),
            current.parts.len(),
            current.kind_compatibility.len(),
            Self::compatibility_rows(target).len(),
            target.parts.len(),
            target.fasteners.len(),
        ])
    }

    fn work_units(rows: [usize; 6]) -> Option<usize> {
        let capacity = rows.into_iter().try_fold(4usize, usize::checked_add)?;
        let pages = if ArtifactMutationPages::<Puzzle5dMutation>::needs_pages(capacity) {
            ArtifactMutationPages::<Puzzle5dMutation>::page_count(capacity)
        } else {
            0
        };
        rows.into_iter().try_fold(PUZZLE5D_SET_ACTIVE_EXAMPLE_FIXED_STEPS.checked_add(pages)?, |units, len| units.checked_add(len.div_ceil(PUZZLE5D_SET_ACTIVE_EXAMPLE_CHUNK)))
    }

    /// 🧮️ Counts the original mutation entries this example switch will emit.
    pub fn mutation_capacity(command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot) -> usize {
        let Some(rows) = Self::row_counts(command, snapshot) else { return 0 };
        if Self::work_units(rows).is_none_or(|units| units > crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS) {
            return 0;
        }
        4 + rows.into_iter().sum::<usize>()
    }

    /// ✂️ The next `PUZZLE5D_SET_ACTIVE_EXAMPLE_CHUNK` rows of a cursored stage, advancing the cursor.
    fn take_chunk(cursor: &mut usize, len: usize) -> std::ops::Range<usize> {
        if *cursor >= len {
            return *cursor..*cursor;
        }
        let end = cursor.saturating_add(PUZZLE5D_SET_ACTIVE_EXAMPLE_CHUNK).min(len);
        let range = *cursor..end;
        *cursor = end;
        range
    }

    pub fn target(command: &Puzzle5dCommand) -> Option<&'static Puzzle5dDocument> {
        let example_id = command.args().and_then(|args| args.get("exampleId")).and_then(Value::as_str).unwrap_or("");
        match example_id {
            "" => Some(&EMPTY_EXAMPLE_DOCUMENT),
            PUZZLE5D_EXAMPLE_CONCRETE_FOREST | "concrete" => Some(&CONCRETE_FOREST_EXAMPLE_DOCUMENT),
            PUZZLE5D_EXAMPLE_NAKAGIN | "nakagin" => Some(&NAKAGIN_EXAMPLE_DOCUMENT),
            PUZZLE5D_EXAMPLE_CAPSULE_DREAM | "capsule" => Some(&CAPSULE_DREAM_EXAMPLE_DOCUMENT),
            _ => None,
        }
    }

    fn compatibility_rows(document: &Puzzle5dDocument) -> &[Value] {
        document.kind_compatibility.as_ref().and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default()
    }

    pub fn push(&mut self, mutation: Puzzle5dMutation) -> Result<(), Fault> {
        if let Some(pages) = self.mutation_pages.as_mut() {
            return pages.push(mutation).map_err(|_| Fault::from("puzzle5d-set-active-example-output-capacity"));
        }
        if self.mutations.len() >= self.mutations.capacity() {
            return Err(Fault::from("puzzle5d-set-active-example-output-capacity"));
        }
        self.mutations.push(mutation);
        Ok(())
    }
}

impl<A: Puzzle5dEditor> semio_plugin::retained_command::ArtifactCommandWork<EditorApp<A>> for Puzzle5dSetActiveExampleWork {
    fn tool_id(&self) -> &'static str {
        "setActiveExample"
    }

    /// 🧯️ A switch beyond the edit's chunk and page budget declares one unit to publish a localized notice.
    fn extent(&self, command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_plugin::app::ArtifactOwnedToolJobContext<EditorApp<A>>>) -> Option<usize> {
        let units = Self::units(command, snapshot)?;
        Some(if units <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS { units } else { 1 })
    }

    fn work_demands(&self, _input: &semio_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        if let Some(demand) = self.mutation_pages.as_ref().and_then(|pages| pages.next_page_demand()) {
            return Ok(demand);
        }
        crate::retained_command::step_demands::<EditorApp<A>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, cx: &mut StepContext<'_>) -> Result<semio_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, semio_framework_job::OriginalTurnFailure> {
        if let Some(pages) = self.mutation_pages.as_mut().filter(|pages| pages.next_page_demand().is_some()) {
            let progress = pages.admit_page(cx.retained_grant())?;
            cx.consume_retained(progress)?;
            cx.consume_fuel(1);
            return Ok(semio_plugin::retained_command::ArtifactCommandWorkStep::Progress {
                stage: "puzzle5d-example-mutation-page",
                preview: br#"{"en":"Preparing example edit","de":"Beispieländerung wird vorbereitet"}"#,
            });
        }
        let original = (|| -> Result<semio_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, semio_framework_job::OriginalTurnFailure> {

        let original: Result<_, Fault> = (|| {
        let semio_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config, interaction: _interaction, hover: _hover, context, .. } = *input;
        let view_state = context.and_then(|context| context.view_state.as_deref());
        if self.before.is_none() {
            let projection = puzzle5d_editor_projection(snapshot);
            let strings = |rows: Option<&Vec<Value>>, key: &str| -> Vec<String> {
                rows.map(|rows| rows.iter().filter_map(|row| row.get(key)).filter_map(Value::as_str).map(str::to_string).collect()).unwrap_or_default()
            };
            self.before = Some(std::sync::Arc::new(Puzzle5dSetActiveExampleBefore {
                fastener_ids: strings(projection.get("fasteners").and_then(Value::as_array), "id"),
                part_ids: strings(projection.get("parts").and_then(Value::as_array), "id"),
                compatibility: projection
                    .get("kindCompatibility")
                    .and_then(Value::as_array)
                    .map(|rows| rows.iter().map(|row| (row.get("source").and_then(Value::as_str).unwrap_or("").to_string(), row.get("target").and_then(Value::as_str).unwrap_or("").to_string())).collect())
                    .unwrap_or_default(),
            }));
        }
        if !self.admitted {
            self.admitted = true;
            if Self::target(command).is_none() || Self::units(command, snapshot).is_none_or(|units| units > crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS) {
                self.stage = Puzzle5dSetActiveExampleStage::Complete;
                return Ok(semio_plugin::retained_command::ArtifactCommandWorkStep::Complete(puzzle5d_notice_emit(view_state, |labels| labels.example_too_large.as_str())));
            }
        }
        let before = self.before.clone().unwrap_or_default();
        let Some(target) = Self::target(command) else {
            self.stage = Puzzle5dSetActiveExampleStage::Complete;
            return Ok(semio_plugin::retained_command::ArtifactCommandWorkStep::Complete(puzzle5d_notice_emit(view_state, |labels| labels.example_too_large.as_str())));
        };
        match self.stage {
            Puzzle5dSetActiveExampleStage::ClearFasteners => {
                let range = Self::take_chunk(&mut self.cursor, before.fastener_ids.len());
                if !range.is_empty() {
                    for id in before.fastener_ids[range].to_vec() {
                        self.push(crate::standards::v1::subsets::any::schema::mutations::disconnect_grips(id))?;
                    }
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle5d-example-clear-fastener", "Removing old fastener", "Alte Verbindung wird entfernt"));
                }
                self.cursor = 0;
                self.stage = Puzzle5dSetActiveExampleStage::ClearParts;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle5d-example-clear-part", "Removing old part", "Altes Teil wird entfernt"))
            }
            Puzzle5dSetActiveExampleStage::ClearParts => {
                let range = Self::take_chunk(&mut self.cursor, before.part_ids.len());
                if !range.is_empty() {
                    for id in before.part_ids[range].to_vec() {
                        self.push(crate::standards::v1::subsets::any::schema::mutations::delete_part(id))?;
                    }
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle5d-example-clear-part", "Removing old part", "Altes Teil wird entfernt"));
                }
                self.cursor = 0;
                self.stage = Puzzle5dSetActiveExampleStage::Label;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle5d-example-label", "Updating document label", "Dokumenttitel wird aktualisiert"))
            }
            Puzzle5dSetActiveExampleStage::Label => {
                self.push(crate::standards::v1::subsets::any::schema::mutations::rename_puzzle5d(target.label.clone()))?;
                self.stage = Puzzle5dSetActiveExampleStage::Domain;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle5d-example-domain", "Updating document domain", "Dokumentdomäne wird aktualisiert"))
            }
            Puzzle5dSetActiveExampleStage::Domain => {
                self.push(crate::standards::v1::subsets::any::schema::mutations::change_domain(target.domain.clone()))?;
                self.stage = Puzzle5dSetActiveExampleStage::Description;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle5d-example-description", "Updating description", "Beschreibung wird aktualisiert"))
            }
            Puzzle5dSetActiveExampleStage::Description => {
                let description = target.meta.as_ref().and_then(|meta| meta.get("description")).and_then(Value::as_str).unwrap_or("");
                self.push(crate::standards::v1::subsets::any::schema::mutations::change_description(description.to_string()))?;
                self.stage = Puzzle5dSetActiveExampleStage::ClearCompatibility;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle5d-example-clear-compatibility", "Removing old compatibility", "Alte Kompatibilität wird entfernt"))
            }
            Puzzle5dSetActiveExampleStage::ClearCompatibility => {
                let range = Self::take_chunk(&mut self.cursor, before.compatibility.len());
                if !range.is_empty() {
                    for (source, target) in before.compatibility[range].to_vec() {
                        self.push(crate::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility(source, target))?;
                    }
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle5d-example-clear-compatibility", "Removing old compatibility", "Alte Kompatibilität wird entfernt"));
                }
                self.cursor = 0;
                self.stage = Puzzle5dSetActiveExampleStage::AddCompatibility;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle5d-example-add-compatibility", "Adding compatibility", "Kompatibilität wird hinzugefügt"))
            }
            Puzzle5dSetActiveExampleStage::AddCompatibility => {
                let rows = Self::compatibility_rows(target);
                let range = Self::take_chunk(&mut self.cursor, rows.len());
                if !range.is_empty() {
                    for row in &rows[range] {
                        let row: crate::standards::v1::subsets::any::schema::document_records::Puzzle5dKindCompatibility = <crate::standards::v1::subsets::any::schema::document_records::Puzzle5dKindCompatibility as FromValue>::from_value(row.to_value()).map_err(|_| Fault::from("puzzle5d-set-active-example-compatibility-malformed"))?;
                        self.push(crate::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility(row.source, row.target, row.bidirectional, row.important, row.specificity, None))?;
                    }
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle5d-example-add-compatibility", "Adding compatibility", "Kompatibilität wird hinzugefügt"));
                }
                self.cursor = 0;
                self.stage = Puzzle5dSetActiveExampleStage::Catalogs;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle5d-example-catalogs", "Updating kind catalogs", "Artenkataloge werden aktualisiert"))
            }
            Puzzle5dSetActiveExampleStage::Catalogs => {
                let catalogs = target.kind_catalogs.as_ref().map(|catalogs| <crate::standards::v1::subsets::any::schema::document_records::Puzzle5dKindCatalogs as FromValue>::from_value(catalogs.to_value())).transpose().map_err(|_| Fault::from("puzzle5d-set-active-example-catalogs-malformed"))?;
                self.push(crate::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs(catalogs))?;
                self.stage = Puzzle5dSetActiveExampleStage::AddParts;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle5d-example-add-part", "Adding example part", "Beispielteil wird hinzugefügt"))
            }
            Puzzle5dSetActiveExampleStage::AddParts => {
                let range = Self::take_chunk(&mut self.cursor, target.parts.len());
                if !range.is_empty() {
                    for part in &target.parts[range] {
                        let value = part.to_value();
                        let part = <crate::standards::v1::subsets::any::schema::document_records::Puzzle5dPart as FromValue>::from_value(value.to_value()).map_err(|_| Fault::from("puzzle5d-set-active-example-part-malformed"))?;
                        self.push(crate::standards::v1::subsets::any::schema::mutations::create_part(part, None))?;
                    }
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle5d-example-add-part", "Adding example part", "Beispielteil wird hinzugefügt"));
                }
                self.cursor = 0;
                self.stage = Puzzle5dSetActiveExampleStage::AddFasteners;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle5d-example-add-fastener", "Adding example fastener", "Beispielverbindung wird hinzugefügt"))
            }
            Puzzle5dSetActiveExampleStage::AddFasteners => {
                let range = Self::take_chunk(&mut self.cursor, target.fasteners.len());
                if !range.is_empty() {
                    for fastener in &target.fasteners[range] {
                        self.push(crate::standards::v1::subsets::any::schema::mutations::connect_grips(
                            fastener.id.clone(),
                            fastener.source.clone(),
                            fastener.target.clone(),
                            fastener.fastener_kind.clone(),
                            fastener.gap,
                            fastener.shift,
                            fastener.rise,
                            fastener.rotation,
                            fastener.turn,
                            fastener.tilt,
                            fastener.x,
                            fastener.y, None,
                        ))?;
                    }
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle5d-example-add-fastener", "Adding example fastener", "Beispielverbindung wird hinzugefügt"));
                }
                self.stage = Puzzle5dSetActiveExampleStage::Complete;
                let selection = _interaction.selection.get(PUZZLE5D_INTERACTION_DOMAIN);
                let targets: Vec<InteractionTarget> = selection
                    .map(|selection| selection.ids.iter().map(|id| InteractionTarget { granularity: selection.granularity.clone(), id: id.clone() }).collect())
                    .unwrap_or_default();
                let interaction_writes = if targets.is_empty() { Vec::new() } else { vec![InteractionWrite { domain: PUZZLE5D_INTERACTION_DOMAIN.into(), targets, merge: MergeMode::Subtractive }] };
                Ok(semio_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit {
                    artifact_mutations: std::mem::take(&mut self.mutations),
                    artifact_mutation_pages: self.mutation_pages.take(),
                    config_mutations: config.mutations_to(&Puzzle5dConfig::default()),
                    interaction_writes,
                    ui_scope: UiDirtyScope::Full,
                    ..Default::default()
                }))
            }
            Puzzle5dSetActiveExampleStage::Complete => Err(Fault::from("puzzle5d-set-active-example-complete-repolled")),
            Puzzle5dSetActiveExampleStage::Closing => Err(Fault::from("puzzle5d-set-active-example-closing")),
        }
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dSetActiveExampleStage::Closing;
        self.close_owners.stage(Puzzle5dSetActiveExampleWorkOwners { mutations: std::mem::take(&mut self.mutations), mutation_pages: self.mutation_pages.take(), before: std::mem::take(&mut self.before).map(semio_framework_value::retirement::arc_custody::ArcLease::new) });
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        semio_framework_job::close_original_nested_step(cx,|cx,_grant|{
            self.close_owners.close_step(cx)
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
        self.stage == Puzzle5dSetActiveExampleStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}

/// 📏️ The retained command route's own wire-admission band, widened off the shared 8 KiB/512 puzzle
/// default to puzzle 3d's (`PUZZLE3D_IMPORT_RAW_BYTES`/`PUZZLE3D_IMPORT_DECODED_ITEMS`): one whole `importSnapshot`
/// file, escaped, plus its envelope (`PUZZLE_IMPORT_RAW_BYTES`) — and a Nakagin-sized document's camera/grid/sun
/// publications, which the narrow band rejects before the job ever admits.
pub const PUZZLE5D_RETAINED_RAW_BYTES: usize = crate::retained_command::PUZZLE_IMPORT_RAW_BYTES;

pub const PUZZLE5D_RETAINED_DECODED_ITEMS: usize = 16_384;

/// 🔢️ Mesh numbers one interactive step of a `registerBrushMesh` page transfers. A page is bounded by the
/// retained wire grant, so a whole stream costs a fixed, small number of steps and no step approaches the
/// lane's own time budget.
const PUZZLE5D_MESH_PAGE_VALUES: usize = 512;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Puzzle5dRegisterBrushMeshStage {
    Positions,
    Indices,
    Derive,
    Complete,
    Closing,
}

/// 📋️ `registerBrushMesh` as real retained Work: the browser's GLB round-trip arrives as two flat number
/// arrays, transferred here one bounded page per step into puzzle 3d's process-wide mesh store (which the
/// brush suggestions and fill runs read). HostOnly — that store is neither document nor config state — but
/// the transfer itself is real, cursored work, not the `Emit::default()` a `BoundedFirstStepCommandWork`
/// would have produced in one unbounded turn.
/// ♻️ The owners one `Puzzle5dRegisterBrushMeshWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dRegisterBrushMeshWorkOwners {
    positions: Vec<f32>,
    indices: Vec<u32>,
}

pub struct Puzzle5dRegisterBrushMeshWork {
    pub stage: Puzzle5dRegisterBrushMeshStage,
    position_cursor: usize,
    index_cursor: usize,
    positions: Vec<f32>,
    indices: Vec<u32>,
    pub close_owners: crate::puzzle_job::WorkClosing<Puzzle5dRegisterBrushMeshWorkOwners>,
}

impl Default for Puzzle5dRegisterBrushMeshWork {
    fn default() -> Self {
        Self { close_owners: Default::default(), stage: Puzzle5dRegisterBrushMeshStage::Positions, position_cursor: 0, index_cursor: 0, positions: Vec::new(), indices: Vec::new() }
    }
}

impl Puzzle5dRegisterBrushMeshWork {
    pub fn array<'a>(command: &'a Puzzle5dCommand, key: &str) -> &'a [Value] {
        command.args().and_then(|args| args.get(key)).and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default()
    }

    pub fn pages(command: &Puzzle5dCommand, key: &str) -> usize {
        Self::array(command, key).len().div_ceil(PUZZLE5D_MESH_PAGE_VALUES)
    }
}

impl<A: Puzzle5dEditor> semio_plugin::retained_command::ArtifactCommandWork<EditorApp<A>> for Puzzle5dRegisterBrushMeshWork {
    fn tool_id(&self) -> &'static str {
        "registerBrushMesh"
    }

    fn extent(&self, command: &Puzzle5dCommand, _snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_plugin::app::ArtifactOwnedToolJobContext<EditorApp<A>>>) -> Option<usize> {
        let items = Self::pages(command, "positions").checked_add(Self::pages(command, "indices"))?.checked_add(3)?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<A>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _cx: &mut StepContext<'_>) -> Result<semio_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, semio_framework_job::OriginalTurnFailure> {
        let original = (|| -> Result<semio_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, semio_framework_job::OriginalTurnFailure> {

        let original: Result<_, Fault> = (|| {
        let semio_plugin::retained_command::ArtifactCommandInputs { command, snapshot: _snapshot, config: _config, interaction: _interaction, hover: _hover, context, .. } = *input;
        let view_state = context.and_then(|context| context.view_state.as_deref());
        match self.stage {
            Puzzle5dRegisterBrushMeshStage::Positions => {
                let page = Self::array(command, "positions");
                let end = page.len().min(self.position_cursor.saturating_add(PUZZLE5D_MESH_PAGE_VALUES));
                self.positions.extend(page[self.position_cursor..end].iter().filter_map(|value| value.as_f64().map(|number| number as f32)));
                self.position_cursor = end;
                if end >= page.len() {
                    self.stage = Puzzle5dRegisterBrushMeshStage::Indices;
                }
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle5d-register-brush-mesh-position", "Reading mesh positions", "Mesh-Positionen werden gelesen"))
            }
            Puzzle5dRegisterBrushMeshStage::Indices => {
                let page = Self::array(command, "indices");
                let end = page.len().min(self.index_cursor.saturating_add(PUZZLE5D_MESH_PAGE_VALUES));
                self.indices.extend(page[self.index_cursor..end].iter().filter_map(|value| value.as_u64().and_then(|number| u32::try_from(number).ok())));
                self.index_cursor = end;
                if end >= page.len() {
                    self.stage = Puzzle5dRegisterBrushMeshStage::Derive;
                }
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle5d-register-brush-mesh-index", "Reading mesh indices", "Mesh-Indizes werden gelesen"))
            }
            Puzzle5dRegisterBrushMeshStage::Derive => {
                self.stage = Puzzle5dRegisterBrushMeshStage::Complete;
                let url = command.args().and_then(|args| args.get("url")).and_then(Value::as_str).unwrap_or_default().to_string();
                let positions = std::mem::take(&mut self.positions);
                let indices = std::mem::take(&mut self.indices);
                if !register_brush_mesh::puzzle5d_install_brush_mesh(&url, &positions, &indices) {
                    return Ok(semio_plugin::retained_command::ArtifactCommandWorkStep::Complete(puzzle5d_notice_emit(view_state, |labels| labels.brush_reason_pose_unavailable.as_str())));
                }
                Ok(semio_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { ui_scope: UiDirtyScope::None, ..Default::default() }))
            }
            Puzzle5dRegisterBrushMeshStage::Complete => Err(Fault::from("puzzle5d-register-brush-mesh-complete-repolled")),
            Puzzle5dRegisterBrushMeshStage::Closing => Err(Fault::from("puzzle5d-register-brush-mesh-closing")),
        }
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dRegisterBrushMeshStage::Closing;
        self.close_owners.stage(Puzzle5dRegisterBrushMeshWorkOwners { positions: std::mem::take(&mut self.positions), indices: std::mem::take(&mut self.indices) });
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        semio_framework_job::close_original_nested_step(cx,|cx,_grant|{
            self.close_owners.close_step(cx)
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
        self.stage == Puzzle5dRegisterBrushMeshStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}
//#endregion 🧵️RetainedCommands
//#endregion 🔖️PlayApp

impl Puzzle5dSetActiveExampleWork {
    pub fn original_birth_demands(capacity: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        if ArtifactMutationPages::<Puzzle5dMutation>::needs_pages(capacity) {
            return Ok(ArtifactMutationPages::<Puzzle5dMutation>::directory_demand(capacity));
        }
        let capacity_bytes = capacity.checked_mul(size_of::<Puzzle5dMutation>()).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "original Puzzle5D vector extent overflow"))?;
        Ok(semio_framework_value::RetirementDemand { capacity_bytes, depth: 1, ..Default::default() })
    }

    pub fn from_original_construction(capacity: usize, scope: &mut semio_framework_action_bus::OriginalToolConstructionScope<'_>) -> Result<(Self, semio_framework_value::RetainedCloneProgress), semio_framework_value::ValueError> {
        let demand = Self::original_birth_demands(capacity)?;
        scope.admit_owner(demand)?;
        if ArtifactMutationPages::<Puzzle5dMutation>::needs_pages(capacity) {
            let pages = ArtifactMutationPages::new(capacity)?;
            let retained_capacity_bytes = pages.directory_bytes();
            return Ok((Self { mutation_pages: Some(pages), ..Self::default() }, semio_framework_value::RetainedCloneProgress { retained_capacity_bytes, ..Default::default() }));
        }
        let mut mutations = Vec::new();
        mutations.try_reserve_exact(capacity).map_err(|_| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed, "original Puzzle5D example mutation allocation was refused"))?;
        let retained_capacity_bytes = mutations.capacity() * size_of::<Puzzle5dMutation>();
        Ok((Self { mutations, ..Self::default() }, semio_framework_value::RetainedCloneProgress { retained_capacity_bytes, ..Default::default() }))
    }
}
