/// 🧲️ Laws of the face overlay invalidation, native component gumballs and native primary geometry.

/// 🔄️ Changing a nonactive exact face invalidates the original overlay without changing active projections.
#[test]
fn world_face_selection_overlay_invalidates_nonactive_source_changes() {
    let fixture:serde_json::Value=serde_json::from_str(read_static("../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🎯️component-selection-merges/🔣️.json")).unwrap();
    let row=&fixture["overlayLifecycle"];
    let mut mesh=fixture["mesh"].clone();mesh["faceIds"]=serde_json::json!([0,1]);mesh["componentReferences"]=serde_json::json!({"face":row["labels"]});
    let mut scene=scene_with_selection_and_domain("{}",Some(("geometry","object")));let world=scene.world_3d.as_mut().unwrap();
    world.meshes_json=serde_json::json!([{"id":"analytic","data":mesh}]).to_string();
    world.instances_json=serde_json::Value::Array(fixture["objects"].as_array().unwrap().iter().map(|object|serde_json::json!({"id":object,"meshId":"analytic","componentSource":{"handle":fixture["source"]["handle"],"revision":fixture["source"]["revision"]}})).collect()).to_string();
    let mut state=World3dState::new("surface".into(),"controller".into());drive_scene_bridge(&mut state,&scene,Rect::new(0.0,0.0,400.0,400.0));
    let mut stale:Option<WorldFaceOverlayMeshCursor>=None;
    for step in row["steps"].as_array().unwrap() {
        let selection=serde_json::json!({"ids":fixture["objects"],"gumballSelectionIds":step["targets"].as_array().unwrap().iter().map(|index|row["targets"][index.as_u64().unwrap() as usize].clone()).collect::<Vec<_>>(),"granularity":"face","componentIds":[0],"activeObjectId":fixture["objects"][0]}).to_string();
        let revision=state.session.interaction_revision;sync_world3d_scene_selection(&mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene,&selection);
        assert_eq!(state.session.interaction_revision,revision.wrapping_add(1),"exact original source selection advances once");
        sync_world3d_scene_selection(&mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene,&selection);assert_eq!(state.session.interaction_revision,revision.wrapping_add(1),"unchanged snapshot retains its revision");
        let equivalent=serde_json::to_string_pretty(&serde_json::from_str::<serde_json::Value>(&selection).unwrap()).unwrap();sync_world3d_scene_selection(&mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene,&equivalent);assert_eq!(state.session.interaction_revision,revision.wrapping_add(1),"equivalent original selection text preserves its owner revision");
        assert_eq!(state.selection.selected_ids,serde_json::from_value::<Vec<String>>(fixture["objects"].clone()).unwrap());assert_eq!(state.selection.component_ids,vec!["0"]);assert_eq!(state.selection.active_object_id.as_deref(),fixture["objects"][0].as_str());
        if let Some(mut cursor)=stale.take() {
            let mut retired=false;
            for _ in 0..1024 {if matches!(cursor.step(&state.session, &state.selection, &state.gumball, &state.scene, &state.overlay),WorldFaceOverlayMeshStep::Stale){retired=true;break;}}
            assert!(retired&&cursor.terminal_is_empty(),"old original overlay retires before publication");
        }
        let mut scan=WorldFaceOverlayScan::default();let mut groups=std::collections::BTreeMap::new();
        for turn in 0..4096 {
            match scan.step(&state.selection, &state.gumball, &state.scene) {
                WorldFaceOverlayScanStep::Complete=>break,
                WorldFaceOverlayScanStep::Pending=>assert!(turn<4095),
                WorldFaceOverlayScanStep::Match(hit)=>{let draw=state.scene.draws.get(usize::from(hit.draw)).unwrap();let instance=&draw.instances[usize::from(hit.instance)];groups.insert(instance.id.to_string(),world_mesh_component_id(*state.scene.meshes.get(&draw.mesh_key).unwrap(),Mesh3dField::FaceIds,hit.triangle));}
            }
        }
        assert_eq!(groups.values().copied().collect::<Vec<_>>(),serde_json::from_value::<Vec<u32>>(step["groups"].clone()).unwrap());
        let mut cursor=WorldFaceOverlayMeshCursor::new("surface",9000+state.session.interaction_revision,state.session.interaction_revision,state.scene.draw_generation).unwrap();
        for _ in 0..1024 {assert!(matches!(cursor.step(&state.session, &state.selection, &state.gumball, &state.scene, &state.overlay),WorldFaceOverlayMeshStep::Pending));if matches!(cursor.owner,WorldPlaceholderOwner::Writing(_)){break;}}
        assert!(matches!(cursor.owner,WorldPlaceholderOwner::Writing(_)),"the original overlay owns a physical partial mesh before supersession");stale=Some(cursor);
    }
    if let Some(mut cursor)=stale {for _ in 0..1024 {if cursor.close_step(){break;}}assert!(cursor.terminal_is_empty());}
    retire_bridged_surface(&mut state);
}

/// 🧽️ A fresh empty original selection snapshot clears each component and hover projection.
#[test]
fn world_scene_selection_snapshot_clears_empty_components_and_hover() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🎯️component-selection-merges/🔣️.json")).unwrap();
    let mut state = World3dState::new("surface".into(),"controller".into());
    let original = serde_json::json!({"ids":[fixture["objects"][0]],"gumballSelectionIds":[fixture["targets"][0]],"granularity":"face","componentIds":[0],"activeObjectId":fixture["objects"][0],"hoveredComponent":{"id":0,"objectId":fixture["objects"][0],"mode":"face"}}).to_string();
    sync_world3d_scene_selection(&mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene,&original);
    assert_eq!(state.selection.component_ids,vec!["0"]);assert!(state.selection.active_object_id.is_some());assert!(state.selection.hovered_component_id.is_some());
    let empty = fixture["steps"].as_array().unwrap().last().unwrap();
    let cleared = serde_json::json!({"ids":[],"granularity":"face","componentIds":empty["groups"],"activeObjectId":empty["active"],"hoveredComponent":null}).to_string();
    sync_world3d_scene_selection(&mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene,&cleared);
    assert!(state.selection.selected_ids.is_empty());
    assert!(state.gumball.gumball_selection_ids.is_empty());
    assert_eq!(state.selection.component_ids,serde_json::from_value::<Vec<String>>(empty["groups"].clone()).unwrap());
    assert_eq!(state.selection.active_object_id,empty["active"].as_str().map(str::to_string));
    assert_eq!(state.selection.hovered_component_id,fixture["clearedHover"]["id"].as_str().map(str::to_string));
    assert_eq!(state.selection.hovered_component_object_id,fixture["clearedHover"]["object"].as_str().map(str::to_string));
    assert_eq!(state.selection.hovered_component_mode,fixture["clearedHover"]["mode"].as_str().map(str::to_string));
}

