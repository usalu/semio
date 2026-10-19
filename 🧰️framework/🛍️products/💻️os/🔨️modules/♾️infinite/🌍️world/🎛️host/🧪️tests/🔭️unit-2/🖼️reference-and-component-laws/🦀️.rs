/// 🖼️ Laws of the reference visual, the world grid, native analytic component picks and face selection overlays.

/// 📷️ An authored preview camera keeps its exact pose while its full projection spec still drives
/// matrix family/FOV. A later local projection command acquires the spec without framing content.
#[test]
fn delivered_preserve_camera_policy_cancels_frame_debt_until_local_projection_selection() {
    let fixture = camera_framing_fixture();
    let projection_fixture: serde_json::Value = serde_json::from_str(read_static("../../../../../../../../🔨️modules/🖱️ui/🪟️viewport/🧪️tests/🧫️fixtures/📐️projection/🔣️.json")).unwrap();
    assert!(projection_fixture["framePolicies"].as_array().unwrap().iter().any(|row| row["value"] == "preserveCamera" && row["expectedFramesContent"] == false));
    let viewport = Rect::new(0.0, 0.0, fixture["viewport"][0].as_f64().unwrap() as f32, fixture["viewport"][1].as_f64().unwrap() as f32);
    let spec = semio_framework_ui_viewport::Viewport3dProjectionSpec {
        mode: semio_framework_ui_viewport::Viewport3dProjectionMode::Orthographic {},
        orientation: semio_framework_ui_viewport::Viewport3dProjectionOrientation::Free {},
    };
    let mut scene = camera_framing_scene(&fixture, "perspective", true, None);
    scene.world_3d.as_mut().expect("world scene").camera_json = serde_json::json!({
        "position": [40, -30, 20],
        "target": [123, 456, 789],
        "up": [0, 0, 1],
        "fov": 50,
        "zoom": 7,
        "projection": spec,
        "projectionFrame": "preserveCamera",
    })
    .to_string();
    let mut state = World3dState::new("surface-preserved-camera".into(), "controller".into());

    drive_scene_bridge(&mut state, &scene, viewport);
    assert_eq!(state.camera.projection_frame_policy, semio_framework_ui_viewport::Viewport3dProjectionFramePolicy::PreserveCamera);
    assert!(!state.camera.projection_frame_owed);
    let accepted = state.camera.orbit.clone();
    sync_world3d_projection_content_frame(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut state.bridge);
    assert_eq!(state.camera.orbit.target.to_array(), accepted.target.to_array());
    assert_eq!(state.camera.orbit.to_camera().position.to_array(), accepted.to_camera().position.to_array());
    assert_eq!(state.camera.orbit.zoom, accepted.zoom);

    let local = semio_framework_ui_viewport::Viewport3dProjectionSpec {
        mode: semio_framework_ui_viewport::Viewport3dProjectionMode::ThreePoint { fov: 50.0 },
        orientation: semio_framework_ui_viewport::Viewport3dProjectionOrientation::Free {},
    };
    assert!(apply_world3d_projection_spec(&mut state.session, &mut state.camera, &mut state.scene, local));
    assert_eq!(state.camera.projection_frame_policy, semio_framework_ui_viewport::Viewport3dProjectionFramePolicy::Content);
    assert!(!state.camera.projection_frame_owed, "a local projection selection retains the accepted authored pose");
    sync_world3d_projection_content_frame(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut state.bridge);
    assert!(!state.camera.projection_frame_owed);
    assert_eq!(state.camera.orbit.target.to_array(), accepted.target.to_array());
    assert_eq!(state.camera.orbit.to_camera().position.to_array(), accepted.to_camera().position.to_array());
}

/// 🖼️ LAW: reference identity, geometry, and visual state come from the shared Three fixture.
#[test]
fn reference_visual_identity_geometry_and_interaction_match_three() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../🧪️tests/🧫️fixtures/🖼️reference-visual/🔣️.json")).expect("reference visual fixture");
    let identity = fixture["identityCases"].as_array().expect("identity cases");
    let shared_url = identity[0]["url"].as_str().expect("shared URL");
    assert_eq!(identity[1]["url"].as_str(), Some(shared_url));

    let geometry = &fixture["geometry"];
    let default_width = geometry["defaultWidthWorld"].as_f64().unwrap() as f32;
    assert_eq!(world_reference_width(&WorldReferenceRecord::default()), default_width, "the neutral fixture and WGPU use React's missing-width authority");
    let source = geometry["sourceNaturalSize"].as_array().expect("source natural size");
    let mut state = World3dState::new("surface".into(), "controller".into());
    let _ = state.scene.reference_pixels.insert(shared_url.into(), test_world_scene_raster(source[0].as_u64().unwrap() as u32, source[1].as_u64().unwrap() as u32, 0));
    for (index, row) in identity.iter().enumerate() {
        state.scene.references.push(WorldReferenceRecord {
            id: Some(row["id"].as_str().unwrap().into()),
            url: Some(shared_url.into()),
            origin: Some([-2.0 + index as f64 * 4.0, 0.0, 0.0]),
            width_world: Some(2.0),
            ..Default::default()
        });
    }
    state.scene.references.push(WorldReferenceRecord { id: Some("locked".into()), url: Some(shared_url.into()), locked: Some(true), ..Default::default() });
    state.session.interaction_revision = 11;
    let mut registry = WorldInteractionRegistryBuildCursor::new(11);
    for _ in 0..16 {
        if with_world_step_context(1, |context| registry.step(&mut state.session, &mut state.interaction, &mut state.scene, context)) == WorldInteractionStep::Complete {
            break;
        }
    }
    let reference_ids: Vec<&str> = state.interaction
        .interaction_objects
        .slots
        .iter()
        .flatten()
        .filter(|entry| entry.revision == 11 && entry.kind == WorldInteractionObjectKind::Reference)
        .map(|entry| entry.id.as_str())
        .collect();
    assert_eq!(reference_ids.len(), 2, "two authored ids sharing one URL remain two interaction objects; locked stays nonselectable");
    for row in identity {
        assert!(reference_ids.contains(&row["expectedInteractionId"].as_str().unwrap()));
    }

    let width = geometry["widthWorld"].as_f64().unwrap() as f32;
    let [actual_width, actual_height] = world_reference_plane_size(&state.scene, shared_url, width);
    let expected_size = geometry["expectedPlaneSize"].as_array().unwrap();
    assert!((actual_width - expected_size[0].as_f64().unwrap() as f32).abs() < 1e-5);
    assert!((actual_height - expected_size[1].as_f64().unwrap() as f32).abs() < 1e-5);
    let origin: [f32; 3] = geometry["origin"].as_array().unwrap().iter().map(|value| value.as_f64().unwrap() as f32).collect::<Vec<_>>().try_into().unwrap();
    let model = Instance3d::model_from_trs(origin, [0.0, 0.0, 0.0, 1.0], [actual_width, actual_height, 1.0]);
    for (index, local) in [Vec3::new(-0.5, -0.5, 0.0), Vec3::new(0.5, -0.5, 0.0), Vec3::new(0.5, 0.5, 0.0), Vec3::new(-0.5, 0.5, 0.0)].into_iter().enumerate() {
        let actual = model.transform_point(local).to_array();
        let expected = geometry["expectedCorners"][index].as_array().unwrap();
        for axis in 0..3 {
            assert!((actual[axis] - expected[axis].as_f64().unwrap() as f32).abs() < 1e-5, "corner {index} axis {axis}");
        }
    }

    state.interaction.interaction_objects.revision = state.session.interaction_revision;
    let mut select = WorldObjectPickCursor::from_ray(state.session.interaction_revision, 12, WorldObjectPickPurpose::ReferenceSelect, Vec3::new(-2.0, 0.0, 2.0), Vec3::new(0.0, 0.0, -1.0));
    select.merge = world_merge_code(false, false, false);
    for _ in 0..=WORLD_INTERACTION_OBJECT_CAPACITY {
        if with_world_step_context(1, |context| select.step(&state.session, &state.interaction, 12, context)) == WorldInteractionStep::Complete {
            break;
        }
    }
    let mut plan = select.finish_plan(&state.session, &state.selection, &state.interaction, 12).expect("reference select plan").expect("reference hit");
    let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
    assert_eq!(with_world_step_context(1, |context| publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut plan, 12, &mut input, context)).unwrap(), WorldInteractionStep::Pending);
    let actions = take_actions(&mut input);
    assert_eq!(actions.len(), 1);
    let targets = actions[0].args.as_ref().and_then(|args| args.get("targets")).and_then(|value| value.as_str()).and_then(|raw| serde_json::from_str::<Vec<serde_json::Value>>(raw).ok()).expect("reference target JSON");
    assert_eq!(targets[0]["id"], format!("{}/{}", state.session.surface_id, identity[0]["expectedInteractionId"].as_str().unwrap()));
    assert_eq!(targets[0]["granularity"], "reference");
}

