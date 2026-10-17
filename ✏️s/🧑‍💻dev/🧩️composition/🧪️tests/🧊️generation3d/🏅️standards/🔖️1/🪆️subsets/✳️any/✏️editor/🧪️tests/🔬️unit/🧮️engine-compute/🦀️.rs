use semio_framework_test_fixtures::repo_text;

// 🧮️ Engine compute: preview payloads, mesh bridges, preview marks and graph projections.

//#region 🔖️EngineComputeTests
/// 🧬️ Rehomed verbatim from the deleted `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — these tests exercise
/// `PreviewPipeline`/`MeshBridge` functions above, all of which are app
/// behavior (they construct or take a [`Generation3dConfig`]), so the tests travel with them.
use semio_framework_ui::wgpu::kernel_3d_scene::{aabb_intersects_frustum, frustum_planes, transform_aabb, Camera3d, Instance3d, Vec3};
use semio_s_artifact_procedural_3d_mutations::central_apply::apply_generation3d_mutation;

fn preview_payload_from_evaluated_fixture(host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot, cfg: &Generation3dConfig) -> (String, String) {
    let eval_json = semio_s_artifact_procedural_3d_model::with_host(host_snapshot, |host| host.evaluate().unwrap_or_default());
    let mut session = FlowEvalSession::new().with_geometry_port(crate::flow_operators::geometry_session().port());
    crate::flow_operators::resolve_preview_geometry(&eval_json, host_snapshot, &cfg.lod_mode, &mut session);
    let payload = preview_payload(&eval_json, host_snapshot, cfg, Some(&session), &Default::default());
    crate::flow_operators::retire_flow_eval_session(session);
    (payload.meshes_json, payload.instances_json)
}

/// 🪪️ The geometry export reads the supplied instance's prepared document and refuses a closed owner.
#[test]
fn geometry_media_export_uses_the_supplied_instance_owner_and_refuses_closed_authority() {
    let _serial = test_serial();
    use semio_framework_plugin::{ArtifactEditor, MediaClass, MediaForm, MediaPayload, TransientView};
    let fixtures: Value = serde_json::from_str(repo_text!("🧰️framework/🔨️modules/🏗️mesh-engine/🥽️polygon/🧪️tests/🧫️fixtures/🔣️.json")).expect("the language-neutral mesh corpus");
    let fixture = fixtures["meshes"].as_array().unwrap().iter().find(|case| case["name"] == "unit-tetrahedron").expect("the independent tetrahedron fixture");
    let source = semio_framework_plugin::MeshData {
        positions: fixture["mesh"]["vertices"].as_array().unwrap().iter().flat_map(|vertex| vertex.as_array().unwrap().iter().map(|coordinate| coordinate.as_f64().unwrap() as f32)).collect(),
        indices: fixture["mesh"]["faces"].as_array().unwrap().iter().flat_map(|face| face.as_array().unwrap().iter().map(|index| index.as_u64().unwrap() as u32)).collect(),
        ..Default::default()
    };
    let document = generation3d_document_from_mesh(&source).expect("an imported tetrahedron document");
    let projection = <Generation3dSnapshot as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&document)).expect("the actual imported graph");
    let owner = context::instance_operation_owner();
    let eval_json = semio_s_artifact_procedural_3d_model::with_host(&projection.host_snapshot, |host| host.evaluate().expect("the imported graph evaluates"));
    owner.with_mut::<Generation3dInstanceOperationOwner, _>(|owner| {
        owner.with_session_waking(|session| {
            let previous = std::mem::replace(session, FlowEvalSession::new().with_geometry_port(crate::flow_operators::geometry_session().port()));
            crate::flow_operators::retire_flow_eval_session(previous);
            session.set_eval_json(eval_json.clone());
            crate::flow_operators::resolve_preview_geometry(&eval_json, &projection.host_snapshot, &Generation3dConfig::default().lod_mode, session);
        })
    }).expect("the supplied owner retains the prepared evaluation");
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&projection, &history);
    let transient = Generation3dTransient::default();
    let transient = TransientView { snapshot: &transient, window: None };
    let media = Generation3dPlayApp::export_media_with_request_context(&owner, "geometry:out", &doc, &transient).expect("the supplied owner exports real geometry");
    assert_eq!(media.media_type.class, MediaClass::ThreeD);
    assert_eq!(media.media_type.form, MediaForm::Mesh);
    let MediaPayload::Structured { schema, json } = media.payload else { panic!("geometry export must carry a structured mesh"); };
    assert_eq!(schema, "3d.mesh");
    let mesh: semio_framework_plugin::MeshData = semio_framework_pack_json::from_json_str(&json, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the exported actual geometry");
    assert!(mesh.positions.len() >= 12);
    assert_eq!(mesh.indices.len() / 3, fixture["expected"]["faces"].as_u64().unwrap() as usize);
    assert_eq!(mesh.indices.len(), source.indices.len());
    assert!(mesh.positions.iter().all(|coordinate| coordinate.is_finite()));
    for (actual, expected) in mesh.indices.iter().zip(&source.indices) {
        assert_eq!(&mesh.positions[*actual as usize * 3..*actual as usize * 3 + 3], &source.positions[*expected as usize * 3..*expected as usize * 3 + 3]);
    }
    context::retire_instance_operation_owner(&owner);
    assert!(Generation3dPlayApp::export_media_with_request_context(&owner, "geometry:out", &doc, &transient).is_err());
    projection.retire_cold();
}

