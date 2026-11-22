//! 🎬️ Scene snapshot conversion: the clipboard fragments, the scene snapshot from and to the document and host snapshots and the scene object conversions. See [`super`].

use super::*;
use semio_s_artifact_puzzle_3d_model::Puzzle3dObject as DocumentObject;

const PUZZLE3D_CLIPBOARD_SCHEMA: &str = "puzzle.3d.clipboard.v1";

pub fn puzzle3d_scene_snapshot_from_play_snapshot(snapshot: &Puzzle3dPlaySnapshot) -> Puzzle3dSceneSnapshot {
    puzzle3d_scene_snapshot_from_document(snapshot.typed())
}

/// 🎯️ Borrows selected objects, resolving selected vortices to their parents when needed.
pub fn puzzle3d_selected_objects_from<'a>(snapshot: &Puzzle3dInteractionSnapshot, document: &'a Puzzle3dSnapshot) -> Vec<&'a DocumentObject> {
    if snapshot.selected.is_empty() {
        return Vec::new();
    }
    let ids: HashSet<&str> = snapshot.selected.iter().map(String::as_str).collect();
    let mut objects: Vec<_> = document.objects.iter().filter(|object| ids.contains(object.id.as_str())).collect();
    if objects.is_empty() {
        objects = document
            .objects
            .iter()
            .filter(|object| {
                object.vortices.iter().any(|vortex| ids.contains(vortex.id.as_str()) || ids.contains(puzzle3d_vortex_full_id(&object.id, &vortex.id).as_str()))
            })
            .collect();
    }
    objects
}

/// 📋️ Copies the selected objects as a scene_snapshot-shaped fragment.
pub fn puzzle3d_copy_fragment(doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>, interaction: &InteractionView<'_>) -> Result<ClipboardFragment, ClipboardError> {
    let document = doc.snapshot.typed();
    let marks = Puzzle3dInteractionSnapshot::from_interaction(interaction);
    puzzle3d_copy_fragment_from(document, &puzzle3d_selected_objects_from(&marks, document))
}

/// 🖇️ Projects only the selected objects and their internal attractions into the clipboard schema.
pub fn puzzle3d_copy_fragment_from(document: &Puzzle3dSnapshot, objects: &[&DocumentObject]) -> Result<ClipboardFragment, ClipboardError> {
    if objects.is_empty() {
        return Err(ClipboardError::EmptySelection);
    }
    let vortex_ids: HashSet<String> = objects.iter().flat_map(|object| {
        object.vortices.iter().map(|vortex| puzzle3d_vortex_full_id(&object.id, &vortex.id))
    }).collect();
    let mut clip = empty_scene_snapshot();
    clip.schema = PUZZLE3D_CLIPBOARD_SCHEMA.into();
    clip.attractions = document.attractions.iter().filter(|attraction| {
        vortex_ids.contains(&attraction.attracting) && vortex_ids.contains(&attraction.attracted)
    }).map(scene_attraction_from_snapshot).collect();
    clip.objects = objects.iter().map(|object| scene_object_from_snapshot(object)).collect();
    Ok(ClipboardFragment {
        schema: PUZZLE3D_CLIPBOARD_SCHEMA.into(),
        media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Design },
        dsl_text: to_json_string(&puzzle3d_projection_value(semio_framework_value::ToValue::to_value(&clip))),
        pack_bytes: None,
        source_app: PUZZLE3D_PLAY_APP_ID.into(),
        label: format!("{} objects", clip.objects.len()),
    })
}

