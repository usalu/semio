//! 🧩️ FEM 3D module engine — solid meshing (pure FE algorithm, moved out of the artifact tree). Meshes
//! every `FemSolid` footprint into `Tet4` elements — extruded along the solid's own `FemAxis`, with the
//! nodes of every solid merged into ONE node set so touching solids share their interface nodes — and
//! translates document loads (incl. `FemLoad::Area`, a pressure on a solid's upward-facing surface)
//! into `crate::model::Model` inputs. Shared by `crate::fem3d_engine`'s `build_model`/`fem3d_solve_all`,
//! `modal_buckling.rs`, the mesh preview and the stdio mesh export, so every consumer sees the same
//! node ids and the same geometry.

/// 🌬️ Translated nodal loads and member-addressed distributed loads.
pub type TranslatedLoads = (Vec<NodalLoad>, Vec<(String, MemberUdl)>);

use crate::fem3d_engine::Fem3dError;
use crate::model::{Bar3, Dof, Elements, Frame3, MemberUdl, NodalLoad, Node, Support};
use crate::{Fem3dSnapshot, FemElement, FemLoad, FemSolid};
use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::SemioPoint3;
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::schema::snapshot::{SemioMesh, SemioMeshSnapshot, SemioPrimitive, SemioTopology};
use std::collections::HashMap;

// #region 🔖️NodeMerge
/// 📏️ Two mesh points closer than this (in every coordinate, metres) are ONE node — the seam every
/// interface between touching solids, and every document node a solid corner lands on, closes over.
pub const NODE_MERGE_TOLERANCE: f64 = 1e-6;

/// 🧲️ The document-wide node registry the solids are meshed into: a document node keeps its own id, a
/// mesh point that lands on a registered node reuses that id, and every other point is registered once
/// as `{solid_id}_m{point_index}` — so two solids meshed to the same interface share their nodes there.
struct NodeMerger {
    nodes: Vec<Node>,
    cells: HashMap<[i64; 3], Vec<usize>>,
}

impl NodeMerger {
    fn new(doc: &Fem3dSnapshot) -> Self {
        let mut merger = Self { nodes: Vec::with_capacity(doc.nodes.len()), cells: HashMap::new() };
        for node in &doc.nodes {
            merger.register(node.id.clone(), [node.x, node.y, node.z]);
        }
        merger
    }

    fn cell(position: [f64; 3]) -> [i64; 3] {
        [(position[0] / NODE_MERGE_TOLERANCE).round() as i64, (position[1] / NODE_MERGE_TOLERANCE).round() as i64, (position[2] / NODE_MERGE_TOLERANCE).round() as i64]
    }

    /// 🔍️ The registered node within tolerance of `position`, searching the 27 quantised cells around it.
    fn find(&self, position: [f64; 3]) -> Option<usize> {
        let cell = Self::cell(position);
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    let Some(candidates) = self.cells.get(&[cell[0] + dx, cell[1] + dy, cell[2] + dz]) else { continue };
                    for &index in candidates {
                        let node = &self.nodes[index].pos;
                        if (node[0] - position[0]).abs() < NODE_MERGE_TOLERANCE && (node[1] - position[1]).abs() < NODE_MERGE_TOLERANCE && (node[2] - position[2]).abs() < NODE_MERGE_TOLERANCE {
                            return Some(index);
                        }
                    }
                }
            }
        }
        None
    }

    fn register(&mut self, id: String, position: [f64; 3]) -> usize {
        let index = self.nodes.len();
        self.nodes.push(Node { id, pos: position });
        self.cells.entry(Self::cell(position)).or_default().push(index);
        index
    }

    /// 🧲️ The id of the node at `position`, registering a synthetic one when nothing sits there yet.
    fn resolve(&mut self, position: [f64; 3], synthetic_id: impl FnOnce() -> String) -> String {
        match self.find(position) {
            Some(index) => self.nodes[index].id.clone(),
            None => {
                let index = self.register(synthetic_id(), position);
                self.nodes[index].id.clone()
            }
        }
    }
}
// #endregion 🔖️NodeMerge

// #region 🔖️SolidMeshing
/// 📐️ Unsigned area of triangle `(p0, p1, p2)` via the shoelace formula — mirrors `fem_2d`'s helper of
/// the same purpose.
fn triangle_area_2d(p0: [f64; 2], p1: [f64; 2], p2: [f64; 2]) -> f64 {
    (0.5 * ((p1[0] - p0[0]) * (p2[1] - p0[1]) - (p2[0] - p0[0]) * (p1[1] - p0[1]))).abs()
}

