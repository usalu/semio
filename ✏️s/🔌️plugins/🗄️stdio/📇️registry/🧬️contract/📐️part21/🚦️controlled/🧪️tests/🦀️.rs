//! ♻️ Retirement witnesses include literal entity and typed names under actual byte grants.
use super::*;

#[test]
fn part21_cohort_retirement_releases_all_literal_name_bytes_within_grants() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🚦️sqlite-cohort/🔣️.json")).unwrap();
    let row = &fixture["retirementCase"];
    let instance = Part21Instance {
        id: row["id"].as_str().unwrap().parse().unwrap(),
        entities: vec![(row["name"].as_str().unwrap().into(), vec![Part21Value::Typed { name: row["typedName"].as_str().unwrap().into(), items: vec![Part21Value::Str(row["value"].as_str().unwrap().into())] }])],
    };
    let maximum_items = usize::try_from(row["maximumItems"].as_u64().unwrap()).unwrap();
    let maximum_bytes = usize::try_from(row["maximumBytes"].as_u64().unwrap()).unwrap();
    let mut retirement = semio_framework_value::retirement::owned_retirement(instance);
    let mut released = 0usize;
    let mut finished = false;
    for _ in 0..256 {
        match retirement.close_step(maximum_items, maximum_bytes).unwrap() {
            semio_framework_value::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= maximum_items);
                assert!(released_bytes <= maximum_bytes);
                released = released.checked_add(released_bytes).unwrap();
            }
            semio_framework_value::SnapshotRetirementStep::Complete => {
                finished = true;
                break;
            }
            semio_framework_value::SnapshotRetirementStep::Blocked => panic!("owned retirement unexpectedly blocked"),
        }
    }
    assert!(finished && retirement.terminal_is_empty());
    assert_eq!(released, usize::try_from(row["expectedReleasedBytes"].as_u64().unwrap()).unwrap());
}