/// 📋️ Rebuilds the copied subgraph with new identities and a shared placement translation.
pub fn puzzle3d_paste_operations(destination: &Puzzle3dSnapshot, fragment: &ClipboardFragment, placement: &PastePlacement) -> Result<Vec<Puzzle3dMutation>, ClipboardError> {
    let expected = MediaType { class: MediaClass::ThreeD, form: MediaForm::Design };
    if fragment.media_type != expected {
        return Err(ClipboardError::IncompatibleMediaType(fragment.media_type.clone()));
    }
    let value = parse(&fragment.dsl_text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| ClipboardError::ParseFailed(error.to_string()))?;
    let clip = Puzzle3dSceneSnapshot::from_value(semio_framework_pack_json::to_dsl_value(&value)).map_err(|error| ClipboardError::ParseFailed(error.to_string()))?;
    if clip.objects.is_empty() {
        return Err(ClipboardError::EmptySelection);
    }
    let offset = placement.point_translation(clip.objects.iter().map(|object| object.origin), [0.5, 0.5, 0.0]);
    let destination_objects: HashSet<&str> = destination.objects.iter().map(|object| object.id.as_str()).collect();
    let destination_vortices: HashSet<&str> = destination.objects.iter().flat_map(|object| &object.vortices)
        .map(|vortex| vortex.id.as_str()).filter(|id| id.contains(':')).collect();
    let mut object_ids: HashSet<String> = clip.objects.iter().map(|object| object.id.clone()).collect();
    let mut vortex_ids: HashSet<String> = clip.objects.iter().flat_map(|object| &object.vortices)
        .filter(|vortex| vortex.id.contains(':')).map(|vortex| vortex.id.clone()).collect();
    let mut endpoint_ids = HashMap::new();
    let mut operations = Vec::with_capacity(clip.objects.len() + clip.attractions.len());
    for mut clone in clip.objects {
        let id = std::iter::repeat_with(next_object_id)
            .find(|id| !destination_objects.contains(id.as_str()) && object_ids.insert(id.clone()))
            .expect("a free object id");
        let source_scope = format!("{}:", clone.id);
        for vortex in &mut clone.vortices {
            let source = puzzle3d_vortex_full_id(&clone.id, &vortex.id);
            let local = source.strip_prefix(&source_scope).unwrap_or(&source);
            let preferred = format!("{id}:{local}");
            let target = std::iter::once(preferred.clone()).chain((2usize..).map(|serial| format!("{preferred}-{serial}")))
                .find(|id| !destination_vortices.contains(id.as_str()) && vortex_ids.insert(id.clone()))
                .expect("a free vortex id");
            if vortex.id.contains(':') || target != preferred {
                vortex.id = target.clone();
            }
            if endpoint_ids.insert(source, target).is_some() {
                return Err(ClipboardError::ParseFailed("duplicate vortex identity".into()));
            }
        }
        clone.id = id;
        for (origin, offset) in clone.origin.iter_mut().zip(offset) {
            *origin += offset;
        }
        let object = snapshot_object_from_host_snapshot(&clone).map_err(|error| ClipboardError::ParseFailed(error.to_string()))?;
        operations.push(semio_s_artifact_puzzle_3d_mutations_object::create_object::mutation::create_object(object, None));
    }
    let destination_attractions: HashSet<&str> = destination.attractions.iter().map(|attraction| attraction.id.as_str()).collect();
    let mut attraction_ids: HashSet<String> = clip.attractions.iter().map(|attraction| attraction.id.clone()).collect();
    for attraction in clip.attractions {
        let (Some(attracting), Some(attracted)) = (endpoint_ids.get(&attraction.attracting), endpoint_ids.get(&attraction.attracted)) else {
            return Err(ClipboardError::ParseFailed("an attraction endpoint is outside the fragment".into()));
        };
        let base = format!("attraction-{attracting}-{attracted}");
        let id = std::iter::once(base.clone()).chain((2usize..).map(|serial| format!("{base}-{serial}")))
            .find(|id| !destination_attractions.contains(id.as_str()) && attraction_ids.insert(id.clone()))
            .expect("a free attraction id");
        operations.push(semio_s_artifact_puzzle_3d_mutations::connect_vortices(
            id, attracting.clone(), attracted.clone(), attraction.gap, attraction.shift, attraction.rise,
            attraction.rotation, attraction.turn, attraction.tilt, attraction.x, attraction.y, None,
        ));
    }
    Ok(operations)
}

pub fn default_scene_snapshot() -> Puzzle3dSceneSnapshot {
    CONCRETE_FOREST_EXAMPLE_SNAPSHOT.clone()
}

pub fn nakagin_scene_snapshot() -> Puzzle3dSceneSnapshot {
    NAKAGIN_EXAMPLE_SNAPSHOT.clone()
}

/// 👁️ Builds the native host view from the editor-owned intrinsic projection.
pub fn puzzle3d_projection_value<T:std::borrow::Borrow<semio_framework_value::DslValue>>(value:T)->Value{
    semio_framework_pack_json::from_dsl_value(value.borrow())
}

