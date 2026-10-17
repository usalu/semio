//! 🌉️ Engine bridge: the typed translation of the transient scene into the precompute `SceneConfig` and the sync and drive calls that feed one precompute session.

use super::*;

/// 🎯️ Bridges this app's own document model into `⚙️engine`'s `SceneConfig` wire shape — schema
/// translation between two independently-evolved Rust types, not a wasm-bindgen boundary. Built
/// TYPED, field by field: routing the whole document through a `dsl::DslValue` tree and back cost a
/// measured 3.5 ms per sync on the 180-object Nakagin document (0.5 ms to build the tree, 3.0 ms to
/// re-parse it), four times this artifact's own per-step budget, for a translation whose every field
/// is a move or a copy. Only the two genuinely untyped members of `Puzzle3dSceneMetadata`
/// (`kindCatalogs`/`kindCompatibility`, which this app's document model keeps as raw `DslValue`)
/// still decode through `FromValue`, and a malformed one refuses the whole scene exactly as the
/// all-`FromValue` shape did. `#[cfg(test)] scene_config_value` below is the differential oracle this
/// is checked against.
pub fn scene_config(envelope: &Puzzle3dScene) -> Option<semio_s_artifact_puzzle_3d_model::SceneConfig> {
    let meta = &envelope.scene_snapshot.meta;
    let kind_catalogs = match meta.kind_catalogs.as_ref() {
        None | Some(semio_framework_value::DslValue::Null) => None,
        Some(catalogs) => Some(FromValue::from_value(catalogs.clone()).ok()?),
    };
    let kind_compatibility = match meta.kind_compatibility.as_ref() {
        None => Vec::new(),
        Some(entries) => FromValue::from_value(entries.clone()).ok()?,
    };
    Some(semio_s_artifact_puzzle_3d_model::SceneConfig {
        scene_snapshot: semio_s_artifact_puzzle_3d_model::EngineSceneSnapshot {
            objects: envelope.scene_snapshot.objects.iter().map(engine_scene_object).collect(),
            attractions: envelope.scene_snapshot.attractions.iter().map(engine_attraction_props).collect(),
            target_volumes: envelope.scene_snapshot.target_volumes.iter().map(engine_world_volume_props).collect(),
        },
        kind_catalogs,
        kind_compatibility,
        contact_tolerance: envelope.runtime.contact_tolerance,
        seed: 1,
        host_rules: semio_s_artifact_puzzle_3d_model::BrushHostRules::default(),
        weights: semio_s_artifact_puzzle_3d_model::BrushKindWeights {
            object_weights: envelope.runtime.object_kind_weights.iter().map(|(kind, weight)| (kind.clone(), *weight)).collect(),
            vortex_weights: envelope.runtime.vortex_kind_weights.iter().map(|(kind, weight)| (kind.clone(), *weight)).collect(),
        },
    })
}

/// 🧱️ One document object as the engine sees it. `anchor` has no counterpart in this app's own
/// document model, so it takes the same default the JSON bridge gave it.
fn engine_scene_object(object: &Puzzle3dObject) -> semio_s_artifact_puzzle_3d_model::EngineSceneObject {
    semio_s_artifact_puzzle_3d_model::EngineSceneObject {
        id: object.id.clone(),
        object_kind: object.object_kind.clone(),
        anchor: semio_s_artifact_puzzle_3d_model::Puzzle3dObjectAnchor::default(),
        mesh_url: object.mesh_url.clone(),
        origin: object.origin,
        orientation: object.orientation,
        scale: object.scale.clone(),
        vortices: object.vortices.iter().map(engine_vortex_props).collect(),
    }
}

fn engine_vortex_props(vortex: &Puzzle3dVortex) -> semio_s_artifact_puzzle_3d_model::VortexProps {
    semio_s_artifact_puzzle_3d_model::VortexProps { id: vortex.id.clone(), vortex_kind: vortex.vortex_kind.clone(), position: vortex.position, direction: vortex.direction }
}