/// 📦️ Supplies the actual document geometry from the concrete Session to its IO bridge.
fn prepared_document_mesh(projection: &Generation3dSnapshot) -> semio_framework_plugin::MeshData {
    let (meshes_json, _) = preview_payload_from_evaluated_fixture(&projection.host_snapshot, &Generation3dConfig::default());
    let meshes: Vec<Value> = serde_json::from_str(&meshes_json).expect("prepared document meshes");
    let meshes = meshes.iter().map(|mesh| mesh_data_from_json(&mesh["data"])).collect::<Vec<_>>();
    merge_meshes(&meshes).expect("prepared document fixtures contain compatible surfaces")
}

#[test]
fn preview_payload_has_meshes_and_instances() {
    let _serial = test_serial();
    let projection = semio_s_artifact_procedural_3d_model::default_snapshot();
    let config = Generation3dConfig::default();
    let (meshes_json, instances_json) = preview_payload_from_evaluated_fixture(&projection.host_snapshot, &config);
    assert_ne!(meshes_json, "[]", "meshes_json was empty");
    assert_ne!(instances_json, "[]", "instances_json was empty");
    let meshes: Vec<Value> = serde_json::from_str(&meshes_json).expect("meshes json");
    let instances: Vec<Value> = serde_json::from_str(&instances_json).expect("instances json");
    assert!(!meshes.is_empty());
    assert!(!instances.is_empty());
    let kind_by_widget: std::collections::HashMap<String, String> = projection
        .host_snapshot
        .widgets
        .iter()
        .map(|widget| {
            let kind = match widget {
                semio_framework_artifact_flow_flow::Widget::Neuron { neuron_kind, .. } => neuron_kind.clone(),
                _ => String::new(),
            };
            (semio_s_artifact_procedural_3d_model::widget_id(widget).to_string(), kind)
        })
        .collect();
    let mut surfaces = 0;
    let mut curves = 0;
    for mesh in &meshes {
        let id = mesh.get("id").and_then(|value| value.as_str()).unwrap_or("");
        assert!(id.starts_with("eval-"), "mesh id must be tessellated eval handle, got {id}");
        let widget = id.trim_start_matches("eval-").split('@').next().unwrap_or_default();
        let kind = kind_by_widget.get(widget).map(String::as_str).unwrap_or_default();
        let data = mesh_data_from_json(&mesh.get("data").cloned().unwrap_or_default());
        assert!(!data.edge_positions.is_empty(), "every preview mesh carries edge geometry, {id} ({kind}) did not");
        // 🧊️ A surface-bearing channel tessellates to real triangles; a CURVE (`brep.curve.*`) and a
        // `math.vector` marker are TRIANGLE-FREE by construction — `vector_marker_mesh` is a single
        // origin→tip segment and the wire tessellation is a polyline, so both carry edge geometry
        // (and the segment's own endpoints) but never an index buffer. Demanding ≥9 positions and
        // ≥3 indices of them asserted something the geometry never had
        // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
        if kind.starts_with("brep.curve.") || kind == "math.vector" {
            assert!(data.indices.is_empty(), "curve/marker preview {id} ({kind}) must be triangle-free, got {} indices", data.indices.len());
            curves += 1;
        } else {
            assert!(data.positions.len() >= 9, "surface preview {id} ({kind}) has too few positions");
            assert!(data.indices.len() >= 3, "surface preview {id} ({kind}) has too few indices");
            surfaces += 1;
        }
    }
    assert!(surfaces > 0, "the default document previews at least one tessellated surface");
    assert!(curves > 0, "the default document previews at least one edge-only curve or vector marker");
    let camera = Camera3d {
        position: Vec3::from_array([config.preview_camera.position[0] as f32, config.preview_camera.position[1] as f32, config.preview_camera.position[2] as f32]),
        target: Vec3::from_array([config.preview_camera.target[0] as f32, config.preview_camera.target[1] as f32, config.preview_camera.target[2] as f32]),
        up: Vec3::new(0.0, 0.0, 1.0),
        fov_y: config.preview_camera.fov as f32 * std::f32::consts::PI / 180.0,
        near: 0.1,
        far: 1000.0,
        ..Camera3d::default()
    };
    let view_proj = camera.view_proj(600.0, 1000.0);
    let planes = frustum_planes(view_proj);
    let mut visible = 0usize;
    for instance in instances {
        let mesh_id = instance.get("meshId").or_else(|| instance.get("mesh_id")).and_then(|value| value.as_str()).unwrap_or("eval-missing");
        let mesh = meshes.iter().find(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(mesh_id)).expect("mesh record");
        let data = mesh_data_from_json(&mesh.get("data").cloned().unwrap_or_default());
        let (mesh_aabb_min, mesh_aabb_max) = aabb_of_positions(&data.positions);
        let position = instance.get("position").and_then(|value| value.as_array()).map_or([0.0, 0.0, 0.0], |items| [items[0].as_f64().unwrap_or(0.0) as f32, items[1].as_f64().unwrap_or(0.0) as f32, items[2].as_f64().unwrap_or(0.0) as f32]);
        assert_eq!(position, [0.0, 0.0, 0.0], "preview instances stay in world space");
        let model = Instance3d::model_from_trs(position, [0.0, 0.0, 0.0, 1.0], [1.0, 1.0, 1.0]);
        let (min, max) = transform_aabb(model, mesh_aabb_min, mesh_aabb_max);
        if aabb_intersects_frustum(&planes, min, max) {
            visible += 1;
        }
    }
    assert!(visible > 0, "no preview instances intersect camera frustum");
    projection.retire_cold();
}

