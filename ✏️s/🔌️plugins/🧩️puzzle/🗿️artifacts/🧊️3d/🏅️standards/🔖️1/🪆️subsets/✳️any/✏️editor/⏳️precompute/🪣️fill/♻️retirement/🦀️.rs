//! ♻️ Retirement: the incremental release of every owner a `FillBuilder` still holds when it closes, one bounded unit per call.

use super::*;

pub(super) enum FillRetiredOwner {
    String(String),
    EngineSceneObject(EngineSceneObject),
    Attraction(AttractionProps),
    WorldVolume(WorldVolumeProps),
    Payload(BrushPlacePayload),
    Placed(PlacedCollisionEntry),
    Candidate(BrushCompatibleCandidate),
    Target(BrushFillVortexTarget),
    PreviewState(BrushPreviewState),
    ObjectKind(ObjectKind),
    VortexKind(VortexKindCatalog),
    CableKind(CableKindCatalog),
    Compat(KindCompatEntry),
    CandidateCache(String, Vec<BrushCompatibleCandidate>),
    Mesh(String, CollisionBody),
    Spatial(CollisionIndexRejectedOwner),
}

fn retire_string(value: &mut String) -> bool {
    if value.capacity() == 0 {
        return true;
    }
    drop(std::mem::take(value));
    false
}

fn retire_option_string(value: &mut Option<String>) -> bool {
    let Some(string) = value.as_mut() else { return true };
    if !retire_string(string) {
        return false;
    }
    value.take();
    false
}

fn retire_dsl_one(value: &mut semio_framework_value::DslValue, depth: usize) -> bool {
    if depth > 16 {
        return false;
    }
    match value {
        semio_framework_value::DslValue::String(string) => {
            if !retire_string(string) {
                return false;
            }
            *value = semio_framework_value::DslValue::Null;
            false
        }
        semio_framework_value::DslValue::Bytes(bytes) => {
            if bytes.capacity() != 0 {
                drop(std::mem::take(bytes));
                return false;
            }
            *value = semio_framework_value::DslValue::Null;
            false
        }
        semio_framework_value::DslValue::Array(values) => {
            if let Some(child) = values.last_mut() {
                if !retire_dsl_one(child, depth + 1) {
                    return false;
                }
                values.pop();
                return false;
            }
            if values.capacity() != 0 {
                drop(std::mem::take(values));
                return false;
            }
            *value = semio_framework_value::DslValue::Null;
            false
        }
        semio_framework_value::DslValue::Object(values) => {
            if let Some((key, child)) = values.last_mut() {
                if !retire_string(key) || !retire_dsl_one(child, depth + 1) {
                    return false;
                }
                values.pop();
                return false;
            }
            if values.capacity() != 0 {
                drop(std::mem::take(values));
                return false;
            }
            *value = semio_framework_value::DslValue::Null;
            false
        }
        semio_framework_value::DslValue::Null | semio_framework_value::DslValue::Bool(_) | semio_framework_value::DslValue::Number(_) => true,
    }
}

fn retire_option_dsl(value: &mut Option<semio_framework_value::DslValue>) -> bool {
    let Some(dsl) = value.as_mut() else { return true };
    if !retire_dsl_one(dsl, 0) {
        return false;
    }
    value.take();
    false
}

fn retire_scene_object(value: &mut EngineSceneObject) -> bool {
    if !retire_string(&mut value.id) || !retire_option_string(&mut value.object_kind) || !retire_option_string(&mut value.mesh_url) || !retire_option_dsl(&mut value.scale) {
        return false;
    }
    if let Some(vortex) = value.vortices.last_mut() {
        if !retire_string(&mut vortex.id) || !retire_option_string(&mut vortex.vortex_kind) {
            return false;
        }
        value.vortices.pop();
        return false;
    }
    if value.vortices.capacity() != 0 {
        drop(std::mem::take(&mut value.vortices));
        return false;
    }
    true
}

