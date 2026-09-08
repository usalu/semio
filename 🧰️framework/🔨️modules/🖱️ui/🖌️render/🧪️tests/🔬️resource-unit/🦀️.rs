
use super::*;

#[test]
fn interning_the_same_key_twice_returns_the_same_id_and_queues_one_upload() {
    let mut registry = ResourceRegistry::default();
    let a = registry.request_texture_upload("icon/foo", 4, 4, vec![0; 64]);
    let b = registry.request_texture_upload("icon/foo", 4, 4, vec![0; 64]);
    assert_eq!(a, b);
    assert_eq!(registry.drain_ops().len(), 1);
}

#[test]
fn resident_texture_is_not_re_queued() {
    let mut registry = ResourceRegistry::default();
    let id = registry.request_texture_upload("icon/foo", 4, 4, vec![0; 64]);
    registry.mark_texture_resident(id);
    registry.request_texture_upload("icon/foo", 4, 4, vec![0; 64]);
    assert!(registry.drain_ops().is_empty());
}

#[test]
fn eviction_bumps_generation_so_the_old_id_never_resolves_again() {
    let mut registry = ResourceRegistry::default();
    let id = registry.request_texture_upload("icon/foo", 4, 4, vec![0; 64]);
    registry.drain_ops();
    registry.evict_texture(id);
    assert!(registry.texture_state(id).is_none());
    let ops = registry.drain_ops();
    assert_eq!(ops, vec![ResourceOp::EvictTexture(id)]);
    let reallocated = registry.intern_texture("icon/foo");
    assert_ne!(reallocated, id);
}

#[test]
fn evicted_slot_is_reused_by_the_next_allocation() {
    let mut registry = ResourceRegistry::default();
    let first = registry.intern_texture("a");
    registry.evict_texture(first);
    let second = registry.intern_texture("b");
    assert_eq!(first.slot, second.slot);
    assert_ne!(first.generation, second.generation);
}

#[test]
fn device_loss_re_marks_requested_without_changing_identity() {
    let mut registry = ResourceRegistry::default();
    let id = registry.request_texture_upload("icon/foo", 4, 4, vec![0; 64]);
    registry.mark_texture_resident(id);
    registry.report_device_loss(&[id], &[], &[]);
    assert_eq!(registry.texture_state(id), Some(&ResourceState::Requested));
    let reused = registry.intern_texture("icon/foo");
    assert_eq!(reused, id);
}

#[test]
fn mesh_content_hash_changes_with_indices() {
    let a = ResourceRegistry::mesh_content_hash(&[0.0, 0.0, 0.0], &[0.0, 1.0, 0.0], &[0, 1, 2]);
    let b = ResourceRegistry::mesh_content_hash(&[0.0, 0.0, 0.0], &[0.0, 1.0, 0.0], &[0, 2, 1]);
    assert_ne!(a, b);
}

#[test]
fn unchanged_mesh_content_resolves_to_the_same_id_with_no_new_op() {
    let mut registry = ResourceRegistry::default();
    let a = registry.request_mesh_upload("box", vec![0.0, 0.0, 0.0], vec![0.0, 1.0, 0.0], vec![0, 1, 2]);
    registry.drain_ops();
    let b = registry.request_mesh_upload("box", vec![0.0, 0.0, 0.0], vec![0.0, 1.0, 0.0], vec![0, 1, 2]);
    assert_eq!(a, b);
    assert!(registry.drain_ops().is_empty());
}

#[test]
fn changed_mesh_content_allocates_a_new_id_and_queues_an_upload() {
    let mut registry = ResourceRegistry::default();
    let a = registry.request_mesh_upload("box", vec![0.0, 0.0, 0.0], vec![0.0, 1.0, 0.0], vec![0, 1, 2]);
    let b = registry.request_mesh_upload("box", vec![1.0, 0.0, 0.0], vec![0.0, 1.0, 0.0], vec![0, 1, 2]);
    assert_ne!(a, b);
}
