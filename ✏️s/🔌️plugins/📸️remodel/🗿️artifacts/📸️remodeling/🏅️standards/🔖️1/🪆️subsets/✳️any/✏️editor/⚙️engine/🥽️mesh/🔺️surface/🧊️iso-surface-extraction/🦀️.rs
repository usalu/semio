//! 🧊️ Iso-surface extraction of the mesh surface: the marching tetrahedra over a TSDF volume with its bounded resumable preparation and the extraction from a dense field. See [`super`].

use super::*;

// #region 🔖️MarchingCubes
type Lattice = (i32, i32, i32);

/// 🔑️ Global lattice-edge key: the sorted pair of integer corner coordinates an interpolated
/// vertex lies between. Generalizes classic marching cubes' `{corner, axis}` key to also cover the
/// face-diagonal edges the tetrahedral decomposition below introduces — any two cells that share a
/// lattice edge (axis-aligned or a shared face's diagonal) compute the *identical* key, so the
/// weld map merges them into one vertex regardless of iteration order.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct EdgeKey(Lattice, Lattice);

impl EdgeKey {
    pub(super) fn new(a: Lattice, b: Lattice) -> Self {
        if a <= b {
            Self(a, b)
        } else {
            Self(b, a)
        }
    }
}

/// ⚖️ One corner sample: its global lattice coordinate, world position and scalar field value.
#[derive(Clone, Copy)]
struct Corner {
    pub(super) key: Lattice,
    pos: [f64; 3],
    val: f64,
}

fn lattice_add(c: Lattice, d: (i32, i32, i32)) -> Lattice {
    (c.0 + d.0, c.1 + d.1, c.2 + d.2)
}

/// 🧊️ The fixed 6-tetrahedron decomposition of a unit cube along its `c0`-`c6` space diagonal
/// (Doi & Koide 1991). Every cube in the lattice uses this *same* relative pattern, so any two
/// cubes sharing a face always assign identical global vertices to identical relative corner
/// slots on that shared face — which is exactly what makes the face-diagonal split agree between
/// neighbors and keeps extraction crack-free without any per-face ambiguity test.
fn cube_tets(corners: &[Corner; 8]) -> [[Corner; 4]; 6] {
    let (c0, c1, c2, c3, c4, c5, c6, c7) = (corners[0], corners[1], corners[2], corners[3], corners[4], corners[5], corners[6], corners[7]);
    [[c0, c1, c2, c6], [c0, c2, c3, c6], [c0, c3, c7, c6], [c0, c7, c4, c6], [c0, c4, c5, c6], [c0, c5, c1, c6]]
}

/// 🎯️ Weight of the requested triangle normal orientation relative to `odd`: `+1.0` when the lone
/// differing vertex is inside the surface (normal should point away from it), `-1.0` when it's
/// outside (normal should point toward it) — see `march_tet`'s doc for the full derivation.
fn orient_triangle(positions: &[[f64; 3]], tri: &mut [u32; 3], odd_pos: [f64; 3], desired_sign: f64) {
    let (a, b, c) = (positions[tri[0] as usize], positions[tri[1] as usize], positions[tri[2] as usize]);
    let normal = cross3(sub3(b, a), sub3(c, a));
    let centroid = scale3(add3(add3(a, b), c), 1.0 / 3.0);
    if dot3(normal, sub3(centroid, odd_pos)) * desired_sign < 0.0 {
        tri.swap(1, 2);
    }
}

/// 🪚️ Weld helper: returns the (possibly newly created) global vertex index for the isosurface
/// crossing point between two corners, canonicalizing argument order first so both directions of
/// traversal hash to the same key and interpolate to the bit-identical position.
fn weld_edge(weld: &mut BTreeMap<EdgeKey, u32>, positions: &mut Vec<[f64; 3]>, a: Corner, b: Corner, iso: f64) -> u32 {
    let (lo, hi) = if a.key <= b.key { (a, b) } else { (b, a) };
    let key = EdgeKey::new(lo.key, hi.key);
    if let Some(&idx) = weld.get(&key) {
        return idx;
    }
    let t = ((iso - lo.val) / (hi.val - lo.val)).clamp(0.0, 1.0);
    let pos = lerp3(lo.pos, hi.pos, t);
    let idx = positions.len() as u32;
    positions.push(pos);
    weld.insert(key, idx);
    idx
}