#[test]
fn document_from_mesh_rejects_empty_or_invalid_geometry() {
    let _serial = test_serial();
    let mesh = semio_framework_plugin::MeshData::default();
    assert!(generation3d_document_from_mesh(&mesh).is_err());
    for (positions, indices) in [(vec![0.0; 8], vec![0, 1, 2]), (vec![0.0; 9], vec![0, 1, 9]), (vec![f32::NAN; 9], vec![0, 1, 2]), (vec![0.0; 9], vec![0, 1, 1])] {
        assert!(generation3d_document_from_mesh(&semio_framework_plugin::MeshData { positions, indices, ..Default::default() }).is_err());
    }
}

#[test]
fn mesh_import_preserves_fixture_coordinates_in_an_editable_graph() {
    let _serial = test_serial();
    let fixtures: Value = serde_json::from_str(repo_text!("🧰️framework/🔨️modules/🏗️mesh-engine/🥽️polygon/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for case in fixtures["meshes"].as_array().unwrap() {
        let source = &case["mesh"];
        if source["faces"].as_array().unwrap().iter().any(|face| face.as_array().unwrap().len() != 3) { continue; }
        let positions = source["vertices"].as_array().unwrap().iter().flat_map(|point| point.as_array().unwrap().iter().map(|coordinate| coordinate.as_f64().unwrap() as f32)).collect::<Vec<_>>();
        let indices = source["faces"].as_array().unwrap().iter().flat_map(|face| face.as_array().unwrap().iter().map(|id| id.as_u64().unwrap() as u32)).collect::<Vec<_>>();
        let mesh = semio_framework_plugin::MeshData { positions: positions.clone(), indices: indices.clone(), ..Default::default() };
        let document = generation3d_document_from_mesh(&mesh).unwrap();
        let projection = <Generation3dSnapshot as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&document)).unwrap();
        let source_fields = semio_s_artifact_procedural_3d_mesh_bridge::imported_source(&projection).map(|(kind, data)| (kind.to_owned(), data.to_owned()));
        let has_mesh_output = projection.host_snapshot.synapses.iter().any(|wire| wire.from_port == "meshOut");
        let prepared = prepared_document_mesh(&projection);
        projection.retire_cold();
        let (kind, data) = source_fields.unwrap();
        assert_eq!(kind, "brep.mesh.construct");
        let imported: Value = serde_json::from_str(&data).unwrap();
        assert_eq!(imported["faces"], source["faces"]);
        let imported_positions = imported["vertices"].as_array().unwrap().iter().flat_map(|point| point.as_array().unwrap().iter().map(|coordinate| coordinate.as_f64().unwrap() as f32)).collect::<Vec<_>>();
        assert_eq!(imported_positions, positions);
        assert!(has_mesh_output);
        let result = generation3d_mesh_from_document(&semio_framework_pack_json::to_dsl_value(&document), &prepared).unwrap();
        assert_eq!(result.indices.len(), indices.len());
        for (actual, expected) in result.indices.iter().zip(&indices) {
            assert_eq!(&result.positions[*actual as usize * 3..*actual as usize * 3 + 3], &positions[*expected as usize * 3..*expected as usize * 3 + 3]);
        }
    }
}

