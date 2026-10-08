
use super::*;
use protocol::DiffAlgebra;

fn curated(delta: CurationCuratedDelta) -> CurationDiff {
    CurationDiff { curated: Some(delta), ..Default::default() }
}

fn item(id: &str, count: u32) -> CuratedItem {
    CuratedItem { object_id: id.into(), count }
}

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🧺️parent-lanes/🔣️.json")).expect("neutral parent lanes")
}

fn state(base: &CurationSnapshot, rows: &serde_json::Value) -> CurationSnapshot {
    let mut result = base.clone();
    result.curated = rows.as_array().expect("curated rows").iter().map(|row| CuratedItem { object_id: row["objectId"].as_str().expect("object identity").to_owned(), count: row["count"].as_u64().expect("count").try_into().expect("u32 count") }).collect();
    result
}

#[test]
fn parent_diff_preserves_child_identity_and_inverts_curated_lanes() {
    let vector = fixture();
    let base = state(&CurationSnapshot::default(), &vector["initial"]);
    let next = state(&base, &vector["final"]);
    let mut delta = CurationCuratedDelta::insertion(0, item("beam-1", 5));
    delta.absorb(CurationCuratedDelta::insertion(1, item("slab-1", 1)));
    let diff = curated(delta);
    assert!(diff.catalog.is_none() && diff.stock_extra.is_none());
    let applied = protocol::apply_diff(&diff, &base).expect("parent diff");
    assert_eq!(applied, next);
    assert_eq!(protocol::apply_diff(&diff.inverse(&base), &applied).expect("inverse"), base);
    let native = semio_framework_pack_json::to_json_string(&applied.curated);
    assert_eq!(serde_json::from_str::<serde_json::Value>(&native).expect("independent JSON witness"), vector["final"]);
    eprintln!("[DEBUG] Sourcing parent diff retained child identity and neutral curated rows={}", applied.curated.len());
}

#[test]
fn parent_diff_composition_matches_the_neutral_intermediate_state() {
    let vector = fixture();
    let base = state(&CurationSnapshot::default(), &vector["initial"]);
    let middle = state(&base, &vector["middle"]);
    let last = state(&base, &vector["final"]);
    let mut composed = curated(CurationCuratedDelta::insertion(0, item("beam-1", 2)));
    let mut second = CurationCuratedDelta::removal(&middle.curated, 0);
    second.absorb(CurationCuratedDelta::insertion(0, item("beam-1", 5)));
    second.absorb(CurationCuratedDelta::insertion(1, item("slab-1", 1)));
    composed.absorb(curated(second));
    assert_eq!(protocol::apply_diff(&composed, &base).expect("composed parent delta"), last);
    assert!(composed.catalog.is_none() && composed.stock_extra.is_none());
    eprintln!("[DEBUG] Sourcing parent diff composition matched independently admitted intermediate and final rows");
}

#[test]
fn positional_curated_delta_inserts_removes_and_moves_at_middle_rows() {
    let item = |id: &str, count: u32| CuratedItem { object_id: id.into(), count };
    let snapshot = |ids: &[(&str, u32)]| CurationSnapshot { curated: ids.iter().map(|(id, count)| item(id, *count)).collect(), ..CurationSnapshot::default() };
    let order = |snapshot: &CurationSnapshot| snapshot.curated.iter().map(|row| row.object_id.clone()).collect::<Vec<_>>();
    let diff = |delta: CurationCuratedDelta| CurationDiff { curated: Some(delta), ..Default::default() };
    let base = snapshot(&[("a", 1), ("b", 2), ("c", 3)]);
    let insert = diff(CurationCuratedDelta::insertion(1, item("x", 9)));
    let inserted = protocol::apply_diff(&insert, &base).expect("valid insertion");
    assert_eq!(order(&inserted), ["a", "x", "b", "c"]);
    let remove = diff(CurationCuratedDelta::removal(&inserted.curated, 2));
    let removed = protocol::apply_diff(&remove, &inserted).expect("valid removal");
    assert_eq!(order(&removed), ["a", "x", "c"]);
    let relocate = diff(CurationCuratedDelta::relocation(&removed.curated, 2, 0));
    let moved = protocol::apply_diff(&relocate, &removed).expect("valid relocation");
    assert_eq!(order(&moved), ["c", "a", "x"]);
    let mut sum = insert;
    sum.absorb(remove);
    sum.absorb(relocate);
    assert_eq!(protocol::apply_diff(&sum, &base).expect("valid sum"), moved);
    assert_eq!(protocol::apply_diff(&sum.inverse(&base), &moved).expect("valid inverse"), base);
}
