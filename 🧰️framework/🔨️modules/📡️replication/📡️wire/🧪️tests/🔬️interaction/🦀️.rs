
use super::*;

//#region 🔖️Fixtures
/// 🌲️ root → {a → {a1, a2}, b → {b1}}, pre-order: root, a, a1, a2, b, b1.
async fn sample_topology() -> DomainTopology {
    let node = |id: &str, parent: Option<&str>| TopologyNode { id: id.into(), granularity: "node".into(), parent: parent.map(Into::into) };
    DomainTopology { ordered: vec![node("root", None), node("a", Some("root")), node("a1", Some("a")), node("a2", Some("a")), node("b", Some("root")), node("b1", Some("b"))] }
}

async fn target(id: &str) -> InteractionTarget {
    InteractionTarget { granularity: "node".into(), id: id.into() }
}

async fn selection(ids: &[&str], anchor: Option<&str>) -> DomainSelection {
    DomainSelection { granularity: "node".into(), ids: ids.iter().map(|id| id.to_string()).collect(), anchor_id: anchor.map(Into::into) }
}

async fn spec(transitive: bool, merges: &[MergeMode]) -> SelectionSpec {
    SelectionSpec { modes: vec![SelectionMode::Multiple, SelectionMode::Single], methods: vec![SelectionMethod::Pick], merges: merges.to_vec(), transitive, broadcast: true }
}

async fn multiple_input(ids: &[&str], merge: MergeMode) -> SelectionInput {
    let mut targets = Vec::with_capacity(ids.len());
    for id in ids {
        targets.push(target(id).await);
    }
    SelectionInput { targets, merge, mode: SelectionMode::Multiple }
}
//#endregion 🔖️Fixtures

//#region 🔖️MergeModes
#[semio_framework_async_macros::async_test]
async fn replace_sets_selection_to_batch_targets() {
    let current = selection(&["a1"], Some("a1")).await;
    let next = next_selection(&spec(false, &[MergeMode::Replace]).await, &current, &sample_topology().await, &multiple_input(&["b", "b1"], MergeMode::Replace).await).await;
    assert_eq!(next.ids, vec!["b".to_string(), "b1".to_string()]);
    assert_eq!(next.anchor_id.as_deref(), Some("b1"));
}

#[semio_framework_async_macros::async_test]
async fn additive_unions_batch_into_current_selection() {
    let current = selection(&["a1"], Some("a1")).await;
    let next = next_selection(&spec(false, &[MergeMode::Additive]).await, &current, &sample_topology().await, &multiple_input(&["a2"], MergeMode::Additive).await).await;
    assert_eq!(next.ids, vec!["a1".to_string(), "a2".to_string()]);
    assert_eq!(next.anchor_id.as_deref(), Some("a2"));
}

#[semio_framework_async_macros::async_test]
async fn subtractive_removes_batch_from_current_selection() {
    let current = selection(&["a1", "a2", "b1"], Some("b1")).await;
    let next = next_selection(&spec(false, &[MergeMode::Subtractive]).await, &current, &sample_topology().await, &multiple_input(&["a2"], MergeMode::Subtractive).await).await;
    assert_eq!(next.ids, vec!["a1".to_string(), "b1".to_string()]);
    assert_eq!(next.anchor_id.as_deref(), Some("a2"), "anchor tracks the last acted-on target, even on removal");
}

#[semio_framework_async_macros::async_test]
async fn invertive_toggles_each_batch_target_independently() {
    let current = selection(&["a1", "a2"], Some("a2")).await;
    let next = next_selection(&spec(false, &[MergeMode::Invertive]).await, &current, &sample_topology().await, &multiple_input(&["a2", "b1"], MergeMode::Invertive).await).await;
    assert_eq!(next.ids, vec!["a1".to_string(), "b1".to_string()], "a2 was present so it toggles off, b1 was absent so it toggles on");
}
//#endregion 🔖️MergeModes

