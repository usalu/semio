//! 🧪️ Surface context survives rerender and stays isolated from sibling surfaces and app instances.

use super::*;
use semio_framework_value::{RetainedCloneGrant,retirement::{RetireOwned,controlled::ControlledRetirement}};
/// 🗑️ Retires one owner through genuine original producer invocations, each granted exactly the cursor's own quote, because dropping an unretired owner abandons its ordered-map roots.
fn drain<T:RetireOwned>(value:T){
    let stock=RetainedCloneGrant{maximum_items:1<<20,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:1<<10};
    let mut bank=semio_framework_value_testing_admission::EffectBank::new(semio_framework_value_testing_admission::Wallet::new(stock,1<<30));
    let mut owner=ControlledRetirement::new(value).unwrap_or_else(|_|panic!("controlled custody"));
    let body=1<<24;
    for _ in 0..1<<20{
        if owner.terminal_is_empty(){return}
        let work=owner.next_computational_work(body).expect("retirement work");let d=owner.next_demand(body).expect("retirement demand");
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
        let(poll,_)=semio_framework_value_testing_admission::driven::<semio_framework_value::OriginalRetirementFailure>(&mut bank,grant,work.max(1),|effect,reply,recover|if recover{owner.recover_step_into(body,effect,reply)}else{owner.step_into(body,effect,reply)});
        bank.take_admitted();
        match poll{semio_framework_value::RetainedWorkPoll::Complete(turn)=>{turn.result.expect("retirement step succeeds");}semio_framework_value::RetainedWorkPoll::Pending{..}=>panic!("retirement step stayed pending")}
    }
    panic!("retirement reaches its terminal within the bound");
}

#[test]
fn auxiliary_context_leases_use_actual_windows_and_never_survive_source_replacement(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🪟️surface-context-lifecycle/🔣️.json")).unwrap();
    let leases:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/context-lease.json")).unwrap();
    let view:ViewModel=serde_json::from_value(fixture["view"].clone()).unwrap();let window=leases["window"].as_str().unwrap();let locale=serde_json::from_value(leases["locale"].clone()).unwrap();
    for row in leases["cases"].as_array().unwrap(){
        let mut contexts=SurfaceContexts::default();assert!(contexts.window_lease(window,locale).is_none());
        contexts.insert("7:left".into(),"graph".into(),view.for_window_instance(window).unwrap()).unwrap();
        let lease=contexts.window_lease(window,locale).unwrap();assert!(contexts.window_lease("missing",locale).is_none());
        match row["operation"].as_str().unwrap(){
            "unchanged"=>{},
            "sibling"=>contexts.insert("7:right".into(),"graph".into(),view.for_window_instance("right").unwrap()).unwrap(),
            "refresh"=>contexts.insert("7:left".into(),"graph".into(),view.for_window_instance(window).unwrap()).unwrap(),
            "update"=>contexts.update_view(&view),
            "locale"=>{let mut next=view.clone();next.locale=semio_ui_locale::Locale::En;contexts.update_view(&next);drain(next);},
            "close"=>contexts.remove("7:left"),
            "close-remount"=>{contexts.remove("7:left");contexts.insert("7:left".into(),"graph".into(),view.for_window_instance(window).unwrap()).unwrap();},
            "ambiguous"=>contexts.insert("alias".into(),"graph".into(),view.for_window_instance(window).unwrap()).unwrap(),
            "exhausted"=>{contexts.version=u64::MAX;contexts.update_view(&view);assert_eq!(contexts.version,u64::MAX);},
            _=>unreachable!(),
        }
        let current=contexts.borrow_window_lease(lease);assert_eq!(current.is_some(),row["valid"].as_bool().unwrap());
        if let Some(current)=current{assert_eq!(current.view_state.window_id.as_deref(),Some(window));assert_eq!(current.view_state.locale,locale);}
        drain(contexts);
    }
    drain(view);
}