fn retire_attraction(value: &mut AttractionProps) -> bool {
    retire_string(&mut value.id) && retire_string(&mut value.attracting) && retire_string(&mut value.attracted)
}

fn retire_world_volume(value: &mut WorldVolumeProps) -> bool {
    retire_string(&mut value.id) && retire_option_dsl(&mut value.scale)
}

fn retire_payload(value: &mut BrushPlacePayload) -> bool {
    retire_string(&mut value.target_vortex_full_id) && retire_string(&mut value.object_kind_id) && retire_option_dsl(&mut value.scale)
}

fn retire_candidate(value: &mut BrushCompatibleCandidate) -> bool {
    retire_string(&mut value.object_kind_id)
}

fn retire_target(value: &mut BrushFillVortexTarget) -> bool {
    retire_string(&mut value.full_id) && retire_string(&mut value.object_id) && retire_option_string(&mut value.object_kind) && retire_option_string(&mut value.vortex_kind)
}

fn retire_preview_state(value: &mut BrushPreviewState) -> bool {
    retire_string(&mut value.target_vortex_full_id) && retire_string(&mut value.object_kind_id) && retire_string(&mut value.mesh_url) && retire_option_dsl(&mut value.scale)
}

fn retire_object_kind(value: &mut ObjectKind) -> bool {
    if !retire_string(&mut value.id) || !retire_option_dsl(&mut value.scale) {
        return false;
    }
    if let Some(representation) = value.representations.last_mut() {
        if !retire_string(&mut representation.id)
            || !retire_string(&mut representation.name)
            || !retire_string(&mut representation.url)
            || !retire_string(&mut representation.mime)
            || !retire_option_string(&mut representation.lod)
            || !retire_string(&mut representation.description)
        {
            return false;
        }
        if let Some(tag) = representation.tags.last_mut() {
            if !retire_string(tag) {
                return false;
            }
            representation.tags.pop();
            return false;
        }
        if representation.tags.capacity() != 0 {
            drop(std::mem::take(&mut representation.tags));
            return false;
        }
        value.representations.pop();
        return false;
    }
    if value.representations.capacity() != 0 {
        drop(std::mem::take(&mut value.representations));
        return false;
    }
    if let Some(vortex) = value.vortices.last_mut() {
        if !retire_string(&mut vortex.id) || !retire_string(&mut vortex.name) || !retire_string(&mut vortex.label) || !retire_string(&mut vortex.description) || !retire_string(&mut vortex.icon) || !retire_option_string(&mut vortex.vortex_kind) {
            return false;
        }
        value.vortices.pop();
        return false;
    }
    if value.vortices.capacity() != 0 {
        drop(std::mem::take(&mut value.vortices));
        return false;
    }
    true
}

