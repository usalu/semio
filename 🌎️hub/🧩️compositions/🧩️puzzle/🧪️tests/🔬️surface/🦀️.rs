
use semio_framework_plugin::artifact_app_laws::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

#[semio_framework_async_macros::async_test]
async fn puzzle2d_viewer_never_mutates() {
    assert_viewer_never_mutates::<semio_s_artifact_puzzle_2d::viewer::puzzle2d::Puzzle2dViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn puzzle2d_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<semio_s_artifact_puzzle_2d::editor::puzzle2d::Puzzle2dPlayApp, semio_s_artifact_puzzle_2d::viewer::puzzle2d::Puzzle2dViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn puzzle3d_viewer_never_mutates() {
    assert_viewer_never_mutates::<semio_s_artifact_puzzle_3d::viewer::puzzle3d::Puzzle3dViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn puzzle3d_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<semio_s_artifact_puzzle_3d::editor::puzzle3d::Puzzle3dPlayApp, semio_s_artifact_puzzle_3d::viewer::puzzle3d::Puzzle3dViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn puzzle5d_viewer_never_mutates() {
    assert_viewer_never_mutates::<semio_s_artifact_puzzle_5d::viewer::puzzle5d::Puzzle5dViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn puzzle5d_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<semio_s_artifact_puzzle_5d::editor::puzzle5d::Puzzle5dPlayApp, semio_s_artifact_puzzle_5d::viewer::puzzle5d::Puzzle5dViewer>().await;
}

/// 🧬️ LAW (ticket 26/09/23, H12): puzzle's guest `codec` answers — through each owning editor's codec table, no app
/// constructed — agree with each kind's independently declared native codec (`ArtifactCodec::of::<Snapshot, Mutation>`)
/// for all three kinds: the same pack-schema hash, the native codec decodes the table's genesis pair, and both print the
/// same op log of it. (The two `dsl` mirrors are different renderings — the store's document text and the kind's own
/// DSL — so they are not compared byte for byte.)
#[semio_framework_async_macros::async_test]
async fn guest_codec_tables_answer_like_the_declared_native_codecs() {
    let runtime = semio_framework_plugin::plugin_runtime::PluginRuntime::new();
    semio_framework_plugin::plugin_runtime::install_plugin_bundle(&runtime, crate::plugin().expect("puzzle assembles"));
    let document_id = format!("artifact-{}", "3".repeat(32));
    let declarations = [semio_s_artifact_puzzle_2d::artifact::<crate::PuzzleApps>(), semio_s_artifact_puzzle_3d::artifact::<crate::PuzzleApps>(), semio_s_artifact_puzzle_5d::artifact::<crate::PuzzleApps>()];
    let mut compared = Vec::new();
    for declaration in declarations {
        for standard in declaration.standards {
            for subset in standard.subsets {
                let codec = subset.io.native.codec;
                let hash = semio_framework_plugin::plugin_runtime::plugin_artifact_pack_schema_hash(&runtime, &codec.schema).await.unwrap_or_else(|fault| panic!("{}: {fault:?}", codec.schema));
                assert_eq!(hash, codec.pack_schema_hash, "{}: pack-schema hash", codec.schema);
                let pair = semio_framework_plugin::plugin_runtime::plugin_artifact_genesis(&runtime, &codec.schema, &document_id).await.unwrap_or_else(|fault| panic!("{}: genesis {fault:?}", codec.schema));
                let guest = semio_framework_plugin::plugin_runtime::plugin_artifact_print_mirror(&runtime, &codec.schema, &pair.pack, &pair.spr).await.unwrap_or_else(|fault| panic!("{}: guest mirror {fault:?}", codec.schema));
                let native = (codec.print_mirror)(&pair.pack, &pair.spr).await.unwrap_or_else(|error| panic!("{}: native mirror {error:?}", codec.schema));
                assert_eq!(guest.ops, native.ops, "{}: op-log mirror", codec.schema);
                assert!(!guest.dsl.is_empty() && !native.dsl.is_empty(), "{}: both render the document", codec.schema);
                compared.push(codec.schema.clone());
            }
        }
    }
    assert_eq!(compared.len(), 3, "every puzzle kind compared: {compared:?}");
}
