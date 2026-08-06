//! 🧊️ Dense 3D grid topology: the exact 2D design of [`crate::wfc::grid2d`] extended to a third axis
//! (same arithmetic-neighbor, zero-adjacency-storage approach, same [`crate::wfc::grid2d::Boundary`]
//! per-axis semantics — reused directly rather than duplicated). `NodeId(z * width * height + y * width + x)`.

use crate::wfc::error::{ModelError, TopologyError};
use crate::wfc::grid2d::{resolve_coord, Boundary};
use crate::wfc::ids::{NodeId, PatternId, RegionId, RelationId};
use crate::wfc::model::ModelBuilder;
use crate::wfc::tiled::TiledModelBuilder;
use crate::wfc::topology::Topology;

// #region 🔖️Stencil
/// 🧊️ Which offsets count as "neighbors" of a 3D cell.
#[derive(Clone, PartialEq, Debug)]
pub enum Stencil3d {
    /// 🧊️ 6-neighbor: the six face-adjacent cells.
    Face6,
    /// 🧊️ 18-neighbor: face- and edge-adjacent cells (Manhattan distance 1 or 2, excluding corners).
    Edge18,
    /// 🧊️ 26-neighbor: every cell in the surrounding 3×3×3 block.
    Vertex26,
    /// 🧊️ An arbitrary offset list, each entry's negation required to also be present.
    Custom(Vec<(i32, i32, i32)>),
}

impl Stencil3d {
    pub fn offsets(&self) -> Vec<(i32, i32, i32)> {
        match self {
            Stencil3d::Face6 => vec![(1, 0, 0), (-1, 0, 0), (0, 1, 0), (0, -1, 0), (0, 0, 1), (0, 0, -1)],
            Stencil3d::Edge18 => vertex26_offsets().into_iter().filter(|&(x, y, z)| x.abs() + y.abs() + z.abs() <= 2).collect(),
            Stencil3d::Vertex26 => vertex26_offsets(),
            Stencil3d::Custom(v) => v.clone(),
        }
    }

    fn validate(&self) -> Result<(), TopologyError> {
        let offsets = self.offsets();
        if offsets.is_empty() {
            return Err(TopologyError::InvalidStencil { reason: "stencil has zero offsets" });
        }
        for (i, &a) in offsets.iter().enumerate() {
            if a == (0, 0, 0) {
                return Err(TopologyError::InvalidStencil { reason: "self-offset (0,0,0) is not supported" });
            }
            for &b in &offsets[i + 1..] {
                if a == b {
                    return Err(TopologyError::InvalidStencil { reason: "duplicate offset" });
                }
            }
            if !offsets.contains(&(-a.0, -a.1, -a.2)) {
                return Err(TopologyError::InvalidStencil { reason: "offset's negation is not present in the stencil" });
            }
        }
        Ok(())
    }
}

fn vertex26_offsets() -> Vec<(i32, i32, i32)> {
    let mut v = Vec::with_capacity(26);
    for dx in -1..=1 {
        for dy in -1..=1 {
            for dz in -1..=1 {
                if (dx, dy, dz) != (0, 0, 0) {
                    v.push((dx, dy, dz));
                }
            }
        }
    }
    v
}

/// 🧊️ Registers one directed relation per stencil offset (paired with its negation as inverse) and
/// returns them in `stencil.offsets()` order, ready to pass to [`Grid3dTopology::new`].
pub fn declare_stencil_relations_3d(builder: &mut ModelBuilder, stencil: &Stencil3d) -> Result<Vec<RelationId>, ModelError> {
    stencil.validate().map_err(|_| ModelError::InvalidSymmetryGroup { reason: "invalid stencil passed to declare_stencil_relations_3d" })?;
    let offsets = stencil.offsets();
    let mut relations = Vec::with_capacity(offsets.len());
    for &(dx, dy, dz) in &offsets {
        relations.push(builder.add_relation(&format!("offset({dx},{dy},{dz})")));
    }
    for (i, &(dx, dy, dz)) in offsets.iter().enumerate() {
        if let Some(j) = offsets.iter().position(|&o| o == (-dx, -dy, -dz)) {
            builder.set_relation_inverse(relations[i], relations[j]);
        }
    }
    Ok(relations)
}

