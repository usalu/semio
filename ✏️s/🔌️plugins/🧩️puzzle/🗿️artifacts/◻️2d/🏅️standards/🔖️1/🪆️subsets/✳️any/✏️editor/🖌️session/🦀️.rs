//! 🖌️ Session of the puzzle 2d editor: the reserved import and clipboard jobs. See [`super`].

use crate::{editor::puzzle2d::snapshot::Puzzle2dInteractionSnapshot, editor::puzzle2d::snapshot::puzzle2d_clear_selection_write, editor::puzzle2d::recorder::puzzle2d_copy_fragment_from, editor::puzzle2d::recorder::puzzle2d_cut_operations_from, editor::puzzle2d::recorder::puzzle2d_paste_operations_on, editor::puzzle2d::snapshot::puzzle2d_selected_node_ids, editor::puzzle2d::snapshot::puzzle2d_selection_is_locked, editor::puzzle2d::snapshot::puzzle2d_selection_write};
use crate::editor::puzzle2d::config::Puzzle2dConfigMutation;
use crate::editor::puzzle2d::snapshot::Puzzle2dPlaySnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::{kernel::Effect, kernel::PastePlacement};
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::Media;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::app::EphemeralEmit;
use semio_framework_pack_json::{json, Value};
use semio_framework_job::StepContext;
use semio_framework_plugin::app::ArtifactToolCompletionRejection;
use semio_framework_plugin::{ArtifactReservedToolJobRequest, ArtifactToolCompletion, MediaPayload};
use crate::editor::puzzle2d::{Puzzle2dPlayApp, interaction, snapshot};

//#region 🔖️PlayApp
//#region 🧵️ReservedJobs

/// 🔢️ Fixed per-collection descriptor budget for one `kit:in` fragment — an oversized collection is
/// refused outright rather than silently truncated.
const PUZZLE2D_IMPORT_SEMANTIC_ITEMS: usize = 64;
const PUZZLE2D_IMPORT_DECODED_ITEMS: usize = 4_096;
/// 🧬️ At most one `connect-kind-compatibility` per admitted relation row, plus the single
/// `replace-kind-catalogs` that carries the merged bundle.
const PUZZLE2D_IMPORT_MUTATION_ITEMS: usize = PUZZLE2D_IMPORT_SEMANTIC_ITEMS + 1;
pub const PUZZLE2D_IMPORT_TOOL_ID: &str = "import-media";
pub const PUZZLE2D_IMPORT_PORT: &str = "kit:in";
/// 🗂️ The exact root keys a `kit.catalog` fragment may carry — the shape
/// `Block2dPlayApp::export_media("catalog:out")` produces via
/// `crate::standards::v1::subsets::any::schema::inferences::puzzle2d_manifest_fragment`, which is puzzle2d's
/// own manifest vocabulary (`s/plugin/puzzle/app/2d/manifest/🔣️.json`), not block3d's.
const PUZZLE2D_IMPORT_ROOT_KEYS: &[&str] = &["schema", "id", "name", "axes", "portKinds", "wireKinds", "edgeKinds", "nodeKinds", "kindCompatibility"];
const PUZZLE2D_IMPORT_COLLECTIONS: &[&str] = &["portKinds", "wireKinds", "edgeKinds", "nodeKinds", "kindCompatibility"];

fn puzzle2d_job_fault(detail: &str) -> crate::puzzle_job::JobTurn {
    let bytes = detail.as_bytes();
    crate::puzzle_job::JobTurn::Fault(bytes[..bytes.len().min(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)].to_vec())
}

fn puzzle2d_import_checkpoint(stage: u8, cursor: usize, decoded_items: usize, progress: u64) -> crate::puzzle_job::JobTurn {
    let mut state = [0u8; 25];
    state[0] = stage;
    state[1..9].copy_from_slice(&(cursor as u64).to_le_bytes());
    state[9..17].copy_from_slice(&(decoded_items as u64).to_le_bytes());
    state[17..25].copy_from_slice(&progress.to_le_bytes());
    crate::puzzle_job::JobTurn::Checkpoint { applied_progress: progress, state: state.to_vec() }
}

fn puzzle2d_import_text(row: &Value, key: &str) -> semio_framework_value::paged::PagedUtf8<{ usize::MAX }> {
    row.get(key).and_then(Value::as_str).unwrap_or_default().into()
}

fn puzzle2d_import_identity(row: &Value) -> Option<&str> {
    row.get("id").and_then(Value::as_str).filter(|id| !id.trim().is_empty())
}

fn puzzle2d_import_presentation<'a>(row: &'a Value, key: &str) -> Option<&'a Value> {
    row.get("presentation").and_then(|presentation| presentation.get(key))
}