//#region 🔖️Range
#[semio_framework_async_macros::async_test]
async fn range_slices_topology_order_between_anchor_and_target() {
    let current = selection(&["a"], Some("a")).await;
    let next = next_selection(&spec(false, &[MergeMode::Range]).await, &current, &sample_topology().await, &multiple_input(&["b1"], MergeMode::Range).await).await;
    assert_eq!(next.ids, vec!["a".to_string(), "a1".to_string(), "a2".to_string(), "b".to_string(), "b1".to_string()]);
    assert_eq!(next.anchor_id.as_deref(), Some("a"), "range never moves the anchor");
}

#[semio_framework_async_macros::async_test]
async fn range_falls_back_to_last_selected_id_when_no_anchor_recorded() {
    let current = selection(&["a1", "a2"], None).await;
    let next = next_selection(&spec(false, &[MergeMode::Range]).await, &current, &sample_topology().await, &multiple_input(&["b"], MergeMode::Range).await).await;
    assert_eq!(next.ids, vec!["a2".to_string(), "b".to_string()]);
    assert_eq!(next.anchor_id.as_deref(), Some("a2"));
}

#[semio_framework_async_macros::async_test]
async fn range_handles_target_before_anchor_in_topology_order() {
    let current = selection(&["b"], Some("b")).await;
    let next = next_selection(&spec(false, &[MergeMode::Range]).await, &current, &sample_topology().await, &multiple_input(&["a1"], MergeMode::Range).await).await;
    assert_eq!(next.ids, vec!["a1".to_string(), "a2".to_string(), "b".to_string()]);
}
//#endregion 🔖️Range

//#region 🔖️SingleClamp
#[semio_framework_async_macros::async_test]
async fn single_mode_clamps_to_last_target_regardless_of_merge() {
    let current = selection(&["a1", "a2"], Some("a1")).await;
    let input = SelectionInput { targets: vec![target("b").await, target("b1").await], merge: MergeMode::Additive, mode: SelectionMode::Single };
    let next = next_selection(&spec(false, &[MergeMode::Additive]).await, &current, &sample_topology().await, &input).await;
    assert_eq!(next.ids, vec!["b1".to_string()]);
    assert_eq!(next.anchor_id.as_deref(), Some("b1"));
}
//#endregion 🔖️SingleClamp

//#region 🔖️Transitive
#[semio_framework_async_macros::async_test]
async fn transitive_select_expands_target_to_descendant_closure() {
    let current = DomainSelection::default();
    let next = next_selection(&spec(true, &[MergeMode::Replace]).await, &current, &sample_topology().await, &multiple_input(&["a"], MergeMode::Replace).await).await;
    assert_eq!(next.ids, vec!["a".to_string(), "a1".to_string(), "a2".to_string()]);
}

#[semio_framework_async_macros::async_test]
async fn transitive_hover_expands_with_root_first() {
    let hover_spec = HoverSpec { enabled: true, transitive: true, channels: default_pointer_channels(), broadcast: true };
    let input = HoverInput { channel: "pointer".into(), targets: vec![target("a").await] };
    let hover = next_hover(&hover_spec, &sample_topology().await, &input).await;
    assert_eq!(hover.ids, vec!["a".to_string(), "a1".to_string(), "a2".to_string()]);
    assert_eq!(hover.ids.first().map(String::as_str), Some("a"), "hovered root sorts first");
}

#[semio_framework_async_macros::async_test]
async fn non_transitive_hover_replaces_with_raw_targets_only() {
    let hover_spec = HoverSpec { enabled: true, transitive: false, channels: default_pointer_channels(), broadcast: true };
    let input = HoverInput { channel: "pointer".into(), targets: vec![target("a").await] };
    let hover = next_hover(&hover_spec, &sample_topology().await, &input).await;
    assert_eq!(hover.ids, vec!["a".to_string()]);
}

#[semio_framework_async_macros::async_test]
async fn empty_hover_targets_clears_the_channel() {
    let hover_spec = HoverSpec::default();
    let hover = next_hover(&hover_spec, &sample_topology().await, &HoverInput { channel: "pointer".into(), targets: Vec::new() }).await;
    assert!(hover.ids.is_empty());
}
//#endregion 🔖️Transitive