/// 🔗️ One attraction as the engine sees it. `x`/`y` are diagram-only coordinates this app's own
/// document model does not carry, so they take the same default the JSON bridge gave them.
fn engine_attraction_props(attraction: &Puzzle3dAttraction) -> semio_s_artifact_puzzle_3d_model::AttractionProps {
    semio_s_artifact_puzzle_3d_model::AttractionProps {
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

fn engine_world_volume_props(volume: &Puzzle3dTargetVolume) -> semio_s_artifact_puzzle_3d_model::WorldVolumeProps {
    semio_s_artifact_puzzle_3d_model::WorldVolumeProps { id: volume.id.clone(), origin: volume.origin, orientation: volume.orientation, scale: volume.scale.clone() }
}

/// 🌉️ The all-`DslValue` bridge `scene_config` replaced, retained as its differential oracle: the
/// derived `ToValue`/`FromValue` machinery is an independent implementation of the same translation,
/// so `puzzle3d_typed_scene_config_matches_the_value_bridge_for_every_example` can prove the typed
/// construction agrees with it on every shipped document.
#[cfg(any(test, feature = "testkit"))]
pub fn scene_config_value(envelope: &Puzzle3dScene) -> semio_framework_value::DslValue {
    semio_framework_value::DslValue::object([
        (
            "sceneSnapshot".to_string(),
            semio_framework_value::DslValue::object([
                ("objects".to_string(), semio_framework_value::ToValue::to_value(&envelope.scene_snapshot.objects)),
                ("attractions".to_string(), semio_framework_value::ToValue::to_value(&envelope.scene_snapshot.attractions)),
                ("targetVolumes".to_string(), semio_framework_value::ToValue::to_value(&envelope.scene_snapshot.target_volumes)),
            ]),
        ),
        ("kindCatalogs".to_string(), envelope.scene_snapshot.meta.kind_catalogs.clone().unwrap_or(semio_framework_value::DslValue::Null)),
        ("kindCompatibility".to_string(), envelope.scene_snapshot.meta.kind_compatibility.clone().unwrap_or_else(|| semio_framework_value::DslValue::Array(Vec::new()))),
        ("contactTolerance".to_string(), semio_framework_value::DslValue::float(envelope.runtime.contact_tolerance)),
        ("seed".to_string(), semio_framework_value::DslValue::uint(1)),
        ("weights".to_string(), semio_framework_value::DslValue::object([("objectWeights".to_string(), semio_framework_value::ToValue::to_value(&envelope.runtime.object_kind_weights)), ("vortexWeights".to_string(), semio_framework_value::ToValue::to_value(&envelope.runtime.vortex_kind_weights))])),
    ])
}

/// 🧊️ Scales the unit box fallback (`mesh_from_kind` extent 1.0) past the collision engine's minimum
/// brush mesh extent (2.0), otherwise its registration is a silent no-operation and brush candidates
/// never populate before a real GLB arrives.
const PUZZLE3D_FALLBACK_MESH_SCALE: f32 = 4.0;

/// 🧊️ The scaled box every mesh identity collides as until its real geometry is installed.
pub fn puzzle3d_fallback_mesh_buffers() -> (Vec<f32>, Vec<u32>) {
    let fallback = mesh_from_kind(PUZZLE3D_FALLBACK_MESH_KIND);
    (fallback.positions.iter().map(|value| value * PUZZLE3D_FALLBACK_MESH_SCALE).collect(), fallback.indices)
}

/// 🧊️ Pushes the current scene into the precompute session. The engine compares the decoded scene
/// structurally and no-operates on a resync, so this is the one call a caller makes per scene — and it
/// is deliberately the LAST half of a sync: installing a mesh identity rebuilds the fill preparation
/// and the brush queue against whatever scene is installed, so seeding first and pushing once costs one
/// rebuild instead of one per mesh (a measured 0.6–4.4 ms each, twelve of them on Nakagin).
pub fn sync_precompute_scene(session: &mut Puzzle3dPrecomputeSession, envelope: &Puzzle3dScene) {
    if let Some(scene) = scene_config(envelope) {
        push_precompute_scene(session, scene);
    }
}

/// 🧊️ Installs an already-built engine scene. Split from `scene_config` so a staged caller pays the
/// document-shaped translation (a measured 1.3 ms on Nakagin) and the engine's own scene install (0.8 ms,
/// it rebuilds the fill preparation and re-arms the brush broad phase) in two separate bounded turns.
pub fn push_precompute_scene(session: &mut Puzzle3dPrecomputeSession, scene: semio_s_artifact_puzzle_3d_model::SceneConfig) {
    let _ = session.dispatch(Puzzle3dEngineCommand::SetScene { scene });
}

/// 🥽️ Seeds the scaled box fallback for ONE mesh identity the session holds no geometry for yet, so
/// a real GLB registered earlier via `registerBrushMesh` survives every resync. Answers whether it
/// registered one, i.e. whether the caller should come back for the next; one registration costs a
/// measured 0.6 ms and the 180-object Nakagin document carries twelve of them, so a staged caller
/// spends one bounded turn on each instead of 8 ms on all of them at once.
pub fn seed_one_precompute_mesh_fallback(session: &mut Puzzle3dPrecomputeSession, envelope: &Puzzle3dScene) -> bool {
    let (fallback_positions, fallback_indices) = puzzle3d_fallback_mesh_buffers();
    if !session.has_mesh(PUZZLE3D_FALLBACK_MESH_KIND) {
        session.register_mesh_fallback(PUZZLE3D_FALLBACK_MESH_KIND, &fallback_positions, &fallback_indices);
        return true;
    }
    let Some(url) = collect_mesh_urls(&envelope.scene_snapshot).into_iter().find(|url| !session.has_mesh(url)) else {
        return false;
    };
    session.register_mesh_fallback(&url, &fallback_positions, &fallback_indices);
    true
}

/// 🧊️ The whole sync, for the callers that are not themselves step-bounded (render, the restored
/// session, the brush lane driver): every owed mesh fallback, then the scene.
pub fn sync_precompute_session(session: &mut Puzzle3dPrecomputeSession, envelope: &Puzzle3dScene) {
    while seed_one_precompute_mesh_fallback(session, envelope) {}
    sync_precompute_scene(session, envelope);
}

pub fn sync_precompute_weights(session: &mut Puzzle3dPrecomputeSession, envelope: &Puzzle3dScene) {
    let object_weights = envelope.runtime.object_kind_weights.iter().map(|(k, v)| (k.clone(), *v)).collect();
    let vortex_weights = envelope.runtime.vortex_kind_weights.iter().map(|(k, v)| (k.clone(), *v)).collect();
    let _ = session.dispatch(Puzzle3dEngineCommand::UpdateKindWeights { object_weights, vortex_weights });
}

/// ⏱️ Bounded to one small chunk per call — `handle` runs synchronously on the UI thread, so a large
/// per-call budget here is exactly what froze the UI: hundreds of Monte-Carlo collision task units, blocking.
pub fn drive_precompute(session: &mut Puzzle3dPrecomputeSession, envelope: &Puzzle3dScene) {
    sync_precompute_session(session, envelope);
    session.precompute_step(8);
}