#[test]
fn generation3d_mesh_bridges_round_trip_through_obj_glb_stl_codecs() {
    let _serial = test_serial();
    use semio_framework_plugin::{GlbExporter, GlbImporter, MeshExporter, MeshImporter, ObjExporter, ObjImporter, StlExporter, StlImporter};
    let default_projection = semio_s_artifact_procedural_3d_model::default_snapshot();
    let document_json: Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&default_projection)).expect("projection json");
    let prepared = prepared_document_mesh(&default_projection);
    default_projection.retire_cold();
    let mesh = generation3d_mesh_from_document(&semio_framework_value::DslValue::from(&document_json), &prepared).expect("mesh from document");
    assert!(!mesh.positions.is_empty());

    let obj_bytes = ObjExporter.export(&mesh).expect("obj export");
    let obj_mesh = ObjImporter.import(&obj_bytes).expect("obj import");
    let obj_document = generation3d_document_from_mesh(&obj_mesh).expect("obj document from mesh");
    <Generation3dSnapshot as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&obj_document)).expect("parseable obj projection").retire_cold();

    let glb_bytes = GlbExporter.export(&mesh).expect("glb export");
    let glb_mesh = GlbImporter.import(&glb_bytes).expect("glb import");
    let glb_document = generation3d_document_from_mesh(&glb_mesh).expect("glb document from mesh");
    <Generation3dSnapshot as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&glb_document)).expect("parseable glb projection").retire_cold();

    let stl_bytes = StlExporter.export(&mesh).expect("stl export");
    let stl_mesh = StlImporter.import(&stl_bytes).expect("stl import");
    let stl_document = generation3d_document_from_mesh(&stl_mesh).expect("stl document from mesh");
    <Generation3dSnapshot as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&stl_document)).expect("parseable stl projection").retire_cold();
}

#[test]
fn rectangle_wire_preview_emits_edge_only_mesh() {
    let _serial = test_serial();
    let projection = <Generation3dSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(semio_s_artifact_procedural_3d_io_text::snapshot::GENERATION3D_EXAMPLE_RECTANGLE_WIRE_TEXT).expect("rectangle wire example");
    let config = Generation3dConfig::default();
    let (meshes_json, instances_json) = preview_payload_from_evaluated_fixture(&projection.host_snapshot, &config);
    let meshes: Vec<Value> = serde_json::from_str(&meshes_json).expect("meshes");
    assert!(!meshes.is_empty(), "rectangle wire preview should tessellate curve edges");
    let data = mesh_data_from_json(&meshes[0].get("data").cloned().unwrap_or_default());
    assert!(data.indices.is_empty(), "wire preview has no shaded triangles");
    assert!(data.edge_positions.len() >= 6, "curve preview should include edge polylines");
    assert!(!instances_json.is_empty());
    projection.retire_cold();
}