/// 📍️ The published Scene pivot belongs to the existing gumball target owner and clears with it.
#[test]
fn world_native_component_gumball_retains_delivered_pivot() {
    let fixture:serde_json::Value=serde_json::from_str(read_static("../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🎯️component-selection-merges/🔣️.json")).unwrap();
    let mut state=World3dState::new("surface".into(),"controller".into());
    for row in fixture["gumball"]["pivotCases"].as_array().unwrap() {
        sync_world3d_scene_selection(&mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene,&serde_json::json!({"ids":[fixture["objects"][0]],"gumballSelectionIds":[fixture["targets"][0]],"granularity":"face","gumballTarget":row["pivot"]}).to_string());
        let expected=std::array::from_fn(|axis|row["pivot"][axis].as_f64().unwrap() as f32);
        assert_eq!(state.gumball.gumball_target,Some(expected));
    }
    sync_world3d_scene_selection(&mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene,"{}");assert_eq!(state.gumball.gumball_target,None);
}

/// 🛠️ Original component gumball capture and publication retain the guest source target lane.
#[test]
fn world_native_component_gumball_retains_original_source_targets() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🎯️component-selection-merges/🔣️.json")).unwrap();
    let row = &fixture["gumball"];let object = &fixture["objects"][row["object"].as_u64().unwrap() as usize];
    let expected = serde_json::Value::Array(row["ids"].as_array().unwrap().iter().map(|index|fixture["targets"][index.as_u64().unwrap() as usize].clone()).collect());
    for pivot_case in row["pivotCases"].as_array().unwrap() {
    for phase in row["phases"].as_array().unwrap() {
        let mut mesh = fixture["mesh"].clone();mesh["componentReferences"] = serde_json::json!({"face":[fixture["source"]["label"]]});
        let mut scene = scene_with_selection_and_domain("{}",Some(("geometry","object")));
        let world = scene.world_3d.as_mut().unwrap();
        world.meshes_json = serde_json::json!([{"id":"analytic","data":mesh}]).to_string();
        world.instances_json = serde_json::json!([{"id":object,"meshId":"analytic","selected":true,"componentSource":{"handle":fixture["source"]["handle"],"revision":fixture["source"]["revision"]}}]).to_string();
        world.camera_json = serde_json::json!({"position":[0,0,4],"target":[0,0,0],"up":[0,1,0],"fov":50}).to_string();
        world.selection_json = serde_json::json!({"ids":[object],"granularity":row["mode"],"componentIds":[0],"activeObjectId":object,"gumballSelectionIds":expected,"gumballTarget":pivot_case["pivot"],"transformMode":"move"}).to_string();
        let mut state = World3dState::new("surface".into(),"controller".into());
        drive_scene_bridge(&mut state,&scene,Rect::new(0.0,0.0,400.0,400.0));
        set_world3d_declared_actions(&mut state.session,&[TRANSLATE_SELECTION_ACTION_ID.into()]);
        let mut registry = WorldInteractionRegistryBuildCursor::new(state.session.interaction_revision);
        for turn in 0..128 {
            match with_world_step_context(1,|context|registry.step(&mut state.session, &mut state.interaction, &mut state.scene,context)) {
                WorldInteractionStep::Complete=>break,
                WorldInteractionStep::Pending=>assert!(turn<127),
                step=>panic!("component gumball registry {step:?}"),
            }
        }
        let mut pick = WorldGumballPickCursor::new(&state.session, &state.selection,12,pivot_case["pointer"][0].as_f64().unwrap() as f32,pivot_case["pointer"][1].as_f64().unwrap() as f32);
        for turn in 0..2048 {
            match with_world_step_context(1,|context|pick.step(&state.session, &state.camera, &state.gumball, &state.interaction, &state.scene,12,context)) {
                WorldInteractionStep::Complete=>break,
                WorldInteractionStep::Pending=>assert!(turn<2047),
                step=>panic!("component gumball capture {step:?}"),
            }
        }
        let mut gesture = pick.finish(&state.session, &state.gumball,12).unwrap_or_else(|_|panic!("current component gumball capture")).expect("actual component gumball handle");
        assert_eq!(gesture.handle,GumballHandle::MoveX);
        assert_eq!([gesture.pivot.x,gesture.pivot.y,gesture.pivot.z],std::array::from_fn::<_,3,_>(|axis|pivot_case["pivot"][axis].as_f64().unwrap() as f32));
        assert_eq!(gesture.begin_update(13,pivot_case["movedPointer"][0].as_f64().unwrap() as f32,pivot_case["movedPointer"][1].as_f64().unwrap() as f32),WorldInteractionStep::Pending);
        for turn in 0..128 {
            match gesture.update_step(&state.session, &state.camera, &state.gumball, &state.interaction, &state.scene) {
                WorldInteractionStep::Complete=>break,
                WorldInteractionStep::Pending=>assert!(turn<127),
                step=>panic!("component gumball motion {step:?}"),
            }
        }
        assert!((gesture.translate.x-row["delta"][0].as_f64().unwrap() as f32).abs()<1e-4);
        let stage = match phase.as_str().unwrap() {"once"=>WorldGumballPhase::Once,"stream"=>WorldGumballPhase::Stream,"commit"=>WorldGumballPhase::Commit,"abort"=>{gesture.translate=Vec3::ZERO;WorldGumballPhase::Abort(WorldCancelReason::Blur)},_=>unreachable!()};
        let action = world_gumball_publish(&state,gesture,stage).expect("original prepared gumball action");
        let args = action.args.as_ref().unwrap();
        assert_eq!(args.get("mode").and_then(|value|value.as_str()),row["mode"].as_str());
        let ids: Vec<&str> = args.get("ids").unwrap().as_array().unwrap().iter().map(|value|value.as_str().unwrap()).collect();
        assert_eq!(ids,expected.as_array().unwrap().iter().map(|value|value.as_str().unwrap()).collect::<Vec<_>>());
        retire_bridged_surface(&mut state);
    }
    }
}