#[test]
fn reference_visual_geometry_reaches_the_actual_textured_scene_pass() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../🧪️tests/🧫️fixtures/🖼️reference-visual/🔣️.json")).unwrap();
    let geometry = &fixture["geometry"];
    let url = fixture["identityCases"][0]["url"].as_str().unwrap();
    let mut scene = scene_with_selection("{}");
    scene.world_3d.as_mut().unwrap().references_json = Some(serde_json::json!([{
        "id": "actual-reference-plane", "url": url, "origin": geometry["origin"],
        "widthWorld": geometry["widthWorld"], "locked": true, "hidden": false
    }]).to_string());
    let bounds = Rect::new(0.0, 0.0, 320.0, 240.0);
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    drive_scene_bridge(&mut state, &scene, bounds);
    let size = &geometry["sourceNaturalSize"];
    let _ = state.scene.reference_pixels.insert(url.into(), test_world_scene_raster(size[0].as_u64().unwrap() as u32, size[1].as_u64().unwrap() as u32, 0));
    let mut gpu = World3dBuildContext::new(WorldCursorWakeAuthority::new());
    let mut draw = semio_framework_ui_draw_contract::DrawList::default();
    let mut atlas = semio_framework_ui_text::FontAtlas::builtin();
    let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
    let theme = semio_framework_ui_theme::Theme::default();
    let mut scroll = HashMap::new();
    let mut collapsed = HashMap::new();
    let mut selects = HashMap::new();
    let mut ctx = semio_framework_ui_wgpu_widget_kit::WidgetContext {
        draw: &mut draw, overlay: None, atlas: &mut atlas, icons: None, input: &mut input, theme: &theme,
        scroll_offsets: &mut scroll, collapsed_sections: &mut collapsed, open_selects: &mut selects,
        interaction_maps: None, pick_clip: None, viewport_height: bounds.h,
    };
    render_world_3d(&scene, bounds, &mut ctx, &mut state, &mut gpu, World3dShadowProfile::World);
    let pass = draw.scene_passes.last().expect("real World3d scene pass");
    assert_eq!(pass.textured_draws.len(), 1);
    assert_eq!(pass.textured_draws[0].instances.len(), 1);
    let actual = &pass.textured_draws[0].instances[0];
    assert_eq!(actual.texture_key, url);
    for (index, local) in [Vec3::new(-0.5, -0.5, 0.0), Vec3::new(0.5, -0.5, 0.0), Vec3::new(0.5, 0.5, 0.0), Vec3::new(-0.5, 0.5, 0.0)].into_iter().enumerate() {
        let point = actual.model.transform_point(local).to_array();
        for axis in 0..3 {
            assert!((point[axis] - geometry["expectedCorners"][index][axis].as_f64().unwrap() as f32).abs() < 1e-5, "emitted corner {index} axis {axis}");
        }
    }
    retire_bridged_surface(&mut state);
}

