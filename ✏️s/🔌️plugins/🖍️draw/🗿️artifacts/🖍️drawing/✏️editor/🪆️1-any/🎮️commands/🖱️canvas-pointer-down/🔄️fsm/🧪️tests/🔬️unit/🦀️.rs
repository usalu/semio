
use super::*;

#[semio_framework_async_macros::async_test]
async fn bitset_set_clear_contains() {
    let mut bits = BitSet::<1>::empty();
    assert!(!bits.contains(NodeId(3)));
    bits.set(NodeId(3));
    assert!(bits.contains(NodeId(3)));
    bits.clear(NodeId(3));
    assert!(!bits.contains(NodeId(3)));
}

#[semio_framework_async_macros::async_test]
async fn bitset_iter_ones_spans_words() {
    let mut bits = BitSet::<2>::empty();
    bits.set(NodeId(0));
    bits.set(NodeId(63));
    bits.set(NodeId(64));
    bits.set(NodeId(100));
    let ids: Vec<u16> = bits.iter_ones().map(|n| n.0).collect();
    assert_eq!(ids, vec![0, 63, 64, 100]);
}

#[semio_framework_async_macros::async_test]
async fn bitset_clear_all_and_is_empty() {
    let mut bits = BitSet::<1>::empty();
    assert!(bits.is_empty());
    bits.set(NodeId(5));
    assert!(!bits.is_empty());
    bits.clear_all();
    assert!(bits.is_empty());
}
