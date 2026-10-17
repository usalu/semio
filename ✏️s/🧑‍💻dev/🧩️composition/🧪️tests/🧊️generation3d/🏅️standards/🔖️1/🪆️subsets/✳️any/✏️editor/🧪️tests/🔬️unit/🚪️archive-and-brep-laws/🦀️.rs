use semio_framework_test_fixtures::repo_text;

/// 🚪️ Archive and brep laws: the archive import door, hub genesis and the brep measurement and scoped selection laws.

//#region 🚪️ArchiveImportDoor
/// 🚪️ A framework document archive of this editor loads into a FRESH instance through the shell's Import Document door
/// (`createApp` → `loadDocumentArchive` → poll → acknowledge). Measured live 2026-09-28 (S20 io-matrix,
/// `s14-s20-io/s20b-local-en-2`): generation3d trapped the guest, generation2d faulted
/// `document-archive-replacement.initializer-failed`. Two faults: the initializer held no publication lease outside tests
/// (`generation3d-publication.authority-missing`) — it now grants itself the lease the host's replacement needs — and the kernel's
/// retained hydration plain-dropped every replayed diff, whose `host_snapshot.layout` `OrderedMap` aborts on drop — it now
/// retires them through `os_vcs::apply_mutation`. The source document is EDITED first so the archive carries history to
/// replay (ticket 26/09/23 slice S19).
#[semio_framework_async_macros::async_test]
async fn a_document_archive_loads_into_a_fresh_instance_through_the_import_door() {
    let _serial = crate::publication_authority::lock();
    let mut source = app_with_registry().await;
    context::dispatch(&mut source, Generation3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: semio_s_artifact_procedural_3d_model::PROCEDURAL_EXAMPLE_SPHERE_TORUS.into() })).await;
    context::settle(&mut source).await;
    let archive = PluginApp::document_archive(&*source).await.expect("the open document exports as a framework archive");
    let expected: Vec<String> = context::snapshot(&*source).host_snapshot.widgets.iter().map(|widget| semio_s_artifact_procedural_3d_model::widget_id(widget).to_string()).collect();
    assert!(!expected.is_empty(), "the exported document carries widgets to compare");
    let mut target = app_with_registry().await;
    PluginApp::begin_document_archive_load(&mut *target, 91, archive).expect("archive admission");
    let mut status = None;
    for _ in 0..200_000 {
        let polled = PluginApp::poll_document_archive_load(&mut *target, 91).await.expect("archive status");
        if matches!(polled.state, protocol::DocumentArchiveLoadState::Ready | protocol::DocumentArchiveLoadState::Cancelled | protocol::DocumentArchiveLoadState::Fault) {
            PluginApp::acknowledge_document_archive_load(&mut *target, 91).expect("archive acknowledgement");
            status = Some(polled);
            break;
        }
        PluginApp::maintenance_step(&mut *target, 1, 4_096).expect("archive maintenance step");
    }
    let status = status.expect("the archive load reaches a terminal state");
    assert_eq!(status.state, protocol::DocumentArchiveLoadState::Ready, "{}", String::from_utf8_lossy(&status.fault));
    let loaded: Vec<String> = context::snapshot(&*target).host_snapshot.widgets.iter().map(|widget| semio_s_artifact_procedural_3d_model::widget_id(widget).to_string()).collect();
    assert_eq!(loaded, expected, "the fresh instance holds exactly the archived document");
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *target);
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *source);
}
//#endregion 🚪️ArchiveImportDoor