/// 🕹️ Component gumballs use the original event authority through capture, motion, release and cancellation.
#[test]
fn world_native_component_gumball_uses_original_event_authority() {
    let fixture:serde_json::Value=serde_json::from_str(read_static("../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🎯️component-selection-merges/🔣️.json")).unwrap();
    let row=&fixture["gumball"];let object=&fixture["objects"][row["object"].as_u64().unwrap() as usize];
    let targets:Vec<String>=row["ids"].as_array().unwrap().iter().map(|index|fixture["targets"][index.as_u64().unwrap() as usize].as_str().unwrap().to_string()).collect();
    for case in row["authorityCases"].as_array().unwrap() {
        let mut mesh=fixture["mesh"].clone();mesh["componentReferences"]=serde_json::json!({"face":[fixture["source"]["label"]]});
        let mut scene=scene_with_selection_and_domain("{}",Some(("geometry","object")));let world=scene.world_3d.as_mut().unwrap();
        world.meshes_json=serde_json::json!([{"id":"analytic","data":mesh}]).to_string();
        world.instances_json=serde_json::json!([{"id":object,"meshId":"analytic","componentSource":{"handle":fixture["source"]["handle"],"revision":fixture["source"]["revision"]}}]).to_string();
        world.camera_json=serde_json::json!({"position":[0,0,4],"target":[0,0,0],"up":[0,1,0],"fov":50}).to_string();
        world.selection_json=serde_json::json!({"ids":[object],"granularity":row["mode"],"componentIds":[0],"activeObjectId":object,"gumballSelectionIds":targets,"gumballTarget":row["pivot"],"transformMode":"move","gumballLiveDispatch":case["live"]}).to_string();
        let mut state=World3dState::new("surface".into(),"controller".into());drive_scene_bridge(&mut state,&scene,Rect::new(0.0,0.0,400.0,400.0));set_world3d_declared_actions(&mut state.session,&[TRANSLATE_SELECTION_ACTION_ID.into()]);
        let mut input=semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();let mut actions=Vec::new();
        for event in 0..4 {
            let finish=case["finish"].as_str().unwrap();if event==3&&finish=="release" {continue;}
            let point=if event==0 {&row["pointer"]} else {&row["movedPointer"]};
            let intent=WorldInteractionIntent {phase:if event==1 {WorldInteractionPhase::PointerMove} else if event==2&&finish!="release" {WorldInteractionPhase::Cancel} else {WorldInteractionPhase::PointerButton},generation:0,x:point[0].as_f64().unwrap() as f32,y:point[1].as_f64().unwrap() as f32,dx:0.0,dy:0.0,delta:0.0,button:if event==2&&finish!="release" {if finish=="blur" {WorldCancelReason::Blur as i16} else {WorldCancelReason::CaptureLost as i16}} else {0},down:event<2,shift:false,ctrl:false,alt:false,meta:false};
            let generation=enqueue_world3d_event(&mut state.selection, &mut state.authority,intent).expect("original component event admission");
            let mut complete=false;
            for _ in 0..4096 {
                match with_world_step_context(1,|context|step_world3d_interaction(&mut state,generation,&mut input,context)) {
                    WorldInteractionAuthorityStep::Pending=>{},
                    WorldInteractionAuthorityStep::Complete|WorldInteractionAuthorityStep::Idle=>{complete=true;break;},
                    step=>panic!("original component event {event} {step:?}"),
                }
            }
            assert!(complete,"each original event terminates under one-unit grants");
            if event==0 {assert!(state.authority.interaction_authority.as_ref().unwrap().gumball.is_some(),"component mode captures its original gumball");}
            actions.extend(take_actions(&mut input).into_iter().filter(|action|action.action==TRANSLATE_SELECTION_ACTION_ID));
        }
        assert_eq!(actions.len(),case["phases"].as_array().unwrap().len());
        for (action,phase) in actions.iter().zip(case["phases"].as_array().unwrap()) {
            let args=action.args.as_ref().unwrap();assert_eq!(args.get("mode").and_then(|value|value.as_str()),row["mode"].as_str());assert_eq!(args.get("phase").and_then(|value|value.as_str()),phase.as_str());
            assert_eq!(args.get("ids").unwrap().as_array().unwrap().iter().map(|value|value.as_str().unwrap()).collect::<Vec<_>>(),targets.iter().map(String::as_str).collect::<Vec<_>>());
        }
        begin_world3d_interaction_close(&mut state.authority);
        for _ in 0..4096 {if with_world_step_context(1,|context|close_world3d_interaction_step(&mut state.authority,&mut input,context)){break;}}
        assert!(world3d_interaction_terminal_is_empty(&state.authority));while !input.close_step().unwrap(){}assert!(input.terminal_is_empty());retire_bridged_surface(&mut state);
    }
}

/// 🧮️ One object's source components consume original action credits rather than an arbitrary sixty-four object limit.
#[test]
fn world_native_component_gumball_captures_wide_source_selection() {
    let fixture:serde_json::Value=serde_json::from_str(read_static("../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🎯️component-selection-merges/🔣️.json")).unwrap();let row=&fixture["gumball"];
    let count=row["wideSelection"]["count"].as_u64().unwrap() as usize;let first=row["wideSelection"]["labelStart"].as_str().unwrap().parse::<u64>().unwrap();let object=fixture["objects"][0].as_str().unwrap();
    let labels:Vec<String>=(0..count).map(|index|(first+index as u64).to_string()).collect();let targets:Vec<String>=labels.iter().enumerate().map(|(index,label)|format!("{object}.face.{index}~{}~{label}~{}",fixture["source"]["handle"].as_str().unwrap(),fixture["source"]["revision"].as_str().unwrap())).collect();
    let mut mesh=fixture["mesh"].clone();mesh["indices"]=serde_json::json!((0..count).flat_map(|_|[0,1,2]).collect::<Vec<_>>());mesh["faceIds"]=serde_json::json!((0..count as u32).collect::<Vec<_>>());mesh["componentReferences"]=serde_json::json!({"face":labels});
    let mut scene=scene_with_selection_and_domain("{}",Some(("geometry","object")));let world=scene.world_3d.as_mut().unwrap();world.meshes_json=serde_json::json!([{"id":"analytic","data":mesh}]).to_string();world.instances_json=serde_json::json!([{"id":object,"meshId":"analytic","componentSource":{"handle":fixture["source"]["handle"],"revision":fixture["source"]["revision"]}}]).to_string();world.camera_json=serde_json::json!({"position":[0,0,4],"target":[0,0,0],"up":[0,1,0],"fov":50}).to_string();world.selection_json=serde_json::json!({"ids":[object],"granularity":"face","gumballSelectionIds":targets,"gumballTarget":row["pivot"],"transformMode":"move"}).to_string();
    let mut state=World3dState::new("surface".into(),"controller".into());drive_scene_bridge(&mut state,&scene,Rect::new(0.0,0.0,400.0,400.0));set_world3d_declared_actions(&mut state.session,&[TRANSLATE_SELECTION_ACTION_ID.into()]);
    let mut registry=WorldInteractionRegistryBuildCursor::new(state.session.interaction_revision);for _ in 0..4096 {if with_world_step_context(1,|context|registry.step(&mut state.session, &mut state.interaction, &mut state.scene,context))==WorldInteractionStep::Complete {break;}}
    let mut pick=WorldGumballPickCursor::new(&state.session, &state.selection,12,row["pointer"][0].as_f64().unwrap() as f32,row["pointer"][1].as_f64().unwrap() as f32);
    for _ in 0..16384 {if with_world_step_context(1,|context|pick.step(&state.session, &state.camera, &state.gumball, &state.interaction, &state.scene,12,context))==WorldInteractionStep::Complete {break;}}
    let mut gesture=pick.finish(&state.session, &state.gumball,12).unwrap_or_else(|_|panic!("current wide original source selection")).expect("actual wide gumball");assert_eq!(usize::from(gesture.selected_len),count);assert_eq!(usize::from(gesture.selected_bytes),targets.iter().map(String::len).sum::<usize>());
    gesture.translate=Vec3::new(1.0,0.0,0.0);let action=world_gumball_publish(&state,gesture,WorldGumballPhase::Once).expect("wide original bounded action");let args=action.args.as_ref().unwrap();assert_eq!(args.get("mode").and_then(|value|value.as_str()),Some("face"));assert_eq!(args.get("ids").unwrap().as_array().unwrap().iter().map(|value|value.as_str().unwrap()).collect::<Vec<_>>(),targets.iter().map(String::as_str).collect::<Vec<_>>());retire_bridged_surface(&mut state);
}

