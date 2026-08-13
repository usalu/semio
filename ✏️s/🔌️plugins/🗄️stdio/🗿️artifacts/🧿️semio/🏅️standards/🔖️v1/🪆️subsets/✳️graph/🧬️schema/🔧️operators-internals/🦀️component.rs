//! ➕️ Graph combination and transformation operators: union, products, complement, contractions, line graphs — NetworkX `operators` module parity, built generically over `graph_core::Storage<P, D>`.

use graph_core::{AttrView, Directed, Directedness, EdgeId, GraphError, GraphView, HandleId, NodeId, Normal, PortModel, PropertyBag, Storage};
use std::collections::{BTreeMap, BTreeSet};

// #region 🔖️Internal
// 🧰️ Shared copy/remap plumbing reused by every set operator, `reverse`, and the contraction family. `NodeId`/`HandleId` are both plain `u64` aliases in the frozen core, so `P::Endpoint: From<NodeId>` is satisfiable for both `Normal` (`Endpoint = NodeId`) and `Ported` (`Endpoint = HandleId`) via std's reflexive `impl<T> From<T> for T`; the `Ported` branch never actually exercises that conversion (its endpoints always resolve through `endpoint_as_handle`), so this is purely a compile-time bridge, never a semantic shortcut.

/// 🔁️ Resolves an old typed endpoint to its counterpart in a freshly built storage, using `node_map`/`handle_map` recorded while copying nodes/handles. Panics if the endpoint's underlying node or handle wasn't copied first — every call site copies nodes (and their handles) before touching edges.
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
fn adjacent<D: Directedness>(g: &Storage<Normal, D>, a: NodeId, b: NodeId) -> bool {
    g.edges_between(a, b).next().is_some()
}

/// 🏗️ Builds the product's node set — one fresh node per `(u, v) ∈ nodes(g) × nodes(h)` — and the interning map every product function needs. Iteration is over `g.nodes()` then `h.nodes()`, both already `BTreeMap`-sorted, so id assignment is deterministic.
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
pub fn cartesian_product<D: Directedness>(g: &Storage<Normal, D>, h: &Storage<Normal, D>) -> (Storage<Normal, D>, BTreeMap<(NodeId, NodeId), NodeId>) {
    let (mut dst, map) = product_skeleton(g, h);
    build_product_edges(g, h, &mut dst, &map, |u1, v1, u2, v2| (u1 == u2 && adjacent(h, v1, v2)) || (v1 == v2 && adjacent(g, u1, u2)));
    (dst, map)
}

/// ⊗ NetworkX `tensor_product` (categorical product): `(u1,v1)-(u2,v2)` iff `u1~u2` in `g` AND `v1~v2` in `h`.
pub fn tensor_product<D: Directedness>(g: &Storage<Normal, D>, h: &Storage<Normal, D>) -> (Storage<Normal, D>, BTreeMap<(NodeId, NodeId), NodeId>) {
    let (mut dst, map) = product_skeleton(g, h);
    build_product_edges(g, h, &mut dst, &map, |u1, v1, u2, v2| adjacent(g, u1, u2) && adjacent(h, v1, v2));
    (dst, map)
}

/// ⊠ NetworkX `strong_product`: union of the cartesian and tensor edge sets.
pub fn strong_product<D: Directedness>(g: &Storage<Normal, D>, h: &Storage<Normal, D>) -> (Storage<Normal, D>, BTreeMap<(NodeId, NodeId), NodeId>) {
    let (mut dst, map) = product_skeleton(g, h);
    build_product_edges(g, h, &mut dst, &map, |u1, v1, u2, v2| (u1 == u2 && adjacent(h, v1, v2)) || (v1 == v2 && adjacent(g, u1, u2)) || (adjacent(g, u1, u2) && adjacent(h, v1, v2)));
    (dst, map)
}

/// 📖️ NetworkX `lexicographic_product`: `(u1,v1)-(u2,v2)` iff `u1~u2` in `g`, OR (`u1==u2` and `v1~v2` in `h`).
pub fn lexicographic_product<D: Directedness>(g: &Storage<Normal, D>, h: &Storage<Normal, D>) -> (Storage<Normal, D>, BTreeMap<(NodeId, NodeId), NodeId>) {
    let (mut dst, map) = product_skeleton(g, h);
    build_product_edges(g, h, &mut dst, &map, |u1, v1, u2, v2| adjacent(g, u1, u2) || (u1 == u2 && adjacent(h, v1, v2)));
    (dst, map)
}
// #endregion 🔖️Products