/// 🔺️ Marches one tetrahedron: 0, 1 or 2 triangles depending on how many of its 4 corners are on
/// the inside (`val < iso`) side. Tetrahedra have triangular faces only, so — unlike a cube face —
/// there is never an ambiguous case to resolve: the crossing-edge set alone fully determines the
/// topology. Triangle winding is fixed by a purely local rule (point the normal from the "odd"
/// vertex's side toward the other side), which independently reproduces the same globally
/// consistent orientation on every tet, so two triangles sharing a welded edge always end up
/// wound oppositely along it without any extra bookkeeping.
fn march_tet(corners: [Corner; 4], iso: f64, weld: &mut BTreeMap<EdgeKey, u32>, positions: &mut Vec<[f64; 3]>, tris: &mut Vec<[u32; 3]>) {
    let inside: [bool; 4] = std::array::from_fn(|i| corners[i].val < iso);
    let inside_count = inside.iter().filter(|&&b| b).count();
    if inside_count == 0 || inside_count == 4 {
        return;
    }
    let mut cross: Vec<(usize, usize, u32)> = Vec::with_capacity(4);
    for i in 0..4 {
        for j in (i + 1)..4 {
            if inside[i] != inside[j] {
                let idx = weld_edge(weld, positions, corners[i], corners[j], iso);
                cross.push((i, j, idx));
            }
        }
    }
    if inside_count == 1 || inside_count == 3 {
        let odd = if inside_count == 1 { inside.iter().position(|&b| b).expect("one inside corner") } else { inside.iter().position(|&b| !b).expect("one outside corner") };
        let pts: Vec<u32> = cross.iter().filter(|&&(i, j, _)| i == odd || j == odd).map(|&(_, _, idx)| idx).collect();
        let mut tri = [pts[0], pts[1], pts[2]];
        let desired_sign = if inside[odd] { 1.0 } else { -1.0 };
        orient_triangle(positions, &mut tri, corners[odd].pos, desired_sign);
        tris.push(tri);
    } else {
        let inside_idx: Vec<usize> = (0..4).filter(|&i| inside[i]).collect();
        let outside_idx: Vec<usize> = (0..4).filter(|&i| !inside[i]).collect();
        let (p0, p1) = (inside_idx[0], inside_idx[1]);
        let (q0, q1) = (outside_idx[0], outside_idx[1]);
        let find = |a: usize, b: usize| cross.iter().find(|&&(i, j, _)| (i == a && j == b) || (i == b && j == a)).expect("crossing edge").2;
        let mut quad = [find(p0, q0), find(p1, q0), find(p1, q1), find(p0, q1)];
        let quad_pos: Vec<[f64; 3]> = quad.iter().map(|&i| positions[i as usize]).collect();
        let normal = cross3(sub3(quad_pos[1], quad_pos[0]), sub3(quad_pos[2], quad_pos[0]));
        let inside_avg = scale3(add3(corners[p0].pos, corners[p1].pos), 0.5);
        let outside_avg = scale3(add3(corners[q0].pos, corners[q1].pos), 0.5);
        if dot3(normal, sub3(outside_avg, inside_avg)) < 0.0 {
            quad.reverse();
        }
        tris.push([quad[0], quad[1], quad[2]]);
        tris.push([quad[0], quad[2], quad[3]]);
    }
}

/// 🧊️ Crack-free marching-cubes extraction from a cross-block-sampled scalar field, dispatching
/// every cube to [`cube_tets`] + [`march_tet`]. `bounds_min`/`bounds_max` (inclusive) are the
/// voxel-corner coordinate range to search — `remodeling_dense::TsdfVolume` does not currently expose
/// block enumeration, so the caller (who performed the integration and knows the reconstruction
/// volume's extent) supplies the search domain explicitly; this is the one deliberate signature
/// deviation from the plan's parameterless `extract_tsdf(vol, iso)`.
pub fn extract_tsdf(vol: &remodeling_dense::TsdfVolume, iso: f64, bounds_min: [i32; 3], bounds_max: [i32; 3]) -> TriMesh {
    let sample = |c: Lattice| -> Option<Corner> {
        let (sdf, _weight) = vol.sample(c.0, c.1, c.2)?;
        let p = [(c.0 as f64 + 0.5) * vol.voxel_size, (c.1 as f64 + 0.5) * vol.voxel_size, (c.2 as f64 + 0.5) * vol.voxel_size];
        Some(Corner { key: c, pos: p, val: sdf })
    };
    let mut weld: BTreeMap<EdgeKey, u32> = BTreeMap::new();
    let mut positions: Vec<[f64; 3]> = Vec::new();
    let mut triangles: Vec<[u32; 3]> = Vec::new();
    for i in bounds_min[0]..bounds_max[0] {
        for j in bounds_min[1]..bounds_max[1] {
            for k in bounds_min[2]..bounds_max[2] {
                let offsets: [(i32, i32, i32); 8] = [(0, 0, 0), (1, 0, 0), (1, 1, 0), (0, 1, 0), (0, 0, 1), (1, 0, 1), (1, 1, 1), (0, 1, 1)];
                let mut corners = [Corner { key: (0, 0, 0), pos: [0.0; 3], val: 0.0 }; 8];
                let mut all_known = true;
                for (slot, off) in offsets.iter().enumerate() {
                    match sample(lattice_add((i, j, k), *off)) {
                        Some(c) => corners[slot] = c,
                        None => {
                            all_known = false;
                            break;
                        }
                    }
                }
                if !all_known {
                    continue;
                }
                for tet in cube_tets(&corners) {
                    march_tet(tet, iso, &mut weld, &mut positions, &mut triangles);
                }
            }
        }
    }
    TriMesh { positions, triangles }
}