/// 🧊️ [`declare_stencil_relations_3d`] for a [`TiledModelBuilder`].
pub fn declare_stencil_relations_3d_tiled(builder: &mut TiledModelBuilder, stencil: &Stencil3d) -> Result<Vec<RelationId>, ModelError> {
    stencil.validate().map_err(|_| ModelError::InvalidSymmetryGroup { reason: "invalid stencil passed to declare_stencil_relations_3d_tiled" })?;
    let offsets = stencil.offsets();
    let mut relations = Vec::with_capacity(offsets.len());
    for &(dx, dy, dz) in &offsets {
        relations.push(builder.relation(&format!("offset({dx},{dy},{dz})")));
    }
    for (i, &(dx, dy, dz)) in offsets.iter().enumerate() {
        if let Some(j) = offsets.iter().position(|&o| o == (-dx, -dy, -dz)) {
            builder.set_relation_inverse(relations[i], relations[j]);
        }
    }
    Ok(relations)
}
// #endregion 🔖️Stencil

// #region 🔖️Topology
/// 🧊️ A dense, z-major-then-row-major 3D grid topology. `NodeId(z*width*height + y*width + x)`.
#[derive(Clone, Debug)]
pub struct Grid3dTopology {
    width: usize,
    height: usize,
    depth: usize,
    offsets: Vec<(i32, i32, i32)>,
    relations: Vec<RelationId>,
    boundary_x: Boundary,
    boundary_y: Boundary,
    boundary_z: Boundary,
    mask: Option<Vec<bool>>,
}

impl Grid3dTopology {
    #[allow(clippy::too_many_arguments)]
    pub fn new(width: usize, height: usize, depth: usize, stencil: &Stencil3d, relations: Vec<RelationId>, boundary_x: Boundary, boundary_y: Boundary, boundary_z: Boundary, mask: Option<Vec<bool>>) -> Result<Self, TopologyError> {
        if width == 0 {
            return Err(TopologyError::ZeroDimension { axis: "width" });
        }
        if height == 0 {
            return Err(TopologyError::ZeroDimension { axis: "height" });
        }
        if depth == 0 {
            return Err(TopologyError::ZeroDimension { axis: "depth" });
        }
        width.checked_mul(height).and_then(|wh| wh.checked_mul(depth)).ok_or(TopologyError::SizeOverflow)?;
        stencil.validate()?;
        let offsets = stencil.offsets();
        if offsets.len() != relations.len() {
            return Err(TopologyError::InvalidStencil { reason: "relations length does not match stencil offset count" });
        }
        if let Some(m) = &mask {
            if m.len() != width * height * depth {
                return Err(TopologyError::MaskShapeMismatch { expected: width * height * depth, actual: m.len() });
            }
        }
        Ok(Self { width, height, depth, offsets, relations, boundary_x, boundary_y, boundary_z, mask })
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }
    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }
    #[inline]
    pub fn depth(&self) -> usize {
        self.depth
    }

    #[inline]
    fn index(&self, x: usize, y: usize, z: usize) -> usize {
        z * self.width * self.height + y * self.width + x
    }

    #[inline]
    pub fn node_at(&self, x: usize, y: usize, z: usize) -> Option<NodeId> {
        if x >= self.width || y >= self.height || z >= self.depth {
            return None;
        }
        Some(NodeId::from_index(self.index(x, y, z)))
    }

    #[inline]
    pub fn coords(&self, n: NodeId) -> (usize, usize, usize) {
        let idx = n.index();
        let plane = self.width * self.height;
        let z = idx / plane;
        let rem = idx % plane;
        (rem % self.width, rem / self.width, z)
    }

    #[inline]
    pub fn is_active(&self, x: usize, y: usize, z: usize) -> bool {
        self.mask.as_ref().is_none_or(|m| m[self.index(x, y, z)])
    }

    pub fn inactive_cells(&self) -> Vec<NodeId> {
        let Some(mask) = &self.mask else { return Vec::new() };
        (0..mask.len()).filter(|&i| !mask[i]).map(NodeId::from_index).collect()
    }

    /// 🧊️ Every `(node, relation, outside_pattern)` an edge cell must be restricted by at init
    /// time, derived from [`Boundary::FixedOutside`] axes.
    pub fn fixed_outside_restrictions(&self) -> Vec<(NodeId, RelationId, PatternId)> {
        let mut out = Vec::new();
        for z in 0..self.depth {
            for y in 0..self.height {
                for x in 0..self.width {
                    if !self.is_active(x, y, z) {
                        continue;
                    }
                    for (i, &(dx, dy, dz)) in self.offsets.iter().enumerate() {
                        let tx = x as i32 + dx;
                        let ty = y as i32 + dy;
                        let tz = z as i32 + dz;
                        let x_out = tx < 0 || tx as usize >= self.width;
                        let y_out = ty < 0 || ty as usize >= self.height;
                        let z_out = tz < 0 || tz as usize >= self.depth;
                        let node = NodeId::from_index(self.index(x, y, z));
                        if x_out {
                            if let Boundary::FixedOutside(p) = self.boundary_x {
                                out.push((node, self.relations[i], p));
                                continue;
                            }
                        }
                        if y_out {
                            if let Boundary::FixedOutside(p) = self.boundary_y {
                                out.push((node, self.relations[i], p));
                                continue;
                            }
                        }
                        if z_out {
                            if let Boundary::FixedOutside(p) = self.boundary_z {
                                out.push((node, self.relations[i], p));
                            }
                        }
                    }
                }
            }
        }
        out
    }
}