/// 🌊️ A live component gesture keeps its captured targets across a real guest mesh/source refresh.
#[test]
fn world_native_component_gumball_survives_live_guest_source_refresh() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🎯️component-selection-merges/🔣️.json")).unwrap();
    let row = &fixture["gumball"];
    let refresh = &row["liveRefresh"];
    let object = &fixture["objects"][row["object"].as_u64().unwrap() as usize];
    let targets: Vec<String> = row["ids"].as_array().unwrap().iter().map(|index| fixture["targets"][index.as_u64().unwrap() as usize].as_str().unwrap().to_string()).collect();
    let mut mesh = fixture["mesh"].clone();
    mesh["componentReferences"] = serde_json::json!({"face":[fixture["source"]["label"]]});
    let mut scene = scene_with_selection_and_domain("{}", Some(("geometry", "object")));
    let world = scene.world_3d.as_mut().unwrap();
    world.meshes_json = serde_json::json!([{"id":"analytic","data":mesh}]).to_string();
    world.instances_json = serde_json::json!([{"id":object,"meshId":"analytic","componentSource":{"handle":fixture["source"]["handle"],"revision":fixture["source"]["revision"]}}]).to_string();
    world.camera_json = refresh["camera"].to_string();
    world.selection_json = serde_json::json!({"ids":[object],"granularity":row["mode"],"componentIds":[0],"activeObjectId":object,"gumballSelectionIds":targets,"gumballTarget":row["pivot"],"transformMode":"move","gumballLiveDispatch":true}).to_string();
    let bounds = Rect::new(0.0, 0.0, refresh["viewport"][0].as_f64().unwrap() as f32, refresh["viewport"][1].as_f64().unwrap() as f32);
    let mut state = World3dState::new("surface".into(), "controller".into());
    drive_scene_bridge(&mut state, &scene, bounds);
    set_world3d_declared_actions(&mut state.session, &[TRANSLATE_SELECTION_ACTION_ID.into()]);
    let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
    let mut actions = Vec::new();
    for event in 0..refresh["pointers"].as_array().unwrap().len() {
        let point = &refresh["pointers"][event];
        let intent = WorldInteractionIntent { phase: if event == 0 || event == 3 { WorldInteractionPhase::PointerButton } else { WorldInteractionPhase::PointerMove }, generation: 0, x: point[0].as_f64().unwrap() as f32, y: point[1].as_f64().unwrap() as f32, dx: 0.0, dy: 0.0, delta: 0.0, button: 0, down: event < 3, shift: false, ctrl: false, alt: false, meta: false };
        let generation = enqueue_world3d_event(&mut state.selection, &mut state.authority, intent).expect("original live-refresh event admission");
        let mut complete = false;
        for _ in 0..4096 {
            match with_world_step_context(1, |context| step_world3d_interaction(&mut state, generation, &mut input, context)) {
                WorldInteractionAuthorityStep::Pending => {}
                WorldInteractionAuthorityStep::Complete | WorldInteractionAuthorityStep::Idle => { complete = true; break; }
                step => panic!("original live-refresh event {event} stopped at {step:?}"),
            }
        }
        assert!(complete, "original live-refresh event {event} terminates under one-unit grants");
        if event == 0 { assert!(state.authority.interaction_authority.as_ref().unwrap().gumball.is_some(), "original pointer captures a real component gumball"); }
        actions.extend(take_actions(&mut input).into_iter().filter(|action| action.action == TRANSLATE_SELECTION_ACTION_ID));
        if event == refresh["refreshAfter"].as_u64().unwrap() as usize {
            assert_eq!(actions.len(), 1, "the first stream precedes the actual guest publication");
            let generation = state.scene.geometry_generation;
            let revision = state.session.interaction_revision;
            for (index, position) in mesh["positions"].as_array_mut().unwrap().iter_mut().enumerate() {
                *position = serde_json::Value::from(position.as_f64().unwrap() + refresh["positions"][event][index % 3].as_f64().unwrap());
            }
            let world = scene.world_3d.as_mut().unwrap();
            world.meshes_json = serde_json::json!([{"id":"analytic","data":mesh}]).to_string();
            world.instances_json = serde_json::json!([{"id":object,"meshId":"analytic","componentSource":{"handle":fixture["source"]["handle"],"revision":refresh["revision"]}}]).to_string();
            drive_scene_bridge(&mut state, &scene, bounds);
            assert!(state.scene.geometry_generation > generation, "a real resident mesh was retired and replaced");
            assert!(state.session.interaction_revision > revision, "the actual guest refresh advanced the original view revision");
            assert!(state.bridge.scene_bridge_retired.is_none(), "the superseded snapshot lease finished bounded retirement");
            assert_eq!(state.gumball.gumball_selection_ids, targets, "the guest preserves the captured start selection");
            assert!(state.authority.interaction_authority.as_ref().unwrap().gumball.is_some(), "guest source publication retains the live gesture");
            let revision = state.session.interaction_revision;
            sync_world3d_state(&mut state, &scene, bounds);
            assert_eq!(with_world_step_context(64, |context| step_world3d_scene_bridge(&mut state, context)), World3dSceneBridgeStep::Idle, "an unchanged guest source stages no new bridge");
            assert_eq!(state.session.interaction_revision, revision, "an unchanged guest source does not advance view authority");
        }
    }
    let expected = refresh["dispatches"].as_array().unwrap();
    assert_eq!(actions.len(), expected.len(), "the refreshed gesture streams twice and commits its remaining tail");
    let mut offset = [0.0; 3];
    for (action, expected) in actions.iter().zip(expected) {
        let args = action.args.as_ref().unwrap();
        assert_eq!(args.get("mode").and_then(|value| value.as_str()), row["mode"].as_str());
        assert_eq!(args.get("phase").and_then(|value| value.as_str()), expected["phase"].as_str());
        assert_eq!(args.get("ids").unwrap().as_array().unwrap().iter().map(|value| value.as_str().unwrap()).collect::<Vec<_>>(), targets.iter().map(String::as_str).collect::<Vec<_>>());
        for (axis, key) in ["dx", "dy", "dz"].iter().enumerate() {
            let delta = args.get(*key).unwrap().as_f64().unwrap();
            assert!((delta - expected[*key].as_f64().unwrap()).abs() < 1e-4, "{key} is the incremental guest effect");
            offset[axis] += delta;
        }
    }
    for (axis, value) in offset.iter().enumerate() { assert!((value - refresh["offset"][axis].as_f64().unwrap()).abs() < 1e-4); }
    begin_world3d_interaction_close(&mut state.authority);
    for _ in 0..4096 { if with_world_step_context(1, |context| close_world3d_interaction_step(&mut state.authority, &mut input, context)) { break; } }
    assert!(world3d_interaction_terminal_is_empty(&state.authority));
    while !input.close_step().unwrap() {}
    assert!(input.terminal_is_empty());
    retire_bridged_surface(&mut state);
}