// #region 🔖️Power
/// 🧭️ Tiny local BFS (not a public API): nodes reachable from `src` within `k` hops via `out_neighbors`, keyed by `NodeId` for determinism.
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
mod tests {
    use super::*;
    use graph_core::Undirected;

    fn und_edge(a: NodeId, b: NodeId) -> Storage<Normal, Undirected> {
        let mut g: Storage<Normal, Undirected> = Storage::new();
        g.add_node_with_id(a, PropertyBag::new());
        g.add_node_with_id(b, PropertyBag::new());
        g.add_edge(a, b);
        g
    }

    fn attrs_of(pairs: &[(&str, &str)]) -> PropertyBag {
        let mut bag = PropertyBag::new();
        for (k, v) in pairs {
            bag.insert(k.to_string(), graph_core::PropertyValue::String(v.to_string()));
        }
        bag
    }

    // #subregion SetOperators
    #[test]
    fn union_of_disjoint_graphs_merges_everything() {
        let g = und_edge(0, 1);
        let mut h: Storage<Normal, Undirected> = Storage::new();
        h.add_node_with_id(10, PropertyBag::new());
        h.add_node_with_id(11, PropertyBag::new());
        h.add_edge(10, 11);

        let merged = union(&g, &h).expect("disjoint node sets must union");
        assert_eq!(merged.node_count(), 4);
        assert_eq!(merged.edge_count(), 2);
        assert!(merged.edges_between(0, 1).next().is_some());
        assert!(merged.edges_between(10, 11).next().is_some());
    }

    #[test]
    fn union_errors_on_overlapping_node_ids() {
        let g = und_edge(0, 1);
        let h = und_edge(1, 2);
        let err = union(&g, &h).expect_err("shared node id 1 must be rejected");
        assert!(matches!(err, GraphError::AmbiguousSolution(_)));
    }

    #[test]
    fn compose_lets_h_overwrite_shared_node_and_edge_attrs() {
        let mut g: Storage<Normal, Undirected> = Storage::new();
        g.add_node_with_id(0, attrs_of(&[("color", "red")]));
        g.add_node_with_id(1, PropertyBag::new());
        g.add_edge_with(0, 1, attrs_of(&[("kind", "g")]));

        let mut h: Storage<Normal, Undirected> = Storage::new();
        h.add_node_with_id(0, attrs_of(&[("color", "blue")]));
        h.add_node_with_id(1, PropertyBag::new());
        h.add_node_with_id(2, PropertyBag::new());
        h.add_edge_with(0, 1, attrs_of(&[("kind", "h")]));

        let composed = semio_compose_rs(&g, &h);
        assert_eq!(composed.node_count(), 3);
        assert_eq!(composed.node_attrs(0).unwrap().get("color").unwrap().as_str(), Some("blue"));
        let edge = composed.edges_between(0, 1).next().unwrap();
        assert_eq!(composed.edge_attrs(edge.id).unwrap().get("kind").unwrap().as_str(), Some("h"));
    }

    #[test]
    fn disjoint_union_relabels_both_graphs_into_fresh_ids() {
        let g = und_edge(0, 1);
        let h = und_edge(0, 1);
        let merged = disjoint_union(&g, &h);
        assert_eq!(merged.node_count(), 4);
        assert_eq!(merged.edge_count(), 2);
    }

    #[test]
    fn intersection_keeps_only_shared_nodes_and_edges() {
        let mut g: Storage<Normal, Undirected> = Storage::new();
        for id in [0, 1, 2] {
            g.add_node_with_id(id, PropertyBag::new());
        }
        g.add_edge(0, 1);
        g.add_edge(1, 2);

        let mut h: Storage<Normal, Undirected> = Storage::new();
        for id in [0, 1, 3] {
            h.add_node_with_id(id, PropertyBag::new());
        }
        h.add_edge(0, 1);

        let inter = intersection(&g, &h);
        assert_eq!(inter.node_count(), 2);
        assert_eq!(inter.edge_count(), 1);
        assert!(inter.edges_between(0, 1).next().is_some());
    }

