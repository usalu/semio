//! 🖌️ Session of the puzzle 2d editor: the reserved import and clipboard jobs. See [`super`].

use crate::{editor::puzzle2d::snapshot::Puzzle2dInteractionSnapshot, editor::puzzle2d::snapshot::puzzle2d_clear_selection_write, editor::puzzle2d::recorder::puzzle2d_copy_fragment_from, editor::puzzle2d::recorder::puzzle2d_cut_operations_from, editor::puzzle2d::recorder::puzzle2d_paste_operations_on, editor::puzzle2d::snapshot::puzzle2d_selected_node_ids, editor::puzzle2d::snapshot::puzzle2d_selection_is_locked, editor::puzzle2d::snapshot::puzzle2d_selection_write};
use crate::editor::puzzle2d::config::Puzzle2dConfigMutation;
use crate::editor::puzzle2d::terminology::puzzle2d_labels;
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
use semio_framework_plugin::component::ArtifactReservedToolInput;
use semio_framework_job::InteractiveJob;
use semio_framework_plugin::component::ArtifactReservedJob;

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
    crate::puzzle_job::JobTurn::Fault(bytes[..bytes.len().min(semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES)].to_vec())
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
    let handles = puzzle2d_import_presentation(row, "handles").and_then(Value::as_array).map_or_else(semio_framework_value_persistent::list::PagedList::new, |templates| templates.iter().enumerate().filter_map(|(index, template)| puzzle2d_import_handle_template(id, index, template)).collect());
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
fn puzzle2d_upsert_catalog_row<T: PartialEq>(rows: &mut semio_framework_value_persistent::list::PagedList<T, { usize::MAX }>, incoming: T, matches: impl Fn(&T) -> bool) -> bool {
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
    pub fragment: Option<Value>,
    pub catalogs: crate::standards::v1::subsets::any::schema::records::Puzzle2dKindCatalogs,
    compatibility: Vec<crate::standards::v1::subsets::any::schema::records::Puzzle2dKindCompatibility>,
    pub mutations: Vec<Puzzle2dMutation>,
    pub stage: Puzzle2dImportStage,
    pub cursor: usize,
    decoded_items: usize,
    pub progress: u64,
    catalog_changed: bool,
    pending_completion_rejection: Option<ArtifactToolCompletionRejection<EditorApp<Puzzle2dPlayApp>>>,
    completed: bool,
    outbox: crate::puzzle_job::JobOutbox,
    closing: bool,
    staged: bool,
    pub owners: crate::puzzle_job::WorkClosing<Puzzle2dImportOwners>,
}

/// ♻️ The owners one `Puzzle2dImportJob` still holds when it closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
struct Puzzle2dImportOwners {
    pub fragment: Option<Value>,
    pub catalogs: crate::standards::v1::subsets::any::schema::records::Puzzle2dKindCatalogs,
    compatibility: Vec<crate::standards::v1::subsets::any::schema::records::Puzzle2dKindCompatibility>,
    pub mutations: Vec<Puzzle2dMutation>,
    pending_completion_rejection: Option<ArtifactToolCompletionRejection<EditorApp<Puzzle2dPlayApp>>>,
}

impl Default for Puzzle2dImportJob {
    fn default() -> Self {
        Self {
            fragment: None,
            catalogs: crate::standards::v1::subsets::any::schema::records::Puzzle2dKindCatalogs::default(),
            compatibility: Vec::new(),
            mutations: Vec::new(),
            stage: Puzzle2dImportStage::Decode,
            cursor: 0,
            decoded_items: 0,
            progress: 0,
            catalog_changed: false,
            pending_completion_rejection: None,
            completed: false,
            outbox: Default::default(),
            closing: false,
            staged: false,
            owners: Default::default(),
        }
    }

}

impl Puzzle2dImportJob {
    pub fn checkpoint(&self) -> crate::puzzle_job::JobTurn {
        puzzle2d_import_checkpoint(self.stage as u8, self.cursor, self.decoded_items, self.progress)
    }