/// 🖱️ Primary points and wires remain selectable and hoverable in the original object domain.
#[test]
fn world_native_primary_geometry_object_pick_uses_original_domain() {
    let wire: serde_json::Value = serde_json::from_str(read_static("../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🎯️analytic-wire-picking/🔣️.json")).unwrap();
    let points: serde_json::Value = serde_json::from_str(read_static("../../../../../../../../🔨️modules/🧊️3d/📐️brep/⚙️engine/🧪️tests/🧫️fixtures/🎯️vertex-provenance/🔣️.json")).unwrap();
    let point = &points["cases"].as_array().unwrap().iter().find(|row|row["kind"] == "point").unwrap()["points"][0];
    let point_mesh = serde_json::json!({"positions":point,"indices":[],"vertexIds":[0],"edgePositions":[],"edgeIds":[]});
    let point_camera = serde_json::json!({"position":[point[0],point[1],point[2].as_f64().unwrap()+4.0],"target":point,"up":[0,1,0],"fov":50});
    let pointer = serde_json::json!([200,200]);
    for (name,mesh,camera,pointer,object) in [
        ("wire",&wire["mesh"],&wire["camera"],&wire["picks"][1]["pointer"],wire["instance"].as_str().unwrap()),
        ("edge-only",&wire["edgeOnly"]["mesh"],&wire["camera"],&wire["picks"][1]["pointer"],wire["instance"].as_str().unwrap()),
        ("point",&point_mesh,&point_camera,&pointer,"point@vertex#0"),
    ] {
        let mut scene = scene_with_selection_and_domain("{}",Some(("geometry","object")));
        let world = scene.world_3d.as_mut().unwrap();
        world.meshes_json = serde_json::json!([{"id":name,"data":mesh}]).to_string();
        world.instances_json = serde_json::json!([{"id":object,"meshId":name}]).to_string();
        world.camera_json = camera.to_string();
        world.selection_json = serde_json::json!({"granularity":"object","targets":{"mesh":true,"vertex":false,"edge":false,"face":false},"showEdges":false}).to_string();
        let mut state = World3dState::new(format!("surface-{name}"),"controller".into());
        drive_scene_bridge(&mut state,&scene,Rect::new(0.0,0.0,400.0,400.0));
        let mut registry = WorldInteractionRegistryBuildCursor::new(state.session.interaction_revision);
        for turn in 0..4096 {
            match with_world_step_context(1,|context|registry.step(&mut state.session, &mut state.interaction, &mut state.scene,context)) {
                WorldInteractionStep::Complete=>break,
                WorldInteractionStep::Pending=>assert!(turn<4095),
                step=>panic!("original primary registry {name} {step:?}"),
            }
        }
        for purpose in [WorldRayPickPurpose::Instance,WorldRayPickPurpose::Hover] {
            let mut cursor = WorldRayPickCursor::new(&state.session, &state.camera,10,purpose,pointer[0].as_f64().unwrap() as f32,pointer[1].as_f64().unwrap() as f32).unwrap();
            for turn in 0..4096 {
                match with_world_step_context(1,|context|cursor.step(&state.session, &state.selection, &state.interaction, &state.scene,10,context)) {
                    WorldInteractionStep::Complete=>break,
                    WorldInteractionStep::Pending=>assert!(turn<4095),
                    step=>panic!("original primary pick {name} {step:?}"),
                }
            }
            let mut plan = cursor.finish_plan(&state.session, &state.selection, &state.interaction, &state.scene,10).unwrap().expect("original primary plan");
            let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
            for turn in 0..256 {
                if with_world_step_context(1,|context|publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene,&mut plan,10,&mut input,context)).unwrap()==WorldInteractionStep::Complete {break;}
                assert!(turn<255);
            }
            let actions = take_actions(&mut input);assert_eq!(actions.len(),1);
            let args = actions[0].args.as_ref().unwrap();
            let targets: serde_json::Value = serde_json::from_str(args.get("targets").unwrap().as_str().unwrap()).unwrap();
            assert_eq!(targets,serde_json::json!([{"granularity":"object","id":object}]),"{name}: original primary object {purpose:?}");
            while !input.close_step().unwrap() {}
            assert!(input.terminal_is_empty());
        }
        retire_bridged_surface(&mut state);
    }
}

