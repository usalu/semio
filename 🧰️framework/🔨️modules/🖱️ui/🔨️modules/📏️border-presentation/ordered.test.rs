use super::Ordered;
use std::collections::BTreeMap;

#[test]
fn owned_ordered_storage_matches_system_tree_through_rotations_and_reuse() {
    let mut actual = Ordered::new();
    let mut expected = BTreeMap::new();
    let mut seed = 7_u64;
    for index in 0..8000 {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let key = (seed >> 32) as usize % 193;
        if index % 3 == 0 { actual.remove(key); expected.remove(&key); }
        else { actual.insert(key, index); expected.insert(key, index); }
        assert_eq!(actual.first(), expected.first_key_value().map(|(&k, &v)| (k, v)));
        assert_eq!(actual.contains(key), expected.contains_key(&key));
        assert_eq!(actual.after(key), expected.range((std::ops::Bound::Excluded(key), std::ops::Bound::Unbounded)).next().map(|(&k, &v)| (k, v)));
    }
    while let Some(first) = expected.pop_first() { assert_eq!(actual.pop_first(), Some(first)); }
    assert_eq!(actual.pop_first(), None);
    loop {
        let before = actual.capacity_bytes();
        let demand = actual.next_release_bytes();
        let done = actual.retire_step();
        assert_eq!(before - actual.capacity_bytes(), demand);
        if done { break; }
    }
    assert_eq!(actual.capacity_bytes(), 0);
}
