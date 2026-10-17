//! ➕️ Add object kind work: the staged addition of one object kind, decoded and checked against the catalog, with its representation and vortex, then published.

use super::*;

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

pub(super) struct Puzzle3dAddObjectKindPayload {
    pub(super) kind_id: String,
    pub(super) origin: [f64; 3],
}

/// ♻️ The owners one `Puzzle3dAddObjectKindWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
pub(super) struct Puzzle3dAddObjectKindWorkOwners {
    catalog_mutation: Option<Puzzle3dMutation>,
    pub(super) mutation: Option<Puzzle3dMutation>,
    pub(super) vortices: Vec<semio_s_artifact_puzzle_3d_model::Puzzle3dVortex>,
    mesh_url: Option<String>,
    pub(super) object_id: Option<String>,
    payload: Option<Puzzle3dAddObjectKindPayload>,
}

pub struct Puzzle3dAddObjectKindWork {
    pub(super) stage: Puzzle3dAddObjectKindStage,
    kind_cursor: usize,
    representation_cursor: usize,
    vortex_cursor: usize,
    kind_index: Option<usize>,
    payload: Option<Puzzle3dAddObjectKindPayload>,
    pub(super) object_id: Option<String>,
    mesh_url: Option<String>,
    pub(super) vortices: Vec<semio_s_artifact_puzzle_3d_model::Puzzle3dVortex>,
    catalog_mutation: Option<Puzzle3dMutation>,
    pub(super) mutation: Option<Puzzle3dMutation>,
    pub(super) close_owners: semio_s_plugin_puzzle_jobs::WorkClosing<Puzzle3dAddObjectKindWorkOwners>,
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

    pub(super) fn object_id(snapshot: &Puzzle3dPlaySnapshot, payload: &Puzzle3dAddObjectKindPayload) -> String {
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
