use super::*;
use crate::value::observe_retirement_allocations as observe;

#[derive(Clone, Copy, crate::RetireOwned)]
struct DerivedCell { x: u32, y: u32, z: f32, solid: bool, tag: u8 }

#[derive(Clone, Copy)]
struct LeafCell { x: u32, y: u32, z: f32, solid: bool, tag: u8 }
crate::artifact_retire_leaf!(LeafCell);

#[derive(Clone, Copy, crate::RetireOwned)]
struct Padded { small: u8, wide: u64 }

#[derive(Clone, Copy, crate::RetireOwned)]
struct Wrapper(DerivedCell);

#[derive(Clone, Copy, crate::RetireOwned)]
enum Kind { Empty, Solid, Liquid }

#[derive(Clone, Copy, crate::RetireOwned)]
enum Shape { Empty, Dot(u32), Segment { start: f32, end: u64 } }

#[derive(crate::RetireOwned)]
struct Pair<T> { left: T, right: T }

#[derive(crate::RetireOwned)]
struct Named { name: String, id: u32 }

#[derive(crate::RetireOwned)]
enum Labeled { Plain(u32), Text(String) }

#[derive(crate::RetireOwned)]
struct Unit;

fn cell(index: usize) -> DerivedCell { DerivedCell { x: index as u32, y: !(index as u32), z: index as f32, solid: index % 2 == 0, tag: (index % 251) as u8 } }

struct Drained { turns: usize, copied: usize, released: usize, born: usize }

fn drain_turns<T: RetireOwned>(value: T, work: usize) -> Drained {
    let mut owner = controlled::ControlledRetirement::new(value).map_err(|(error, _)| error).unwrap();
    let (mut copied, mut released, mut born) = (0, 0, 0);
    for turn in 0..2_000_000 {
        if owner.terminal_is_empty() { return Drained { turns: turn, copied, released, born }; }
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: work, maximum_capacity_bytes: owner.next_capacity_byte_demand(work).unwrap(), maximum_release_bytes: owner.next_release_byte_demand().unwrap(), maximum_depth: owner.next_depth_demand().unwrap() };
        let (step, heap) = observe(|| owner.step(grant).unwrap());
        let progress = step.progress();
        assert!(progress.fits(grant));
        assert_eq!(heap, (progress.retained_capacity_bytes, progress.released_bytes));
        copied += progress.copied_bytes;
        released += progress.released_bytes;
        born += progress.retained_capacity_bytes;
    }
    panic!("plain retirement did not finish within the turn ceiling");
}

fn width<T: RetireOwned>() -> usize { T::retirement_element_copy_bytes() }

#[test]
fn derived_retirement_declares_the_padded_width_of_plain_data_and_nothing_else() {
    assert_eq!(size_of::<DerivedCell>(), 16);
    assert_eq!(width::<DerivedCell>(), size_of::<DerivedCell>());
    assert_eq!(width::<Padded>(), 16, "padding is part of the page-wise width");
    assert_eq!(width::<Wrapper>(), size_of::<Wrapper>());
    assert_eq!(width::<Kind>(), size_of::<Kind>());
    assert_eq!(width::<Shape>(), size_of::<Shape>());
    assert_eq!(width::<Pair<u16>>(), 4);
    assert_eq!(width::<Pair<DerivedCell>>(), 2 * size_of::<DerivedCell>());
    assert_eq!(width::<Pair<String>>(), 0, "a generic instantiated with an owner is element-wise");
    assert_eq!(width::<Named>(), 0, "a String field keeps the struct element-wise");
    assert_eq!(width::<Labeled>(), 0, "one non-plain variant keeps the whole enum element-wise");
    assert_eq!(width::<Unit>(), 0, "zero-sized data has no payload width");
    assert_eq!(width::<LeafCell>(), width::<DerivedCell>(), "the derive equals the leaf macro for the same plain layout");
}

#[test]
fn option_array_and_tuple_propagate_plainness_of_their_components() {
    assert_eq!(width::<Option<u32>>(), size_of::<Option<u32>>());
    assert_eq!(width::<Option<[f32; 3]>>(), size_of::<Option<[f32; 3]>>());
    assert_eq!(width::<Option<String>>(), 0);
    assert_eq!(width::<Option<DerivedCell>>(), size_of::<Option<DerivedCell>>());
    assert_eq!(width::<[f32; 3]>(), 12);
    assert_eq!(width::<[DerivedCell; 4]>(), 4 * size_of::<DerivedCell>());
    assert_eq!(width::<[String; 2]>(), 0);
    assert_eq!(width::<(u32, u16)>(), size_of::<(u32, u16)>());
    assert_eq!(width::<(u8, Kind, Option<u32>)>(), size_of::<(u8, Kind, Option<u32>)>());
    assert_eq!(width::<(u32, String)>(), 0);
    assert_eq!(width::<Vec<u32>>(), 0, "a vector owns a heap allocation and is never an inline payload");
    assert_eq!(width::<Box<u32>>(), 0);
}

