/// 🔲️ Laws of the object, component and marquee cursors, the face overlay and the right-gesture intents.

#[test]
fn terrain_visibility_requires_the_complete_family_marker() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    let tile = (3, 4, 5);
    assert!(!terrain_family_visible(&state.terrain, tile));
    state.terrain.terrain_built_tiles.insert(tile);
    assert!(terrain_family_visible(&state.terrain, tile));
}

fn allocate_oracle_mesh(token:Mesh3dWriteToken) {
    loop {
        let grant=world_mesh_ownership_grant(0,ui_scene::math::mesh3d_allocate_capacity_byte_demand(token).unwrap(),0,1);
        let step=mesh3d_allocate_step(token,grant).unwrap();
        assert!(step.progress.fits(grant));
        if step.complete{break}
    }
}

fn close_oracle_mesh(lease:Mesh3dLease) {
    loop {
        let demand=ui_scene::math::mesh3d_close_demands(lease,4096).unwrap();
        let grant=world_mesh_ownership_grant(4096,demand.capacity_bytes,demand.release_bytes,demand.depth);
        let step=mesh3d_close_step(lease,grant).unwrap();
        assert!(step.progress.fits(grant));
        if step.complete{break}
    }
}

fn face_overlay_test_mesh_with_faces(generation: u64, revision: u64, face_ids: &[u32]) -> Mesh3dLease {
    let triangles = u32::try_from(face_ids.len()).expect("fixture triangle count");
    let vertices = triangles * 3;
    let schema = Mesh3dSchema { vertices, indices: vertices, face_ids: triangles, vertex_ids: 0, edges: 0, edge_ids: 0, uvs: 0, colors: 0 ,surface_uvs:[0;4],tangents:0,};
    let grant=world_mesh_ownership_grant(0,ui_scene::math::mesh3d_begin_capacity_byte_demand(),0,1);
    let (token,progress)=mesh3d_begin(generation,revision,schema,grant).expect("face overlay fixture claim");
    assert!(progress.fits(grant));
    allocate_oracle_mesh(token);
    for triangle in 0..triangles {
        let x = triangle as f32 * 2.0;
        for position in [[x, 0.0, 0.0], [x + 1.0, 0.0, 0.0], [x, 1.0, 0.0]] {
            mesh3d_write_vec3(token, Mesh3dField::Positions, position).unwrap();
            mesh3d_write_vec3(token, Mesh3dField::Normals, [0.0, 0.0, 1.0]).unwrap();
        }
    }
    for index in 0..vertices {
        mesh3d_write_u32(token, Mesh3dField::Indices, index).unwrap();
    }
    for face_id in face_ids {
        mesh3d_write_u32(token, Mesh3dField::FaceIds, *face_id).unwrap();
    }
    mesh3d_seal(token).expect("face overlay fixture lease")
}

fn face_overlay_test_mesh(generation: u64, revision: u64, face_id: u32) -> Mesh3dLease {
    face_overlay_test_mesh_with_faces(generation, revision, &[face_id])
}

