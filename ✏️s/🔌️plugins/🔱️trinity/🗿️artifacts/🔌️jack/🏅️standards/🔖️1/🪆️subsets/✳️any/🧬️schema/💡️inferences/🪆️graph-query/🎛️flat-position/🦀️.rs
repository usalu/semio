//! 🎛️ Derived positions consume retained endpoints and composition-owned numeric displacement.
use crate::{retained_jack_graph, JackGraphQuery};
use semio_framework_schema_composition::ChildRefFields;
use semio_framework_value::{ValueError,ValueRefusalKind};
use std::collections::{BTreeMap,BTreeSet};
#[derive(Clone,Copy,Debug,Default,PartialEq,value_derive::ToValue,value_derive::FromValue)]
#[value(rename_all="camelCase")]
pub struct JackFlatPositionUv {pub u:f64,pub v:f64}
#[derive(Clone,Debug,Default,PartialEq,value_derive::ToValue,value_derive::FromValue)]
#[value(rename_all="camelCase")]
pub struct JackFlatPosition {pub positions:BTreeMap<String,JackFlatPositionUv>}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}
/// 🧭️ Exact owner admission and topology validation precede a deterministic component walk.
pub fn infer_jack_flat_position(selected:ChildRefFields<'_>,root:Option<&str>,graph:&dyn JackGraphQuery)->Result<JackFlatPosition,ValueError>{
    let graph = retained_jack_graph(selected, graph)?;
    let mut nodes = (0..graph.node_count())
        .map(|index| graph.node_id(index).ok_or_else(|| invalid("Jack graph capability omitted a declared node")))
        .collect::<Result<Vec<_>, _>>()?;
    nodes.sort_unstable_by(|left, right| crate::identity_order(left, right));
    if nodes.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err(invalid("Jack inference requires unique node identities"));
    }
    let seed = match root {
        Some(id) => Some(nodes[nodes.binary_search_by(|node| crate::identity_order(node, id))
            .map_err(|_| invalid("Jack flattened root requires retained node"))?]),
        None => nodes.first().copied(),
    };

    let mut edges = (0..graph.edge_count())
        .map(|index| graph.edge(index).ok_or_else(|| invalid("Jack graph capability omitted a declared edge")))
        .collect::<Result<Vec<_>, _>>()?;
    edges.sort_unstable_by(|left, right| crate::identity_order(left.id, right.id));
    if edges.windows(2).any(|pair| pair[0].id == pair[1].id) {
        return Err(invalid("Jack inference requires unique edge identities"));
    }
    let members = nodes.iter().copied().collect::<BTreeSet<_>>();
    let mut incoming = BTreeSet::new();
    let mut outgoing = BTreeMap::<_, Vec<_>>::new();
    for edge in edges {
        if !members.contains(edge.source) || !members.contains(edge.target) {
            return Err(invalid("Jack inference edge requires retained endpoints"));
        }
        let (u, v) = graph.displacement(edge.id)?;
        incoming.insert(edge.target);
        outgoing.entry(edge.source).or_default().push((edge.target, finite_position(u, v)?));
    }

    let mut flat = BTreeMap::new();
    if let Some(seed) = seed {
        extend(&outgoing, &mut flat, seed)?;
    }
    for node in nodes.iter().copied().filter(|node| !incoming.contains(node)) {
        extend(&outgoing, &mut flat, node)?;
    }
    for node in nodes {
        extend(&outgoing, &mut flat, node)?;
    }
    Ok(JackFlatPosition { positions: flat.into_iter().map(|(id, position)| (id.to_owned(), position)).collect() })
}

fn finite_position(u: f64, v: f64) -> Result<JackFlatPositionUv, ValueError> {
    if !u.is_finite() || !v.is_finite() {
        return Err(invalid("Jack flattened position must be finite"));
    }
    Ok(JackFlatPositionUv { u, v })
}

/// 🧭️ Visits all nodes reachable from the seed, so the remaining nodes retain their original indegrees.
fn extend<'a>(
    outgoing: &BTreeMap<&str, Vec<(&'a str, JackFlatPositionUv)>>,
    flat: &mut BTreeMap<&'a str, JackFlatPositionUv>,
    seed: &'a str,
) -> Result<(), ValueError> {
    use std::collections::btree_map::Entry;

    let Entry::Vacant(seed_position) = flat.entry(seed) else { return Ok(()) };
    seed_position.insert(JackFlatPositionUv::default());
    let mut stack = vec![seed];
    while let Some(source) = stack.pop() {
        let held = flat[source];
        for &(target, offset) in outgoing.get(source).into_iter().flatten() {
            if let Entry::Vacant(position) = flat.entry(target) {
                position.insert(finite_position(held.u + offset.u, held.v + offset.v)?);
                stack.push(target);
            }
        }
    }
    Ok(())
}