/// 🌐️ LAW: the actual World producer publishes one retained procedural grid scalar and no
/// finite grid line vertices. The later overlay lanes remain ordinary line geometry.
#[test]
fn an_actual_world_grid_is_one_camera_projected_procedural_scalar_without_grid_lines() {
    let mut scene = scene_with_selection("{}");
    let world = scene.world_3d.as_mut().unwrap();world.meshes_json = "[]".into();world.instances_json = "[]".into();
    let bounds = Rect::new(0.0, 0.0, 320.0, 240.0);
    let mut state = World3dState::new("surface-grid".into(), "controller-grid".into());
    state.camera.orbit.target = Vec3::new(7.0, 11.0, 0.0);
    state.camera.orbit.distance = 59.0;
    state.camera.orbit.yaw = -0.63;
    state.camera.orbit.pitch = 0.71;
    state.scene.lod.show_grid = true;
    state.scene.lod.grid_datum = Some([3.0, 5.0, 2.0]);
    drive_scene_bridge(&mut state, &scene, bounds);
    let mut gpu = World3dBuildContext::new(WorldCursorWakeAuthority::new());
    let mut draw = semio_framework_ui_draw_contract::DrawList::default();
    let mut atlas = semio_framework_ui_text::FontAtlas::builtin();
    let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
    let theme = semio_framework_ui_theme::Theme::default();
    let mut scroll = HashMap::new();
    let mut collapsed = HashMap::new();
    let mut selects = HashMap::new();
    let mut ctx = semio_framework_ui_wgpu_widget_kit::WidgetContext {
        draw: &mut draw, overlay: None, atlas: &mut atlas, icons: None, input: &mut input, theme: &theme,
        scroll_offsets: &mut scroll, collapsed_sections: &mut collapsed, open_selects: &mut selects,
        interaction_maps: None, pick_clip: None, viewport_height: bounds.h,
    };

    let expected_camera = state.camera.orbit.to_camera();
    let expected_step = lod_grid_step_world(scene_lod(&state.session, &state.camera, &state.scene), state.scene.lod.grid_factor).expect("positive retained LOD step");
    let expected_fade = camera_grid_fade_distance(&expected_camera, 2.0, expected_step, bounds.w, bounds.h);
    render_world_3d(&scene, bounds, &mut ctx, &mut state, &mut gpu, World3dShadowProfile::World);
    let pass = draw.scene_passes.last().expect("actual World3d scene pass");
    let grid = pass.procedural_grid.expect("one actual retained procedural grid");
    assert!((grid.plane_z - 2.001).abs() < 1e-6, "only the rendered plane receives the datum offset");
    assert_eq!(grid.camera_plane_projection, [expected_camera.position.x, expected_camera.position.y, 2.001], "the infinite plane follows camera position rather than the orbit target");
    assert!((grid.cell_size - expected_step as f32).abs() < f32::EPSILON, "the retained cell scalar owns the actual LOD step");
    assert!((grid.fade_distance - expected_fade).abs() < f32::EPSILON, "the fade helper receives the unshifted datum and logical viewport");
    assert_eq!(grid.cell_color, [theme.text_element.r, theme.text_element.g, theme.text_element.b], "the retained grid owns the linear element token");
    let retained_line_vertices = pass.line_draws.iter().map(|draw| draw.vertices.len()).sum::<usize>();
    assert_eq!(retained_line_vertices, 0, "the grid is one procedural scalar, never a camera-target-anchored LineList");
    retire_bridged_surface(&mut state);
}

#[test]
fn isolated_world_presentation_suppresses_chrome_and_input_but_retains_frame_work() {
    let mut scene = scene_with_selection("{}");
    scene.world_3d.as_mut().expect("world fixture").presentation_json = Some(
        serde_json::to_string(&World3dPresentation {
            show_grid: false,
            show_gizmo: false,
            interactive: false,
            viewport_mask: SceneViewportMask3d::Ellipse,
            clear: World3dPresentationClear::Transparent,
            source_aspect: Some(2.0),
        })
        .expect("typed isolated presentation"),
    );
    let bounds = Rect::new(0.0, 0.0, 396.0, 196.0);
    let mut state = World3dState::new("surface-icon".into(), "controller-icon".into());
    drive_scene_bridge(&mut state, &scene, bounds);
    state.scene.retired_draws = Some(WorldDrawRegistry::default());
    let mut gpu = World3dBuildContext::new(WorldCursorWakeAuthority::new());
    let mut draw = semio_framework_ui_draw_contract::DrawList::default();
    let mut atlas = semio_framework_ui_text::FontAtlas::builtin();
    let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
    let theme = semio_framework_ui_theme::Theme::default();
    let mut scroll = HashMap::new();
    let mut collapsed = HashMap::new();
    let mut selects = HashMap::new();
    let mut ctx = semio_framework_ui_wgpu_widget_kit::WidgetContext {
        draw: &mut draw, overlay: None, atlas: &mut atlas, icons: None, input: &mut input, theme: &theme,
        scroll_offsets: &mut scroll, collapsed_sections: &mut collapsed, open_selects: &mut selects,
        interaction_maps: None, pick_clip: None, viewport_height: bounds.h,
    };
    render_world_3d(&scene, bounds, &mut ctx, &mut state, &mut gpu, World3dShadowProfile::IconPng);
    let pass = draw.scene_passes.last().expect("isolated world scene pass");
    assert_eq!(pass.viewport_mask, SceneViewportMask3d::Ellipse);
    assert_eq!(pass.clear_color, None, "transparent presentation leaves the accepted card in place");
    assert!(pass.procedural_grid.is_none());
    let camera = state.camera.orbit.to_camera();
    let expected = ui_scene::math::projection_spec_view_proj(&camera, state.camera.projection_spec, bounds.w, bounds.w / 2.0).to_cols_array();
    assert_eq!(pass.view_proj, expected, "authored 2:1 source aspect survives the 396×196 physical stretch");
    assert!(input.staged_hits().is_empty(), "noninteractive worlds publish no pointer owner");
    assert_eq!(draw.layers.iter().map(|layer| layer.ui_instances.len() + layer.overlay_ui_instances.len()).sum::<usize>(), 0, "noninteractive isolated worlds paint no gizmo or fallback clear quad");
    assert!(gpu.take_cursor_wake().expect("wake authority remains open").is_some(), "retained scene work still wakes an isolated world");
    state.scene.retired_draws = None;
    retire_bridged_surface(&mut state);
}