#[test]
fn face_overlay_writer_matches_legacy_winding_and_retires_stale_generation() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.session.interaction_revision = 7;
    state.scene.draw_generation = 11;
    state.selection.granularity = "face".into();
    state.selection.active_object_id = Some("object".into());
    state.selection.component_ids.push("12".into());
    let source = face_overlay_test_mesh(400, 7, 12);
    publish_world3d_mesh_lease(&mut state.interaction, &mut state.scene, "source".into(), source).unwrap();
    let version = *state.scene.mesh_versions.get("source").expect("source mesh version");
    state.scene.draws.push(SceneDraw3d { mesh_key: "source".into(), mesh_version: version, instances: vec![Instance3d { component_source: None, id: "object".into(), model: Mat4::identity(), color: [1.0; 4], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() }).unwrap();

    let mut cursor = WorldFaceOverlayMeshCursor::new("surface", 500, 7, 11).unwrap();
    let mut published = None;
    for _ in 0..128 {
        match cursor.step(&state.session, &state.selection, &state.gumball, &state.scene, &state.overlay) {
            WorldFaceOverlayMeshStep::Pending => {}
            WorldFaceOverlayMeshStep::Ready { index, color, key, lease } => {
                assert_eq!((index, color, key.as_str()), (0, [0.35, 0.75, 1.0, 0.62], "component-face-overlay:surface:500:0"));
                let schema = lease.schema().unwrap();
                assert_eq!((schema.vertices, schema.indices), (3, 6));
                assert_eq!(lease.vec3(Mesh3dField::Positions, 0).unwrap(), [0.0, 0.0, FACE_OVERLAY_OFFSET * 0.5]);
                assert_eq!(lease.vec3(Mesh3dField::Positions, 1).unwrap(), [1.0, 0.0, FACE_OVERLAY_OFFSET * 0.5]);
                assert_eq!(lease.vec3(Mesh3dField::Positions, 2).unwrap(), [0.0, 1.0, FACE_OVERLAY_OFFSET * 0.5]);
                assert_eq!((0..6).map(|item| lease.u32(Mesh3dField::Indices, item).unwrap()).collect::<Vec<_>>(), vec![0, 1, 2, 0, 2, 1]);
                published = Some(lease);
                cursor.published_colors[index] = Some(color);
            }
            WorldFaceOverlayMeshStep::Complete { generation, revision, draw_generation, colors } => {
                assert_eq!((generation, revision, draw_generation), (500, 7, 11));
                assert_eq!(colors, [Some([0.35, 0.75, 1.0, 0.62]), None, None]);
                break;
            }
            WorldFaceOverlayMeshStep::Stale | WorldFaceOverlayMeshStep::Fault => panic!("valid overlay fixture faulted"),
        }
    }
    assert!(cursor.terminal_is_empty());
    let published = published.expect("selected overlay published");
    mesh3d_begin_close(published).unwrap();
    close_oracle_mesh(published);

    let mut stale = WorldFaceOverlayMeshCursor::new("surface", 600, 7, 11).unwrap();
    assert!(matches!(stale.step(&state.session, &state.selection, &state.gumball, &state.scene, &state.overlay), WorldFaceOverlayMeshStep::Pending));
    state.session.interaction_revision = 8;
    let mut terminal = false;
    for _ in 0..32 {
        if matches!(stale.step(&state.session, &state.selection, &state.gumball, &state.scene, &state.overlay), WorldFaceOverlayMeshStep::Stale) {
            terminal = true;
            break;
        }
    }
    assert!(terminal && stale.terminal_is_empty());
    assert!(WorldFaceOverlayMeshCursor::new(&"x".repeat(WORLD_DYNAMIC_ID_BYTE_CAPACITY), 700, 1, 1).is_err());
}

#[test]
fn face_overlay_family_becomes_visible_only_after_every_bucket_is_published() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.session.interaction_revision = 7;
    state.scene.draw_generation = 11;
    state.selection.granularity = "face".into();
    state.selection.active_object_id = Some("object".into());
    state.selection.marquee_preview_ids.push("10".into());
    state.selection.component_ids.push("12".into());
    state.selection.hovered_component_id = Some("11".into());
    state.selection.hovered_component_object_id = Some("object".into());
    state.selection.hovered_component_mode = Some("face".into());
    let source = face_overlay_test_mesh_with_faces(750, 7, &[10, 11, 12]);
    publish_world3d_mesh_lease(&mut state.interaction, &mut state.scene, "source".into(), source).unwrap();
    let version = *state.scene.mesh_versions.get("source").expect("source mesh version");
    state.scene.draws.push(SceneDraw3d { mesh_key: "source".into(), mesh_version: version, instances: vec![Instance3d { component_source: None, id: "object".into(), model: Mat4::identity(), color: [1.0; 4], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() }).unwrap();
    state.overlay.face_overlay_build = Some(WorldFaceOverlayMeshCursor::new("surface", 800, 7, 11).unwrap());

    let mut staged_seen = false;
    for _ in 0..1_024 {
        step_component_face_overlay_build(&mut state);
        if state.scene.meshes.contains_key("component-face-overlay:surface:800:0") && state.overlay.face_overlay_generation.is_none() {
            staged_seen = true;
        }
        if state.overlay.face_overlay_generation == Some(800) {
            break;
        }
    }
    assert!(staged_seen, "a completed bucket remains invisible while the family transaction is partial");
    assert_eq!(state.overlay.face_overlay_generation, Some(800));
    assert_eq!(state.overlay.face_overlay_colors, [Some([1.0, 0.85, 0.35, 0.36]), Some([0.35, 0.75, 1.0, 0.48]), Some([0.35, 0.75, 1.0, 0.62])]);
    for index in 0..3 {
        assert!(state.scene.meshes.contains_key(&format!("component-face-overlay:surface:800:{index}")), "every nonempty category publishes before family visibility");
    }

    state.overlay.face_overlay_build = Some(WorldFaceOverlayMeshCursor::new("surface", 900, 7, 11).unwrap());
    for _ in 0..1_024 {
        step_component_face_overlay_build(&mut state);
        if state.scene.meshes.contains_key("component-face-overlay:surface:900:0") {
            break;
        }
    }
    assert_eq!(state.overlay.face_overlay_generation, Some(800), "superseding partial family never changes the visible generation");
    state.session.interaction_revision = 8;
    for _ in 0..64 {
        step_component_face_overlay_build(&mut state);
        if state.overlay.face_overlay_build.is_none() {
            break;
        }
    }
    assert_eq!(state.overlay.face_overlay_generation, Some(800), "stale family retirement preserves the last complete publication");
    assert!(state.scene.meshes.contains_key("component-face-overlay:surface:900:0"), "stale partial publication remains owned by the generation-qualified registry until bounded retirement");
}

#[test]
fn world_object_registry_build_is_one_owner_per_turn_and_interruptible() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.session.interaction_revision = 7;
    let mesh = publish_oracle_mesh(mesh_oracle_from_buffers(vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0], vec![0, 1, 2]));
    store_mesh(&mut state.session, &mut state.interaction, &mut state.authority, &mut state.scene, "mesh".into(), mesh);
    let mesh_version = *state.scene.mesh_versions.get("mesh").expect("mesh version");
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh".into(), mesh_version, instances: vec![Instance3d { component_source: None, id: "instance".into(), model: Mat4::identity(), color: [1.0; 4], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    state.scene.vortices.push(WorldVortexRecord { full_id: "vortex".into(), position: Some([1.0, 2.0, 3.0]), radius: Some(0.5), ..Default::default() });
    state.scene.references.push(WorldReferenceRecord { url: Some("reference".into()), origin: Some([4.0, 5.0, 6.0]), width_world: Some(2.0), hidden: Some(false), ..Default::default() });
    state.scene.reference_pixels.insert("reference".into(), test_world_scene_raster(2, 1, 0));

    let mut cursor = WorldInteractionRegistryBuildCursor::new(7);
    assert_eq!(with_world_step_context(0, |context| cursor.step(&mut state.session, &mut state.interaction, &mut state.scene, context)), WorldInteractionStep::Pending);
    let mut turns = 0;
    while with_world_step_context(1, |context| cursor.step(&mut state.session, &mut state.interaction, &mut state.scene, context)) == WorldInteractionStep::Pending {
        turns += 1;
        assert!(turns < 16);
    }
    assert!(state.interaction.interaction_objects.terminal_for_revision(7));
    let live: Vec<_> = state.interaction.interaction_objects.slots.iter().flatten().filter(|slot| slot.revision == 7).collect();
    assert_eq!(live.len(), 3);
    assert!(live.iter().any(|slot| slot.kind == WorldInteractionObjectKind::Instance && slot.id.as_str() == "instance"));
    assert!(live.iter().any(|slot| slot.kind == WorldInteractionObjectKind::Vortex && slot.id.as_str() == "vortex"));
    assert!(live.iter().any(|slot| slot.kind == WorldInteractionObjectKind::Reference && slot.id.as_str() == "reference"));

    let token = state.interaction
        .interaction_objects
        .slots
        .iter()
        .enumerate()
        .find_map(|(index, slot)| slot.as_ref().filter(|slot| slot.kind == WorldInteractionObjectKind::Reference).map(|slot| WorldInteractionObjectToken { slot: index as u16, generation: slot.generation, revision: slot.revision }))
        .expect("reference token");
    assert_eq!(state.interaction.interaction_objects.resolve(token).map(|slot| slot.values[4]), Some(1.0));

    let mut interrupted = WorldInteractionRegistryBuildCursor::new(8);
    assert!(with_world_step_context(1, |context| interrupted.close_step(context)));
    assert_eq!(interrupted.phase, WorldInteractionRegistryBuildPhase::Complete);
}

