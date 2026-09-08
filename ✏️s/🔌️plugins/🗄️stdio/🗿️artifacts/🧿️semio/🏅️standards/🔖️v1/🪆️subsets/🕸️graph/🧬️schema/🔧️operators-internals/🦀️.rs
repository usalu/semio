//! ➕️ Graph combination and transformation operators: union, products, complement, contractions, line graphs — NetworkX `operators` module parity, built generically over `graph_core::Storage<P, D>`.

use graph_core::{AttrView, Directed, Directedness, EdgeId, GraphError, GraphView, HandleId, NodeId, Normal, PortModel, PropertyBag, Storage};
use std::collections::{BTreeMap, BTreeSet};

// #region 🔖️Internal
// 🧰️ Shared copy/remap plumbing reused by every set operator, `reverse`, and the contraction family. `NodeId`/`HandleId` are both plain `u64` aliases in the frozen core, so `P::Endpoint: From<NodeId>` is satisfiable for both `Normal` (`Endpoint = NodeId`) and `Ported` (`Endpoint = HandleId`) via std's reflexive `impl<T> From<T> for T`; the `Ported` branch never actually exercises that conversion (its endpoints always resolve through `endpoint_as_handle`), so this is purely a compile-time bridge, never a semantic shortcut.

/// 🔁️ Resolves an old typed endpoint to its counterpart in a freshly built storage, using `node_map`/`handle_map` recorded while copying nodes/handles. Panics if the endpoint's underlying node or handle wasn't copied first — every call site copies nodes (and their handles) before touching edges.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn translate_endpoint<P: PortModel>(endpoint: P::Endpoint, node_map: &BTreeMap<NodeId, NodeId>, handle_map: &BTreeMap<HandleId, HandleId>) -> P::Endpoint
where
    P::Endpoint: From<NodeId>,
{
    match P::endpoint_as_handle(endpoint) {
        Some(old_handle) => {
            let new_handle = *handle_map.get(&old_handle).expect("endpoint handle must be copied before its edges");
            P::try_handle_endpoint(new_handle).expect("handle id round-trips through try_handle_endpoint for Ported")
        }
        None => {
            let old_node = P::endpoint_as_u64(endpoint);
            let new_node = *node_map.get(&old_node).expect("endpoint node must be copied before its edges");
            P::Endpoint::from(new_node)
        }
    }
}

/// 📋️ Copies one edge of `src` into `dst`, translating its endpoints through `node_map`/`handle_map`; no-operation if the edge id vanished between enumeration and lookup.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn copy_edge<P: PortModel, D: Directedness>(dst: &mut Storage<P, D>, src: &Storage<P, D>, edge_id: EdgeId, node_map: &BTreeMap<NodeId, NodeId>, handle_map: &BTreeMap<HandleId, HandleId>)
where
    P::Endpoint: From<NodeId>,
{
    let Some((old_source, old_target)) = src.edge_endpoints(edge_id) else { return };
    let attrs = src.edge_attrs(edge_id).cloned().unwrap_or_else(PropertyBag::new);
    let new_source = translate_endpoint::<P>(old_source, node_map, handle_map);
    let new_target = translate_endpoint::<P>(old_target, node_map, handle_map);
    dst.add_edge_with(new_source, new_target, attrs);
}

/// 🏗️ Copies exactly the given node ids (and, for ported storages, every handle anchored on them) from `src` into `dst`, preserving `NodeId` values via `add_node_with_id`. Returns the (here, identity-ish but explicit) old→new node and handle maps that `copy_edge`/`translate_endpoint` need.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn copy_nodes_from<P: PortModel, D: Directedness>(dst: &mut Storage<P, D>, src: &Storage<P, D>, ids: impl IntoIterator<Item = NodeId>) -> (BTreeMap<NodeId, NodeId>, BTreeMap<HandleId, HandleId>) {
    let mut node_map = BTreeMap::new();
    let mut handle_map = BTreeMap::new();
    for id in ids {
        let attrs = src.node_attrs(id).cloned().unwrap_or_else(PropertyBag::new);
        let new_id = dst.add_node_with_id(id, attrs);
        node_map.insert(id, new_id);
        if P::HAS_PORTS {
            for &old_handle in src.handles(id) {
                if let Some(new_handle) = dst.add_handle(new_id) {
                    handle_map.insert(old_handle, new_handle);
                }
            }
        }
    }
    (node_map, handle_map)
}