/// 🎨️ LAW: the retained Basic-material appearance follows authored opacity and live theme tokens.
#[test]
fn reference_visual_state_uses_authored_opacity_and_live_theme() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../🧪️tests/🧫️fixtures/🖼️reference-visual/🔣️.json")).expect("reference visual fixture");
    for row in fixture["stateCases"].as_array().unwrap() {
        if row["sourceProfileId"] != "world-image" || row["hidden"].as_bool().unwrap() {
            continue;
        }
        let theme = if row["themeId"] == "semio-dark" { semio_framework_ui_theme::Theme::dark() } else { semio_framework_ui_theme::Theme::light() };
        let id = "reference-authored-a";
        let reference = WorldReferenceRecord {
            id: Some(id.into()),
            url: Some("shared.png".into()),
            opacity: row["baseOpacity"].as_f64(),
            locked: row["locked"].as_bool(),
            hidden: row["hidden"].as_bool(),
            ..Default::default()
        };
        let mut state = World3dState::new("surface".into(), "controller".into());
        if row["selected"].as_bool().unwrap() {
            state.selection.selected_ids.push(id.into());
        }
        if row["hovered"].as_bool().unwrap() {
            state.selection.local_hover_id = Some(format!("reference:{id}"));
        }
        let actual = world_reference_visual(&state.selection, &reference, &theme);
        let expected = &row["expected"];
        assert!((actual.content_opacity - expected["contentOpacity"].as_f64().unwrap() as f32).abs() < 1e-6, "{} content opacity", row["id"]);
        let expected_background = match expected["backgroundSemantic"].as_str() {
            Some("activeBase") => [theme.selected.r, theme.selected.g, theme.selected.b, theme.selected.a],
            Some("hoverBase") => [theme.row_hover.r, theme.row_hover.g, theme.row_hover.b, theme.row_hover.a],
            _ => [0.0; 4],
        };
        assert_eq!(actual.background, expected_background, "{} background", row["id"]);
        let expected_outline = match expected["outlineSemantic"].as_str() {
            Some("activeBase") => Some([theme.celebrate[0].r, theme.celebrate[0].g, theme.celebrate[0].b, 1.0]),
            Some("accentSecondary") => Some([theme.celebrate[1].r, theme.celebrate[1].g, theme.celebrate[1].b, 0.9]),
            _ => None,
        };
        assert_eq!(actual.outline, expected_outline, "{} outline", row["id"]);
    }
}

/// 🎯️ Native picks and marquee retain exact analytic labels through the original scene bridge.
#[test]
fn world_native_analytic_component_picks_and_marquee_preserve_original_source() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../🧪️tests/🧫️fixtures/🎯️analytic-component-picking/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let expected = row["expected"].as_str().unwrap();
        let oracle = format!("{}.face.{}~{}~{}~{}", row["instance"].as_str().unwrap(), row["group"].as_u64().unwrap(), row["handle"].as_str().unwrap(), row["label"].as_str().unwrap(), row["revision"].as_str().unwrap());
        assert_eq!(expected, oracle);
        assert!(row["label"].as_str().unwrap().parse::<u64>().unwrap() > 9_007_199_254_740_992);
        let mut mesh = fixture["mesh"].clone();
        mesh["componentReferences"] = serde_json::json!({"face":[row["label"]]});
        let mut scene = scene_with_selection_and_domain("{}", Some(("geometry", "object")));
        let world = scene.world_3d.as_mut().unwrap();
        world.meshes_json = serde_json::json!([{"id":"analytic","data":mesh}]).to_string();
        world.instances_json = serde_json::json!([{"id":row["instance"],"meshId":"analytic","componentSource":{"handle":row["handle"],"revision":row["revision"]}}]).to_string();
        world.camera_json = fixture["camera"].to_string();
        let mut state = World3dState::new("surface".into(), "controller".into());
        drive_scene_bridge(&mut state, &scene, Rect::new(0.0, 0.0, 400.0, 400.0));
        state.selection.granularity = "face".into();
        state.selection.active_object_id = Some(row["instance"].as_str().unwrap().into());
        let mut registry = WorldInteractionRegistryBuildCursor::new(state.session.interaction_revision);
        for turn in 0..128 {
            match with_world_step_context(1, |context| registry.step(&mut state.session, &mut state.interaction, &mut state.scene, context)) {
                WorldInteractionStep::Complete => break,
                WorldInteractionStep::Pending => assert!(turn < 127),
                step => panic!("analytic registry {step:?}"),
            }
        }
        let mut cursor = WorldComponentPickCursor::new(&state.session, &state.camera, &state.selection, 10, WorldComponentPickPurpose::Select, 200.0, 200.0).unwrap();
        for turn in 0..WORLD_INTERACTION_OBJECT_CAPACITY + 8 {
            match with_world_step_context(1, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 10, context)) {
                WorldInteractionStep::Complete => break,
                WorldInteractionStep::Pending => assert!(turn < WORLD_INTERACTION_OBJECT_CAPACITY + 7),
                step => panic!("analytic pick {step:?}"),
            }
        }
        assert_eq!(cursor.best.map(|hit| hit.id), Some(0));
        let mut plan = cursor.finish_plan(&state.session, &state.selection, &state.interaction, &state.scene, 10).unwrap().unwrap();
        let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
        for turn in 0..128 {
            if with_world_step_context(1, |context| publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut plan, 10, &mut input, context)).unwrap() == WorldInteractionStep::Complete { break; }
            assert!(turn < 127);
        }
        let actions = take_actions(&mut input);
        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0].action, "interactionSelect");
        let args = actions[0].args.as_ref().unwrap();
        assert_eq!(args.get("domainId").and_then(|value| value.as_str()), Some("geometry"));
        let targets: serde_json::Value = serde_json::from_str(args.get("targets").and_then(|value| value.as_str()).unwrap()).unwrap();
        assert_eq!(targets[0]["id"].as_str(), Some(expected));
        let mut gesture = WorldMarqueeGesture::new(state.session.interaction_revision, 11, [0.0, 0.0]);
        assert!(gesture.push([400.0, 400.0]));
        let mut marquee = WorldMarqueePickCursor::new(&state.session, &state.camera, &state.selection, 12, gesture).unwrap();
        for turn in 0..1024 {
            match with_world_step_context(1, |context| marquee.step(&state.session, &state.selection, &state.interaction, &state.scene, 12, context)) {
                WorldInteractionStep::Complete => break,
                WorldInteractionStep::Pending => assert!(turn < 1023),
                step => panic!("analytic marquee {step:?}"),
            }
        }
        assert_eq!(marquee.results.lens[0], 1);
        let mut publish = WorldComponentMarqueePublishJob::new(12, marquee.gesture, marquee.results, WorldComponentKind::Face, false, false);
        for turn in 0..1024 {
            if with_world_step_context(1, |context| publish.step(&state.session, &state.interaction, &state.scene, 12, &mut input, context)).unwrap() == WorldInteractionStep::Complete { break; }
            assert!(turn < 1023);
        }
        let actions = take_actions(&mut input);
        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0].action, "interactionSelect");
        let targets: serde_json::Value = serde_json::from_str(actions[0].args.as_ref().unwrap().get("targets").and_then(|value| value.as_str()).unwrap()).unwrap();
        assert_eq!(targets[0]["id"].as_str(), Some(expected));
    }
}