#[test]
fn world_vortex_and_reference_pick_cursors_resume_revalidate_and_close() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.session.interaction_revision = 3;
    state.interaction.interaction_objects.revision = 3;
    let vortex = state.interaction.interaction_objects.admit(3, WorldInteractionObjectKind::Vortex, "vortex", None, Mat4::identity(), [0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.5, 0.0], None).expect("vortex token");
    state.interaction.interaction_objects.admit(3, WorldInteractionObjectKind::Reference, "reference", None, Mat4::identity(), [0.0, 0.0, 0.0, 2.0, 1.0, 0.0, 0.0, 0.0], None).expect("reference token");

    let mut vortex_cursor = WorldObjectPickCursor::from_ray(3, 8, WorldObjectPickPurpose::VortexSelect, Vec3::new(0.0, -2.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    assert_eq!(with_world_step_context(0, |context| vortex_cursor.step(&state.session, &state.interaction, 8, context)), WorldInteractionStep::Pending);
    for _ in 0..=WORLD_INTERACTION_OBJECT_CAPACITY {
        if with_world_step_context(1, |context| vortex_cursor.step(&state.session, &state.interaction, 8, context)) == WorldInteractionStep::Complete {
            break;
        }
    }
    assert_eq!(vortex_cursor.best.map(|(token, _)| token), Some(vortex));
    let mut plan = vortex_cursor.finish_plan(&state.session, &state.selection, &state.interaction, 8).expect("vortex plan").expect("vortex hit");
    let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
    assert_eq!(with_world_step_context(1, |context| publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut plan, 8, &mut input, context)).unwrap(), WorldInteractionStep::Pending);
    let actions = take_actions(&mut input);
    assert_eq!(actions.len(), 1);

    state.interaction.interaction_objects.revision = state.session.interaction_revision;
    let mut reference_cursor = WorldObjectPickCursor::from_ray(state.session.interaction_revision, 9, WorldObjectPickPurpose::ReferenceHover, Vec3::new(0.0, 0.0, 2.0), Vec3::new(0.0, 0.0, -1.0));
    for _ in 0..=WORLD_INTERACTION_OBJECT_CAPACITY {
        if with_world_step_context(1, |context| reference_cursor.step(&state.session, &state.interaction, 9, context)) == WorldInteractionStep::Complete {
            break;
        }
    }
    assert!(reference_cursor.best.is_some());
    state.session.interaction_revision = state.session.interaction_revision.wrapping_add(1);
    assert!(matches!(reference_cursor.finish_plan(&state.session, &state.selection, &state.interaction, 9), Err(WorldInteractionStep::Stale)));
    assert!(!reference_cursor.close_step());
    assert!(reference_cursor.close_step());
}

