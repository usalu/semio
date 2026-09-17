
use super::*;
use crate::engine::space::S_PLAY_PARAMETERS_BODY_KEY;
use semio_framework_plugin::{TreeWindowRequest, ViewModel, TREE_WINDOW_DEFAULT_ROWS};

fn labels() -> &'static SStudioLabels {
    semio_framework_plugin::resolve_labels::<SStudioLabels>(&ViewModel::default())
}

fn project(node: BuiltNode) -> String {
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree { root: node }).expect("parameters tree projection")
}

/// 🪟️ A workflow an order of magnitude past one viewport — the subject of every window law below.
async fn oversized_workflow(count: usize) -> WorkflowSnapshot {
    let mut projection = semio_framework_os::empty_workflow_snapshot().await;
    projection.parameters = (0..count)
        .map(|index| WorkflowParameter::Numeric { id: format!("parameter-{index}"), name: format!("Parameter {index}"), value: index as f64, min: Some(0.0), max: Some(100.0), step: Some(1.0) })
        .collect();
    projection
}

/// 🪟️ The panel body exactly as the host reads it, for the host-known windows in `requests`.
fn window_body(projection: &WorkflowSnapshot, requests: Vec<TreeWindowRequest>) -> String {
    let view = ViewModel { tree_windows: requests, ..Default::default() };
    project(render(projection, labels(), &TreeWindows::for_body(&view, S_PLAY_PARAMETERS_BODY_KEY)).expect("render"))
}

fn request(node_key: &str, open: Option<bool>, offset: u32, rows: u32) -> TreeWindowRequest {
    TreeWindowRequest { body_key: S_PLAY_PARAMETERS_BODY_KEY.into(), node_key: node_key.into(), open, offset, rows }
}

/// 🔑️ The **window path** the host addresses one parameter's field window by: the enclosing list
/// section's key, then the parameter's own key, joined by `TREE_WINDOW_PATH_SEPARATOR`.
fn parameter_path(id: &str) -> String {
    format!("{S_PLAY_PARAMETERS_LIST_KEY}{}s-play-parameters.{id}", semio_framework_plugin::TREE_WINDOW_PATH_SEPARATOR)
}

#[semio_framework_async_macros::async_test]
async fn render_produces_the_add_parameter_header() {
    let projection = semio_framework_os::empty_workflow_snapshot().await;
    let json = project(render(&projection, labels(), &TreeWindows::unhosted()).expect("render"));
    assert!(json.contains("addParameter"), "header must carry the add-parameter action: {json}");
    assert!(json.contains("parameter"), "empty parameter count copy must render: {json}");
}

//#region 🪟️WindowLaws
/// 🪟️ Law (a): the parameter list stamps its FULL extent and materialises at most one viewport — the
/// old `column` of `Container(Section)` nodes stamped nothing and hard-failed past its 32nd child.
#[semio_framework_async_macros::async_test]
async fn an_oversized_parameter_set_stamps_its_total_and_never_a_continuation_row() {
    let projection = oversized_workflow(200).await;
    let json = window_body(&projection, Vec::new());
    assert!(json.contains("\"total\":200"), "the parameter list stamps its full extent: {json}");
    assert!(!json.contains(".more"), "no continuation row survives: {json}");
    assert!(!json.contains("\"+"), "no `+N` label survives: {json}");
    assert!(json.matches("\"key\":\"s-play-parameters.parameter-").count() <= TREE_WINDOW_DEFAULT_ROWS as usize, "first paint materialises about one viewport: {json}");
}

/// 🪟️ Law (b): a container the host closed stamps its total and materialises nothing.
#[semio_framework_async_macros::async_test]
async fn a_closed_parameter_list_stamps_its_total_and_materialises_no_children() {
    let projection = oversized_workflow(200).await;
    let json = window_body(&projection, vec![request(S_PLAY_PARAMETERS_LIST_KEY, Some(false), 0, 0)]);
    assert!(json.contains("\"total\":200"), "a closed list still stamps its extent: {json}");
    assert!(!json.contains("\"key\":\"s-play-parameters.parameter-"), "a closed list materialises no rows: {json}");
}

/// 🪟️ Law (c): a host window materialises exactly `[offset, offset + rows)`, keyed by the raw parameter id.
#[semio_framework_async_macros::async_test]
async fn a_host_window_materialises_exactly_its_slice_of_the_parameter_set() {
    let projection = oversized_workflow(200).await;
    let json = window_body(&projection, vec![request(S_PLAY_PARAMETERS_LIST_KEY, Some(true), 80, 4)]);
    assert!(json.contains("\"offset\":80"), "the list reports its offset: {json}");
    for index in 80..84 {
        assert!(json.contains(&format!("\"key\":\"s-play-parameters.parameter-{index}\"")), "parameter {index} is inside the window: {json}");
    }
    assert!(!json.contains("\"key\":\"s-play-parameters.parameter-79\""), "the row before the window stays out: {json}");
    assert!(!json.contains("\"key\":\"s-play-parameters.parameter-84\""), "the row after the window stays out: {json}");
}

/// 🪟️ Law (c'), nested level: every parameter is itself a windowed container over its editable field
/// rows, so the host learns each parameter's own field extent and can close it independently.
#[semio_framework_async_macros::async_test]
async fn each_parameter_is_a_windowed_container_over_its_own_field_rows() {
    let projection = oversized_workflow(3).await;
    let json = window_body(&projection, Vec::new());
    assert!(json.contains("\"total\":6"), "a numeric parameter stamps its six field rows: {json}");
    assert!(json.contains("s-play-parameters.parameter-0.name"), "the name row materialises: {json}");
    assert!(json.contains("s-play-parameters.parameter-0.step"), "the step constraint row materialises: {json}");
    let closed = window_body(&projection, vec![request(&parameter_path("parameter-0"), Some(false), 0, 0)]);
    assert!(!closed.contains("s-play-parameters.parameter-0.name"), "a closed parameter materialises no field rows: {closed}");
    assert!(closed.contains("s-play-parameters.parameter-1.name"), "its siblings are unaffected: {closed}");
}

/// 🪟️ Law (d), unbound-tree variant: parameters are edited, not picked — the tree declares no
/// interaction domain and stamps no granularity, and every row keeps its own control binding.
#[semio_framework_async_macros::async_test]
async fn the_parameters_tree_binds_no_domain_and_every_row_keeps_its_own_control() {
    let json = window_body(&oversized_workflow(2).await, Vec::new());
    assert!(!json.contains("interactionDomain"), "the parameters tree declares no interaction domain: {json}");
    assert!(!json.contains("interactionSelect"), "the parameters tree stamps no tree-level pick binding: {json}");
    assert!(!json.contains("\"granularity\""), "parameter rows are not pick targets: {json}");
    assert!(json.contains("patchParameter"), "field rows keep their own patch binding: {json}");
    assert!(json.contains("removeParameter"), "the remove row keeps its own action: {json}");
}
//#endregion 🪟️WindowLaws
