
use semio_plugin::artifact_app_laws::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

#[semio_framework_async_macros::async_test]
async fn puzzle2d_viewer_never_mutates() {
    assert_viewer_never_mutates::<semio_s_artifact_puzzle_2d::viewer::puzzle2d::Puzzle2dViewer>(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity()).await;
}

#[semio_framework_async_macros::async_test]
async fn puzzle2d_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<semio_s_artifact_puzzle_2d::editor::puzzle2d::Puzzle2dPlayApp, semio_s_artifact_puzzle_2d::viewer::puzzle2d::Puzzle2dViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn puzzle3d_viewer_never_mutates() {
    assert_viewer_never_mutates::<semio_s_artifact_puzzle_3d_viewer::Puzzle3dViewer>(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity()).await;
}

#[semio_framework_async_macros::async_test]
async fn puzzle3d_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<semio_s_artifact_puzzle_3d_editor::Puzzle3dPlayApp, semio_s_artifact_puzzle_3d_viewer::Puzzle3dViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn puzzle5d_viewer_never_mutates() {
    assert_viewer_never_mutates::<semio_s_artifact_puzzle_5d::viewer::puzzle5d::Puzzle5dViewer>(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity()).await;
}

#[semio_framework_async_macros::async_test]
async fn puzzle5d_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<semio_s_artifact_puzzle_5d::editor::puzzle5d::Puzzle5dPlayApp, semio_s_artifact_puzzle_5d::viewer::puzzle5d::Puzzle5dViewer>().await;
}

/// 🧬️ LAW (ticket 26/09/23, H12): puzzle's guest `codec` answers — through each owning editor's codec table, no app
/// constructed — agree with each kind's independently declared native codec (`artifact_codec::<Snapshot, Mutation>`)
/// for all three kinds: the same pack-schema hash, the native codec decodes the table's genesis pair, and both print the
/// same op log of it. (The two `dsl` mirrors are different renderings — the store's document text and the kind's own
/// DSL — so they are not compared byte for byte.)
#[semio_framework_async_macros::async_test]
async fn guest_codec_tables_answer_like_the_declared_native_codecs() {
    let runtime = semio_plugin::plugin_runtime::PluginRuntime::new(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT).expect("explicit test mounted owner policy");
    semio_plugin::plugin_runtime::install_plugin_bundle(&runtime, crate::plugin().expect("puzzle assembles"));
    let document_id = format!("artifact-{}", "3".repeat(32));
    let declarations = [semio_s_artifact_puzzle_2d::artifact::<crate::PuzzleApps>(), semio_s_artifact_puzzle_3d::artifact::<crate::PuzzleApps>(), semio_s_artifact_puzzle_5d::artifact::<crate::PuzzleApps>()];
    let mut compared = Vec::new();
    for declaration in declarations {
        for standard in declaration.standards {
            for subset in standard.subsets {
                let codec = subset.io.native.codec;
                let hash = semio_plugin::plugin_runtime::plugin_artifact_pack_schema_hash(&runtime, &codec.schema).await.unwrap_or_else(|fault| panic!("{}: {fault:?}", codec.schema));
                assert_eq!(hash, codec.pack_schema_hash, "{}: pack-schema hash", codec.schema);
                let pair = semio_plugin::plugin_runtime::plugin_artifact_genesis(&runtime, &codec.schema, &document_id).await.unwrap_or_else(|fault| panic!("{}: genesis {fault:?}", codec.schema));
                let guest = semio_plugin::plugin_runtime::plugin_artifact_print_mirror(&runtime, &codec.schema, &pair.pack, &pair.spr).await.unwrap_or_else(|fault| panic!("{}: guest mirror {fault:?}", codec.schema));
                let native = (codec.reference().expect("declared actual cold reference capability").print_mirror)(&pair.pack, &pair.spr).await.unwrap_or_else(|error| panic!("{}: native mirror {error:?}", codec.schema));
                assert_eq!(guest.ops, native.ops, "{}: op-log mirror", codec.schema);
                assert!(!guest.dsl.is_empty() && !native.dsl.is_empty(), "{}: both render the document", codec.schema);
                compared.push(codec.schema.clone());
            }
        }
    }
    assert_eq!(compared.len(), 3, "every puzzle kind compared: {compared:?}");
}