#[test]
fn world_component_cursor_steps_one_topology_item_and_rejects_aba() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.session.interaction_revision = 5;
    state.interaction.interaction_objects.revision = 5;
    state.selection.granularity = "face".into();
    let mesh = publish_oracle_mesh(triangle_mesh_oracle());
    store_mesh(&mut state.session, &mut state.interaction, &mut state.authority, &mut state.scene, "mesh".into(), mesh);
    let mesh_token = state.interaction
        .interaction_meshes
        .slots
        .iter()
        .enumerate()
        .find_map(|(index, slot)| slot.as_ref().filter(|slot| slot.id.as_str() == "mesh").map(|slot| WorldInteractionMeshToken { slot: index as u16, generation: slot.generation }))
        .expect("mesh token");
    let object = state.interaction.interaction_objects.admit(5, WorldInteractionObjectKind::Instance, "object", Some(mesh_token), Mat4::identity(), [0.0; 8], None).expect("object token");
    let mut cursor = WorldComponentPickCursor {
        revision: 5,
        generation: 10,
        purpose: WorldComponentPickPurpose::Select,
        kind: WorldComponentKind::Face,
        local_x: 0.0,
        local_y: 0.0,
        viewport: Rect { x: 0.0, y: 0.0, w: 100.0, h: 100.0 },
        view_projection: Mat4::identity(),
        projection_spec: ui_scene::math::default_projection_spec(),
        origin: Vec3::new(0.0, 0.0, 2.0),
        direction: Vec3::new(0.0, 0.0, -1.0),
        slot: object.slot,
        current: None,
        topology: 0,
        merge: 0,
        best: None,
        complete: false,
    };
    assert_eq!(with_world_step_context(1, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 10, context)), WorldInteractionStep::Pending);
    assert_eq!(cursor.current, Some(object));
    assert_eq!(with_world_step_context(1, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 10, context)), WorldInteractionStep::Pending);
    assert_eq!(cursor.topology, 1);
    assert_eq!(cursor.best.map(|hit| hit.id), Some(0));
    cursor.current = None;
    cursor.slot = WORLD_INTERACTION_OBJECT_CAPACITY as u16;
    assert_eq!(with_world_step_context(1, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 10, context)), WorldInteractionStep::Pending);
    assert_eq!(with_world_step_context(1, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 10, context)), WorldInteractionStep::Complete);
    let mut plan = cursor.finish_plan(&state.session, &state.selection, &state.interaction, &state.scene, 10).expect("component plan").expect("component hit");
    let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
    assert_eq!(with_world_step_context(1, |context| publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut plan, 10, &mut input, context)).unwrap(), WorldInteractionStep::Pending);
    assert_eq!(take_actions(&mut input).len(), 1);

    state.interaction.interaction_objects.revision = state.session.interaction_revision;
    let mut replacement_model = Mat4::identity();
    replacement_model.cols[3][0] = 1.0;
    let replacement = state.interaction.interaction_objects.admit(state.session.interaction_revision, WorldInteractionObjectKind::Instance, "object", Some(mesh_token), replacement_model, [0.0; 8], None).expect("replacement object token");
    assert_ne!(replacement, object);
    cursor.revision = state.session.interaction_revision;
    cursor.complete = true;
    cursor.best = Some(WorldComponentHit { object, id: 0, primary: 1.0, secondary: 0.0 });
    assert!(matches!(cursor.finish_plan(&state.session, &state.selection, &state.interaction, &state.scene, 10), Err(WorldInteractionStep::Stale)));
    assert!(!cursor.close_step());
    assert!(cursor.close_step());
}

/// 🖱️📋️ **The right-gesture law.** A right press/drag/release over a world surface publishes no
/// action while the gesture runs — the menu is the shell's and the camera report is the settle's.
/// Plain right is bound to NOTHING in React's orbit map (`resolveWorldOrbitMouseButtonsIdle`:
/// `RIGHT: null`, pan on Shift, orbit on Alt), so an unmodified right-drag moves no camera either.
#[test]
fn a_right_gesture_publishes_nothing_while_it_runs() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.session.interaction_revision = 4;
    state.interaction.interaction_objects.revision = 4;
    state.selection.hovered_vortex_id = Some("vortex".into());
    let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
    let mut authority = WorldInteractionAuthority::default();
    authority.next_generation = 4;
    authority.queue.push(WorldInteractionIntent::pointer_button(0.0, 0.0, true, 2, &PointerModifiers::default())).unwrap();
    authority.queue.slots[0].as_mut().unwrap().generation = 1;
    authority.queue.push(WorldInteractionIntent::pointer_move(10.0, 0.0, 10.0, 0.0, true, 2, &PointerModifiers::default())).unwrap();
    authority.queue.slots[1].as_mut().unwrap().generation = 2;
    authority.queue.push(WorldInteractionIntent::pointer_button(10.0, 0.0, false, 2, &PointerModifiers::default())).unwrap();
    authority.queue.slots[2].as_mut().unwrap().generation = 3;
    state.authority.interaction_authority = Some(authority);
    state.interaction.interaction_objects.revision = state.session.interaction_revision;
    for generation in 1..=3 {
        assert_eq!(with_world_step_context(1, |context| step_world3d_interaction(&mut state, generation, &mut input, context)), WorldInteractionAuthorityStep::Complete);
    }
    assert!(take_actions(&mut input).is_empty());
}