#[test]
fn retained_surface_render_borrows_the_exact_mounted_view_without_materialization(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🪟️surface-context-lifecycle/🔣️.json")).unwrap();
    let view:ViewModel=serde_json::from_value(fixture["view"].clone()).unwrap();
    let surface=&fixture["surfaces"][0];let id=surface["id"].as_str().unwrap();let window=surface["windowId"].as_str().unwrap();let body=surface["bodyKey"].as_str().unwrap();
    let mut contexts=SurfaceContexts::default();contexts.insert(id.into(),body.into(),view.for_window_instance(window).unwrap()).unwrap();
    let binding=contexts.slots.iter().flatten().find(|binding|binding.surface==id).unwrap();
    for _ in 0..3{let actual=contexts.borrow(id).unwrap();assert!(std::ptr::eq(actual.view_state,binding.view_state.as_ref().unwrap()));assert!(std::ptr::eq(actual.body_key.as_ptr(),binding.body_key.as_ptr()));assert_eq!(actual.view_state.window_id.as_deref(),Some(window));}
    assert!(contexts.borrow("unmounted").is_none());
    drain(contexts);drain(view);
}

#[semio_framework_async_macros::async_test]
async fn surface_context_retains_host_preferences_and_window_identity() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🪟️surface-context-lifecycle/🔣️.json")).unwrap();
    let view: semio_framework::ViewModel = serde_json::from_value(fixture["view"].clone()).unwrap();
    let mut contexts = SurfaceContexts::default();
    let mut oracle = serde_json::Map::new();
    for surface in fixture["surfaces"].as_array().unwrap() {
        let id = surface["id"].as_str().unwrap();
        let projected = surface["windowId"].as_str().map(|window| view.for_window_instance(window).unwrap()).unwrap_or_else(|| view.clone());
        contexts.insert(id.into(), surface["bodyKey"].as_str().unwrap().into(), projected).unwrap();
        oracle.insert(id.into(), surface.clone());
    }
    for (id, expected) in &oracle {
        let actual = contexts.borrow(id).unwrap();
        assert_eq!(actual.body_key, expected["bodyKey"].as_str().unwrap());
        assert_eq!(actual.view_state.window_id.as_deref(), expected["windowId"].as_str());
        assert_eq!(actual.view_state.active_utility_id.as_deref(), expected["activeUtilityId"].as_str());
        assert_eq!(actual.view_state.locale, view.locale);
        assert_eq!(actual.view_state.terminology, view.terminology);
    }
    let mut updated = fixture["view"].clone();
    for (key, value) in fixture["update"].as_object().unwrap() {
        updated[key] = value.clone();
    }
    let updated: semio_framework::ViewModel = serde_json::from_value(updated).unwrap();
    contexts.update_view(&updated);
    for (id, expected) in &oracle {
        let actual = contexts.borrow(id).unwrap();
        assert_eq!(actual.view_state.locale, updated.locale);
        assert_eq!(actual.view_state.terminology, updated.terminology);
        assert_eq!(actual.view_state.window_id.as_deref(), expected["windowId"].as_str());
        let utility = expected["windowId"].as_str().and_then(|window| fixture["update"]["activeUtilityByWindowId"][window].as_str());
        assert_eq!(actual.view_state.active_utility_id.as_deref(), utility);
    }
    let hidden = fixture["hidden"].as_str().unwrap();
    contexts.remove(hidden);
    oracle.remove(hidden);
    assert!(contexts.borrow(hidden).is_none());
    assert!(contexts.borrow(fixture["survivor"].as_str().unwrap()).is_some());
    assert!(SurfaceContexts::default().borrow(fixture["survivor"].as_str().unwrap()).is_none());
    assert_eq!(contexts.len(), oracle.len());
    let mut closed = updated.clone();
    closed.window_instances.retain(|window| !fixture["closedWindowIds"].as_array().unwrap().iter().any(|id| id.as_str() == Some(window.id.as_str())));
    contexts.update_view(&closed);
    assert_eq!(contexts.len(), 1);
    assert!(contexts.borrow(fixture["panelSurvivor"].as_str().unwrap()).is_some());
    contexts.remove(fixture["panelSurvivor"].as_str().unwrap());
    assert_eq!(contexts.len(), 0);
    assert!(contexts.view_state.is_none());
    drain(contexts);drain(updated);drain(closed);drain(view);
}