fn puzzle2d_import_presentation_text(row: &Value, key: &str) -> semio_framework_value::paged::PagedUtf8<{ usize::MAX }> {
    puzzle2d_import_presentation(row, key).and_then(Value::as_str).unwrap_or_default().into()
}

/// 🏷️ Manifest rows carry `name` and no `label`; the catalog row wants both, so an absent label
/// mirrors the name rather than inventing a third spelling.
fn puzzle2d_import_label(row: &Value, name: &str) -> semio_framework_value::paged::PagedUtf8<{ usize::MAX }> {
    row.get("label").and_then(Value::as_str).filter(|label| !label.trim().is_empty()).unwrap_or(name).into()
}

fn puzzle2d_import_handle_template(node_kind_id: &str, index: usize, template: &Value) -> Option<crate::standards::v1::subsets::any::schema::records::Puzzle2dHandleTemplate> {
    let handle_kind = template.get("handleKind").and_then(Value::as_str).filter(|kind| !kind.trim().is_empty())?;
    let name = template.get("name").and_then(Value::as_str).unwrap_or_default();
    Some(crate::standards::v1::subsets::any::schema::records::Puzzle2dHandleTemplate {
        id: puzzle2d_import_identity(template).map_or_else(|| format!("{node_kind_id}-h{index}"), str::to_string).into(),
        label: puzzle2d_import_label(template, name),
        name: name.into(),
        description: puzzle2d_import_text(template, "description"),
        icon: puzzle2d_import_text(template, "icon"),
        handle_kind: Some(handle_kind.into()),
        angle: template.get("angle").and_then(Value::as_f64).filter(|angle| angle.is_finite()).unwrap_or_default(),
        t: template.get("t").and_then(Value::as_f64).filter(|t| t.is_finite()),
        mandatory: template.get("mandatory").and_then(Value::as_bool),
        radius: template.get("radius").and_then(Value::as_f64).filter(|radius| radius.is_finite()),
    })
}

fn puzzle2d_import_node_kind(row: &Value) -> Option<crate::standards::v1::subsets::any::schema::records::Puzzle2dCatalogNodeKind> {
    let id = puzzle2d_import_identity(row)?;
    let name = row.get("name").and_then(Value::as_str).unwrap_or(id);
    let handles = puzzle2d_import_presentation(row, "handles").and_then(Value::as_array).map_or_else(semio_framework_value::list::PagedList::new, |templates| templates.iter().enumerate().filter_map(|(index, template)| puzzle2d_import_handle_template(id, index, template)).collect());
    Some(crate::standards::v1::subsets::any::schema::records::Puzzle2dCatalogNodeKind {
        id: id.into(),
        label: puzzle2d_import_label(row, name),
        name: name.into(),
        description: puzzle2d_import_text(row, "description"),
        icon: puzzle2d_import_presentation_text(row, "icon"),
        image: puzzle2d_import_presentation_text(row, "image"),
        unit: puzzle2d_import_text(row, "unit"),
        is_abstract: row.get("abstract").and_then(Value::as_bool).unwrap_or_default(),
        base_kinds: Default::default(),
        representations: Default::default(),
        handles,
        attributes: Default::default(),
        authors: Default::default(),
    })
}

fn puzzle2d_import_handle_kind(row: &Value) -> Option<crate::standards::v1::subsets::any::schema::records::Puzzle2dCatalogHandleKind> {
    let id = puzzle2d_import_identity(row)?;
    Some(crate::standards::v1::subsets::any::schema::records::Puzzle2dCatalogHandleKind {
        id: id.into(),
        code: None,
        label: Some(puzzle2d_import_label(row, row.get("name").and_then(Value::as_str).unwrap_or(id))),
        order: None,
        compatible_with: Default::default(),
        description: puzzle2d_import_text(row, "description"),
        icon: puzzle2d_import_presentation_text(row, "icon"),
        color: puzzle2d_import_presentation_text(row, "color"),
        default_wire_kind: puzzle2d_import_presentation_text(row, "defaultWireKind"),
    })
}

fn puzzle2d_import_edge_kind(row: &Value) -> Option<crate::standards::v1::subsets::any::schema::records::Puzzle2dCatalogEdgeKind> {
    let id = puzzle2d_import_identity(row)?;
    let name = row.get("name").and_then(Value::as_str).unwrap_or(id);
    Some(crate::standards::v1::subsets::any::schema::records::Puzzle2dCatalogEdgeKind {
        id: id.into(),
        label: puzzle2d_import_label(row, name),
        name: name.into(),
        description: puzzle2d_import_text(row, "description"),
        icon: puzzle2d_import_presentation_text(row, "icon"),
        color: puzzle2d_import_presentation_text(row, "color"),
    })
}