    pub fn rows(&self, key: &str) -> &[Value] {
        self.fragment.as_ref().and_then(|fragment| fragment.get(key)).and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[])
    }

    pub fn meta<'a>(&self,request:&'a ArtifactReservedToolJobRequest<EditorApp<Puzzle2dPlayApp>>,key:&str)->Option<&'a Value>{
        request.snapshot.value().get("meta").and_then(|meta|meta.get(key)).filter(|value|!value.is_null())
    }

    fn push_mutation(&mut self, mutation: Puzzle2dMutation) -> Result<(), &'static str> {
        if self.mutations.len() >= PUZZLE2D_IMPORT_MUTATION_ITEMS {
            return Err("puzzle2d kit:in mutation limit exceeded");
        }
        self.mutations.push(mutation);
        Ok(())
    }

    pub fn decode(&mut self,request:&ArtifactReservedToolJobRequest<EditorApp<Puzzle2dPlayApp>>)->Option<crate::puzzle_job::JobTurn>{
        let Some(ArtifactReservedToolInput::Media{port,media})=request.input.as_ref()else{return Some(puzzle2d_job_fault("puzzle2d import lost its original media owner"))};
        if port != PUZZLE2D_IMPORT_PORT {
            return Some(puzzle2d_job_fault("puzzle2d import only implements kit:in"));
        }
        let encoded;
        let media_json=match &media.payload{
            MediaPayload::Structured{json,..}=>json.as_str(),
            MediaPayload::Intrinsic{value,..}=>{encoded=semio_framework_pack_json::to_json_string(value);encoded.as_str()},
            MediaPayload::Binary{..}=>return Some(puzzle2d_job_fault("puzzle2d kit:in requires a structured payload")),
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
        let existing_catalogs = self.meta(request,"kindCatalogs").cloned();
        let existing_compatibility = self.meta(request,"kindCompatibility").and_then(|value| value.as_array().cloned()).unwrap_or_default();
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
    pub fn turn(&mut self, request:&ArtifactReservedToolJobRequest<EditorApp<Puzzle2dPlayApp>>, cx: &mut StepContext<'_>) -> crate::puzzle_job::JobTurn {
        if self.closing || cx.is_cancelled() {
            return crate::puzzle_job::JobTurn::Cancelled;
        }
        if self.pending_completion_rejection.is_some() {
            return puzzle2d_job_fault("puzzle2d import completion remains rejected");
        }
        match self.stage {
            Puzzle2dImportStage::Decode => {
                cx.set_stage("puzzle2d-import-decode");
                if let Some(outcome) = self.decode(request) {
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
                    let completion=&request.completion;
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
    fn step_value<'a>(&'a mut self, request:&ArtifactReservedToolJobRequest<EditorApp<Puzzle2dPlayApp>>, cx: &mut StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        match self.outbox.phase(cx)? {
            crate::puzzle_job::OutboxPhase::Building => return self.outbox.advance(cx).map_err(semio_framework_job::OriginalTurnFailure::from),
            crate::puzzle_job::OutboxPhase::Delivered | crate::puzzle_job::OutboxPhase::Retiring => {
                self.outbox.retire_step(cx)?;
                return Ok(None);
            }
            crate::puzzle_job::OutboxPhase::Idle => {}
        }
        let turn = if cx.fuel_exhausted() || cx.deadline_exceeded() { crate::puzzle_job::JobTurn::Yield } else { self.turn(request,cx) };
        self.outbox.settle(turn, cx).map_err(semio_framework_job::OriginalTurnFailure::from)
    }
}

impl semio_framework_plugin::tool_contract::ArtifactReservedWork<EditorApp<Puzzle2dPlayApp>> for Puzzle2dImportJob {
    fn step<'a>(&'a mut self,request:&'a mut ArtifactReservedToolJobRequest<EditorApp<Puzzle2dPlayApp>>,cx:&mut StepContext<'_>,_:&mut semio_framework_job::OriginalJobReturnClaim<'_>)->semio_framework_job::InteractiveJobStep<'a>{semio_framework_job::InteractiveJobStep::from_callback(self.step_value(request,cx))}

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        self.outbox.borrow_outcome(descriptor)
    }

    fn begin_close(&mut self){self.closing=true;}
    fn close_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
        if !self.staged{return Ok(semio_framework_value::RetirementDemand{copy_bytes:std::mem::size_of::<Puzzle2dImportOwners>()+std::mem::size_of::<Option<Puzzle2dImportOwners>>()+std::mem::size_of::<bool>(),depth:1,..Default::default()})}
        crate::puzzle_job::job_close_demands(&self.outbox,&self.owners,body)
    }
    fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        self.closing=true;
        if !self.staged{
            let demand=self.close_demands(cx.retained_grant().maximum_copy_bytes)?;
            return semio_framework_job::close_original_retained_step(cx,demand,|_|{
                self.owners.stage(Puzzle2dImportOwners{fragment:self.fragment.take(),catalogs:std::mem::take(&mut self.catalogs),compatibility:std::mem::take(&mut self.compatibility),mutations:std::mem::take(&mut self.mutations),pending_completion_rejection:self.pending_completion_rejection.take()});self.staged=true;
                semio_framework_value::RetainedWorkOutcome{result:Ok(semio_framework_value::RetainedCloneStep::Progress(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work:1}
            })
        }
        semio_framework_job::close_original_nested_step(cx,|cx,_|crate::puzzle_job::job_close_step(&mut self.outbox,&mut self.owners,cx))
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.staged && self.outbox.terminal_is_empty() && self.owners.is_empty()
    }
}


/// 📋️ One-step reserved copy/cut/paste job — the framework route is an empty stub unless the app owns
/// this producer, so puzzle2d owning it is what makes `mod+c`/`mod+x`/`mod+v` and the context-menu
/// clipboard rows do anything at all. Every arm answers in one step: a cut is copy-plus-delete as ONE
/// artifact edit, and a paste is one insert whose clones the app re-selects.
pub struct Puzzle2dClipboardJob{
    rejection:Option<ArtifactToolCompletionRejection<EditorApp<Puzzle2dPlayApp>>>,
    fault:Option<Vec<u8>>,
    outbox:crate::puzzle_job::JobOutbox,
    closing:bool,
    staged:bool,
    completed:bool,
    owners:crate::puzzle_job::WorkClosing<Puzzle2dClipboardOwners>,
}
#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
struct Puzzle2dClipboardOwners{rejection:Option<ArtifactToolCompletionRejection<EditorApp<Puzzle2dPlayApp>>>,fault:Option<Vec<u8>>}
impl Default for Puzzle2dClipboardJob{
    fn default()->Self{Self{rejection:None,fault:None,outbox:Default::default(),closing:false,staged:false,completed:false,owners:Default::default()}}
}
impl Puzzle2dClipboardJob{
    pub fn emit(request:&ArtifactReservedToolJobRequest<EditorApp<Puzzle2dPlayApp>>) -> Result<Emit<Puzzle2dMutation, Puzzle2dConfigMutation, NoDraftMutation>, Fault> {
        let Some(ArtifactReservedToolInput::Action{args,interaction})=request.input.as_ref()else{return Ok(Emit::default())};
        let args=args.as_ref();let owned=request.snapshot.as_ref();let snapshot=owned.value();let state=interaction.get();
        let selected=match state.selection.get(crate::editor::puzzle2d::snapshot::PUZZLE2D_INTERACTION_DOMAIN).filter(|selection|!selection.ids.is_empty()){
            Some(selection)=>puzzle2d_selected_node_ids(snapshot,&selection.ids),
            None=>puzzle2d_selected_node_ids(snapshot,state.selection.values().flat_map(|selection|selection.ids.iter())),
        };
        Ok(match request.tool_id.as_str() {
            "copy" => match puzzle2d_copy_fragment_from(snapshot, &selected) {
                Ok(fragment) => Emit { effects: vec![Effect::ClipboardWrite { fragment }], ui_scope: UiDirtyScope::None, ..Default::default() },
                Err(_) => Emit::default(),
            },
            "cut" => {
                let Ok(fragment) = puzzle2d_copy_fragment_from(snapshot, &selected) else { return Ok(Emit::default()) };
                if puzzle2d_selection_is_locked(snapshot, &selected) {
                    let labels=request.ui_axes.map(|(locale,terminology)|crate::editor::puzzle2d::terminology::puzzle2d_labels_for_axes(locale,terminology)).ok_or_else(||Fault::from("puzzle2d clipboard notification requires the caller's explicit presentation axes"))?;
                    return Ok(Emit { effects: vec![Effect::Notify { message: labels.cut_locked.as_str().to_string() }], ui_scope: UiDirtyScope::None, ..Default::default() });
                }
                let mutations = puzzle2d_cut_operations_from(owned, &selected);
                let clear = puzzle2d_clear_selection_write(snapshot, &selected).into_iter().collect();
                Emit { artifact_mutations: mutations, effects: vec![Effect::ClipboardWrite { fragment }], interaction_writes: clear, ..Default::default() }
            }
            "paste" => {
                let Some(fragment) = args.as_ref().and_then(|value| value.get("fragment")).and_then(|value| semio_framework_value::FromValue::from_value(value.clone()).ok()) else {
                    return Ok(Emit::default());
                };
                let placement = PastePlacement::default();
                match puzzle2d_paste_operations_on(owned, &fragment, &placement) {
                    Ok((mutations, pasted)) => Emit { artifact_mutations: mutations, interaction_writes: vec![puzzle2d_selection_write(snapshot, &pasted)], ..Default::default() },
                    Err(_) => Emit::default(),
                }
            }
            _ => Emit::default(),
        })
    }
    fn turn(&mut self,request:&mut ArtifactReservedToolJobRequest<EditorApp<Puzzle2dPlayApp>>,cx:&mut StepContext<'_>)->Result<crate::puzzle_job::JobTurn,semio_framework_job::OriginalTurnFailure>{
        use crate::puzzle_job::JobTurn;
        use semio_framework_value::{RetainedCloneProgress,RetirementDemand};
        if self.closing||cx.is_cancelled(){return Ok(JobTurn::Cancelled)}
        if cx.should_yield(){return Ok(JobTurn::Yield)}
        if self.fault.is_some(){
            let bytes=2*std::mem::size_of::<Vec<u8>>();if !cx.retained_work_can_enter(RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()}){return Ok(JobTurn::Yield)}
            let source=self.fault.take().unwrap();cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(JobTurn::Fault(source))
        }
        if self.rejection.is_some(){
            let text=b"puzzle2d-clipboard-completion-rejected";
            let bytes=text.len()+2*std::mem::size_of::<Vec<u8>>();
            semio_framework_job::admit_original_construction(cx,&mut self.fault,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:bytes,capacity_bytes:text.len(),release_bytes:0,depth:1},|_|{
                let source=text.to_vec();let capacity=source.capacity();semio_framework_value::RetainedWorkOutcome{result:Ok((source,RetainedCloneProgress{copied_items:1,copied_bytes:bytes,retained_capacity_bytes:capacity,..Default::default()})),work:1}
            })?;
            return Ok(JobTurn::Yield)
        }
        if !request.raw_wire.is_empty(){
            let bytes=2*std::mem::size_of::<Vec<u8>>();if !cx.retained_work_can_enter(RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()}){return Ok(JobTurn::Yield)}
            let source=std::mem::take(&mut request.raw_wire);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(JobTurn::Prepare(source))
        }
        if self.completed{return Ok(JobTurn::Complete)}
        let emit=Self::emit(request);cx.consume_fuel(1);
        if let Err(rejected)=request.completion.complete(emit,EphemeralEmit::default()){self.rejection=Some(rejected);return Ok(JobTurn::Yield)}
        self.completed=true;Ok(JobTurn::Complete)
    }
    fn step_value<'a>(&'a mut self,request:&mut ArtifactReservedToolJobRequest<EditorApp<Puzzle2dPlayApp>>,cx:&mut StepContext<'_>)->Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure>{
        match self.outbox.phase(cx)?{
            crate::puzzle_job::OutboxPhase::Building=>return self.outbox.advance(cx).map_err(Into::into),
            crate::puzzle_job::OutboxPhase::Delivered|crate::puzzle_job::OutboxPhase::Retiring=>{self.outbox.retire_step(cx)?;return Ok(None)}
            crate::puzzle_job::OutboxPhase::Idle=>{}
        }
        let turn=self.turn(request,cx)?;self.outbox.settle(turn,cx).map_err(Into::into)
    }
}
impl semio_framework_plugin::tool_contract::ArtifactReservedWork<EditorApp<Puzzle2dPlayApp>> for Puzzle2dClipboardJob{
    fn step<'a>(&'a mut self,request:&'a mut ArtifactReservedToolJobRequest<EditorApp<Puzzle2dPlayApp>>,cx:&mut StepContext<'_>,_:&mut semio_framework_job::OriginalJobReturnClaim<'_>)->semio_framework_job::InteractiveJobStep<'a>{semio_framework_job::InteractiveJobStep::from_callback(self.step_value(request,cx))}
    fn borrow_outcome<'a>(&'a self,descriptor:&'a semio_framework_job::JobOutcomeDescriptor)->Result<semio_framework_job::JobOutcomeView<'a>,semio_framework_value::ValueError>{self.outbox.borrow_outcome(descriptor)}
    fn begin_close(&mut self){self.closing=true;}
    fn close_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
        if !self.staged{return Ok(semio_framework_value::RetirementDemand{copy_bytes:std::mem::size_of::<Puzzle2dClipboardOwners>()+std::mem::size_of::<Option<Puzzle2dClipboardOwners>>()+std::mem::size_of::<bool>(),depth:1,..Default::default()})}
        crate::puzzle_job::job_close_demands(&self.outbox,&self.owners,body)
    }
    fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        self.closing=true;if !self.staged{
            let demand=self.close_demands(cx.retained_grant().maximum_copy_bytes)?;
            return semio_framework_job::close_original_retained_step(cx,demand,|_|{
                self.owners.stage(Puzzle2dClipboardOwners{rejection:self.rejection.take(),fault:self.fault.take()});self.staged=true;
                semio_framework_value::RetainedWorkOutcome{result:Ok(semio_framework_value::RetainedCloneStep::Progress(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work:1}
            })
        }
        semio_framework_job::close_original_nested_step(cx,|cx,_|crate::puzzle_job::job_close_step(&mut self.outbox,&mut self.owners,cx))
    }
    fn terminal_is_empty(&self)->bool{self.closing&&self.staged&&self.outbox.terminal_is_empty()&&self.owners.is_empty()}
}
//#endregion 🧵️ReservedJobs
//#endregion 🔖️PlayApp
