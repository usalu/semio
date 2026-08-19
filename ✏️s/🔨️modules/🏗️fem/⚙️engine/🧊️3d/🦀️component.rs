//! ⚙️ FEM 3D module engine — headless compute (pure FE algorithm, moved out of the artifact tree: an
//! artifact is a schema + io system, never an engine). Errors and the top-level `build_model`/
//! `fem3d_solve`/`fem3d_solve_all` entry points live here. Solid meshing lives in `🕸️meshing`,
//! modal/buckling in `🎵️modal-buckling`, mesh preview + nodal stress in `🗺️mesh-preview`.
//! `empty_fem3d_snapshot` lives in the artifact's own `🧬️schema/🦀️component.rs` (pure document
//! helper); `fem3d_io`/its ports and the scene-render bridge (`fem3d_scene_parts`/`fem3d_camera_json`
//! and their helpers — app-facing, reference `crate::app_surface` and shared by the model/results
//! windows) live in `🎛️apps/🧊️3d/🦀️component.rs`.

use crate::artifacts::fem3d::Fem3dSnapshot;
use crate::analyses;
use crate::fem3d_engine::meshing;
use std::collections::HashMap;

// #region 🔖️Errors
/// ⚠️ Everything that can go wrong resolving or solving a `Fem3dSnapshot`.
#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum Fem3dError {
    #[error("material not found: {0}")]
    MaterialNotFound(String),
    #[error("section not found: {0}")]
    SectionNotFound(String),
    #[error("node not found: {0}")]
    NodeNotFound(String),
    #[error("unknown solid id: {0}")]
    UnknownSolidId(String),
    #[error("solid {solid_id} failed to mesh: {reason}")]
    MeshFailed { solid_id: String, reason: String },
    #[error("load case not found: {0}")]
    LoadCaseNotFound(String),
    #[error("mode index out of range: {0}")]
    ModeIndexOutOfRange(usize),
    #[error(transparent)]
    Fem(#[from] crate::model::FemError),
}
// #endregion 🔖️Errors

// #region 🔖️Bridge
/// 🌉️ Resolves a `Fem3dSnapshot` load case into a `crate::model::Model`: nodes, `Bar3`/`Frame3`/`Tet4`
/// elements (materials/sections looked up by id), supports, and the named load case's translated loads.
pub async fn build_model(doc: &Fem3dSnapshot, case_id: &str) -> Result<crate::model::Model, Fem3dError> {
    let (nodes, elements, solids, supports) = meshing::resolve_geometry(doc)?;
    let case = doc.load_cases.iter().find(|c| c.id == case_id).ok_or_else(|| Fem3dError::LoadCaseNotFound(case_id.to_string()))?;
    let (nodal_loads, member_loads) = meshing::translate_loads(&case.loads, &solids)?;
    Ok(crate::model::Model { nodes, elements, supports, nodal_loads, member_loads })
}

/// 🚀️ Frozen entry point: builds the model for `case_id` and runs `crate::model::solve_linear_static`.
/// Consumed directly by `fem-plugin`; do not rename or change this signature.
pub async fn fem3d_solve(doc: &Fem3dSnapshot, case_id: &str) -> Result<crate::model::StaticResult, String> {
    let model = build_model(doc, case_id).map_err(|e| e.to_string())?;
    crate::model::solve_linear_static(&model).map_err(|e| e.to_string())
}