/// 🧾️ Builds the native scene directly from canonical typed authority.
pub fn scene_from_snapshot(document: &Puzzle3dSnapshot, runtime: Puzzle3dRuntime, active_utility: &str) -> Puzzle3dScene {
    Puzzle3dScene { scene_snapshot: puzzle3d_scene_snapshot_from_document(document), runtime, active_utility: active_utility.to_string() }
}

/// 🌉️ Projects the persisted document into the renderer's scene schema.
pub fn puzzle3d_scene_snapshot_from_document(document: &Puzzle3dSnapshot) -> Puzzle3dSceneSnapshot {
    Puzzle3dSceneSnapshot {
        schema: document.schema.clone(),
        domain: document.domain.clone(),
        meta: Puzzle3dSceneMetadata { kind_catalogs: document.meta.kind_catalogs.as_ref().map(semio_framework_value::ToValue::to_value), kind_compatibility: Some(semio_framework_value::ToValue::to_value(&document.meta.kind_compatibility)) },
        objects: document.objects.iter().map(scene_object_from_snapshot).collect(),
        attractions: document.attractions.iter().map(scene_attraction_from_snapshot).collect(),
        target_volumes: document.target_volumes.iter().map(scene_target_volume_from_snapshot).collect(),
        references: document.references.iter().map(scene_reference_from_snapshot).collect(),
    }
}

/// 🧬️ Inverse of [`puzzle3d_scene_snapshot_from_document`] — the document delta must compare typed snapshots,
/// Native-only scale and catalog views must admit their typed fields or refuse the operation.
pub fn puzzle3d_snapshot_from_host_snapshot(scene_snapshot: &Puzzle3dSceneSnapshot) -> Result<Puzzle3dSnapshot,semio_framework_value::ValueError> {
    Ok(Puzzle3dSnapshot {
        schema: scene_snapshot.schema.clone(),
        domain: scene_snapshot.domain.clone(),
        meta: semio_s_artifact_puzzle_3d_model::Puzzle3dMeta {
            kind_catalogs: scene_snapshot.meta.kind_catalogs.clone().map(FromValue::from_value).transpose()?,
            kind_compatibility: match scene_snapshot.meta.kind_compatibility.clone(){Some(value)=>FromValue::from_value(value)?,None=>Default::default()},
        },
        objects: scene_snapshot.objects.iter().map(snapshot_object_from_host_snapshot).collect::<Result<_,_>>()?,
        attractions: scene_snapshot.attractions.iter().map(snapshot_attraction_from_host_snapshot).collect(),
        target_volumes: scene_snapshot.target_volumes.iter().map(snapshot_target_volume_from_host_snapshot).collect::<Result<_,_>>()?,
        references: scene_snapshot.references.iter().map(snapshot_reference_from_host_snapshot).collect(),
    })
}

fn snapshot_object_from_host_snapshot(object: &Puzzle3dObject) -> Result<semio_s_artifact_puzzle_3d_model::Puzzle3dObject,semio_framework_value::ValueError> {
    Ok(semio_s_artifact_puzzle_3d_model::Puzzle3dObject {
        id: object.id.clone(),
        label: object.label.clone(),
        object_kind: object.object_kind.clone(),
        anchor: object.anchor,
        origin: object.origin,
        orientation: object.orientation,
        scale: object.scale.clone().map(FromValue::from_value).transpose()?,
        mesh_url: object.mesh_url.clone(),
        vortices: object.vortices.iter().map(snapshot_vortex_from_host_snapshot).collect(),
        hidden: object.hidden,
        locked: object.locked,
    })
}

fn snapshot_vortex_from_host_snapshot(vortex: &Puzzle3dVortex) -> semio_s_artifact_puzzle_3d_model::Puzzle3dVortex {
    semio_s_artifact_puzzle_3d_model::Puzzle3dVortex { id: vortex.id.clone(), vortex_kind: vortex.vortex_kind.clone(), label: vortex.label.clone(), position: vortex.position, direction: vortex.direction, radius: vortex.radius, hidden: vortex.hidden, locked: vortex.locked }
}