/// 🪡️ Original object picking orders overlapping sloping wires by their actual closest point.
#[test]
fn world_native_overlapping_wires_pick_original_closest_depth() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🎯️component-selection-merges/🔣️.json")).unwrap();
    let law = &fixture["gumball"]["overlappingWirePick"];
    let cases = fixture["gumball"]["raySegmentClosestCases"].as_array().unwrap();
    let rows = law["caseNames"].as_array().unwrap().iter().map(|name| cases.iter().find(|row| row["name"] == *name).unwrap()).collect::<Vec<_>>();
    let mut scene = scene_with_selection_and_domain("{}", Some(("geometry", "object")));
    let world = scene.world_3d.as_mut().unwrap();
    world.meshes_json = serde_json::Value::Array(rows.iter().enumerate().map(|(index, row)| serde_json::json!({"id":format!("wire-{index}"),"data":{"positions":[],"indices":[],"vertexIds":[],"edgePositions":row["a"].as_array().unwrap().iter().chain(row["b"].as_array().unwrap()).collect::<Vec<_>>(),"edgeIds":[0]}})).collect()).to_string();
    world.instances_json = serde_json::Value::Array(law["objects"].as_array().unwrap().iter().enumerate().map(|(index, id)| serde_json::json!({"id":id,"meshId":format!("wire-{index}")})).collect()).to_string();
    world.camera_json = law["camera"].to_string();
    world.selection_json = serde_json::json!({"granularity":"object","targets":{"mesh":true,"vertex":false,"edge":false,"face":false}}).to_string();
    let viewport: [f32; 4] = serde_json::from_value(law["viewport"].clone()).unwrap();
    let mut state = World3dState::new("surface-overlapping-wires".into(), "controller".into());
    drive_scene_bridge(&mut state, &scene, Rect::new(viewport[0], viewport[1], viewport[2], viewport[3]));
    let mut registry = WorldInteractionRegistryBuildCursor::new(state.session.interaction_revision);
    for turn in 0..4096 { match with_world_step_context(1, |context| registry.step(&mut state.session, &mut state.interaction, &mut state.scene, context)) { WorldInteractionStep::Complete => break, WorldInteractionStep::Pending => assert!(turn < 4095), step => panic!("original overlapping registry {step:?}") } }
    for purpose in [WorldRayPickPurpose::Instance, WorldRayPickPurpose::Hover] {
        let mut cursor = WorldRayPickCursor::new(&state.session, &state.camera, 10, purpose, law["pointer"][0].as_f64().unwrap() as f32, law["pointer"][1].as_f64().unwrap() as f32).unwrap();
        for turn in 0..4096 { match with_world_step_context(1, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 10, context)) { WorldInteractionStep::Complete => break, WorldInteractionStep::Pending => assert!(turn < 4095), step => panic!("original overlapping pick {step:?}") } }
        let mut plan = cursor.finish_plan(&state.session, &state.selection, &state.interaction, &state.scene, 10).unwrap().unwrap();
        let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
        for turn in 0..256 { if with_world_step_context(1, |context| publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut plan, 10, &mut input, context)).unwrap() == WorldInteractionStep::Complete { break; } assert!(turn < 255); }
        let actions = take_actions(&mut input);
        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0].action, if purpose == WorldRayPickPurpose::Hover { "interactionHover" } else { "interactionSelect" });
        if purpose == WorldRayPickPurpose::Instance { assert_eq!(actions[0].args.as_ref().unwrap().get("merge").and_then(|value| value.as_str()), Some("replace")); }
        let targets: serde_json::Value = serde_json::from_str(actions[0].args.as_ref().unwrap().get("targets").unwrap().as_str().unwrap()).unwrap();
        assert_eq!(targets, serde_json::json!([{"granularity":law["granularity"],"id":law["selectedObject"]}]), "original {purpose:?} chooses the actual near edge rather than its far midpoint");
        while !input.close_step().unwrap() {}
        assert!(input.terminal_is_empty());
    }
    state.selection.granularity = "edge".into();
    state.selection.selection_targets.edge = true;
    for purpose in [WorldComponentPickPurpose::Select, WorldComponentPickPurpose::Hover] {
        let mut cursor = WorldComponentPickCursor::new(&state.session, &state.camera, &state.selection, 10, purpose, law["pointer"][0].as_f64().unwrap() as f32, law["pointer"][1].as_f64().unwrap() as f32).unwrap();
        for turn in 0..4096 { match with_world_step_context(1, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 10, context)) { WorldInteractionStep::Complete => break, WorldInteractionStep::Pending => assert!(turn < 4095), step => panic!("original overlapping component pick {step:?}") } }
        let best = cursor.best.expect("original overlapping component hit");
        assert_eq!(state.interaction.interaction_objects.resolve(best.object).unwrap().id.as_str(), law["selectedObject"].as_str().unwrap());
        assert!((best.primary - 1.0).abs() < 1e-5);
        let mut plan = cursor.finish_plan(&state.session, &state.selection, &state.interaction, &state.scene, 10).unwrap().unwrap();
        let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
        for turn in 0..256 { if with_world_step_context(1, |context| publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut plan, 10, &mut input, context)).unwrap() == WorldInteractionStep::Complete { break; } assert!(turn < 255); }
        let actions = take_actions(&mut input);
        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0].action, if purpose == WorldComponentPickPurpose::Hover { "interactionHover" } else { "interactionSelect" });
        if purpose == WorldComponentPickPurpose::Select { assert_eq!(actions[0].args.as_ref().unwrap().get("merge").and_then(|value| value.as_str()), Some("replace")); }
        let targets: serde_json::Value = serde_json::from_str(actions[0].args.as_ref().unwrap().get("targets").unwrap().as_str().unwrap()).unwrap();
        assert_eq!(targets, serde_json::json!([{"granularity":"edge","id":format!("{}.edge.0", law["selectedObject"].as_str().unwrap())}]), "original component {purpose:?} publishes the actual near edge");
        while !input.close_step().unwrap() {}
        assert!(input.terminal_is_empty());
        while !cursor.close_step() {}
    }
    retire_bridged_surface(&mut state);
}