    #[test]
    fn difference_keeps_gs_nodes_and_only_gs_own_edges() {
        let mut g: Storage<Normal, Undirected> = Storage::new();
        for id in [0, 1, 2] {
            g.add_node_with_id(id, PropertyBag::new());
        }
        g.add_edge(0, 1);
        g.add_edge(1, 2);

        let mut h: Storage<Normal, Undirected> = Storage::new();
        for id in [0, 1] {
            h.add_node_with_id(id, PropertyBag::new());
        }
        h.add_edge(0, 1);

        let diff = difference(&g, &h);
        assert_eq!(diff.node_count(), 3);
        assert_eq!(diff.edge_count(), 1);
        assert!(diff.edges_between(1, 2).next().is_some());
        assert!(diff.edges_between(0, 1).next().is_none());
    }

    #[test]
    fn symmetric_difference_keeps_edges_unique_to_either_side() {
        let g = und_edge(0, 1);
        let mut h: Storage<Normal, Undirected> = Storage::new();
        h.add_node_with_id(0, PropertyBag::new());
        h.add_node_with_id(1, PropertyBag::new());
        h.add_node_with_id(2, PropertyBag::new());
        h.add_edge(0, 1);
        h.add_edge(1, 2);

        let symdiff = symmetric_difference(&g, &h);
        assert_eq!(symdiff.node_count(), 3);
        assert_eq!(symdiff.edge_count(), 1);
        assert!(symdiff.edges_between(1, 2).next().is_some());
        assert!(symdiff.edges_between(0, 1).next().is_none());
    }
    // #endsubregion

    // #subregion Complement
    #[test]
    fn complement_of_a_path_matches_a_hand_count() {
        // 🔺️ Path 0-1-2 (undirected, 3 nodes, 2 edges) has C(3,2)-2 = 1 missing pair: 0-2.
        let mut g: Storage<Normal, Undirected> = Storage::new();
        for id in [0, 1, 2] {
            g.add_node_with_id(id, PropertyBag::new());
        }
        g.add_edge(0, 1);
        g.add_edge(1, 2);

        let comp = complement(&g);
        assert_eq!(comp.node_count(), 3);
        assert_eq!(comp.edge_count(), 1);
        assert!(comp.edges_between(0, 2).next().is_some());
    }

    #[test]
    fn reverse_round_trips_the_edge_set() {
        let mut g: Storage<Normal, Directed> = Storage::new();
        for id in [0, 1, 2] {
            g.add_node_with_id(id, PropertyBag::new());
        }
        g.add_edge(0, 1);
        g.add_edge(1, 2);

        let once = reverse(&g);
        assert!(once.edges_between(1, 0).next().is_some());
        assert!(once.edges_between(2, 1).next().is_some());

        let twice = reverse(&once);
        let original: BTreeSet<(NodeId, NodeId)> = g.edges().map(|e| (e.u, e.v)).collect();
        let round_tripped: BTreeSet<(NodeId, NodeId)> = twice.edges().map(|e| (e.u, e.v)).collect();
        assert_eq!(original, round_tripped);
    }
    // #endsubregion

    // #subregion Products
    #[test]
    fn cartesian_product_of_two_2node_paths_is_a_4cycle() {
        let g = und_edge(0, 1);
        let h = und_edge(0, 1);
        let (prod, map) = cartesian_product(&g, &h);
        assert_eq!(prod.node_count(), 4);
        assert_eq!(prod.edge_count(), 4);
        for &n in map.values() {
            assert_eq!(prod.degree(n), 2);
        }
    }

    #[test]
    fn tensor_product_requires_adjacency_on_both_sides() {
        let g = und_edge(0, 1);
        let h = und_edge(0, 1);
        let (prod, _map) = tensor_product(&g, &h);
        assert_eq!(prod.node_count(), 4);
        // 🔗️ (0,0)-(1,1) and (0,1)-(1,0): exactly 2 edges.
        assert_eq!(prod.edge_count(), 2);
    }

    #[test]
    fn strong_product_is_the_union_of_cartesian_and_tensor() {
        let g = und_edge(0, 1);
        let h = und_edge(0, 1);
        let (cart, _) = cartesian_product(&g, &h);
        let (tens, _) = tensor_product(&g, &h);
        let (strong, _) = strong_product(&g, &h);
        assert!(strong.edge_count() >= cart.edge_count());
        assert!(strong.edge_count() >= tens.edge_count());
    }