/// 🖱️ **The stale-aim law.** A queued intent outlives a relayout: dragging the dock's split gutter
/// moves a pane out from under the moves already enqueued against its previous bounds. Every pick
/// cursor refuses a point outside the CURRENT pick rect, and that refusal used to FAULT the authority
/// — a frame fault, which kills the page, so the whole wgpu shell tore down mid-drag (ticket
/// 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w9c-behaviour-parity-run-2.md` step 15). A stale aim
/// retires like any other answered intent; a point the surface DOES cover still faults, because there
/// the refusal is real.
///
/// 🖱️ The button no CAMERA gesture claims still answers, and never faults — a middle press used to
/// fall through to the unclaimed-intent fault, which killed the page on the first middle-click of
/// any world surface. What it answers is the selection React publishes for any button (packet
/// W13c's `plan_world3d_non_primary_press_pick`, through R3F's `onPointerMissed`); the release
/// retires as before.
#[test]
fn an_intent_aimed_outside_the_surface_retires_instead_of_faulting_the_frame() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.session.bounds = Rect { x: 536.0, y: 54.0, w: 1061.0, h: 913.0 };
    state.interaction.interaction_objects.revision = state.session.interaction_revision;
    let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
    let mut authority = WorldInteractionAuthority::default();
    authority.next_generation = 2;
    authority.queue.push(WorldInteractionIntent::pointer_move(534.4, 500.0, -34.0, -453.0, false, 0, &PointerModifiers::default())).unwrap();
    authority.queue.slots[0].as_mut().unwrap().generation = 1;
    state.authority.interaction_authority = Some(authority);
    assert_eq!(with_world_step_context(1, |context| step_world3d_interaction(&mut state, 1, &mut input, context)), WorldInteractionAuthorityStep::Complete, "🖱️ a hover the pane no longer covers is retired");
    assert_eq!(world3d_interaction_front_generation(&state.camera, &state.authority), None, "🖱️ and it leaves the queue rather than blocking it");
    assert!(!state.authority.interaction_authority.as_ref().expect("authority").faulted, "🖱️ the authority stays usable for the next gesture");
    assert!(take_actions(&mut input).is_empty());

    let mut authority = WorldInteractionAuthority::default();
    authority.next_generation = 3;
    authority.queue.push(WorldInteractionIntent::pointer_button(1000.0, 400.0, true, 1, &PointerModifiers::default())).unwrap();
    authority.queue.slots[0].as_mut().unwrap().generation = 1;
    authority.queue.push(WorldInteractionIntent::pointer_button(1000.0, 400.0, false, 1, &PointerModifiers::default())).unwrap();
    authority.queue.slots[1].as_mut().unwrap().generation = 2;
    state.authority.interaction_authority = Some(authority);
    for generation in 1..=2 {
        let mut turns = 0;
        loop {
            let step = with_world_step_context(1, |context| step_world3d_interaction(&mut state, generation, &mut input, context));
            assert_ne!(step, WorldInteractionAuthorityStep::Fault, "🖱️ a middle press inside the pane never faults");
            turns += 1;
            assert!(turns < 64, "🖱️ a bounded press answers in bounded turns: generation={generation}, census={}", state.interaction_census());
            if step == WorldInteractionAuthorityStep::Complete {
                break;
            }
            assert_eq!(step, WorldInteractionAuthorityStep::Pending);
        }
    }
    assert!(!state.authority.interaction_authority.as_ref().expect("authority").faulted);
    assert_eq!(take_actions(&mut input).into_iter().map(|action| action.action).collect::<Vec<_>>(), ["interactionSelect"], "🖱️ the press publishes the selection React's own `handleEmptyClick` publishes for any button");
}

#[test]
fn world_marquee_overlay_points_reads_authority_gesture() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    let mut gesture = WorldMarqueeGesture::new(1, 1, [10.0, 20.0]);
    assert!(gesture.push([100.0, 80.0]));
    state.authority.interaction_authority.as_mut().unwrap().marquee = Some(gesture);
    assert!(!state.selection.marquee_active);
    let points = world_marquee_overlay_points(&state.selection, &state.authority).expect("overlay points");
    assert_eq!(points, vec![[10.0, 20.0], [100.0, 80.0]]);
}

#[test]
fn world_marquee_click_retires_points_before_exact_release_retry() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.interaction.interaction_objects.revision = state.session.interaction_revision;
    let modifiers = PointerModifiers::default();
    let mut down = WorldInteractionIntent::pointer_button(0.0, 0.0, true, 0, &modifiers);
    down.generation = 1;
    let mut up = WorldInteractionIntent::pointer_button(1.0, 1.0, false, 0, &modifiers);
    up.generation = 2;
    state.authority.interaction_authority.as_mut().unwrap().queue.push(down).unwrap();
    state.authority.interaction_authority.as_mut().unwrap().queue.push(up).unwrap();
    let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
    assert_eq!(with_world_step_context(1, |context| step_world3d_interaction(&mut state, 1, &mut input, context)), WorldInteractionAuthorityStep::Complete);
    assert_eq!(state.authority.interaction_authority.as_ref().unwrap().marquee.as_ref().map(|gesture| gesture.len), Some(1));
    assert_eq!(with_world_step_context(1, |context| step_world3d_interaction(&mut state, 2, &mut input, context)), WorldInteractionAuthorityStep::Pending);
    assert!(state.authority.interaction_authority.as_ref().unwrap().marquee.as_ref().is_some_and(|gesture| gesture.retiring));
    assert_eq!(with_world_step_context(1, |context| step_world3d_interaction(&mut state, 2, &mut input, context)), WorldInteractionAuthorityStep::Stale);
    assert_eq!(with_world_step_context(1, |context| step_world3d_interaction(&mut state, 2, &mut input, context)), WorldInteractionAuthorityStep::Stale);
    assert!(state.authority.interaction_authority.as_ref().unwrap().marquee.is_none());
    assert_eq!(world3d_interaction_front_generation(&state.camera, &state.authority), Some(2));
}