fn snapshot_attraction_from_host_snapshot(attraction: &Puzzle3dAttraction) -> semio_s_artifact_puzzle_3d_model::Puzzle3dAttraction {
    semio_s_artifact_puzzle_3d_model::Puzzle3dAttraction {
        id: attraction.id.clone(),
        attracting: attraction.attracting.clone(),
        attracted: attraction.attracted.clone(),
        gap: attraction.gap,
        shift: attraction.shift,
        rise: attraction.rise,
        rotation: attraction.rotation,
        turn: attraction.turn,
        tilt: attraction.tilt,
        x: attraction.x,
        y: attraction.y,
    }
}

fn snapshot_target_volume_from_host_snapshot(volume: &Puzzle3dTargetVolume) -> Result<semio_s_artifact_puzzle_3d_model::Puzzle3dTargetVolume,semio_framework_value::ValueError> {
    Ok(semio_s_artifact_puzzle_3d_model::Puzzle3dTargetVolume { id: volume.id.clone(), origin: volume.origin, orientation: volume.orientation, scale: volume.scale.clone().map(FromValue::from_value).transpose()?, hidden: volume.hidden, locked: volume.locked })
}

fn snapshot_reference_from_host_snapshot(reference: &Puzzle3dReference) -> semio_s_artifact_puzzle_3d_model::Puzzle3dReference {
    semio_s_artifact_puzzle_3d_model::Puzzle3dReference {
        id: reference.id.clone(),
        source: semio_s_artifact_puzzle_3d_model::Puzzle3dReferenceSource { url: reference.source.url.clone(), media_kind: reference.source.media_kind.clone() },
        origin: reference.origin,
        width_world: reference.width_world,
        locked: reference.locked,
        hidden: reference.hidden,
    }
}

pub fn scene_object_from_snapshot(object: &semio_s_artifact_puzzle_3d_model::Puzzle3dObject) -> Puzzle3dObject {
    Puzzle3dObject {
        anchor: object.anchor,
        id: object.id.clone(),
        label: object.label.clone(),
        object_kind: object.object_kind.clone(),
        origin: object.origin,
        orientation: object.orientation,
        scale: object.scale.as_ref().map(semio_framework_value::ToValue::to_value),
        mesh_url: object.mesh_url.clone(),
        vortices: object.vortices.iter().map(scene_vortex_from_snapshot).collect(),
        hidden: object.hidden,
        locked: object.locked,
    }
}

fn scene_vortex_from_snapshot(vortex: &semio_s_artifact_puzzle_3d_model::Puzzle3dVortex) -> Puzzle3dVortex {
    Puzzle3dVortex { id: vortex.id.clone(), vortex_kind: vortex.vortex_kind.clone(), label: vortex.label.clone(), position: vortex.position, direction: vortex.direction, radius: vortex.radius, hidden: vortex.hidden, locked: vortex.locked }
}

fn scene_attraction_from_snapshot(attraction: &semio_s_artifact_puzzle_3d_model::Puzzle3dAttraction) -> Puzzle3dAttraction {
    Puzzle3dAttraction {
        x: attraction.x,
        y: attraction.y,
        id: attraction.id.clone(),
        attracting: attraction.attracting.clone(),
        attracted: attraction.attracted.clone(),
        gap: attraction.gap,
        shift: attraction.shift,
        rise: attraction.rise,
        rotation: attraction.rotation,
        turn: attraction.turn,
        tilt: attraction.tilt,
    }
}

fn scene_target_volume_from_snapshot(volume: &semio_s_artifact_puzzle_3d_model::Puzzle3dTargetVolume) -> Puzzle3dTargetVolume {
    Puzzle3dTargetVolume { id: volume.id.clone(), origin: volume.origin, orientation: volume.orientation, scale: volume.scale.as_ref().map(semio_framework_value::ToValue::to_value), hidden: volume.hidden, locked: volume.locked }
}

fn scene_reference_from_snapshot(reference: &semio_s_artifact_puzzle_3d_model::Puzzle3dReference) -> Puzzle3dReference {
    Puzzle3dReference {
        id: reference.id.clone(),
        source: Puzzle3dReferenceSource { url: reference.source.url.clone(), media_kind: reference.source.media_kind.clone() },
        origin: reference.origin,
        width_world: reference.width_world,
        locked: reference.locked,
        hidden: reference.hidden,
    }
}