/// 🧊️ Resumable, bounded isosurface extraction for the interactive worker: surface nets (Gibson
/// 1998, "Constrained Elastic Surface Nets") over the TSDF lattice. One fuel unit visits one
/// lattice point `p` and its three positive axis edges; every edge whose two samples straddle `iso`
/// emits one quad joining the vertices of the four voxel cubes that share that edge (computed once,
/// cached by cube origin, so neighbours always weld to the identical vertex and the net is
/// crack-free).
///
/// 🧵️ Manifold by construction (after Schaefer, Ju & Warren 2007, "Manifold Dual Contouring"): a
/// cube does not get ONE vertex but one per cluster of its crossing edges, two crossings clustering
/// when they are consecutive on a shared cube face — a face with four crossings pairs the ones
/// around each inside corner, a rule that depends only on the face's corner signs and is therefore
/// decided identically by both cubes sharing the face. Two surface sheets passing through one cube
/// (the naive net's non-manifold edge and pinch-vertex source, common where a thin truncation-band
/// shell folds) thus stay two sheets with their own vertices.
///
/// 🪓️ The field the net is cut from is TOTAL: every lattice point has a sign, observed or not.
/// A TSDF only carries values inside the truncation band around observed surfaces, so a net over
/// the observed samples alone stops wherever observation stops — behind every surface, past every
/// frustum edge — and comes out as an open sheet full of boundary loops that no bounded hole fill
/// closes. Before the walk, a fuel-bounded flood fill from the domain's (unobserved) margin marks
/// every lattice point reachable through unobserved or non-negative samples as *outside*; every
/// other unobserved point is enclosed by observed surface and counts as *inside* (the classic
/// space-carving closure). The net over that field is closed by construction: an object seen from
/// all around meshes as one solid, a surface seen from one side as a closed shell one truncation
/// band thick, and the batch pipeline's `close_voxel` re-voxelisation is never needed.
///
/// 📐️ Why not the marching-tetrahedra split [`extract_tsdf`] uses: the six-tet decomposition
/// adds face and body diagonals, so it emits roughly one vertex per crossed lattice edge of seven
/// edge families and two to four triangles per surface cube — on the accepted 8³ sphere fixture it
/// crossed the 512-element interactive envelope halfway through the domain (292 vertices / 513
/// triangles), and the synthetic orbit reconstruction faulted at the surface stage the same way.
/// A surface net spends one vertex per surface cube and two triangles per crossed axis edge, which
/// is the leanest closed surface the lattice supports and fits the envelope with room to spare.
#[derive(semio_framework_value::RetireOwned)]
pub struct TsdfExtractionPreparation {
    iso: f64,
    bounds_min: [i32; 3],
    bounds_max: [i32; 3],
    /// 🕸️ Lattice points per step along each axis: `1` is the full-resolution net, and a coarser
    /// stride is what [`MeshPipeline`] falls back to when the full net would not fit the
    /// interactive element envelope.
    stride: i32,
    /// 🧭️ First lattice point of the carved domain: one stride below `bounds_min`, so the domain
    /// carries one ring of never-integrated points around the requested box on every side. The ring
    /// is the flood fill's seed (it is unobserved by construction) and it lets the walk emit the
    /// quads of surfaces cut by the box's faces.
    pub(super) origin: [i32; 3],
    /// 🔢️ Strided points per axis of the carved domain, ring included.
    count: [usize; 3],
    /// 🧭️ Walk cursor in domain index space.
    cursor: [usize; 3],
    /// 🪓️ One bit per domain point: reachable from the margin through unobserved or non-negative
    /// samples, i.e. outside. Meaningful once `carved`.
    outside: Vec<u64>,
    /// 🪓️ One bit per domain point: already queued by the flood fill.
    queued: Vec<u64>,
    /// 🪓️ Flood-fill frontier (domain indices).
    frontier: VecDeque<u32>,
    carved: bool,
    /// 🧵️ Per surface cube: the net vertex of each of its twelve local edges' crossing cluster
    /// (`u32::MAX` on an edge without a crossing).
    cube_vertices: BTreeMap<Lattice, [u32; 12]>,
    pub(super) positions: Vec<[f64; 3]>,
    pub(super) triangles: Vec<[u32; 3]>,
    complete: bool,
    exceeded: bool,
    /// 👁️ Emit only the observed surface: a crossing counts only when both of its samples were
    /// integrated, so the carved field never contributes geometry (see [`Self::observed_only`]).
    observed_only: bool,
}

