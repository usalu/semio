
use super::*;

#[test]
fn insert_and_get_round_trip() {
    let mut arena = Arena::new();
    let id = arena.insert(42);
    assert_eq!(arena.get(id), Some(&42));
}

#[test]
fn remove_invalidates_the_old_node_id() {
    let mut arena = Arena::new();
    let id = arena.insert(1);
    assert_eq!(arena.remove(id), Some(1));
    assert_eq!(arena.get(id), None);
    assert_eq!(arena.remove(id), None);
}

#[test]
fn reused_slot_bumps_generation_so_old_id_does_not_alias_new_value() {
    let mut arena = Arena::new();
    let a = arena.insert(1);
    arena.remove(a);
    let b = arena.insert(2);
    assert_eq!(b.index, a.index);
    assert_ne!(b.generation, a.generation);
    assert_eq!(arena.get(a), None);
    assert_eq!(arena.get(b), Some(&2));
}

#[test]
fn iterates_over_live_slots_only() {
    let mut arena = Arena::new();
    let a = arena.insert(10);
    let b = arena.insert(20);
    arena.remove(a);
    let remaining: Vec<i32> = arena.iter().map(|(_, value)| *value).collect();
    assert_eq!(remaining, vec![20]);
    assert!(arena.contains(b));
    assert!(!arena.contains(a));
}