#[test]
fn all_bundled_examples_emit_preview_meshes() {
    let _serial = test_serial();
    let config = Generation3dConfig::default();
    let cases = [
        ("hexagonal-mushroom-column", semio_s_artifact_procedural_3d_model::PROCEDURAL_EXAMPLE_HEX_COLUMN),
        ("rectangle-extrude-volume", semio_s_artifact_procedural_3d_model::PROCEDURAL_EXAMPLE_RECT_EXTRUDE),
        ("sphere-cut-with-torus", semio_s_artifact_procedural_3d_model::PROCEDURAL_EXAMPLE_SPHERE_TORUS),
        ("box-fillet-preview", semio_s_artifact_procedural_3d_model::PROCEDURAL_EXAMPLE_BOX_FILLET),
        ("sphere-box-fuse", semio_s_artifact_procedural_3d_model::PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE),
        ("face-sweep-extrude", semio_s_artifact_procedural_3d_model::PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE),
        ("rectangle-wire-preview", semio_s_artifact_procedural_3d_model::PROCEDURAL_EXAMPLE_RECTANGLE_WIRE),
        ("mesh-workbench", semio_s_artifact_procedural_3d_model::PROCEDURAL_EXAMPLE_MESH_WORKBENCH),
        ("box-shell-preview", semio_s_artifact_procedural_3d_model::PROCEDURAL_EXAMPLE_BOX_SHELL),
    ];
    for (label, example_id) in cases {
        let projection = semio_s_artifact_procedural_3d_model::example_snapshot(example_id).unwrap_or_else(|| panic!("{label}: missing projection"));
        let (meshes_json, instances_json) = preview_payload_from_evaluated_fixture(&projection.host_snapshot, &config);
        assert_ne!(meshes_json, "[]", "{label}: meshes empty; eval may have failed");
        assert_ne!(instances_json, "[]", "{label}: instances empty");
        let meshes: Vec<Value> = serde_json::from_str(&meshes_json).unwrap_or_else(|err| panic!("{label}: meshes json: {err}"));
        assert!(!meshes.is_empty(), "{label}: no mesh entries");
        projection.retire_cold();
    }
}

#[test]
fn preview_tolerance_follows_lod_mode() {
    assert!((preview_tolerance("coarse") - 0.15).abs() < 1e-9);
    assert!((preview_tolerance("fine") - 0.02).abs() < 1e-9);
    assert!((preview_tolerance("") - 0.05).abs() < 1e-9);
}

#[test]
fn wireframe_show_mode_strips_shaded_triangles() {
    let _serial = test_serial();
    let projection = semio_s_artifact_procedural_3d_model::default_snapshot();
    let config = Generation3dConfig { show_mode: "wireframe".into(), ..Default::default() };
    let (meshes_json, _) = preview_payload_from_evaluated_fixture(&projection.host_snapshot, &config);
    let meshes: Vec<Value> = serde_json::from_str(&meshes_json).expect("meshes");
    assert!(!meshes.is_empty());
    let data = mesh_data_from_json(&meshes[0].get("data").cloned().unwrap_or_default());
    assert!(data.indices.is_empty());
    assert!(!data.edge_positions.is_empty());
    projection.retire_cold();
}

#[test]
fn generation3d_io_declares_the_params_and_geometry_ports() {
    let io = ::semio_framework_async::poll::resolve_ready(generation3d_io());
    assert_eq!(io.artifact_schema, "generation.3d");
    assert_eq!(io.artifact.id, "3d.generation");
    let params = io.ports.iter().find(|port| port.id == "params:in").expect("params:in declared");
    assert_eq!(params.direction, semio_framework_plugin::MediaPortDirection::In);
    assert!(!params.required);
    let geometry = io.ports.iter().find(|port| port.id == "geometry:out").expect("geometry:out declared");
    assert_eq!(geometry.direction, semio_framework_plugin::MediaPortDirection::Out);
    assert_eq!(geometry.kind_id.as_deref(), Some("3d.mesh"));
    assert_eq!(geometry.multiplicity, semio_framework::PortMultiplicity::Many);
}

/// 🔌️ One `preview: true` neuron with two output channels (a point channel and a vector
/// channel — neither needs a brep kernel) must yield one instance PER CHANNEL, each id
/// qualified with its own channel, not one flattened instance for the whole widget.
fn preview_widget_fixture(id: &str, output_ports: Vec<String>) -> semio_framework_artifact_flow_flow::FlowHostSnapshot {
    let widget = semio_framework_artifact_flow_flow::Widget::Neuron { id: id.into(), neuron_kind: "test.multi".into(), params: semio_framework_artifact_flow_flow::neural::Dictionary::new(), input_ports: Vec::new(), output_ports, preview: true };
    semio_framework_artifact_flow_flow::FlowHostSnapshot { schema: "flow.host_snapshot".into(), camera: semio_framework_artifact_flow_flow::CameraJson { x: 0.0, y: 0.0, zoom: 1.0 }, widgets: vec![widget], synapses: Vec::new(), layout: Default::default() }
}