//#region 🌱️HubGenesis
/// 🌱️ The hub creates a `generation3d` document through this plugin's `codec.genesis` export — the SDK's one producer
/// `artifact_app_genesis_pair` — and the pair it prints must parse back to a live document. Measured on hub 7800
/// (2026-09-28, W4 open-plan probe): creation failed `ordered-map root must be explicitly retired before drop`, the
/// producer plain-dropped the initial snapshot whose `host_snapshot.layout` is an `OrderedMap` (ticket 26/09/23 slice S19).
#[semio_framework_async_macros::async_test]
async fn a_hub_genesis_pair_is_produced_and_parses_back_without_trapping() {
    let pair = semio_framework_plugin::artifact_app_genesis_pair::<semio_framework_plugin::EditorApp<Generation3dPlayApp>>("artifact-0123456789abcdef0123456789abcdef").await.expect("the genesis export produces a pair");
    assert!(!pair.pack.is_empty() && !pair.spr.is_empty(), "a genesis pair carries both a pack and an SPR");
    let snapshot = semio_framework_plugin::artifact_pair_snapshot::<semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot, semio_s_artifact_procedural_3d_mutations::Generation3dMutation>(&pair.pack, &pair.spr)
        .await
        .expect("the genesis pair parses back");
    let initial = <Generation3dPlayApp as semio_framework_plugin::ArtifactEditor>::initial_snapshot();
    let expected: Vec<String> = initial.host_snapshot.widgets.iter().map(|widget| semio_s_artifact_procedural_3d_model::widget_id(widget).to_string()).collect();
    let loaded: Vec<String> = snapshot.host_snapshot.widgets.iter().map(|widget| semio_s_artifact_procedural_3d_model::widget_id(widget).to_string()).collect();
    assert_eq!(loaded, expected, "the genesis document is exactly the editor's initial document");
    initial.retire_cold();
    snapshot.retire_cold();
}
//#endregion 🌱️HubGenesis