/// 🎯️ `targets` is a JSON-encoded STRING, not a structured array: the framework's own decoder is
/// `parse_interaction_targets`, which reads the arg with `DslValue::as_str`
/// (`💻️os/🔨️modules/🔌️plugin/🦀️.rs`), and the manifest declares it `ActionArgDef::text`. An array
/// makes `as_str` answer `None` and the guest faults the job — ticket
/// 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-world3d-interaction-2026-09-13.md`.
#[test]
fn world_marquee_pages_build_one_target_per_grant_and_publish_atomically_fifo() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.session.interaction_revision = 6;
    state.interaction.interaction_objects.revision = 6;
    let mut results = WorldMarqueeResultPages::default();
    for index in 0..=WORLD_MARQUEE_RESULT_PAGE_CAPACITY {
        let id = format!("target-{index:03}");
        let token = state.interaction.interaction_objects.admit(6, WorldInteractionObjectKind::Instance, &id, None, Mat4::identity(), [0.0; 8], None).expect("marquee result token");
        assert!(results.push(WorldMarqueeResult::Object(token), id.len()));
    }
    let mut gesture = WorldMarqueeGesture::new(6, 7, [0.0, 0.0]);
    assert!(gesture.push([100.0, 100.0]));
    let mut job = WorldMarqueePublishJob::new(8, gesture, results, false, false);
    let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
    let mut turns = 0;
    loop {
        turns += 1;
        let step = with_world_step_context(1, |context| job.step(&state.session, &state.interaction, 8, &mut input, context)).expect("bounded marquee page step");
        if step == WorldInteractionStep::Complete {
            break;
        }
        assert!(turns < 400);
    }
    let actions = take_actions(&mut input);
    assert_eq!(actions.len(), 2);
    let page_targets = |action: &ActionDescriptor| {
        let raw = action.args.as_ref().and_then(|args| args.get("targets")).and_then(|value| value.as_str()).expect("targets is JSON text");
        serde_json::from_str::<Vec<serde_json::Value>>(raw).expect("targets decodes as the framework does")
    };
    assert_eq!(page_targets(&actions[0]).len(), WORLD_MARQUEE_RESULT_PAGE_CAPACITY);
    assert_eq!(page_targets(&actions[1]).len(), 1);
    assert_eq!(page_targets(&actions[0])[0]["granularity"], resolved_domain_granularity_id(&state.session), "a marquee target uses the scene's declared interaction granularity");
    assert_eq!(job.results.page_len, 0);
    assert_eq!(job.gesture.len, 0);
}

#[test]
fn world_marquee_page_claim_saturation_preserves_all_results_for_exact_retry() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.session.interaction_revision = 6;
    state.interaction.interaction_objects.revision = 6;
    let id = "target";
    let token = state.interaction.interaction_objects.admit(6, WorldInteractionObjectKind::Instance, id, None, Mat4::identity(), [0.0; 8], None).expect("target token");
    let mut results = WorldMarqueeResultPages::default();
    assert!(results.push(WorldMarqueeResult::Object(token), id.len()));
    let mut gesture = WorldMarqueeGesture::new(6, 7, [0.0, 0.0]);
    assert!(gesture.push([100.0, 100.0]));
    let mut job = WorldMarqueePublishJob::new(8, gesture, results, false, false);
    let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
    let mut blockers = Vec::new();
    while let Ok(claim) = input.claim_action(1) {
        blockers.push(claim);
    }
    assert!(matches!(with_world_step_context(1, |context| job.step(&state.session, &state.interaction, 8, &mut input, context)), Err(semio_framework_ui_action::BoundedActionFault::ItemCredits)));
    assert!(job.prepared.is_none());
    assert_eq!(job.results.lens[0], 1);
    input.release_action_claim(blockers.pop().expect("retry claim")).expect("release retry claim");
    assert_eq!(with_world_step_context(1, |context| job.step(&state.session, &state.interaction, 8, &mut input, context)).unwrap(), WorldInteractionStep::Pending);
    assert!(job.prepared.is_some());
    while !job.close_step(&mut input) {}
    for claim in blockers {
        input.release_action_claim(claim).expect("release blocker");
    }
    assert!(take_actions(&mut input).is_empty());
    assert_eq!(job.results.page_len, 0);
    assert_eq!(job.gesture.len, 0);
}

/// 🔺️ The marquee triangle carrying its OWN vertex, edge and face id buffers — what a decoded GLB
/// brings and what `screen_select_components` needs before it can answer anything but the whole
/// instance: with empty id buffers its `"vertex"`/`"edge"` arms are guarded off
/// (`🎬️scene/📐️math/🎯️picking/🦀️.rs:299`, `:316`) and the oracle falls through to the instance-id arm, which
/// is not a component census at all.
fn component_triangle_mesh_oracle() -> TerrainMeshOracleData {
    let mut data = triangle_mesh_oracle();
    data.vertex_ids = vec![11, 12, 13];
    data.face_ids = vec![31];
    data.edge_positions = vec![-1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, -1.0, -1.0, 0.0];
    data.edge_ids = vec![21, 22, 23];
    data
}

fn world_marquee_geometry_fixture(instance_count: usize) -> World3dState {
    world_marquee_geometry_fixture_from(triangle_mesh_oracle(), instance_count)
}