/// 🆕️ Copies the whole of `src` into `dst` under a **fresh** id space (`add_node_with` instead of `add_node_with_id`) — the `disjoint_union` building block.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn copy_all_fresh<P: PortModel, D: Directedness>(dst: &mut Storage<P, D>, src: &Storage<P, D>)
where
    P::Endpoint: From<NodeId>,
{
    let mut node_map = BTreeMap::new();
    let mut handle_map = BTreeMap::new();
    for id in src.nodes() {
        let attrs = src.node_attrs(id).cloned().unwrap_or_else(PropertyBag::new);
        let new_id = dst.add_node_with(attrs);
        node_map.insert(id, new_id);
        if P::HAS_PORTS {
            for &old_handle in src.handles(id) {
                if let Some(new_handle) = dst.add_handle(new_id) {
                    handle_map.insert(old_handle, new_handle);
                }
            }
        }
    }
    for edge in src.edges() {
        copy_edge(dst, src, edge.id, &node_map, &handle_map);
    }
}
// #endregion 🔖️Internal

// #region 🔖️SetOperators
/// 🤝️ NetworkX `union`: `g` and `h` must have disjoint node id sets — returns `GraphError::AmbiguousSolution` otherwise (a runtime check standing in for what NetworkX enforces at call time). On success, every node/edge of both graphs is copied into a fresh storage, ids unchanged.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn union<P: PortModel, D: Directedness>(g: &Storage<P, D>, h: &Storage<P, D>) -> Result<Storage<P, D>, GraphError>
where
    P::Endpoint: From<NodeId>,
{
    let g_nodes: BTreeSet<NodeId> = g.nodes().collect();
    if h.nodes().any(|n| g_nodes.contains(&n)) {
        return Err(GraphError::AmbiguousSolution("union requires g and h to have disjoint node id sets".to_string()));
    }
    let mut dst: Storage<P, D> = Storage::new();
    let (gn, gh) = copy_nodes_from(&mut dst, g, g.nodes());
    for edge in g.edges() {
        copy_edge(&mut dst, g, edge.id, &gn, &gh);
    }
    let (hn, hh) = copy_nodes_from(&mut dst, h, h.nodes());
    for edge in h.edges() {
        copy_edge(&mut dst, h, edge.id, &hn, &hh);
    }
    Ok(dst)
}

/// 🆕️ NetworkX `disjoint_union`: always succeeds by relabelling both inputs into a fresh, non-overlapping id space — `g`'s nodes first (in `NodeId` order), then `h`'s.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn disjoint_union<P: PortModel, D: Directedness>(g: &Storage<P, D>, h: &Storage<P, D>) -> Storage<P, D>
where
    P::Endpoint: From<NodeId>,
{
    let mut dst: Storage<P, D> = Storage::new();
    copy_all_fresh(&mut dst, g);
    copy_all_fresh(&mut dst, h);
    dst
}

/// 🧩️ NetworkX `semio_compose_rs`: union of node/edge sets in the *shared* id space; wherever both graphs define the same node or (for non-multi storages) the same edge, `h`'s attributes overwrite `g`'s matching keys (via `PropertyBag::extend`), non-conflicting keys from both survive. For ported (multi-edge) storages there is no "same edge" identity beyond a fresh parallel edge, so both graphs' edges simply accumulate.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_compose_rs<P: PortModel, D: Directedness>(g: &Storage<P, D>, h: &Storage<P, D>) -> Storage<P, D>
where
    P::Endpoint: From<NodeId>,
{
    let mut dst: Storage<P, D> = Storage::new();
    let (gn, gh) = copy_nodes_from(&mut dst, g, g.nodes());
    for edge in g.edges() {
        copy_edge(&mut dst, g, edge.id, &gn, &gh);
    }
    let (hn, hh) = copy_nodes_from(&mut dst, h, h.nodes());
    for edge in h.edges() {
        copy_edge(&mut dst, h, edge.id, &hn, &hh);
    }
    dst
}