/// 🪟️ Original object and component cursors pick the visible portion of an authored clipped edge.
#[test]
fn world_native_clipped_wire_pick_preserves_original_visible_depth() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🎯️component-selection-merges/🔣️.json")).unwrap();
    let law = &fixture["gumball"]["clippedWirePick"];
    let mut scene = scene_with_selection_and_domain("{}", Some(("geometry", "object")));
    let world = scene.world_3d.as_mut().unwrap();
    world.meshes_json = serde_json::json!([{"id":"clipped-wire","data":{"positions":[],"indices":[],"vertexIds":[],"edgePositions":law["a"].as_array().unwrap().iter().chain(law["b"].as_array().unwrap()).collect::<Vec<_>>(),"edgeIds":[law["edgeId"]]}}]).to_string();
    world.instances_json = serde_json::json!([{"id":law["object"],"meshId":"clipped-wire"}]).to_string();
    world.camera_json = law["camera"].to_string();
    world.selection_json = serde_json::json!({"granularity":"object","targets":{"mesh":true,"vertex":false,"edge":true,"face":false}}).to_string();
    let viewport: [f32; 4] = serde_json::from_value(law["viewport"].clone()).unwrap();
    let mut state = World3dState::new("surface-clipped-wire".into(), "controller".into());
    drive_scene_bridge(&mut state, &scene, Rect::new(viewport[0], viewport[1], viewport[2], viewport[3]));
    let camera = state.camera.orbit.to_camera();
    assert_eq!(camera.near, law["camera"]["near"].as_f64().unwrap() as f32);
    assert_eq!(camera.far, law["camera"]["far"].as_f64().unwrap() as f32);
    let mut registry = WorldInteractionRegistryBuildCursor::new(state.session.interaction_revision);
    for turn in 0..4096 { match with_world_step_context(1, |context| registry.step(&mut state.session, &mut state.interaction, &mut state.scene, context)) { WorldInteractionStep::Complete => break, WorldInteractionStep::Pending => assert!(turn < 4095), step => panic!("original clipped registry {step:?}") } }
    let pointer = [law["pointer"][0].as_f64().unwrap() as f32, law["pointer"][1].as_f64().unwrap() as f32];
    for purpose in [WorldRayPickPurpose::Instance, WorldRayPickPurpose::Hover] {
        let mut cursor = WorldRayPickCursor::new(&state.session, &state.camera, 10, purpose, pointer[0], pointer[1]).unwrap();
        for turn in 0..4096 { match with_world_step_context(1, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 10, context)) { WorldInteractionStep::Complete => break, WorldInteractionStep::Pending => assert!(turn < 4095), step => panic!("original clipped object pick {step:?}") } }
        assert!((cursor.best.unwrap().distance - law["closestDepth"].as_f64().unwrap() as f32).abs() < 1e-5);
        let mut plan = cursor.finish_plan(&state.session, &state.selection, &state.interaction, &state.scene, 10).unwrap().unwrap();
        let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
        for turn in 0..256 { if with_world_step_context(1, |context| publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut plan, 10, &mut input, context)).unwrap() == WorldInteractionStep::Complete { break; } assert!(turn < 255); }
        let actions = take_actions(&mut input); assert_eq!(actions.len(), 1);
        assert_eq!(actions[0].action, if purpose == WorldRayPickPurpose::Hover { "interactionHover" } else { "interactionSelect" });
        let targets: serde_json::Value = serde_json::from_str(actions[0].args.as_ref().unwrap().get("targets").unwrap().as_str().unwrap()).unwrap();
        assert_eq!(targets, serde_json::json!([{"granularity":"object","id":law["object"]}]));
        while !input.close_step().unwrap() {}
        assert!(input.terminal_is_empty());
    }
    state.selection.granularity = "edge".into();
    for purpose in [WorldComponentPickPurpose::Select, WorldComponentPickPurpose::Hover] {
        let mut cursor = WorldComponentPickCursor::new(&state.session, &state.camera, &state.selection, 10, purpose, pointer[0], pointer[1]).unwrap();
        for turn in 0..4096 { match with_world_step_context(1, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 10, context)) { WorldInteractionStep::Complete => break, WorldInteractionStep::Pending => assert!(turn < 4095), step => panic!("original clipped component pick {step:?}") } }
        let best = cursor.best.unwrap(); assert_eq!(best.id, law["edgeId"].as_u64().unwrap() as u32);
        assert!((best.primary - law["closestDepth"].as_f64().unwrap() as f32).abs() < 1e-5);
        let mut plan = cursor.finish_plan(&state.session, &state.selection, &state.interaction, &state.scene, 10).unwrap().unwrap();
        let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
        for turn in 0..256 { if with_world_step_context(1, |context| publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut plan, 10, &mut input, context)).unwrap() == WorldInteractionStep::Complete { break; } assert!(turn < 255); }
        let actions = take_actions(&mut input); assert_eq!(actions.len(), 1);
        assert_eq!(actions[0].action, if purpose == WorldComponentPickPurpose::Hover { "interactionHover" } else { "interactionSelect" });
        let targets: serde_json::Value = serde_json::from_str(actions[0].args.as_ref().unwrap().get("targets").unwrap().as_str().unwrap()).unwrap();
        assert_eq!(targets, serde_json::json!([{"granularity":"edge","id":format!("{}.edge.{}", law["object"].as_str().unwrap(), law["edgeId"].as_u64().unwrap())}]));
        while !input.close_step().unwrap() {}
        assert!(input.terminal_is_empty());
        while !cursor.close_step() {}
    }
    for rectangle in law["marquees"].as_array().unwrap() {
        let point = |value: &serde_json::Value| [value[0].as_f64().unwrap() as f32, value[1].as_f64().unwrap() as f32];
        let mut gesture = WorldMarqueeGesture::new(state.session.interaction_revision, 11, point(&rectangle["points"][0]));
        assert!(gesture.push(point(&rectangle["points"][1])));
        let mut marquee = WorldMarqueePickCursor::new(&state.session, &state.camera, &state.selection, 12, gesture).unwrap();
        for turn in 0..4096 { match with_world_step_context(1, |context| marquee.step(&state.session, &state.selection, &state.interaction, &state.scene, 12, context)) { WorldInteractionStep::Complete => break, WorldInteractionStep::Pending => assert!(turn < 4095), step => panic!("original clipped marquee {step:?}") } }
        assert_eq!(marquee.results.lens[0], 1, "{} includes the visible clipped segment", rectangle["name"]);
        let mut publish = WorldComponentMarqueePublishJob::new(12, marquee.gesture, marquee.results, WorldComponentKind::Edge, false, false);
        let mut input = semio_framework_ui_wgpu_input::InputState::<ActionDescriptor>::default();
        for turn in 0..1024 { if with_world_step_context(1, |context| publish.step(&state.session, &state.interaction, &state.scene, 12, &mut input, context)).unwrap() == WorldInteractionStep::Complete { break; } assert!(turn < 1023); }
        let actions = take_actions(&mut input); assert_eq!(actions.len(), 1); assert_eq!(actions[0].action, "interactionSelect");
        let targets: serde_json::Value = serde_json::from_str(actions[0].args.as_ref().unwrap().get("targets").unwrap().as_str().unwrap()).unwrap();
        assert_eq!(targets, serde_json::json!([{"granularity":"edge","id":format!("{}.edge.{}", law["object"].as_str().unwrap(), law["edgeId"].as_u64().unwrap())}]));
        while !input.close_step().unwrap() {}
        assert!(input.terminal_is_empty());
    }
    retire_bridged_surface(&mut state);
}

