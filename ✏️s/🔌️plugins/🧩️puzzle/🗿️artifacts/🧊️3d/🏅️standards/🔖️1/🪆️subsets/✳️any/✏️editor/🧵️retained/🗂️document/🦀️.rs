//! 🗂️ Puzzle 3d retained document works — the staged edits that read the document before they emit: kind weights,
//! object kinds, the inspector patch, attraction creation and the active example switch. Every work is written against
//! the [`Puzzle3dEditor`] contract, not the concrete editor.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

use {semio_s_artifact_puzzle_3d_editor_snapshot::CONCRETE_FOREST_EXAMPLE_SNAPSHOT, semio_s_artifact_puzzle_3d_editor_snapshot::EMPTY_EXAMPLE_SNAPSHOT, semio_s_artifact_puzzle_3d_editor_snapshot::NAKAGIN_EXAMPLE_SNAPSHOT, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_ATTRACTION, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_OBJECT, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_REFERENCE, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_TARGET_VOLUME, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_VORTEX, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_ID_COUNTER, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_INTERACTION_DOMAIN, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dAttraction, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dObject, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dReference, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dSceneSnapshot, semio_s_artifact_puzzle_3d_editor_dispatch::Puzzle3dScopeClass, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dTargetVolume, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dVortex, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_apply_distribution_weight, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_axis_index, semio_s_artifact_puzzle_3d_editor_dispatch::puzzle3d_command_scope_class, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_next_object_label, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_object_joint_total, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_resolve_number_edit, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_scene_snapshot_from_document, semio_s_artifact_puzzle_3d_editor_dispatch::puzzle3d_scope, semio_s_artifact_puzzle_3d_editor_snapshot::value_as_vec3};
use {semio_s_artifact_puzzle_3d_editor_config::PUZZLE3D_EXAMPLE_CONCRETE_FOREST, semio_s_artifact_puzzle_3d_editor_config::PUZZLE3D_EXAMPLE_NAKAGIN};
use semio_s_artifact_puzzle_3d_commands_document::delete_target_volume;
use {semio_s_artifact_puzzle_3d_editor_config::Puzzle3dConfig, semio_s_artifact_puzzle_3d_editor_config::Puzzle3dConfigMutation, semio_s_artifact_puzzle_3d_editor_config::Puzzle3dConfigSetObjectKindWeights, semio_s_artifact_puzzle_3d_editor_config::Puzzle3dConfigSetVortexKindWeights, semio_s_artifact_puzzle_3d_editor_config::Puzzle3dConfigSetActiveExampleId};
use semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dPlaySnapshot;
use semio_s_artifact_puzzle_3d_mutations::Puzzle3dMutation;
use {semio_framework_job::InteractiveJobCloseStep, semio_framework_job::StepContext};
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::InteractionWrite;
use semio_framework_value::FromValue;
use {semio_framework_pack_json::object, semio_framework_pack_json::to_string, semio_framework_pack_json::Value};
use {std::collections::HashMap, std::collections::HashSet};
use std::sync::atomic::Ordering;
use semio_s_artifact_puzzle_3d_model::puzzle3d_vortex_full_id;
use semio_s_artifact_puzzle_3d_model::derive_attraction_params;
use semio_s_artifact_puzzle_3d_model::quat_normalize;
use semio_s_artifact_puzzle_3d_editor_snapshot as snapshot;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle3dKindWeightStage {
    Catalog,
    Validate,
    SumOthers,
    Changed,
    Build,
    Publish,
    Complete,
    Closing,
}

/// ♻️ The owners one `Puzzle3dKindWeightWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle3dKindWeightWorkOwners {
    ids: Vec<String>,
    result: HashMap<String, f64>,
    changed_id: Option<String>,
}

pub struct Puzzle3dKindWeightWork {
    tool_id: &'static str,
    stage: Puzzle3dKindWeightStage,
    cursor: usize,
    ids: Vec<String>,
    result: HashMap<String, f64>,
    missing: bool,
    base_sum: f64,
    other_sum: f64,
    other_count: usize,
    changed_id: Option<String>,
    requested: f64,
    ignored: bool,
    close_owners: semio_s_plugin_puzzle_jobs::WorkClosing<Puzzle3dKindWeightWorkOwners>,
}

impl Puzzle3dKindWeightWork {
    fn new(tool_id: &'static str) -> Self {
        Self { close_owners: Default::default(),
            tool_id,
            stage: Puzzle3dKindWeightStage::Catalog,
            cursor: 0,
            ids: Vec::with_capacity(semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_DECODED_ITEMS),
            result: HashMap::with_capacity(semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_DECODED_ITEMS),
            missing: false,
            base_sum: 0.0,
            other_sum: 0.0,
            other_count: 0,
            changed_id: None,
            requested: 1.0,
            ignored: false,
        }
    }

    fn weights<'a>(&self, config: &'a Puzzle3dConfig) -> &'a HashMap<String, f64> {
        if self.tool_id == "setObjectKindWeight" {
            &config.object_kind_weights
        } else {
            &config.vortex_kind_weights
        }
    }

    fn base_weight(&self, config: &Puzzle3dConfig, id: &str) -> f64 {
        if self.ids.is_empty() {
            return 0.0;
        }
        if self.missing || self.weights(config).is_empty() {
            return 1.0 / self.ids.len() as f64;
        }
        let value = self.weights(config).get(id).copied().unwrap_or(0.0);
        if (self.base_sum - 1.0).abs() > 0.001 && self.base_sum.abs() > f64::EPSILON {
            value / self.base_sum
        } else {
            value
        }
    }

}