fn world_marquee_geometry_fixture_from(data: TerrainMeshOracleData, instance_count: usize) -> World3dState {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.session.bounds = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.session.pick_bounds = state.session.bounds;
    state.session.interaction_revision = 2;
    let mesh = publish_oracle_mesh(data);
    store_mesh(&mut state.session, &mut state.interaction, &mut state.authority, &mut state.scene, "mesh".into(), mesh);
    let mesh_version = *state.scene.mesh_versions.get("mesh").expect("mesh version");
    let instances = (0..instance_count).map(|index| Instance3d { component_source: None, id: format!("object-{index:03}"), model: Mat4::identity(), color: [1.0; 4], selected: false, hovered: false, material: Default::default() }).collect();
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh".into(), mesh_version, instances, shadow_role: Default::default() });
    let mut registry = WorldInteractionRegistryBuildCursor::new(2);
    let mut turns = 0;
    while with_world_step_context(1, |context| registry.step(&mut state.session, &mut state.interaction, &mut state.scene, context)) == WorldInteractionStep::Pending {
        turns += 1;
        assert!(turns < instance_count + 16);
    }
    assert!(state.interaction.interaction_objects.terminal_for_revision(2));
    state
}

fn world_marquee_cursor_ids(state: &World3dState, points: &[[f32; 2]]) -> Vec<String> {
    let mut gesture = WorldMarqueeGesture::new(state.session.interaction_revision, 7, points[0]);
    for point in points.iter().skip(1) {
        assert!(gesture.push(*point));
    }
    let mut cursor = WorldMarqueePickCursor::new(&state.session, &state.camera, &state.selection, 8, gesture).expect("marquee cursor");
    let mut turns = 0;
    loop {
        turns += 1;
        if with_world_step_context(1, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 8, context)) == WorldInteractionStep::Complete {
            break;
        }
        assert!(turns < 8_000);
    }
    let mut ids = Vec::new();
    for page in 0..usize::from(cursor.results.page_len) {
        for item in 0..usize::from(cursor.results.lens[page]) {
            let WorldMarqueeResult::Object(token) = cursor.results.pages[page][item].expect("marquee result token") else {
                panic!("object marquee produced component token");
            };
            ids.push(state.interaction.interaction_objects.resolve(token).expect("live result token").id.as_str().to_owned());
        }
    }
    ids
}

#[test]
fn world_marquee_mesh_cursor_matches_legacy_window_crossing_disjoint_and_degenerate_cases() {
    let state = world_marquee_geometry_fixture(1);
    let viewport = render_pick_viewport(&state.session);
    let view_projection = state.camera.orbit.to_camera().view_proj(800.0, 800.0);
    let mesh = state.scene.meshes.get("mesh").unwrap();
    let projected: Vec<_> = (0..3).map(|index| ui_scene::math::project_point(view_projection, world_mesh_vertex(*mesh, index).unwrap(), viewport.w, viewport.h).unwrap()).collect();
    let min_x = projected.iter().map(|point| point[0]).fold(f32::INFINITY, f32::min);
    let max_x = projected.iter().map(|point| point[0]).fold(f32::NEG_INFINITY, f32::max);
    let min_y = projected.iter().map(|point| point[1]).fold(f32::INFINITY, f32::min);
    let max_y = projected.iter().map(|point| point[1]).fold(f32::NEG_INFINITY, f32::max);
    let cases = [[[min_x - 2.0, min_y - 2.0], [max_x + 2.0, max_y + 2.0]], [[(min_x + max_x) * 0.5, min_y - 2.0], [min_x - 2.0, (min_y + max_y) * 0.5]], [[0.0, 0.0], [2.0, 2.0]], [[3.0, 3.0], [3.0, 3.0]]];
    for points in cases {
        let crossing = marquee_is_crossing_from_path(&points, false);
        let (meshes, draws) = scene_geometry(&state.scene);
        let legacy = screen_select_instances(&meshes, &draws, view_projection, ui_scene::math::default_projection_spec(), viewport.w, viewport.h, &points, true, crossing);
        let retained = world_marquee_cursor_ids(&state, &points);
        assert_eq!(retained, legacy, "points={points:?}");
    }
}

#[test]
fn world_marquee_mesh_cursor_preserves_legacy_multi_page_draw_order() {
    let state = world_marquee_geometry_fixture(WORLD_MARQUEE_RESULT_PAGE_CAPACITY + 1);
    let points = [[0.0, 0.0], [400.0, 400.0]];
    let view_projection = state.camera.orbit.to_camera().view_proj(800.0, 800.0);
    let (meshes, draws) = scene_geometry(&state.scene);
    let legacy = screen_select_instances(&meshes, &draws, view_projection, ui_scene::math::default_projection_spec(), 400.0, 400.0, &points, true, false);
    let retained = world_marquee_cursor_ids(&state, &points);
    assert_eq!(retained, legacy);
    assert_eq!(retained.len(), WORLD_MARQUEE_RESULT_PAGE_CAPACITY + 1);
}

