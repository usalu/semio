use super::*;
use serde_json::Value;

struct GraphType<const N: usize>;

fn supports_graph(nodes: &[Value], index: usize) -> bool {
    macro_rules! select { ($($n:literal),*) => { match index {
        $($n => conjunctive_retirement_support::<GraphType<$n>>(|| nodes[index]["supported"].as_bool().unwrap() && nodes[index]["children"].as_array().unwrap().iter().all(|child| supports_graph(nodes, child.as_u64().unwrap() as usize))),)*
        _ => panic!("undeclared support test type"),
    } }; }
    select!(0,1,2,3,4,5,6,7)
}

#[test]
fn conjunctive_support_matches_recursive_portable_graphs_without_heap() {
    let fixture: Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let nodes = row["nodes"].as_array().unwrap();
        for node in nodes { for child in node["children"].as_array().unwrap() { assert!((child.as_u64().unwrap() as usize) < nodes.len()); } }
        let (result, heap) = crate::observe_retirement_allocations(|| supports_graph(nodes, row["root"].as_u64().unwrap() as usize));
        assert_eq!(result, row["expected"].as_bool().unwrap(), "{}", row["id"]);
        assert_eq!(heap, (0,0));
    }
}

#[derive(semio_framework_value_derive::RetireOwned)]
#[value(crate="crate")]
enum Recursive { Leaf(u8), Next(Box<Recursive>) }
#[derive(semio_framework_value_derive::RetireOwned)]
#[value(crate="crate")]
enum UnsupportedRecursive { Next(Box<UnsupportedRecursive>), Unsupported(std::vec::IntoIter<u8>) }
#[derive(semio_framework_value_derive::RetireOwned)]
#[value(crate="crate")]
struct MutualLeft(Option<Box<MutualRight>>);
#[derive(semio_framework_value_derive::RetireOwned)]
#[value(crate="crate")]
struct MutualRight(Option<Box<MutualLeft>>);

#[test]
fn derived_recursive_retirement_support_checks_every_noncyclic_component() {
    use crate::retirement::RetireOwned;
    let (supported, heap) = crate::observe_retirement_allocations(|| (Recursive::controlled_retirement_supported(), UnsupportedRecursive::controlled_retirement_supported(), MutualLeft::controlled_retirement_supported(), MutualRight::controlled_retirement_supported()));
    assert_eq!(supported, (true,false,true,true));
    assert_eq!(heap, (0,0));
}

#[test]
fn conjunctive_support_restores_exact_frontier_after_unwind() {
    assert!(conjunctive_retirement_support::<GraphType<0>>(|| {
        assert!(std::panic::catch_unwind(|| conjunctive_retirement_support::<GraphType<1>>(|| panic!("test-only support callback panic"))).is_err());
        assert!(conjunctive_retirement_support::<GraphType<0>>(|| panic!("active outer frontier was lost")));
        assert!(!conjunctive_retirement_support::<GraphType<1>>(|| false));
        true
    }));
    assert!(!conjunctive_retirement_support::<GraphType<0>>(|| false));
}

#[test]
fn conjunctive_support_keeps_queries_current_and_threads_independent() {
    let available = std::cell::Cell::new(true);
    for expected in [true,false,true] {
        available.set(expected);
        let (actual, heap) = crate::observe_retirement_allocations(|| conjunctive_retirement_support::<GraphType<0>>(|| available.get() && conjunctive_retirement_support::<GraphType<0>>(|| panic!("cycle callback repeated"))));
        assert_eq!(actual, expected);
        assert_eq!(heap, (0,0));
    }
    std::thread::scope(|scope| {
        let children: Vec<_> = (0..3).map(|_| scope.spawn(|| {
            assert!(!conjunctive_retirement_support::<GraphType<0>>(|| false));
            assert!(conjunctive_retirement_support::<GraphType<0>>(|| conjunctive_retirement_support::<GraphType<0>>(|| panic!("thread cycle callback repeated"))));
        })).collect();
        for child in children { child.join().unwrap(); }
    });
}
