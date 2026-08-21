//! 🧩️ FEM 3D module engine — solid meshing (pure FE algorithm, moved out of the artifact tree). Meshes
//! every `FemSolid` footprint into `Tet4` elements and translates document loads (incl. `FemLoad::Area`,
//! which needs a meshed solid's top surface) into `crate::model::Model` inputs — shared by
//! `crate::fem3d_engine`'s `build_model`/`fem3d_solve_all` and `modal_buckling.rs`.

use crate::artifacts::fem3d::{Fem3dSnapshot, FemElement, FemLoad};
use crate::fem3d_engine::Fem3dError;
use crate::model::{Bar3, Dof, Element, Elements, Frame3, MemberUdl, NodalLoad, Node, Support};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint3;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::{SemioMesh, SemioMeshSnapshot, SemioPrimitive, SemioTopology};
use std::collections::HashMap;

// #region 🔖️SolidMeshing
/// 📐️ Unsigned area of triangle `(p0, p1, p2)` via the shoelace formula — mirrors `fem_2d`'s helper of
/// the same purpose.
async fn triangle_area_2d(p0: [f64; 2], p1: [f64; 2], p2: [f64; 2]) -> f64 {
    (0.5 * ((p1[0] - p0[0]) * (p2[1] - p0[1]) - (p2[0] - p0[0]) * (p1[1] - p0[1]))).abs()
}

/// 🌐️ One meshed `FemSolid`'s resolved geometry, reused by `resolve_geometry`'s caller for area-load
/// tributary-area computation. `node_ids`/`points` cover the FULL volume mesh (every layer); the top
/// surface (needed for `FemLoad::Area`) is exactly `top_footprint_tris` over `top_footprint_points`
/// (the ORIGINAL flat triangulation, since `extrude_tri_mesh` numbers each layer's points in that same
/// order — see its doc — so the top layer's node ids are `node_ids[top_offset + footprint_point_index]`).
pub struct MeshedSolid {
    pub solid_id: String,
    pub node_ids: Vec<String>,
    pub top_offset: usize,
    pub top_footprint_points: Vec<[f64; 2]>,
    pub top_footprint_tris: Vec<[u32; 3]>,
}

/// 🧩️ `resolve_geometry`'s resolved `(nodes, elements, meshed solids, supports)` quadruple.
pub type ResolvedGeometry = (Vec<Node>, Vec<Elements>, Vec<MeshedSolid>, Vec<Support>);

/// 🌉️ Resolves a `Fem3dSnapshot`'s nodes/elements/supports (materials/sections looked up by id) plus
/// every `FemSolid` meshed into `Tet4` elements (footprint triangulated via `crate::mesh::triangulate`,
/// extruded via `extrude_tri_mesh`, split via `split_to_tets` — mirrors `fem_2d::build_nodes_and_elements`'s
/// `Tri3Cst` region meshing) — the geometry shared by `build_model`, `fem3d_solve_all`, modal, and
/// buckling. A solid boundary point coinciding (within `1e-9`, all of x/y/z) with an existing document
/// node reuses that node's id; otherwise a node is synthesized once per unique mesh point as
/// `{solid_id}_m{point_index}`.
pub async fn resolve_geometry(doc: &Fem3dSnapshot) -> Result<ResolvedGeometry, Fem3dError> {
    let mut nodes: Vec<Node> = doc.nodes.iter().map(|node| Node { id: node.id.clone(), pos: [node.x, node.y, node.z] }).collect();
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

    let mut meshed_solids = Vec::with_capacity(doc.solids.len());
    for solid in &doc.solids {
        let material = doc.materials.iter().find(|m| m.id == solid.material_id).ok_or_else(|| Fem3dError::MaterialNotFound(solid.material_id.clone()))?;
        let domain = crate::mesh::PlanarDomain { outer: solid.outline.clone(), holes: solid.holes.clone() };
        let opts = crate::mesh::MeshOpts { max_edge: solid.mesh_size, min_angle_deg: 20.0 };
        let tri_mesh = crate::mesh::triangulate(&domain, &opts).map_err(|e| Fem3dError::MeshFailed { solid_id: solid.id.clone(), reason: e.to_string() })?;
        let layers = solid.layers.max(1);
        let volume_mesh = crate::mesh::extrude_tri_mesh(&tri_mesh, solid.height, layers);
        let tet_mesh = crate::mesh::split_to_tets(&volume_mesh);
        let points: Vec<[f64; 3]> = tet_mesh.points.iter().map(|p| [p[0], p[1], p[2] + solid.base_z]).collect();

        let mut node_ids = Vec::with_capacity(points.len());
        for (point_index, p) in points.iter().enumerate() {
            let id = match doc.nodes.iter().find(|n| (n.x - p[0]).abs() < 1e-9 && (n.y - p[1]).abs() < 1e-9 && (n.z - p[2]).abs() < 1e-9) {
                Some(n) => n.id.clone(),
                None => {
                    let synthetic_id = format!("{}_m{}", solid.id, point_index);
                    nodes.push(Node { id: synthetic_id.clone(), pos: *p });
                    synthetic_id
                }
            };
            node_ids.push(id);
        }

        for (cell_index, cell) in tet_mesh.cells.iter().enumerate() {
            let crate::mesh::Cell::Tet4(t) = cell else { continue };
            let tet_nodes = [node_ids[t[0] as usize].clone(), node_ids[t[1] as usize].clone(), node_ids[t[2] as usize].clone(), node_ids[t[3] as usize].clone()];
            elements.push(crate::elements3d::Tet4 { id: format!("{}_c{}", solid.id, cell_index), nodes: tet_nodes, e: material.e, nu: material.nu, density: material.rho }.into());
        }

        // The LAST extrusion layer's points are the top surface — `extrude_tri_mesh` numbers points
        // layer-by-layer in `tri_mesh.points`' own order (see its doc), so index `top_offset + i` is
        // footprint point `i`'s top-layer node, and `tri_mesh.tris` is directly the top surface's
        // triangulation (that layer's wedges pass their top cap through `split_to_tets` unsplit).
        let top_offset = layers * tri_mesh.points.len();
        meshed_solids.push(MeshedSolid { solid_id: solid.id.clone(), node_ids, top_offset, top_footprint_points: tri_mesh.points, top_footprint_tris: tri_mesh.tris });
    }

    let supports = doc.supports.iter().map(|support| Support { node_id: support.node_id.clone(), fixed: support.fixed.iter().map(|dof| Dof::from(*dof)).collect() }).collect();
    Ok((nodes, elements, meshed_solids, supports))
}