/// ∩ NetworkX-style `intersection`: nodes present in both `g` and `h` (by `NodeId`), edges present in both (by `(u, v)` pair) — attributes copied from `g`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn intersection<P: PortModel, D: Directedness>(g: &Storage<P, D>, h: &Storage<P, D>) -> Storage<P, D>
where
    P::Endpoint: From<NodeId>,
{
    let mut dst: Storage<P, D> = Storage::new();
    let g_nodes: BTreeSet<NodeId> = g.nodes().collect();
    let common: Vec<NodeId> = h.nodes().filter(|n| g_nodes.contains(n)).collect();
    let (node_map, handle_map) = copy_nodes_from(&mut dst, g, common);
    for edge in g.edges() {
        if !node_map.contains_key(&edge.u) || !node_map.contains_key(&edge.v) {
            continue;
        }
        if h.edges_between(edge.u, edge.v).next().is_none() {
            continue;
        }
        copy_edge(&mut dst, g, edge.id, &node_map, &handle_map);
    }
    dst
}

/// ➖️ NetworkX `difference`: every node of `g`, but only the edges of `g` that have no counterpart `(u, v)` in `h`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn difference<P: PortModel, D: Directedness>(g: &Storage<P, D>, h: &Storage<P, D>) -> Storage<P, D>
where
    P::Endpoint: From<NodeId>,
{
    let mut dst: Storage<P, D> = Storage::new();
    let (node_map, handle_map) = copy_nodes_from(&mut dst, g, g.nodes());
    for edge in g.edges() {
        if h.edges_between(edge.u, edge.v).next().is_some() {
            continue;
        }
        copy_edge(&mut dst, g, edge.id, &node_map, &handle_map);
    }
    dst
}

/// ⊕ NetworkX `symmetric_difference`: nodes of both `g` and `h`; edges present in exactly one of the two (by `(u, v)` pair). Node attrs prefer `g`'s when a node exists in both.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn symmetric_difference<P: PortModel, D: Directedness>(g: &Storage<P, D>, h: &Storage<P, D>) -> Storage<P, D>
where
    P::Endpoint: From<NodeId>,
{
    let mut dst: Storage<P, D> = Storage::new();
    let all_nodes: BTreeSet<NodeId> = g.nodes().chain(h.nodes()).collect();
    let mut node_map = BTreeMap::new();
    let mut g_handle_map = BTreeMap::new();
    let mut h_handle_map = BTreeMap::new();
    for id in all_nodes {
        let attrs = g.node_attrs(id).or_else(|| h.node_attrs(id)).cloned().unwrap_or_else(PropertyBag::new);
        let new_id = dst.add_node_with_id(id, attrs);
        node_map.insert(id, new_id);
        if P::HAS_PORTS {
            if g.contains_node(id) {
                for &old_handle in g.handles(id) {
                    if let Some(new_handle) = dst.add_handle(new_id) {
                        g_handle_map.insert(old_handle, new_handle);
                    }
                }
            }
            if h.contains_node(id) {
                for &old_handle in h.handles(id) {
                    if let Some(new_handle) = dst.add_handle(new_id) {
                        h_handle_map.insert(old_handle, new_handle);
                    }
                }
            }
        }
    }
    for edge in g.edges() {
        if h.edges_between(edge.u, edge.v).next().is_some() {
            continue;
        }
        copy_edge(&mut dst, g, edge.id, &node_map, &g_handle_map);
    }
    for edge in h.edges() {
        if g.edges_between(edge.u, edge.v).next().is_some() {
            continue;
        }
        copy_edge(&mut dst, h, edge.id, &node_map, &h_handle_map);
    }
    dst
}
// #endregion 🔖️SetOperators