#[test]
fn mesh_preview_renders_without_a_brep_conversion_in_editor_and_viewer() {
    let _serial = test_serial();
    let fixture = preview_widget_fixture("mesh", vec!["meshOut".into()]);
    let expected = semio_framework::mesh_box(1.0, 1.0, 1.0);
    let preview = semio_framework_os_flow::mesh::encode_base64(&semio_framework_os_flow::mesh::encode_mesh_pack(&expected).unwrap());
    let data = json!({"vertices": expected.positions.chunks_exact(3).collect::<Vec<_>>(), "faces": expected.indices.chunks_exact(3).collect::<Vec<_>>()}).to_string();
    let eval_json = json!({"mesh": {"out": {"meshOut": {"$schema": "mesh", "data": data, "preview": preview}}}}).to_string();
    let (meshes, instances) = preview_payload_from_eval(&eval_json, &fixture, &Generation3dConfig::default());
    let meshes: Vec<Value> = serde_json::from_str(&meshes).unwrap();
    let instances: Vec<Value> = serde_json::from_str(&instances).unwrap();
    assert_eq!(meshes.len(), 1);
    assert_eq!(meshes[0]["role"], "solid");
    assert_eq!(mesh_data_from_json(&meshes[0]["data"]), expected);
    assert_eq!(instances[0]["id"], "mesh@meshOut#0");
    let viewer = semio_s_artifact_procedural_3d_viewer_modes::view::windows::preview::preview_payload(&eval_json, &fixture, &Default::default(), None, &Default::default());
    let viewer_meshes: Vec<Value> = serde_json::from_str(&viewer.meshes_json).unwrap();
    assert_eq!(mesh_data_from_json(&viewer_meshes[0]["data"]), expected);
    fixture.retire_cold();
}

#[test]
fn preview_payload_channel_qualifies_ids_across_two_output_channels() {
    let _serial = test_serial();
    let fixture = preview_widget_fixture("multi", vec!["a".into(), "b".into()]);
    let eval_json = json!({
        "multi": {
            "out": {
                "a": { "$schema": "point", "x": 1.0, "y": 2.0, "z": 3.0 },
                "b": { "$schema": "vector", "x": 4.0, "y": 5.0, "z": 6.0 }
            }
        }
    })
    .to_string();
    let config = Generation3dConfig::default();
    let (meshes_json, instances_json) = preview_payload_from_eval(&eval_json, &fixture, &config);
    let instances: Vec<Value> = serde_json::from_str(&instances_json).expect("instances json");
    assert_eq!(instances.len(), 2, "two output channels should yield two preview instances, got {instances:?}");
    let ids: std::collections::HashSet<&str> = instances.iter().filter_map(|entry| entry.get("id").and_then(Value::as_str)).collect();
    assert!(ids.contains("multi@a#0"), "point-channel instance id missing, got {ids:?}");
    assert!(ids.contains("multi@b#0"), "vector-channel instance id missing, got {ids:?}");
    let meshes: Vec<Value> = serde_json::from_str(&meshes_json).expect("meshes json");
    assert_eq!(meshes.len(), 2, "each inline channel mints its own mesh, got {meshes:?}");
}

/// 🔌️ A single channel whose value is a `$schema: "list"` dictionary (the wire form
/// `semio_framework_artifact_flow_flow::neural::Dictionary` lists actually take) of N geometry-bearing entries must flatten
/// to N instances, indexed `#0..#{N-1}` in list order — proven here with inline points so the
/// test needs no brep kernel/session.
#[test]
fn preview_payload_flattens_a_list_channel_into_indexed_instances() {
    let _serial = test_serial();
    let fixture = preview_widget_fixture("listy", vec!["points".into()]);
    let eval_json = json!({
        "listy": {
            "out": {
                "points": {
                    "$schema": "list",
                    "0": { "$schema": "point", "x": 1.0, "y": 0.0, "z": 0.0 },
                    "1": { "$schema": "point", "x": 2.0, "y": 0.0, "z": 0.0 },
                    "2": { "$schema": "point", "x": 3.0, "y": 0.0, "z": 0.0 }
                }
            }
        }
    })
    .to_string();
    let config = Generation3dConfig::default();
    let (_meshes_json, instances_json) = preview_payload_from_eval(&eval_json, &fixture, &config);
    let instances: Vec<Value> = serde_json::from_str(&instances_json).expect("instances json");
    assert_eq!(instances.len(), 3, "a 3-entry list channel should yield 3 instances, got {instances:?}");
    for index in 0..3 {
        let expected_id = format!("listy@points#{index}");
        assert!(instances.iter().any(|entry| entry.get("id").and_then(Value::as_str) == Some(expected_id.as_str())), "missing {expected_id} in {instances:?}");
    }
}

