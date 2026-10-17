//! 📥️ Import job of the puzzle 5d editor session: the reserved snapshot import job with its fixed-page state machine. See [`super`].
use semio_framework_job::InteractiveJob;
use semio_framework_plugin::ArtifactReservedJob;

use crate::editor::puzzle5d::snapshot::Puzzle5dPlaySnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use semio_framework_plugin::ArtifactReservedToolJobRequest;
use semio_framework_plugin::ArtifactToolCompletion;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::EphemeralEmit;
use semio_framework_plugin::Media;
use semio_framework_value::FromValue;
use semio_framework_pack_json::{parse, Value};
use crate::editor::puzzle5d::session::support::{puzzle5d_import_checkpoint, puzzle5d_job_fault, puzzle5d_step_envelope};
use crate::editor::puzzle5d::Puzzle5dPlayApp;
use crate::editor::puzzle5d::entities::{PUZZLE5D_IMPORT_DECODED_ITEMS, PUZZLE5D_IMPORT_MEDIA_BYTES, PUZZLE5D_IMPORT_MUTATIONS_PER_PAGE, PUZZLE5D_IMPORT_MUTATION_ITEMS, PUZZLE5D_IMPORT_MUTATION_PAGES, PUZZLE5D_IMPORT_SEMANTIC_ITEMS, PUZZLE5D_RESERVED_PAGE_BYTES, puzzle5d_editor_projection};
use semio_framework_job::StepContext;

//#region 🧵️ReservedJobs
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dImportStage {
    Envelope,
    Decode,
    CensusParts,
    ReserveCatalogParts,
    ReserveCatalogGrips,
    ReserveCatalogFasteners,
    ReserveCatalogRopes,
    ReserveCompatibility,
    ReservePartIndex,
    ReserveGripIndex,
    ReserveCompatibilityIndex,
    ReserveMutations,
    LoadCatalogParts,
    LoadCatalogGrips,
    LoadCatalogFasteners,
    LoadCatalogRopes,
    LoadCompatibility,
    IndexParts,
    IndexGrips,
    IndexCompatibility,
    Parts,
    PartReserve,
    PartVortices,
    PartPublish,
    Grips,
    Compatibility,
    CatalogMutation,
    Complete,
}

/// ♻️ The owners one `Puzzle5dImportJob` still holds when it closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dImportOwners {
    pub raw: Vec<u8>,
    pub port: String,
    media_json: Option<String>,
    pub snapshot: Option<std::sync::Arc<Puzzle5dPlaySnapshot>>,
    pub fragment: Option<Value>,
    pub catalogs: crate::standards::v1::subsets::any::schema::document_records::Puzzle5dKindCatalogs,
    pub compatibility: Vec<crate::standards::v1::subsets::any::schema::document_records::Puzzle5dKindCompatibility>,
    part_index: Vec<(String, usize)>,
    grip_index: Vec<(String, usize)>,
    compatibility_index: Vec<((String, String), usize)>,
    mutation_pages: [Vec<Puzzle5dMutation>; PUZZLE5D_IMPORT_MUTATION_PAGES],
    current_part: Option<crate::standards::v1::subsets::any::schema::document_records::Puzzle5dCatalogPartKind>,
    pub completion: Option<ArtifactToolCompletion<EditorApp<Puzzle5dPlayApp>>>,
    pub pending_completion_rejection: Option<semio_framework_plugin::app::ArtifactToolCompletionRejection<EditorApp<Puzzle5dPlayApp>>>,
}

pub struct Puzzle5dImportJob {
    pub raw: Vec<u8>,
    pub raw_cursor: usize,
    pub raw_page: [u8; PUZZLE5D_RESERVED_PAGE_BYTES],
    pub raw_page_len: usize,
    pub progress: u64,
    pub stage: Puzzle5dImportStage,
    pub port: String,
    media_json: Option<String>,
    pub snapshot: Option<std::sync::Arc<Puzzle5dPlaySnapshot>>,
    pub fragment: Option<Value>,
    pub catalogs: crate::standards::v1::subsets::any::schema::document_records::Puzzle5dKindCatalogs,
    had_catalogs: bool,
    catalog_changed: bool,
    pub compatibility: Vec<crate::standards::v1::subsets::any::schema::document_records::Puzzle5dKindCompatibility>,
    part_index: Vec<(String, usize)>,
    grip_index: Vec<(String, usize)>,
    compatibility_index: Vec<((String, String), usize)>,
    mutation_pages: [Vec<Puzzle5dMutation>; PUZZLE5D_IMPORT_MUTATION_PAGES],
    pub cursor: usize,
    pub nested_cursor: usize,
    pub decoded_items: usize,
    current_part: Option<crate::standards::v1::subsets::any::schema::document_records::Puzzle5dCatalogPartKind>,
    pub completion: Option<ArtifactToolCompletion<EditorApp<Puzzle5dPlayApp>>>,
    pub pending_completion_rejection: Option<semio_framework_plugin::app::ArtifactToolCompletionRejection<EditorApp<Puzzle5dPlayApp>>>,
    pub completed: bool,
    pub outbox: crate::puzzle_job::JobOutbox,
    pub closing: bool,
    pub owners: crate::puzzle_job::WorkClosing<Puzzle5dImportOwners>,
}