#[semio_framework_async_macros::async_test]
async fn mesh_brep_measurement_outputs_are_selected_read_only_in_every_locale() {
    use semio_s_artifact_procedural_3d_model::with_host;
    let _serial = test_serial();
    let _unlinked = crate::flow_operators::UnlinkedFlowExtensions::contributed();
    let fixture: Value = serde_json::from_str(repo_text!("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎚️set-widget-input/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let publication = &fixture["measurementPublication"];
    let mut app = app().await;
    let mut snapshot = semio_s_artifact_procedural_3d_model::empty_generation3d_snapshot();
    let projected = with_host(&snapshot.host_snapshot, |host| {
        host.add_widget(&json!({"kind":"neuron","id":"solid","neuronKind":publication["kind"]}).to_string(), 0.0, 0.0).unwrap();
        let params: serde_json::Map<String, Value> = publication["params"].as_object().unwrap().iter().map(|(name, value)| (name.clone(), json!({"$schema":"number","value":value}))).collect();
        host.set_neuron_params("solid", &Value::Object(params).to_string()).unwrap();
        for (index, row) in publication["cases"].as_array().unwrap().iter().enumerate() {
            let id = row["output"].as_str().unwrap();
            host.add_widget(&json!({"kind":"neuron","id":id,"neuronKind":row["kind"]}).to_string(), 220.0, index as f64 * 100.0).unwrap();
            host.connect_ports("solid", publication["output"].as_str().unwrap(), id, "geometry").unwrap();
        }
        host.host_snapshot.clone()
    });
    std::mem::replace(&mut snapshot.host_snapshot, projected).retire_cold();
    let payload = semio_framework_pack_json::to_json_string(&snapshot);
    snapshot.retire_cold();
    context::dispatch(&mut app, Generation3dCommand::ImportDocument(import_document::ImportDocument { name:"measurements.json".into(), payload, widget_id:None, channel:None, texture_id:None })).await;
    context::drain_flow_eval_ticks(&mut app).await;
    let rendered = context::render(&mut app, flow_window::GENERATION_3D_PLAY_BODY_MAIN).await;
    let scene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes::<semio_framework_plugin::NodeGraphScene>(&rendered).unwrap();
    let evaluation: Value = serde_json::from_str(scene.eval_json.as_ref().unwrap()).unwrap();
    let status = scene.status_json.as_ref().unwrap();
    let committed = context::snapshot(&app);
    for row in publication["cases"].as_array().unwrap() {
        let id = row["output"].as_str().unwrap();
        assert_eq!(node_eval_status(status,id),"ok","{id}: {status}");
        let value = evaluation[id]["out"][id]["value"].as_f64().expect("the real query publishes its numeric result");
        assert!((value-row["value"].as_f64().unwrap()).abs()<1e-10,"{id}: {evaluation}");
        context::select_graph(&mut app,"node",&[id]).await;
        for locale in [semio_framework_ui_locale::Locale::En,semio_framework_ui_locale::Locale::De] {
            let view = semio_framework_plugin::ViewModel::new(locale,semio_framework_ui_locale::Terminology::Native);
            let rendered = context::render_with_view(&mut app,semio_s_artifact_procedural_3d_editor_panels::inspection::GENERATION_3D_PLAY_BODY_INSPECTION,&view).await;
            fn output<'a>(node:&'a Value,key:&str)->Option<&'a Value> { if node["key"]==key { return Some(node); } node["children"].as_array()?.iter().find_map(|child|output(child,key)) }
            let tree: Value = serde_json::from_str(&rendered).unwrap();
            let output = output(&tree,&format!("procedural-play-inspector.output.{id}")).unwrap_or_else(||panic!("{id}: {rendered}"));
            assert!(output.to_string().contains(&value.to_string()));
            assert!(!output.to_string().contains("setWidgetInput"));
            assert_eq!(context::snapshot(&app),committed,"{id}: reading the query preserves the document");
        }
    }
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
}

#[semio_framework_async_macros::async_test]
async fn mesh_brep_live_scoped_selection_edits_the_evaluated_solid_and_restores_history() {
    use semio_s_artifact_procedural_3d_model::with_host;
    use semio_s_artifact_procedural_3d_mut_widgets::change_widget_input::WidgetInputValue;
    let _serial = test_serial();
    let _unlinked = crate::flow_operators::UnlinkedFlowExtensions::contributed();
    let fixture: Value = serde_json::from_str(repo_text!("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎚️set-widget-input/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let publication = &fixture["measurementPublication"];
    let live = &fixture["liveAnalyticPublication"];
    let source = live["source"].as_str().unwrap();
    let consumer = live["consumer"].as_str().unwrap();
    let feature = live["feature"].as_str().unwrap();
    let selector = live["selector"].as_str().unwrap();
    let mut app = app().await;
    let opened_actor = app.test_parent_store_mut().local_actor_id().0.clone();
    assert_eq!(opened_actor, live["openedActor"].as_str().unwrap());
    let mut snapshot = semio_s_artifact_procedural_3d_model::empty_generation3d_snapshot();
    let projected = with_host(&snapshot.host_snapshot, |host| {
        host.add_widget(&json!({"kind":"neuron","id":source,"neuronKind":publication["kind"]}).to_string(),0.0,0.0).unwrap();
        for widget in &mut host.host_snapshot.widgets {
            if let semio_framework_artifact_flow_flow::Widget::Neuron { id,preview,.. } = widget { if id==source { *preview=true; } }
        }
        let params: serde_json::Map<String,Value> = publication["params"].as_object().unwrap().iter().map(|(name,value)|(name.clone(),json!({"$schema":"number","value":value}))).collect();
        host.set_neuron_params(source,&Value::Object(params).to_string()).unwrap();
        let measurement = publication["cases"].as_array().unwrap().iter().find(|row|row["output"]==consumer).unwrap();
        host.add_widget(&json!({"kind":"neuron","id":consumer,"neuronKind":measurement["kind"]}).to_string(),600.0,0.0).unwrap();
        host.connect_ports(source,publication["output"].as_str().unwrap(),consumer,"geometry").unwrap();
        host.host_snapshot.clone()
    });
    std::mem::replace(&mut snapshot.host_snapshot,projected).retire_cold();
    let payload = semio_framework_pack_json::to_json_string(&snapshot);
    snapshot.retire_cold();
    context::dispatch(&mut app,Generation3dCommand::ImportDocument(import_document::ImportDocument { name:"live-brep-selection.json".into(),payload,widget_id:None,channel:None,texture_id:None })).await;
    context::drain_flow_eval_ticks(&mut app).await;
    let before = context::snapshot(&app);
    let graph = context::render(&mut app,flow_window::GENERATION_3D_PLAY_BODY_MAIN).await;
    let graph = semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes::<semio_framework_plugin::NodeGraphScene>(&graph).unwrap();
    assert_eq!(node_eval_status(graph.status_json.as_ref().unwrap(),source),"ok");
    let evaluation: Value = serde_json::from_str(graph.eval_json.as_ref().unwrap()).unwrap();
    let original = evaluation[consumer]["out"][consumer]["value"].as_f64().unwrap();
    let expected = publication["cases"].as_array().unwrap().iter().find(|row|row["output"]==consumer).unwrap()["value"].as_f64().unwrap();
    assert!((original-expected).abs()<1e-10,"the admitted source matches the independent box oracle");
    let preview = context::render(&mut app,edit_preview::GENERATION_3D_PLAY_BODY_PREVIEW).await;
    let world = semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes::<semio_framework_ui::wgpu::World3dScene>(&preview).unwrap();
    let instances: Value = serde_json::from_str(&world.instances_json).unwrap();
    let meshes: Value = serde_json::from_str(&world.meshes_json).unwrap();
    let instance = instances.as_array().unwrap().iter().find(|instance|instance["id"].as_str().is_some_and(|id|id.starts_with(&format!("{source}@")))).expect("the evaluated source reaches the viewport");
    let mesh = meshes.as_array().unwrap().iter().find(|mesh|mesh["id"]==instance["meshId"]).unwrap();
    let source_extent = |instances: &Value, meshes: &Value| {
        let instance = instances.as_array().unwrap().iter().find(|instance| instance["id"].as_str().is_some_and(|id| id.starts_with(&format!("{source}@")))).expect("the original source remains in the actual viewport");
        let mesh = meshes.as_array().unwrap().iter().find(|mesh| mesh["id"] == instance["meshId"]).unwrap();
        let data = mesh_data_from_json(&mesh["data"]);
        assert!(data.positions.len() >= 9 && data.indices.len() >= 3);
        let (min, max) = aabb_of_positions(&data.positions);
        let vertices = data.positions.chunks_exact(3).map(|point| parry3d::na::Point3::new(point[0], point[1], point[2])).collect();
        let triangles = data.indices.chunks_exact(3).map(|triangle| [triangle[0], triangle[1], triangle[2]]).collect();
        let oracle = parry3d::shape::TriMesh::with_flags(vertices, triangles, parry3d::shape::TriMeshFlags::MERGE_DUPLICATE_VERTICES).local_aabb();
        assert_eq!(min, [oracle.mins.x, oracle.mins.y, oracle.mins.z]);
        assert_eq!(max, [oracle.maxs.x, oracle.maxs.y, oracle.maxs.z]);
        let extent = [max[0] - min[0], max[1] - min[1], max[2] - min[2]];
        for axis in 0..3 { assert!((extent[axis] as f64 - live["sourceExtent"][axis].as_f64().unwrap()).abs() < 1e-6, "the original viewport source extent agrees with the neutral Three oracle: {extent:?}"); }
        extent
    };
    let original_extent = source_extent(&instances, &meshes);
    let granularity = live["componentGranularity"].as_str().unwrap();
    let group = live["group"].as_u64().unwrap();
    let label = mesh["data"]["componentReferences"][granularity][group as usize].as_str().expect("the real tessellation supplies its exact topology label");
    let handle = instance["componentSource"]["handle"].as_str().unwrap();
    let revision = instance["componentSource"]["revision"].as_str().unwrap();
    let selected = format!("{}.{granularity}.{group}~{handle}~{label}~{revision}",instance["id"].as_str().unwrap());
    assert!(selection::ComponentTarget::parse(&selected).is_some());
    context::select_domain(&mut app,selection::DOMAIN,granularity,&[&selected]).await;
    let view = context::preview_views("procedural-preview-test","procedural-preview-test-other").0;
    let change = context::act_with_view(&mut app,"editMeshSelection",json!({"operation":live["operation"],"amount":live["amount"]}),view.clone()).await.unwrap();
    let committed = context::snapshot(&app);
    let history_row = PluginApp::history_snapshot(&mut *app).await.expect("original history").upserts.into_iter().filter(|entry| entry.applied && entry.transaction.as_ref().is_some_and(|transaction| transaction.tool == "s.procedural.generation3d@1/*#editor#editMeshSelection")).max_by_key(|entry| entry.seq).expect("the selected edit has its real tool history row");
    assert_eq!(history_row.author.as_deref(), Some(opened_actor.as_str()));
    assert_eq!(app.test_parent_store_mut().local_actor_id().0, opened_actor);
    assert_eq!(committed.host_snapshot.widgets.len(),before.host_snapshot.widgets.len()+2);
    let source_row = committed.host_snapshot.widgets.iter().find(|widget|semio_s_artifact_procedural_3d_model::widget_id(widget)==selector).unwrap();
    let literal = |widget: &semio_framework_artifact_flow_flow::Widget,key: &str| match widget {
        semio_framework_artifact_flow_flow::Widget::Neuron { params,.. } => params.get(key).and_then(|value|WidgetInputValue::of_literal(&semio_framework_value::ToValue::to_value(value))),
        _ => None,
    };
    assert_eq!(literal(source_row,"sourceHandle"),Some(WidgetInputValue::Text(handle.into())));
    assert_eq!(literal(source_row,"faceLabels"),Some(WidgetInputValue::Text(json!([label]).to_string())));
    assert!(committed.host_snapshot.synapses.iter().any(|wire|wire.from==selector&&wire.from_port=="selectedFaces"&&wire.to==feature&&wire.to_port=="openFaces"));
    assert!(committed.host_snapshot.synapses.iter().any(|wire|wire.from==feature&&wire.to==consumer&&wire.to_port=="geometry"));
    let run = context::drive_preview_run(&mut app,&view,&change.effects).await;
    assert!(run.answered>0,"the selected edit crosses the packaged inference route");
    let graph = context::render(&mut app,flow_window::GENERATION_3D_PLAY_BODY_MAIN).await;
    let graph = semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes::<semio_framework_plugin::NodeGraphScene>(&graph).unwrap();
    for id in [selector,feature,consumer] { assert_eq!(node_eval_status(graph.status_json.as_ref().unwrap(),id),"ok","{id}: {:?}",graph.eval_json); }
    let evaluation: Value = serde_json::from_str(graph.eval_json.as_ref().unwrap()).unwrap();
    let edited = evaluation[consumer]["out"][consumer]["value"].as_f64().unwrap();
    assert!(edited.is_finite()&&edited>0.0&&edited<original,"the physical shell changes the downstream solid volume: {evaluation}");
    for verb in live["history"].as_array().unwrap() {
        let verb = verb.as_str().unwrap();
        semio_framework_plugin::artifact_app_laws::settle_history_verb(&mut *app,verb,semio_framework_plugin::artifact_app_laws::meta("local").instance_id).await;
        let restored = context::snapshot(&app);
        assert_eq!(&restored,if verb=="undo" { &before } else { &committed });
        context::drain_flow_eval_ticks(&mut app).await;
        let graph = context::render(&mut app,flow_window::GENERATION_3D_PLAY_BODY_MAIN).await;
        let graph = semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes::<semio_framework_plugin::NodeGraphScene>(&graph).unwrap();
        assert_eq!(node_eval_status(graph.status_json.as_ref().unwrap(),consumer),"ok");
        let evaluation: Value = serde_json::from_str(graph.eval_json.as_ref().unwrap()).unwrap();
        let actual = evaluation[consumer]["out"][consumer]["value"].as_f64().unwrap();
        assert!((actual-if verb=="undo" { original } else { edited }).abs()<1e-10,"{verb}: evaluated geometry and consumer restore together");
        let preview = context::render(&mut app,edit_preview::GENERATION_3D_PLAY_BODY_PREVIEW).await;
        let world = semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes::<semio_framework_ui::wgpu::World3dScene>(&preview).unwrap();
        let instances: Value = serde_json::from_str(&world.instances_json).unwrap();
        let meshes: Value = serde_json::from_str(&world.meshes_json).unwrap();
        assert_eq!(source_extent(&instances, &meshes), original_extent, "{verb}: actual viewport source extent restores through the original history");
        let restored_row = PluginApp::history_snapshot(&mut *app).await.expect("restored original history").upserts.into_iter().find(|entry| entry.seq == history_row.seq).expect("the original edit remains represented in history");
        assert_eq!(restored_row.author.as_deref(), Some(opened_actor.as_str()), "{verb}: original history retains the opening actor");
        assert_eq!(app.test_parent_store_mut().local_actor_id().0, opened_actor);
    }
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
}
