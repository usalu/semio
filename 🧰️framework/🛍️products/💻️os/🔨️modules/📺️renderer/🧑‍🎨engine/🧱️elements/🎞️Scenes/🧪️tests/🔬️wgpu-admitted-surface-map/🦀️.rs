
use super::*;

#[test]
fn preserves_admission_order_with_constant_index_access() {
    let mut surfaces = AdmittedSurfaceMap::default();
    surfaces.try_insert("third".to_string(), 3usize).unwrap();
    surfaces.try_insert("first".to_string(), 1usize).unwrap();
    surfaces.try_insert("third".to_string(), 30usize).unwrap();
    assert_eq!(surfaces.id_at(0), Some("third"));
    assert_eq!(surfaces.id_at(1), Some("first"));
    assert_eq!(surfaces.id_at(2), None);
    assert_eq!(surfaces.get("third"), Some(&30));
}

#[test]
fn rejects_the_257th_surface_before_map_ownership() {
    let mut surfaces = AdmittedSurfaceMap::default();
    for index in 0..SCENE_SURFACE_CAPACITY {
        surfaces.try_insert(format!("surface-{index}"), index).unwrap();
    }
    let rejected = surfaces.try_insert("overflow".to_string(), SCENE_SURFACE_CAPACITY).expect_err("exact capacity owner");
    assert_eq!((rejected.id.as_str(), rejected.value), ("overflow", SCENE_SURFACE_CAPACITY));
    surfaces.retain_rejected(rejected).unwrap();
    assert_eq!(surfaces.len(), SCENE_SURFACE_CAPACITY);
    assert_eq!(surfaces.id_at(SCENE_SURFACE_CAPACITY), None);
    assert_eq!(surfaces.take_fault(), Some("scene surface item credits exceeded"));
}

#[test]
fn replacement_removal_and_clear_preserve_order_invariants() {
    let mut surfaces = AdmittedSurfaceMap::default();
    surfaces.try_insert("a".to_string(), 1usize).unwrap();
    surfaces.try_insert("b".to_string(), 2usize).unwrap();
    surfaces.try_insert("a".to_string(), 3usize).unwrap();
    assert_eq!(surfaces.id_at(0), Some("a"));
    assert_eq!(surfaces.id_at(1), Some("b"));
    assert_eq!(surfaces.remove("a"), Some(3));
    assert_eq!(surfaces.id_at(0), Some("b"));
    assert_eq!(surfaces.id_at(1), None);
    surfaces.begin_close();
    let mut closed = Vec::new();
    while let Some(owner) = surfaces.close_step() {
        closed.push((owner.id, owner.value));
    }
    assert!(surfaces.is_empty());
    assert!(surfaces.terminal_is_empty());
    assert_eq!(surfaces.id_at(0), None);
    assert_eq!(surfaces.take_fault(), None);
}

#[test]
fn replacement_and_slot_reuse_invalidate_surface_aba_tokens() {
    let mut surfaces = AdmittedSurfaceMap::default();
    surfaces.try_insert("surface".to_string(), 1usize).unwrap();
    let first = surfaces.token("surface").unwrap();
    surfaces.try_insert("surface".to_string(), 2usize).unwrap();
    let second = surfaces.token("surface").unwrap();
    assert_ne!(first, second);
    assert_eq!(surfaces.get_token(first), None);
    assert_eq!(surfaces.get_token(second), Some(&2));
    assert_eq!(surfaces.remove("surface"), Some(2));
    surfaces.try_insert("replacement".to_string(), 3usize).unwrap();
    assert_eq!(surfaces.get_token(second), None);
}

#[test]
fn production_surface_authority_has_no_hash_map_or_structural_deref() {
    let source = include_str!("../../🎯️targets/🧊️wgpu/🦀️.rs");
    let authority = source.split("#[cfg(test)]\nmod admitted_surface_map_tests").next().unwrap();
    assert!(!authority.contains("values: HashMap<String, T>"));
    assert!(!authority.contains("DerefMut"));
    assert!(authority.contains("slots: Box<[Option<AdmittedSurfaceEntry<T>>; SCENE_SURFACE_CAPACITY]>"));
}