impl<A: Puzzle3dEditor> semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>> for Puzzle3dKindWeightWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(&self, _command: &Puzzle3dCommand, snapshot: &Puzzle3dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<A>>>) -> Option<usize> {
        let catalogs = snapshot.typed().meta.kind_catalogs.as_ref()?;
        let count = if self.tool_id == "setObjectKindWeight" { catalogs.objects.len() } else { catalogs.vortices.len() };
        let items = count.checked_mul(4)?.checked_add(3)?;
        (count <= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_DECODED_ITEMS && items <= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        semio_s_plugin_puzzle_retained::step_demands::<EditorApp<A>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config, interaction: _interaction, hover: _hover, context: _context, .. } = *input;
        match self.stage {
            Puzzle3dKindWeightStage::Catalog => {
                let catalogs = snapshot.typed().meta.kind_catalogs.as_ref().ok_or_else(|| Fault::from("puzzle3d-kind-weight-catalog-owner"))?;
                let id = if self.tool_id == "setObjectKindWeight" { catalogs.objects.get(self.cursor).map(|entry| entry.id.as_str()) } else { catalogs.vortices.get(self.cursor).map(|entry| entry.id.as_str()) };
                if let Some(id) = id {
                    if self.ids.len() >= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_DECODED_ITEMS {
                        return Err(Fault::from("puzzle3d-kind-weight-catalog-capacity"));
                    }
                    self.ids.push(id.to_string());
                    self.cursor += 1;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-kind-weight-catalog", "Reading kind owner", "Artinhaber wird gelesen"));
                }
                self.changed_id = Some(command.args().and_then(|args| args.get("kindId")).and_then(Value::as_str).unwrap_or("").to_string());
                let requested = command.args().and_then(|args| args.get("value")).and_then(Value::as_f64).unwrap_or(1.0).clamp(0.0, 1.0);
                self.requested = if self.tool_id == "setVortexKindWeight" {
                    if let Some(object_kind_id) = command.args().and_then(|args| args.get("objectKindId")).and_then(Value::as_str) {
                        let vortex_ids = self.ids.clone();
                        let object_weight = puzzle3d_object_joint_total(&config.vortex_kind_weights, &config.object_kind_weights, object_kind_id, &vortex_ids);
                        if object_weight <= f64::EPSILON {
                            self.ignored = true;
                            self.stage = Puzzle3dKindWeightStage::Publish;
                            return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-kind-weight-publish", "Ignoring zero-weight child", "Kind mit Nullgewicht wird ignoriert"));
                        }
                        requested
                    } else {
                        requested
                    }
                } else {
                    requested
                };
                self.cursor = 0;
                self.stage = Puzzle3dKindWeightStage::Validate;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-kind-weight-validate", "Validating current weights", "Aktuelle Gewichte werden geprüft"))
            }
            Puzzle3dKindWeightStage::Validate => {
                let Some(id) = self.ids.get(self.cursor) else {
                    self.cursor = 0;
                    self.stage = Puzzle3dKindWeightStage::SumOthers;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-kind-weight-sum", "Measuring sibling weights", "Geschwistergewichte werden gemessen"));
                };
                let weights = self.weights(config);
                self.missing |= !weights.contains_key(id);
                self.base_sum += weights.get(id).copied().unwrap_or(0.0);
                self.cursor += 1;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-kind-weight-validate", "Validating kind weight", "Artgewicht wird geprüft"))
            }
            Puzzle3dKindWeightStage::SumOthers => {
                let Some(id) = self.ids.get(self.cursor) else {
                    self.cursor = 0;
                    self.stage = Puzzle3dKindWeightStage::Changed;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-kind-weight-changed", "Preparing changed weight", "Geändertes Gewicht wird vorbereitet"));
                };
                if self.changed_id.as_deref() != Some(id.as_str()) {
                    self.other_sum += self.base_weight(config, id);
                    self.other_count += 1;
                }
                self.cursor += 1;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-kind-weight-sum", "Measuring sibling weight", "Geschwistergewicht wird gemessen"))
            }
            Puzzle3dKindWeightStage::Changed => {
                if self.ids.len() >= 2 {
                    let changed_id = self.changed_id.clone().ok_or_else(|| Fault::from("puzzle3d-kind-weight-changed-owner"))?;
                    self.result.insert(changed_id, self.requested);
                }
                self.stage = Puzzle3dKindWeightStage::Build;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-kind-weight-build", "Building normalized weights", "Normalisierte Gewichte werden aufgebaut"))
            }
            Puzzle3dKindWeightStage::Build => {
                let Some(id) = self.ids.get(self.cursor).cloned() else {
                    self.stage = Puzzle3dKindWeightStage::Publish;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-kind-weight-publish", "Preparing weight publication", "Gewichtsveröffentlichung wird vorbereitet"));
                };
                self.cursor += 1;
                let value = if self.ids.len() == 1 {
                    1.0
                } else if self.changed_id.as_deref() == Some(id.as_str()) {
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-kind-weight-build", "Keeping changed weight", "Geändertes Gewicht wird beibehalten"));
                } else {
                    let remainder = (1.0 - self.requested).max(0.0);
                    if self.other_sum <= f64::EPSILON {
                        remainder / self.other_count.max(1) as f64
                    } else {
                        self.base_weight(config, &id) / self.other_sum * remainder
                    }
                };
                self.result.insert(id, value);
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-kind-weight-build", "Building kind weight", "Artgewicht wird aufgebaut"))
            }
            Puzzle3dKindWeightStage::Publish => {
                self.stage = Puzzle3dKindWeightStage::Complete;
                if self.ignored {
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit::default()));
                }
                let catalogs = snapshot.typed().meta.kind_catalogs.as_ref().ok_or_else(|| Fault::from("puzzle3d-kind-weight-catalog-owner"))?;
                let object_ids: Vec<String> = catalogs.objects.iter().map(|entry| entry.id.clone()).collect();
                let vortex_ids: Vec<String> = catalogs.vortices.iter().map(|entry| entry.id.clone()).collect();
                let kind_id = self.changed_id.as_deref().unwrap_or("");
                let object_kind_id = command.args().and_then(|args| args.get("objectKindId")).and_then(Value::as_str);
                let mut object_weights = config.object_kind_weights.clone();
                let mut joint_weights = config.vortex_kind_weights.clone();
                puzzle3d_apply_distribution_weight(&mut object_weights, &mut joint_weights, &object_ids, &vortex_ids, self.tool_id, kind_id, object_kind_id, self.requested);
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit {
                    config_mutations: vec![
                        Puzzle3dConfigMutation::SetObjectKindWeights(Puzzle3dConfigSetObjectKindWeights{ value: object_weights }),
                        Puzzle3dConfigMutation::SetVortexKindWeights(Puzzle3dConfigSetVortexKindWeights{ value: joint_weights }),
                    ],
                    ui_scope: puzzle3d_scope(puzzle3d_command_scope_class(self.tool_id)),
                    ..Default::default()
                }))
            }
            Puzzle3dKindWeightStage::Complete => Err(Fault::from("puzzle3d-kind-weight-complete-repolled")),
            Puzzle3dKindWeightStage::Closing => Err(Fault::from("puzzle3d-kind-weight-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle3dKindWeightStage::Closing;
        self.close_owners.stage(Puzzle3dKindWeightWorkOwners { ids: std::mem::take(&mut self.ids), result: std::mem::take(&mut self.result), changed_id: std::mem::take(&mut self.changed_id) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
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
        self.stage == Puzzle3dKindWeightStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle3dAddObjectKindStage {
    Decode,
    Catalog,
    Kind,
    Representation,
    Vortex,
    Publish,
    Complete,
    Closing,
}

#[derive(semio_framework_value::RetireOwned)]

struct Puzzle3dAddObjectKindPayload {
    kind_id: String,
    origin: [f64; 3],
}

/// ♻️ The owners one `Puzzle3dAddObjectKindWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle3dAddObjectKindWorkOwners {
    catalog_mutation: Option<Puzzle3dMutation>,
    mutation: Option<Puzzle3dMutation>,
    vortices: Vec<semio_s_artifact_puzzle_3d_model::Puzzle3dVortex>,
    mesh_url: Option<String>,
    object_id: Option<String>,
    payload: Option<Puzzle3dAddObjectKindPayload>,
}

pub struct Puzzle3dAddObjectKindWork {
    stage: Puzzle3dAddObjectKindStage,
    kind_cursor: usize,
    representation_cursor: usize,
    vortex_cursor: usize,
    kind_index: Option<usize>,
    payload: Option<Puzzle3dAddObjectKindPayload>,
    object_id: Option<String>,
    mesh_url: Option<String>,
    vortices: Vec<semio_s_artifact_puzzle_3d_model::Puzzle3dVortex>,
    catalog_mutation: Option<Puzzle3dMutation>,
    mutation: Option<Puzzle3dMutation>,
    close_owners: semio_s_plugin_puzzle_jobs::WorkClosing<Puzzle3dAddObjectKindWorkOwners>,
}

impl Default for Puzzle3dAddObjectKindWork {
    fn default() -> Self {
        Self { close_owners: Default::default(),
            stage: Puzzle3dAddObjectKindStage::Decode,
            kind_cursor: 0,
            representation_cursor: 0,
            vortex_cursor: 0,
            kind_index: None,
            payload: None,
            object_id: None,
            mesh_url: None,
            vortices: Vec::with_capacity(PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT),
            catalog_mutation: None,
            mutation: None,
        }
    }
}

impl Puzzle3dAddObjectKindWork {
    fn decode(command: &Puzzle3dCommand) -> Puzzle3dAddObjectKindPayload {
        let args = command.args();
        Puzzle3dAddObjectKindPayload {
            kind_id: args.and_then(|args| args.get("objectKind")).and_then(Value::as_str).unwrap_or("Object").to_string(),
            origin: args.and_then(|args| args.get("origin")).and_then(value_as_vec3).unwrap_or([0.0, 0.0, 0.0]),
        }
    }

    fn object_id(snapshot: &Puzzle3dPlaySnapshot, payload: &Puzzle3dAddObjectKindPayload) -> String {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        snapshot.typed().objects.len().hash(&mut hasher);
        payload.kind_id.hash(&mut hasher);
        for value in payload.origin {
            value.to_bits().hash(&mut hasher);
        }
        format!("puzzle3d.object.{:016x}", hasher.finish())
    }

    /// 🧱️ The declared default object kind an uncatalogued document materializes on its first
    /// `addObjectKind`: one catalog row carrying the command's own `objectKind` default id, with no
    /// representation and no vortex template, so the object this gesture creates references a real
    /// catalogued kind instead of a dangling one.
    fn declared_default_kind(kind_id: &str) -> semio_s_artifact_puzzle_3d_model::Puzzle3dCatalogObjectKind {
        semio_s_artifact_puzzle_3d_model::Puzzle3dCatalogObjectKind {
            id: kind_id.to_string(),
            name: kind_id.to_string(),
            label: kind_id.to_string(),
            description: String::new(),
            icon: String::new(),
            image: String::new(),
            unit: String::new(),
            is_abstract: false,
            base_kinds: Vec::new(),
            representations: Vec::new(),
            vortices: Vec::new(),
            attributes: Vec::new(),
            authors: Vec::new(),
        }
    }

}

impl<A: Puzzle3dEditor> semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>> for Puzzle3dAddObjectKindWork {
    fn tool_id(&self) -> &'static str {
        "addObjectKind"
    }

    /// 📏️ A document with no kind catalogs is THREE turns, not a refusal and not a no-op: decode,
    /// materialize the declared default kind catalog, publish. Returning `None` here instead
    /// conflated "nothing to do" with "over capacity", and the preflight reported *"puzzle command
    /// exceeds fixed semantic work capacity"* for every `addObjectKind` after `setActiveExample ""`
    /// cleared the catalogs.
    fn extent(&self, _command: &Puzzle3dCommand, snapshot: &Puzzle3dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<A>>>) -> Option<usize> {
        let Some(catalogs) = snapshot.typed().meta.kind_catalogs.as_ref() else { return Some(3) };
        let items = catalogs.objects.len().checked_add(PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT.checked_mul(2)?)?.checked_add(3)?;
        (items <= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        semio_s_plugin_puzzle_retained::step_demands::<EditorApp<A>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, interaction: _interaction, hover: _hover, context: _context, .. } = *input;
        let catalogs = snapshot.typed().meta.kind_catalogs.as_ref();
        match self.stage {
            Puzzle3dAddObjectKindStage::Decode => {
                let payload = Self::decode(command);
                self.object_id = Some(Self::object_id(snapshot, &payload));
                self.payload = Some(payload);
                self.stage = if catalogs.is_some() { Puzzle3dAddObjectKindStage::Kind } else { Puzzle3dAddObjectKindStage::Catalog };
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-add-kind-scan", "Finding object kind", "Objektart wird gesucht"))
            }
            Puzzle3dAddObjectKindStage::Catalog => {
                let payload = self.payload.as_ref().ok_or_else(|| Fault::from("puzzle3d-add-kind-payload-owner"))?;
                let catalogs = semio_s_artifact_puzzle_3d_model::Puzzle3dKindCatalogs { objects: vec![Self::declared_default_kind(&payload.kind_id)], ..Default::default() };
                self.catalog_mutation = Some(semio_s_artifact_puzzle_3d_mutations_catalog::replace_kind_catalogs::mutation::replace_kind_catalogs(Some(catalogs)));
                self.stage = Puzzle3dAddObjectKindStage::Publish;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-add-kind-catalog", "Creating the default object kind", "Standard-Objektart wird angelegt"))
            }
            Puzzle3dAddObjectKindStage::Kind => {
                let catalogs = catalogs.ok_or_else(|| Fault::from("puzzle3d-add-kind-catalog-owner"))?;
                let payload = self.payload.as_ref().ok_or_else(|| Fault::from("puzzle3d-add-kind-payload-owner"))?;
                let Some(kind) = catalogs.objects.get(self.kind_cursor) else {
                    self.stage = Puzzle3dAddObjectKindStage::Publish;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-add-kind-publish", "Preparing default object", "Standardobjekt wird vorbereitet"));
                };
                if kind.id == payload.kind_id {
                    if kind.representations.len() > PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT || kind.vortices.len() > PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT {
                        return Err(Fault::from("puzzle3d-add-kind-catalog-capacity"));
                    }
                    self.kind_index = Some(self.kind_cursor);
                    self.stage = Puzzle3dAddObjectKindStage::Representation;
                } else {
                    self.kind_cursor += 1;
                }
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-add-kind-scan", "Scanning object kind", "Objektart wird geprüft"))
            }
            Puzzle3dAddObjectKindStage::Representation => {
                let catalogs = catalogs.ok_or_else(|| Fault::from("puzzle3d-add-kind-catalog-owner"))?;
                let kind = catalogs.objects.get(self.kind_index.ok_or_else(|| Fault::from("puzzle3d-add-kind-owner"))?).ok_or_else(|| Fault::from("puzzle3d-add-kind-cursor"))?;
                let Some(representation) = kind.representations.get(self.representation_cursor) else {
                    self.stage = Puzzle3dAddObjectKindStage::Vortex;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-add-kind-vortex", "Preparing object vortices", "Objekt-Vortices werden vorbereitet"));
                };
                self.representation_cursor += 1;
                if self.mesh_url.is_none() && !representation.url.is_empty() {
                    self.mesh_url = Some(representation.url.clone());
                }
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-add-kind-representation", "Reading object representation", "Objektdarstellung wird gelesen"))
            }
            Puzzle3dAddObjectKindStage::Vortex => {
                let catalogs = catalogs.ok_or_else(|| Fault::from("puzzle3d-add-kind-catalog-owner"))?;
                let kind = catalogs.objects.get(self.kind_index.ok_or_else(|| Fault::from("puzzle3d-add-kind-owner"))?).ok_or_else(|| Fault::from("puzzle3d-add-kind-cursor"))?;
                let Some(template) = kind.vortices.get(self.vortex_cursor) else {
                    self.stage = Puzzle3dAddObjectKindStage::Publish;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-add-kind-publish", "Preparing object publication", "Objektveröffentlichung wird vorbereitet"));
                };
                let index = self.vortex_cursor;
                self.vortex_cursor += 1;
                self.vortices.push(semio_s_artifact_puzzle_3d_model::Puzzle3dVortex {
                    id: if template.id.is_empty() { format!("v{index}") } else { template.id.clone() },
                    label: (!template.label.is_empty()).then(|| template.label.clone()),
                    vortex_kind: template.vortex_kind.clone(),
                    position: template.point,
                    direction: Some(template.direction),
                    radius: template.radius,
                    hidden: false,
                    locked: false,
                });
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-add-kind-vortex", "Building one object vortex", "Ein Objekt-Vortex wird aufgebaut"))
            }
            Puzzle3dAddObjectKindStage::Publish => {
                let payload = self.payload.take().ok_or_else(|| Fault::from("puzzle3d-add-kind-payload-owner"))?;
                let object_id = self.object_id.take().ok_or_else(|| Fault::from("puzzle3d-add-kind-object-owner"))?;
                let scene_snapshot = puzzle3d_scene_snapshot_from_document(snapshot.typed());
                let object = semio_s_artifact_puzzle_3d_model::Puzzle3dObject {
                    id: object_id.clone(),
                    label: Some(puzzle3d_next_object_label(&scene_snapshot.objects, &scene_snapshot, &payload.kind_id)),
                    object_kind: Some(payload.kind_id),
                    anchor: Default::default(),
                    origin: payload.origin,
                    orientation: Some([0.0, 0.0, 0.0, 1.0]),
                    scale: None,
                    mesh_url: self.mesh_url.take(),
                    vortices: std::mem::take(&mut self.vortices),
                    hidden: false,
                    locked: false,
                };
                self.mutation = Some(semio_s_artifact_puzzle_3d_mutations_object::create_object::mutation::create_object(object, None));
                self.stage = Puzzle3dAddObjectKindStage::Complete;
                let artifact_mutations = self.catalog_mutation.take().into_iter().chain(self.mutation.take()).collect();
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit {
                    artifact_mutations,
                    ui_scope: puzzle3d_scope(puzzle3d_command_scope_class("addObjectKind")),
                    interaction_writes: vec![InteractionWrite::replace(PUZZLE3D_INTERACTION_DOMAIN, PUZZLE3D_GRANULARITY_OBJECT, [object_id])],
                    ..Default::default()
                }))
            }
            Puzzle3dAddObjectKindStage::Complete => Err(Fault::from("puzzle3d-add-kind-complete-repolled")),
            Puzzle3dAddObjectKindStage::Closing => Err(Fault::from("puzzle3d-add-kind-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle3dAddObjectKindStage::Closing;
        self.close_owners.stage(Puzzle3dAddObjectKindWorkOwners { catalog_mutation: std::mem::take(&mut self.catalog_mutation), mutation: std::mem::take(&mut self.mutation), vortices: std::mem::take(&mut self.vortices), mesh_url: std::mem::take(&mut self.mesh_url), object_id: std::mem::take(&mut self.object_id), payload: std::mem::take(&mut self.payload) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
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
        self.stage == Puzzle3dAddObjectKindStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle3dPatchInspectorStage {
    Selection,
    Objects,
    Vortices,
    Attractions,
    AttractionReconnect,
    References,
    Volumes,
    Complete,
    Closing,
}

/// ♻️ The owners one `Puzzle3dPatchInspectorWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle3dPatchInspectorWorkOwners {
    selected: HashSet<String>,
    mutations: Vec<Puzzle3dMutation>,
    pending_attraction: Option<semio_s_artifact_puzzle_3d_model::Puzzle3dAttraction>,
}

pub struct Puzzle3dPatchInspectorWork {
    stage: Puzzle3dPatchInspectorStage,
    selection_cursor: usize,
    item_cursor: usize,
    child_cursor: usize,
    selected: HashSet<String>,
    pending_attraction: Option<semio_s_artifact_puzzle_3d_model::Puzzle3dAttraction>,
    mutations: Vec<Puzzle3dMutation>,
    close_owners: semio_s_plugin_puzzle_jobs::WorkClosing<Puzzle3dPatchInspectorWorkOwners>,
}

impl Default for Puzzle3dPatchInspectorWork {
    fn default() -> Self {
        Self { close_owners: Default::default(),
            stage: Puzzle3dPatchInspectorStage::Selection,
            selection_cursor: 0,
            item_cursor: 0,
            child_cursor: 0,
            selected: HashSet::with_capacity(semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_DECODED_ITEMS),
            pending_attraction: None,
            mutations: Vec::with_capacity(semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS),
        }
    }
}

impl Puzzle3dPatchInspectorWork {
    fn entity(command: &Puzzle3dCommand) -> &str {
        command.args().and_then(|args| args.get("entity")).and_then(Value::as_str).unwrap_or("")
    }

    fn field(command: &Puzzle3dCommand) -> &str {
        command.args().and_then(|args| args.get("field")).and_then(Value::as_str).unwrap_or("")
    }

    fn granularity(entity: &str) -> &str {
        match entity {
            "object" => PUZZLE3D_GRANULARITY_OBJECT,
            "vortex" => PUZZLE3D_GRANULARITY_VORTEX,
            "attraction" => PUZZLE3D_GRANULARITY_ATTRACTION,
            "reference" => PUZZLE3D_GRANULARITY_REFERENCE,
            "targetVolume" => PUZZLE3D_GRANULARITY_TARGET_VOLUME,
            _ => "",
        }
    }

    fn source_id<'a>(command: &'a Puzzle3dCommand, interaction: &'a protocol::InteractionState, index: usize) -> Option<&'a str> {
        if let Some(ids) = command.args().and_then(|args| args.get("ids")).and_then(Value::as_array).filter(|ids| !ids.is_empty()) {
            return ids.get(index).and_then(Value::as_str);
        }
        let granularity = Self::granularity(Self::entity(command));
        interaction.selection.get(PUZZLE3D_INTERACTION_DOMAIN).filter(|selection| selection.granularity == granularity).and_then(|selection| selection.ids.get(index)).map(String::as_str)
    }

    fn source_len(command: &Puzzle3dCommand, interaction: &protocol::InteractionState) -> usize {
        command.args().and_then(|args| args.get("ids")).and_then(Value::as_array).filter(|ids| !ids.is_empty()).map_or_else(
            || {
                let granularity = Self::granularity(Self::entity(command));
                interaction.selection.get(PUZZLE3D_INTERACTION_DOMAIN).filter(|selection| selection.granularity == granularity).map_or(0, |selection| selection.ids.len())
            },
            Vec::len,
        )
    }

    fn first_stage(entity: &str) -> Puzzle3dPatchInspectorStage {
        match entity {
            "object" => Puzzle3dPatchInspectorStage::Objects,
            "vortex" => Puzzle3dPatchInspectorStage::Vortices,
            "attraction" => Puzzle3dPatchInspectorStage::Attractions,
            "reference" => Puzzle3dPatchInspectorStage::References,
            "targetVolume" => Puzzle3dPatchInspectorStage::Volumes,
            _ => Puzzle3dPatchInspectorStage::Complete,
        }
    }

    fn push(&mut self, mutation: Puzzle3dMutation) -> Result<(), Fault> {
        if self.mutations.len() >= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS {
            return Err(Fault::from("puzzle3d-patch-inspector-output-capacity"));
        }
        self.mutations.push(mutation);
        Ok(())
    }

    fn complete<A: Puzzle3dEditor>(&mut self) -> semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>> {
        self.stage = Puzzle3dPatchInspectorStage::Complete;
        semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), ui_scope: puzzle3d_scope(puzzle3d_command_scope_class("patchInspector")), ..Default::default() })
    }

    fn scale(value: Option<semio_s_artifact_puzzle_3d_model::Puzzle3dScale>) -> [f64; 3] {
        match value {
            Some(semio_s_artifact_puzzle_3d_model::Puzzle3dScale::Uniform(value)) => [value; 3],
            Some(semio_s_artifact_puzzle_3d_model::Puzzle3dScale::Vec3(value)) => value,
            None => [1.0; 3],
        }
    }

    fn object_mutation(command: &Puzzle3dCommand, object: &semio_s_artifact_puzzle_3d_model::Puzzle3dObject) -> Option<Puzzle3dMutation> {
        let args = command.args()?;
        let field = Self::field(command);
        let value = args.get("value");
        let delta = args.get("delta");
        match field {
            "hidden" => value.and_then(Value::as_bool).map(|value| semio_s_artifact_puzzle_3d_mutations_object::change_object_hidden::mutation::change_object_hidden(object.id.clone(), value)),
            "locked" => value.and_then(Value::as_bool).map(|value| semio_s_artifact_puzzle_3d_mutations_object::change_object_locked::mutation::change_object_locked(object.id.clone(), value)),
            "label" => Some(semio_s_artifact_puzzle_3d_mutations_object::edit_object_label::mutation::edit_object_label(object.id.clone(), value.and_then(Value::as_str).map(str::to_string))),
            "objectKind" => Some(semio_s_artifact_puzzle_3d_mutations_object::change_object_kind::mutation::change_object_kind(object.id.clone(), value.and_then(Value::as_str).map(str::to_string))),
            "meshUrl" => Some(semio_s_artifact_puzzle_3d_mutations_object::change_object_mesh::mutation::change_object_mesh(object.id.clone(), value.and_then(Value::as_str).map(str::to_string))),
            "origin" => value.and_then(value_as_vec3).map(|origin| semio_s_artifact_puzzle_3d_mutations_pose::move_object::mutation::move_object(object.id.clone(), origin)),
            _ => {
                if let Some(axis) = puzzle3d_axis_index(field, "origin") {
                    let mut origin = object.origin;
                    origin[axis] = puzzle3d_resolve_number_edit(origin[axis], value, delta)?;
                    return Some(semio_s_artifact_puzzle_3d_mutations_pose::move_object::mutation::move_object(object.id.clone(), origin));
                }
                if let Some(axis) = puzzle3d_axis_index(field, "scale") {
                    let mut scale = Self::scale(object.scale);
                    scale[axis] = puzzle3d_resolve_number_edit(scale[axis], value, delta)?;
                    return Some(semio_s_artifact_puzzle_3d_mutations_pose::scale_object::mutation::scale_object(object.id.clone(), Some(semio_s_artifact_puzzle_3d_model::Puzzle3dScale::Vec3(scale))));
                }
                let axis = puzzle3d_axis_index(field, "orientation")?;
                let mut orientation = object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
                orientation[axis] = puzzle3d_resolve_number_edit(orientation[axis], value, delta)?;
                Some(semio_s_artifact_puzzle_3d_mutations_pose::rotate_object::mutation::rotate_object(object.id.clone(), Some(quat_normalize(orientation))))
            }
        }
    }

    fn vortex_mutation(command: &Puzzle3dCommand, object_id: &str, vortex: &semio_s_artifact_puzzle_3d_model::Puzzle3dVortex) -> Option<Puzzle3dMutation> {
        let args = command.args()?;
        let field = Self::field(command);
        let value = args.get("value");
        let delta = args.get("delta");
        let mut next = vortex.clone();
        match field {
            "hidden" => next.hidden = value.and_then(Value::as_bool)?,
            "locked" => next.locked = value.and_then(Value::as_bool)?,
            "vortexKind" => next.vortex_kind = value.and_then(Value::as_str).map(str::to_string),
            "position" => next.position = value.and_then(value_as_vec3)?,
            "direction" => next.direction = Some(value.and_then(value_as_vec3)?),
            "radius" => next.radius = Some(puzzle3d_resolve_number_edit(next.radius.unwrap_or(0.35), value, delta)?),
            _ => {
                if let Some(axis) = puzzle3d_axis_index(field, "position") {
                    next.position[axis] = puzzle3d_resolve_number_edit(next.position[axis], value, delta)?;
                } else {
                    let axis = puzzle3d_axis_index(field, "direction")?;
                    let mut direction = next.direction.unwrap_or([0.0, 0.0, 1.0]);
                    direction[axis] = puzzle3d_resolve_number_edit(direction[axis], value, delta)?;
                    next.direction = Some(direction);
                }
            }
        }
        Some(semio_s_artifact_puzzle_3d_mutations_vortex::replace_object_vortex::mutation::replace_object_vortex(object_id.to_string(), vortex.id.clone(), next))
    }

    fn attraction_geometry(command: &Puzzle3dCommand, attraction: &semio_s_artifact_puzzle_3d_model::Puzzle3dAttraction) -> Option<Puzzle3dMutation> {
        let args = command.args()?;
        let field = Self::field(command);
        let value = args.get("value");
        let delta = args.get("delta");
        let mut geometry = [attraction.gap, attraction.shift, attraction.rise, attraction.rotation, attraction.turn, attraction.tilt, attraction.x, attraction.y];
        let index = match field {
            "gap" => 0,
            "shift" => 1,
            "rise" => 2,
            "rotation" => 3,
            "turn" => 4,
            "tilt" => 5,
            _ => return None,
        };
        geometry[index] = puzzle3d_resolve_number_edit(geometry[index], value, delta)?;
        Some(semio_s_artifact_puzzle_3d_mutations_vortex::replace_attraction_geometry::mutation::replace_attraction_geometry(semio_s_artifact_puzzle_3d_mutations_vortex::replace_attraction_geometry::mutation::ReplaceAttractionGeometry {
            id: attraction.id.clone(),
            new_gap: geometry[0],
            new_shift: geometry[1],
            new_rise: geometry[2],
            new_rotation: geometry[3],
            new_turn: geometry[4],
            new_tilt: geometry[5],
            new_x: geometry[6],
            new_y: geometry[7],
        }))
    }

    fn reference_mutation(command: &Puzzle3dCommand, reference: &semio_s_artifact_puzzle_3d_model::Puzzle3dReference) -> Option<Puzzle3dMutation> {
        let args = command.args()?;
        let field = Self::field(command);
        let value = args.get("value");
        let delta = args.get("delta");
        match field {
            "hidden" => value.and_then(Value::as_bool).map(|value| semio_s_artifact_puzzle_3d_mutations_reference::change_reference_hidden::mutation::change_reference_hidden(reference.id.clone(), value)),
            "locked" => value.and_then(Value::as_bool).map(|value| semio_s_artifact_puzzle_3d_mutations_reference::change_reference_locked::mutation::change_reference_locked(reference.id.clone(), value)),
            "sourceUrl" | "mediaKind" => {
                let mut source = reference.source.clone();
                if field == "sourceUrl" {
                    source.url = value.and_then(Value::as_str)?.to_string();
                } else {
                    source.media_kind = value.and_then(Value::as_str).map(str::to_string);
                }
                Some(semio_s_artifact_puzzle_3d_mutations_reference::replace_reference_source::mutation::replace_reference_source(reference.id.clone(), source))
            }
            "origin" => value.and_then(value_as_vec3).map(|origin| semio_s_artifact_puzzle_3d_mutations_reference::move_reference::mutation::move_reference(reference.id.clone(), origin)),
            "widthWorld" => puzzle3d_resolve_number_edit(reference.width_world, value, delta).map(|width| semio_s_artifact_puzzle_3d_mutations_reference::resize_reference::mutation::resize_reference(reference.id.clone(), width)),
            _ => {
                let axis = puzzle3d_axis_index(field, "origin")?;
                let mut origin = reference.origin;
                origin[axis] = puzzle3d_resolve_number_edit(origin[axis], value, delta)?;
                Some(semio_s_artifact_puzzle_3d_mutations_reference::move_reference::mutation::move_reference(reference.id.clone(), origin))
            }
        }
    }

    fn volume_mutation(command: &Puzzle3dCommand, volume: &semio_s_artifact_puzzle_3d_model::Puzzle3dTargetVolume) -> Option<Puzzle3dMutation> {
        let args = command.args()?;
        let field = Self::field(command);
        let value = args.get("value");
        let delta = args.get("delta");
        match field {
            "hidden" => value.and_then(Value::as_bool).map(|value| semio_s_artifact_puzzle_3d_mutations_volume::change_target_volume_hidden::mutation::change_target_volume_hidden(volume.id.clone(), value)),
            "locked" => value.and_then(Value::as_bool).map(|value| semio_s_artifact_puzzle_3d_mutations_volume::change_target_volume_locked::mutation::change_target_volume_locked(volume.id.clone(), value)),
            "origin" => value.and_then(value_as_vec3).map(|origin| semio_s_artifact_puzzle_3d_mutations_volume::move_target_volume::mutation::move_target_volume(volume.id.clone(), origin)),
            _ => {
                if let Some(axis) = puzzle3d_axis_index(field, "origin") {
                    let mut origin = volume.origin;
                    origin[axis] = puzzle3d_resolve_number_edit(origin[axis], value, delta)?;
                    return Some(semio_s_artifact_puzzle_3d_mutations_volume::move_target_volume::mutation::move_target_volume(volume.id.clone(), origin));
                }
                if let Some(axis) = puzzle3d_axis_index(field, "scale") {
                    let mut scale = Self::scale(volume.scale);
                    scale[axis] = puzzle3d_resolve_number_edit(scale[axis], value, delta)?;
                    return Some(semio_s_artifact_puzzle_3d_mutations_volume::scale_target_volume::mutation::scale_target_volume(volume.id.clone(), Some(semio_s_artifact_puzzle_3d_model::Puzzle3dScale::Vec3(scale))));
                }
                let axis = puzzle3d_axis_index(field, "orientation")?;
                let mut orientation = volume.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
                orientation[axis] = puzzle3d_resolve_number_edit(orientation[axis], value, delta)?;
                Some(semio_s_artifact_puzzle_3d_mutations_volume::rotate_target_volume::mutation::rotate_target_volume(volume.id.clone(), Some(quat_normalize(orientation))))
            }
        }
    }
}

impl<A: Puzzle3dEditor> semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>> for Puzzle3dPatchInspectorWork {
    fn tool_id(&self) -> &'static str {
        "patchInspector"
    }

    fn extent(&self, command: &Puzzle3dCommand, snapshot: &Puzzle3dPlaySnapshot, interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<A>>>) -> Option<usize> {
        let source = Self::source_len(command, interaction);
        let document = snapshot.typed();
        let scan = match Self::entity(command) {
            "object" => document.objects.len().checked_add(1)?,
            "vortex" => {
                let mut vortices = 0usize;
                for object in &document.objects {
                    vortices = vortices.checked_add(object.vortices.len())?;
                }
                document.objects.len().checked_add(1)?.checked_add(vortices)?
            }
            "attraction" => document.attractions.len().checked_mul(2)?.checked_add(1)?,
            "reference" => document.references.len().checked_add(1)?,
            "targetVolume" => document.target_volumes.len().checked_add(1)?,
            _ => 0,
        };
        let items = source.checked_add(scan)?.checked_add(1)?;
        (source <= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_DECODED_ITEMS && items <= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        semio_s_plugin_puzzle_retained::step_demands::<EditorApp<A>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, interaction, hover: _hover, context: _context, .. } = *input;
        match self.stage {
            Puzzle3dPatchInspectorStage::Selection => {
                if let Some(id) = Self::source_id(command, interaction, self.selection_cursor) {
                    if self.selected.len() >= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_DECODED_ITEMS {
                        return Err(Fault::from("puzzle3d-patch-inspector-selection-capacity"));
                    }
                    self.selected.insert(id.to_string());
                    self.selection_cursor += 1;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-patch-inspector-selection", "Reading inspector target", "Inspektionsziel wird gelesen"));
                }
                self.stage = Self::first_stage(Self::entity(command));
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-patch-inspector-scan", "Finding inspector target", "Inspektionsziel wird gesucht"))
            }
            Puzzle3dPatchInspectorStage::Objects => {
                let Some(object) = snapshot.typed().objects.get(self.item_cursor) else { return Ok(self.complete()) };
                self.item_cursor += 1;
                if self.selected.contains(&object.id) {
                    if let Some(mutation) = Self::object_mutation(command, object) {
                        self.push(mutation)?;
                    }
                }
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-patch-inspector-object", "Patching object", "Objekt wird geändert"))
            }
            Puzzle3dPatchInspectorStage::Vortices => {
                let Some(object) = snapshot.typed().objects.get(self.item_cursor) else { return Ok(self.complete()) };
                let Some(vortex) = object.vortices.get(self.child_cursor) else {
                    self.item_cursor += 1;
                    self.child_cursor = 0;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-patch-inspector-vortex-owner", "Advancing vortex owner", "Vortex-Eigentümer wird gewechselt"));
                };
                self.child_cursor += 1;
                if self.selected.contains(&puzzle3d_vortex_full_id(&object.id, &vortex.id)) {
                    if let Some(mutation) = Self::vortex_mutation(command, &object.id, vortex) {
                        self.push(mutation)?;
                    }
                }
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-patch-inspector-vortex", "Patching vortex", "Vortex wird geändert"))
            }
            Puzzle3dPatchInspectorStage::Attractions => {
                let Some(attraction) = snapshot.typed().attractions.get(self.item_cursor) else { return Ok(self.complete()) };
                self.item_cursor += 1;
                if !self.selected.contains(&attraction.id) {
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-patch-inspector-attraction", "Scanning attraction", "Anziehung wird geprüft"));
                }
                let field = Self::field(command);
                if matches!(field, "attracting" | "attracted") {
                    let Some(value) = command.args().and_then(|args| args.get("value")).and_then(Value::as_str) else {
                        return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-patch-inspector-attraction", "Skipping malformed attraction", "Fehlerhafte Anziehung wird übersprungen"));
                    };
                    let mut next = attraction.clone();
                    if field == "attracting" {
                        next.attracting = value.to_string();
                    } else {
                        next.attracted = value.to_string();
                    }
                    self.push(semio_s_artifact_puzzle_3d_mutations_vortex::disconnect_vortices::mutation::disconnect_vortices(attraction.id.clone()))?;
                    self.pending_attraction = Some(next);
                    self.stage = Puzzle3dPatchInspectorStage::AttractionReconnect;
                } else if let Some(mutation) = Self::attraction_geometry(command, attraction) {
                    self.push(mutation)?;
                }
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-patch-inspector-attraction", "Patching attraction", "Anziehung wird geändert"))
            }
            Puzzle3dPatchInspectorStage::AttractionReconnect => {
                let attraction = self.pending_attraction.take().ok_or_else(|| Fault::from("puzzle3d-patch-inspector-attraction-owner"))?;
                self.push(semio_s_artifact_puzzle_3d_mutations_vortex::connect_vortices::mutation::connect_vortices(
                    attraction.id,
                    attraction.attracting,
                    attraction.attracted,
                    attraction.gap,
                    attraction.shift,
                    attraction.rise,
                    attraction.rotation,
                    attraction.turn,
                    attraction.tilt,
                    attraction.x,
                    attraction.y, None,
                ))?;
                self.stage = Puzzle3dPatchInspectorStage::Attractions;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-patch-inspector-attraction-reconnect", "Reconnecting attraction", "Anziehung wird neu verbunden"))
            }
            Puzzle3dPatchInspectorStage::References => {
                let Some(reference) = snapshot.typed().references.get(self.item_cursor) else { return Ok(self.complete()) };
                self.item_cursor += 1;
                if self.selected.contains(&reference.id) {
                    if let Some(mutation) = Self::reference_mutation(command, reference) {
                        self.push(mutation)?;
                    }
                }
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-patch-inspector-reference", "Patching reference", "Referenz wird geändert"))
            }
            Puzzle3dPatchInspectorStage::Volumes => {
                let Some(volume) = snapshot.typed().target_volumes.get(self.item_cursor) else { return Ok(self.complete()) };
                self.item_cursor += 1;
                if self.selected.contains(&volume.id) {
                    if let Some(mutation) = Self::volume_mutation(command, volume) {
                        self.push(mutation)?;
                    }
                }
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-patch-inspector-volume", "Patching target volume", "Zielvolumen wird geändert"))
            }
            Puzzle3dPatchInspectorStage::Complete => Err(Fault::from("puzzle3d-patch-inspector-complete-repolled")),
            Puzzle3dPatchInspectorStage::Closing => Err(Fault::from("puzzle3d-patch-inspector-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle3dPatchInspectorStage::Closing;
        self.close_owners.stage(Puzzle3dPatchInspectorWorkOwners { selected: std::mem::take(&mut self.selected), mutations: std::mem::take(&mut self.mutations), pending_attraction: std::mem::take(&mut self.pending_attraction) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
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
        self.stage == Puzzle3dPatchInspectorStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}

pub const PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle3dCreateAttractionStage {
    Existing,
    Attracting,
    Attracted,
    Compatibility,
    Publish,
    Complete,
    Closing,
}

#[derive(semio_framework_value::RetireOwned)]

struct Puzzle3dAttractionEndpoint {
    vortex_id: String,
    vortex_kind: Option<String>,
    local_position: [f64; 3],
    local_direction: [f64; 3],
    object_position: [f64; 3],
    object_orientation: [f64; 4],
}

/// ♻️ The owners one `Puzzle3dCreateAttractionWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle3dCreateAttractionWorkOwners {
    attracting: Option<Puzzle3dAttractionEndpoint>,
    attracted: Option<Puzzle3dAttractionEndpoint>,
}

pub struct Puzzle3dCreateAttractionWork {
    stage: Puzzle3dCreateAttractionStage,
    item_cursor: usize,
    child_cursor: usize,
    compatibility_cursor: usize,
    attracting: Option<Puzzle3dAttractionEndpoint>,
    attracted: Option<Puzzle3dAttractionEndpoint>,
    compatible: bool,
    close_owners: semio_s_plugin_puzzle_jobs::WorkClosing<Puzzle3dCreateAttractionWorkOwners>,
}

impl Default for Puzzle3dCreateAttractionWork {
    fn default() -> Self {
        Self { close_owners: Default::default(), stage: Puzzle3dCreateAttractionStage::Existing, item_cursor: 0, child_cursor: 0, compatibility_cursor: 0, attracting: None, attracted: None, compatible: false }
    }
}

impl Puzzle3dCreateAttractionWork {
    fn ids(command: &Puzzle3dCommand) -> (&str, &str) {
        let args = command.args();
        (args.and_then(|args| args.get("attracting")).and_then(Value::as_str).unwrap_or(""), args.and_then(|args| args.get("attracted")).and_then(Value::as_str).unwrap_or(""))
    }

    fn endpoint(object: &semio_s_artifact_puzzle_3d_model::Puzzle3dObject, vortex: &semio_s_artifact_puzzle_3d_model::Puzzle3dVortex) -> Puzzle3dAttractionEndpoint {
        Puzzle3dAttractionEndpoint {
            vortex_id: puzzle3d_vortex_full_id(&object.id, &vortex.id),
            vortex_kind: vortex.vortex_kind.clone(),
            local_position: vortex.position,
            local_direction: vortex.direction.unwrap_or([0.0, 0.0, -1.0]),
            object_position: object.origin,
            object_orientation: object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
        }
    }

    fn scan_endpoint(&mut self, snapshot: &Puzzle3dPlaySnapshot, requested: &str) -> Result<Option<Puzzle3dAttractionEndpoint>, Fault> {
        let Some(object) = snapshot.typed().objects.get(self.item_cursor) else {
            return Ok(None);
        };
        if object.vortices.len() > PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT {
            return Err(Fault::from("puzzle3d-create-attraction-vortex-capacity"));
        }
        let Some(vortex) = object.vortices.get(self.child_cursor) else {
            self.item_cursor += 1;
            self.child_cursor = 0;
            return Ok(None);
        };
        self.child_cursor += 1;
        Ok((puzzle3d_vortex_full_id(&object.id, &vortex.id) == requested).then(|| Self::endpoint(object, vortex)))
    }

    fn begin_endpoint_scan(&mut self, stage: Puzzle3dCreateAttractionStage) {
        self.item_cursor = 0;
        self.child_cursor = 0;
        self.stage = stage;
    }

    fn complete<A: Puzzle3dEditor>(&mut self) -> semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>> {
        self.stage = Puzzle3dCreateAttractionStage::Complete;
        semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit::default())
    }
}

impl<A: Puzzle3dEditor> semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>> for Puzzle3dCreateAttractionWork {
    fn tool_id(&self) -> &'static str {
        "createAttraction"
    }

    fn extent(&self, _command: &Puzzle3dCommand, snapshot: &Puzzle3dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<A>>>) -> Option<usize> {
        let document = snapshot.typed();
        let mut object_vortices = 0usize;
        for object in &document.objects {
            object_vortices = object_vortices.checked_add(object.vortices.len())?;
        }
        let endpoint_scan_stage = object_vortices.checked_add(document.objects.len())?.checked_add(1)?;
        let items = document.attractions.len().checked_add(1)?.checked_add(endpoint_scan_stage)?.checked_add(endpoint_scan_stage)?.checked_add(document.meta.kind_compatibility.len())?.checked_add(2)?;
        (items <= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        semio_s_plugin_puzzle_retained::step_demands::<EditorApp<A>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, interaction: _interaction, hover: _hover, context: _context, .. } = *input;
        let (attracting_id, attracted_id) = Self::ids(command);
        if attracting_id.is_empty() || attracted_id.is_empty() || attracting_id == attracted_id {
            return Ok(self.complete());
        }
        match self.stage {
            Puzzle3dCreateAttractionStage::Existing => {
                let Some(attraction) = snapshot.typed().attractions.get(self.item_cursor) else {
                    self.begin_endpoint_scan(Puzzle3dCreateAttractionStage::Attracting);
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-create-attraction-attracting", "Finding attracting vortex", "Anziehender Vortex wird gesucht"));
                };
                self.item_cursor += 1;
                if (attraction.attracting == attracting_id && attraction.attracted == attracted_id) || (attraction.attracting == attracted_id && attraction.attracted == attracting_id) {
                    return Ok(self.complete());
                }
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-create-attraction-existing", "Checking existing attraction", "Bestehende Anziehung wird geprüft"))
            }
            Puzzle3dCreateAttractionStage::Attracting => {
                if self.item_cursor >= snapshot.typed().objects.len() {
                    return Ok(self.complete());
                }
                if let Some(endpoint) = self.scan_endpoint(snapshot, attracting_id)? {
                    self.attracting = Some(endpoint);
                    self.begin_endpoint_scan(Puzzle3dCreateAttractionStage::Attracted);
                }
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-create-attraction-attracting", "Scanning attracting vortex", "Anziehender Vortex wird geprüft"))
            }
            Puzzle3dCreateAttractionStage::Attracted => {
                if self.item_cursor >= snapshot.typed().objects.len() {
                    return Ok(self.complete());
                }
                if let Some(endpoint) = self.scan_endpoint(snapshot, attracted_id)? {
                    self.attracted = Some(endpoint);
                    self.compatibility_cursor = 0;
                    self.compatible = snapshot.typed().meta.kind_compatibility.is_empty();
                    self.stage = Puzzle3dCreateAttractionStage::Compatibility;
                }
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-create-attraction-attracted", "Scanning attracted vortex", "Angezogener Vortex wird geprüft"))
            }
            Puzzle3dCreateAttractionStage::Compatibility => {
                let attracting_kind = self.attracting.as_ref().and_then(|endpoint| endpoint.vortex_kind.as_deref());
                let attracted_kind = self.attracted.as_ref().and_then(|endpoint| endpoint.vortex_kind.as_deref());
                let (Some(attracting_kind), Some(attracted_kind)) = (attracting_kind, attracted_kind) else {
                    return Ok(self.complete());
                };
                let Some(row) = snapshot.typed().meta.kind_compatibility.get(self.compatibility_cursor) else {
                    if self.compatible {
                        self.stage = Puzzle3dCreateAttractionStage::Publish;
                        return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-create-attraction-publish", "Preparing attraction", "Anziehung wird vorbereitet"));
                    }
                    return Ok(self.complete());
                };
                self.compatibility_cursor += 1;
                self.compatible |= (row.source == attracting_kind && row.target == attracted_kind) || (row.bidirectional && row.source == attracted_kind && row.target == attracting_kind);
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-create-attraction-compatibility", "Checking vortex compatibility", "Vortex-Kompatibilität wird geprüft"))
            }
            Puzzle3dCreateAttractionStage::Publish => {
                let attracting = self.attracting.take().ok_or_else(|| Fault::from("puzzle3d-create-attraction-attracting-owner"))?;
                let attracted = self.attracted.take().ok_or_else(|| Fault::from("puzzle3d-create-attraction-attracted-owner"))?;
                let (gap, shift, rise, rotation, turn, tilt) = derive_attraction_params(
                    attracting.object_position,
                    attracting.object_orientation,
                    attracting.local_position,
                    attracting.local_direction,
                    attracted.local_position,
                    attracted.local_direction,
                    attracted.object_position,
                    attracted.object_orientation,
                );
                let id = format!("attraction-{}", PUZZLE3D_ID_COUNTER.fetch_add(1, Ordering::Relaxed));
                self.stage = Puzzle3dCreateAttractionStage::Complete;
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit {
                    artifact_mutations: vec![semio_s_artifact_puzzle_3d_mutations_vortex::connect_vortices::mutation::connect_vortices(id, attracting.vortex_id, attracted.vortex_id, gap, shift, rise, rotation, turn, tilt, 0.0, 0.0, None)],
                    ui_scope: puzzle3d_scope(puzzle3d_command_scope_class("createAttraction")),
                    ..Default::default()
                }))
            }
            Puzzle3dCreateAttractionStage::Complete => Err(Fault::from("puzzle3d-create-attraction-complete-repolled")),
            Puzzle3dCreateAttractionStage::Closing => Err(Fault::from("puzzle3d-create-attraction-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle3dCreateAttractionStage::Closing;
        self.close_owners.stage(Puzzle3dCreateAttractionWorkOwners { attracting: std::mem::take(&mut self.attracting), attracted: std::mem::take(&mut self.attracted) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
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
        self.stage == Puzzle3dCreateAttractionStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle3dSetActiveExampleStage {
    DeleteAttractions,
    DeleteObjects,
    DeleteVolumes,
    DeleteReferences,
    DeleteCompatibility,
    Domain,
    Catalogs,
    CreateObjects,
    CreateAttractions,
    CreateVolumes,
    CreateReferences,
    CreateCompatibility,
    Publish,
    Complete,
    Closing,
}

/// 🔢️ Steps `Puzzle3dSetActiveExampleWork` spends on stage TRANSITIONS rather than on document items:
/// each of the five delete stages and each of the five create stages answers its own exhaustion with one
/// `Progress`, `Domain` and `Catalogs` are one step each, and `Publish` is the final `Complete` call —
/// 5 + 1 + 1 + 5 + 1 = 13. The extent is an upper bound on `step()` calls (the sibling `patchInspector`
/// law asserts `iterations <= extent`), so it has to carry all of them; it declared `3`, which is short
/// by ten and made a real example load overrun its own declared envelope.
const PUZZLE3D_SET_ACTIVE_EXAMPLE_FIXED_STEPS: usize = 13;

const PUZZLE3D_SET_ACTIVE_EXAMPLE_CHUNK: usize = 8;

/// ♻️ The owners one `Puzzle3dSetActiveExampleWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle3dSetActiveExampleWorkOwners {
    mutations: Vec<Puzzle3dMutation>,
}

pub struct Puzzle3dSetActiveExampleWork {
    stage: Puzzle3dSetActiveExampleStage,
    cursor: usize,
    mutations: Vec<Puzzle3dMutation>,
    close_owners: semio_s_plugin_puzzle_jobs::WorkClosing<Puzzle3dSetActiveExampleWorkOwners>,
}

impl Default for Puzzle3dSetActiveExampleWork {
    fn default() -> Self {
        Self { close_owners: Default::default(), stage: Puzzle3dSetActiveExampleStage::DeleteAttractions, cursor: 0, mutations: Vec::with_capacity(semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS) }
    }
}

impl Puzzle3dSetActiveExampleWork {
    fn target(command: &Puzzle3dCommand) -> Option<&'static Puzzle3dSceneSnapshot> {
        match command.args().and_then(|args| args.get("exampleId")).and_then(Value::as_str).unwrap_or("") {
            "" => Some(&EMPTY_EXAMPLE_SNAPSHOT),
            PUZZLE3D_EXAMPLE_CONCRETE_FOREST | "concrete" => Some(&CONCRETE_FOREST_EXAMPLE_SNAPSHOT),
            PUZZLE3D_EXAMPLE_NAKAGIN | "nakagin" => Some(&NAKAGIN_EXAMPLE_SNAPSHOT),
            _ => None,
        }
    }

    /// 🏷️ The CANONICAL example id this command loads, for the alias the picker may have sent
    /// (`concrete`/`nakagin`). Persisted on the shared config so `export_snapshot` can name its
    /// download after the example instead of one constant filename.
    fn canonical_example_id(command: &Puzzle3dCommand) -> Option<&'static str> {
        match command.args().and_then(|args| args.get("exampleId")).and_then(Value::as_str).unwrap_or("") {
            "" => Some(""),
            PUZZLE3D_EXAMPLE_CONCRETE_FOREST | "concrete" => Some(PUZZLE3D_EXAMPLE_CONCRETE_FOREST),
            PUZZLE3D_EXAMPLE_NAKAGIN | "nakagin" => Some(PUZZLE3D_EXAMPLE_NAKAGIN),
            _ => None,
        }
    }

    fn compatibility_rows(target: &Puzzle3dSceneSnapshot) -> &[semio_framework_value::DslValue] {
        target.meta.kind_compatibility.as_ref().and_then(semio_framework_value::DslValue::as_array).unwrap_or_default()
    }

    fn push(&mut self, mutation: Puzzle3dMutation) -> Result<(), Fault> {
        if self.mutations.len() >= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS {
            return Err(Fault::from("puzzle3d-set-active-example-output-capacity"));
        }
        self.mutations.push(mutation);
        Ok(())
    }

    fn take_chunk(cursor: &mut usize, len: usize) -> std::ops::Range<usize> {
        if *cursor >= len {
            return *cursor..*cursor;
        }
        let end = cursor.saturating_add(PUZZLE3D_SET_ACTIVE_EXAMPLE_CHUNK).min(len);
        let range = *cursor..end;
        *cursor = end;
        range
    }

    fn advance(&mut self, stage: Puzzle3dSetActiveExampleStage) {
        self.cursor = 0;
        self.stage = stage;
    }
}

impl<A: Puzzle3dEditor> semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>> for Puzzle3dSetActiveExampleWork {
    fn tool_id(&self) -> &'static str {
        "setActiveExample"
    }

    fn extent(&self, command: &Puzzle3dCommand, snapshot: &Puzzle3dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<A>>>) -> Option<usize> {
        let target = Self::target(command)?;
        let document = snapshot.typed();
        let items = document
            .attractions
            .len()
            .checked_add(document.objects.len())?
            .checked_add(document.target_volumes.len())?
            .checked_add(document.references.len())?
            .checked_add(document.meta.kind_compatibility.len())?
            .checked_add(target.attractions.len())?
            .checked_add(target.objects.len())?
            .checked_add(target.target_volumes.len())?
            .checked_add(target.references.len())?
            .checked_add(Self::compatibility_rows(target).len())?
            .checked_add(PUZZLE3D_SET_ACTIVE_EXAMPLE_FIXED_STEPS)?;
        (items <= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        semio_s_plugin_puzzle_retained::step_demands::<EditorApp<A>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config, interaction: _interaction, hover: _hover, context: _context, .. } = *input;
        let Some(target) = Self::target(command) else { return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit::default())) };
        match self.stage {
            Puzzle3dSetActiveExampleStage::DeleteAttractions => {
                let items = &snapshot.typed().attractions;
                let range = Self::take_chunk(&mut self.cursor, items.len());
                if !range.is_empty() {
                    for attraction in &items[range] {
                        self.push(semio_s_artifact_puzzle_3d_mutations_vortex::disconnect_vortices::mutation::disconnect_vortices(attraction.id.clone()))?;
                    }
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-example-delete-attraction", "Removing old attraction", "Alte Anziehung wird entfernt"));
                }
                self.advance(Puzzle3dSetActiveExampleStage::DeleteObjects);
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-example-delete-object", "Removing old object", "Altes Objekt wird entfernt"))
            }
            Puzzle3dSetActiveExampleStage::DeleteObjects => {
                let items = &snapshot.typed().objects;
                let range = Self::take_chunk(&mut self.cursor, items.len());
                if !range.is_empty() {
                    for object in &items[range] {
                        self.push(semio_s_artifact_puzzle_3d_mutations_object::delete_object::mutation::delete_object(object.id.clone()))?;
                    }
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-example-delete-object", "Removing old object", "Altes Objekt wird entfernt"));
                }
                self.advance(Puzzle3dSetActiveExampleStage::DeleteVolumes);
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-example-delete-volume", "Removing old target volume", "Altes Zielvolumen wird entfernt"))
            }
            Puzzle3dSetActiveExampleStage::DeleteVolumes => {
                let items = &snapshot.typed().target_volumes;
                let range = Self::take_chunk(&mut self.cursor, items.len());
                if !range.is_empty() {
                    for volume in &items[range] {
                        self.push(semio_s_artifact_puzzle_3d_mutations_volume::delete_target_volume::mutation::delete_target_volume(volume.id.clone()))?;
                    }
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-example-delete-volume", "Removing old target volume", "Altes Zielvolumen wird entfernt"));
                }
                self.advance(Puzzle3dSetActiveExampleStage::DeleteReferences);
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-example-delete-reference", "Removing old reference", "Alte Referenz wird entfernt"))
            }
            Puzzle3dSetActiveExampleStage::DeleteReferences => {
                let items = &snapshot.typed().references;
                let range = Self::take_chunk(&mut self.cursor, items.len());
                if !range.is_empty() {
                    for reference in &items[range] {
                        self.push(semio_s_artifact_puzzle_3d_mutations_reference::delete_reference::mutation::delete_reference(reference.id.clone()))?;
                    }
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-example-delete-reference", "Removing old reference", "Alte Referenz wird entfernt"))
                }
                self.advance(Puzzle3dSetActiveExampleStage::DeleteCompatibility);
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-example-delete-compatibility", "Removing old compatibility", "Alte Kompatibilität wird entfernt"))
            }
            Puzzle3dSetActiveExampleStage::DeleteCompatibility => {
                let items = &snapshot.typed().meta.kind_compatibility;
                let range = Self::take_chunk(&mut self.cursor, items.len());
                if !range.is_empty() {
                    for row in &items[range] {
                        self.push(semio_s_artifact_puzzle_3d_mutations_catalog::disconnect_kind_compatibility::mutation::disconnect_kind_compatibility(row.source.clone(), row.target.clone()))?;
                    }
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-example-delete-compatibility", "Removing old compatibility", "Alte Kompatibilität wird entfernt"));
                }
                self.advance(Puzzle3dSetActiveExampleStage::Domain);
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-example-domain", "Updating document domain", "Dokumentdomäne wird aktualisiert"))
            }
            Puzzle3dSetActiveExampleStage::Domain => {
                self.push(semio_s_artifact_puzzle_3d_mutations_catalog::change_domain::mutation::change_domain(target.domain.clone()))?;
                self.advance(Puzzle3dSetActiveExampleStage::Catalogs);
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-example-catalogs", "Updating kind catalogs", "Artenkataloge werden aktualisiert"))
            }
            Puzzle3dSetActiveExampleStage::Catalogs => {
                let catalogs = target.meta.kind_catalogs.as_ref().map(|catalogs| FromValue::from_value(catalogs.clone())).transpose().map_err(|_| Fault::from("puzzle3d-set-active-example-catalogs-malformed"))?;
                self.push(semio_s_artifact_puzzle_3d_mutations_catalog::replace_kind_catalogs::mutation::replace_kind_catalogs(catalogs))?;
                self.advance(Puzzle3dSetActiveExampleStage::CreateObjects);
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-example-create-object", "Adding example object", "Beispielobjekt wird hinzugefügt"))
            }
            Puzzle3dSetActiveExampleStage::CreateObjects => {
                let items = &target.objects;
                let range = Self::take_chunk(&mut self.cursor, items.len());
                if !range.is_empty() {
                    for object in &items[range] {
                        let value = semio_framework_value::ToValue::to_value(object);
                        let object = FromValue::from_value(value).map_err(|_| Fault::from("puzzle3d-set-active-example-object-malformed"))?;
                        self.push(semio_s_artifact_puzzle_3d_mutations_object::create_object::mutation::create_object(object, None))?;
                    }
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-example-create-object", "Adding example object", "Beispielobjekt wird hinzugefügt"));
                }
                self.advance(Puzzle3dSetActiveExampleStage::CreateAttractions);
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-example-create-attraction", "Adding example attraction", "Beispielanziehung wird hinzugefügt"))
            }
            Puzzle3dSetActiveExampleStage::CreateAttractions => {
                let items = &target.attractions;
                let range = Self::take_chunk(&mut self.cursor, items.len());
                if !range.is_empty() {
                    for attraction in &items[range] {
                        self.push(semio_s_artifact_puzzle_3d_mutations_vortex::connect_vortices::mutation::connect_vortices(
                            attraction.id.clone(),
                            attraction.attracting.clone(),
                            attraction.attracted.clone(),
                            attraction.gap,
                            attraction.shift,
                            attraction.rise,
                            attraction.rotation,
                            attraction.turn,
                            attraction.tilt,
                            0.0,
                            0.0, None,
                        ))?;
                    }
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-example-create-attraction", "Adding example attraction", "Beispielanziehung wird hinzugefügt"));
                }
                self.advance(Puzzle3dSetActiveExampleStage::CreateVolumes);
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-example-create-volume", "Adding example target volume", "Beispielzielvolumen wird hinzugefügt"))
            }
            Puzzle3dSetActiveExampleStage::CreateVolumes => {
                let items = &target.target_volumes;
                let range = Self::take_chunk(&mut self.cursor, items.len());
                if !range.is_empty() {
                    for volume in &items[range] {
                        let value = semio_framework_value::ToValue::to_value(volume);
                        let volume = FromValue::from_value(value).map_err(|_| Fault::from("puzzle3d-set-active-example-volume-malformed"))?;
                        self.push(semio_s_artifact_puzzle_3d_mutations_volume::create_target_volume::mutation::create_target_volume(volume, None))?;
                    }
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-example-create-volume", "Adding example target volume", "Beispielzielvolumen wird hinzugefügt"));
                }
                self.advance(Puzzle3dSetActiveExampleStage::CreateReferences);
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-example-create-reference", "Adding example reference", "Beispielreferenz wird hinzugefügt"))
            }
            Puzzle3dSetActiveExampleStage::CreateReferences => {
                let items = &target.references;
                let range = Self::take_chunk(&mut self.cursor, items.len());
                if !range.is_empty() {
                    for reference in &items[range] {
                        let value = semio_framework_value::ToValue::to_value(reference);
                        let reference = FromValue::from_value(value).map_err(|_| Fault::from("puzzle3d-set-active-example-reference-malformed"))?;
                        self.push(semio_s_artifact_puzzle_3d_mutations_reference::create_reference::mutation::create_reference(reference, None))?;
                    }
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-example-create-reference", "Adding example reference", "Beispielreferenz wird hinzugefügt"));
                }
                self.advance(Puzzle3dSetActiveExampleStage::CreateCompatibility);
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-example-create-compatibility", "Adding example compatibility", "Beispielkompatibilität wird hinzugefügt"))
            }
            Puzzle3dSetActiveExampleStage::CreateCompatibility => {
                let items = Self::compatibility_rows(target);
                let range = Self::take_chunk(&mut self.cursor, items.len());
                if !range.is_empty() {
                    for row in items[range].iter().cloned() {
                        let row: semio_s_artifact_puzzle_3d_model::Puzzle3dKindCompatibility = FromValue::from_value(row).map_err(|_| Fault::from("puzzle3d-set-active-example-compatibility-malformed"))?;
                        self.push(semio_s_artifact_puzzle_3d_mutations_catalog::connect_kind_compatibility::mutation::connect_kind_compatibility(row.source, row.target, row.bidirectional, row.important, row.specificity, None))?;
                    }
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-example-create-compatibility", "Adding example compatibility", "Beispielkompatibilität wird hinzugefügt"));
                }
                self.advance(Puzzle3dSetActiveExampleStage::Publish);
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-example-publish", "Publishing example", "Beispiel wird veröffentlicht"))
            }
            Puzzle3dSetActiveExampleStage::Publish => {
                self.stage = Puzzle3dSetActiveExampleStage::Complete;
                let example_id = Self::canonical_example_id(command).unwrap_or_default();
                let config_mutations = if config.active_example_id == example_id {
                    Vec::new()
                } else {
                    vec![Puzzle3dConfigMutation::SetActiveExampleId(Puzzle3dConfigSetActiveExampleId{ value: example_id.to_string() })]
                };
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit {
                    artifact_mutations: std::mem::take(&mut self.mutations),
                    config_mutations,
                    ui_scope: puzzle3d_scope(Puzzle3dScopeClass::Chrome),
                    ..Default::default()
                }))
            }
            Puzzle3dSetActiveExampleStage::Complete => Err(Fault::from("puzzle3d-set-active-example-complete-repolled")),
            Puzzle3dSetActiveExampleStage::Closing => Err(Fault::from("puzzle3d-set-active-example-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle3dSetActiveExampleStage::Closing;
        self.close_owners.stage(Puzzle3dSetActiveExampleWorkOwners { mutations: std::mem::take(&mut self.mutations) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
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
        self.stage == Puzzle3dSetActiveExampleStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}
