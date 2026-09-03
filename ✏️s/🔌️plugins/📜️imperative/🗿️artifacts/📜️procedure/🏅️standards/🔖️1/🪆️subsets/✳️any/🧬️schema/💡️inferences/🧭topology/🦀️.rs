//! 🧭 `topology` — one named inference: execution-order topology stats derived from the
//! imperative document's own `Path`/`Step` tree (depth-first execution order, per-step nesting
//! depth, cycle-freedom, total step count across every nested `Step::bodies` scope).

use crate::artifacts::procedure::Path;
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
mod tests {
    use super::*;
    use crate::artifacts::procedure::Step;
    use std::collections::BTreeMap as StdBTreeMap;

    fn step(id: &str, bodies: StdBTreeMap<String, Path>) -> Step {
        Step { id: id.into(), kind: "noop".into(), params: Default::default(), bodies }
    }

    #[semio_framework_async_macros::async_test]
    async fn a_flat_path_orders_steps_at_depth_zero() {
        let path = Path { steps: vec![step("a", StdBTreeMap::new()), step("b", StdBTreeMap::new())] };
        let topology = compute_procedure_topology(&path);
        assert_eq!(topology.topo_order, vec!["a".to_string(), "b".to_string()]);
        assert_eq!(topology.depth.get("a"), Some(&0));
        assert_eq!(topology.depth.get("b"), Some(&0));
        assert_eq!(topology.node_count, 2);
        assert!(topology.cycle_free);
    }

    #[semio_framework_async_macros::async_test]
    async fn a_nested_body_step_sits_one_depth_deeper_and_still_counts() {
        let inner = Path { steps: vec![step("inner", StdBTreeMap::new())] };
        let mut bodies = StdBTreeMap::new();
        bodies.insert("then".to_string(), inner);
        let path = Path { steps: vec![step("outer", bodies)] };
        let topology = compute_procedure_topology(&path);
        assert_eq!(topology.topo_order, vec!["outer".to_string(), "inner".to_string()]);
        assert_eq!(topology.depth.get("outer"), Some(&0));
        assert_eq!(topology.depth.get("inner"), Some(&1));
        assert_eq!(topology.node_count, 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn an_empty_path_is_the_zero_topology() {
        assert_eq!(compute_procedure_topology(&Path::new()), ProcedureTopology::default());
    }
}
//#endregion 🧪️Tests