/// 📐️ A world-space triangle's area vector (`cross(e0, e1) / 2`): its length is the area, its direction
/// the face normal, so the `+Z` component is the area projected onto the horizontal plane.
fn triangle_area_vector(a: [f64; 3], b: [f64; 3], c: [f64; 3]) -> [f64; 3] {
    let e0 = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    let e1 = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
    [(e0[1] * e1[2] - e0[2] * e1[1]) * 0.5, (e0[2] * e1[0] - e0[0] * e1[2]) * 0.5, (e0[0] * e1[1] - e0[1] * e1[0]) * 0.5]
}

/// 🗺️ One meshed `FemSolid`'s complete geometry in WORLD coordinates, with document-wide node ids:
/// the volume points and tets (tet indices into `points`, so `tets[i]` is element `"{solid_id}_c{i}"`),
/// the outward-oriented boundary triangulation (via `crate::mesh::boundary_faces`) for surface
/// rendering and surface loads, and the footprint triangulation the volume was extruded from.
pub struct SolidMesh {
    pub solid_id: String,
    pub points: Vec<[f64; 3]>,
    pub tets: Vec<[u32; 4]>,
    pub boundary_tris: Vec<[u32; 3]>,
    pub node_ids: Vec<String>,
    pub footprint: crate::mesh::TriMesh2,
}

/// 🌐️ A footprint triangulation extruded along `solid.axis` from `solid.base_z` — every point lifted
/// through `FemAxis::to_world`, so the same wedge/tet pipeline serves a floor slab, a wall drawn in
/// elevation and a roof section extruded along its ridge.
fn extrude_solid(solid: &FemSolid, footprint: &crate::mesh::TriMesh2) -> (Vec<[f64; 3]>, Vec<[u32; 4]>, Vec<[u32; 3]>) {
    let volume = crate::mesh::extrude_tri_mesh(footprint, solid.height, solid.layers.max(1));
    let tet_mesh = crate::mesh::split_to_tets(&volume);
    let points: Vec<[f64; 3]> = tet_mesh.points.iter().map(|p| solid.axis.to_world(p[0], p[1], p[2] + solid.base_z)).collect();
    let tets: Vec<[u32; 4]> = tet_mesh
        .cells
        .iter()
        .filter_map(|cell| match cell {
            crate::mesh::Cell::Tet4(tet) => Some(*tet),
            _ => None,
        })
        .collect();
    let boundary = crate::mesh::boundary_faces(&crate::mesh::VolumeMesh { points: points.clone(), cells: tet_mesh.cells });
    (points, tets, boundary)
}

fn triangulate_solid(solid: &FemSolid) -> Result<crate::mesh::TriMesh2, Fem3dError> {
    let domain = crate::mesh::PlanarDomain { outer: solid.outline.clone(), holes: solid.holes.clone() };
    let opts = crate::mesh::MeshOpts { max_edge: solid.mesh_size, min_angle_deg: 20.0 };
    crate::mesh::triangulate(&domain, &opts).map_err(|e| Fem3dError::MeshFailed { solid_id: solid.id.clone(), reason: e.to_string() })
}

/// 🧩️ Meshes every `FemSolid` of `doc` into ONE merged node set: the document's own nodes first (each
/// keeps its id), then solid by solid in document order, a mesh point within [`NODE_MERGE_TOLERANCE`]
/// of an already registered node reusing that node's id — which is what turns a raft, the walls on it
/// and the slab on the walls into one connected structure. Returns the merged nodes and every solid's
/// mesh; `SolidMesh::node_ids[i]` is the document-wide id of `SolidMesh::points[i]`.
pub fn mesh_solids(doc: &Fem3dSnapshot) -> Result<(Vec<Node>, Vec<SolidMesh>), Fem3dError> {
    let mut merger = NodeMerger::new(doc);
    let mut meshes = Vec::with_capacity(doc.solids.len());
    for solid in &doc.solids {
        let footprint = triangulate_solid(solid)?;
        let (points, tets, boundary_tris) = extrude_solid(solid, &footprint);
        let node_ids = points.iter().enumerate().map(|(point_index, point)| merger.resolve(*point, || format!("{}_m{point_index}", solid.id))).collect();
        meshes.push(SolidMesh { solid_id: solid.id.clone(), points, tets, boundary_tris, node_ids, footprint });
    }
    Ok((merger.nodes, meshes))
}

/// 🧩️ `resolve_geometry`'s resolved `(nodes, elements, meshed solids, supports)` quadruple.
pub type ResolvedGeometry = (Vec<Node>, Vec<Elements>, Vec<SolidMesh>, Vec<Support>);