// #region 🔖️Complement
/// 🌓️ NetworkX `complement`: same nodes as `g`, an edge between `u ≠ v` iff `g` has none there. The result collapses to `Normal` port model — a complement graph has no natural handle/port structure to inherit from an arbitrary `P`. Directed inputs complement every ordered pair; undirected inputs complement every unordered pair once (relying on `Storage`'s symmetric undirected adjacency).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn complement<P: PortModel, D: Directedness>(g: &Storage<P, D>) -> Storage<Normal, D> {
    let mut dst: Storage<Normal, D> = Storage::new();
    let nodes: Vec<NodeId> = g.nodes().collect();
    for &n in &nodes {
        let attrs = g.node_attrs(n).cloned().unwrap_or_else(PropertyBag::new);
        dst.add_node_with_id(n, attrs);
    }
    if D::DIRECTED {
        for &u in &nodes {
            for &v in &nodes {
                if u != v && g.edges_between(u, v).next().is_none() {
                    dst.add_edge(u, v);
                }
            }
        }
    } else {
        for (i, &u) in nodes.iter().enumerate() {
            for &v in &nodes[i + 1..] {
                if g.edges_between(u, v).next().is_none() {
                    dst.add_edge(u, v);
                }
            }
        }
    }
    dst
}

/// ↩️ NetworkX `reverse`: swaps every edge's source/target, preserving node/edge attributes and `NodeId`s (and, for ported storages, a fresh handle per original handle). Only meaningful for directed graphs — an undirected overload is deliberately not provided since reversing an undirected edge is a no-operation by definition, and offering one would just invite dead call sites.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn reverse<P: PortModel>(g: &Storage<P, Directed>) -> Storage<P, Directed>
where
    P::Endpoint: From<NodeId>,
{
    let mut dst: Storage<P, Directed> = Storage::new();
    let (node_map, handle_map) = copy_nodes_from(&mut dst, g, g.nodes());
    for edge in g.edges() {
        let Some((old_source, old_target)) = g.edge_endpoints(edge.id) else { continue };
        let attrs = g.edge_attrs(edge.id).cloned().unwrap_or_else(PropertyBag::new);
        let new_source = translate_endpoint::<P>(old_target, &node_map, &handle_map);
        let new_target = translate_endpoint::<P>(old_source, &node_map, &handle_map);
        dst.add_edge_with(new_source, new_target, attrs);
    }
    dst
}
// #endregion 🔖️Complement

// #region 🔖️Products
/// 🔗️ True iff `g` has an edge between `a` and `b` (order-sensitive for directed `g`; `Storage`'s undirected adjacency is already symmetric, so order doesn't matter there).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn adjacent<D: Directedness>(g: &Storage<Normal, D>, a: NodeId, b: NodeId) -> bool {
    g.edges_between(a, b).next().is_some()
}

/// 🏗️ Builds the product's node set — one fresh node per `(u, v) ∈ nodes(g) × nodes(h)` — and the interning map every product function needs. Iteration is over `g.nodes()` then `h.nodes()`, both already `BTreeMap`-sorted, so id assignment is deterministic.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn product_skeleton<D: Directedness>(g: &Storage<Normal, D>, h: &Storage<Normal, D>) -> (Storage<Normal, D>, BTreeMap<(NodeId, NodeId), NodeId>) {
    let mut dst: Storage<Normal, D> = Storage::new();
    let mut map = BTreeMap::new();
    for u in g.nodes() {
        for v in h.nodes() {
            map.insert((u, v), dst.add_node());
        }
    }
    (dst, map)
}

/// 🔗️ Shared quadruple-loop edge builder for the four products below: wires `(u1, v1)-(u2, v2)` whenever `include` says so. Deliberately not optimized past `O(|Vg|² · |Vh|²)` — products are meant for small graphs here (see module docs).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn build_product_edges<D: Directedness>(g: &Storage<Normal, D>, h: &Storage<Normal, D>, dst: &mut Storage<Normal, D>, map: &BTreeMap<(NodeId, NodeId), NodeId>, mut include: impl FnMut(NodeId, NodeId, NodeId, NodeId) -> bool) {
    let g_nodes: Vec<NodeId> = g.nodes().collect();
    let h_nodes: Vec<NodeId> = h.nodes().collect();
    for &u1 in &g_nodes {
        for &v1 in &h_nodes {
            for &u2 in &g_nodes {
                for &v2 in &h_nodes {
                    if include(u1, v1, u2, v2) {
                        dst.add_edge(map[&(u1, v1)], map[&(u2, v2)]);
                    }
                }
            }
        }
    }
}