/// 🌬️ Converts a `FemLoad::Area` (uniform pressure, Pa) into per-node global `-Z` nodal loads on a
/// solid's TOP surface — `pressure * tributaryArea` at each top node, tributary area `(1/3)` of the
/// summed area of every top-surface triangle touching that node. Mirrors `fem_2d::area_load_nodal_loads`.
async fn area_load_nodal_loads_3d(solid: &MeshedSolid, pressure: f64) -> Vec<NodalLoad> {
    let mut tributary: HashMap<String, f64> = HashMap::new();
    for tri in &solid.top_footprint_tris {
        let area = triangle_area_2d(solid.top_footprint_points[tri[0] as usize], solid.top_footprint_points[tri[1] as usize], solid.top_footprint_points[tri[2] as usize]);
        for &idx in tri {
            let node_id = solid.node_ids[solid.top_offset + idx as usize].clone();
            *tributary.entry(node_id).or_insert(0.0) += area / 3.0;
        }
    }
    tributary.into_iter().map(|(node_id, trib)| NodalLoad { node_id, dof: Dof::Tz, value: -pressure * trib }).collect()
}

//#region 🔖️SemioMeshBridge
/// 🌉️ Builds a real `SemioMeshSnapshot` from every `FemSolid`'s genuinely triangulated,
/// height-extruded boundary: `crate::mesh::triangulate` the footprint (same call
/// `resolve_geometry` above already makes before its own `extrude_tri_mesh`/`split_to_tets`/
/// `Tet4` pipeline), extruded by the solid's OWN `height` (offset by `base_z`), then
/// `boundary_faces` for the outward-oriented outer surface — real, tested geometry (`crate::mesh`'s
/// own volume/area-preservation tests), not fabricated bytes. Feeds the `s.stdio.obj`/`s.stdio.stl`
/// export leaves, which hand this to stdio's real `SemioMeshToObj`/`SemioMeshToStl` bridge +
/// `encode_obj`/`encode_stl_ascii` grammar — this function does no byte-level encoding itself.
///
/// `FemElement::Bar`/`Frame` line members carry no real cross-section PROFILE in the persisted
/// data (only `area`/`iy`/`iz`/`j`, scalar section properties) so no honest 3D solid can be
/// derived from them — they contribute no geometry here. A pure bar/frame model (no `solids`)
/// yields an empty, still-structurally-valid mesh (and thus an empty .obj/.stl) rather than a
/// fabricated shape.
pub(crate) async fn build_semio_mesh_snapshot(doc: &Fem3dSnapshot) -> SemioMeshSnapshot {
    let mut meshes = Vec::with_capacity(doc.solids.len());
    for solid in &doc.solids {
        let domain = crate::mesh::PlanarDomain { outer: solid.outline.clone(), holes: solid.holes.clone() };
        let opts = crate::mesh::MeshOpts { max_edge: solid.mesh_size, min_angle_deg: 20.0 };
        let Ok(tri_mesh) = crate::mesh::triangulate(&domain, &opts) else { continue };
        let volume = crate::mesh::extrude_tri_mesh(&tri_mesh, solid.height, 1);
        let tets = crate::mesh::split_to_tets(&volume);
        let faces = crate::mesh::boundary_faces(&tets);
        let positions: Vec<SemioPoint3> = tets.points.iter().map(|p| SemioPoint3 { x: p[0], y: p[1], z: p[2] + solid.base_z }).collect();
        let indices: Vec<u32> = faces.iter().flat_map(|f| f.iter().copied()).collect();
        meshes.push(SemioMesh { id: solid.id.clone(), primitives: vec![SemioPrimitive { id: format!("{}-surface", solid.id), topology: SemioTopology::Triangles, positions, indices, ..Default::default() }] });
    }
    SemioMeshSnapshot { meshes, ..Default::default() }
}
//#endregion 🔖️SemioMeshBridge

/// 🌬️ Translates one `FemLoadCase`'s loads into `(nodal_loads, member_loads)`, resolving `Area` loads
/// against the already-meshed `solids` — shared by `build_model`, `fem3d_solve_all`, and buckling's
/// reference-case resolution.
pub async fn translate_loads(loads: &[FemLoad], solids: &[MeshedSolid]) -> Result<(Vec<NodalLoad>, Vec<(String, MemberUdl)>), Fem3dError> {
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
