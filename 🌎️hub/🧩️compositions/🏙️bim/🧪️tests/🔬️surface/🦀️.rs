use semio_framework_plugin::artifact_app_laws::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

#[semio_framework_async_macros::async_test]
async fn bim_viewer_never_mutates() {
    assert_viewer_never_mutates::<crate::viewer::bim::BimModelViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn bim_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<crate::editor::bim::BimModelApp, crate::viewer::bim::BimModelViewer>().await;
}

#[test]
fn bim_plugin_assembles_a_real_manifest_not_the_assembly_failed_stub() {
    let plugin = super::plugin().expect("bim plugin must assemble cleanly");
    assert_ne!(plugin.manifest.plugin_id, "assembly-failed", "manifest must not be the try_build()-failed stub");
    assert_eq!(plugin.manifest.plugin_id, "bim");
    assert_eq!(plugin.manifest.apps.len(), 2, "one editor and one viewer surface, exactly");
}

#[test]
fn bim_editor_and_viewer_declare_the_same_dialect() {
    let editor = crate::editor::bim::create_bim_app();
    let viewer = crate::viewer::bim::create_bim_viewer();
    assert_eq!(editor.dialect, viewer.dialect);
}

#[test]
fn bim_declares_its_native_codec_and_the_ifc_gltf_and_svg_hops_through_the_declaration_tree() {
    let tree = crate::artifacts::model::artifact::<super::BimApps>();
    assert_eq!(tree.kind.as_str(), "s.bim.model");
    let subset = &tree.standards[0].subsets[0];
    assert_eq!(subset.io.native.codec.schema, crate::artifacts::model::BIM_MODEL_DOCUMENT_SCHEMA);
    let hops: Vec<_> = subset.io.entries.iter().map(|entry| (entry.from.artifact_kind, entry.into.artifact_kind)).collect();
    assert_eq!(hops, [("s.bim.model", "s.stdio.ifc"), ("s.bim.model", "s.stdio.gltf"), ("s.bim.model", "s.stdio.svg"), ("s.stdio.ifc", "s.bim.model")]);
}