fn puzzle2d_import_wire_kind(row: &Value) -> Option<crate::standards::v1::subsets::any::schema::records::Puzzle2dCatalogWireKind> {
    let id = puzzle2d_import_identity(row)?;
    let name = row.get("name").and_then(Value::as_str).unwrap_or(id);
    Some(crate::standards::v1::subsets::any::schema::records::Puzzle2dCatalogWireKind {
        id: id.into(),
        label: puzzle2d_import_label(row, name),
        name: name.into(),
        description: puzzle2d_import_text(row, "description"),
        icon: puzzle2d_import_presentation_text(row, "icon"),
        color: puzzle2d_import_presentation_text(row, "color"),
        default_edge_kind: puzzle2d_import_presentation_text(row, "defaultEdgeKind"),
    })
}

/// 🗂️ Id-keyed upsert — deterministic and order-independent, so a `multiplicity: Many` port that fans
/// in several catalog producers converges on the same bundle whatever order they arrive in. Reports
/// whether the bundle actually changed, so an idempotent re-delivery emits no operation at all.
fn puzzle2d_upsert_catalog_row<T: PartialEq>(rows: &mut semio_framework_value::list::PagedList<T, { usize::MAX }>, incoming: T, matches: impl Fn(&T) -> bool) -> bool {
    match rows.iter().position(matches) {
        Some(index) if rows[index] == incoming => false,
        Some(index) => {
            rows[index] = incoming;
            true
        }
        None => {
            rows.push(incoming);
            true
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle2dImportStage {
    Decode,
    HandleKinds,
    NodeKinds,
    EdgeKinds,
    WireKinds,
    Compatibility,
    CatalogMutation,
    Complete,
}

/// 🎞️ `kit:in`'s concrete resumable importer: it normalizes one `kit.catalog` manifest fragment
/// (`portKinds`/`wireKinds`/`edgeKinds`/`nodeKinds`/`kindCompatibility`) into puzzle2d's own typed
/// `meta.kindCatalogs` vocabulary (`handles`/`wires`/`edges`/`nodes`) and its `meta.kindCompatibility`
/// table, one row per bounded step, and publishes the result as real undoable `Puzzle2dMutation`s —
/// never a direct state write. The producer this pairs with is `Block2dPlayApp`'s `"catalog:out"`
/// port, whose payload IS this exact 2d manifest shape (see
/// `🧱️block/🗿️artifacts/◻️2d/…/💡️inferences/🦀️.rs`'s `puzzle2d_manifest_fragment`).
pub struct Puzzle2dImportJob {
    pub port: String,
    media_json: Option<String>,
    pub snapshot: Option<std::sync::Arc<Puzzle2dPlaySnapshot>>,
    pub fragment: Option<Value>,
    pub catalogs: crate::standards::v1::subsets::any::schema::records::Puzzle2dKindCatalogs,
    compatibility: Vec<crate::standards::v1::subsets::any::schema::records::Puzzle2dKindCompatibility>,
    pub mutations: Vec<Puzzle2dMutation>,
    pub stage: Puzzle2dImportStage,
    pub cursor: usize,
    decoded_items: usize,
    pub progress: u64,
    catalog_changed: bool,
    pub completion: Option<ArtifactToolCompletion<EditorApp<Puzzle2dPlayApp>>>,
    pending_completion_rejection: Option<ArtifactToolCompletionRejection<EditorApp<Puzzle2dPlayApp>>>,
    completed: bool,
    outbox: crate::puzzle_job::JobOutbox,
    closing: bool,
    pub owners: crate::puzzle_job::WorkClosing<Puzzle2dImportOwners>,
}

/// ♻️ The owners one `Puzzle2dImportJob` still holds when it closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle2dImportOwners {
    pub port: String,
    media_json: Option<String>,
    pub snapshot: Option<std::sync::Arc<Puzzle2dPlaySnapshot>>,
    pub fragment: Option<Value>,
    pub catalogs: crate::standards::v1::subsets::any::schema::records::Puzzle2dKindCatalogs,
    compatibility: Vec<crate::standards::v1::subsets::any::schema::records::Puzzle2dKindCompatibility>,
    pub mutations: Vec<Puzzle2dMutation>,
    pub completion: Option<ArtifactToolCompletion<EditorApp<Puzzle2dPlayApp>>>,
    pending_completion_rejection: Option<ArtifactToolCompletionRejection<EditorApp<Puzzle2dPlayApp>>>,
}

impl Puzzle2dImportJob {
    pub fn new(request: ArtifactReservedToolJobRequest<EditorApp<Puzzle2dPlayApp>>, port: String, media: Media) -> Self {
        let media_json = match media.payload {
            MediaPayload::Structured { json, .. } => Some(json),
            MediaPayload::Intrinsic { value, .. } => Some(semio_framework_pack_json::to_json_string(&value)),
            MediaPayload::Binary { .. } => None,
        };
        Self {
            port,
            media_json,
            snapshot: Some(request.snapshot),
            fragment: None,
            catalogs: crate::standards::v1::subsets::any::schema::records::Puzzle2dKindCatalogs::default(),
            compatibility: Vec::new(),
            mutations: Vec::new(),
            stage: Puzzle2dImportStage::Decode,
            cursor: 0,
            decoded_items: 0,
            progress: 0,
            catalog_changed: false,
            completion: Some(request.completion),
            pending_completion_rejection: None,
            completed: false,
            outbox: Default::default(),
            closing: false,
            owners: Default::default(),
        }
    }

    pub fn checkpoint(&self) -> crate::puzzle_job::JobTurn {
        puzzle2d_import_checkpoint(self.stage as u8, self.cursor, self.decoded_items, self.progress)
    }

    pub fn rows(&self, key: &str) -> &[Value] {
        self.fragment.as_ref().and_then(|fragment| fragment.get(key)).and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[])
    }

    pub fn meta(&self, key: &str) -> Option<&Value> {
        self.snapshot.as_ref().and_then(|snapshot| snapshot.value().get("meta")).and_then(|meta| meta.get(key)).filter(|value| !value.is_null())
    }

    fn push_mutation(&mut self, mutation: Puzzle2dMutation) -> Result<(), &'static str> {
        if self.mutations.len() >= PUZZLE2D_IMPORT_MUTATION_ITEMS {
            return Err("puzzle2d kit:in mutation limit exceeded");
        }
        self.mutations.push(mutation);
        Ok(())
    }

    pub fn decode(&mut self) -> Option<crate::puzzle_job::JobTurn> {
        if self.port != PUZZLE2D_IMPORT_PORT {
            return Some(puzzle2d_job_fault("puzzle2d import only implements kit:in"));
        }
        let Some(media_json) = self.media_json.as_ref() else {
            return Some(puzzle2d_job_fault("puzzle2d kit:in requires a structured payload"));
        };
        let Ok(fragment) = semio_framework_pack_json::parse(media_json,semio_framework_pack_json::JsonMemberPolicy::Reject) else {
            return Some(puzzle2d_job_fault("puzzle2d kit:in payload is not valid json"));
        };
        if fragment.as_object().is_none() {
            return Some(puzzle2d_job_fault("puzzle2d kit:in root must be an object"));
        }
        if fragment.as_object().is_some_and(|root| root.iter().any(|(key, _)| !PUZZLE2D_IMPORT_ROOT_KEYS.contains(&key))) {
            return Some(puzzle2d_job_fault("puzzle2d kit:in root contains an unknown field"));
        }
        if fragment.get("schema").is_some_and(|value| value.as_str() != Some("manifest")) {
            return Some(puzzle2d_job_fault("puzzle2d kit:in schema must be manifest when present"));
        }
        if PUZZLE2D_IMPORT_COLLECTIONS.iter().any(|key| fragment.get(*key).is_some_and(|value| value.as_array().is_none())) {
            return Some(puzzle2d_job_fault("puzzle2d kit:in collection must be an array"));
        }
        if PUZZLE2D_IMPORT_COLLECTIONS.iter().any(|key| fragment.get(*key).and_then(Value::as_array).is_some_and(|rows| rows.len() > PUZZLE2D_IMPORT_SEMANTIC_ITEMS)) {
            return Some(puzzle2d_job_fault("puzzle2d kit:in collection exceeds its fixed descriptor cap"));
        }
        let existing_catalogs = self.meta("kindCatalogs").cloned();
        let existing_compatibility = self.meta("kindCompatibility").and_then(|value| value.as_array().cloned()).unwrap_or_default();
        let fragment_items = PUZZLE2D_IMPORT_COLLECTIONS.iter().try_fold(0usize, |total, key| total.checked_add(fragment.get(*key).and_then(Value::as_array).map_or(0, Vec::len)));
        let catalog_items = existing_catalogs.as_ref().map(|catalogs| ["nodes", "handles", "edges", "wires"].into_iter().map(|slice| catalogs.get(slice).and_then(Value::as_array).map_or(0, Vec::len)).sum::<usize>()).unwrap_or_default();
        self.decoded_items = match fragment_items.and_then(|items| items.checked_add(catalog_items)).and_then(|items| items.checked_add(existing_compatibility.len())) {
            Some(items) if items <= PUZZLE2D_IMPORT_DECODED_ITEMS => items,
            _ => return Some(puzzle2d_job_fault("puzzle2d kit:in decoded item limit exceeded")),
        };
        self.catalogs = match existing_catalogs.as_ref() {
            Some(value) => match <crate::standards::v1::subsets::any::schema::records::Puzzle2dKindCatalogs as semio_framework_value::FromValue>::from_value(semio_framework_value::ToValue::to_value(value)) {
                Ok(catalogs) => catalogs,
                Err(_) => return Some(puzzle2d_job_fault("puzzle2d kit:in cannot read the document's own kind catalogs")),
            },
            None => crate::standards::v1::subsets::any::schema::records::Puzzle2dKindCatalogs::default(),
        };
        for row in &existing_compatibility {
            match <crate::standards::v1::subsets::any::schema::records::Puzzle2dKindCompatibility as semio_framework_value::FromValue>::from_value(semio_framework_value::ToValue::to_value(row)) {
                Ok(parsed) => self.compatibility.push(parsed),
                Err(_) => return Some(puzzle2d_job_fault("puzzle2d kit:in cannot read the document's own kind relations")),
            }
        }
        self.catalog_changed = existing_catalogs.is_none() && PUZZLE2D_IMPORT_COLLECTIONS.iter().any(|key| fragment.get(*key).and_then(Value::as_array).is_some_and(|rows| !rows.is_empty()));
        self.fragment = Some(fragment);
        None
    }
}

