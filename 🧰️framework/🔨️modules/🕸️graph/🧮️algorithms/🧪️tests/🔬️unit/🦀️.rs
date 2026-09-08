
use super::*;

// 🚫️async: E5-class executor bridge, sanctioned per R4 clause 5 — `#[test]` cannot run
// an `async fn` directly (std has no executor for it), so every async test body in this
// module runs through this instead. Sound because this crate performs no real I/O: every
// future here resolves on its first poll, so a single poll (never a spin-park loop) is
// enough — panics loudly if that invariant is ever violated rather than hanging.
fn block_on_test<F: std::future::Future>(fut: F) -> F::Output {
    use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
    fn noop(_: *const ()) {}
    fn clone_raw(_: *const ()) -> RawWaker {
        RawWaker::new(std::ptr::null(), &VTABLE)
    }
    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone_raw, noop, noop, noop);
    let raw = RawWaker::new(std::ptr::null(), &VTABLE);
    let waker = unsafe { Waker::from_raw(raw) };
    let mut cx = Context::from_waker(&waker);
    let mut fut = Box::pin(fut);
    match fut.as_mut().poll(&mut cx) {
        Poll::Ready(v) => v,
        Poll::Pending => panic!("block_on_test: future did not complete synchronously"),
    }
}

fn adj_from(n: usize, edges: &[(usize, usize)], directed: bool) -> Adjacency {
    adjacency(n, edges, directed)
}

#[test]
fn bfs_order_visits_reachable_nodes_breadth_first() {
    block_on_test(async {
        let adj = adj_from(5, &[(0, 1), (0, 2), (1, 3), (2, 4)], true);
        let order = bfs_order(&adj, &[0]);
        assert_eq!(order, vec![0, 1, 2, 3, 4]);
    });
}

#[test]
fn bfs_distances_unreachable_is_none() {
    block_on_test(async {
        let adj = adj_from(3, &[(0, 1)], true);
        let dist = bfs_distances(&adj, 0);
        assert_eq!(dist, vec![Some(0), Some(1), None]);
    });
}

#[test]
fn topo_sort_orders_dependencies_before_dependents() {
    block_on_test(async {
        let adj = adj_from(4, &[(0, 1), (0, 2), (1, 3), (2, 3)], true);
        let order = topo_sort(&adj).expect("acyclic");
        let pos = |n: usize| order.iter().position(|&x| x == n).unwrap();
        assert!(pos(0) < pos(1));
        assert!(pos(1) < pos(3));
        assert!(pos(2) < pos(3));
    });
}

#[test]
fn topo_sort_detects_cycle() {
    block_on_test(async {
        let adj = adj_from(3, &[(0, 1), (1, 2), (2, 0)], true);
        let err = topo_sort(&adj).unwrap_err();
        assert_eq!(err.cycle.len(), 3);
    });
}

#[test]
fn topo_levels_groups_independent_nodes() {
    block_on_test(async {
        let adj = adj_from(4, &[(0, 2), (1, 2), (2, 3)], true);
        let levels = topo_levels(&adj).expect("acyclic");
        assert_eq!(levels[0], vec![0, 1]);
        assert_eq!(levels[1], vec![2]);
        assert_eq!(levels[2], vec![3]);
    });
}

#[test]
fn would_create_cycle_detects_back_edge() {
    block_on_test(async {
        let adj = adj_from(3, &[(0, 1), (1, 2)], true);
        assert!(would_create_cycle(&adj, 2, 0));
        assert!(!would_create_cycle(&adj, 0, 2));
    });
}

#[test]
fn would_create_cycle_ids_matches_index_version() {
    block_on_test(async {
        let existing = vec![("a".to_string(), "b".to_string()), ("b".to_string(), "c".to_string())];
        assert!(would_create_cycle_ids(&existing, "c", "a"));
        assert!(!would_create_cycle_ids(&existing, "a", "c"));
    });
}

#[test]
fn connected_components_groups_weak_components() {
    block_on_test(async {
        let adj = adj_from(5, &[(0, 1), (1, 2), (3, 4)], true);
        let labels = connected_components(&adj);
        assert_eq!(labels[0], labels[1]);
        assert_eq!(labels[1], labels[2]);
        assert_eq!(labels[3], labels[4]);
        assert_ne!(labels[0], labels[3]);
    });
}