/// ⊞ NetworkX `cartesian_product`: `(u1,v1)-(u2,v2)` iff (`u1==u2` and `v1~v2` in `h`) or (`v1==v2` and `u1~u2` in `g`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn cartesian_product<D: Directedness>(g: &Storage<Normal, D>, h: &Storage<Normal, D>) -> (Storage<Normal, D>, BTreeMap<(NodeId, NodeId), NodeId>) {
    let (mut dst, map) = product_skeleton(g, h);
    build_product_edges(g, h, &mut dst, &map, |u1, v1, u2, v2| (u1 == u2 && adjacent(h, v1, v2)) || (v1 == v2 && adjacent(g, u1, u2)));
    (dst, map)
}

/// ⊗ NetworkX `tensor_product` (categorical product): `(u1,v1)-(u2,v2)` iff `u1~u2` in `g` AND `v1~v2` in `h`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn tensor_product<D: Directedness>(g: &Storage<Normal, D>, h: &Storage<Normal, D>) -> (Storage<Normal, D>, BTreeMap<(NodeId, NodeId), NodeId>) {
    let (mut dst, map) = product_skeleton(g, h);
    build_product_edges(g, h, &mut dst, &map, |u1, v1, u2, v2| adjacent(g, u1, u2) && adjacent(h, v1, v2));
    (dst, map)
}

/// ⊠ NetworkX `strong_product`: union of the cartesian and tensor edge sets.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn strong_product<D: Directedness>(g: &Storage<Normal, D>, h: &Storage<Normal, D>) -> (Storage<Normal, D>, BTreeMap<(NodeId, NodeId), NodeId>) {
    let (mut dst, map) = product_skeleton(g, h);
    build_product_edges(g, h, &mut dst, &map, |u1, v1, u2, v2| (u1 == u2 && adjacent(h, v1, v2)) || (v1 == v2 && adjacent(g, u1, u2)) || (adjacent(g, u1, u2) && adjacent(h, v1, v2)));
    (dst, map)
}

/// 📖️ NetworkX `lexicographic_product`: `(u1,v1)-(u2,v2)` iff `u1~u2` in `g`, OR (`u1==u2` and `v1~v2` in `h`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn lexicographic_product<D: Directedness>(g: &Storage<Normal, D>, h: &Storage<Normal, D>) -> (Storage<Normal, D>, BTreeMap<(NodeId, NodeId), NodeId>) {
    let (mut dst, map) = product_skeleton(g, h);
    build_product_edges(g, h, &mut dst, &map, |u1, v1, u2, v2| adjacent(g, u1, u2) || (u1 == u2 && adjacent(h, v1, v2)));
    (dst, map)
}
// #endregion 🔖️Products

// #region 🔖️Power
/// 🧭️ Tiny local BFS (not a public API): nodes reachable from `src` within `k` hops via `out_neighbors`, keyed by `NodeId` for determinism.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn bfs_within<D: Directedness>(g: &Storage<Normal, D>, src: NodeId, k: usize) -> BTreeSet<NodeId> {
    let mut seen: BTreeSet<NodeId> = BTreeSet::from([src]);
    let mut frontier = vec![src];
    for _ in 0..k {
        let mut next = Vec::new();
        for &u in &frontier {
            for v in g.out_neighbors(u) {
                if seen.insert(v) {
                    next.push(v);
                }
            }
        }
        if next.is_empty() {
            break;
        }
        frontier = next;
    }
    seen
}

