use semio_plugin::artifact_app_laws::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

fn descriptor() -> (serde_json::Value, serde_json::Value) {
    let contract: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🛂️committed-descriptor/🔣️.json")).expect("BIM registration contract");
    let path = semio_framework_test_fixtures::repo_root!().join(contract["descriptorPath"].as_str().expect("descriptor path")).with_file_name("🔣️.json");
    let published = serde_json::from_reader(std::io::BufReader::new(std::fs::File::open(path).expect("published descriptor file"))).expect("published descriptor JSON");
    (contract, published)
}

#[semio_framework_async_macros::async_test]
async fn bim_viewer_never_mutates() {
    assert_viewer_never_mutates::<semio_s_artifact_bim_model_viewer::viewer::bim::BimModelViewer>(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity()).await;
}

#[semio_framework_async_macros::async_test]
async fn bim_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<semio_s_artifact_bim_model_bim::editor::bim::BimModelApp, semio_s_artifact_bim_model_viewer::viewer::bim::BimModelViewer>().await;
}

#[test]
fn bim_catalog_and_plugin_assembly_expose_the_same_document_apps() {
    let (contract, descriptor) = descriptor();
    let plugin_id = contract["pluginId"].as_str().expect("plugin identity");
    let expected: Vec<String> = serde_json::from_value(contract["applicationIds"].clone()).expect("application identities");
    let plugin = super::plugin().expect("BIM plugin assembles");
    assert_eq!(plugin.manifest.plugin_id, plugin_id);
    assert_eq!(descriptor["manifest"]["pluginId"], plugin_id);
    let mut assembled: Vec<&str> = plugin.manifest.apps.iter().map(|app| app.id.as_str()).collect();
    let mut published: Vec<&str> = descriptor["manifest"]["apps"].as_array().expect("published apps").iter().map(|app| app["id"].as_str().expect("published app identity")).collect();
    assembled.sort_unstable();
    published.sort_unstable();
    assert_eq!(assembled, expected);
    assert_eq!(published, expected);
}

#[test]
fn bim_editor_and_viewer_declare_the_same_dialect() {
    let editor = semio_s_artifact_bim_model_bim::editor::bim::create_bim_app();
    let viewer = semio_s_artifact_bim_model_viewer::viewer::bim::create_bim_viewer();
    assert_eq!(editor.dialect, viewer.dialect);
}

#[test]
fn bim_declares_its_native_codec_and_all_foreign_dialect_hops() {
    let (contract, descriptor) = descriptor();
    let expected: Vec<(String, String)> = serde_json::from_value(contract["ioHops"].clone()).expect("BIM declared dialect hops");
    let tree = crate::artifacts::model::artifact::<super::BimApps>();
    assert_eq!(tree.kind.as_str(), "s.bim.model");
    let subset = &tree.standards[0].subsets[0];
    assert_eq!(subset.io.native.codec.schema, semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::BIM_MODEL_DOCUMENT_SCHEMA);
    let mut native: Vec<_> = subset.io.entries.iter().map(|entry| {
        let (direction, counterpart) = if entry.from.artifact_kind == "s.bim.model" {
            ("export", &entry.into)
        } else {
            assert_eq!(entry.into.artifact_kind, "s.bim.model");
            ("import", &entry.from)
        };
        (direction.to_owned(), format!("{}@{}/{}", counterpart.artifact_kind, counterpart.standard.0, counterpart.subset.0))
    }).collect();
    let mut published: Vec<_> = descriptor["contributions"]["ioEntries"].as_array().expect("BIM published dialect hops").iter().map(|entry| {
        let direction = entry["direction"].as_str().expect("direction");
        let counterpart = &entry["counterpart"];
        let field = |name| counterpart[name].as_str().expect("counterpart dialect field");
        (direction.to_owned(), format!("{}@{}/{}", field("artifactKind"), field("standard"), field("subset")))
    }).collect();
    native.sort_unstable();
    published.sort_unstable();
    assert_eq!(native, expected);
    assert_eq!(published, expected);
}