#[test]
fn world_marquee_lasso_edge_cursor_matches_legacy_and_rejects_object_aba() {
    let mut state = world_marquee_geometry_fixture(1);
    state.selection.selection_method = "lasso".into();
    let viewport = render_pick_viewport(&state.session);
    let view_projection = state.camera.orbit.to_camera().view_proj(800.0, 800.0);
    let points = [[0.0, 0.0], [400.0, 0.0], [400.0, 400.0], [0.0, 400.0]];
    let (meshes, draws) = scene_geometry(&state.scene);
    let legacy = screen_select_instances(&meshes, &draws, view_projection, ui_scene::math::default_projection_spec(), viewport.w, viewport.h, &points, false, marquee_is_crossing_from_path(&points, true));
    assert_eq!(world_marquee_cursor_ids(&state, &points), legacy);

    let mut gesture = WorldMarqueeGesture::new(state.session.interaction_revision, 7, points[0]);
    for point in points.iter().skip(1) {
        assert!(gesture.push(*point));
    }
    let mut cursor = WorldMarqueePickCursor::new(&state.session, &state.camera, &state.selection, 8, gesture).unwrap();
    while cursor.current.is_none() {
        assert_eq!(with_world_step_context(1, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 8, context)), WorldInteractionStep::Pending);
    }
    let token = cursor.current.unwrap();
    let entry = *state.interaction.interaction_objects.resolve(token).unwrap();
    let mut replacement = entry.model;
    replacement.cols[3][0] += 1.0;
    let next = state.interaction.interaction_objects.admit(state.session.interaction_revision, WorldInteractionObjectKind::Instance, entry.id.as_str(), entry.mesh, replacement, entry.values, None).expect("replacement token");
    assert_ne!(next, token);
    assert_eq!(with_world_step_context(1, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 8, context)), WorldInteractionStep::Stale);
    let mut turns = 0;
    while !cursor.close_step() {
        turns += 1;
        assert!(turns < WORLD_INTERACTION_MARQUEE_POINT_CAPACITY + 4);
    }
    assert_eq!(cursor.gesture.len, 0);
}

fn world_component_marquee_cursor_ids(state: &World3dState, points: &[[f32; 2]]) -> Vec<u32> {
    let mut gesture = WorldMarqueeGesture::new(state.session.interaction_revision, 7, points[0]);
    for point in points.iter().skip(1) {
        assert!(gesture.push(*point));
    }
    let mut cursor = WorldMarqueePickCursor::new(&state.session, &state.camera, &state.selection, 8, gesture).expect("component marquee cursor");
    let mut turns = 0;
    loop {
        turns += 1;
        if with_world_step_context(1, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 8, context)) == WorldInteractionStep::Complete {
            break;
        }
        assert!(turns < 8_000);
    }
    let mut ids = Vec::new();
    for item in 0..usize::from(cursor.results.lens[0]) {
        let WorldMarqueeResult::Component { id, .. } = cursor.results.pages[0][item].expect("component marquee result") else {
            panic!("component marquee produced object result");
        };
        ids.push(id);
    }
    ids.sort_unstable();
    ids
}

#[test]
fn world_component_marquee_cursor_matches_legacy_vertex_edge_face_geometry() {
    let mut state = world_marquee_geometry_fixture_from(component_triangle_mesh_oracle(), 1);
    let viewport = render_pick_viewport(&state.session);
    let view_projection = state.camera.orbit.to_camera().view_proj(800.0, 800.0);
    let points = [[0.0, 0.0], [400.0, 400.0]];
    for granularity in ["vertex", "edge", "face"] {
        state.selection.granularity = granularity.into();
        let (meshes, draws) = scene_geometry(&state.scene);
        let mut legacy: Vec<u32> = screen_select_components(&meshes, &draws, view_projection, ui_scene::math::default_projection_spec(), viewport.w, viewport.h, &points, true, granularity, None, false).into_iter().map(|id| id.parse().expect("numeric component id")).collect();
        legacy.sort_unstable();
        assert!(!legacy.is_empty(), "the oracle must answer a component census, not an empty one: granularity={granularity}");
        assert_eq!(world_component_marquee_cursor_ids(&state, &points), legacy, "granularity={granularity}");
    }
}

#[test]
fn world_component_marquee_publish_preserves_shared_merge_and_exact_targets() {
    let mut state = world_marquee_geometry_fixture(1);
    state.selection.granularity = "vertex".into();
    state.selection.component_ids = vec!["7".into()];
    state.gumball.gumball_selection_ids = vec!["object-000.vertex.7".into()];
    state.selection.active_object_id = Some("object-000".into());
    let object = state.interaction.interaction_objects.instance_order[0].expect("instance token");
    let mut results = WorldMarqueeResultPages::default();
    assert!(results.push(WorldMarqueeResult::Component { object, id: 8 }, 0));
    let mut gesture = WorldMarqueeGesture::new(state.session.interaction_revision, 7, [0.0, 0.0]);
    assert!(gesture.push([400.0, 400.0]));
    let mut job = WorldComponentMarqueePublishJob::new(8, gesture, results, WorldComponentKind::Vertex, true, false);
    let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
    let mut turns = 0;
    loop {
        turns += 1;
        if with_world_step_context(1, |context| job.step(&state.session, &state.interaction, &state.scene, 8, &mut input, context)).expect("component marquee step") == WorldInteractionStep::Complete {
            break;
        }
        assert!(turns < 128);
    }
    let events = take_actions(&mut input);
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].action, "interactionSelect");
    let args = events[0].args.as_ref().unwrap();
    assert_eq!(args.get("merge").and_then(|value| value.as_str()), Some("additive"));
    let targets: serde_json::Value = serde_json::from_str(args.get("targets").and_then(|value| value.as_str()).unwrap()).unwrap();
    assert_eq!(targets, serde_json::json!([{"granularity":"vertex","id":"surface/object-000.vertex.8"}]));
    apply_world_action_preview(&mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene, &events[0]);
    assert_eq!(state.selection.component_ids, vec!["7".to_string(), "8".to_string()]);
    assert_eq!(job.results.page_len, 0);
    assert_eq!(job.gesture.len, 0);
}