/// 🧊️ Local corner offsets of a voxel cube, the order the edge and face tables index.
const CUBE_CORNER_OFFSETS: [(i32, i32, i32); 8] = [(0, 0, 0), (1, 0, 0), (1, 1, 0), (0, 1, 0), (0, 0, 1), (1, 0, 1), (1, 1, 1), (0, 1, 1)];

/// 🧊️ The twelve cube edges as local corner pairs.
const CUBE_EDGES: [(usize, usize); 12] = [(0, 1), (1, 2), (3, 2), (0, 3), (4, 5), (5, 6), (7, 6), (4, 7), (0, 4), (1, 5), (2, 6), (3, 7)];

/// 🧊️ The six cube faces as cyclic corner quadruples.
const CUBE_FACES: [[usize; 4]; 6] = [[0, 1, 2, 3], [4, 5, 6, 7], [0, 1, 5, 4], [3, 2, 6, 7], [0, 3, 7, 4], [1, 2, 6, 5]];

/// 🔢️ Local edge index joining two local corners (either order).
fn cube_edge_index(a: usize, b: usize) -> usize {
    CUBE_EDGES.iter().position(|&(x, y)| (x == a && y == b) || (x == b && y == a)).expect("cube corners share an edge")
}

/// 🔢️ Local edge index of the edge leaving local corner `start` (a 0/1 offset) along `axis`.
fn cube_edge_index_from(start: [i32; 3], axis: usize) -> usize {
    let mut end = start;
    end[axis] += 1;
    let corner = |offset: [i32; 3]| CUBE_CORNER_OFFSETS.iter().position(|&(x, y, z)| [x, y, z] == offset).expect("a cube corner");
    cube_edge_index(corner(start), corner(end))
}

const SURFACE_NET_ELEMENT_CEILING: usize = 512;

/// 🧊️ Coarsest lattice stride the interactive extraction falls back to before it refuses the
/// volume: a 32-cell-per-axis reconstruction domain still yields a surface at every stride up to
/// this one, and beyond it the net would be too coarse to carry the reconstruction's shape.
const MAXIMUM_SURFACE_NET_STRIDE: i32 = 8;

/// ⏱️ Lattice points one interactive pipeline dispatch visits: an unobserved point is one block
/// lookup, and even a point whose three edges all cross samples at most twelve new cube vertices.
pub const TSDF_EXTRACTION_POINTS_PER_DISPATCH: usize = 32;

/// ⏱️ Flood-fill cells one fuel unit expands: a cell is six block lookups and six bit tests, a
/// fraction of what one walked lattice point with its cube vertices costs.
const TSDF_CARVE_CELLS_PER_UNIT: usize = 4;

impl TsdfExtractionPreparation {
    pub fn new(iso: f64, bounds_min: [i32; 3], bounds_max: [i32; 3]) -> Self {
        Self::new_strided(iso, bounds_min, bounds_max, 1)
    }

