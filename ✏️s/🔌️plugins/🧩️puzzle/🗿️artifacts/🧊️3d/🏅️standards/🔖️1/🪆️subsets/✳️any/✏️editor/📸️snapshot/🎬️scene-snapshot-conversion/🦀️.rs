//! 🎬️ Scene snapshot conversion: the clipboard fragments, the scene snapshot from and to the document and host snapshots and the scene object conversions. See [`super`].

use super::*;

const PUZZLE3D_CLIPBOARD_SCHEMA: &str = "puzzle.3d.clipboard.v1";

fn puzzle3d_scene_snapshot_from_view(doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>) -> Puzzle3dSceneSnapshot {
    puzzle3d_scene_snapshot_from_play_snapshot(doc.snapshot)
}

pub fn puzzle3d_scene_snapshot_from_play_snapshot(snapshot: &Puzzle3dPlaySnapshot) -> Puzzle3dSceneSnapshot {
    puzzle3d_scene_snapshot_from_document(snapshot.typed())
}

fn puzzle3d_selected_objects(scene_snapshot: &Puzzle3dSceneSnapshot, interaction: &InteractionView<'_>) -> Vec<Puzzle3dObject> {
    puzzle3d_selected_objects_from(&Puzzle3dInteractionSnapshot::from_interaction(interaction), scene_snapshot)
}

pub fn puzzle3d_selected_objects_from(snapshot: &Puzzle3dInteractionSnapshot, scene_snapshot: &Puzzle3dSceneSnapshot) -> Vec<Puzzle3dObject> {
    let ids = snapshot.selected_object_ids();
    let mut objects: Vec<Puzzle3dObject> = scene_snapshot.objects.iter().filter(|object| ids.contains(&object.id)).cloned().collect();
    if objects.is_empty() {
        objects = scene_snapshot.objects.iter().filter(|object| snapshot.selected.iter().any(|id| id == &object.id)).cloned().collect();
    }
    if objects.is_empty() {
        objects = scene_snapshot
            .objects
            .iter()
            .filter(|object| {
                object.vortices.iter().any(|vortex| snapshot.selected.iter().any(|id| id == &vortex.id || id == &puzzle3d_vortex_full_id(&object.id, &vortex.id)))
            })
            .cloned()
            .collect();
    }
    objects
}

/// 📋️ Copies the selected objects as a scene_snapshot-shaped fragment.
pub fn puzzle3d_copy_fragment(doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>, interaction: &InteractionView<'_>) -> Result<ClipboardFragment, ClipboardError> {
    let scene_snapshot = puzzle3d_scene_snapshot_from_view(doc);
    puzzle3d_copy_fragment_from(&scene_snapshot, puzzle3d_selected_objects(&scene_snapshot, interaction))
}

pub fn puzzle3d_copy_fragment_from(scene_snapshot: &Puzzle3dSceneSnapshot, objects: Vec<Puzzle3dObject>) -> Result<ClipboardFragment, ClipboardError> {
    let _ = scene_snapshot;
    if objects.is_empty() {
        return Err(ClipboardError::EmptySelection);
    }
    let mut clip = empty_scene_snapshot();
    clip.schema = PUZZLE3D_CLIPBOARD_SCHEMA.into();
    clip.objects = objects;
    Ok(ClipboardFragment {
        schema: PUZZLE3D_CLIPBOARD_SCHEMA.into(),
        media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Design },
        dsl_text: to_json_string(&puzzle3d_projection_value(semio_framework_value::ToValue::to_value(&clip))),
        pack_bytes: None,
        source_app: PUZZLE3D_PLAY_APP_ID.into(),
        label: format!("{} objects", clip.objects.len()),
    })
}

/// 📋 Clones a copied scene_snapshot fragment with fresh ids, one `create-object` per clone.
pub fn puzzle3d_paste_operations(fragment: &ClipboardFragment, placement: &PastePlacement) -> Result<Vec<Puzzle3dMutation>, ClipboardError> {
    let expected = MediaType { class: MediaClass::ThreeD, form: MediaForm::Design };
    if fragment.media_type != expected {
        return Err(ClipboardError::IncompatibleMediaType(fragment.media_type.clone()));
    }
    let value = parse(&fragment.dsl_text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| ClipboardError::ParseFailed(error.to_string()))?;
    let clip = Puzzle3dSceneSnapshot::from_value(semio_framework_pack_json::to_dsl_value(&value)).map_err(|error| ClipboardError::ParseFailed(error.to_string()))?;
    if clip.objects.is_empty() {
        return Err(ClipboardError::EmptySelection);
    }
    let offset = placement.position.unwrap_or([0.5, 0.5, 0.0]);
    clip.objects
        .into_iter()
        .map(|mut clone| {
            clone.id = next_object_id();
            clone.origin[0] += offset[0];
            clone.origin[1] += offset[1];
            clone.origin[2] += offset[2];
            snapshot_object_from_host_snapshot(&clone).map(|object| semio_s_artifact_puzzle_3d_mutations_object::create_object::mutation::create_object(object, None)).map_err(|error| ClipboardError::ParseFailed(error.to_string()))
        })
        .collect()
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

/// 🌉️ The persisted `Puzzle3dSnapshot` as this app's own structural-twin `Puzzle3dSceneSnapshot`. The
/// twin deliberately does not carry `anchor`, a vortex `label` or an attraction's diagram `x`/`y`, so
/// those are dropped here exactly as the `Value` bridge dropped them (an unknown key is ignored by
/// `FromValue`), and the two untyped catalog members are serialized once instead of per object.
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
        anchor: semio_s_artifact_puzzle_3d_model::Puzzle3dObjectAnchor::default(),
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
    semio_s_artifact_puzzle_3d_model::Puzzle3dVortex { id: vortex.id.clone(), vortex_kind: vortex.vortex_kind.clone(), label: None, position: vortex.position, direction: vortex.direction, radius: vortex.radius, hidden: vortex.hidden, locked: vortex.locked }
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
        x: 0.0,
        y: 0.0,
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
    Puzzle3dVortex { id: vortex.id.clone(), vortex_kind: vortex.vortex_kind.clone(), position: vortex.position, direction: vortex.direction, radius: vortex.radius, hidden: vortex.hidden, locked: vortex.locked }
}

fn scene_attraction_from_snapshot(attraction: &semio_s_artifact_puzzle_3d_model::Puzzle3dAttraction) -> Puzzle3dAttraction {
    Puzzle3dAttraction {
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
