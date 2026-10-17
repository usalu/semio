//! 🕸️ Half-edge topology of the mesh surface: the topology error and the half-edge structure with its manifold, boundary and neighbour queries. See [`super`].

use super::*;

// #region 🔖️Topology
/// ⚠️ Why a [`TriMesh`] failed to become a [`HalfedgeTopology`]: it wasn't yet an oriented
/// 2-manifold.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TopologyError {
    NonManifoldEdge { a: u32, b: u32, face_count: usize },
    InconsistentOrientation { a: u32, b: u32 },
    NonManifoldVertex(u32),
    DegenerateTriangle(usize),
}

impl std::fmt::Display for TopologyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonManifoldEdge { a, b, face_count } => write!(f, "edge ({a},{b}) has {face_count} incident faces"),
            Self::InconsistentOrientation { a, b } => write!(f, "edge ({a},{b}) is traversed the same direction by two faces"),
            Self::NonManifoldVertex(v) => write!(f, "vertex {v} has more than one incident fan"),
            Self::DegenerateTriangle(idx) => write!(f, "triangle {idx} is degenerate"),
        }
    }
}

impl std::error::Error for TopologyError {}

#[derive(Clone, Copy, Debug)]
struct Halfedge {
    pub(super) origin: u32,
    twin: Option<u32>,
    pub(super) next: u32,
}

/// 🕸️ Halfedge adjacency over an oriented 2-manifold [`TriMesh`], own lightweight structure built
/// fresh from `EdgeMap` checks each time (not a general-purpose editable mesh kernel — read-only,
/// exists purely to answer "who is my twin" and "walk this boundary").
pub struct HalfedgeTopology {
    halfedges: Vec<Halfedge>,
}

impl HalfedgeTopology {
    /// 🏗️ Builds halfedge adjacency, rejecting anything that isn't already an oriented 2-manifold:
    /// every edge has at most 2 incident faces, each directed edge appears at most once, and every
    /// vertex's incident faces form a single fan (no pinch points).
    pub fn build(mesh: &TriMesh) -> Result<Self, TopologyError> {
        for (idx, tri) in mesh.triangles.iter().enumerate() {
            if tri[0] == tri[1] || tri[1] == tri[2] || tri[0] == tri[2] {
                return Err(TopologyError::DegenerateTriangle(idx));
            }
        }
        let edges = mesh.edge_map();
        for (&(a, b), faces) in &edges {
            if faces.len() > 2 {
                return Err(TopologyError::NonManifoldEdge { a, b, face_count: faces.len() });
            }
        }
        let mut halfedges = Vec::with_capacity(mesh.triangles.len() * 3);
        for tri in &mesh.triangles {
            let base = halfedges.len() as u32;
            for (k, &origin) in tri.iter().enumerate() {
                halfedges.push(Halfedge { origin, twin: None, next: base + ((k as u32 + 1) % 3) });
            }
        }
        let mut directed: HashMap<(u32, u32), u32> = HashMap::new();
        for (he_id, he) in halfedges.iter().enumerate() {
            let dest = halfedges[he.next as usize].origin;
            if directed.insert((he.origin, dest), he_id as u32).is_some() {
                return Err(TopologyError::InconsistentOrientation { a: he.origin.min(dest), b: he.origin.max(dest) });
            }
        }
        let twins: Vec<Option<u32>> = halfedges
            .iter()
            .map(|he| {
                let dest = halfedges[he.next as usize].origin;
                directed.get(&(dest, he.origin)).copied()
            })
            .collect();
        for (he, twin) in halfedges.iter_mut().zip(twins) {
            he.twin = twin;
        }
        let topology = Self { halfedges };
        topology.check_vertex_fans(mesh.vertex_count())?;
        Ok(topology)
    }

    fn destination(&self, he: u32) -> u32 {
        self.halfedges[self.halfedges[he as usize].next as usize].origin
    }

    /// 🌀️ Rejects pinch vertices: walks the fan around every vertex (via `twin`/`next`) from each
    /// outgoing halfedge and confirms every halfedge originating there is reached by a single walk.
    fn check_vertex_fans(&self, vertex_count: usize) -> Result<(), TopologyError> {
        let mut outgoing: Vec<Vec<u32>> = vec![Vec::new(); vertex_count];
        for (id, he) in self.halfedges.iter().enumerate() {
            outgoing[he.origin as usize].push(id as u32);
        }
        for (v, out) in outgoing.iter().enumerate() {
            if out.len() <= 1 {
                continue;
            }
            let start = *out.iter().find(|&&he| self.halfedges[he as usize].twin.is_none()).unwrap_or(&out[0]);
            let mut visited = HashSet::new();
            let mut cur = start;
            loop {
                visited.insert(cur);
                let prev_around = self.prev(cur);
                match self.halfedges[prev_around as usize].twin {
                    Some(t) if !visited.contains(&t) => cur = t,
                    _ => break,
                }
            }
            if visited.len() != out.len() {
                return Err(TopologyError::NonManifoldVertex(v as u32));
            }
        }
        Ok(())
    }

    fn prev(&self, he: u32) -> u32 {
        let mut cur = he;
        loop {
            let next = self.halfedges[cur as usize].next;
            if next == he {
                return cur;
            }
            cur = next;
        }
    }

    /// 🔁️ Every boundary loop (halfedges with no twin), each as an ordered vertex cycle; empty
    /// when the mesh is already closed.
    pub fn boundary_loops(&self) -> Vec<Vec<u32>> {
        let mut boundary_out: BTreeMap<u32, u32> = BTreeMap::new();
        for (id, he) in self.halfedges.iter().enumerate() {
            if he.twin.is_none() {
                boundary_out.insert(he.origin, id as u32);
            }
        }
        let mut visited = vec![false; self.halfedges.len()];
        let mut loops = Vec::new();
        for (&_start_vertex, &start_he) in &boundary_out {
            if visited[start_he as usize] {
                continue;
            }
            let mut verts = Vec::new();
            let mut he = start_he;
            loop {
                visited[he as usize] = true;
                verts.push(self.halfedges[he as usize].origin);
                let dest = self.destination(he);
                let Some(&next_he) = boundary_out.get(&dest) else { break };
                he = next_he;
                if he == start_he {
                    break;
                }
            }
            loops.push(verts);
        }
        loops
    }
}
// #endregion 🔖️Topology