/// 🔋️ NetworkX `power`: edge `u-v` (`u ≠ v`) iff the shortest-path distance from `u` to `v` is at most `k` hops. Requires `k ≥ 1`, matching NetworkX's own precondition.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn power<D: Directedness>(g: &Storage<Normal, D>, k: usize) -> Storage<Normal, D> {
    assert!(k >= 1, "power requires k >= 1");
    let mut dst: Storage<Normal, D> = Storage::new();
    let nodes: Vec<NodeId> = g.nodes().collect();
    for &n in &nodes {
        let attrs = g.node_attrs(n).cloned().unwrap_or_else(PropertyBag::new);
        dst.add_node_with_id(n, attrs);
    }
    for &src in &nodes {
        for v in bfs_within(g, src, k) {
            if v != src {
                dst.add_edge(src, v);
            }
        }
    }
    dst
}
// #endregion 🔖️Power

// #region 🔖️Contraction
/// 🫂️ NetworkX `contracted_nodes`: merges `v` into `u` — `v` is removed, its attrs merged onto `u` (`u`'s keys win on conflict, matching `PropertyBag::extend` order), and every edge that touched `v` is redirected to `u`. An edge that becomes a `u-u` self-loop *because of* the merge (i.e. it originally touched `v`) is dropped unless `self_loops` is true; a self-loop that already existed on `u` before the merge is always kept. Always returns a fresh copy — mirrors NetworkX's non-mutating default (`copy=True`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn contracted_nodes<P: PortModel, D: Directedness>(g: &Storage<P, D>, u: NodeId, v: NodeId, self_loops: bool) -> Storage<P, D>
where
    P::Endpoint: From<NodeId>,
{
    let mut dst: Storage<P, D> = Storage::new();
    let mut node_map = BTreeMap::new();
    let mut handle_map = BTreeMap::new();
    for n in g.nodes() {
        if n == v {
            continue;
        }
        let mut attrs = g.node_attrs(n).cloned().unwrap_or_else(PropertyBag::new);
        if n == u {
            if let Some(v_attrs) = g.node_attrs(v) {
                let mut merged = v_attrs.clone();
                merged.extend(attrs);
                attrs = merged;
            }
        }
        let new_id = dst.add_node_with_id(n, attrs);
        node_map.insert(n, new_id);
        if P::HAS_PORTS {
            for &old_handle in g.handles(n) {
                if let Some(new_handle) = dst.add_handle(new_id) {
                    handle_map.insert(old_handle, new_handle);
                }
            }
        }
    }
    let new_u = node_map[&u];
    node_map.insert(v, new_u);
    if P::HAS_PORTS {
        for &old_handle in g.handles(v) {
            if let Some(new_handle) = dst.add_handle(new_u) {
                handle_map.insert(old_handle, new_handle);
            }
        }
    }
    for edge in g.edges() {
        let resolve = |n: NodeId| if n == v { u } else { n };
        let (ru, rv) = (resolve(edge.u), resolve(edge.v));
        let created_by_merge = ru == u && rv == u && !(edge.u == u && edge.v == u);
        if created_by_merge && !self_loops {
            continue;
        }
        copy_edge(&mut dst, g, edge.id, &node_map, &handle_map);
    }
    dst
}

/// 🔗️ NetworkX `contracted_edge`: contracts the two endpoints of `edge`; `GraphError::EdgeNotFound` if it doesn't exist.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn contracted_edge<P: PortModel, D: Directedness>(g: &Storage<P, D>, edge: EdgeId, self_loops: bool) -> Result<Storage<P, D>, GraphError>
where
    P::Endpoint: From<NodeId>,
{
    let Some(edge_ref) = g.edges().find(|e| e.id == edge) else {
        return Err(GraphError::EdgeNotFound(edge));
    };
    Ok(contracted_nodes(g, edge_ref.u, edge_ref.v, self_loops))
}