#[test]
fn surface_context_capacity_reuses_closed_windows_without_partial_updates() {
    let mut contexts = SurfaceContexts::default();
    let mut view = ViewModel {
        window_instances: vec![semio_framework::ViewWindowInstance { id: "closed".into(), window_kind_id: "graph".into() }],
        window_id: Some("closed".into()),
        ..ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native)
    };
    for index in 0..semio_ui_contract::UI_RESIDENT_SLOTS {
        contexts.insert(index.to_string(), "graph".into(), view.clone()).unwrap();
    }
    assert!(contexts.insert("overflow".into(), "graph".into(), view.clone()).is_err());
    assert_eq!(contexts.len(), semio_ui_contract::UI_RESIDENT_SLOTS);
    view.window_instances.clear();
    view.window_id = None;
    contexts.insert("panel".into(), "properties".into(), view).unwrap();
    assert_eq!(contexts.len(), 1);
    assert!(contexts.borrow("panel").unwrap().view_state.window_id.is_none());
    drain(contexts);
}

/// 🧩️ A reserved section surface keeps the FULL host view it was mounted with — no window/panel
/// narrowing, and no dependence on which sibling surface mounted last — and no window closing ever
/// prunes it, because a section is keyed by no window at all.
#[test]
fn reserved_section_surfaces_keep_the_unnarrowed_view_and_outlive_their_windows() {
    let mut contexts = SurfaceContexts::default();
    let view = ViewModel {
        window_instances: vec![semio_framework::ViewWindowInstance { id: "left".into(), window_kind_id: "graph".into() }],
        window_id: Some("left".into()),
        active_tool_id: Some("fill".into()),
        ..ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native)
    };
    contexts.insert("7:left".into(), "graph".into(), view.for_window_instance("left").unwrap()).unwrap();
    for section in UiRefreshSection::ALL {
        contexts.insert(format!("7:{}", section.body_key()), section.body_key().into(), view.clone()).unwrap();
    }
    contexts.insert("7:panel".into(), "properties".into(), view.for_panel()).unwrap();
    for section in UiRefreshSection::ALL {
        let context = contexts.borrow(&format!("7:{}", section.body_key())).unwrap();
        assert_eq!(context.body_key, section.body_key());
        assert_eq!(context.view_state.window_id.as_deref(), Some("left"));
        assert_eq!(context.view_state.active_tool_id.as_deref(), Some("fill"));
        assert_eq!(context.view_state.window_instances.len(), 1);
    }
    let mut closed = view.clone();
    closed.window_instances.clear();
    contexts.update_view(&closed);
    assert!(contexts.borrow("7:left").is_none());
    for section in UiRefreshSection::ALL {
        assert!(contexts.borrow(&format!("7:{}", section.body_key())).is_some());
        assert!(contexts.is_section(&format!("7:{}", section.body_key())));
        contexts.remove(&format!("7:{}", section.body_key()));
        assert!(!contexts.is_section(&format!("7:{}", section.body_key())));
    }
    contexts.remove("7:panel");
    assert_eq!(contexts.len(), 0);
    drain(contexts);drain(closed);drain(view);
}

