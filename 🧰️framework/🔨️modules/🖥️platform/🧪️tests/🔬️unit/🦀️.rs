
use super::*;
use crate::ui::{ModeDefinition, WindowKindDefinition};
use ui_wgpu::wgpu::LocalizedLabel;

#[semio_framework_async_macros::async_test]
async fn adds_first_app_as_active() {
    let mut platform = Platform::new(None).await;
    platform
        .add_app(AppDefinition {
            id: "draw-play".into(),
            role: crate::ui::AppRole::Editor,
            dialect: crate::ArtifactDialect { artifact_kind: "s.test.draw-play".into(), standard: "1".into(), subset: "*".into() },
            label: LocalizedLabel::data("Draw"),
            breadcrumb: vec!["semio".into(), "draw".into()],
            icon_id: None,
            controller_id: "draw-play".into(),
            modes: crate::ui::Modes::one(ModeDefinition { id: "edit".into(), label: LocalizedLabel::data("Edit"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }),
            default_mode_id: "edit".into(),
            window_kinds: crate::ui::WindowKinds::one(WindowKindDefinition {
                id: "composite".into(),
                label: LocalizedLabel::data("Canvas"),
                body_key: "composite".into(),
                surface_kind: ui_wgpu::wgpu::SurfaceKind::Canvas2d,
                icon_id: "pen-tool".into(),
                options: ui_wgpu::wgpu::WindowOptions::default(),
                actions: Vec::new(),
                utilities: Vec::new(),
                interactions: Vec::new(),
                params_schema: None,
                artifact_snapshot_schema: None,
                input_event_schema: None,
                output_schema: None,
                capabilities: Vec::new(),
            }),
            panel_tabs: vec![],
            keybindings: vec![],
            utilities: vec![],
            tools: vec![],
            commands: vec![],
            interactions: Vec::new(),
            named_layouts: Vec::new(),
            default_layout: None,
            terminologies: Vec::new(),
            terminology_breadcrumbs: std::collections::HashMap::new(),
            introduction: None,
            tutorials: Vec::new(),
            dialogs: Vec::new(),
            media_inputs: Vec::new(),
            media_outputs: Vec::new(),
            artifact_kinds: Vec::new(),
            config: crate::ConfigSpec::empty().await,
            command_grammar: crate::CommandGrammar::empty().await,
            io: crate::AppIo::default(),
        })
        .await;
    assert_eq!(platform.active_app_id, "draw-play");
}

async fn minimal_app(id: &str) -> AppDefinition {
    AppDefinition {
        id: id.into(),
        role: crate::ui::AppRole::Editor,
        dialect: crate::ArtifactDialect { artifact_kind: format!("s.test.{id}"), standard: "1".into(), subset: "*".into() },
        label: LocalizedLabel::data(id),
        breadcrumb: vec!["semio".into(), id.into()],
        icon_id: None,
        controller_id: id.into(),
        modes: crate::ui::Modes::one(ModeDefinition { id: "edit".into(), label: LocalizedLabel::data("Edit"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }),
        default_mode_id: "edit".into(),
        window_kinds: crate::ui::WindowKinds::one(WindowKindDefinition {
            id: "main".into(),
            label: LocalizedLabel::data("Main"),
            body_key: "main".into(),
            surface_kind: ui_wgpu::wgpu::SurfaceKind::Canvas2d,
            icon_id: "pen-tool".into(),
            options: ui_wgpu::wgpu::WindowOptions::default(),
            actions: Vec::new(),
            utilities: Vec::new(),
            interactions: Vec::new(),
            params_schema: None,
            artifact_snapshot_schema: None,
            input_event_schema: None,
            output_schema: None,
            capabilities: Vec::new(),
        }),
        panel_tabs: vec![],
        keybindings: vec![],
        utilities: vec![],
        tools: vec![],
        commands: vec![],
        interactions: Vec::new(),
        named_layouts: Vec::new(),
        default_layout: None,
        terminologies: Vec::new(),
        terminology_breadcrumbs: std::collections::HashMap::new(),
        introduction: None,
        tutorials: Vec::new(),
        dialogs: Vec::new(),
        media_inputs: Vec::new(),
        media_outputs: Vec::new(),
        artifact_kinds: Vec::new(),
        config: crate::ConfigSpec::empty().await,
        command_grammar: crate::CommandGrammar::empty().await,
        io: crate::AppIo::default(),
    }
}

#[semio_framework_async_macros::async_test]
async fn set_active_app_id_is_noop_when_unchanged() {
    let mut platform = Platform::new(None).await;
    platform.add_app(minimal_app("draw").await).await;
    let generation_before = platform.chrome_generation;
    platform.set_active_app_id("draw".into()).await;
    assert_eq!(platform.chrome_generation, generation_before, "same id must not bump chrome_generation");
    platform.set_active_app_id("other".into()).await;
    assert_eq!(platform.chrome_generation, generation_before + 1);
    assert_eq!(platform.active_app_id, "other");
}

#[semio_framework_async_macros::async_test]
async fn get_active_app_falls_back_to_first_when_active_id_unknown() {
    let mut platform = Platform::new(None).await;
    platform.add_app(minimal_app("draw").await).await;
    platform.active_app_id = "missing".into();
    assert_eq!(platform.get_active_app().await.unwrap().id, "draw");
}

#[semio_framework_async_macros::async_test]
async fn set_panel_visibility_is_noop_when_unchanged_else_bumps_chrome_generation() {
    let mut platform = Platform::new(None).await;
    let generation_before = platform.chrome_generation;
    platform.set_panel_visibility(PanelVisibility::default()).await;
    assert_eq!(platform.chrome_generation, generation_before, "same visibility must not bump generation");
    platform.set_panel_visibility(PanelVisibility { left_side_panel: true, right_side_panel: false }).await;
    assert_eq!(platform.chrome_generation, generation_before + 1);
}

#[semio_framework_async_macros::async_test]
async fn notify_and_notify_chrome_increment_independently() {
    let mut platform = Platform::new(None).await;
    platform.notify().await;
    platform.notify().await;
    platform.notify_chrome().await;
    assert_eq!(platform.generation, 2);
    assert_eq!(platform.chrome_generation, 1);
}