#[test]
fn composite_of_options_and_arrays_becomes_plain_through_the_derive() {
    #[derive(Clone, Copy, crate::RetireOwned)]
    struct MeshVertexLike { position: [f32; 3], normal: Option<[f32; 3]>, halfedge: Option<u32> }
    assert_eq!(width::<MeshVertexLike>(), size_of::<MeshVertexLike>());
    #[derive(crate::RetireOwned)]
    struct Owning { position: [f32; 3], names: Vec<u32> }
    assert_eq!(width::<Owning>(), 0);
}

#[test]
fn a_plain_derived_vector_retires_in_pages_and_matches_the_leaf_macro_receipt_for_receipt() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🧮️plain-elements/🔣️.json")).unwrap();
    let work = fixture["workBytes"].as_u64().unwrap() as usize;
    let overhead = fixture["overheadTurns"].as_u64().unwrap() as usize;
    let mut previous: Option<(usize, usize)> = None;
    for count in fixture["counts"].as_array().unwrap().iter().map(|count| count.as_u64().unwrap() as usize) {
        let derived: Vec<DerivedCell> = (0..count).map(cell).collect();
        let leaf: Vec<LeafCell> = derived.iter().map(|cell| LeafCell { x: cell.x, y: cell.y, z: cell.z, solid: cell.solid, tag: cell.tag }).collect();
        let capacity_bytes = derived.capacity() * size_of::<DerivedCell>();
        let (derived_receipt, leaf_receipt) = (drain_turns(derived, work), drain_turns(leaf, work));
        let payload = count * size_of::<DerivedCell>();
        let pages = payload.div_ceil(work);
        assert_eq!(derived_receipt.copied, payload, "padded logical payload work");
        assert!(derived_receipt.turns <= pages + overhead, "{count} elements took {} turns for {pages} pages", derived_receipt.turns);
        assert_eq!((derived_receipt.turns, derived_receipt.copied, derived_receipt.released, derived_receipt.born), (leaf_receipt.turns, leaf_receipt.copied, leaf_receipt.released, leaf_receipt.born), "derive and leaf macro retire identically");
        assert_eq!(derived_receipt.released, capacity_bytes + derived_receipt.born, "std Vec backing plus every scaffold is released exactly once");
        if let Some((previous_count, previous_turns)) = previous {
            let extra_pages = ((count - previous_count) * size_of::<DerivedCell>()).div_ceil(work);
            assert!(derived_receipt.turns - previous_turns <= extra_pages + 2, "turn growth is linear in pages, not in elements");
        }
        previous = Some((count, derived_receipt.turns));
    }
}

#[test]
fn data_enums_options_and_arrays_of_plain_elements_retire_in_pages() {
    let work = 4096;
    let shapes: Vec<Shape> = (0..50_000).map(|index| match index % 3 { 0 => Shape::Empty, 1 => Shape::Dot(index as u32), _ => Shape::Segment { start: 1.5, end: index as u64 } }).collect();
    let shape_pages = (shapes.len() * size_of::<Shape>()).div_ceil(work);
    assert!(drain_turns(shapes, work).turns <= shape_pages + 16);
    let kinds: Vec<Kind> = (0..50_000).map(|index| [Kind::Empty, Kind::Solid, Kind::Liquid][index % 3]).collect();
    let kind_pages = kinds.len().div_ceil(work);
    assert!(drain_turns(kinds, work).turns <= kind_pages + 16);
    let optionals: Vec<Option<[f32; 3]>> = (0..50_000).map(|index| (index % 2 == 0).then_some([1.0, 2.0, 3.0])).collect();
    let optional_pages = (optionals.len() * size_of::<Option<[f32; 3]>>()).div_ceil(work);
    assert!(drain_turns(optionals, work).turns <= optional_pages + 16);
    let arrays: Vec<[DerivedCell; 2]> = (0..20_000).map(|index| [cell(index), cell(index + 1)]).collect();
    let array_pages = (arrays.len() * size_of::<[DerivedCell; 2]>()).div_ceil(work);
    assert!(drain_turns(arrays, work).turns <= array_pages + 16);
    let generics: Vec<Pair<u16>> = (0..50_000).map(|index| Pair { left: index as u16, right: 1 }).collect();
    let generic_pages = (generics.len() * size_of::<Pair<u16>>()).div_ceil(work);
    assert!(drain_turns(generics, work).turns <= generic_pages + 16);
}

#[test]
fn a_vector_with_an_owning_field_stays_element_wise() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🧮️plain-elements/🔣️.json")).unwrap();
    let count = fixture["elementWiseControlCount"].as_u64().unwrap() as usize;
    let minimum = fixture["elementWiseControlMinimumTurns"].as_u64().unwrap() as usize;
    let named: Vec<Named> = (0..count).map(|index| Named { name: format!("n{index}"), id: index as u32 }).collect();
    let receipt = drain_turns(named, 65536);
    assert!(receipt.turns >= minimum, "{} turns for {count} owning elements", receipt.turns);
    let labeled: Vec<Labeled> = (0..count).map(|index| if index % 2 == 0 { Labeled::Plain(index as u32) } else { Labeled::Text(index.to_string()) }).collect();
    assert!(drain_turns(labeled, 65536).turns >= minimum);
}
