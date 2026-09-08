use super::*;
use std::collections::BTreeSet;

#[test]
fn time_ordered_ids_have_v7_shape_and_do_not_repeat() {
    let ids: Vec<_> = (0..10_000).map(|_| time_ordered_id()).collect();
    assert!(ids.iter().all(|id| id.len() == 36 && id.as_bytes()[14] == b'7' && matches!(id.as_bytes()[19], b'8' | b'9' | b'a' | b'b')));
    assert_eq!(ids.iter().collect::<BTreeSet<_>>().len(), ids.len());
}

#[test]
fn entropy_changes_successive_buffers() {
    let mut first = [0u8; 32];
    let mut second = [0u8; 32];
    fill_entropy(&mut first).expect("platform entropy");
    fill_entropy(&mut second).expect("platform entropy");
    assert_ne!(first, second);
}