    /// 🧊️ [`Self::new`] over every `stride`-th lattice point: the same net on a coarser lattice, so
    /// a volume whose full-resolution surface would not fit the element envelope still yields a
    /// (lower-resolution) closed surface instead of no surface at all.
    pub fn new_strided(iso: f64, bounds_min: [i32; 3], bounds_max: [i32; 3], stride: i32) -> Self {
        let stride = stride.max(1);
        let cells = (bounds_max[0] - bounds_min[0]).max(0) as usize * (bounds_max[1] - bounds_min[1]).max(0) as usize * (bounds_max[2] - bounds_min[2]).max(0) as usize;
        let origin = bounds_min.map(|coordinate| coordinate - stride);
        // 🔢️ Points `origin + stride·i` with `i` in `0..count` end exactly one stride past the last
        // in-box point, so the domain's first and last layer on every axis are the unobserved ring.
        let count = std::array::from_fn(|axis| ((bounds_max[axis] - bounds_min[axis]).max(-stride) + 2 * stride) as usize / stride as usize + 1);
        let points = count[0] * count[1] * count[2];
        let words = points.div_ceil(64);
        Self {
            iso,
            bounds_min,
            bounds_max,
            stride,
            origin,
            count,
            cursor: [0; 3],
            outside: vec![0; words],
            queued: vec![0; words],
            frontier: VecDeque::new(),
            carved: false,
            cube_vertices: BTreeMap::new(),
            positions: Vec::new(),
            triangles: Vec::new(),
            complete: cells == 0,
            exceeded: false,
            observed_only: false,
        }
    }

    /// 👁️ The same extraction emitting only crossings between two OBSERVED samples: the measured
    /// surface as an open sheet, its rim wherever observation ends, for the pipeline's hole fill to
    /// cap. The carved total field still decides the manifold clustering, but it adds no faces:
    /// under a capture that never saw the object's underside, the carving floods the unobserved
    /// core through the open bottom and the total field's surface is a hat one truncation band
    /// thick — its inner wall sits a truncation inside the object and, on a coarse lattice, the
    /// thin band aliases into fragments.
    pub fn observed_only(mut self) -> Self {
        self.observed_only = true;
        self
    }

    /// 🔁 The same extraction restarted one stride coarser, or `None` once the coarsest admitted
    /// lattice has been tried.
    pub fn coarsened(&self) -> Option<Self> {
        (self.stride < MAXIMUM_SURFACE_NET_STRIDE).then(|| Self { observed_only: self.observed_only, ..Self::new_strided(self.iso, self.bounds_min, self.bounds_max, self.stride.saturating_mul(2)) })
    }

    /// 🔢️ Domain index of a lattice point, or `None` when it lies off the strided domain.
    fn domain_index(&self, point: Lattice) -> Option<usize> {
        let coordinates = [point.0, point.1, point.2];
        let mut index = 0usize;
        for axis in 0..3 {
            let offset = coordinates[axis] - self.origin[axis];
            if offset < 0 || offset % self.stride != 0 {
                return None;
            }
            let slot = (offset / self.stride) as usize;
            if slot >= self.count[axis] {
                return None;
            }
            index = index * self.count[axis] + slot;
        }
        Some(index)
    }

    fn domain_point(&self, index: usize) -> Lattice {
        let k = index % self.count[2];
        let j = (index / self.count[2]) % self.count[1];
        let i = index / (self.count[2] * self.count[1]);
        (self.origin[0] + i as i32 * self.stride, self.origin[1] + j as i32 * self.stride, self.origin[2] + k as i32 * self.stride)
    }

    fn bit(words: &[u64], index: usize) -> bool {
        words[index / 64] & (1u64 << (index % 64)) != 0
    }

    fn set_bit(words: &mut [u64], index: usize) {
        words[index / 64] |= 1u64 << (index % 64);
    }

    /// 🔭️ The integrated sample at a lattice point, if the point lies inside the requested box and
    /// was integrated. Samples beyond the box are deliberately not consulted: the box's faces are
    /// where the extraction closes, and the ring outside it must be unobserved for the flood fill's
    /// single seed to reach all of it.
    fn observed(&self, volume: &remodeling_dense::TsdfVolume, point: Lattice) -> Option<f64> {
        let coordinates = [point.0, point.1, point.2];
        let inside_box = (0..3).all(|axis| coordinates[axis] >= self.bounds_min[axis] && coordinates[axis] <= self.bounds_max[axis]);
        inside_box.then(|| volume.sample(point.0, point.1, point.2).map(|(value, _)| value)).flatten()
    }

    /// 🪓️ The carved field at a lattice point: the integrated sample where one exists, otherwise
    /// `±truncation` by whether the flood fill reached the point from the margin.
    fn field(&self, volume: &remodeling_dense::TsdfVolume, point: Lattice) -> f64 {
        if let Some(value) = self.observed(volume, point) {
            return value;
        }
        let outside = self.domain_index(point).is_none_or(|index| Self::bit(&self.outside, index));
        if outside { volume.truncation.abs() + self.iso } else { -volume.truncation.abs() + self.iso }
    }