pub(super) fn retire_retained_owner(owner: &mut FillRetiredOwner) -> bool {
    match owner {
        FillRetiredOwner::String(value) => retire_string(value),
        FillRetiredOwner::EngineSceneObject(value) => retire_scene_object(value),
        FillRetiredOwner::Attraction(value) => retire_attraction(value),
        FillRetiredOwner::WorldVolume(value) => retire_world_volume(value),
        FillRetiredOwner::Payload(value) => retire_payload(value),
        FillRetiredOwner::Placed(value) => retire_string(&mut value.object_id) && retire_string(&mut value.mesh_url),
        FillRetiredOwner::Candidate(value) => retire_candidate(value),
        FillRetiredOwner::Target(value) => retire_target(value),
        FillRetiredOwner::PreviewState(value) => retire_preview_state(value),
        FillRetiredOwner::ObjectKind(value) => retire_object_kind(value),
        FillRetiredOwner::VortexKind(value) => {
            if !retire_string(&mut value.id)
                || !retire_option_string(&mut value.code)
                || !retire_option_string(&mut value.label)
                || !retire_string(&mut value.description)
                || !retire_string(&mut value.icon)
                || !retire_string(&mut value.color)
                || !retire_option_string(&mut value.default_cable_kind)
            {
                return false;
            }
            if let Some(entry) = value.compatible_with.last_mut() {
                if !retire_string(entry) {
                    return false;
                }
                value.compatible_with.pop();
                return false;
            }
            if value.compatible_with.capacity() != 0 {
                drop(std::mem::take(&mut value.compatible_with));
                return false;
            }
            true
        }
        FillRetiredOwner::CableKind(value) => retire_string(&mut value.id) && retire_option_string(&mut value.default_attraction_kind),
        FillRetiredOwner::Compat(value) => retire_string(&mut value.source) && retire_string(&mut value.target) && retire_option_string(&mut value.specificity),
        FillRetiredOwner::CandidateCache(key, values) => {
            if !retire_string(key) {
                return false;
            }
            if let Some(value) = values.last_mut() {
                if !retire_candidate(value) {
                    return false;
                }
                values.pop();
                return false;
            }
            if values.capacity() != 0 {
                drop(std::mem::take(values));
                return false;
            }
            true
        }
        FillRetiredOwner::Mesh(key, body) => {
            if !retire_string(key) {
                return false;
            }
            if body.parts.pop().is_some() {
                return false;
            }
            if body.parts.capacity() != 0 {
                drop(std::mem::take(&mut body.parts));
                return false;
            }
            true
        }
        FillRetiredOwner::Spatial(owner) => owner.retire_one(),
    }
}

fn release_vec_backing<T>(values: &mut Vec<T>) -> bool {
    if values.capacity() == 0 {
        return false;
    }
    debug_assert!(values.is_empty());
    drop(std::mem::take(values));
    true
}

pub(super) fn take_scene_owner(value: &mut FixedSceneOwner, current: &mut Option<FillRetiredOwner>) -> bool {
    if let Some(value) = value.objects.pop() {
        *current = Some(FillRetiredOwner::EngineSceneObject(value));
        return true;
    }
    if value.objects.retire_backing() {
        return true;
    }
    if let Some(value) = value.attractions.pop() {
        *current = Some(FillRetiredOwner::Attraction(value));
        return true;
    }
    if value.attractions.retire_backing() {
        return true;
    }
    if let Some(value) = value.target_volumes.pop() {
        *current = Some(FillRetiredOwner::WorldVolume(value));
        return true;
    }
    value.target_volumes.retire_backing()
}

pub(super) fn take_sequence_owner(fill: &mut FillBuilder, current: &mut Option<FillRetiredOwner>) -> bool {
    if let Some(value) = fill.sequence.pop() {
        *current = Some(FillRetiredOwner::Payload(value));
        return true;
    }
    if release_vec_backing(&mut fill.sequence) {
        return true;
    }
    if let Some(value) = fill.appended_objects.pop() {
        *current = Some(FillRetiredOwner::EngineSceneObject(value));
        return true;
    }
    if release_vec_backing(&mut fill.appended_objects) {
        return true;
    }
    if let Some(value) = fill.appended_attractions.pop() {
        *current = Some(FillRetiredOwner::Attraction(value));
        return true;
    }
    if release_vec_backing(&mut fill.appended_attractions) {
        return true;
    }
    if let Some(value) = fill.placed.pop() {
        *current = Some(FillRetiredOwner::Placed(value));
        return true;
    }
    release_vec_backing(&mut fill.placed)
}

pub(super) fn take_lookup_owner(fill: &mut FillBuilder, current: &mut Option<FillRetiredOwner>) -> bool {
    if let Some((key, _)) = fill.placed_lookup.pop_first() {
        *current = Some(FillRetiredOwner::String(key));
        return true;
    }
    if let Some((key, values)) = fill.candidate_cache.pop_first() {
        *current = Some(FillRetiredOwner::CandidateCache(key, values));
        return true;
    }
    false
}