/// 🌉️ Resolves a `Fem3dSnapshot`'s nodes/elements/supports (materials/sections looked up by id) plus
/// every `FemSolid` meshed into `Tet4` elements through [`mesh_solids`] — the geometry shared by
/// `build_model`, `fem3d_solve_all`, modal, and buckling.
pub fn resolve_geometry(doc: &Fem3dSnapshot) -> Result<ResolvedGeometry, Fem3dError> {
    let node_exists = |id: &str| doc.nodes.iter().any(|n| n.id == id);
    let mut elements: Vec<Elements> = Vec::with_capacity(doc.elements.len());
    for element in &doc.elements {
        match element {
            FemElement::Bar { id, start, end, material_id, section_id } => {
                let material = doc.materials.iter().find(|m| &m.id == material_id).ok_or_else(|| Fem3dError::MaterialNotFound(material_id.clone()))?;
                let section = doc.sections.iter().find(|s| &s.id == section_id).ok_or_else(|| Fem3dError::SectionNotFound(section_id.clone()))?;
                if !node_exists(start) {
                    return Err(Fem3dError::NodeNotFound(start.clone()));
                }
                if !node_exists(end) {
                    return Err(Fem3dError::NodeNotFound(end.clone()));
                }
                elements.push(Bar3 { id: id.clone(), node_a: start.clone(), node_b: end.clone(), e: material.e, a: section.area, density: material.rho }.into());
            }
            FemElement::Frame { id, start, end, material_id, section_id, roll } => {
                let material = doc.materials.iter().find(|m| &m.id == material_id).ok_or_else(|| Fem3dError::MaterialNotFound(material_id.clone()))?;
                let section = doc.sections.iter().find(|s| &s.id == section_id).ok_or_else(|| Fem3dError::SectionNotFound(section_id.clone()))?;
                if !node_exists(start) {
                    return Err(Fem3dError::NodeNotFound(start.clone()));
                }
                if !node_exists(end) {
                    return Err(Fem3dError::NodeNotFound(end.clone()));
                }
                elements.push(Frame3 { id: id.clone(), node_a: start.clone(), node_b: end.clone(), e: material.e, g: material.g, a: section.area, iy: section.iy, iz: section.iz, j: section.j, roll: *roll, density: material.rho }.into());
            }
        }
    }

    for solid in &doc.solids {
        if !doc.materials.iter().any(|m| m.id == solid.material_id) {
            return Err(Fem3dError::MaterialNotFound(solid.material_id.clone()));
        }
    }
    let (nodes, meshes) = mesh_solids(doc)?;
    for (solid, mesh) in doc.solids.iter().zip(&meshes) {
        let material = doc.materials.iter().find(|m| m.id == solid.material_id).ok_or_else(|| Fem3dError::MaterialNotFound(solid.material_id.clone()))?;
        for (cell_index, tet) in mesh.tets.iter().enumerate() {
            let tet_nodes = [mesh.node_ids[tet[0] as usize].clone(), mesh.node_ids[tet[1] as usize].clone(), mesh.node_ids[tet[2] as usize].clone(), mesh.node_ids[tet[3] as usize].clone()];
            elements.push(crate::elements3d::Tet4 { id: format!("{}_c{cell_index}", solid.id), nodes: tet_nodes, e: material.e, nu: material.nu, density: material.rho }.into());
        }
    }

    let supports = doc.supports.iter().map(|support| Support { node_id: support.node_id.clone(), fixed: support.fixed.iter().map(|dof| Dof::from(*dof)).collect() }).collect();
    Ok((nodes, elements, meshes, supports))
}

/// 🌬️ Converts a `FemLoad::Area` (uniform pressure, Pa) into per-node global `-Z` nodal loads over a
/// solid's UPWARD-FACING surface: every boundary triangle whose outward normal has a positive `+Z`
/// component carries `pressure × its horizontally projected area`, spread a third to each corner —
/// the snow/floor-load convention (a load per plan area), which makes a sloped roof and a flat slab
/// take the same load per footprint square metre. On a `Z`-extruded solid the upward surface is
/// exactly its top footprint, so the tributary areas equal the footprint triangle thirds.
fn area_load_nodal_loads_3d(solid: &SolidMesh, pressure: f64) -> Vec<NodalLoad> {
    let mut tributary: HashMap<String, f64> = HashMap::new();
    for tri in &solid.boundary_tris {
        let area = triangle_area_vector(solid.points[tri[0] as usize], solid.points[tri[1] as usize], solid.points[tri[2] as usize]);
        if area[2] <= 0.0 {
            continue;
        }
        for &idx in tri {
            *tributary.entry(solid.node_ids[idx as usize].clone()).or_insert(0.0) += area[2] / 3.0;
        }
    }
    let mut loads: Vec<(String, f64)> = tributary.into_iter().collect();
    loads.sort_by(|a, b| a.0.cmp(&b.0));
    loads.into_iter().map(|(node_id, trib)| NodalLoad { node_id, dof: Dof::Tz, value: -pressure * trib }).collect()
}

