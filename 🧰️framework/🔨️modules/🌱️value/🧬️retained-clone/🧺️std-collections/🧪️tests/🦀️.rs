//! 🧪️ Standard collection clones equal the `Clone` oracle and the serde_json oracle under entry-sized grants and early cancellation.
use super::*;
use crate::retained_clone::RetainedCloneSource;
use std::ops::Deref;
use serde::{Serialize, de::DeserializeOwned};
use std::fmt::Debug;

struct Held<M: RetainedClone>(RetainedCloneSource<M>);

impl<M: RetainedClone> Held<M> {
    fn new(owner: M) -> Self {
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: RetainedCloneSource::<M>::constructor_copy_bytes(), maximum_capacity_bytes: RetainedCloneSource::<M>::owned_constructor_capacity_bytes::<()>(), maximum_depth: 1, ..Default::default() };
        Self(RetainedCloneSource::admit_owned(owner, (), grant).unwrap_or_else(|_| panic!("standard collection source admission")).0)
    }
}

impl<M: RetainedClone> Deref for Held<M> {
    type Target = RetainedCloneSource<M>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<M: RetainedClone> Drop for Held<M> {
    fn drop(&mut self) {
        if std::thread::panicking() { return; }
        for _ in 0..100_000 {
            if self.0.terminal_is_empty() { return; }
            let copy = self.0.next_close_copy_byte_demand().unwrap();
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: self.0.next_close_capacity_byte_demand(copy).unwrap(), maximum_release_bytes: self.0.next_close_release_byte_demand().unwrap(), maximum_depth: self.0.next_close_depth_demand().unwrap() };
            assert!(self.0.close_step(grant).unwrap().progress().fits(grant));
        }
        panic!("standard collection source did not close");
    }
}

fn wide() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 4, maximum_copy_bytes: 1 << 16, maximum_capacity_bytes: 1 << 16, maximum_release_bytes: 1 << 16, maximum_depth: 64 }
}

fn close_all<M: RetainedClone>(cursor: &mut M::Cursor) {
    cursor.begin_close();
    for _ in 0..100_000 {
        if cursor.terminal_is_empty() { return; }
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: cursor.next_close_copy_byte_demand().unwrap(), maximum_capacity_bytes: cursor.next_close_capacity_byte_demand(1 << 16).unwrap(), maximum_release_bytes: cursor.next_close_release_byte_demand().unwrap(), maximum_depth: cursor.next_close_depth_demand().unwrap().max(1) };
        let step = cursor.close_step(grant).unwrap();
        assert!(step.progress().fits(grant));
    }
    panic!("standard collection clone close did not terminate");
}

fn clone_exactly<M: RetainedCloneEntries + Clone + PartialEq + Debug + Serialize>(original: M, expected: &serde_json::Value, normalize: fn(serde_json::Value) -> serde_json::Value) -> (usize, usize) {
    let oracle = original.clone();
    let count = original.entry_count();
    let source = Held::new(original);
    let mut cursor = M::retained_clone_cursor();
    let (mut output, mut turns, mut copied_items, mut quoted) = (None, 0usize, 0usize, 0usize);
    while output.is_none() {
        turns += 1;
        assert!(turns < 100_000, "standard collection clone did not terminate");
        let grant = match cursor.advance_demands(source.borrow(), 1 << 16) {
            Ok(demand) => {
                let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
                for denied in [RetainedCloneGrant { maximum_items: 0, ..grant }, RetainedCloneGrant { maximum_depth: 0, ..grant }] {
                    assert_eq!(cursor.advance(source.borrow(), denied).unwrap().progress(), RetainedCloneProgress::default());
                }
                if demand.copy_bytes > 0 {
                    let smaller=RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant};let selected=cursor.advance_demands(source.borrow(),smaller.maximum_copy_bytes).unwrap();let step=cursor.advance(source.borrow(),smaller).unwrap();assert!(step.progress().fits(smaller));assert!(step.progress().copied_bytes<=selected.copy_bytes);if selected.copy_bytes>smaller.maximum_copy_bytes{assert_eq!(step.progress(),RetainedCloneProgress::default(),"selected atomic copy demand {selected:?} must be binding");}
                }
                quoted += 1;
                grant
            }
            Err(_) => wide(),
        };
        let step = cursor.advance(source.borrow(), grant).unwrap();
        assert!(step.progress().fits(grant));
        copied_items += step.progress().copied_items;
        if let RetainedCloneStep::Complete(_) = step { output = cursor.take(); }
    }
    let output = output.unwrap();
    assert_eq!(output, oracle);
    assert_eq!(normalize(serde_json::to_value(&output).unwrap()), normalize(expected.clone()));
    assert!(copied_items >= count);
    drop(output);
    close_all::<M>(&mut cursor);
    (turns, quoted)
}

fn cancel_midway<M: RetainedCloneEntries + Clone>(original: M, stop_after: usize) {
    let source = Held::new(original);
    let mut cursor = M::retained_clone_cursor();
    for _ in 0..stop_after {
        if let RetainedCloneStep::Complete(_) = cursor.advance(source.borrow(), wide()).unwrap() { break; }
    }
    close_all::<M>(&mut cursor);
}