/// 🔌️ A channel carrying only pure data (a number, no handle, no `x`/`y`/`z`) is not
/// geometry-bearing and must not fabricate a placeholder preview instance.
#[test]
fn preview_payload_emits_no_instance_for_a_pure_data_channel() {
    let _serial = test_serial();
    let fixture = preview_widget_fixture("scalar", vec!["value".into()]);
    let eval_json = json!({
        "scalar": {
            "out": {
                "value": { "$schema": "number", "value": 42.0 }
            }
        }
    })
    .to_string();
    let config = Generation3dConfig::default();
    let (meshes_json, instances_json) = preview_payload_from_eval(&eval_json, &fixture, &config);
    assert_eq!(meshes_json, "[]", "pure-data channel must not fabricate mesh geometry");
    assert_eq!(instances_json, "[]", "pure-data channel must not fabricate a preview instance");
}
/// 🕹️ The three id forms one mark can take, and the transitive reach of each: a node-level
/// mark covers every channel and every instance below it, a channel-level mark covers only its
/// own channel, and an instance-level mark covers only itself.
#[test]
fn preview_marks_resolve_node_channel_and_instance_ids() {
    let node = PreviewInteractionMarks { source_revision: None, components: Default::default(), hovered: ["multi".to_string()].into_iter().collect(), selected: Default::default() };
    assert!(node.hovers("multi", "a", 0) && node.hovers("multi", "b", 3));
    assert!(!node.hovers("other", "a", 0));

    let channel = PreviewInteractionMarks { source_revision: None, components: Default::default(), hovered: ["multi@b".to_string()].into_iter().collect(), selected: Default::default() };
    assert!(channel.hovers("multi", "b", 0) && channel.hovers("multi", "b", 7));
    assert!(!channel.hovers("multi", "a", 0));

    let instance = PreviewInteractionMarks { source_revision: None, components: Default::default(), hovered: ["multi@b#2".to_string()].into_iter().collect(), selected: Default::default() };
    assert!(instance.hovers("multi", "b", 2));
    assert!(!instance.hovers("multi", "b", 1));
}

/// 🕹️ Graph → world: hovering the NODE in the node graph must light up every one of its
/// channels' preview geometry, not just one.
#[test]
fn preview_payload_marks_every_channel_of_a_hovered_node() {
    let _serial = test_serial();
    let fixture = preview_widget_fixture("multi", vec!["a".into(), "b".into()]);
    let eval_json = json!({ "multi": { "out": {
            "a": { "$schema": "point", "x": 1.0, "y": 2.0, "z": 3.0 },
            "b": { "$schema": "vector", "x": 4.0, "y": 5.0, "z": 6.0 }
        } } })
    .to_string();
    let marks = PreviewInteractionMarks { source_revision: None, components: Default::default(), hovered: ["multi".to_string()].into_iter().collect(), selected: ["multi@a".to_string()].into_iter().collect() };
    let payload = preview_payload(&eval_json, &fixture, &Generation3dConfig::default(), None, &marks);
    let instances: Vec<Value> = serde_json::from_str(&payload.instances_json).expect("instances json");
    assert_eq!(instances.len(), 2);
    assert!(instances.iter().all(|entry| entry.get("hovered").and_then(Value::as_bool) == Some(true)), "node hover must reach every channel: {instances:?}");
    assert_eq!(payload.selected_ids, vec!["multi@a#0".to_string()], "channel-level selection must not spill onto the sibling channel");
    assert!(payload.hovered_id.is_some(), "the scene needs a concrete hovered instance to paint");
}

