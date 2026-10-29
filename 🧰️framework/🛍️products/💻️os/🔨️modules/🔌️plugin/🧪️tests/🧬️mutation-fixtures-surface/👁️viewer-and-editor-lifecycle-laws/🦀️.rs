/// 👁️ Viewer and editor lifecycle laws of the surface fixture: the viewer never mutates, the declared lifecycle hooks, the shared dialect, the registry less viewer wrapper, the editor fixture mutations and the natural bytes through the real metadata path.

#[semio_framework_async_macros::async_test]
async fn viewer_never_mutates_the_document_or_draft_store() {
    assert_viewer_never_mutates::<SurfaceViewerFixture>(crate::app_testing::artifact_app_laws::fixture_mounted_policy(), &mut crate::app_testing::artifact_app_laws::fixture_identity()).await;
}

#[semio_framework_async_macros::async_test]
async fn bounded_viewer_fixture_preserves_declared_lifecycle_hooks() {
    let mut original_caller=crate::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().unwrap();let mut cx=original_caller.context();
    type Fixture = crate::app_testing::artifact_app_laws::BoundedViewerFixture<SurfaceViewerFixture>;
    let demand = <Fixture as ArtifactViewer>::mounted_job_maintenance_demands(u32::MAX, 0).expect("maintenance quote");
    assert_eq!((demand.release_bytes, demand.depth), (7, 1));
    let grant = semio_framework_plugin_ownership::bounded_ownership::plugin_demand_grant(demand);
    let short = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_release_bytes: demand.release_bytes - 1, ..grant };
    assert_eq!(<Fixture as ArtifactViewer>::mounted_job_maintenance_step(u32::MAX,&mut cx,short).expect("maintenance hook"), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(Default::default()), "a grant below the quoted release axis yields");
    assert_eq!(<Fixture as ArtifactViewer>::mounted_job_maintenance_step(u32::MAX,&mut cx,grant).expect("maintenance hook"), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, released_bytes: 7, ..Default::default() }));
    assert_eq!(<Fixture as ArtifactViewer>::mounted_job_close_step(u32::MAX,&mut cx,grant).expect("close hook"), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Blocked { reason: "surface viewer lifecycle witness" });
    assert!(!<Fixture as ArtifactViewer>::mounted_jobs_terminal_is_empty(u32::MAX));
    assert_eq!(<Fixture as ArtifactViewer>::config_schema(), "semio.testkit-surface.viewer-config/v1");
    assert_eq!(<Fixture as ArtifactViewer>::command_id(&SurfaceViewerCommand::Noop), "surface-viewer-noop");
    let fuel=cx.fuel_remaining();drop(cx);original_caller.retire(fuel).unwrap();
}

#[semio_framework_async_macros::async_test]
async fn editor_and_viewer_share_one_dialect() {
    assert_editor_and_viewer_share_dialect::<SurfaceEditorFixture, SurfaceViewerFixture>().await;
}

#[semio_framework_async_macros::async_test]
async fn new_viewer_constructs_a_registry_less_wrapper() {
    let mut original_caller=crate::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().expect("original caller policy admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
    let mut app = new_viewer::<SurfaceViewerFixture>(protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), crate::app_testing::artifact_app_laws::fixture_mounted_policy(), &mut crate::app_testing::artifact_app_laws::fixture_identity()).await;
    assert_eq!(app.snapshot().unwrap().count, 0);
    close_registered_fixture_app(&mut app, crate::app_testing::artifact_app_laws::fixture_mounted_policy(),cx).expect("original fixture terminal close");

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original caller paid terminal retirement");result
}