pub(super) fn take_catalog_owner(fill: &mut FillBuilder, current: &mut Option<FillRetiredOwner>) -> bool {
    if let Some(value) = fill.catalogs.objects.pop() {
        *current = Some(FillRetiredOwner::ObjectKind(value));
        return true;
    }
    if fill.catalogs.objects.retire_backing() {
        return true;
    }
    if let Some(value) = fill.catalogs.vortices.pop() {
        *current = Some(FillRetiredOwner::VortexKind(value));
        return true;
    }
    if fill.catalogs.vortices.retire_backing() {
        return true;
    }
    if let Some(value) = fill.catalogs.cables.pop() {
        *current = Some(FillRetiredOwner::CableKind(value));
        return true;
    }
    if fill.catalogs.cables.retire_backing() {
        return true;
    }
    if let Some(value) = fill.kind_compatibility.pop() {
        *current = Some(FillRetiredOwner::Compat(value));
        return true;
    }
    fill.kind_compatibility.retire_backing()
}

fn take_weight_map_owner<const N: usize>(values: &mut FixedOwnerMap<String, f64, N>, current: &mut Option<FillRetiredOwner>) -> bool {
    if let Some((key, _)) = values.pop_first() {
        *current = Some(FillRetiredOwner::String(key));
        return true;
    }
    false
}

pub(super) fn take_weight_mesh_owner(fill: &mut FillBuilder, current: &mut Option<FillRetiredOwner>) -> bool {
    if take_weight_map_owner(&mut fill.weights.object_weights, current) || take_weight_map_owner(&mut fill.weights.vortex_weights, current) {
        return true;
    }
    if let Some((key, body)) = fill.meshes.pop_first() {
        *current = Some(FillRetiredOwner::Mesh(key, body));
        return true;
    }
    false
}

pub(super) fn take_target_owner(fill: &mut FillBuilder, current: &mut Option<FillRetiredOwner>) -> bool {
    if let Some(value) = fill.targets.pop() {
        *current = Some(FillRetiredOwner::Target(value));
        return true;
    }
    if release_vec_backing(&mut fill.targets) {
        return true;
    }
    if let Some(value) = fill.blocked_vortex_ids.pop_first() {
        *current = Some(FillRetiredOwner::String(value));
        return true;
    }
    false
}

pub(super) fn take_candidate_owner(fill: &mut FillBuilder, current: &mut Option<FillRetiredOwner>) -> bool {
    if let Some(value) = fill.candidates.pop() {
        *current = Some(FillRetiredOwner::Candidate(value));
        return true;
    }
    if release_vec_backing(&mut fill.candidates) {
        return true;
    }
    if let Some(value) = fill.candidate_seen.pop_first() {
        *current = Some(FillRetiredOwner::String(value));
        return true;
    }
    if let Some(value) = fill.candidate_raw.pop() {
        *current = Some(FillRetiredOwner::Candidate(value));
        return true;
    }
    release_vec_backing(&mut fill.candidate_raw)
}

pub(super) fn take_candidate_order_owner(fill: &mut FillBuilder) -> bool {
    for values in [&mut fill.candidate_weights, &mut fill.candidate_tree] {
        if values.pop().is_some() || release_vec_backing(values) {
            return true;
        }
    }
    false
}

pub(super) fn retire_fixed_collection_backing(fill: &mut FillBuilder) -> bool {
    fill.placed_lookup.retire_backing()
        || fill.candidate_cache.retire_backing()
        || fill.weights.object_weights.retire_backing()
        || fill.weights.vortex_weights.retire_backing()
        || fill.meshes.retire_backing()
        || fill.blocked_vortex_ids.retire_backing()
        || fill.candidate_seen.retire_backing()
}

pub(super) fn take_target_weight_owner(fill: &mut FillBuilder) -> bool {
    for values in [&mut fill.target_weights, &mut fill.target_tree] {
        if values.pop().is_some() || release_vec_backing(values) {
            return true;
        }
    }
    false
}

pub(super) fn scene_terminal_owners_empty(value: &FixedSceneOwner) -> bool {
    value.objects.terminal_owners_empty() && value.attractions.terminal_owners_empty() && value.target_volumes.terminal_owners_empty()
}