/// 📐️ The footprint area of one meshed solid's triangulation, in the footprint plane.
pub fn footprint_area(mesh: &SolidMesh) -> f64 {
    mesh.footprint.tris.iter().map(|tri| triangle_area_2d(mesh.footprint.points[tri[0] as usize], mesh.footprint.points[tri[1] as usize], mesh.footprint.points[tri[2] as usize])).sum()
}

//#region 🔖️SemioMeshBridge
/// 🌉️ Builds a real `SemioMeshSnapshot` from every `FemSolid`'s genuinely triangulated, extruded
/// boundary — the same [`mesh_solids`] pipeline the solver runs (the solid's own `axis`, `base_z`,
/// `height` and `layers`), reduced to `boundary_faces` for the outward-oriented outer surface — real,
/// tested geometry (`crate::mesh`'s own volume/area-preservation tests), not fabricated bytes. Feeds
/// the `s.stdio.obj`/`s.stdio.stl` export leaves, which hand this to stdio's real
/// `SemioMeshToObj`/`SemioMeshToStl` bridge + `encode_obj`/`encode_stl_ascii` grammar — this function
/// does no byte-level encoding itself.
///
/// `FemElement::Bar`/`Frame` line members carry no real cross-section PROFILE in the persisted
/// data (only `area`/`iy`/`iz`/`j`, scalar section properties) so no honest 3D solid can be
/// derived from them — they contribute no geometry here. A pure bar/frame model (no `solids`)
/// yields an empty, still-structurally-valid mesh (and thus an empty .obj/.stl) rather than a
/// fabricated shape. A solid that fails to mesh is skipped, so one degenerate outline never voids
/// the export of its neighbours.
pub(crate) fn build_semio_mesh_snapshot(doc: &Fem3dSnapshot) -> SemioMeshSnapshot {
    let mut meshes = Vec::with_capacity(doc.solids.len());
    for solid in &doc.solids {
        let Ok(footprint) = triangulate_solid(solid) else { continue };
        let single_layer = FemSolid { layers: 1, ..solid.clone() };
        let (points, _, faces) = extrude_solid(&single_layer, &footprint);
        let positions: Vec<SemioPoint3> = points.iter().map(|p| SemioPoint3 { x: p[0], y: p[1], z: p[2] }).collect();
        let indices: Vec<u32> = faces.iter().flat_map(|f| f.iter().copied()).collect();
        meshes.push(SemioMesh { id: solid.id.clone(), primitives: vec![SemioPrimitive { id: format!("{}-surface", solid.id), topology: SemioTopology::Triangles, positions, indices, ..Default::default() }] });
    }
    SemioMeshSnapshot { meshes, ..Default::default() }
}
//#endregion 🔖️SemioMeshBridge

/// 🌬️ Translates one `FemLoadCase`'s loads into `(nodal_loads, member_loads)`, resolving `Area` loads
/// against the already-meshed `solids` — shared by `build_model`, `fem3d_solve_all`, and buckling's
/// reference-case resolution.
pub fn translate_loads(loads: &[FemLoad], solids: &[SolidMesh]) -> Result<TranslatedLoads, Fem3dError> {
    let mut nodal_loads = Vec::new();
    let mut member_loads = Vec::new();
    for load in loads {
        match load {
            FemLoad::Nodal { node_id, dof, value, .. } => nodal_loads.push(NodalLoad { node_id: node_id.clone(), dof: Dof::from(*dof), value: *value }),
            FemLoad::MemberUdl { element_id, wx, wy, wz, .. } => member_loads.push((element_id.clone(), MemberUdl { wx: *wx, wy: *wy, wz: *wz })),
            FemLoad::Area { solid_id, pressure, .. } => {
                let solid = solids.iter().find(|s| &s.solid_id == solid_id).ok_or_else(|| Fem3dError::UnknownSolidId(solid_id.clone()))?;
                nodal_loads.extend(area_load_nodal_loads_3d(solid, *pressure));
            }
        }
    }
    Ok((nodal_loads, member_loads))
}
// #endregion 🔖️SolidMeshing