fn puzzle5d_import_vec3(value: Option<&Value>) -> Result<[f64; 3], &'static str> {
    let values = value.and_then(Value::as_array).ok_or("puzzle5d kit:in vector is not an array")?;
    if values.len() != 3 {
        return Err("puzzle5d kit:in vector must contain exactly three coordinates");
    }
    let mut result = [0.0; 3];
    for (index, target) in result.iter_mut().enumerate() {
        *target = values.get(index).and_then(Value::as_f64).filter(|value| value.is_finite()).ok_or("puzzle5d kit:in vector contains a non-finite coordinate")?;
    }
    Ok(result)
}

fn puzzle5d_import_keys_are(value: &Value, allowed: &[&str]) -> bool {
    value.as_object().is_some_and(|object| object.iter().all(|(key, _)| allowed.contains(&key)))
}

fn puzzle5d_decode_import_fragment(media_json: &str) -> Result<Value, String> {
    if media_json.len() > PUZZLE5D_IMPORT_MEDIA_BYTES {
        return Err("puzzle5d kit:in payload exceeds its predecode cap".into());
    }
    parse(media_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

impl Puzzle5dImportJob {
    pub fn new(request: ArtifactReservedToolJobRequest<EditorApp<Puzzle5dPlayApp>>, port: String, media: Media) -> Self {
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

    pub fn checkpoint(&self) -> crate::puzzle_job::JobTurn {
        puzzle5d_import_checkpoint(self.stage as u8, self.cursor, self.nested_cursor, self.decoded_items, self.progress)
    }

    pub fn rows(&self, key: &str) -> &[Value] {
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
    pub fn turn(&mut self, cx: &mut StepContext<'_>) -> crate::puzzle_job::JobTurn {
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
                    let parsed = match <crate::standards::v1::subsets::any::schema::document_records::Puzzle5dCatalogPartKind as FromValue>::from_value(semio_framework_pack_json::to_dsl_value(row)) {
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
                    let parsed = match <crate::standards::v1::subsets::any::schema::document_records::Puzzle5dCatalogGripKind as FromValue>::from_value(semio_framework_pack_json::to_dsl_value(row)) {
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
                    let parsed = match <crate::standards::v1::subsets::any::schema::document_records::Puzzle5dCatalogFastenerKind as FromValue>::from_value(semio_framework_pack_json::to_dsl_value(row)) {
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
                    let parsed = match <crate::standards::v1::subsets::any::schema::document_records::Puzzle5dCatalogRopeKind as FromValue>::from_value(semio_framework_pack_json::to_dsl_value(row)) {
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
                    let parsed = match <crate::standards::v1::subsets::any::schema::document_records::Puzzle5dKindCompatibility as FromValue>::from_value(semio_framework_pack_json::to_dsl_value(row)) {
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
                    self.current_part = Some(crate::standards::v1::subsets::any::schema::document_records::Puzzle5dCatalogPartKind {
                        id,
                        name,
                        label,
                        representations: mesh_url.map(|url| vec![crate::standards::v1::subsets::any::schema::document_records::Puzzle5dRepresentation { id: "mesh".into(), name: "mesh".into(), url, mime: "model/gltf-binary".into(), ..Default::default() }]).unwrap_or_default(),
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
                    part.grips.push(crate::standards::v1::subsets::any::schema::document_records::Puzzle5dGripTemplate {
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
                    let next = crate::standards::v1::subsets::any::schema::document_records::Puzzle5dCatalogGripKind {
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
                    let parsed = match <crate::standards::v1::subsets::any::schema::document_records::Puzzle5dKindCompatibility as FromValue>::from_value(semio_framework_pack_json::to_dsl_value(row)) {
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

impl InteractiveJob for Puzzle5dImportJob {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
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

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        self.outbox.borrow_outcome(descriptor)
    }

    fn begin_close(&mut self) {
        if std::mem::replace(&mut self.closing, true) {
            return;
        }
        self.owners.stage(Puzzle5dImportOwners {
            raw: std::mem::take(&mut self.raw),
            port: std::mem::take(&mut self.port),
            media_json: self.media_json.take(),
            snapshot: self.snapshot.take(),
            fragment: self.fragment.take(),
            catalogs: std::mem::take(&mut self.catalogs),
            compatibility: std::mem::take(&mut self.compatibility),
            part_index: std::mem::take(&mut self.part_index),
            grip_index: std::mem::take(&mut self.grip_index),
            compatibility_index: std::mem::take(&mut self.compatibility_index),
            mutation_pages: std::mem::take(&mut self.mutation_pages),
            current_part: self.current_part.take(),
            completion: self.completion.take(),
            pending_completion_rejection: self.pending_completion_rejection.take(),
        });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.begin_close();
        crate::puzzle_job::job_close_step(&mut self.outbox, &mut self.owners, grant)
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

impl ArtifactReservedJob for Puzzle5dImportJob {}
//#endregion 🧵️ReservedJobs