    /// 🪓️ Expands the flood fill by at most `cell_budget` cells; `true` once the frontier is empty.
    fn carve(&mut self, volume: &remodeling_dense::TsdfVolume, cell_budget: usize) -> bool {
        if self.frontier.is_empty() && !Self::bit(&self.queued, 0) {
            // 🌱️ The whole margin ring is unobserved and 6-connected, so one seed reaches all of it.
            Self::set_bit(&mut self.queued, 0);
            self.frontier.push_back(0);
        }
        for _ in 0..cell_budget.max(1) {
            let Some(index) = self.frontier.pop_front() else { break };
            let index = index as usize;
            Self::set_bit(&mut self.outside, index);
            let point = self.domain_point(index);
            for offset in [(self.stride, 0, 0), (-self.stride, 0, 0), (0, self.stride, 0), (0, -self.stride, 0), (0, 0, self.stride), (0, 0, -self.stride)] {
                let neighbour = lattice_add(point, offset);
                let Some(neighbour_index) = self.domain_index(neighbour) else { continue };
                if Self::bit(&self.queued, neighbour_index) {
                    continue;
                }
                let passable = self.observed(volume, neighbour).is_none_or(|value| value >= self.iso);
                if !passable {
                    continue;
                }
                Self::set_bit(&mut self.queued, neighbour_index);
                self.frontier.push_back(neighbour_index as u32);
            }
        }
        self.frontier.is_empty()
    }

    fn advance_cursor(&mut self) {
        self.cursor[2] += 1;
        if self.cursor[2] >= self.count[2] {
            self.cursor[2] = 0;
            self.cursor[1] += 1;
            if self.cursor[1] >= self.count[1] {
                self.cursor[1] = 0;
                self.cursor[0] += 1;
                if self.cursor[0] >= self.count[0] {
                    self.complete = true;
                }
            }
        }
    }

    /// 📍️ The net vertices of the voxel cube at `origin`, one per crossing cluster, created on first
    /// use: each is the mean of the interpolated crossings of its cluster's edges in the carved
    /// field. Indexed by local edge; `u32::MAX` where the edge does not cross.
    fn cube_vertices(&mut self, volume: &remodeling_dense::TsdfVolume, origin: Lattice) -> [u32; 12] {
        if let Some(&table) = self.cube_vertices.get(&origin) {
            return table;
        }
        let stride = self.stride;
        let corners: [Corner; 8] = std::array::from_fn(|slot| {
            let offset = CUBE_CORNER_OFFSETS[slot];
            let lattice = lattice_add(origin, (offset.0 * stride, offset.1 * stride, offset.2 * stride));
            Corner { key: lattice, pos: [(lattice.0 as f64 + 0.5) * volume.voxel_size, (lattice.1 as f64 + 0.5) * volume.voxel_size, (lattice.2 as f64 + 0.5) * volume.voxel_size], val: self.field(volume, lattice) }
        });
        let inside: [bool; 8] = std::array::from_fn(|slot| corners[slot].val < self.iso);
        let known: [bool; 8] = std::array::from_fn(|slot| !self.observed_only || self.observed(volume, corners[slot].key).is_some());
        let crossing: [bool; 12] = std::array::from_fn(|edge| inside[CUBE_EDGES[edge].0] != inside[CUBE_EDGES[edge].1] && known[CUBE_EDGES[edge].0] && known[CUBE_EDGES[edge].1]);
        // 🧵️ Union-find over the twelve edges: consecutive crossings on a face join one cluster.
        let mut parent: [usize; 12] = std::array::from_fn(|edge| edge);
        fn find(parent: &mut [usize; 12], edge: usize) -> usize {
            let mut root = edge;
            while parent[root] != root {
                root = parent[root];
            }
            let mut walk = edge;
            while parent[walk] != root {
                let next = parent[walk];
                parent[walk] = root;
                walk = next;
            }
            root
        }
        // 🧵️ Faces with four crossings first: the corners alternate, and each inside corner's two
        // edges (edge k-1 and edge k meet at corner k) pair up, keeping the inside corners apart.
        // The two pairs of such a face are then never joined by the cube's other faces — joining
        // them would put four quads on one net edge (both cubes sharing the face merging both
        // pairs), the one way this net came out non-manifold.
        let mut separated: Vec<(usize, usize)> = Vec::new();
        for face in CUBE_FACES {
            let edges: [usize; 4] = std::array::from_fn(|k| cube_edge_index(face[k], face[(k + 1) % 4]));
            if (0..4).filter(|&k| crossing[edges[k]]).count() != 4 {
                continue;
            }
            let mut pairs: Vec<usize> = Vec::with_capacity(2);
            for k in 0..4 {
                if inside[face[k]] {
                    let (a, b) = (find(&mut parent, edges[(k + 3) % 4]), find(&mut parent, edges[k]));
                    parent[a] = b;
                    pairs.push(edges[k]);
                }
            }
            if let [first, second] = pairs[..] {
                separated.push((first, second));
            }
        }
        for face in CUBE_FACES {
            let edges: [usize; 4] = std::array::from_fn(|k| cube_edge_index(face[k], face[(k + 1) % 4]));
            let crossings: Vec<usize> = (0..4).filter(|&k| crossing[edges[k]]).collect();
            if crossings.len() != 2 {
                continue;
            }
            let (a, b) = (find(&mut parent, edges[crossings[0]]), find(&mut parent, edges[crossings[1]]));
            if a == b {
                continue;
            }
            let would_join_separated = separated.iter().any(|&(first, second)| {
                let (p, q) = (find(&mut parent, first), find(&mut parent, second));
                (p == a && q == b) || (p == b && q == a)
            });
            if !would_join_separated {
                parent[a] = b;
            }
        }
        let mut sums = [([0.0f64; 3], 0usize); 12];
        for edge in 0..12 {
            if !crossing[edge] {
                continue;
            }
            let (a, b) = (corners[CUBE_EDGES[edge].0], corners[CUBE_EDGES[edge].1]);
            let t = ((self.iso - a.val) / (b.val - a.val)).clamp(0.0, 1.0);
            let root = find(&mut parent, edge);
            sums[root].0 = add3(sums[root].0, lerp3(a.pos, b.pos, t));
            sums[root].1 += 1;
        }
        let mut table = [u32::MAX; 12];
        let mut cluster_vertex = [u32::MAX; 12];
        for edge in 0..12 {
            if !crossing[edge] {
                continue;
            }
            let root = find(&mut parent, edge);
            if cluster_vertex[root] == u32::MAX {
                cluster_vertex[root] = self.positions.len() as u32;
                self.positions.push(scale3(sums[root].0, 1.0 / sums[root].1 as f64));
            }
            table[edge] = cluster_vertex[root];
        }
        self.cube_vertices.insert(origin, table);
        table
    }

