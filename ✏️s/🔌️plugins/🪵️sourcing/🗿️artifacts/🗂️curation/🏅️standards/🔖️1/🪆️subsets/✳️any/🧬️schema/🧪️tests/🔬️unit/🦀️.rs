
use super::*;

fn sample_document() -> CurationSnapshot {
    crate::curation_snapshot_from_stock(&demo_stock(), Vec::new())
}

#[semio_framework_async_macros::async_test]
async fn filtered_stock_matches_query() {
    let document = sample_document();
    let filters = Filters { query: "glulam".into(), ..Default::default() };
    let filtered = filtered_stock(&document, &filters);
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].id, "beam-glulam-gl24h");
}

#[semio_framework_async_macros::async_test]
async fn filtered_stock_matches_module() {
    let document = sample_document();
    let filters = Filters { module_ids: vec!["slabs".into()], ..Default::default() };
    let filtered = filtered_stock(&document, &filters);
    assert!(filtered.iter().all(|kind| kind.module_id == "slabs"));
    assert_eq!(filtered.len(), 3);
}

#[semio_framework_async_macros::async_test]
async fn filtered_stock_matches_typology_prefix() {
    let document = sample_document();
    let filters = Filters { typology_path: vec!["beams".into(), "steel".into()], ..Default::default() };
    let filtered = filtered_stock(&document, &filters);
    assert_eq!(filtered.len(), 2);
    assert!(filtered.iter().all(|kind| kind.typology_path.starts_with(&["beams".to_string(), "steel".to_string()])));
}