//#region 🔖️ValidateState
async fn sample_outline() -> InteractionOutline {
    InteractionOutline { id: "graph".into(), granularity_ids: vec!["node".into(), "edge".into()], selection: spec(false, &[MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive, MergeMode::Range]).await }
}

#[semio_framework_async_macros::async_test]
async fn validate_state_prunes_ids_absent_from_topology() {
    let def = sample_outline().await;
    let mut topo = InteractionTopology::default();
    topo.domains.insert("graph".into(), sample_topology().await);

    let mut state = InteractionState::default();
    state.selection.insert("graph".into(), selection(&["a1", "deleted-node", "b1"], Some("deleted-node")).await);
    state.hover.insert("graph".into(), DomainHover { channel: "pointer".into(), ids: vec!["a1".into(), "gone".into()] });
    state.active_mode.insert("graph".into(), SelectionMode::Multiple);
    state.active_granularity.insert("graph".into(), "node".into());

    let validated = validate_state(&[def], &topo, &state).await;
    let graph_selection = validated.selection.get("graph").expect("graph domain kept");
    assert_eq!(graph_selection.ids, vec!["a1".to_string(), "b1".to_string()], "deleted-node pruned");
    assert_eq!(graph_selection.anchor_id, None, "stale anchor pruned along with its id");
    assert_eq!(validated.hover.get("graph").unwrap().ids, vec!["a1".to_string()], "gone pruned");
}

#[semio_framework_async_macros::async_test]
async fn validate_state_drops_undeclared_domains_and_granularities() {
    let def = sample_outline().await;
    let topo = InteractionTopology::default();

    let mut state = InteractionState::default();
    state.selection.insert("mesh".into(), selection(&["x"], None).await);
    state.active_granularity.insert("graph".into(), "face".into());

    let validated = validate_state(&[def], &topo, &state).await;
    assert!(!validated.selection.contains_key("mesh"), "undeclared domain dropped");
    assert_eq!(validated.active_granularity.get("graph").map(String::as_str), Some("node"), "undeclared granularity resets to the default");
}

#[semio_framework_async_macros::async_test]
async fn validate_state_clamps_single_mode_selection_to_first_id() {
    let def = sample_outline().await;
    let mut topo = InteractionTopology::default();
    topo.domains.insert("graph".into(), sample_topology().await);

    let mut state = InteractionState::default();
    state.selection.insert("graph".into(), selection(&["a1", "a2", "b1"], None).await);
    state.active_mode.insert("graph".into(), SelectionMode::Single);

    let validated = validate_state(&[def], &topo, &state).await;
    assert_eq!(validated.selection.get("graph").unwrap().ids, vec!["a1".to_string()]);
}
//#endregion 🔖️ValidateState

//#region 🔖️Serde
/// 🌱️ Rewritten off `serde_json` (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
/// 26/09/01): asserts the same internally-tagged shape directly on `DslValue` instead of a JSON
/// string — this crate cannot depend on `pack::json` (it sits below `pack` in the DAG), and the
/// `DslValue` tree IS the wire shape `ToValue`/`FromValue` produce/consume.
#[semio_framework_async_macros::async_test]
async fn hierarchy_provider_to_value_is_internally_tagged() {
    let path_delimited = HierarchyProvider::PathDelimited { delimiter: "/".into() };
    let value = crate::value::ToValue::to_value(&path_delimited);
    assert_eq!(value, crate::value::DslValue::object(vec![("kind".to_string(), crate::value::DslValue::String("pathDelimited".to_string())), ("delimiter".to_string(), crate::value::DslValue::String("/".to_string())),]));
    assert_eq!(<HierarchyProvider as crate::value::FromValue>::from_value(value).unwrap(), path_delimited);

    let flat_value = crate::value::ToValue::to_value(&HierarchyProvider::Flat);
    assert_eq!(flat_value, crate::value::DslValue::object(vec![("kind".to_string(), crate::value::DslValue::String("flat".to_string()))]));
}
//#endregion 🔖️Serde
