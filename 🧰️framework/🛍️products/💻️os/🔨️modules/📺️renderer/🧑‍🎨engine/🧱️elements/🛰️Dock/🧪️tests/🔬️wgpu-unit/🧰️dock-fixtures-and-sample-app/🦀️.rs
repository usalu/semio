/// 🧰️ Dock fixtures and sample app: the tab and stack builders and the sample app definition with its windows, layouts and panels.

fn tab(id: &str) -> DockStackTab {
    DockStackTab::new(id)
}

fn tabs(ids: &[&str]) -> Vec<DockStackTab> {
    ids.iter().map(|id| DockStackTab::new(*id)).collect()
}

fn stack_tabs(ids: &[&str], active: &str) -> DockNode {
    DockNode::Stack { windows: tabs(ids), active: active.into() }
}

fn sample_app(window_ids: &[&str], layout: Option<WindowLayout>) -> AppDefinition {
    AppDefinition {
        id: "test".into(),
        role: AppRole::Editor,
        dialect: ArtifactDialect { artifact_kind: "s.test.dock".into(), standard: "1".into(), subset: "*".into() },
        label: LocalizedLabel::data("Test"),
        breadcrumb: vec!["semio".into(), "test".into()],
        icon_id: None,
        controller_id: "test".into(),
        modes: semio_framework::Modes::one(ModeDefinition { id: "default".into(), label: LocalizedLabel::data("Default"), icon_id: "pencil".into(), tools: vec![], layout_id: None, commands: vec![] }),
        default_mode_id: "default".into(),
        window_kinds: semio_framework::WindowKinds::try_from(
            window_ids
                .iter()
                .map(|id| WindowKindDefinition {
                    initial_utility_id: None,
                    id: (*id).into(),
                    label: LocalizedLabel::data(*id),
                    body_key: format!("{id}.body"),
                    surface_kind: semio_framework_ui_declaration::ui::SurfaceKind::Canvas2d,
                    icon_id: "app-window".into(),
                    options: WindowOptions::default(),
                    actions: vec![],
                    interactions: vec![],
                    utilities: vec![],
                    params_schema: None,
                    artifact_snapshot_schema: None,
                    input_event_schema: None,
                    output_schema: None,
                    capabilities: vec![],
                })
                .collect::<Vec<_>>(),
        )
        .expect("sample_app tests always pass at least one window id"),
        panel_tabs: vec![PanelTabDefinition { kind: PanelTabKind::App("tab".into()), label: LocalizedLabel::data("Tab"), group: PanelGroup::Workbench, body_key: Some("tab.body".into()), children: vec![] }],
        keybindings: vec![],
        actions: vec![],
        interactions: vec![],
        utilities: vec![],
        tools: vec![],
        commands: vec![],
        named_layouts: vec![],
        default_layout: layout,
        terminologies: vec![],
        terminology_breadcrumbs: HashMap::new(),
        introduction: None,
        tutorials: Vec::new(),
        dialogs: Vec::new(),
        media_inputs: Vec::new(),
        media_outputs: Vec::new(),
        artifact_kinds: Vec::new(),
        config: semio_framework_async::block_on(semio_framework::ConfigSpec::empty()),
        command_grammar: semio_framework_async::block_on(semio_framework::CommandGrammar::empty()),
        io: semio_framework::AppIo::default(),
        fault_notices: Vec::new(),
    }
}