/// 🪟️ WAVE B56 LAW: the app's DEFAULT window surface is a real window INSTANCE and has host context;
/// the synthetic surface named `window` is refused outright.
///
/// 🦾️ This law used to pin the opposite for its second half: it mounted `7:window` and required a host
/// context for it, because leftover Viewport dirty defaulted the guest surface to the bare name
/// (wave W-G3 §8.29) and `windowHostContextBindings` aliased the last bound window onto it. That alias was
/// a fourteenth resident surface no pane reads, and the reconcile reservation it could not obtain left
/// every real surface deferred with the reactor answering `more-work` for ever
/// (`reserve_refusal=1:window:registry-reservation-unavailable`, wave B54 §6.4). The DEFAULT-window
/// semantics are kept exactly — a real default still resolves, projects and retains its own host view —
/// only the synthetic name is gone. Ticket 26/09/02/PUZZLE-3D-END-TO-END.
#[semio_framework_async_macros::async_test]
async fn default_window_surface_has_host_context_and_the_synthetic_name_is_refused() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🪟️surface-context-lifecycle/🔣️.json")).unwrap();
    let view: semio_framework::ViewModel = serde_json::from_value(fixture["view"].clone()).unwrap();
    let default = &fixture["defaultWindowSurface"];
    let mut contexts = SurfaceContexts::default();
    let projected = view.for_window_instance(default["windowId"].as_str().unwrap()).unwrap();
    contexts.insert(default["id"].as_str().unwrap().into(), default["bodyKey"].as_str().unwrap().into(), projected.clone()).unwrap();
    let actual = contexts.borrow(default["id"].as_str().unwrap()).expect("the app's default window surface must retain host context");
    assert_eq!(actual.body_key, default["bodyKey"].as_str().unwrap());
    assert_eq!(actual.view_state.window_id.as_deref(), default["windowId"].as_str());
    assert!(contexts.is_window(default["id"].as_str().unwrap()));
    let synthetic = default["syntheticId"].as_str().unwrap();
    let refused = contexts.insert(synthetic.into(), default["bodyKey"].as_str().unwrap().into(), projected).expect_err("the synthetic `window` surface must never mount for an app with window instances");
    assert_eq!(refused, default["syntheticRefusal"].as_str().unwrap());
    assert!(contexts.borrow(synthetic).is_none());
    assert_eq!(contexts.background_surfaces(), vec![default["id"].as_str().unwrap().to_owned()]);
    drain(contexts);drain(view);
}

/// 🪟️ WAVE B56 LAW: background work addresses EVERY mounted window instance, and an instance with no
/// window falls back to its app-level panels rather than a synthetic window.
///
/// 🦾️ The reactor's three background dirty sites (a job's progress, its completion, a document-backbone
/// message) minted `"<instance>:window"`, so the real panes never received the update at all. Ticket
/// 26/09/02/PUZZLE-3D-END-TO-END wave B56.
#[semio_framework_async_macros::async_test]
async fn background_surfaces_name_every_window_then_fall_back_to_panels() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🪟️surface-context-lifecycle/🔣️.json")).unwrap();
    let view: semio_framework::ViewModel = serde_json::from_value(fixture["view"].clone()).unwrap();
    let mut contexts = SurfaceContexts::default();
    let windows: Vec<String> = view.window_instances.iter().map(|window| format!("7:{}", window.id)).collect();
    for window in &view.window_instances {
        let projected = view.for_window_instance(&window.id).unwrap();
        contexts.insert(format!("7:{}", window.id), fixture["defaultWindowSurface"]["bodyKey"].as_str().unwrap().into(), projected).unwrap();
    }
    contexts.insert("7:panel".into(), "properties".into(), view.clone()).unwrap();
    assert_eq!(contexts.background_surfaces(), windows);
    for window in &windows {
        contexts.remove(window);
    }
    assert_eq!(contexts.background_surfaces(), vec!["7:panel".to_owned()]);
    assert!(!contexts.background_surfaces().iter().any(|surface| surface.ends_with(":window")));
    drain(contexts);drain(view);
}

/// ⚖️ A remote edit merged into the document re-projects EVERY bound surface: the windows and the
/// panels beside them (the peer's inspector stayed stale while `background_surfaces` named windows only).
#[semio_framework_async_macros::async_test]
async fn document_surfaces_name_every_window_and_every_panel() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🪟️surface-context-lifecycle/🔣️.json")).unwrap();
    let view: semio_framework::ViewModel = serde_json::from_value(fixture["view"].clone()).unwrap();
    let mut contexts = SurfaceContexts::default();
    let mut expected: Vec<String> = view.window_instances.iter().map(|window| format!("7:{}", window.id)).collect();
    for window in &view.window_instances {
        let projected = view.for_window_instance(&window.id).unwrap();
        contexts.insert(format!("7:{}", window.id), fixture["defaultWindowSurface"]["bodyKey"].as_str().unwrap().into(), projected).unwrap();
    }
    contexts.insert("7:panel".into(), "properties".into(), view.clone()).unwrap();
    expected.push("7:panel".to_owned());
    let mut surfaces = contexts.document_surfaces();
    surfaces.sort();
    expected.sort();
    assert_eq!(surfaces, expected);
    assert!(contexts.background_surfaces().len() < surfaces.len(), "background work still addresses windows only");
    drain(contexts);drain(view);
}