/// 🖱️ Optimistic preview reads the original encoded target text and exact analytic source.
#[test]
fn world_component_contract_preview_reads_exact_target_text() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../🧪️tests/🧫️fixtures/🎯️analytic-component-picking/🔣️.json")).unwrap();
    let row = &fixture["cases"][0];
    let mut mesh = fixture["mesh"].clone(); mesh["componentReferences"] = serde_json::json!({"face":[row["label"]]});
    let mut scene = scene_with_selection_and_domain("{}", Some(("geometry", "object")));
    let world = scene.world_3d.as_mut().unwrap();
    world.meshes_json = serde_json::json!([{"id":"analytic","data":mesh}]).to_string();
    world.instances_json = serde_json::json!([{"id":row["instance"],"meshId":"analytic","componentSource":{"handle":row["handle"],"revision":row["revision"]}}]).to_string();
    let mut state = World3dState::new("surface".into(), "controller".into());
    drive_scene_bridge(&mut state, &scene, Rect::new(0.0, 0.0, 400.0, 400.0));
    let targets = serde_json::json!([{"granularity":"face","id":row["expected"]}]).to_string();
    let action = ActionDescriptor { controller_id: "controller".into(), action: "interactionSelect".into(), args: Some(semio_framework::dsl_value!({"domainId":"geometry","targets":targets,"merge":"replace","method":"pick"})) };
    apply_world_action_preview(&mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene, &action);
    assert_eq!(state.selection.component_ids, vec!["0".to_string()]);
    assert_eq!(state.selection.granularity, "face");
    let mut action = action;
    action.action = "interactionHover".into();
    apply_world_action_preview(&mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene, &action);
    assert_eq!(state.selection.hovered_component_id.as_deref(), Some("0"));
    assert_eq!(state.selection.hovered_component_object_id.as_deref(), row["instance"].as_str());
    assert_eq!(state.selection.hovered_component_mode.as_deref(), Some("face"));
}

/// 📍️ Native vertex picking skips surface samples and resolves the original analytic point label.
#[test]
fn world_native_analytic_vertex_pick_skips_surface_samples() {
    assert_native_original_vertex_selection(false);
}

/// 🎨️ Authored face and corner attributes retain unindexed original vertices through expansion.
#[test]
fn world_native_analytic_vertex_pick_survives_authored_expansion() {
    assert_native_original_vertex_selection(true);
}

fn assert_native_original_vertex_selection(expanded: bool) {
    let mut fixture: serde_json::Value = serde_json::from_str(read_static("../../../🧪️tests/🧫️fixtures/🎯️analytic-vertex-picking/🔣️.json")).unwrap();
    if expanded { fixture["mesh"]["attributes"] = fixture["expandedAttributes"].clone(); }
    let mut scene = scene_with_selection_and_domain("{}", Some(("geometry", "object")));
    let world = scene.world_3d.as_mut().unwrap();
    world.meshes_json = serde_json::json!([{"id":"analytic","data":fixture["mesh"]}]).to_string();
    world.instances_json = serde_json::json!([{"id":fixture["instance"],"meshId":"analytic","componentSource":{"handle":fixture["handle"],"revision":fixture["revision"]}}]).to_string();
    world.camera_json = fixture["camera"].to_string();
    let mut state = World3dState::new("surface".into(), "controller".into());
    drive_scene_bridge(&mut state, &scene, Rect::new(0.0, 0.0, 400.0, 400.0));
    state.selection.granularity = "vertex".into();
    let mut registry = WorldInteractionRegistryBuildCursor::new(state.session.interaction_revision);
    for turn in 0..128 {
        match with_world_step_context(1, |context| registry.step(&mut state.session, &mut state.interaction, &mut state.scene, context)) {
            WorldInteractionStep::Complete => break,
            WorldInteractionStep::Pending => assert!(turn < 127),
            step => panic!("original vertex registry {step:?}"),
        }
    }
    let mut cursor = WorldComponentPickCursor::new(&state.session, &state.camera, &state.selection, 10, WorldComponentPickPurpose::Select, 200.0, 200.0).unwrap();
    for turn in 0..2048 {
        match with_world_step_context(1, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 10, context)) {
            WorldInteractionStep::Complete => break,
            WorldInteractionStep::Pending => assert!(turn < 2047),
            step => panic!("original vertex pick {step:?}"),
        }
    }
    assert_eq!(cursor.best.map(|hit| hit.id), Some(0), "surface sample at the pointer is not an original vertex");
    let mut plan = cursor.finish_plan(&state.session, &state.selection, &state.interaction, &state.scene, 10).unwrap().unwrap();
    let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
    for turn in 0..128 {
        if with_world_step_context(1, |context| publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut plan, 10, &mut input, context)).unwrap() == WorldInteractionStep::Complete { break; }
        assert!(turn < 127);
    }
    let actions = take_actions(&mut input);
    assert_eq!(actions.len(), 1);
    let targets: serde_json::Value = serde_json::from_str(actions[0].args.as_ref().unwrap().get("targets").unwrap().as_str().unwrap()).unwrap();
    assert_eq!(targets, serde_json::json!([{"granularity":"vertex","id":fixture["expected"]}]));
    let start: [f32; 2] = serde_json::from_value(fixture["marquee"]["start"].clone()).unwrap();
    let end: [f32; 2] = serde_json::from_value(fixture["marquee"]["end"].clone()).unwrap();
    let mut gesture = WorldMarqueeGesture::new(state.session.interaction_revision, 11, start);
    assert!(gesture.push(end));
    let mut marquee = WorldMarqueePickCursor::new(&state.session, &state.camera, &state.selection, 12, gesture).unwrap();
    for turn in 0..1024 {
        match with_world_step_context(1, |context| marquee.step(&state.session, &state.selection, &state.interaction, &state.scene, 12, context)) {
            WorldInteractionStep::Complete => break,
            WorldInteractionStep::Pending => assert!(turn < 1023),
            step => panic!("original vertex marquee {step:?}"),
        }
    }
    let groups: Vec<u32> = marquee.results.pages[0][..usize::from(marquee.results.lens[0])].iter().map(|result| match result.unwrap() { WorldMarqueeResult::Component { id, .. } => id, _ => panic!("original vertex component") }).collect();
    let expected_groups: Vec<u32> = serde_json::from_value(fixture["marquee"]["groups"].clone()).unwrap();
    assert_eq!(groups, expected_groups);
    let mut publish = WorldComponentMarqueePublishJob::new(12, marquee.gesture, marquee.results, WorldComponentKind::Vertex, false, false);
    for turn in 0..1024 {
        if with_world_step_context(1, |context| publish.step(&state.session, &state.interaction, &state.scene, 12, &mut input, context)).unwrap() == WorldInteractionStep::Complete { break; }
        assert!(turn < 1023);
    }
    let actions = take_actions(&mut input);
    assert_eq!(actions.len(), 1);
    let targets: serde_json::Value = serde_json::from_str(actions[0].args.as_ref().unwrap().get("targets").unwrap().as_str().unwrap()).unwrap();
    assert_eq!(targets, serde_json::json!([{"granularity":"vertex","id":fixture["expected"]}]));
}