fn typed<M: DeserializeOwned>(value: &serde_json::Value) -> M { serde_json::from_value(value.clone()).unwrap() }

#[test]
fn standard_collection_clones_equal_clone_and_serde_oracles_per_fixture_row() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    assert_eq!(fixture["schema"], "framework.value.retained-clone.std-collections.v1");
    for row in fixture["cases"].as_array().unwrap() {
        let value = &row["value"];
        let (turns, quoted) = match row["kind"].as_str().unwrap() {
            "hash-map" => clone_exactly(typed::<HashMap<String, String>>(value), value, identity),
            "btree-map" => clone_exactly(typed::<BTreeMap<String, Vec<String>>>(value), value, identity),
            "hash-set" => clone_exactly(typed::<HashSet<String>>(value), value, sorted),
            "btree-set" => clone_exactly(typed::<BTreeSet<u32>>(value), value, identity),
            kind => panic!("unknown fixture kind {kind}"),
        };
        eprintln!("[DEBUG] standard collection clone {} turns={turns} quotedTurns={quoted}", row["name"]);
    }
}

fn identity(value: serde_json::Value) -> serde_json::Value { value }

fn sorted(value: serde_json::Value) -> serde_json::Value {
    let mut items = value.as_array().unwrap().clone();
    items.sort_by_key(|item| item.to_string());
    serde_json::Value::Array(items)
}

#[test]
fn standard_collection_clones_cancel_at_every_turn_without_abandoning_ownership() {
    let map: HashMap<String, String> = (0..5).map(|index| (format!("k{index}"), format!("v{index}"))).collect();
    let tree: BTreeMap<String, Vec<String>> = (0..5).map(|index| (format!("k{index}"), vec![format!("v{index}")])).collect();
    let set: HashSet<String> = (0..5).map(|index| format!("k{index}")).collect();
    let ordered: BTreeSet<u32> = (0..5).collect();
    for stop in 0..64 {
        cancel_midway(map.clone(), stop);
        cancel_midway(tree.clone(), stop);
        cancel_midway(set.clone(), stop);
        cancel_midway(ordered.clone(), stop);
    }
}

#[test]
fn standard_collection_option_and_nesting_clone_through_the_entry_cursor() {
    let nested: BTreeMap<String, Option<BTreeSet<u32>>> = [("none".to_string(), None), ("some".to_string(), Some([1u32, 2].into_iter().collect()))].into_iter().collect();
    let expected = serde_json::json!({ "none": null, "some": [1, 2] });
    clone_exactly(nested, &expected, identity);
}

#[test]
fn standard_collection_scalar_clones_are_fully_quoted_turn_by_turn() {
    let map: HashMap<u32, u64> = (0..9).map(|index| (index, u64::from(index) * 3)).collect();
    let expected = serde_json::to_value(map.iter().map(|(key, value)| (key.to_string(), *value)).collect::<BTreeMap<_, _>>()).unwrap();
    let (turns, quoted) = clone_exactly(map, &expected, |value| value);
    assert_eq!(turns, quoted, "scalar entry cursors quote every advance");
    let set: BTreeSet<u32> = (0..9).collect();
    let (turns, quoted) = clone_exactly(set.clone(), &serde_json::to_value(&set).unwrap(), |value| value);
    assert_eq!(turns, quoted);
}

#[test]
fn reselected_string_copy_budget_matches_exact_utf8_prefix_and_actual_maximum(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();for row in fixture["copySelection"].as_array().unwrap(){let text=row["source"].as_str().unwrap();let source=Held::new(text.to_owned());let mut cursor=String::retained_clone_cursor();for _ in 0..4{if cursor.phase==1{break}let grant=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:text.len(),maximum_depth:1,..Default::default()};assert!(cursor.advance(source.borrow(),grant).unwrap().progress().fits(grant));}assert_eq!(cursor.phase,1);let requested=row["requestedBytes"].as_u64().unwrap()as usize;let maximum=row["admittedBytes"].as_u64().unwrap()as usize;let original=cursor.advance_demands(source.borrow(),requested).unwrap();let selected=cursor.advance_demands(source.borrow(),maximum).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:maximum,maximum_depth:selected.depth,..Default::default()};let step=cursor.advance(source.borrow(),grant).unwrap();assert!(step.progress().fits(grant));assert_eq!(step.progress().copied_bytes,row["expectedCopiedBytes"].as_u64().unwrap()as usize);assert!(step.progress().copied_bytes<=selected.copy_bytes);assert!(maximum<original.copy_bytes);let prefix=row["expectedPrefix"].as_str().unwrap();assert_eq!(cursor.output.as_deref().unwrap(),prefix);let oracle=String::from_utf8(text.as_bytes()[..prefix.len()].to_vec()).unwrap();assert_eq!(cursor.output.as_deref().unwrap(),oracle);close_all::<String>(&mut cursor);eprintln!("[DEBUG] Actual reselected UTF8 copy requested={requested} admitted={maximum} copied={} exactPrefix={prefix}",step.progress().copied_bytes);}
}