    #[test]
    fn lexicographic_product_includes_cross_block_edges() {
        let g = und_edge(0, 1);
        let h = und_edge(0, 1);
        let (lex, _map) = lexicographic_product(&g, &h);
        assert_eq!(lex.node_count(), 4);
        // 🔗️ g-adjacent pairs pull in all 4 combinations of (v1, v2) for u1~u2, plus the within-block h edges.
        assert!(lex.edge_count() >= 4);
    }
    // #endsubregion

    // #subregion Power
    #[test]
    fn power_connects_nodes_within_k_hops() {
        let mut g: Storage<Normal, Undirected> = Storage::new();
        for id in [0, 1, 2, 3] {
            g.add_node_with_id(id, PropertyBag::new());
        }
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 3);

        let p2 = power(&g, 2);
        assert!(p2.edges_between(0, 2).next().is_some());
        assert!(p2.edges_between(0, 3).next().is_none());

        let p3 = power(&g, 3);
        assert!(p3.edges_between(0, 3).next().is_some());
    }
    // #endsubregion

    // #subregion Contraction
    #[test]
    fn contracted_nodes_merges_v_into_u_and_respects_self_loops_flag() {
        let mut g: Storage<Normal, Undirected> = Storage::new();
        for id in [0, 1, 2] {
            g.add_node_with_id(id, PropertyBag::new());
        }
        g.add_edge(0, 1); // the edge that becomes a self-loop when 1 merges into 0
        g.add_edge(1, 2);

        let dropped = contracted_nodes(&g, 0, 1, false);
        assert_eq!(dropped.node_count(), 2);
        assert!(!dropped.contains_node(1));
        assert!(dropped.edges_between(0, 0).next().is_none());
        assert!(dropped.edges_between(0, 2).next().is_some());

        let kept = contracted_nodes(&g, 0, 1, true);
        assert!(kept.edges_between(0, 0).next().is_some());
        assert!(kept.edges_between(0, 2).next().is_some());
    }

    #[test]
    fn contracted_edge_contracts_its_own_endpoints() {
        let mut g: Storage<Normal, Undirected> = Storage::new();
        for id in [0, 1, 2] {
            g.add_node_with_id(id, PropertyBag::new());
        }
        let e = g.add_edge(0, 1);
        g.add_edge(1, 2);

        let contracted = contracted_edge(&g, e, false).expect("edge exists");
        assert_eq!(contracted.node_count(), 2);

        let err = contracted_edge(&g, 9999, false).expect_err("missing edge id");
        assert!(matches!(err, GraphError::EdgeNotFound(9999)));
    }

    #[test]
    fn quotient_graph_connects_blocks_that_have_a_crossing_edge() {
        let mut g: Storage<Normal, Undirected> = Storage::new();
        for id in [0, 1, 2, 3] {
            g.add_node_with_id(id, PropertyBag::new());
        }
        g.add_edge(0, 1);
        g.add_edge(2, 3);
        g.add_edge(1, 2);

        let (quotient, block_of) = quotient_graph(&g, &[vec![0, 1], vec![2, 3]]);
        assert_eq!(quotient.node_count(), 2);
        let (b0, b1) = (block_of[&0], block_of[&2]);
        assert!(quotient.edges_between(b0, b1).next().is_some());
    }
    // #endsubregion

    // #subregion LineGraph
    #[test]
    fn line_graph_of_a_triangle_is_itself_a_triangle() {
        let mut g: Storage<Normal, Undirected> = Storage::new();
        for id in [0, 1, 2] {
            g.add_node_with_id(id, PropertyBag::new());
        }
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(0, 2);

        let (lg, node_of_edge) = line_graph(&g);
        assert_eq!(lg.node_count(), 3);
        assert_eq!(lg.edge_count(), 3);
        for &n in node_of_edge.values() {
            assert_eq!(lg.degree(n), 2);
        }
    }
    // #endsubregion

    // #subregion Mycielski
    #[test]
    fn mycielskian_of_a_single_edge_is_the_5cycle() {
        let g = und_edge(0, 1);
        let myc = mycielskian(&g);
        assert_eq!(myc.node_count(), 5);
        assert_eq!(myc.edge_count(), 5);
        for n in myc.nodes() {
            assert_eq!(myc.degree(n), 2);
        }
    }
    // #endsubregion
}
// #endregion 🔖️Tests
