use super::*;
use semio_framework_plugin::{ActionBinding, Component, TreeWindowRequest, UiValue, ViewModel};

fn node_by_key<'a>(node: &'a BuiltNode, key: &str) -> Option<&'a BuiltNode> {
    if node.key.as_str() == key {
        return Some(node);
    }
    node.children.iter().find_map(|child| node_by_key(child, key))
}

/// 🎬️ What `node` dispatches: its record bindings, and — for a table row — each row action's verb on the row's ONE target.
fn own_bindings(node: &BuiltNode) -> Vec<ActionBinding> {
    let row_verbs = match &node.component {
        Component::TableRow(props) => props.target.as_ref().map_or_else(Vec::new, |target| props.row_actions.iter().map(|row_action| target.binding(&row_action.verb).expect("credited row binding")).collect()),
        _ => Vec::new(),
    };
    node.bindings.iter().map(|binding| binding.credited_clone().expect("credited record binding")).chain(row_verbs).collect()
}

fn binding_named(node: &BuiltNode, action: &str) -> Option<ActionBinding> {
    own_bindings(node).into_iter().find(|binding| binding.action.name.as_str() == action).or_else(|| node.children.iter().find_map(|child| binding_named(child, action)))
}

fn node_with_binding<'a>(node: &'a BuiltNode, action: &str) -> Option<&'a BuiltNode> {
    own_bindings(node).iter().any(|binding| binding.action.name.as_str() == action).then_some(node).or_else(|| node.children.iter().find_map(|child| node_with_binding(child, action)))
}

fn number_arg(binding: &ActionBinding, key: &str) -> Option<f64> {
    let Some(UiValue::Map(arguments)) = &binding.args else { return None };
    arguments.iter().find_map(|(name, value)| (name.as_str() == key).then_some(value)).and_then(|value| match value {
        UiValue::Number(value) => Some(value),
        _ => None,
    })
}

fn text_arg(binding: &ActionBinding, key: &str) -> Option<String> {
    let Some(UiValue::Map(arguments)) = &binding.args else { return None };
    arguments.iter().find_map(|(name, value)| (name.as_str() == key).then_some(value)).and_then(|value| match value {
        UiValue::Text(value) => Some(value.as_str().to_owned()),
        _ => None,
    })
}

