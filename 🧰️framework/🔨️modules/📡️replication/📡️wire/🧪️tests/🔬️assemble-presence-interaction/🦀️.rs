
use super::*;

async fn selection(ids: &[&str]) -> DomainSelection {
    DomainSelection { granularity: "node".into(), ids: ids.iter().map(|id| id.to_string()).collect(), anchor_id: None }
}

async fn hover(channel: &str, ids: &[&str]) -> DomainHover {
    DomainHover { channel: channel.into(), ids: ids.iter().map(|id| id.to_string()).collect() }
}

async fn broadcasting_hover_spec() -> HoverSpec {
    HoverSpec { enabled: true, transitive: false, channels: vec!["pointer".into()], broadcast: true }
}

async fn broadcasting_selection_spec() -> SelectionSpec {
    SelectionSpec { modes: vec![SelectionMode::Multiple], methods: vec![SelectionMethod::Pick], merges: vec![MergeMode::Replace], transitive: false, broadcast: true }
}

#[semio_framework_async_macros::async_test]
async fn assemble_presence_interaction_includes_broadcasting_domains() {
    let mut state = InteractionState::default();
    state.selection.insert("graph".into(), selection(&["n1", "n2"]).await);
    state.hover.insert("graph".into(), hover("pointer", &["n3"]).await);
    state.active_granularity.insert("graph".into(), "node".into());

    let hover_specs = BTreeMap::from([("graph".to_string(), broadcasting_hover_spec().await)]);
    let selection_specs = BTreeMap::from([("graph".to_string(), broadcasting_selection_spec().await)]);

    let interaction = assemble_presence_interaction("draw", &state, &hover_specs, &selection_specs).await;
    assert_eq!(interaction.app_id, "draw");
    assert_eq!(interaction.domains.len(), 1);
    let domain = &interaction.domains[0];
    assert_eq!(domain.domain, "graph");
    assert_eq!(domain.granularity, "node");
    assert_eq!(domain.selected, vec!["n1".to_string(), "n2".to_string()]);
    assert_eq!(domain.hovered, vec!["n3".to_string()]);
}

#[semio_framework_async_macros::async_test]
async fn assemble_presence_interaction_omits_domains_with_broadcast_disabled() {
    let mut state = InteractionState::default();
    state.selection.insert("private".into(), selection(&["secret"]).await);
    state.hover.insert("private".into(), hover("pointer", &["secret"]).await);

    let hover_specs = BTreeMap::from([("private".to_string(), HoverSpec { broadcast: false, ..broadcasting_hover_spec().await })]);
    let selection_specs = BTreeMap::from([("private".to_string(), SelectionSpec { broadcast: false, ..broadcasting_selection_spec().await })]);

    let interaction = assemble_presence_interaction("draw", &state, &hover_specs, &selection_specs).await;
    assert!(interaction.domains.is_empty(), "broadcast:false on both halves drops the domain entirely");
}

#[semio_framework_async_macros::async_test]
async fn assemble_presence_interaction_only_broadcasts_the_pointer_hover_channel() {
    let mut state = InteractionState::default();
    state.hover.insert("graph".into(), hover("drag-preview", &["n1"]).await);

    let hover_specs = BTreeMap::from([("graph".to_string(), broadcasting_hover_spec().await)]);
    let selection_specs = BTreeMap::new();

    let interaction = assemble_presence_interaction("draw", &state, &hover_specs, &selection_specs).await;
    assert!(interaction.domains.is_empty(), "a non-pointer hover channel never broadcasts");
}

#[semio_framework_async_macros::async_test]
async fn assemble_presence_interaction_respects_each_half_independently() {
    let mut state = InteractionState::default();
    state.selection.insert("graph".into(), selection(&["n1"]).await);
    state.hover.insert("graph".into(), hover("pointer", &["n2"]).await);

    let hover_specs = BTreeMap::from([("graph".to_string(), HoverSpec { broadcast: false, ..broadcasting_hover_spec().await })]);
    let selection_specs = BTreeMap::from([("graph".to_string(), broadcasting_selection_spec().await)]);

    let interaction = assemble_presence_interaction("draw", &state, &hover_specs, &selection_specs).await;
    assert_eq!(interaction.domains.len(), 1);
    assert_eq!(interaction.domains[0].selected, vec!["n1".to_string()], "selection still broadcasts");
    assert!(interaction.domains[0].hovered.is_empty(), "hover suppressed by its own broadcast:false");
}