/// 🤝️ Equal component groups retain their original object ownership through every shared merge mode.
#[test]
fn world_component_selection_merge_preserves_remaining_object_projection() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🎯️component-selection-merges/🔣️.json")).unwrap();
    let mut mesh = fixture["mesh"].clone();mesh["componentReferences"] = serde_json::json!({"face":[fixture["source"]["label"]]});
    let mut scene = scene_with_selection_and_domain("{}",Some(("geometry","object")));
    let world = scene.world_3d.as_mut().unwrap();
    world.meshes_json = serde_json::json!([{"id":"analytic","data":mesh}]).to_string();
    world.instances_json = serde_json::Value::Array(fixture["objects"].as_array().unwrap().iter().map(|object|serde_json::json!({"id":object,"meshId":"analytic","componentSource":{"handle":fixture["source"]["handle"],"revision":fixture["source"]["revision"]}})).collect()).to_string();
    let mut state = World3dState::new("surface".into(),"controller".into());
    drive_scene_bridge(&mut state,&scene,Rect::new(0.0,0.0,400.0,400.0));
    for row in fixture["steps"].as_array().unwrap() {
        let targets = serde_json::Value::Array(row["incoming"].as_array().unwrap().iter().map(|index|serde_json::json!({"granularity":"face","id":fixture["targets"][index.as_u64().unwrap() as usize]})).collect()).to_string();
        let action = ActionDescriptor { controller_id:"controller".into(),action:"interactionSelect".into(),args:Some(semio_framework::dsl_value!({"domainId":"geometry","targets":targets,"merge":row["merge"].as_str().unwrap(),"method":"pick"})) };
        apply_world_action_preview(&mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene,&action);
        let expected: Vec<String> = row["selected"].as_array().unwrap().iter().map(|index|fixture["targets"][index.as_u64().unwrap() as usize].as_str().unwrap().to_string()).collect();
        let expected_active = row["active"].as_u64().map(|index|fixture["objects"][index as usize].as_str().unwrap());
        assert_eq!(state.gumball.gumball_selection_ids,expected,"original encoded targets {row}");
        let objects: Vec<String> = row["selected"].as_array().unwrap().iter().map(|index|fixture["objects"][index.as_u64().unwrap() as usize].as_str().unwrap().to_string()).collect::<std::collections::BTreeSet<_>>().into_iter().collect();
        assert_eq!(state.selection.selected_ids,objects,"plain original Scene object projection {row}");
        assert_eq!(state.selection.component_ids,serde_json::from_value::<Vec<String>>(row["groups"].clone()).unwrap(),"active original group projection {row}");
        assert_eq!(state.selection.active_object_id.as_deref(),expected_active,"active original object {row}");
    }
}

/// 🧵️ The original Scene bridge retains pure analytic wires for vertex and edge selection.
#[test]
fn world_native_analytic_wire_preserves_original_vertex_and_edge_picking() {
    assert_native_original_wire(false);
}

/// 🧵️ Edge-only source samples remain visible without inventing selectable topology vertices.
#[test]
fn world_native_analytic_wire_preserves_edge_only_scene() {
    assert_native_original_wire(true);
}

fn assert_native_original_wire(edge_only: bool) {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🎯️analytic-wire-picking/🔣️.json")).unwrap();
    let mut scene = scene_with_selection_and_domain("{}",Some(("geometry","object")));
    let world = scene.world_3d.as_mut().unwrap();
    world.meshes_json = serde_json::json!([{"id":"wire","data":if edge_only {&fixture["edgeOnly"]["mesh"]}else{&fixture["mesh"]}}]).to_string();
    world.instances_json = serde_json::json!([{"id":fixture["instance"],"meshId":"wire","componentSource":{"handle":fixture["handle"],"revision":fixture["revision"]}}]).to_string();
    world.camera_json = fixture["camera"].to_string();
    let mut state = World3dState::new("surface".into(),"controller".into());
    drive_scene_bridge(&mut state,&scene,Rect::new(0.0,0.0,400.0,400.0));
    let schema = state.scene.meshes.get("wire").expect("original wire is published").schema().unwrap();
    assert_eq!((schema.vertices,schema.edges,schema.indices),(if edge_only {fixture["edgeOnly"]["expected"]["renderVertices"].as_u64().unwrap() as u32}else{4},4,0));
    if edge_only {
        assert_eq!(schema.vertex_ids,0,"render samples never become topology vertices");
        let (min,max)=state.scene.meshes.get("wire").unwrap().aabb().unwrap();
        assert_eq!(min,serde_json::from_value::<[f32;3]>(fixture["edgeOnly"]["expected"]["boundsMin"].clone()).unwrap());
        assert_eq!(max,serde_json::from_value::<[f32;3]>(fixture["edgeOnly"]["expected"]["boundsMax"].clone()).unwrap());
    }
    for row in fixture["picks"].as_array().unwrap() {
        if edge_only && row["kind"]=="vertex" {continue;}
        state.selection.granularity = row["kind"].as_str().unwrap().into();
        let mut registry = WorldInteractionRegistryBuildCursor::new(state.session.interaction_revision);
        for turn in 0..128 {
            match with_world_step_context(1,|context|registry.step(&mut state.session, &mut state.interaction, &mut state.scene,context)) {
                WorldInteractionStep::Complete=>break,
                WorldInteractionStep::Pending=>assert!(turn<127),
                step=>panic!("original wire registry {step:?}"),
            }
        }
        let mut cursor = WorldComponentPickCursor::new(&state.session, &state.camera, &state.selection,10,WorldComponentPickPurpose::Select,row["pointer"][0].as_f64().unwrap() as f32,row["pointer"][1].as_f64().unwrap() as f32).unwrap();
        for turn in 0..2048 {
            match with_world_step_context(1,|context|cursor.step(&state.session, &state.selection, &state.interaction, &state.scene,10,context)) {
                WorldInteractionStep::Complete=>break,
                WorldInteractionStep::Pending=>assert!(turn<2047),
                step=>panic!("original wire pick {step:?}"),
            }
        }
        assert_eq!(cursor.best.map(|hit|hit.id),Some(row["group"].as_u64().unwrap() as u32));
        let mut plan = cursor.finish_plan(&state.session, &state.selection, &state.interaction, &state.scene,10).unwrap().unwrap();
        let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
        for turn in 0..128 {
            if with_world_step_context(1,|context|publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene,&mut plan,10,&mut input,context)).unwrap()==WorldInteractionStep::Complete {break;}
            assert!(turn<127);
        }
        let actions = take_actions(&mut input);assert_eq!(actions.len(),1);
        let targets: serde_json::Value = serde_json::from_str(actions[0].args.as_ref().unwrap().get("targets").unwrap().as_str().unwrap()).unwrap();
        assert_eq!(targets,serde_json::json!([{"granularity":row["kind"],"id":row["expected"]}]));
    }
    if edge_only {
        state.selection.granularity = "vertex".into();
        state.selection.selection_targets.vertex = true;
        let mut registry = WorldInteractionRegistryBuildCursor::new(state.session.interaction_revision);
        for turn in 0..2048 {
            match with_world_step_context(1,|context|registry.step(&mut state.session, &mut state.interaction, &mut state.scene,context)) {
                WorldInteractionStep::Complete=>break,
                WorldInteractionStep::Pending=>assert!(turn<2047),
                step=>panic!("edge-only vertex registry {step:?}"),
            }
        }
        let mut cursor = WorldComponentPickCursor::new(&state.session, &state.camera, &state.selection,10,WorldComponentPickPurpose::Select,fixture["picks"][0]["pointer"][0].as_f64().unwrap() as f32,fixture["picks"][0]["pointer"][1].as_f64().unwrap() as f32).unwrap();
        for turn in 0..2048 {
            match with_world_step_context(1,|context|cursor.step(&state.session, &state.selection, &state.interaction, &state.scene,10,context)) {
                WorldInteractionStep::Complete=>break,
                WorldInteractionStep::Pending=>assert!(turn<2047),
                step=>panic!("edge-only vertex pick {step:?}"),
            }
        }
        assert!(cursor.best.is_none(),"packed render endpoints are not authored topology vertices");
        for (method,points) in [
            ("rectangle",vec![[0.0,0.0],[400.0,400.0]]),
            ("lasso",vec![[0.0,0.0],[400.0,0.0],[400.0,400.0],[0.0,400.0]]),
        ] {
            state.selection.selection_method = method.into();
            assert!(world_component_marquee_cursor_ids(&state,&points).is_empty(),"{method}: packed endpoints have no topology selection");
        }
        let mut lines = Vec::new();append_component_overlays(&state.selection, &state.gumball, &state.scene, &state.overlay,&semio_framework_ui_theme::Theme::default(),&state.camera.orbit.to_camera(),state.session.bounds,&mut lines);
        assert_eq!(lines.len(),fixture["edgeOnly"]["expected"]["renderVertices"].as_u64().unwrap() as usize,"no vertex glyph is synthesized for an edge endpoint");
    }
    retire_bridged_surface(&mut state);
}