#[semio_framework_async_macros::async_test]
async fn definition_uses_the_frozen_window_kit_kind_id() {
    assert_eq!(definition().id, WINDOW_KIND_ID);
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_scene_node_for_the_default_document() {
    let _node = render(&WavSnapshot::default(), Locale::En, semio_framework_plugin::UiPublicationRevision(1));
}

#[semio_framework_async_macros::async_test]
async fn definition_localizes_audio_actions_without_global_revision_forms() {
    let definition = definition();
    let route = |id: &str| definition.actions.iter().find(|action| action.id == id).expect("declared WAV action");
    let append = route(semio_s_artifact_stdio_contract::ADD_TABLE_ROW_ACTION_ID);
    assert_eq!(append.label.resolve(semio_framework_ui_locale::Terminology::Native, Locale::En), "Append frame");
    assert_eq!(append.label.resolve(semio_framework_ui_locale::Terminology::Native, Locale::De), "Frame anhängen");
    assert!(definition.actions.iter().all(|action| action.keys.is_none()));
    assert!(definition.actions.iter().all(|action| !action.in_palette));
}

#[semio_framework_async_macros::async_test]
async fn every_audio_command_has_one_definition_and_a_revision_bound_surface_control() {
    let document = WavSnapshot {
        fmt: crate::standards::riff_pcm::subsets::any::schema::snapshot::WavFmt { channels: 2, sample_rate: 8_000, byte_rate: 32_000, block_align: 4, bits_per_sample: 16, ..Default::default() },
        data: WavData::Pcm16(vec![1, 2]),
        ..WavSnapshot::default()
    };
    let revision = "0123456789abcdef";
    let definition = definition();
    let root = render_revisioned(&document, revision, semio_framework_plugin::UiPublicationRevision(23), Locale::En, &TreeWindows::unhosted()).expect("complete audio action surface assembles");
    for action_id in crate::editor::wav::edit_audio::TOOL_IDS {
        assert_eq!(definition.actions.iter().filter(|action| action.id == *action_id).count(), 1, "{action_id} definition");
        let control = node_with_binding(&root, action_id).unwrap_or_else(|| panic!("{action_id} surface control"));
        assert!(matches!(&control.component, Component::Button(_) | Component::Input(_) | Component::TableRow(_)), "{action_id} must remain a natively keyboard-operable control (a row action is painted as a native button)");
        let binding = binding_named(control, action_id).expect("control owns binding");
        assert_eq!(text_arg(&binding, "revision").as_deref(), Some(revision), "{action_id} revision binding");
    }
}

#[semio_framework_async_macros::async_test]
async fn wide_audio_uses_complete_windowed_coordinates_and_revision_bound_controls() {
    let channels = 64u32;
    let revision = "0123456789abcdef";
    let document = WavSnapshot {
        fmt: crate::standards::riff_pcm::subsets::any::schema::snapshot::WavFmt { channels: channels as u16, sample_rate: 1_000, byte_rate: channels * 1_000, block_align: channels as u16, bits_per_sample: 8, ..Default::default() },
        data: WavData::Pcm8((0..channels * 2).map(|index| index as u8).collect()),
        ..WavSnapshot::default()
    };
    let view = ViewModel {
        locale: Locale::En,
        tree_windows: vec![
            TreeWindowRequest { body_key: BODY_KEY.into(), node_key: SAMPLE_TABLE_ID.into(), open: Some(true), offset: channels * 2 - 1, rows: 1 },
            TreeWindowRequest { body_key: BODY_KEY.into(), node_key: CHANNEL_TABLE_ID.into(), open: Some(true), offset: channels - 1, rows: 1 },
        ],
        ..ViewModel::new(Locale::En, semio_framework_ui_locale::Terminology::Native)
    };
    let windows = TreeWindows::for_body(&view, BODY_KEY);
    let root = render_revisioned(&document, revision, semio_framework_plugin::UiPublicationRevision(23), Locale::En, &windows).expect("wide audio surface assembles");
    let samples = node_by_key(&root, SAMPLE_TABLE_ID).expect("coordinate table");
    let Component::Table(sample_props) = &samples.component else { panic!("sample surface is a table") };
    assert_eq!(sample_props.window.map(|window| (window.total, window.offset)), Some((channels * 2, channels * 2 - 1)));
    assert_eq!(samples.children[0].key.as_str(), format!("sample-{}", channels * 2 - 1));
    let sample = binding_named(samples, crate::editor::wav::edit_audio::SET_SAMPLE_ACTION_ID).expect("late-channel sample edit binding");
    assert_eq!(number_arg(&sample, "row"), Some(1.0));
    assert_eq!(number_arg(&sample, "column"), Some(63.0));
    assert_eq!(text_arg(&sample, "revision").as_deref(), Some(revision));
    let channels_table = node_by_key(&root, CHANNEL_TABLE_ID).expect("channel controls");
    let Component::Table(channel_props) = &channels_table.component else { panic!("channel controls are a table") };
    assert_eq!(channel_props.window.map(|window| (window.total, window.offset)), Some((channels, channels - 1)));
    let insert_channel = binding_named(channels_table, crate::editor::wav::edit_audio::INSERT_CHANNEL_ACTION_ID).expect("late-channel insert binding");
    assert_eq!(number_arg(&insert_channel, "column"), Some(63.0));
    assert_eq!(text_arg(&insert_channel, "revision").as_deref(), Some(revision));
    assert_eq!(text_arg(&binding_named(&root, crate::editor::wav::edit_audio::SET_SAMPLE_RATE_ACTION_ID).expect("sample-rate control"), "revision").as_deref(), Some(revision));
    assert_eq!(text_arg(&binding_named(&root, semio_s_artifact_stdio_contract::ADD_TABLE_COLUMN_ACTION_ID).expect("append-channel control"), "revision").as_deref(), Some(revision));
}

#[semio_framework_async_macros::async_test]
async fn german_toolbar_uses_audio_terms() {
    let document = WavSnapshot { data: WavData::Pcm16(vec![0]), ..WavSnapshot::default() };
    let root = render_revisioned(&document, "rev", semio_framework_plugin::UiPublicationRevision(23), Locale::De, &TreeWindows::unhosted()).expect("German audio surface assembles");
    let append_frame = node_by_key(&root, "append-frame").expect("append-frame button");
    let Component::Button(props) = &append_frame.component else { panic!("append frame is a button") };
    assert_eq!(props.label.0.as_str(), "Frame anhängen");
    let append_channel = node_by_key(&root, "append-channel").expect("append-channel button");
    let Component::Button(props) = &append_channel.component else { panic!("append channel is a button") };
    assert_eq!(props.label.0.as_str(), "Kanal anhängen");
}