#[test]
fn strongly_connected_components_finds_cycle_as_one_component() {
    block_on_test(async {
        let adj = adj_from(4, &[(0, 1), (1, 2), (2, 0), (2, 3)], true);
        let sccs = strongly_connected_components(&adj);
        let cyclic = sccs.iter().find(|c| c.contains(&0)).unwrap();
        assert_eq!(cyclic, &vec![0, 1, 2]);
        assert!(sccs.iter().any(|c| c == &vec![3]));
    });
}

#[test]
fn union_find_unions_and_queries_sets() {
    block_on_test(async {
        let mut uf = UnionFind::new(4);
        uf.union(0, 1);
        uf.union(2, 3);
        assert!(uf.same_set(0, 1));
        assert!(!uf.same_set(0, 2));
    });
}

#[test]
fn dijkstra_prefers_cheaper_longer_path() {
    block_on_test(async {
        let adj = adj_from(3, &[(0, 1), (1, 2), (0, 2)], true);
        let mut weights = HashMap::new();
        weights.insert((0, 1), 1.0);
        weights.insert((1, 2), 1.0);
        weights.insert((0, 2), 5.0);
        let dist = dijkstra(&adj, &weights, 0);
        assert_eq!(dist[2], Some(2.0));
    });
}

#[test]
fn id_index_is_deterministic_and_sorted() {
    block_on_test(async {
        let edges = [("c".to_string(), "a".to_string()), ("a".to_string(), "b".to_string())];
        let index = IdIndex::from_edges(edges.iter().map(|(a, b)| (a.as_str(), b.as_str())));
        assert_eq!(index.id_of(0), Some("a"));
        assert_eq!(index.id_of(1), Some("b"));
        assert_eq!(index.id_of(2), Some("c"));
    });
}

#[test]
fn id_index_from_ids_dedupes_and_reports_len() {
    block_on_test(async {
        let index = IdIndex::from_ids(["b", "a", "a"].into_iter());
        assert_eq!(index.len(), 2);
        assert!(!index.is_empty());
        assert_eq!(index.index_of("a"), Some(0));
        assert_eq!(index.index_of("z"), None);
        assert!(IdIndex::from_ids(std::iter::empty()).is_empty());
    });
}

#[test]
fn adjacency_accessors_expose_node_count_and_neighbor_lists() {
    block_on_test(async {
        let adj = adj_from(3, &[(0, 1), (0, 2)], true);
        assert_eq!(adj.node_count(), 3);
        assert_eq!(adj.out_neighbors(0), &[1, 2]);
        assert_eq!(adj.in_neighbors(1), &[0]);
        assert!(adj.in_neighbors(0).is_empty());
    });
}

#[test]
fn bfs_order_ignores_out_of_range_seeds() {
    block_on_test(async {
        let adj = adj_from(2, &[(0, 1)], true);
        assert_eq!(bfs_order(&adj, &[5]), Vec::<usize>::new());
    });
}

#[test]
fn topo_levels_detects_cycle() {
    block_on_test(async {
        let adj = adj_from(3, &[(0, 1), (1, 2), (2, 0)], true);
        let err = topo_levels(&adj).unwrap_err();
        assert_eq!(err.cycle.len(), 3);
    });
}

#[test]
fn would_create_cycle_self_loop_is_always_a_cycle() {
    block_on_test(async {
        let adj = adj_from(2, &[(0, 1)], true);
        assert!(would_create_cycle(&adj, 1, 1));
    });
}

#[test]
fn is_reachable_from_equals_to_is_trivially_true() {
    block_on_test(async {
        let adj = adj_from(2, &[], true);
        assert!(is_reachable(&adj, 1, 1));
    });
}

#[test]
fn dijkstra_unreachable_node_stays_none() {
    block_on_test(async {
        let adj = adj_from(3, &[(0, 1)], true);
        let dist = dijkstra(&adj, &HashMap::new(), 0);
        assert_eq!(dist, vec![Some(0.0), Some(1.0), None]);
    });
}

#[test]
fn dijkstra_out_of_range_from_returns_empty() {
    block_on_test(async {
        let adj = adj_from(2, &[(0, 1)], true);
        assert_eq!(dijkstra(&adj, &HashMap::new(), 9), vec![None, None]);
    });
}