    pub fn advance(&mut self, volume: &remodeling_dense::TsdfVolume, point_budget: usize) -> bool {
        if self.complete {
            return true;
        }
        if !self.carved {
            self.carved = self.carve(volume, point_budget.max(1) * TSDF_CARVE_CELLS_PER_UNIT);
            return false;
        }
        for _ in 0..point_budget.max(1) {
            if self.complete {
                break;
            }
            let p = self.domain_point(self.cursor[0] * self.count[1] * self.count[2] + self.cursor[1] * self.count[2] + self.cursor[2]);
            let here = self.field(volume, p);
            let inside = here < self.iso;
            // (axis, first other axis, second other axis) in cyclic order, so the cube cycle
            // below winds counter-clockwise about +axis.
            for (axis, b, c) in [(0usize, 1usize, 2usize), (1, 2, 0), (2, 0, 1)] {
                let stride = self.stride;
                let unit = |along: usize| -> (i32, i32, i32) { (i32::from(along == 0) * stride, i32::from(along == 1) * stride, i32::from(along == 2) * stride) };
                let next = lattice_add(p, unit(axis));
                if self.domain_index(next).is_none() {
                    continue;
                }
                let there = self.field(volume, next);
                if inside == (there < self.iso) {
                    continue;
                }
                if self.observed_only && (self.observed(volume, p).is_none() || self.observed(volume, next).is_none()) {
                    continue;
                }
                let (eb, ec) = (unit(b), unit(c));
                let minus = |q: Lattice, d: (i32, i32, i32)| (q.0 - d.0, q.1 - d.1, q.2 - d.2);
                // Each cube with the local (0/1) offset of `p` inside it, which names the local edge.
                let cubes = [(minus(minus(p, eb), ec), (1, 1)), (minus(p, ec), (0, 1)), (p, (0, 0)), (minus(p, eb), (1, 0))];
                let mut quad = [0u32; 4];
                let mut complete_quad = true;
                for (slot, (cube, (along_b, along_c))) in cubes.into_iter().enumerate() {
                    let mut start = [0i32; 3];
                    start[b] = along_b;
                    start[c] = along_c;
                    let local = cube_edge_index_from(start, axis);
                    let vertex = self.cube_vertices(volume, cube)[local];
                    if vertex == u32::MAX {
                        complete_quad = false;
                    } else {
                        quad[slot] = vertex;
                    }
                }
                if !complete_quad {
                    continue;
                }
                // The outward normal points from the inside sample toward the outside one:
                // +axis when `p` is inside, so the counter-clockwise cycle keeps its order.
                if !inside {
                    quad.reverse();
                }
                self.triangles.push([quad[0], quad[1], quad[2]]);
                self.triangles.push([quad[0], quad[2], quad[3]]);
            }
            // 👁️ An observed-only sheet still has its rim and holes to cap: a quarter of the envelope
            // stays free for the hole fill's triangles.
            let ceiling = if self.observed_only { SURFACE_NET_ELEMENT_CEILING * 3 / 4 } else { SURFACE_NET_ELEMENT_CEILING };
            if self.positions.len() > ceiling || self.triangles.len() > ceiling {
                self.exceeded = true;
                self.complete = true;
                break;
            }
            self.advance_cursor();
        }
        self.complete
    }