#[semio_framework_async_macros::async_test]
async fn filtered_stock_matches_min_availability() {
    let document = sample_document();
    let filters = Filters { min_availability: 20, ..Default::default() };
    let filtered = filtered_stock(&document, &filters);
    assert!(filtered.iter().all(|kind| kind.availability >= 20));
    assert!(!filtered.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn curation_delta_clamps_to_availability_and_zero_floor() {
    let mut document = sample_document();
    curation_delta(&mut document, "beam-steel-hea160", 100);
    assert_eq!(curated_count(&document, "beam-steel-hea160"), 8);
    curation_delta(&mut document, "beam-steel-hea160", -1000);
    assert_eq!(curated_count(&document, "beam-steel-hea160"), 0);
    assert!(document.curated.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn curation_delta_unknown_object_is_noop() {
    let mut document = sample_document();
    curation_delta(&mut document, "does-not-exist", 5);
    assert!(document.curated.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn curation_set_removes_entry_at_zero() {
    let mut document = sample_document();
    curation_set(&mut document, "slab-clt-160", 5);
    assert_eq!(curated_count(&document, "slab-clt-160"), 5);
    curation_set(&mut document, "slab-clt-160", 0);
    assert_eq!(curated_count(&document, "slab-clt-160"), 0);
    assert!(document.curated.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn typology_contains_and_flatten() {
    let module = beams::BeamsModule;
    let tree = module.typology();
    assert!(typology_contains(&tree, &["beams".into(), "steel".into(), "ipe".into()]));
    assert!(!typology_contains(&tree, &["beams".into(), "concrete".into()]));
    let flattened = typology_flatten(&tree);
    assert!(flattened.iter().any(|(path, _)| path == &vec!["beams".to_string(), "solid-timber".to_string(), "glulam".to_string()]));
}

fn assert_mesh_spec_is_valid(spec: &MeshDataSpec) {
    assert!(!spec.positions.is_empty());
    assert_eq!(spec.positions.len() % 3, 0);
    assert_eq!(spec.positions.len(), spec.normals.len());
    assert_eq!(spec.indices.len() % 3, 0);
    let vertex_count = (spec.positions.len() / 3) as u32;
    assert!(spec.indices.iter().all(|&i| i < vertex_count));
}

#[semio_framework_async_macros::async_test]
async fn box_recipe_produces_valid_mesh() {
    assert_mesh_spec_is_valid(&mesh_spec_for(&GeometryRecipe::Box { width: 0.2, height: 0.4, depth: 6.0 }));
}

#[semio_framework_async_macros::async_test]
async fn frame_recipe_concatenates_four_pieces_into_a_valid_mesh() {
    let spec = mesh_spec_for(&GeometryRecipe::Frame { width: 1.0, height: 1.2, depth: 0.08, profile: 0.08 });
    assert_mesh_spec_is_valid(&spec);
    let single_box = box_mesh_spec(1.0, 0.08, 0.08);
    assert_eq!(spec.positions.len(), single_box.positions.len() * 4);
    assert_eq!(spec.indices.len(), single_box.indices.len() * 4);
}

#[semio_framework_async_macros::async_test]
async fn grid_placement_centers_around_origin() {
    let positions: Vec<(f64, f64)> = (0..9).map(|i| grid_placement(9, i, 2.0)).collect();
    let sum_x: f64 = positions.iter().map(|(x, _)| x).sum();
    let sum_z: f64 = positions.iter().map(|(_, z)| z).sum();
    assert!(sum_x.abs() < 1e-9);
    assert!(sum_z.abs() < 1e-9);
    let unique: std::collections::HashSet<(i64, i64)> = positions.iter().map(|(x, z)| ((x * 1000.0) as i64, (z * 1000.0) as i64)).collect();
    assert_eq!(unique.len(), 9);
}

#[semio_framework_async_macros::async_test]
async fn grid_scale_normalizes_to_cell_size() {
    let recipe = GeometryRecipe::Box { width: 0.2, height: 0.4, depth: 6.0 };
    let scale = grid_scale(&recipe, 2.0);
    assert!((bounding_extent(&recipe) * scale - 2.0).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn curation_document_dsl_round_trips_sample_and_empty() {
    store::os_store::test_support::assert_dsl_round_trip(&sample_document());
    store::os_store::test_support::assert_dsl_round_trip(&CurationSnapshot::default());
    store::os_store::test_support::assert_dsl_pack_equivalence(&sample_document());
    store::os_store::test_support::assert_dsl_pack_equivalence(&CurationSnapshot::default());
}

#[semio_framework_async_macros::async_test]
async fn available_modules_tracks_contributed_modules() {
    assert_eq!(available_modules("[]").len(), 3);
    let beams = beams::BeamsModule;
    let entry = semio_framework::ProgramContributionEntry {
        plugin_id: "sourcing-module-beams".into(),
        topic_contribution: Some(semio_framework::TopicContribution::new(
            "sourcing.module",
            semio_framework::DslValue::object([
                ("appId".to_string(), semio_framework::DslValue::String(SOURCING_CURATION_APP_ID.to_string())),
                ("moduleId".to_string(), semio_framework::DslValue::String(beams.module_id().to_string())),
                ("label".to_string(), semio_framework::DslValue::String(beams.label().to_string())),
                ("iconId".to_string(), semio_framework::DslValue::String("beam".to_string())),
                ("typologyJson".to_string(), semio_framework::DslValue::String(semio_framework_os_kernel::json::to_json_string(&beams.typology()))),
                ("kindsJson".to_string(), semio_framework::DslValue::String(semio_framework_os_kernel::json::to_json_string(&beams.demo_kinds()))),
            ]),
        )),
    };
    let contributions_json = dsl::json::to_json_string(&vec![entry]);
    let modules = available_modules(&contributions_json);
    assert_eq!(modules.len(), 4);
    assert_eq!(modules[0].module_id, "beams");
}

#[semio_framework_async_macros::async_test]
async fn sourcing_module_contributions_are_configuration_owned() {
    use semio_framework::{ProgramContributionEntry, TopicContribution};
    let entry = ProgramContributionEntry {
        plugin_id: "sourcing-module-test".into(),
        topic_contribution: Some(TopicContribution::new(
            "sourcing.module",
            semio_framework::DslValue::object([
                ("appId".to_string(), semio_framework::DslValue::String("sourcing-curation".to_string())),
                ("moduleId".to_string(), semio_framework::DslValue::String("hot-test".to_string())),
                ("label".to_string(), semio_framework::DslValue::String("Hot Test".to_string())),
                ("iconId".to_string(), semio_framework::DslValue::String("box".to_string())),
                ("typologyJson".to_string(), semio_framework::DslValue::String(semio_framework_os_kernel::json::to_json_string(&TypologyNode::new("hot-test", "Hot Test", vec![])))),
                ("kindsJson".to_string(), semio_framework::DslValue::String("[]".to_string())),
            ]),
        )),
    };
    let json = dsl::json::to_json_string(&vec![entry]);
    assert!(sourcing_modules(&json).iter().any(|module| module.module_id() == "hot-test"));
    assert!(!sourcing_modules("[]").iter().any(|module| module.module_id() == "hot-test"));
}

#[semio_framework_async_macros::async_test]
async fn sourcing_contribution_envelope_rejects_depth_string_and_cardinality_plus_one_before_parse() {
    let depth_plus_one = format!("{}0{}", "[".repeat(SOURCING_JSON_MAX_DEPTH + 1), "]".repeat(SOURCING_JSON_MAX_DEPTH + 1));
    assert!(!sourcing_json_envelope_is_bounded(&depth_plus_one));
    assert_eq!(sourcing_modules(&depth_plus_one).len(), 3, "invalid contribution envelope installs nothing");

    let string_plus_one = format!("\"{}\"", "x".repeat(SOURCING_JSON_MAX_STRING_BYTES + 1));
    assert!(!sourcing_json_envelope_is_bounded(&string_plus_one));
    let items_plus_one = format!("[{}]", vec!["0"; SOURCING_JSON_MAX_ITEMS].join(","));
    assert!(!sourcing_json_envelope_is_bounded(&items_plus_one));
}