impl Puzzle2dImportJob {
    pub fn turn(&mut self, cx: &mut StepContext<'_>) -> crate::puzzle_job::JobTurn {
        if self.closing || cx.is_cancelled() {
            return crate::puzzle_job::JobTurn::Cancelled;
        }
        if self.pending_completion_rejection.is_some() {
            return puzzle2d_job_fault("puzzle2d import completion remains rejected");
        }
        match self.stage {
            Puzzle2dImportStage::Decode => {
                cx.set_stage("puzzle2d-import-decode");
                if let Some(outcome) = self.decode() {
                    return outcome;
                }
                self.stage = Puzzle2dImportStage::HandleKinds;
                self.cursor = 0;
            }
            Puzzle2dImportStage::HandleKinds => {
                cx.set_stage("puzzle2d-import-handle-kind");
                let admitted = self.rows("portKinds").get(self.cursor).map(puzzle2d_import_handle_kind);
                match admitted {
                    Some(None) => return puzzle2d_job_fault("puzzle2d kit:in port kind is missing its identity"),
                    Some(Some(kind)) => {
                        let id = kind.id.clone();
                        let changed = puzzle2d_upsert_catalog_row(&mut self.catalogs.handles, kind, |row| row.id == id);
                        self.catalog_changed |= changed;
                        self.cursor += 1;
                    }
                    None => {
                        self.stage = Puzzle2dImportStage::NodeKinds;
                        self.cursor = 0;
                    }
                }
            }
            Puzzle2dImportStage::NodeKinds => {
                cx.set_stage("puzzle2d-import-node-kind");
                let admitted = self.rows("nodeKinds").get(self.cursor).map(puzzle2d_import_node_kind);
                match admitted {
                    Some(None) => return puzzle2d_job_fault("puzzle2d kit:in node kind is missing its identity"),
                    Some(Some(kind)) => {
                        let id = kind.id.clone();
                        let changed = puzzle2d_upsert_catalog_row(&mut self.catalogs.nodes, kind, |row| row.id == id);
                        self.catalog_changed |= changed;
                        self.cursor += 1;
                    }
                    None => {
                        self.stage = Puzzle2dImportStage::EdgeKinds;
                        self.cursor = 0;
                    }
                }
            }
            Puzzle2dImportStage::EdgeKinds => {
                cx.set_stage("puzzle2d-import-edge-kind");
                let admitted = self.rows("edgeKinds").get(self.cursor).map(puzzle2d_import_edge_kind);
                match admitted {
                    Some(None) => return puzzle2d_job_fault("puzzle2d kit:in edge kind is missing its identity"),
                    Some(Some(kind)) => {
                        let id = kind.id.clone();
                        let changed = puzzle2d_upsert_catalog_row(&mut self.catalogs.edges, kind, |row| row.id == id);
                        self.catalog_changed |= changed;
                        self.cursor += 1;
                    }
                    None => {
                        self.stage = Puzzle2dImportStage::WireKinds;
                        self.cursor = 0;
                    }
                }
            }
            Puzzle2dImportStage::WireKinds => {
                cx.set_stage("puzzle2d-import-wire-kind");
                let admitted = self.rows("wireKinds").get(self.cursor).map(puzzle2d_import_wire_kind);
                match admitted {
                    Some(None) => return puzzle2d_job_fault("puzzle2d kit:in wire kind is missing its identity"),
                    Some(Some(kind)) => {
                        let id = kind.id.clone();
                        let changed = puzzle2d_upsert_catalog_row(&mut self.catalogs.wires, kind, |row| row.id == id);
                        self.catalog_changed |= changed;
                        self.cursor += 1;
                    }
                    None => {
                        self.stage = Puzzle2dImportStage::Compatibility;
                        self.cursor = 0;
                    }
                }
            }
            Puzzle2dImportStage::Compatibility => {
                cx.set_stage("puzzle2d-import-kind-relation");
                let admitted = self.rows("kindCompatibility").get(self.cursor).cloned();
                let Some(row) = admitted else {
                    self.stage = Puzzle2dImportStage::CatalogMutation;
                    self.cursor = 0;
                    self.progress = self.progress.saturating_add(1);
                    cx.consume_fuel(1);
                    return self.checkpoint();
                };
                let Ok(parsed) = <crate::standards::v1::subsets::any::schema::records::Puzzle2dKindCompatibility as semio_framework_value::FromValue>::from_value(semio_framework_value::ToValue::to_value(&row)) else {
                    return puzzle2d_job_fault("puzzle2d kit:in kind relation is malformed");
                };
                let existing = self.compatibility.iter().position(|entry| entry.source == parsed.source && entry.target == parsed.target);
                if !existing.is_some_and(|index| self.compatibility[index] == parsed) {
                    match existing {
                        Some(index) => self.compatibility[index] = parsed.clone(),
                        None => self.compatibility.push(parsed.clone()),
                    }
                    if let Err(error) = self.push_mutation(crate::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility(parsed.source, parsed.target, parsed.bidirectional, parsed.important, parsed.specificity, None)) {
                        return puzzle2d_job_fault(error);
                    }
                }
                self.cursor += 1;
            }
            Puzzle2dImportStage::CatalogMutation => {
                cx.set_stage("puzzle2d-import-kind-catalogs");
                if self.catalog_changed {
                    let mutation = crate::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs(Some(std::mem::take(&mut self.catalogs)));
                    if let Err(error) = self.push_mutation(mutation) {
                        return puzzle2d_job_fault(error);
                    }
                    self.catalog_changed = false;
                }
                self.stage = Puzzle2dImportStage::Complete;
            }
            Puzzle2dImportStage::Complete => {
                cx.set_stage("puzzle2d-import-publish");
                if !self.completed {
                    let mutations = std::mem::take(&mut self.mutations);
                    let Some(completion) = self.completion.as_ref() else {
                        return puzzle2d_job_fault("puzzle2d import lost its completion authority");
                    };
                    if !completion.has_mounted_consumer() {
                        return puzzle2d_job_fault("puzzle2d import completion consumer is absent");
                    }
                    if let Err(rejected) = completion.complete(Ok(Emit { artifact_mutations: mutations, ui_scope: UiDirtyScope::Full, ..Default::default() }), EphemeralEmit::default()) {
                        let message = rejected.fault.message.clone();
                        self.pending_completion_rejection = Some(rejected);
                        return puzzle2d_job_fault(&message);
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

impl Puzzle2dImportJob {
    /// 🪜️ The step body; its value refusals are lifted into the original failure at the trait boundary.
    fn step_value<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
        match self.outbox.phase(cx)? {
            crate::puzzle_job::OutboxPhase::Building => return self.outbox.advance(cx),
            crate::puzzle_job::OutboxPhase::Delivered | crate::puzzle_job::OutboxPhase::Retiring => {
                self.outbox.retire_step(cx)?;
                return Ok(None);
            }
            crate::puzzle_job::OutboxPhase::Idle => {}
        }
        let turn = if cx.fuel_exhausted() || cx.deadline_exceeded() { crate::puzzle_job::JobTurn::Yield } else { self.turn(cx) };
        self.outbox.settle(turn, cx)
    }
}

impl InteractiveJob for Puzzle2dImportJob {
    fn step<'a>(&'a mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure>{self.step_value(cx).map_err(semio_framework_job::OriginalTurnFailure::from)}

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        self.outbox.borrow_outcome(descriptor)
    }

    fn begin_close(&mut self) {
        if std::mem::replace(&mut self.closing, true) {
            return;
        }
        self.owners.stage(Puzzle2dImportOwners {
            port: std::mem::take(&mut self.port),
            media_json: self.media_json.take(),
            snapshot: self.snapshot.take(),
            fragment: self.fragment.take(),
            catalogs: std::mem::take(&mut self.catalogs),
            compatibility: std::mem::take(&mut self.compatibility),
            mutations: std::mem::take(&mut self.mutations),
            completion: self.completion.take(),
            pending_completion_rejection: self.pending_completion_rejection.take(),
        });
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        semio_framework_job::close_original_granted_step(cx,|grant|{
            self.begin_close();
            crate::puzzle_job::job_close_step(&mut self.outbox, &mut self.owners, grant)
        })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.owners, 0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.owners, maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.owners, 0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.owners, 0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.outbox.terminal_is_empty() && self.owners.is_empty()
    }
}

impl ArtifactReservedJob for Puzzle2dImportJob {}

/// 📋️ One-step reserved copy/cut/paste job — the framework route is an empty stub unless the app owns
/// this producer, so puzzle2d owning it is what makes `mod+c`/`mod+x`/`mod+v` and the context-menu
/// clipboard rows do anything at all. Every arm answers in one step: a cut is copy-plus-delete as ONE
/// artifact edit, and a paste is one insert whose clones the app re-selects.
pub struct Puzzle2dClipboardJob {
    pub tool_id: String,
    pub labels: Option<&'static crate::editor::puzzle2d::terminology::Puzzle2dLabels>,
    pub snapshot: Option<std::sync::Arc<Puzzle2dPlaySnapshot>>,
    pub raw_wire: Vec<u8>,
    pub args: Option<semio_framework_value::DslValue>,
    pub interaction: Option<semio_framework::InteractionState>,
    pub hover: Option<semio_framework_plugin::app::InteractionHoverState>,
    pub completion: Option<ArtifactToolCompletion<EditorApp<Puzzle2dPlayApp>>>,
    rejection: Option<ArtifactToolCompletionRejection<EditorApp<Puzzle2dPlayApp>>>,
    outbox: crate::puzzle_job::JobOutbox,
    closing: bool,
    pub owners: crate::puzzle_job::WorkClosing<Puzzle2dClipboardOwners>,
}

/// ♻️ The owners one `Puzzle2dClipboardJob` still holds when it closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle2dClipboardOwners {
    pub args: Option<semio_framework_value::DslValue>,
    pub interaction: Option<semio_framework::InteractionState>,
    pub hover: Option<semio_framework_plugin::app::InteractionHoverState>,
    pub tool_id: String,
    pub snapshot: Option<std::sync::Arc<Puzzle2dPlaySnapshot>>,
    pub raw_wire: Vec<u8>,
    pub completion: Option<ArtifactToolCompletion<EditorApp<Puzzle2dPlayApp>>>,
    rejection: Option<ArtifactToolCompletionRejection<EditorApp<Puzzle2dPlayApp>>>,
}

impl Puzzle2dClipboardJob {
    pub fn new(request: ArtifactReservedToolJobRequest<EditorApp<Puzzle2dPlayApp>>) -> Self {
        let ArtifactReservedToolInput::Action { args, interaction, hover } = request.input else { unreachable!("clipboard routes carry action input") };
        let labels = request.ui_axes.map(|(locale, terminology)| puzzle2d_labels(&semio_framework_plugin::ViewModel::new(locale, terminology)));
        Self { tool_id: request.tool_id, labels, snapshot: Some(request.snapshot), raw_wire: request.raw_wire, args, interaction: Some(interaction), hover: Some(hover), completion: Some(request.completion), rejection: None, outbox: Default::default(), closing: false, owners: Default::default() }
    }

    pub fn emit(&mut self) -> Result<Emit<Puzzle2dMutation, Puzzle2dConfigMutation, NoDraftMutation>, Fault> {
        let (Some(interaction), Some(hover)) = (self.interaction.as_ref(), self.hover.as_ref()) else {
            return Ok(Emit::default());
        };
        let args = self.args.as_ref();
        let Some(owned) = self.snapshot.clone() else { return Ok(Emit::default()) };
        let snapshot = owned.value();
        let marks = Puzzle2dInteractionSnapshot::from_state(&interaction, &hover);
        let selected = puzzle2d_selected_node_ids(snapshot, marks.selected_ids());
        Ok(match self.tool_id.as_str() {
            // 🧾️ An unresolvable selection answers with ZERO effects — the same shape as "the hotkey
            // never reached the guest", exactly as puzzle3d's clipboard route answers it.
            "copy" => match puzzle2d_copy_fragment_from(snapshot, &selected) {
                Ok(fragment) => Emit { effects: vec![Effect::ClipboardWrite { fragment }], ui_scope: UiDirtyScope::None, ..Default::default() },
                Err(_) => Emit::default(),
            },
            "cut" => {
                let Ok(fragment) = puzzle2d_copy_fragment_from(snapshot, &selected) else { return Ok(Emit::default()) };
                if puzzle2d_selection_is_locked(snapshot, &selected) {
                    let labels = self.labels.ok_or_else(|| Fault::from("puzzle2d clipboard notification requires the caller's explicit presentation axes"))?;
                    return Ok(Emit { effects: vec![Effect::Notify { message: labels.cut_locked.as_str().to_string() }], ui_scope: UiDirtyScope::None, ..Default::default() });
                }
                let mutations = puzzle2d_cut_operations_from(&owned, &selected);
                let clear = puzzle2d_clear_selection_write(snapshot, &selected).into_iter().collect();
                Emit { artifact_mutations: mutations, effects: vec![Effect::ClipboardWrite { fragment }], interaction_writes: clear, ..Default::default() }
            }
            "paste" => {
                let Some(fragment) = args.as_ref().and_then(|value| value.get("fragment")).and_then(|value| semio_framework_value::FromValue::from_value(value.clone()).ok()) else {
                    return Ok(Emit::default());
                };
                let placement = PastePlacement::default();
                match puzzle2d_paste_operations_on(&owned, &fragment, &placement) {
                    Ok((mutations, pasted)) => Emit { artifact_mutations: mutations, interaction_writes: vec![puzzle2d_selection_write(snapshot, &pasted)], ..Default::default() },
                    Err(_) => Emit::default(),
                }
            }
            _ => Emit::default(),
        })
    }
}

impl Puzzle2dClipboardJob {
    /// 🪜️ The step body; its value refusals are lifted into the original failure at the trait boundary.
    fn step_value<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
        match self.outbox.phase(cx)? {
            crate::puzzle_job::OutboxPhase::Building => return self.outbox.advance(cx),
            crate::puzzle_job::OutboxPhase::Delivered | crate::puzzle_job::OutboxPhase::Retiring => {
                self.outbox.retire_step(cx)?;
                return Ok(None);
            }
            crate::puzzle_job::OutboxPhase::Idle => {}
        }
        let turn = self.turn(cx);
        self.outbox.settle(turn, cx)
    }
}

impl InteractiveJob for Puzzle2dClipboardJob {
    fn step<'a>(&'a mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure>{self.step_value(cx).map_err(semio_framework_job::OriginalTurnFailure::from)}

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        self.outbox.borrow_outcome(descriptor)
    }

    fn begin_close(&mut self) {
        if std::mem::replace(&mut self.closing, true) {
            return;
        }
        self.owners.stage(Puzzle2dClipboardOwners { args: self.args.take(), interaction: self.interaction.take(), hover: self.hover.take(), tool_id: std::mem::take(&mut self.tool_id), snapshot: self.snapshot.take(), raw_wire: std::mem::take(&mut self.raw_wire), completion: self.completion.take(), rejection: self.rejection.take() });
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        semio_framework_job::close_original_granted_step(cx,|grant|{
            self.begin_close();
            crate::puzzle_job::job_close_step(&mut self.outbox, &mut self.owners, grant)
        })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.owners, 0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.owners, maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.owners, 0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.owners, 0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.outbox.terminal_is_empty() && self.owners.is_empty()
    }
}

impl Puzzle2dClipboardJob {
    pub fn turn(&mut self, cx: &StepContext<'_>) -> crate::puzzle_job::JobTurn {
        if self.closing || cx.is_cancelled() {
            return crate::puzzle_job::JobTurn::Cancelled;
        }
        if !self.raw_wire.is_empty() {
            return crate::puzzle_job::JobTurn::Prepare(std::mem::take(&mut self.raw_wire));
        }
        let emit = self.emit();
        let Some(completion) = self.completion.as_ref() else { return crate::puzzle_job::JobTurn::Fault(b"puzzle2d-clipboard-completion-owner".to_vec()) };
        if let Err(rejected) = completion.complete(emit, EphemeralEmit::default()) {
            self.rejection = Some(rejected);
            return crate::puzzle_job::JobTurn::Fault(b"puzzle2d-clipboard-completion-rejected".to_vec());
        }
        crate::puzzle_job::JobTurn::Complete
    }
}

impl ArtifactReservedJob for Puzzle2dClipboardJob {}
//#endregion 🧵️ReservedJobs
//#endregion 🔖️PlayApp
