use super::*;

struct NonDefaultState {
    value: u32,
}

#[test]
fn worker_cell_owned_initializer_supports_non_default_state_and_retains_one_instance() {
    let cell = WorkerCell::new(|| NonDefaultState { value: 73 });
    assert_eq!(cell.borrow().value, 73);
    cell.borrow_mut().value = 109;
    assert_eq!(cell.borrow().value, 109);
    assert!(std::ptr::eq(cell.state(), cell.state()));
}

#[test]
fn worker_cell_test_thread_states_are_isolated_by_cell_address_and_thread_identity() {
    static CELL: WorkerCell<NonDefaultState> = WorkerCell::new(|| NonDefaultState { value: 17 });
    CELL.borrow_mut().value = 33;
    let child = std::thread::spawn(|| {
        assert_eq!(CELL.borrow().value, 17);
        CELL.borrow_mut().value = 88;
        CELL.borrow().value
    });
    assert_eq!(child.join().unwrap(), 88);
    assert_eq!(CELL.borrow().value, 33);
}

#[test]
fn retained_ui_unadmitted_read_refuses_with_the_named_locale_fault() {
    let cell = UiEngineCell(WorkerCell::new(|| None));
    assert!(cell.0.borrow().is_none());
    let refusal = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let engine = cell.borrow();
        let _ = std::ops::Deref::deref(&engine);
    }))
    .expect_err("unadmitted retained UI cannot be read");
    let message = refusal.downcast_ref::<String>().map(String::as_str).or_else(|| refusal.downcast_ref::<&str>().copied());
    assert_eq!(message, Some(UI_ENGINE_LOCALE_UNRESOLVED));
}

#[test]
fn retained_ui_locale_readmission_preserves_the_actual_surface_tree_and_revision() {
    let node = UiNode::Text(ui_wgpu::wgpu::UiTextNode {
        value: semio_framework_ui_locale::Label::data("retained document"),
        emphasize: None,
        data_attributes: None,
        presence: ui_wgpu::wgpu::UiPresence::default(),
        menu: None,
    });
    install_ui_engine_locale(semio_framework_ui_locale::Locale::De);
    UI_ENGINE.with(|cell| cell.borrow_mut().apply_tree("locale-owner", &node));
    let before = UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        (engine.surface_token("locale-owner"), engine.tree_revision("locale-owner"), engine.tree("locale-owner").unwrap().root)
    });
    assert!(before.0.is_some());
    assert!(before.2.is_some());
    for locale in [semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Locale::De] {
        install_ui_engine_locale(locale);
        UI_ENGINE.with(|cell| {
            let engine = cell.borrow();
            assert_eq!((engine.surface_token("locale-owner"), engine.tree_revision("locale-owner"), engine.tree("locale-owner").unwrap().root), before);
            assert_eq!(engine.tree("locale-owner").unwrap().node(before.2.unwrap()).unwrap().spec.0, node);
        });
    }
}