/// 🧷️ A pure point paints through the original native primitive owner in ordinary object mode.
#[test]
fn world_native_analytic_point_paints_without_component_targeting() {
    let fixture:serde_json::Value=serde_json::from_str(read_static("../../../../../../../../🔨️modules/🧊️3d/📐️brep/⚙️engine/🧪️tests/🧫️fixtures/🎯️vertex-provenance/🔣️.json")).unwrap();
    let row=fixture["cases"].as_array().unwrap().iter().find(|row|row["kind"]=="point").unwrap();
    let point=serde_json::from_value::<[f32;3]>(row["points"][0].clone()).unwrap();
    let mut scene=scene_with_selection("{}");let world=scene.world_3d.as_mut().unwrap();
    world.meshes_json=serde_json::json!([{"id":"point","data":{"positions":point,"normals":[],"indices":[],"vertexIds":[0],"edgePositions":[],"edgeIds":[]}}]).to_string();
    world.instances_json=serde_json::json!([{"id":"point@vertex#0","meshId":"point"}]).to_string();
    world.camera_json=serde_json::json!({"position":[point[0],point[1],point[2]+4.0],"target":point,"up":[0,1,0],"fov":50}).to_string();
    world.selection_json=serde_json::json!({"granularity":"object","ids":[],"showEdges":false,"targets":{"mesh":true,"vertex":false,"edge":false,"face":false}}).to_string();
    let mut state=World3dState::new("surface-point".into(),"controller".into());let bounds=Rect::new(0.0,0.0,400.0,400.0);drive_scene_bridge(&mut state,&scene,bounds);
    let schema=state.scene.meshes.get("point").unwrap().schema().unwrap();assert_eq!((schema.vertices,schema.vertex_ids,schema.indices,schema.edges),(1,1,0,0));
    let mut lines=Vec::new();append_component_overlays(&state.selection, &state.gumball, &state.scene, &state.overlay,&semio_framework_ui_theme::Theme::default(),&state.camera.orbit.to_camera(),bounds,&mut lines);
    assert!(!lines.is_empty(),"an unselected pure point has a primary native visual");
    for pair in lines.chunks_exact(2) {for axis in 0..3 {assert!(((pair[0].position[axis]+pair[1].position[axis])*0.5-point[axis]).abs()<1e-5);}}
    assert!(state.selection.selected_ids.is_empty());assert_eq!(state.selection.granularity,"object");retire_bridged_surface(&mut state);
}

/// 🪡️ A wire is primary geometry even when surface outlines and component targeting are disabled.
#[test]
fn world_native_analytic_wire_paints_without_surface_outlines() {
    let fixture:serde_json::Value=serde_json::from_str(read_static("../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🎯️analytic-wire-picking/🔣️.json")).unwrap();
    for show_edges in [true,false] {
        let mut scene=scene_with_selection("{}");let world=scene.world_3d.as_mut().unwrap();
        world.meshes_json=serde_json::json!([{"id":"wire","data":fixture["mesh"]}]).to_string();world.instances_json=serde_json::json!([{"id":fixture["instance"],"meshId":"wire"}]).to_string();world.camera_json=fixture["camera"].to_string();
        world.selection_json=serde_json::json!({"granularity":"object","ids":[],"showEdges":show_edges,"targets":{"mesh":true,"vertex":false,"edge":false,"face":false}}).to_string();
        let mut state=World3dState::new("surface-wire".into(),"controller".into());let bounds=Rect::new(0.0,0.0,400.0,400.0);drive_scene_bridge(&mut state,&scene,bounds);
        let mut lines=Vec::new();append_component_overlays(&state.selection, &state.gumball, &state.scene, &state.overlay,&semio_framework_ui_theme::Theme::default(),&state.camera.orbit.to_camera(),bounds,&mut lines);
        assert_eq!(lines.iter().flat_map(|line|line.position).collect::<Vec<_>>(),serde_json::from_value::<Vec<f32>>(fixture["mesh"]["edgePositions"].clone()).unwrap(),"primary original wire showEdges={show_edges}");
        retire_bridged_surface(&mut state);
    }
}