impl Topology for Grid3dTopology {
    #[inline]
    fn node_count(&self) -> usize {
        self.width * self.height * self.depth
    }

    fn arc_count(&self) -> usize {
        let mut count = 0;
        for z in 0..self.depth {
            for y in 0..self.height {
                for x in 0..self.width {
                    if !self.is_active(x, y, z) {
                        continue;
                    }
                    for &(dx, dy, dz) in &self.offsets {
                        let rx = resolve_coord(x as i32 + dx, self.width, self.boundary_x);
                        let ry = resolve_coord(y as i32 + dy, self.height, self.boundary_y);
                        let rz = resolve_coord(z as i32 + dz, self.depth, self.boundary_z);
                        if let (Some(nx), Some(ny), Some(nz)) = (rx, ry, rz) {
                            if self.is_active(nx, ny, nz) {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
        count
    }

    #[inline]
    fn region_of(&self, _n: NodeId) -> RegionId {
        RegionId(0)
    }

    fn for_each_out_arc(&self, n: NodeId, mut f: impl FnMut(NodeId, RelationId)) {
        let (x, y, z) = self.coords(n);
        if !self.is_active(x, y, z) {
            return;
        }
        for (i, &(dx, dy, dz)) in self.offsets.iter().enumerate() {
            let rx = resolve_coord(x as i32 + dx, self.width, self.boundary_x);
            let ry = resolve_coord(y as i32 + dy, self.height, self.boundary_y);
            let rz = resolve_coord(z as i32 + dz, self.depth, self.boundary_z);
            if let (Some(nx), Some(ny), Some(nz)) = (rx, ry, rz) {
                if self.is_active(nx, ny, nz) {
                    f(NodeId::from_index(self.index(nx, ny, nz)), self.relations[i]);
                }
            }
        }
    }

    fn for_each_in_arc(&self, n: NodeId, mut f: impl FnMut(NodeId, RelationId, usize)) {
        let (x, y, z) = self.coords(n);
        if !self.is_active(x, y, z) {
            return;
        }
        for (i, &(dx, dy, dz)) in self.offsets.iter().enumerate() {
            let rx = resolve_coord(x as i32 - dx, self.width, self.boundary_x);
            let ry = resolve_coord(y as i32 - dy, self.height, self.boundary_y);
            let rz = resolve_coord(z as i32 - dz, self.depth, self.boundary_z);
            if let (Some(sx), Some(sy), Some(sz)) = (rx, ry, rz) {
                if self.is_active(sx, sy, sz) {
                    f(NodeId::from_index(self.index(sx, sy, sz)), self.relations[i], n.index() * self.offsets.len() + i);
                }
            }
        }
    }

    fn max_in_degree(&self) -> usize {
        self.offsets.len()
    }
}
// #endregion 🔖️Topology

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn face6_edge18_vertex26_offset_counts() {
        assert_eq!(Stencil3d::Face6.offsets().len(), 6);
        assert_eq!(Stencil3d::Edge18.offsets().len(), 18);
        assert_eq!(Stencil3d::Vertex26.offsets().len(), 26);
    }

    #[test]
    fn all_built_in_stencils_validate() {
        Stencil3d::Face6.validate().unwrap();
        Stencil3d::Edge18.validate().unwrap();
        Stencil3d::Vertex26.validate().unwrap();
    }

    #[test]
    fn node_at_and_coords_roundtrip() {
        let mut b = ModelBuilder::new();
        b.add_pattern(1.0);
        let rels = declare_stencil_relations_3d(&mut b, &Stencil3d::Face6).unwrap();
        let topo = Grid3dTopology::new(3, 4, 5, &Stencil3d::Face6, rels, Boundary::Open, Boundary::Open, Boundary::Open, None).unwrap();
        let n = topo.node_at(1, 2, 3).unwrap();
        assert_eq!(topo.coords(n), (1, 2, 3));
        assert_eq!(topo.node_at(3, 0, 0), None);
    }

    #[test]
    fn open_boundary_corner_has_three_neighbors() {
        let mut b = ModelBuilder::new();
        b.add_pattern(1.0);
        let rels = declare_stencil_relations_3d(&mut b, &Stencil3d::Face6).unwrap();
        let topo = Grid3dTopology::new(3, 3, 3, &Stencil3d::Face6, rels, Boundary::Open, Boundary::Open, Boundary::Open, None).unwrap();
        let corner = topo.node_at(0, 0, 0).unwrap();
        let mut out = Vec::new();
        topo.for_each_out_arc(corner, |m, _| out.push(m));
        assert_eq!(out.len(), 3);
    }

    #[test]
    fn wrap_boundary_connects_all_axes() {
        let mut b = ModelBuilder::new();
        b.add_pattern(1.0);
        let rels = declare_stencil_relations_3d(&mut b, &Stencil3d::Face6).unwrap();
        let topo = Grid3dTopology::new(3, 3, 3, &Stencil3d::Face6, rels, Boundary::Wrap, Boundary::Wrap, Boundary::Wrap, None).unwrap();
        let corner = topo.node_at(0, 0, 0).unwrap();
        let mut out = Vec::new();
        topo.for_each_out_arc(corner, |m, _| out.push(m));
        assert_eq!(out.len(), 6);
    }

    #[test]
    fn mask_excludes_inactive_voxels() {
        let mut b = ModelBuilder::new();
        b.add_pattern(1.0);
        let rels = declare_stencil_relations_3d(&mut b, &Stencil3d::Face6).unwrap();
        let mut mask = vec![true; 27];
        mask[13] = false; // center of 3x3x3
        let topo = Grid3dTopology::new(3, 3, 3, &Stencil3d::Face6, rels, Boundary::Open, Boundary::Open, Boundary::Open, Some(mask)).unwrap();
        let neighbor = topo.node_at(1, 1, 0).unwrap(); // directly below center
        let mut out = Vec::new();
        topo.for_each_out_arc(neighbor, |m, _| out.push(m));
        assert!(!out.contains(&topo.node_at(1, 1, 1).unwrap()));
        assert_eq!(topo.inactive_cells(), vec![NodeId(13)]);
    }

    #[test]
    fn in_arc_matches_out_arc_on_open_boundary() {
        let mut b = ModelBuilder::new();
        b.add_pattern(1.0);
        let rels = declare_stencil_relations_3d(&mut b, &Stencil3d::Vertex26).unwrap();
        let topo = Grid3dTopology::new(3, 3, 3, &Stencil3d::Vertex26, rels, Boundary::Open, Boundary::Open, Boundary::Open, None).unwrap();
        for z in 0..3 {
            for y in 0..3 {
                for x in 0..3 {
                    let n = topo.node_at(x, y, z).unwrap();
                    let mut outgoing = Vec::new();
                    topo.for_each_out_arc(n, |m, r| outgoing.push((m, r)));
                    let mut reconstructed = Vec::new();
                    for oz in 0..3 {
                        for oy in 0..3 {
                            for ox in 0..3 {
                                let other = topo.node_at(ox, oy, oz).unwrap();
                                let mut theirs = Vec::new();
                                topo.for_each_in_arc(other, |src, r, _slot| theirs.push((src, r)));
                                for &(s, r) in &theirs {
                                    if s == n {
                                        reconstructed.push((other, r));
                                    }
                                }
                            }
                        }
                    }
                    outgoing.sort_by_key(|&(m, r)| (m.get(), r.get()));
                    reconstructed.sort_by_key(|&(m, r)| (m.get(), r.get()));
                    assert_eq!(outgoing, reconstructed, "voxel ({x},{y},{z}) out-arcs must match in-arc reconstruction");
                }
            }
        }
    }
}
// #endregion 🔖️Tests