/// 🕹️ Graph → world, narrowed: hovering one PORT lights up only that channel's geometry.
#[test]
fn preview_payload_marks_only_the_hovered_channel() {
    let _serial = test_serial();
    let fixture = preview_widget_fixture("multi", vec!["a".into(), "b".into()]);
    let eval_json = json!({ "multi": { "out": {
            "a": { "$schema": "point", "x": 1.0, "y": 2.0, "z": 3.0 },
            "b": { "$schema": "vector", "x": 4.0, "y": 5.0, "z": 6.0 }
        } } })
    .to_string();
    let marks = PreviewInteractionMarks { source_revision: None, components: Default::default(), hovered: ["multi@b".to_string()].into_iter().collect(), selected: Default::default() };
    let payload = preview_payload(&eval_json, &fixture, &Generation3dConfig::default(), None, &marks);
    let instances: Vec<Value> = serde_json::from_str(&payload.instances_json).expect("instances json");
    let hovered: Vec<&str> = instances.iter().filter(|entry| entry.get("hovered").and_then(Value::as_bool) == Some(true)).filter_map(|entry| entry.get("id").and_then(Value::as_str)).collect();
    assert_eq!(hovered, vec!["multi@b#0"], "only the hovered channel may light up: {instances:?}");
    assert_eq!(payload.hovered_id.as_deref(), Some("multi@b#0"));
}

/// 🕹️ World → graph: hovering one preview INSTANCE in the 3D world resolves back to its node
/// and its port, which is what the node-graph window paints.
#[test]
fn graph_marks_project_instance_hover_back_onto_its_node_and_port() {
    let marks = PreviewInteractionMarks { source_revision: None, components: Default::default(), hovered: ["multi@b#0".to_string()].into_iter().collect(), selected: ["multi@a#1".to_string()].into_iter().collect() };
    assert_eq!(marks.hovered_graph_target(), Some(("multi".to_string(), Some("b".to_string()))));
    assert!(marks.graph_highlight_ids().contains(&"multi".to_string()));
    assert_eq!(marks.graph_selection_ids(), vec!["multi".to_string()]);
    assert_eq!(PreviewInteractionMarks::widget_of("multi@b#0"), "multi");
    assert_eq!(PreviewInteractionMarks::port_of("multi@b#0"), Some("b"));
    assert_eq!(PreviewInteractionMarks::port_of("multi"), None);
}

/// 🕸️ Every port the node graph paints is also an interaction target parented to its widget —
/// the topology link `HoverSpec { transitive: true }` walks.
#[test]
fn interaction_topology_ports_match_the_node_graph_port_ids() {
    let _serial = test_serial();
    let projection = semio_s_artifact_procedural_3d_model::default_snapshot();
    let ports_by_node = generation3d_port_ids_by_node(&projection.host_snapshot);
    assert!(!ports_by_node.is_empty(), "default fixture should project graph nodes");
    assert!(ports_by_node.values().any(|ports| !ports.is_empty()), "default fixture should project at least one port");
    for (node_id, ports) in &ports_by_node {
        for port in ports {
            assert!(port.starts_with(&format!("{node_id}@")), "port {port} must be qualified by its node {node_id}");
            assert_eq!(PreviewInteractionMarks::widget_of(port), node_id.as_str());
        }
    }
    projection.retire_cold();
}
/// 👁️ Which widget kinds contribute preview geometry: a neuron only when its author-set toggle
/// is on, an output preview always, a cluster always (it has no toggle of its own, and its
/// inner `semio_framework_artifact_flow_flow::neural::Neuron`s have none either), and a pure input widget never.
#[test]
fn widget_preview_eligibility_covers_neurons_output_previews_and_clusters() {
    let on = semio_framework_artifact_flow_flow::Widget::Neuron { id: "n".into(), neuron_kind: "k".into(), params: semio_framework_artifact_flow_flow::neural::Dictionary::new(), input_ports: Vec::new(), output_ports: Vec::new(), preview: true };
    let off = semio_framework_artifact_flow_flow::Widget::Neuron { id: "n".into(), neuron_kind: "k".into(), params: semio_framework_artifact_flow_flow::neural::Dictionary::new(), input_ports: Vec::new(), output_ports: Vec::new(), preview: false };
    let output = semio_framework_artifact_flow_flow::Widget::OutputPreview { id: "p".into(), preview: Default::default(), expanded: Default::default() };
    let cluster = semio_framework_artifact_flow_flow::Widget::Cluster { id: "c".into(), name: "Cluster".into(), tree: Default::default(), flow: Default::default() };
    let slider = semio_framework_artifact_flow_flow::Widget::InputSlider { id: "s".into(), label: "S".into(), value: 0.0, min: 0.0, max: 1.0, step: 0.1 };
    assert!(widget_previews(&on));
    assert!(!widget_previews(&off));
    assert!(widget_previews(&output));
    assert!(widget_previews(&cluster));
    assert!(!widget_previews(&slider));
}
//#endregion 🔖️EngineComputeTests