/// 🖌️ Equal local face groups retain their original object when the selected overlay is scanned.
#[test]
fn world_face_selection_overlay_is_scoped_to_original_object() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🎯️component-selection-merges/🔣️.json")).unwrap();
    let mut mesh = fixture["mesh"].clone();mesh["componentReferences"] = serde_json::json!({"face":[fixture["source"]["label"]]});
    let mut scene = scene_with_selection_and_domain("{}",Some(("geometry","object")));
    let world = scene.world_3d.as_mut().unwrap();
    world.meshes_json = serde_json::json!([{"id":"analytic","data":mesh}]).to_string();
    world.instances_json = serde_json::Value::Array(fixture["objects"].as_array().unwrap().iter().map(|object|serde_json::json!({"id":object,"meshId":"analytic","componentSource":{"handle":fixture["source"]["handle"],"revision":fixture["source"]["revision"]}})).collect()).to_string();
    let mut state = World3dState::new("surface".into(),"controller".into());
    drive_scene_bridge(&mut state,&scene,Rect::new(0.0,0.0,400.0,400.0));
    for row in fixture["steps"].as_array().unwrap() {
        let targets = serde_json::Value::Array(row["incoming"].as_array().unwrap().iter().map(|index|serde_json::json!({"granularity":"face","id":fixture["targets"][index.as_u64().unwrap() as usize]})).collect()).to_string();
        apply_world_action_preview(&mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene,&ActionDescriptor {controller_id:"controller".into(),action:"interactionSelect".into(),args:Some(semio_framework::dsl_value!({"domainId":"geometry","targets":targets,"merge":row["merge"].as_str().unwrap(),"method":"pick"}))});
        let mut scan = WorldFaceOverlayScan::default();let mut objects = std::collections::BTreeSet::new();
        for turn in 0..4096 {
            match scan.step(&state.selection, &state.gumball, &state.scene) {
                WorldFaceOverlayScanStep::Complete=>break,
                WorldFaceOverlayScanStep::Pending=>assert!(turn<4095),
                WorldFaceOverlayScanStep::Match(hit)=>{
                    assert_eq!(hit.category,2);
                    let draw = state.scene.draws.get(usize::from(hit.draw)).unwrap();
                    objects.insert(draw.instances[usize::from(hit.instance)].id.to_string());
                }
            }
        }
        let expected: std::collections::BTreeSet<String> = row["selected"].as_array().unwrap().iter().map(|index|fixture["objects"][index.as_u64().unwrap() as usize].as_str().unwrap().to_string()).collect();
        assert_eq!(objects,expected,"original selected face overlays {row}");
    }
}

/// 🔐️ The original analytic face scan consumes the fresh Guest source lane and refuses ambiguity.
#[test]
fn world_face_selection_overlay_validates_fresh_guest_source_targets() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🎯️component-selection-merges/🔣️.json")).unwrap();
    let refusals: serde_json::Value = serde_json::from_str(read_static("../../../../../📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧪️tests/🧫️fixtures/🎯️analytic-component-target.json")).unwrap();
    let mut rows: Vec<serde_json::Value> = fixture["steps"].as_array().unwrap().iter().map(|step|serde_json::json!({"name":step["merge"],"targets":step["selected"].as_array().unwrap().iter().map(|index|fixture["targets"][index.as_u64().unwrap() as usize].clone()).collect::<Vec<_>>(),"labels":[fixture["source"]["label"]],"selected":step["selected"],"active":step["active"],"groups":step["groups"]})).collect();
    rows.extend(refusals["overlayRefusals"].as_array().unwrap().iter().map(|row|serde_json::json!({"name":row["name"],"targets":row["targets"],"labels":row["labels"],"selected":[],"active":0,"groups":["0"]})));
    for row in rows {
        let mut mesh=fixture["mesh"].clone();mesh["componentReferences"]=serde_json::json!({"face":row["labels"]});
        let mut scene=scene_with_selection_and_domain("{}",Some(("geometry","object")));let world=scene.world_3d.as_mut().unwrap();
        world.meshes_json=serde_json::json!([{"id":"analytic","data":mesh}]).to_string();
        world.instances_json=serde_json::Value::Array(fixture["objects"].as_array().unwrap().iter().map(|object|serde_json::json!({"id":object,"meshId":"analytic","componentSource":{"handle":fixture["source"]["handle"],"revision":fixture["source"]["revision"]}})).collect()).to_string();
        world.selection_json=serde_json::json!({"ids":fixture["objects"],"gumballSelectionIds":row["targets"],"granularity":"face","componentIds":row["groups"],"activeObjectId":row["active"].as_u64().map(|index|fixture["objects"][index as usize].clone())}).to_string();
        let mut state=World3dState::new("surface".into(),"controller".into());drive_scene_bridge(&mut state,&scene,Rect::new(0.0,0.0,400.0,400.0));
        assert_eq!(state.selection.selected_ids,serde_json::from_value::<Vec<String>>(fixture["objects"].clone()).unwrap(),"Guest ids retain their plain object contract");
        let mut scan=WorldFaceOverlayScan::default();let mut objects=std::collections::BTreeSet::new();
        for turn in 0..4096 {
            match scan.step(&state.selection, &state.gumball, &state.scene) {
                WorldFaceOverlayScanStep::Complete=>break,
                WorldFaceOverlayScanStep::Pending=>assert!(turn<4095),
                WorldFaceOverlayScanStep::Match(hit)=>{assert_eq!(hit.category,2);objects.insert(state.scene.draws.get(usize::from(hit.draw)).unwrap().instances[usize::from(hit.instance)].id.to_string());}
            }
        }
        let expected: std::collections::BTreeSet<String>=row["selected"].as_array().unwrap().iter().map(|index|fixture["objects"][index.as_u64().unwrap() as usize].as_str().unwrap().to_string()).collect();
        assert_eq!(objects,expected,"fresh original Guest source lane {}",row["name"]);
        retire_bridged_surface(&mut state);
    }
}