/// ✅️ The editor surface mutates through the SAME route a product editor uses: a declared
/// `Migrated` `app_command`, an exact app-owned `SurfaceFixtureFactory` keyed on the derived
/// controller id, and a real `InteractiveJob` that hands back one exact completion. Before the
/// factory existed this law dispatched the generic `"typed-command"` verb into a registry-less
/// wrapper and died on `interactive-job.unknown-key` — it proved what the runtime forbids, not that
/// an editor mutates.
///
/// 🔁️ A migrated verb's dispatch only ADMITS: the document advances when the worker's emit walks
/// the bounded publication ladder, which is the product's own route and what the host drives every
/// turn. Asserting the count before settling would assert that a migrated editor does NOT mutate.
#[semio_framework_async_macros::async_test]
async fn editor_fixture_still_mutates_normally() {
    let mut original_caller=crate::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().expect("original caller policy admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
    let mut app = new_registered_app::<EditorApp<SurfaceEditorFixture>, _>(surface_manifest(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), crate::app_testing::artifact_app_laws::fixture_mounted_policy(), &mut crate::app_testing::artifact_app_laws::fixture_identity()).await;
    crate::app_testing::artifact_app_laws::dispatch_original_fixture_command(&mut app, &SurfaceEditorCommand::Increment, &meta("local"), crate::app_testing::artifact_app_laws::fixture_mounted_policy(), &mut crate::app_testing::artifact_app_laws::fixture_identity(), cx).await.expect("increment");
    let receipt = crate::app_testing::artifact_app_laws::settle_registered_typed_operation(&mut app, meta("local").instance_id, crate::app_testing::artifact_app_laws::fixture_mounted_policy(), &mut crate::app_testing::artifact_app_laws::fixture_identity(),cx).await.expect("the admitted operation settles");
    assert!(receipt.lanes.contains(&crate::tool_contract::TypedOperationResultLane::Artifact), "the increment settles on the Artifact publication lane it declares, got {:?}", receipt.lanes);
    assert_eq!(app.snapshot().unwrap().count, 1);
    close_registered_fixture_app(&mut app, crate::app_testing::artifact_app_laws::fixture_mounted_policy(),cx).expect("original fixture terminal close");

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original caller paid terminal retirement");result
}

#[semio_framework_async_macros::async_test]
async fn registered_editor_consumes_intrinsic_natural_bytes_through_the_real_media_route() {
    let mut original_caller=crate::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().expect("original caller policy admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📄️natural-file-lifecycle/🔣️.json")).expect("natural-file lifecycle fixture");
    let row = &fixture["registeredEditor"];
    let format_kind = row["formatKind"].as_str().expect("registered editor format kind");
    let data = row["octets"].as_array().expect("registered editor octets").iter().map(|octet| octet.as_u64().expect("octet") as u8).collect::<Vec<_>>();
    let expected = i32::try_from(row["expectedCount"].as_i64().expect("expected surface count")).expect("i32 surface count");
    let artifact = crate::app_contract::MediaArtifact {
        descriptor: crate::app_contract::MediaArtifactDescriptor {
            edge_id: None,
            port_id: Some(crate::app_contract::NATURAL_FILE_PORT.into()),
            kind_id: Some(format_kind.into()),
            media_type: None,
            wire: semio_framework::MediaWireFormat::Binary { format_kind: format_kind.into() },
            blob_hash: None,
        },
        data,
    };
    let mut app = new_registered_app::<EditorApp<SurfaceEditorFixture>, _>(surface_manifest(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), crate::app_testing::artifact_app_laws::fixture_mounted_policy(), &mut crate::app_testing::artifact_app_laws::fixture_identity()).await;
    app.consume_media(crate::app_contract::NATURAL_FILE_PORT, artifact).await.expect("registered editor consumes natural bytes");
    let receipt = crate::app_testing::artifact_app_laws::settle_registered_typed_operation(&mut app, meta("local").instance_id, crate::app_testing::artifact_app_laws::fixture_mounted_policy(), &mut crate::app_testing::artifact_app_laws::fixture_identity(),cx).await.expect("natural-file import settles");
    let loads = receipt.effects.into_iter().filter_map(|effect| match effect { semio_framework::kernel::Effect::LoadDocument { pack, spr } => Some(store::os_store::serialization::ArtifactPackFiles { pack, spr, ops: String::new() }), _ => None }).collect::<Vec<_>>();
    assert_eq!(loads.len(), 1, "natural-file Open publishes exactly one document load, no mutation");
    crate::app_testing::artifact_app_laws::load_document(&mut app, &loads[0], &mut crate::app_testing::artifact_app_laws::fixture_identity()).await.expect("the document load lands");
    assert_eq!(app.snapshot().expect("imported surface snapshot").count, expected);
    let history_rows = app.history_snapshot().await.expect("natural-file import history").upserts.into_iter().filter(|entry| entry.edit_id.is_some()).count();
    let expected_history_rows = usize::try_from(fixture["lifecycle"]["expected"]["openedHistoryEntries"].as_u64().expect("opened history entries")).expect("usize history count");
    assert_eq!(history_rows, expected_history_rows, "natural-file Open is a document load: it writes no history row into the fresh owner history");
    close_registered_fixture_app(&mut app, crate::app_testing::artifact_app_laws::fixture_mounted_policy(),cx).expect("original fixture terminal close");

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original caller paid terminal retirement");result
}