    pub fn exceeded(&self) -> bool {
        self.exceeded
    }

    /// 👁️ Whether this extraction emits only the observed surface.
    pub fn is_observed_only(&self) -> bool {
        self.observed_only
    }

    pub fn finish(self) -> Option<TriMesh> {
        (self.complete && !self.exceeded).then_some(TriMesh { positions: self.positions, triangles: self.triangles })
    }
}

/// 🧊️ A total (every cell defined), contiguous dense scalar field over an axis-aligned lattice —
/// the simple non-hashed counterpart to [`remodeling_dense::TsdfVolume`] used by
/// [`extract_dense_grid`], primarily as the [`close_voxel`] fallback's occupancy field.
#[derive(Clone, Debug)]
pub struct DenseField {
    pub nx: usize,
    pub ny: usize,
    pub nz: usize,
    pub origin: [f64; 3],
    pub voxel: f64,
    pub values: Vec<f64>,
}

impl DenseField {
    pub fn new(nx: usize, ny: usize, nz: usize, origin: [f64; 3], voxel: f64, fill: f64) -> Self {
        Self { nx, ny, nz, origin, voxel, values: vec![fill; nx * ny * nz] }
    }

    pub fn index(&self, i: i32, j: i32, k: i32) -> Option<usize> {
        if i < 0 || j < 0 || k < 0 || i as usize >= self.nx || j as usize >= self.ny || k as usize >= self.nz {
            return None;
        }
        Some((k as usize * self.ny + j as usize) * self.nx + i as usize)
    }

    pub fn get(&self, i: i32, j: i32, k: i32) -> Option<f64> {
        self.index(i, j, k).map(|idx| self.values[idx])
    }

    pub fn set(&mut self, i: i32, j: i32, k: i32, value: f64) {
        if let Some(idx) = self.index(i, j, k) {
            self.values[idx] = value;
        }
    }

    pub fn world_corner(&self, i: i32, j: i32, k: i32) -> [f64; 3] {
        add3(self.origin, scale3([i as f64, j as f64, k as f64], self.voxel))
    }
}

/// 🧊️ Marching-cubes extraction over a total [`DenseField`] (every corner defined, so no
/// unknown-corner skipping is needed) — same crack-free tetrahedral kernel as [`extract_tsdf`],
/// reused verbatim rather than re-implemented, so the [`close_voxel`] fallback's correctness rests
/// on the identical, already-tested welding and orientation logic.
pub fn extract_dense_grid(field: &DenseField, iso: f64) -> TriMesh {
    let mut weld: BTreeMap<EdgeKey, u32> = BTreeMap::new();
    let mut positions: Vec<[f64; 3]> = Vec::new();
    let mut triangles: Vec<[u32; 3]> = Vec::new();
    for i in 0..(field.nx as i32 - 1) {
        for j in 0..(field.ny as i32 - 1) {
            for k in 0..(field.nz as i32 - 1) {
                let offsets: [(i32, i32, i32); 8] = [(0, 0, 0), (1, 0, 0), (1, 1, 0), (0, 1, 0), (0, 0, 1), (1, 0, 1), (1, 1, 1), (0, 1, 1)];
                let mut corners = [Corner { key: (0, 0, 0), pos: [0.0; 3], val: 0.0 }; 8];
                for (slot, off) in offsets.iter().enumerate() {
                    let c = lattice_add((i, j, k), *off);
                    let val = field.get(c.0, c.1, c.2).expect("interior cell corner is always in range");
                    corners[slot] = Corner { key: c, pos: field.world_corner(c.0, c.1, c.2), val };
                }
                for tet in cube_tets(&corners) {
                    march_tet(tet, iso, &mut weld, &mut positions, &mut triangles);
                }
            }
        }
    }
    TriMesh { positions, triangles }
}
// #endregion 🔖️MarchingCubes
