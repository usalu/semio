//! 🎓️ The onboarding declarations and their playback engine: introduction steps, interactions, points, gestures and
//! demonstrations round-trip their tagged camelCase wire form; tutorials validate, interpolate cameras, compose UI state,
//! slice the timeline (against the language-agnostic `🧫️fixtures/🎞️tutorial-document-track.json`) and declare their
//! reserved start and record actions.

use super::*;
use semio_framework_manifest_arg_schema::ActionArgControl;
use semio_framework_manifest_address::window_element_id;

//#region 🔖️IntroductionTests
#[semio_framework_async_macros::async_test]
async fn introduction_step_serde_defaults() {
    let step: IntroductionStepDefinition =
        serde_json::from_str(r#"{"id":"welcome","title":{"native":{"en":"Welcome","de":"Welcome"},"reuse":{"en":"Welcome","de":"Welcome"}},"body":{"native":{"en":"Hi there","de":"Hi there"},"reuse":{"en":"Hi there","de":"Hi there"}}}"#).unwrap();
    assert_eq!(step.introduce, None);
    assert!(step.show.is_empty());
    let json = serde_json::to_string(&step).unwrap();
    let round: IntroductionStepDefinition = serde_json::from_str(&json).unwrap();
    assert_eq!(round, step);

    let with_targets = IntroductionStepDefinition::new("viewport", LocalizedLabel::data("The Viewport"), LocalizedLabel::data("…")).introduce(window_element_id("puzzle3d-main")).show(vec![window_element_id("puzzle3d-secondary")]);
    let json = serde_json::to_string(&with_targets).unwrap();
    assert!(json.contains("\"introduce\":\"framework.window.puzzle3dMain\""), "{json}");
    let round: IntroductionStepDefinition = serde_json::from_str(&json).unwrap();
    assert_eq!(round, with_targets);
}

#[semio_framework_async_macros::async_test]
async fn introduction_interaction_kind_round_trips_tagged() {
    for (kind, tag) in [
        (IntroductionInteractionKind::Action(ActionRef::new("add")), "action"),
        (IntroductionInteractionKind::Utility(UtilityRef::new("brush")), "utility"),
        (IntroductionInteractionKind::Tool(ToolRef::new("fill").await), "tool"),
        (IntroductionInteractionKind::Panel("framework.panel.catalogue".into()), "panel"),
        (IntroductionInteractionKind::Expand("puzzle3d-play-kinds.objects".into()), "expand"),
        (IntroductionInteractionKind::Pan("puzzle3d-main".into()), "pan"),
        (IntroductionInteractionKind::Zoom("puzzle3d-main".into()), "zoom"),
        (IntroductionInteractionKind::Orbit("puzzle3d-main".into()), "orbit"),
    ] {
        let json = serde_json::to_string(&kind).unwrap();
        assert!(json.contains(&format!("\"kind\":\"{tag}\"")), "{json}");
        let round: IntroductionInteractionKind = serde_json::from_str(&json).unwrap();
        assert_eq!(round, kind);
    }
}

#[semio_framework_async_macros::async_test]
async fn introduction_interaction_round_trips_and_defaults() {
    let interaction = IntroductionInteraction::zoom("puzzle3d-main", "Zoom in").await;
    assert_eq!(interaction.celebrate, None);
    let json = serde_json::to_string(&interaction).unwrap();
    assert!(!json.contains("celebrate"), "{json}");
    let round: IntroductionInteraction = serde_json::from_str(&json).unwrap();
    assert_eq!(round, interaction);

    let with_celebrate = IntroductionInteraction::pan("puzzle3d-main", "Pan").await.celebrate(window_element_id("puzzle3d-main")).await;
    let json = serde_json::to_string(&with_celebrate).unwrap();
    assert!(json.contains("\"celebrate\":\"framework.window.puzzle3dMain\""), "{json}");
    let round: IntroductionInteraction = serde_json::from_str(&json).unwrap();
    assert_eq!(round, with_celebrate);

    let step: IntroductionStepDefinition =
        serde_json::from_str(r#"{"id":"welcome","title":{"native":{"en":"Welcome","de":"Welcome"},"reuse":{"en":"Welcome","de":"Welcome"}},"body":{"native":{"en":"Hi there","de":"Hi there"},"reuse":{"en":"Hi there","de":"Hi there"}}}"#).unwrap();
    assert!(step.interactions.is_empty());
    assert!(!step.ordered);

    let with_interactions = IntroductionStepDefinition::new("viewport", LocalizedLabel::data("Viewport"), LocalizedLabel::data("…")).interact_ordered(vec![
        IntroductionInteraction::zoom("puzzle3d-main", "Zoom").await,
        IntroductionInteraction::pan("puzzle3d-main", "Pan").await,
        IntroductionInteraction::orbit("puzzle3d-main", "Orbit").await,
    ]);
    assert!(with_interactions.ordered);
    assert_eq!(with_interactions.interactions.len(), 3);
    let json = serde_json::to_string(&with_interactions).unwrap();
    let round: IntroductionStepDefinition = serde_json::from_str(&json).unwrap();
    assert_eq!(round, with_interactions);
}

#[semio_framework_async_macros::async_test]
async fn introduction_point_round_trips_tagged_camel_case() {
    for (point, tag) in [
        (IntroductionPoint::Element { id: "transform".into(), offset: None }, "element"),
        (IntroductionPoint::Element { id: "transform".into(), offset: Some([0.25, 0.75]) }, "element"),
        (IntroductionPoint::Screen { x: 10.0, y: 20.0 }, "screen"),
        (IntroductionPoint::ScreenNormalized { x: 0.5, y: 0.5 }, "screenNormalized"),
        (IntroductionPoint::Window { id: window_element_id("puzzle3d-main"), x: 40.0, y: 60.0 }, "window"),
        (IntroductionPoint::WindowNormalized { id: window_element_id("puzzle3d-main"), x: 0.5, y: 0.55 }, "windowNormalized"),
        (IntroductionPoint::Scene { id: window_element_id("puzzle3d-main"), position: [1.0, 2.0, 3.0] }, "scene"),
        (IntroductionPoint::Canvas { id: window_element_id("puzzle3d-main"), x: 12.0, y: 34.0 }, "canvas"),
        (IntroductionPoint::entity(window_element_id("puzzle3d-main"), "vortex", "seed-left-001:v0").await, "entity"),
        (IntroductionPoint::any_entity(window_element_id("puzzle3d-main"), "vortex").await, "entity"),
        (IntroductionPoint::Entity { id: window_element_id("puzzle3d-main"), domain: "node".into(), entity: "add".into(), offset: Some([0.25, 0.75]) }, "entity"),
        (IntroductionPoint::curve(window_element_id("puzzle3d-main"), "attraction", "a1", 0.5).await, "curve"),
        (IntroductionPoint::domain_value(window_element_id("puzzle3d-main"), "slider", "fillCount", 3.0).await, "domain"),
    ] {
        let json = serde_json::to_string(&point).unwrap();
        assert!(json.contains(&format!("\"kind\":\"{tag}\"")), "{json}");
        let round: IntroductionPoint = serde_json::from_str(&json).unwrap();
        assert_eq!(round, point);
    }
    // 🏷️ "*" (any-entity wildcard) must round-trip byte-for-byte, not get normalized away.
    let wildcard = IntroductionPoint::any_entity(window_element_id("puzzle3d-main"), "vortex").await;
    let json = serde_json::to_string(&wildcard).unwrap();
    assert!(json.contains("\"entity\":\"*\""), "{json}");
}

#[semio_framework_async_macros::async_test]
async fn introduction_gesture_round_trips_tagged_camel_case() {
    let at = IntroductionPoint::Element { id: "tool.fill".into(), offset: None };
    for (gesture, tag) in [
        (IntroductionGesture::LeftClick { at: at.clone() }, "leftClick"),
        (IntroductionGesture::RightClick { at: at.clone() }, "rightClick"),
        (IntroductionGesture::DoubleClick { at: at.clone() }, "doubleClick"),
        (IntroductionGesture::Drag { from: at.clone(), to: at.clone(), button: IntroductionPointerButton::Left, modifiers: vec![] }, "drag"),
        (IntroductionGesture::Scroll { at: at.clone(), delta_y: 100.0 }, "scroll"),
        (IntroductionGesture::Orbit { from: at.clone(), to: at.clone(), button: IntroductionPointerButton::Right, modifiers: vec![IntroductionKeyModifier::Alt] }, "orbit"),
    ] {
        let json = serde_json::to_string(&gesture).unwrap();
        assert!(json.contains(&format!("\"kind\":\"{tag}\"")), "{json}");
        let round: IntroductionGesture = serde_json::from_str(&json).unwrap();
        assert_eq!(round, gesture);
    }

    // 🐢️ `rename_all` on an enum renames only the variant tag, not fields *within* a struct variant —
    // `rename_all_fields` is required too, or this field would silently serialize snake_case
    // (`delta_y`) and desync from the generated TS type's camelCase `deltaY` (see `UiDirtyScope`).
    let scroll_json = serde_json::to_string(&IntroductionGesture::Scroll { at, delta_y: 100.0 }).unwrap();
    assert!(scroll_json.contains("\"deltaY\":100.0"), "{scroll_json}");
    assert!(!scroll_json.contains("delta_y"), "{scroll_json}");
}

#[semio_framework_async_macros::async_test]
async fn introduction_gesture_drag_orbit_default_button_and_modifiers() {
    let at = IntroductionPoint::Element { id: "puzzle3d-main".into(), offset: None };
    let drag: IntroductionGesture = serde_json::from_str(r#"{"kind":"drag","from":{"kind":"element","id":"puzzle3d-main"},"to":{"kind":"element","id":"puzzle3d-main"}}"#).unwrap();
    assert_eq!(drag, IntroductionGesture::Drag { from: at.clone(), to: at.clone(), button: IntroductionPointerButton::Left, modifiers: vec![] });
    // ⚖️ Defaults are still INFERRED on the way in (the input literal above names neither field),
    // but they are always WRITTEN on the way out: `🤖️generated/🪪️manifest/🟦️.ts` declares both
    // `button: IntroductionPointerButton` and `modifiers: Array<IntroductionKeyModifier>` as
    // required, so omitting a defaulted value handed the host `undefined` where its own types
    // promised a value. Asserting the omission — as this test previously did — pinned the defect.
    let drag_json = serde_json::to_string(&drag).unwrap();
    assert!(drag_json.contains("\"button\":\"left\""), "{drag_json}");
    assert!(drag_json.contains("\"modifiers\":[]"), "{drag_json}");

    let orbit: IntroductionGesture = serde_json::from_str(r#"{"kind":"orbit","from":{"kind":"element","id":"puzzle3d-main"},"to":{"kind":"element","id":"puzzle3d-main"}}"#).unwrap();
    assert_eq!(orbit, IntroductionGesture::Orbit { from: at.clone(), to: at.clone(), button: IntroductionPointerButton::Right, modifiers: vec![IntroductionKeyModifier::Alt] });
    let orbit_json = serde_json::to_string(&orbit).unwrap();
    assert!(orbit_json.contains("\"button\":\"right\""), "{orbit_json}");
    assert!(orbit_json.contains("\"modifiers\":[\"alt\"]"), "{orbit_json}");

    let middle_drag = IntroductionGesture::Drag { from: at.clone(), to: at, button: IntroductionPointerButton::Middle, modifiers: vec![] };
    let middle_json = serde_json::to_string(&middle_drag).unwrap();
    assert!(middle_json.contains("\"button\":\"middle\""), "{middle_json}");
    let round: IntroductionGesture = serde_json::from_str(&middle_json).unwrap();
    assert_eq!(round, middle_drag);
}

#[semio_framework_async_macros::async_test]
async fn introduction_demonstration_round_trips_and_defaults() {
    let at = IntroductionPoint::Element { id: "transform".into(), offset: None };
    let demo = IntroductionDemonstration::left_click(at.clone()).await;
    assert_eq!(demo.cursor, None);
    let json = serde_json::to_string(&demo).unwrap();
    assert!(!json.contains("cursor"), "{json}");
    let round: IntroductionDemonstration = serde_json::from_str(&json).unwrap();
    assert_eq!(round, demo);

    let with_cursor = IntroductionDemonstration { gesture: IntroductionGesture::Drag { from: at.clone(), to: at, button: IntroductionPointerButton::Left, modifiers: vec![] }, cursor: Some(IntroductionCursor::Grabbing) };
    let json = serde_json::to_string(&with_cursor).unwrap();
    assert!(json.contains("\"cursor\":\"grabbing\""), "{json}");
    let round: IntroductionDemonstration = serde_json::from_str(&json).unwrap();
    assert_eq!(round, with_cursor);

    let step: IntroductionStepDefinition =
        serde_json::from_str(r#"{"id":"welcome","title":{"native":{"en":"Welcome","de":"Welcome"},"reuse":{"en":"Welcome","de":"Welcome"}},"body":{"native":{"en":"Hi there","de":"Hi there"},"reuse":{"en":"Hi there","de":"Hi there"}}}"#).unwrap();
    assert!(step.demonstrations.is_empty());
    let json = serde_json::to_string(&step).unwrap();
    assert!(json.contains("\"demonstrations\":[]"), "{json}");

    // 🎬️ A step can sequence several demonstrations (e.g. zoom, then pan, then orbit).
    let with_demos = IntroductionStepDefinition::new("viewport", LocalizedLabel::data("Viewport"), LocalizedLabel::data("…")).demonstrate(vec![
        IntroductionDemonstration::scroll(IntroductionPoint::Screen { x: 400.0, y: 300.0 }, -100.0).await,
        IntroductionDemonstration::drag(IntroductionPoint::Screen { x: 300.0, y: 300.0 }, IntroductionPoint::Screen { x: 400.0, y: 320.0 }).await,
        IntroductionDemonstration::orbit(IntroductionPoint::Screen { x: 300.0, y: 300.0 }, IntroductionPoint::Screen { x: 500.0, y: 300.0 }).await,
    ]);
    assert_eq!(with_demos.demonstrations.len(), 3);
    let json = serde_json::to_string(&with_demos).unwrap();
    let round: IntroductionStepDefinition = serde_json::from_str(&json).unwrap();
    assert_eq!(round, with_demos);
}
//#endregion 🔖️IntroductionTests

//#region 🔖️TutorialTests
async fn minimal_tutorial() -> TutorialDefinition {
    TutorialDefinition {
        id: "welcome-tour".into(),
        title: LocalizedLabel::data("Welcome Tour"),
        description: None,
        duration_ms: 10_000,
        chapters: vec![TutorialChapter { id: "start".into(), at: 0, title: LocalizedLabel::data("Start"), body: None }],
        base: TutorialBase { document_dsl: None, example_id: Some("concrete-forest".into()), ui: TutorialUiSnapshot::default(), cameras: vec![] },
        tracks: TutorialTracks::default(),
        recorded_at: None,
    }
}

#[semio_framework_async_macros::async_test]
async fn tutorial_definition_serde_defaults() {
    let json = r#"{"id":"t","title":{"native":{"en":"T","de":"T"},"reuse":{"en":"T","de":"T"}},"durationMs":1000,"base":{"ui":{}},"tracks":{}}"#;
    let def: TutorialDefinition = serde_json::from_str(json).unwrap();
    assert!(def.description.is_none());
    assert!(def.chapters.is_empty());
    assert!(def.tracks.narration.is_empty());
    assert!(def.tracks.document.is_empty());
    assert!(def.base.cameras.is_empty());
    let round = serde_json::to_string(&def).unwrap();
    let round: TutorialDefinition = serde_json::from_str(&round).unwrap();
    assert_eq!(round, def);
}

#[semio_framework_async_macros::async_test]
async fn tutorial_asset_src_round_trips_tagged_camel_case() {
    for asset in
        [TutorialAssetSrc::Url { url: "https://example.test/clip.webm".into() }, TutorialAssetSrc::Blob { hash: "abc123".into(), size: 42, media_type: "video/webm".into() }, TutorialAssetSrc::DataUrl { data: "data:audio/webm;base64,AA==".into() }]
    {
        let json = serde_json::to_string(&asset).unwrap();
        assert!(json.contains("\"kind\":"), "{json}");
        let round: TutorialAssetSrc = serde_json::from_str(&json).unwrap();
        assert_eq!(round, asset);
    }
    let json = serde_json::to_string(&TutorialAssetSrc::Blob { hash: "abc".into(), size: 1, media_type: "video/webm".into() }).unwrap();
    assert!(json.contains("\"mediaType\""), "field must be camelCase: {json}");
}

#[semio_framework_async_macros::async_test]
async fn tutorial_event_kind_round_trips_tagged_camel_case() {
    let action = TutorialEventKind::Action { action: "addObjectKind".into(), args: Some(semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({"kindId": "beam"}))) };
    let json = serde_json::to_string(&action).unwrap();
    assert!(json.contains("\"kind\":\"action\""), "{json}");
    let round: TutorialEventKind = serde_json::from_str(&json).unwrap();
    assert_eq!(round, action);

    let key = TutorialEventKind::Key { keys: "mod+z".into() };
    let json = serde_json::to_string(&key).unwrap();
    let round: TutorialEventKind = serde_json::from_str(&json).unwrap();
    assert_eq!(round, key);
}

#[semio_framework_async_macros::async_test]
async fn tutorial_ui_change_round_trips_tagged_camel_case() {
    let change = TutorialUiChange::ActiveUtility { window_id: "puzzle3d-main".into(), utility_id: Some("transform".into()) };
    let json = serde_json::to_string(&change).unwrap();
    assert!(json.contains("\"windowId\":\"puzzle3d-main\""), "field must be camelCase: {json}");
    assert!(json.contains("\"utilityId\":\"transform\""), "field must be camelCase: {json}");
    let round: TutorialUiChange = serde_json::from_str(&json).unwrap();
    assert_eq!(round, change);

    let tree = TutorialUiChange::TreeExpansion { id: "puzzle3d-play-kinds.objects".into(), expanded: true };
    let json = serde_json::to_string(&tree).unwrap();
    let round: TutorialUiChange = serde_json::from_str(&json).unwrap();
    assert_eq!(round, tree);

    let selection = TutorialUiChange::Selection { domain_id: "mesh".into(), granularity: "face".into(), ids: vec!["f1".into(), "f2".into()] };
    let json = serde_json::to_string(&selection).unwrap();
    assert!(json.contains("\"domainId\":\"mesh\""), "field must be camelCase: {json}");
    assert!(json.contains("\"granularity\":\"face\""), "{json}");
    let round: TutorialUiChange = serde_json::from_str(&json).unwrap();
    assert_eq!(round, selection);
}

#[semio_framework_async_macros::async_test]
async fn tutorial_document_event_kind_round_trips_tagged_camel_case() {
    let edit = TutorialDocumentEventKind::Edit {
        forwards: vec![semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({"op": "translate"}))],
        backwards: vec![semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({"inverse": true, "op": "translate"}))],
        description: Some("Move object".into()),
    };
    let json = serde_json::to_string(&edit).unwrap();
    assert!(json.contains("\"kind\":\"edit\""), "{json}");
    assert!(json.contains("\"description\":\"Move object\""), "{json}");
    let round: TutorialDocumentEventKind = serde_json::from_str(&json).unwrap();
    assert_eq!(round, edit);

    let undo = TutorialDocumentEventKind::Undo;
    let json = serde_json::to_string(&undo).unwrap();
    assert_eq!(json, r#"{"kind":"undo"}"#);
}

#[semio_framework_async_macros::async_test]
async fn tutorial_camera_state_round_trips_tagged_camel_case() {
    let orbit = TutorialCameraState::Orbit { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], up: [0.0, 0.0, 1.0], fov: Some(50.0) };
    let json = serde_json::to_string(&orbit).unwrap();
    assert!(json.contains("\"kind\":\"orbit\""), "{json}");
    let round: TutorialCameraState = serde_json::from_str(&json).unwrap();
    assert_eq!(round, orbit);

    let canvas = TutorialCameraState::Canvas { x: 1.0, y: 2.0, zoom: 3.0 };
    let json = serde_json::to_string(&canvas).unwrap();
    assert!(json.contains("\"kind\":\"canvas\""), "{json}");
    let round: TutorialCameraState = serde_json::from_str(&json).unwrap();
    assert_eq!(round, canvas);
}

#[semio_framework_async_macros::async_test]
async fn validate_tutorial_rejects_unsorted_and_out_of_range_tracks() {
    let mut def = minimal_tutorial().await;
    def.tracks.narration = vec![
        TutorialNarrationCue { id: "b".into(), at: 500, duration_ms: 100, text: LocalizedLabel::data("b"), audio: None, voice: None, rate: 1.0, captions: vec![] },
        TutorialNarrationCue { id: "a".into(), at: 100, duration_ms: 100, text: LocalizedLabel::data("a"), audio: None, voice: None, rate: 1.0, captions: vec![] },
    ];
    assert!(validate_tutorial(&def).is_err(), "unsorted narration must be rejected");

    let mut def = minimal_tutorial().await;
    def.tracks.narration = vec![TutorialNarrationCue { id: "a".into(), at: 999_999, duration_ms: 100, text: LocalizedLabel::data("a"), audio: None, voice: None, rate: 1.0, captions: vec![] }];
    assert!(validate_tutorial(&def).is_err(), "entry beyond durationMs must be rejected");

    let mut def = minimal_tutorial().await;
    def.chapters.push(TutorialChapter { id: "start".into(), at: 0, title: LocalizedLabel::data("Dup"), body: None });
    assert!(validate_tutorial(&def).is_err(), "duplicate chapter id must be rejected");

    let mut def = minimal_tutorial().await;
    def.base.cameras.push(TutorialCameraKeyframe { at: 5, window_id: "w".into(), camera: TutorialCameraState::Canvas { x: 0.0, y: 0.0, zoom: 1.0 }, easing: TutorialEasing::default() });
    assert!(validate_tutorial(&def).is_err(), "base camera keyframe must be at == 0");

    assert!(validate_tutorial(&minimal_tutorial().await).is_ok());
}

#[semio_framework_async_macros::async_test]
async fn tutorial_camera_interpolation_lerps_position_and_target() {
    let prev = TutorialCameraKeyframe { at: 0, window_id: "w".into(), camera: TutorialCameraState::Orbit { position: [0.0, 0.0, 0.0], target: [0.0, 0.0, 0.0], up: [0.0, 0.0, 1.0], fov: Some(40.0) }, easing: TutorialEasing::Linear };
    let next = TutorialCameraKeyframe { at: 1000, window_id: "w".into(), camera: TutorialCameraState::Orbit { position: [10.0, 0.0, 0.0], target: [0.0, 0.0, 0.0], up: [0.0, 0.0, 1.0], fov: Some(60.0) }, easing: TutorialEasing::Linear };
    let mid = interpolate_tutorial_camera(&prev, &next, 500.0);
    match mid {
        TutorialCameraState::Orbit { position, fov, .. } => {
            assert!((position[0] - 5.0).abs() < 1e-9, "expected midpoint lerp, got {position:?}");
            assert_eq!(fov, Some(50.0));
        }
        other => panic!("expected Orbit, got {other:?}"),
    }
    let start = interpolate_tutorial_camera(&prev, &next, 0.0);
    assert_eq!(start, prev.camera);
    let end = interpolate_tutorial_camera(&prev, &next, 1000.0);
    assert_eq!(end, next.camera);
}

#[semio_framework_async_macros::async_test]
async fn tutorial_camera_interpolation_zooms_in_log_space() {
    let prev = TutorialCameraKeyframe { at: 0, window_id: "w".into(), camera: TutorialCameraState::Canvas { x: 0.0, y: 0.0, zoom: 1.0 }, easing: TutorialEasing::Linear };
    let next = TutorialCameraKeyframe { at: 1000, window_id: "w".into(), camera: TutorialCameraState::Canvas { x: 0.0, y: 0.0, zoom: 4.0 }, easing: TutorialEasing::Linear };
    let mid = interpolate_tutorial_camera(&prev, &next, 500.0);
    match mid {
        TutorialCameraState::Canvas { zoom, .. } => assert!((zoom - 2.0).abs() < 1e-9, "log-space midpoint of 1..4 is 2, got {zoom}"),
        other => panic!("expected Canvas, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn tutorial_camera_interpolation_hold_snaps_at_keyframe() {
    let prev = TutorialCameraKeyframe { at: 0, window_id: "w".into(), camera: TutorialCameraState::Canvas { x: 0.0, y: 0.0, zoom: 1.0 }, easing: TutorialEasing::Hold };
    let next = TutorialCameraKeyframe { at: 1000, window_id: "w".into(), camera: TutorialCameraState::Canvas { x: 0.0, y: 0.0, zoom: 4.0 }, easing: TutorialEasing::Hold };
    assert_eq!(interpolate_tutorial_camera(&prev, &next, 999.0), prev.camera);
    assert_eq!(interpolate_tutorial_camera(&prev, &next, 1000.0), next.camera);
}

#[semio_framework_async_macros::async_test]
async fn tutorial_camera_at_holds_first_pose_before_first_keyframe_and_last_pose_after() {
    let mut def = minimal_tutorial().await;
    def.tracks.camera = vec![
        TutorialCameraKeyframe { at: 100, window_id: "w".into(), camera: TutorialCameraState::Canvas { x: 0.0, y: 0.0, zoom: 1.0 }, easing: TutorialEasing::Linear },
        TutorialCameraKeyframe { at: 900, window_id: "w".into(), camera: TutorialCameraState::Canvas { x: 0.0, y: 0.0, zoom: 9.0 }, easing: TutorialEasing::Linear },
    ];
    assert_eq!(tutorial_camera_at(&def, "w", 0.0), Some(TutorialCameraState::Canvas { x: 0.0, y: 0.0, zoom: 1.0 }));
    assert_eq!(tutorial_camera_at(&def, "w", 10_000.0), Some(TutorialCameraState::Canvas { x: 0.0, y: 0.0, zoom: 9.0 }));
    assert_eq!(tutorial_camera_at(&def, "other-window", 500.0), None);
}

#[semio_framework_async_macros::async_test]
async fn compose_tutorial_ui_applies_snapshot_then_deltas() {
    let mut def = minimal_tutorial().await;
    def.base.ui.active_tool_id = Some("fill".into());
    def.tracks.ui = vec![
        TutorialUiKeyframe { at: 100, sample: TutorialUiSample::Snapshot { state: Box::new(TutorialUiSnapshot { active_mode_id: Some("edit".into()), ..Default::default() }) } },
        TutorialUiKeyframe { at: 200, sample: TutorialUiSample::Delta { changes: vec![TutorialUiChange::ActiveTool { id: Some("brush".into()) }] } },
        TutorialUiKeyframe { at: 300, sample: TutorialUiSample::Delta { changes: vec![TutorialUiChange::PanelTab { group: "top-left".into(), tab_id: Some("catalogue".into()) }] } },
        TutorialUiKeyframe { at: 400, sample: TutorialUiSample::Delta { changes: vec![TutorialUiChange::Selection { domain_id: "mesh".into(), granularity: "face".into(), ids: vec!["f1".into()] }] } },
    ];
    // Before any sample: the base snapshot alone.
    let at_0 = compose_tutorial_ui(&def, 0.0);
    assert_eq!(at_0.active_tool_id, Some("fill".into()));
    assert_eq!(at_0.active_mode_id, None);
    // After the snapshot but before its deltas.
    let at_100 = compose_tutorial_ui(&def, 100.0);
    assert_eq!(at_100.active_mode_id, Some("edit".into()));
    assert_eq!(at_100.active_tool_id, None, "snapshot replaces the base wholesale");
    // After one delta.
    let at_200 = compose_tutorial_ui(&def, 250.0);
    assert_eq!(at_200.active_tool_id, Some("brush".into()));
    // After both deltas.
    let at_300 = compose_tutorial_ui(&def, 300.0);
    assert_eq!(at_300.active_tool_id, Some("brush".into()));
    assert_eq!(at_300.active_panel_tab_by_group.get("top-left"), Some(&"catalogue".to_string()));
    // After the selection delta: the framework-owned domain selection lands in `interaction_selection`.
    let at_400 = compose_tutorial_ui(&def, 400.0);
    let selection = at_400.interaction_selection.get("mesh").expect("mesh domain selection");
    assert_eq!(selection.granularity, "face");
    assert_eq!(selection.ids, vec!["f1".to_string()]);
}

#[semio_framework_async_macros::async_test]
async fn tutorial_document_track_language_neutral_serde_parity() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🎞️tutorial-document-track.json")).unwrap();
    let mut def = minimal_tutorial().await;
    def.tracks.document = serde_json::from_value(fixture["document"].clone()).unwrap();
    assert_eq!(serde_json::to_value(&def.tracks.document).unwrap(), fixture["document"]);
    let tracks = serde_json::to_value(&def.tracks).unwrap();
    assert_eq!(tracks.get("document"), Some(&fixture["document"]));
    assert!(tracks.get("artifact").is_none());
    for vector in fixture["cases"].as_array().unwrap() {
        let slice = tutorial_slice(&def, vector["from"].as_f64().unwrap(), vector["to"].as_f64().unwrap());
        assert_eq!(slice.forward, vector["forward"].as_bool().unwrap());
        assert_eq!(serde_json::to_value(slice.document.iter().map(|event| event.at).collect::<Vec<_>>()).unwrap(), vector["expectedAt"]);
    }
}

#[semio_framework_async_macros::async_test]
async fn tutorial_slice_forward_and_reverse_cross_document_events() {
    let mut def = minimal_tutorial().await;
    def.tracks.document = vec![
        TutorialDocumentEvent {
            at: 100,
            kind: TutorialDocumentEventKind::Edit {
                forwards: vec![semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({"op": "add", "id": "a"}))],
                backwards: vec![semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({"op": "remove", "id": "a"}))],
                description: None,
            },
        },
        TutorialDocumentEvent {
            at: 200,
            kind: TutorialDocumentEventKind::Edit {
                forwards: vec![semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({"op": "add", "id": "b"}))],
                backwards: vec![semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({"op": "remove", "id": "b"}))],
                description: None,
            },
        },
    ];
    let forward = tutorial_slice(&def, 0.0, 250.0);
    assert!(forward.forward);
    assert_eq!(forward.document.len(), 2);
    let TutorialDocumentEventKind::Edit { forwards, .. } = &forward.document[0].kind else { panic!("expected Edit") };
    assert_eq!(forwards[0].get("id").and_then(DslValue::as_str), Some("a"), "forward order applies oldest-first");

    let backward = tutorial_slice(&def, 250.0, 0.0);
    assert!(!backward.forward);
    assert_eq!(backward.document.len(), 2);
    let TutorialDocumentEventKind::Edit { backwards, .. } = &backward.document[0].kind else { panic!("expected Edit") };
    assert_eq!(backwards[0].get("id").and_then(DslValue::as_str), Some("b"), "backward order unwinds newest-first");

    let empty = tutorial_slice(&def, 250.0, 250.0);
    assert!(empty.document.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn tutorial_slice_partitions_events_document_and_ui_by_track() {
    let mut def = minimal_tutorial().await;
    def.tracks.events = vec![TutorialEvent { at: 50, kind: TutorialEventKind::Action { action: "setFillCount".into(), args: None } }];
    def.tracks.ui = vec![TutorialUiKeyframe { at: 50, sample: TutorialUiSample::Delta { changes: vec![TutorialUiChange::ActiveTool { id: Some("fill".into()) }] } }];
    let slice = tutorial_slice(&def, 0.0, 100.0);
    assert_eq!(slice.events.len(), 1);
    assert_eq!(slice.ui_changes.len(), 1);
    assert!(slice.document.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn start_tutorial_action_definition_offers_declared_tutorials_as_select_options() {
    let action = start_tutorial_action_definition(std::slice::from_ref(&minimal_tutorial().await));
    assert_eq!(action.id, START_TUTORIAL_ACTION_ID);
    assert!(!action.in_palette, "shell owns palette discovery via the dedicated Play Tutorial command");
    assert_eq!(action.args.len(), 1);
    assert!(action.args[0].required);
    match action.args[0].control() {
        ActionArgControl::Select { options } => {
            assert_eq!(options.len(), 1);
            assert_eq!(options[0].value, "welcome-tour");
        }
        other => panic!("expected Select control, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn record_tutorial_action_definition_is_shell_intercepted_and_out_of_palette() {
    let action = record_tutorial_action_definition();
    assert_eq!(action.id, RECORD_TUTORIAL_ACTION_ID);
    assert!(!action.in_palette);
    assert_eq!(action.kind, ActionKind::View);
}
//#endregion 🔖️TutorialTests