#[test]
fn declared_io_routes_match_the_complete_advertised_roster() {
    use semio_framework_artifact_reference::io::text::artifact_reference::DialectCoordinateText as _;
    use semio_os_kernel::io::io_mechanism::IoEntryDirection;
    let contract: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🚪️io-routes/🔣️.json")).unwrap();
    let declarations = [semio_s_artifact_puzzle_2d::artifact::<crate::PuzzleApps>(), semio_s_artifact_puzzle_3d::artifact::<crate::PuzzleApps>(), semio_s_artifact_puzzle_5d::artifact::<crate::PuzzleApps>()];
    assert_eq!(contract["artifacts"].as_array().unwrap().len(), declarations.len());
    for (declaration, expected) in declarations.into_iter().zip(contract["artifacts"].as_array().unwrap()) {
        let mut imports = std::collections::BTreeSet::new();
        let mut exports = std::collections::BTreeSet::new();
        for standard in declaration.standards {
            for subset in standard.subsets {
                let native = semio_framework_artifact_reference::ArtifactDialect::from(subset.dialect).to_coordinate();
                assert_eq!(native, expected["native"].as_str().unwrap());
                for entry in subset.io.entries {
                    let from = semio_framework_artifact_reference::ArtifactDialect::from(entry.from).to_coordinate();
                    let into = semio_framework_artifact_reference::ArtifactDialect::from(entry.into).to_coordinate();
                    match entry.direction {
                        IoEntryDirection::Import => { assert_eq!(into, native); assert!(imports.insert(from)); }
                        IoEntryDirection::Export => { assert_eq!(from, native); assert!(exports.insert(into)); }
                    }
                }
            }
        }
        for (actual, key) in [(imports, "imports"), (exports, "exports")] {
            let expected: std::collections::BTreeSet<String> = expected[key].as_array().unwrap().iter().map(|value| value.as_str().unwrap().to_owned()).collect();
            assert_eq!(actual, expected);
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn declared_exact_io_routes_restore_the_registered_native_document() {
    use semio_framework_artifact_reference::io::text::artifact_reference::DialectCoordinateText as _;
    use semio_framework::io_schema::{IoPayload,IoRoute};
    let runtime = semio_plugin::plugin_runtime::PluginRuntime::new(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT).expect("explicit test mounted owner policy");
    semio_plugin::plugin_runtime::install_plugin_bundle(&runtime, crate::plugin().expect("actual Puzzle declaration registers"));
    let contract: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🚪️io-routes/🔣️.json")).unwrap();
    let declarations = [semio_s_artifact_puzzle_2d::artifact::<crate::PuzzleApps>(), semio_s_artifact_puzzle_3d::artifact::<crate::PuzzleApps>(), semio_s_artifact_puzzle_5d::artifact::<crate::PuzzleApps>()];
    let routes = semio_os_kernel::io::io_mechanism::io_entries();
    let mut compared = 0;
    let mut exported_count = 0;
    for (declaration, expected) in declarations.into_iter().zip(contract["artifacts"].as_array().unwrap()) {
        for standard in declaration.standards {
            for subset in standard.subsets {
                let codec = subset.io.native.codec;
                let pair = semio_plugin::plugin_runtime::plugin_artifact_genesis(&runtime, &codec.schema, &format!("artifact-{}", "4".repeat(32))).await.expect("actual owning guest codec genesis");
                let before = (codec.reference().expect("declared actual cold reference capability").print_mirror)(&pair.pack, &pair.spr).await.expect("native reference mirror");
                for foreign in expected["exports"].as_array().unwrap() {
                    let foreign = foreign.as_str().unwrap();
                    let export = routes.iter().find(|entry| entry.from.to_coordinate() == expected["native"].as_str().unwrap() && entry.into.to_coordinate() == foreign).expect("advertised exact export registered").clone();
                    let export_fidelity=export.fidelity;
                    let mut progress = |_| true;
                    let exported = original_owners::io_run(&IoRoute { fidelity: export.fidelity, hops: vec![export] }, IoPayload::Binary(pair.pack.clone()), semio_os_kernel::sqlite_snapshot::SqliteDatabaseLimits::default(), &mut progress).await.expect("actual registered export with original authorities").value;
                    let (kind,length) = match &exported { IoPayload::Text(text) => ("text",text.len()), IoPayload::Binary(bytes) => ("binary",bytes.len()) };
                    assert_eq!(kind,expected["payloadKinds"][foreign].as_str().unwrap(),"{} through {} carrier",codec.schema,foreign);
                    assert!(length>0,"actual declared format has material bytes");exported_count+=1;
                    if !expected["imports"].as_array().unwrap().iter().any(|value|value.as_str()==Some(foreign)){continue}
                    let import = routes.iter().find(|entry| entry.from.to_coordinate() == foreign && entry.into.to_coordinate() == expected["native"].as_str().unwrap()).expect("advertised exact import registered").clone();
                    assert_eq!(export_fidelity,semio_framework::io_schema::IoFidelity::Exact);
                    assert_eq!(import.fidelity,semio_framework::io_schema::IoFidelity::Exact);
                    let restored = original_owners::io_run(&IoRoute { fidelity: import.fidelity, hops: vec![import] }, exported, semio_os_kernel::sqlite_snapshot::SqliteDatabaseLimits::default(), &mut progress).await.expect("actual registered import with original authorities").value;
                    let IoPayload::Binary(restored) = restored else { panic!("binary native constructor publishes native pack") };
                    let after = (codec.reference().expect("declared actual cold reference capability").print_mirror)(&restored, &pair.spr).await.expect("restored native reference mirror");
                    assert_eq!(after.dsl, before.dsl, "{} through {}", codec.schema, foreign);
                    assert_eq!(after.ops, before.ops, "{} through {}", codec.schema, foreign);
                    compared += 1;
                }
            }
        }
    }
    assert_eq!(exported_count,13,"every advertised export preserves its declared carrier");
    assert_eq!(compared, 7, "all declared bidirectional formats execute");
}
