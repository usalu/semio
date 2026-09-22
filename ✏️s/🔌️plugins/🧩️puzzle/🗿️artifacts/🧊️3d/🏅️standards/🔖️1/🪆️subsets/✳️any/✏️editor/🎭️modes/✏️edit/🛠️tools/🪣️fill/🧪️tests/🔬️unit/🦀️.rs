use super::*;

#[test]
fn fill_all_objects_preparation_requires_real_geometry() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../⏳️precompute/🪣️fill/🧫️fixtures/🎞️fill-run.json")).expect("fill law");
    for case in fixture["laws"]["allObjects"]["preparedGeometry"].as_array().expect("mesh cases") {
        let url = case["url"].as_str().expect("url");
        if case["registered"].as_bool().expect("registered") {
            let (positions, indices) = puzzle3d_fallback_mesh_buffers();
            assert!(crate::editor::puzzle3d::precompute::derive_brush_mesh(url, &positions, &indices).is_some());
        }
        let identity = ToolRunIdentity::new(semio_framework_tool_run::ToolRunId { app_instance_id: 113, run: 1 }, [0; 32]);
        let scene = SceneConfig {
            fixture: Default::default(), kind_catalogs: None, kind_compatibility: Vec::new(), contact_tolerance: 0.0,
            seed: 1, host_rules: Default::default(), weights: Default::default(),
        };
        let mut job = Puzzle3dFillToolRunJob::new(identity, scene, vec![url.into()], FillToolRunTarget::Run { requested: 1, checkpoint: None, provisional: 0 }, FillRunInputs::of(&Puzzle3dConfig::default()));
        let FillToolRunPhase::Preparing(preparation) = &mut job.phase else { panic!("preparation") };
        assert!(!Puzzle3dFillToolRunJob::prepare_one(preparation));
        assert!(Puzzle3dFillToolRunJob::prepare_one(preparation));
        eprintln!("[DEBUG] all-objects mesh {url}: available={}", preparation.meshes.contains_key(url));
        assert_eq!(preparation.meshes.contains_key(url), case["expectedAvailable"].as_bool().expect("available"));
    }
}
