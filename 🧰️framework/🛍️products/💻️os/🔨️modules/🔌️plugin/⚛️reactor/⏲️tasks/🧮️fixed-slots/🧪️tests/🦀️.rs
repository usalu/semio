use super::*;

#[test]
fn sparse_rotation_visits_each_admitted_slot_once_after_insertion_and_removal() {
    let numbers = |s: &str| s.split(',').filter(|n| !n.is_empty()).map(|n| n.parse::<usize>().unwrap()).collect::<Vec<_>>();
    for row in include_str!("🧫️fixtures/occupancy.txt").lines() {
        let mut fields = row.split('|');
        let start = fields.next().unwrap().parse().unwrap();
        let active = numbers(fields.next().unwrap());
        let expected = numbers(fields.next().unwrap());
        let mut slots = ReactorFixedSlots::unallocated();
        assert_eq!(slots.iter_from(start).count(), 0);
        assert!(slots.admit_backing());
        for &index in &active { slots.insert(index, index).unwrap(); }
        assert_eq!(slots.iter_from(start).map(|(i, v)| { assert_eq!(i,*v); i }).collect::<Vec<_>>(), expected);
        for &index in &active { assert_eq!(slots.take(index),Some(index)); }
        assert_eq!(slots.iter().count(), 0);
    }
}