/// 🎨️ Primary analytic geometry follows the original object-style and live-theme authority.
#[test]
fn world_native_primary_geometry_paint_uses_original_style_tokens() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../🧪️tests/🧫️fixtures/🎨️primary-geometry-style/🔣️.json")).unwrap();
    let geometry: serde_json::Value = serde_json::from_str(read_static("../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🎯️component-source/🔣️.json")).unwrap();
    let mut custom = semio_framework_ui_theme::Theme::default();
    custom.border_normal = semio_framework_ui_theme::Rgba::new(0.13,0.21,0.34,1.0);
    custom.border_emphasized = semio_framework_ui_theme::Rgba::new(0.55,0.34,0.21,1.0);
    custom.celebrate[0] = semio_framework_ui_theme::Rgba::new(0.21,0.55,0.34,1.0);
    for kind in fixture["geometry"].as_array().unwrap() {
        let mesh = geometry["zeroIndex"]["accepted"].as_array().unwrap().iter().find(|row|row["kind"] == *kind).unwrap();
        for row in fixture["states"].as_array().unwrap() {
            let object = format!("primary-{}",kind.as_str().unwrap());
            let mut scene = scene_with_selection("{}");let world = scene.world_3d.as_mut().unwrap();
            world.meshes_json = serde_json::json!([{"id":"primary","data":{"positions":mesh["positions"].as_array().unwrap().iter().flat_map(|point|point.as_array().unwrap()).collect::<Vec<_>>(),"indices":[],"vertexIds":mesh["vertexIds"],"edgePositions":mesh["edges"].as_array().unwrap().iter().flat_map(|edge|edge.as_array().unwrap()).flat_map(|point|point.as_array().unwrap()).collect::<Vec<_>>(),"edgeIds":mesh["edgeIds"]}}]).to_string();
            world.instances_json = serde_json::json!([{"id":object,"meshId":"primary"}]).to_string();
            world.selection_json = serde_json::json!({"granularity":"object","showEdges":false,"ids":if row["selected"].as_bool().unwrap() {vec![object.as_str()]} else {vec![]},"hoveredId":if row["hovered"].as_bool().unwrap() {Some(object.as_str())} else {None},"targets":{"mesh":true,"vertex":false,"edge":false,"face":false}}).to_string();
            let bounds = Rect::new(0.0,0.0,400.0,400.0);let mut state = World3dState::new("surface-style".into(),"controller".into());drive_scene_bridge(&mut state,&scene,bounds);
            for theme in [semio_framework_ui_theme::Theme::default(),custom.clone()] {
                let color = match row["line"].as_str().unwrap() {
                    "var(--border-normal-color)"=>theme.border_normal,
                    "var(--border-emphasized-color)"=>theme.border_emphasized,
                    "var(--color-primary)"=>theme.celebrate[0],
                    _=>panic!("neutral style token"),
                };
                let mut lines = Vec::new();append_component_overlays(&state.selection, &state.gumball, &state.scene, &state.overlay,&theme,&state.camera.orbit.to_camera(),bounds,&mut lines);
                assert!(!lines.is_empty());
                assert!(lines.iter().all(|line|line.color == [color.r,color.g,color.b,color.a]),"{kind}: {} follows the original primary line token",row["name"]);
            }
            retire_bridged_surface(&mut state);
        }
    }
}

/// 🧊️ Original zero-index scene leases reach actual GPU buffers without fabricated triangles.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn world_native_primary_geometry_gpu_upload_preserves_original_zero_indices() {
    let mut gpu = match semio_framework_async::block_on(ui_gpu::GpuContext::headless(1, 1)) {
        Ok(gpu) => gpu,
        Err(error) if error.starts_with("offscreen adapter:") => {  return; }
        Err(error) => panic!("original primary GPU initialization {error}"),
    };
    let wire: serde_json::Value = serde_json::from_str(read_static("../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🎯️analytic-wire-picking/🔣️.json")).unwrap();
    let points: serde_json::Value = serde_json::from_str(read_static("../../../../../../../../🔨️modules/🧊️3d/📐️brep/⚙️engine/🧪️tests/🧫️fixtures/🎯️vertex-provenance/🔣️.json")).unwrap();
    let point = &points["cases"].as_array().unwrap().iter().find(|row| row["kind"] == "point").unwrap()["points"][0];
    let point_mesh = serde_json::json!({"positions":point,"indices":[],"vertexIds":[0],"edgePositions":[],"edgeIds":[]});
    let mut results = Vec::new();
    for (name, mesh) in [("wire", &wire["mesh"]), ("edge-only", &wire["edgeOnly"]["mesh"]), ("point", &point_mesh)] {
        let mut scene = scene_with_selection("{}"); let world = scene.world_3d.as_mut().unwrap();
        world.meshes_json = serde_json::json!([{"id":name,"data":mesh}]).to_string();
        world.instances_json = serde_json::json!([{"id":name,"meshId":name}]).to_string();
        let mut state = World3dState::new("surface-gpu".into(), "controller".into());
        drive_scene_bridge(&mut state, &scene, Rect::new(0.0, 0.0, 400.0, 400.0));
        let lease = *state.scene.meshes.get(name).expect("original bridge published the primitive lease");
        assert_eq!(lease.schema().unwrap().indices, 0);
        let version = live_world3d_mesh_version(&state.scene, name);
        let mut result = Err("original GPU upload turn ceiling");
        for _ in 0..128 {
            match gpu.ensure_mesh_step(name, version, lease) {
                Ok(false) => {}
                Ok(true) => { result = Ok(()); break; }
                Err(error) => { result = Err(error); break; }
            }
        }
        if result.is_ok() {
            let resident = gpu.mesh_observation(name, version).unwrap();
            assert_eq!(resident.index_count, 0);
            assert_eq!(resident.index_bytes, 0, "an original primitive owns no fabricated index bytes");
        }
        for _ in 0..4096 { if gpu.close_mesh_upload_step() { break; } }
        assert!(gpu.mesh_upload_terminal_is_empty());
        for _ in 0..4096 { if gpu.retire_mesh_exact_step(name, version).unwrap() { break; } }
        assert!(gpu.mesh_observation(name, version).is_none());
        retire_bridged_surface(&mut state); results.push((name, result));
    }
    for _ in 0..4096 { if gpu.close_mesh_table_step() { break; } }
    assert!(gpu.mesh_table_terminal_is_empty());
    for (name, result) in results { assert!(result.is_ok(), "{name}: actual original GPU upload {result:?}"); }
}

fn pixel(rgba: &[u8], size: u32, x: u32, y: u32) -> [u8; 4] {
    let at = ((y * size + x) * 4) as usize;
    [rgba[at], rgba[at + 1], rgba[at + 2], rgba[at + 3]]
}

fn terrain_test_grant()->semio_framework_value::retained_clone::RetainedCloneGrant {semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:16*1024*1024,maximum_depth:64}}

pub(crate) fn terrain_test_retire_then_dynamic(state:&mut World3dState,context:&mut semio_framework_job::StepContext<'_>)->bool {
    let grant=terrain_test_grant();
    let terrain=step_world3d_terrain_retirement(&mut state.session, &mut state.terrain,grant,context);
    assert!(terrain.ownership.fits(grant));
    match terrain.step {
        WorldTerrainMeshPublicationStep::Idle=>step_world3d_dynamic_retirement(state,context),
        WorldTerrainMeshPublicationStep::Pending|WorldTerrainMeshPublicationStep::Complete=>false,
        WorldTerrainMeshPublicationStep::Fault(fault)=>panic!("typed terrain retirement refusal: {fault:?}"),
    }
}

fn world_original_fixture_grant()->semio_framework_value::RetainedCloneGrant{
    let law:serde_json::Value=serde_json::from_str(read_static("../../../🧪️tests/🔬️unit/🧫️retained-attributes.json")).unwrap();let axes=&law["grant"];
    semio_framework_value::RetainedCloneGrant{maximum_items:axes["items"].as_u64().unwrap()as usize,maximum_copy_bytes:axes["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:axes["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:axes["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:axes["depth"].as_u64().unwrap()as usize}
}