/// 🌉️ Builds an `AnalysisModel` plus one `analyses::LoadCase` per `doc.load_cases` entry and one
/// `analyses::Combination` per `doc.combinations` entry, solving them ALL at once via
/// `crate::analyses::solve_multi_case` (self-weight honored via `doc.materials`' `rho`, gravity
/// fixed at `[0.0, 0.0, -9.81]` — this crate is Z-up, per `FemNode`'s `{x,y,z}` fields and the existing
/// cantilever test's `Dof::Tz` tip load). Returns results keyed by case id ∪ combination id.
pub async fn fem3d_solve_all(doc: &Fem3dSnapshot) -> Result<HashMap<String, crate::model::StaticResult>, Fem3dError> {
    let (nodes, elements, solids, supports) = meshing::resolve_geometry(doc)?;
    let model = analyses::AnalysisModel { nodes, elements, supports };
    let mut cases = Vec::with_capacity(doc.load_cases.len());
    for case in &doc.load_cases {
        let (nodal_loads, member_loads) = meshing::translate_loads(&case.loads, &solids)?;
        cases.push(analyses::LoadCase { id: case.id.clone(), nodal_loads, member_loads, self_weight: case.self_weight });
    }
    let combinations: Vec<analyses::Combination> = doc.combinations.iter().map(|combination| analyses::Combination { id: combination.id.clone(), terms: combination.terms.iter().map(|(id, factor)| (id.clone(), *factor)).collect() }).collect();
    analyses::solve_multi_case(&model, &cases, &combinations, [0.0, 0.0, -9.81]).map_err(Fem3dError::from)
}
// #endregion 🔖️Bridge

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::fem3d_engine::modal_buckling;
    use crate::artifacts::fem3d::{FemAnalysisSettings, FemCombination, FemDof, FemElement, FemLoadCase, FemMaterial, FemNode, FemSection, FemSolid, FemSupport};
    use crate::model::{Dof, ElementResult};
    use std::collections::BTreeMap;

    // #region 🔖️Fixtures
    async fn cantilever_fixture() -> (Fem3dSnapshot, f64, f64, f64, f64, f64) {
        let e = 210e9;
        let g = 80.77e9;
        let a = 0.00538;
        let iy = 0.0000369;
        let iz = 0.0000133;
        let j = 0.00000060;
        let l = 3.0;
        let p = 5000.0;
        let doc = Fem3dSnapshot {
            nodes: vec![FemNode { id: "n1".into(), x: 0.0, y: 0.0, z: 0.0 }, FemNode { id: "n2".into(), x: l, y: 0.0, z: 0.0 }],
            elements: vec![FemElement::Frame { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "hea200".into(), roll: 0.0 }],
            materials: vec![FemMaterial { id: "steel".into(), name: "Steel".into(), e, g, nu: 0.3, rho: 7850.0 }],
            sections: vec![FemSection { id: "hea200".into(), name: "HEA200".into(), area: a, iy, iz, j }],
            solids: vec![],
            supports: vec![FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: FemDof::ALL.to_vec() }],
            load_cases: vec![FemLoadCase { id: "point".into(), name: "Point Load".into(), loads: vec![crate::artifacts::fem3d::FemLoad::Nodal { id: "l1".into(), node_id: "n2".into(), dof: FemDof::Tz, value: -p }], self_weight: false }],
            combinations: vec![],
            analysis: FemAnalysisSettings::default(),
        };
        (doc, e, iy, l, p, iz)
    }

    /// 🔺️ A free 3D joint needs at least 3 non-coplanar bars to be kinematically determinate — two
    /// bars only span a plane, leaving one direction with zero stiffness (a mechanism). Hence n4/b3.
    async fn truss_fixture() -> Fem3dSnapshot {
        Fem3dSnapshot {
            nodes: vec![FemNode { id: "n1".into(), x: 0.0, y: 0.0, z: 0.0 }, FemNode { id: "n2".into(), x: 2.0, y: 0.0, z: 0.0 }, FemNode { id: "n3".into(), x: 1.0, y: 1.0, z: 2.0 }, FemNode { id: "n4".into(), x: 1.0, y: -1.0, z: 0.0 }],
            elements: vec![
                FemElement::Bar { id: "b1".into(), start: "n1".into(), end: "n3".into(), material_id: "steel".into(), section_id: "rod".into() },
                FemElement::Bar { id: "b2".into(), start: "n2".into(), end: "n3".into(), material_id: "steel".into(), section_id: "rod".into() },
                FemElement::Bar { id: "b3".into(), start: "n4".into(), end: "n3".into(), material_id: "steel".into(), section_id: "rod".into() },
            ],
            materials: vec![FemMaterial { id: "steel".into(), name: "Steel".into(), e: 210e9, g: 80.77e9, nu: 0.3, rho: 7850.0 }],
            sections: vec![FemSection { id: "rod".into(), name: "Rod".into(), area: 0.001, iy: 1e-6, iz: 1e-6, j: 1e-6 }],
            solids: vec![],
            supports: vec![
                FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: FemDof::ALL.to_vec() },
                FemSupport { id: "s2".into(), node_id: "n2".into(), fixed: FemDof::ALL.to_vec() },
                FemSupport { id: "s3".into(), node_id: "n4".into(), fixed: FemDof::ALL.to_vec() },
            ],
            load_cases: vec![FemLoadCase { id: "drop".into(), name: "Drop".into(), loads: vec![crate::artifacts::fem3d::FemLoad::Nodal { id: "l1".into(), node_id: "n3".into(), dof: FemDof::Tz, value: -1000.0 }], self_weight: false }],
            combinations: vec![],
            analysis: FemAnalysisSettings::default(),
        }
    }

    /// 🧱️ A 2m x 1m x 0.5m slab footprint at the origin, meshed at `mesh_size`, with all 4 footprint
    /// corners as pre-placed document nodes fully fixed in translation (`Tet4` has no rotational DOF) —
    /// mirrors `fem_2d`'s `rectangle_region_doc` fixture pattern for `FemSolid`.
    async fn solid_slab_doc() -> Fem3dSnapshot {
        Fem3dSnapshot {
            nodes: vec![FemNode { id: "sc0".into(), x: 0.0, y: 0.0, z: 0.0 }, FemNode { id: "sc1".into(), x: 2.0, y: 0.0, z: 0.0 }, FemNode { id: "sc2".into(), x: 2.0, y: 1.0, z: 0.0 }, FemNode { id: "sc3".into(), x: 0.0, y: 1.0, z: 0.0 }],
            elements: vec![],
            materials: vec![FemMaterial { id: "concrete".into(), name: "Concrete".into(), e: 30e9, g: 12.5e9, nu: 0.2, rho: 2400.0 }],
            sections: vec![],
            solids: vec![FemSolid { id: "sol1".into(), name: "Slab".into(), outline: vec![[0.0, 0.0], [2.0, 0.0], [2.0, 1.0], [0.0, 1.0]], holes: vec![], base_z: 0.0, height: 0.5, layers: 1, mesh_size: 1.0, material_id: "concrete".into() }],
            supports: vec![
                FemSupport { id: "s1".into(), node_id: "sc0".into(), fixed: vec![FemDof::Tx, FemDof::Ty, FemDof::Tz] },
                FemSupport { id: "s2".into(), node_id: "sc1".into(), fixed: vec![FemDof::Tx, FemDof::Ty, FemDof::Tz] },
                FemSupport { id: "s3".into(), node_id: "sc2".into(), fixed: vec![FemDof::Tx, FemDof::Ty, FemDof::Tz] },
                FemSupport { id: "s4".into(), node_id: "sc3".into(), fixed: vec![FemDof::Tx, FemDof::Ty, FemDof::Tz] },
            ],
            load_cases: vec![FemLoadCase { id: "self".into(), name: "Self Weight".into(), loads: vec![], self_weight: true }],
            combinations: vec![],
            analysis: FemAnalysisSettings::default(),
        }
    }
    // #endregion 🔖️Fixtures

    // #region 🔖️BuildModel
    #[test]
    async fn build_model_rejects_dangling_material() {
        let (mut doc, ..) = cantilever_fixture();
        if let FemElement::Frame { material_id, .. } = &mut doc.elements[0] {
            *material_id = "missing".into();
        }
        let err = build_model(&doc, "point").unwrap_err();
        assert!(err.to_string().contains("missing"), "error should name the dangling id: {err}");
    }

    #[test]
    async fn build_model_rejects_dangling_section() {
        let (mut doc, ..) = cantilever_fixture();
        if let FemElement::Frame { section_id, .. } = &mut doc.elements[0] {
            *section_id = "missing".into();
        }
        let err = build_model(&doc, "point").unwrap_err();
        assert!(err.to_string().contains("missing"), "error should name the dangling id: {err}");
    }

    #[test]
    async fn build_model_rejects_dangling_node() {
        let (mut doc, ..) = cantilever_fixture();
        if let FemElement::Frame { end, .. } = &mut doc.elements[0] {
            *end = "missing".into();
        }
        let err = build_model(&doc, "point").unwrap_err();
        assert!(err.to_string().contains("missing"), "error should name the dangling id: {err}");
    }
    // #endregion 🔖️BuildModel

    // #region 🔖️CantileverBenchmark
    #[test]
    async fn cantilever_tip_load_matches_analytical_solution() {
        let (doc, e, iy, l, p, _iz) = cantilever_fixture();
        let result = fem3d_solve(&doc, "point").expect("solves");

        let expected_deflection = p * l.powi(3) / (3.0 * e * iy);
        let expected_rotation = p * l.powi(2) / (2.0 * e * iy);
        let expected_base_moment = p * l;

        let n2 = result.displacements.iter().find(|d| d.node_id == "n2").unwrap();
        let deflection = n2.values[Dof::Tz.index()].abs();
        let rotation = n2.values[Dof::Ry.index()].abs();
        assert!((deflection - expected_deflection).abs() / expected_deflection < 0.01, "deflection {deflection} vs {expected_deflection}");
        assert!((rotation - expected_rotation).abs() / expected_rotation < 0.01, "rotation {rotation} vs {expected_rotation}");

        let reaction_tz = result.reactions.iter().find(|r| r.node_id == "n1" && r.dof == Dof::Tz).unwrap();
        assert!((reaction_tz.value - p).abs() < p * 0.01 || (reaction_tz.value + p).abs() < p * 0.01, "reaction {}", reaction_tz.value);
        assert!((reaction_tz.value + (-p)).abs() < p * 0.01, "reaction + applied load should be ~0: {}", reaction_tz.value);

        let reaction_ry = result.reactions.iter().find(|r| r.node_id == "n1" && r.dof == Dof::Ry).unwrap();
        assert!((reaction_ry.value.abs() - expected_base_moment).abs() / expected_base_moment < 0.01, "base moment {} vs {}", reaction_ry.value, expected_base_moment);

        let (_, element_result) = result.elements.iter().find(|(id, _)| id == "e1").unwrap();
        match element_result {
            ElementResult::Beam { stations } => {
                let base = stations.first().unwrap();
                let tip = stations.last().unwrap();
                let base_tol = (expected_base_moment * 0.01).max(1.0);
                assert!((base.m.abs() - expected_base_moment).abs() < base_tol, "base moment {} vs {}", base.m, expected_base_moment);
                assert!(tip.m.abs() < base_tol, "tip moment should be ~0: {}", tip.m);
            }
            _ => panic!("expected beam result"),
        }
    }

    #[test]
    async fn truss_3d_solve_is_finite_and_balanced() {
        let doc = truss_fixture();
        let result = fem3d_solve(&doc, "drop").expect("solves");
        for &v in &result.checks.reaction_sum {
            assert!(v.abs() < 1e-6, "reaction_sum should balance the applied load: {:?}", result.checks.reaction_sum);
        }
        for (_, element_result) in &result.elements {
            match element_result {
                ElementResult::Bar { n } => {
                    assert!(n.is_finite());
                    assert!(n.abs() > 1e-6, "bar force should be nonzero under load");
                }
                _ => panic!("expected bar result"),
            }
        }
    }

    #[test]
    async fn fem3d_solve_unknown_case_id_errors() {
        let (doc, ..) = cantilever_fixture();
        let err = fem3d_solve(&doc, "missing-case").unwrap_err();
        assert!(err.contains("load case not found"), "error was: {err}");
    }
    // #endregion 🔖️CantileverBenchmark

    // #region 🔖️SolveAll
    #[test]
    async fn fem3d_solve_all_returns_case_and_combination_results() {
        let (mut doc, ..) = cantilever_fixture();
        doc.load_cases.push(FemLoadCase { id: "point2".into(), name: "Point Load 2".into(), loads: vec![crate::artifacts::fem3d::FemLoad::Nodal { id: "l2".into(), node_id: "n2".into(), dof: FemDof::Tz, value: -2000.0 }], self_weight: false });
        doc.combinations = vec![FemCombination { id: "uls".into(), name: "ULS".into(), terms: BTreeMap::from([("point".into(), 1.35), ("point2".into(), 1.0)]) }];

        let results = fem3d_solve_all(&doc).expect("solves");
        let mut keys: Vec<&String> = results.keys().collect();
        keys.sort();
        assert_eq!(keys, vec!["point", "point2", "uls"], "expected exactly the case and combination ids");

        let point = results.get("point").unwrap();
        let point2 = results.get("point2").unwrap();
        let uls = results.get("uls").unwrap();
        for pd in &point.displacements {
            let p2d = point2.displacements.iter().find(|d| d.node_id == pd.node_id).unwrap();
            let ud = uls.displacements.iter().find(|d| d.node_id == pd.node_id).unwrap();
            for k in 0..6 {
                let expected = 1.35 * pd.values[k] + 1.0 * p2d.values[k];
                assert!((ud.values[k] - expected).abs() < 1e-8, "combo displacement mismatch at {} dof {k}: {} vs {}", pd.node_id, ud.values[k], expected);
            }
        }
    }

    #[test]
    async fn self_weight_case_produces_nonzero_reactions() {
        let (mut doc, _e, _iy, l, _p, _iz) = cantilever_fixture();
        let (area, rho) = (doc.sections[0].area, doc.materials[0].rho);
        doc.load_cases = vec![FemLoadCase { id: "self".into(), name: "Self Weight".into(), loads: vec![], self_weight: true }];

        let results = fem3d_solve_all(&doc).expect("solves");
        let result = results.get("self").unwrap();

        let total_tz_reaction: f64 = result.reactions.iter().filter(|r| r.dof == Dof::Tz).map(|r| r.value).sum();
        let expected = rho * area * l * 9.81;
        assert!(total_tz_reaction.abs() > 1e-6, "self-weight reaction should be nonzero");
        assert!((total_tz_reaction - expected).abs() / expected < 0.02, "reaction sum {total_tz_reaction} vs expected {expected}");
    }

    /// 🌬️ A `FemLoad::MemberUdl` on the cantilever fixture's `Frame3`: base shear must equal the
    /// classical `wL` total, same benchmark `elements3d::tests::frame3_udl_cantilever_matches_hand_calc`
    /// checks headlessly, now exercised through the document bridge's load translation.
    #[test]
    async fn member_udl_load_matches_total_wl() {
        let (mut doc, _e, _iy, l, _p, _iz) = cantilever_fixture();
        let w = 800.0;
        doc.load_cases = vec![FemLoadCase { id: "udl".into(), name: "UDL".into(), loads: vec![crate::artifacts::fem3d::FemLoad::MemberUdl { id: "u1".into(), element_id: "e1".into(), wx: 0.0, wy: 0.0, wz: -w }], self_weight: false }];
        let results = fem3d_solve_all(&doc).expect("solves");
        let result = results.get("udl").unwrap();
        let total_tz_reaction: f64 = result.reactions.iter().filter(|r| r.dof == Dof::Tz).map(|r| r.value).sum();
        let expected = w * l;
        assert!((total_tz_reaction - expected).abs() / expected < 1e-6, "reaction sum {total_tz_reaction} vs expected {expected}");
    }
    // #endregion 🔖️SolveAll

    // #region 🔖️Solids
    #[test]
    async fn solid_self_weight_matches_total_mass_times_gravity() {
        let doc = solid_slab_doc();
        let results = fem3d_solve_all(&doc).expect("solid self-weight solves");
        let result = results.get("self").unwrap();
        let total_tz_reaction: f64 = result.reactions.iter().filter(|r| r.dof == Dof::Tz).map(|r| r.value).sum();
        let (footprint_area, height, rho, g) = (2.0 * 1.0, 0.5, 2400.0, 9.81);
        let expected = rho * footprint_area * height * g;
        assert!((total_tz_reaction - expected).abs() / expected < 1e-6, "reaction sum {total_tz_reaction} vs expected {expected}");
    }

    /// ⚖️ A uniform pressure over the solid's top face must balance EXACTLY (mesh-independent, since
    /// tributary-area nodal loads sum to `pressure * footprintArea` regardless of triangulation) —
    /// possible only now that `fem_3d` meshes solids at all.
    #[test]
    async fn solid_area_load_matches_pressure_times_footprint_area() {
        let mut doc = solid_slab_doc();
        doc.load_cases = vec![FemLoadCase { id: "pressure".into(), name: "Pressure".into(), loads: vec![crate::artifacts::fem3d::FemLoad::Area { id: "a1".into(), solid_id: "sol1".into(), pressure: 8000.0 }], self_weight: false }];
        let results = fem3d_solve_all(&doc).expect("solid pressure load solves");
        let result = results.get("pressure").unwrap();
        let total_tz_reaction: f64 = result.reactions.iter().filter(|r| r.dof == Dof::Tz).map(|r| r.value).sum();
        let expected = 8000.0 * 2.0 * 1.0;
        assert!((total_tz_reaction - expected).abs() / expected < 1e-6, "reaction sum {total_tz_reaction} vs expected {expected}");
    }
    // #endregion 🔖️Solids

    // #region 🔖️ExampleFixture
    /// 🧾️ Cross-cutting within the module engine: solves + meshes + nodal-stresses + buckles the
    /// bundled default example fixture in one pass — kept here (rather than split per sub-module) since
    /// it exercises `build_model`/`fem3d_solve_all` (this file), `fem3d_mesh_preview`/
    /// `fem3d_nodal_von_mises` (`mesh_preview.rs`) and `fem3d_buckling` (`modal_buckling.rs`) together.
    #[test]
    async fn example_fixture_parses() {
        let doc: Fem3dSnapshot = crate::artifacts::fem3d::dsl::parse_dsl(crate::artifacts::fem3d::dsl::FEM3D_EXAMPLE_TEXT).expect("example fixture parses");
        assert_eq!(doc.nodes.len(), 16);
        assert_eq!(doc.elements.len(), 16);
        assert_eq!(doc.solids.len(), 1);
        let result = fem3d_solve(&doc, "dead").expect("example fixture solves");
        assert!(result.checks.residual_norm < 1e-6);

        let all_results = fem3d_solve_all(&doc).expect("example fixture solves all");
        assert!(all_results.contains_key("dead"), "expected dead case result");
        assert!(all_results.contains_key("live"), "expected live case result");
        assert!(all_results.contains_key("uls"), "expected uls combination result");
        let dead = all_results.get("dead").expect("expected dead case result (solid area load + member UDL + self-weight)");
        assert!(dead.checks.residual_norm < 1e-6, "residual {}", dead.checks.residual_norm);

        let previews = crate::fem3d_engine::mesh_preview::fem3d_mesh_preview(&doc).expect("mesh preview succeeds");
        assert_eq!(previews.len(), 1);
        assert!(!previews[0].tets.is_empty(), "expected at least one tet");
        assert!(!previews[0].boundary_tris.is_empty(), "expected boundary triangles");

        let averaged = crate::fem3d_engine::mesh_preview::fem3d_nodal_von_mises(&doc, "dead").expect("nodal von mises solves");
        assert!(!averaged.is_empty(), "expected at least one averaged nodal value");
        for v in averaged.values() {
            assert!(v.is_finite() && *v >= 0.0, "von mises {v} should be finite and non-negative");
        }

        let buckling = modal_buckling::fem3d_buckling(&doc, "dead").expect("buckling resolves for the dead case's compressed column");
        assert!(buckling.factors[0].is_finite() && buckling.factors[0] > 1.0, "expected an illustrative (finite, >1) load factor: {:?}", buckling.factors);
    }
    // #endregion 🔖️ExampleFixture
}
// #endregion 🧪️Tests
