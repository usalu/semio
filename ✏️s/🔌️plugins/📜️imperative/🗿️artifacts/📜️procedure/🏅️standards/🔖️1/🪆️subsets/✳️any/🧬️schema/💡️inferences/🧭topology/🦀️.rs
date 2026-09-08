//! 🧭 `topology` — one named inference: execution-order topology stats derived from the
//! imperative document's own `Path`/`Step` tree (depth-first execution order, per-step nesting
//! depth, cycle-freedom, total step count across every nested `Step::bodies` scope).

use crate::Path;
use std::collections::BTreeMap;

//#region 🔖️Topology
/// 🧭 Whole-snapshot topology summary — a plain scalar inference (no per-entity `InferredField`
/// caching: a `Path`/`Step` document is a tree, not a general graph, so a single depth-first walk
/// on every read is both cheap at pilot scale and already total/deterministic; there is no
/// per-entity dependency-hash boundary the way puzzle3d's flatten chain has).
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ProcedureTopology {
    pub topo_order: Vec<String>,
    pub depth: BTreeMap<String, u32>,
    pub cycle_free: bool,
    pub node_count: u32,
}

impl Default for ProcedureTopology {
    fn default() -> Self {
        Self { topo_order: Vec::new(), depth: BTreeMap::new(), cycle_free: true, node_count: 0 }
    }
}

/// 🧭 Depth-first walk over `path.steps` and every nested `Step::bodies` scope (a `BTreeMap`, so
/// scope-key iteration order is already deterministic); `cycle_free` is always `true` — a
/// `Path`/`Step` tree cannot contain a cycle by construction (no step ever references another
/// step's id, only owns nested `Path`s).
pub fn compute_procedure_topology(path: &Path) -> ProcedureTopology {
    let mut topo_order = Vec::new();
    let mut depth = BTreeMap::new();
    walk(path, 0, &mut topo_order, &mut depth);
    ProcedureTopology { node_count: topo_order.len() as u32, topo_order, depth, cycle_free: true }
}

fn walk(path: &Path, level: u32, topo_order: &mut Vec<String>, depth: &mut BTreeMap<String, u32>) {
    for step in &path.steps {
        topo_order.push(step.id.clone());
        depth.insert(step.id.clone(), level);
        for nested in step.bodies.values() {
            walk(nested, level + 1, topo_order, depth);
        }
    }
}
//#endregion 🔖️Topology

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
