use semio_framework_test_fixtures::fixture_text;
use super::*;

fn execute(keys: &[[u64; 3]]) -> (Vec<usize>, usize) {
    let mut order: Vec<usize> = (0..keys.len()).collect();
    let mut cursor = HeapSortCursor::new(keys.len());
    let mut work = 0;
    loop {
        let comparison = match cursor.next() {
            HeapSortWork::Complete => break,
            HeapSortWork::Compare { left, right } => Some(keys[order[left]].cmp(&keys[order[right]])),
            HeapSortWork::Swap { left, right } => { order.swap(left, right); None },
            HeapSortWork::Advance => None,
        };
        cursor.advance(comparison).expect("exact pending sort work");
        let restored = HeapSortCursor::from_snapshot(cursor.snapshot()).expect("portable exact sort state");
        assert_eq!(restored, cursor);
        cursor = restored;
        work += 1;
        assert!(work <= 16 * keys.len().max(1) * ((keys.len() + 1).ilog2() as usize + 1));
    }
    (order, work)
}

#[test]
fn retained_index_sort_matches_portable_order_and_bounds_every_action() {
    let corpus: serde_json::Value = serde_json::from_str(fixture_text!("../../../../📶️sort/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for case in corpus["cases"].as_array().unwrap() {
        let keys: Vec<[u64; 3]> = case["keys"].as_array().unwrap().iter().map(|key| std::array::from_fn(|axis| key[axis].as_u64().unwrap())).collect();
        let expected: Vec<usize> = case["order"].as_array().unwrap().iter().map(|index| index.as_u64().unwrap() as usize).collect();
        assert_eq!(execute(&keys).0, expected, "{}", case["id"]);
    }
    for count in [2, 17, 128, 1024, 4097] {
        let keys: Vec<[u64; 3]> = (0..count).rev().map(|value| [value as u64, 0, 0]).collect();
        let (order, work) = execute(&keys);
        assert!(order.windows(2).all(|pair| keys[pair[0]] <= keys[pair[1]]));
        println!("[DEBUG] shared heap-sort {count} original indices completed in {work} bounded actions");
    }
}

#[test]
fn retained_index_sort_refuses_wrong_observation_without_changing_cursor() {
    let mut cursor = HeapSortCursor::new(8);
    let original = cursor;
    assert_eq!(cursor.advance(Some(std::cmp::Ordering::Equal)), Err(HeapSortFault::UnexpectedComparison));
    assert_eq!(cursor, original);
    cursor.advance(None).unwrap();
    cursor.advance(None).unwrap();
    let original = cursor;
    assert!(matches!(cursor.next(), HeapSortWork::Compare { .. }));
    assert_eq!(cursor.advance(None), Err(HeapSortFault::MissingComparison));
    assert_eq!(cursor, original);
    println!("[DEBUG] shared heap-sort invalid observations preserve the exact cursor");
}