/// 🧱️ NetworkX `quotient_graph`: one node per partition block, edge between (or, thoroughly, within — see below) blocks iff some original edge crosses there. Choice: an edge entirely inside one block produces a self-loop on that block's node, mirroring NetworkX's default `quotient_graph` relation (`∃ u∈B, v∈C : u~v` in `g`, which includes `B==C`) rather than silently dropping intra-block structure. Returns the quotient graph (always `Normal`, since blocks have no port structure) plus the original-node→block-node map.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn quotient_graph<P: PortModel, D: Directedness>(g: &Storage<P, D>, partition: &[Vec<NodeId>]) -> (Storage<Normal, D>, BTreeMap<NodeId, NodeId>) {
    let mut dst: Storage<Normal, D> = Storage::new();
    let mut block_of: BTreeMap<NodeId, NodeId> = BTreeMap::new();
    for block in partition {
        let block_id = dst.add_node();
        for &n in block {
            block_of.insert(n, block_id);
        }
    }
    for edge in g.edges() {
        let (Some(&bu), Some(&bv)) = (block_of.get(&edge.u), block_of.get(&edge.v)) else { continue };
        dst.add_edge(bu, bv);
    }
    (dst, block_of)
}
// #endregion 🔖️Contraction

// #region 🔖️LineGraph
/// 🪢️ NetworkX `line_graph`: one node per edge of `g`. Undirected `g`: two line-graph nodes connect iff their original edges share an endpoint. Directed `g`: connects `e1 -> e2` iff `e1`'s target is `e2`'s source (NetworkX's directed line graph). The result stays `Storage<Normal, D>` — same directedness as the input — branching internally on `D::DIRECTED` rather than hardcoding `Undirected`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn line_graph<D: Directedness>(g: &Storage<Normal, D>) -> (Storage<Normal, D>, BTreeMap<EdgeId, NodeId>) {
    let mut dst: Storage<Normal, D> = Storage::new();
    let mut node_of_edge: BTreeMap<EdgeId, NodeId> = BTreeMap::new();
    let edges: Vec<_> = g.edges().collect();
    for edge in &edges {
        node_of_edge.insert(edge.id, dst.add_node());
    }
    if D::DIRECTED {
        for e1 in &edges {
            for e2 in &edges {
                if e1.id != e2.id && e1.v == e2.u {
                    dst.add_edge(node_of_edge[&e1.id], node_of_edge[&e2.id]);
                }
            }
        }
    } else {
        for i in 0..edges.len() {
            for e2 in &edges[i + 1..] {
                let e1 = &edges[i];
                if e1.u == e2.u || e1.u == e2.v || e1.v == e2.u || e1.v == e2.v {
                    dst.add_edge(node_of_edge[&e1.id], node_of_edge[&e2.id]);
                }
            }
        }
    }
    (dst, node_of_edge)
}
// #endregion 🔖️LineGraph

// #region 🔖️Mycielski
/// 🕸️ Mycielski construction: for every node `v`, adds a shadow node `v'`; for every edge `u-v`, adds `u-v'` and `v-u'` (alongside the original `u-v`); adds one apex node `z` connected to every shadow node. Self-loops in `g` are skipped for shadow wiring (undefined for this construction) but their original endpoint is still present as a node.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mycielskian<D: Directedness>(g: &Storage<Normal, D>) -> Storage<Normal, D> {
    let mut dst: Storage<Normal, D> = Storage::new();
    let nodes: Vec<NodeId> = g.nodes().collect();
    let mut original_id = BTreeMap::new();
    let mut shadow_id = BTreeMap::new();
    for &n in &nodes {
        let attrs = g.node_attrs(n).cloned().unwrap_or_else(PropertyBag::new);
        original_id.insert(n, dst.add_node_with_id(n, attrs));
    }
    for &n in &nodes {
        shadow_id.insert(n, dst.add_node());
    }
    let apex = dst.add_node();
    for edge in g.edges() {
        if edge.u == edge.v {
            continue;
        }
        dst.add_edge(original_id[&edge.u], original_id[&edge.v]);
        dst.add_edge(original_id[&edge.u], shadow_id[&edge.v]);
        dst.add_edge(original_id[&edge.v], shadow_id[&edge.u]);
    }
    for &n in &nodes {
        dst.add_edge(shadow_id[&n], apex);
    }
    dst
}
// #endregion 🔖️Mycielski

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
