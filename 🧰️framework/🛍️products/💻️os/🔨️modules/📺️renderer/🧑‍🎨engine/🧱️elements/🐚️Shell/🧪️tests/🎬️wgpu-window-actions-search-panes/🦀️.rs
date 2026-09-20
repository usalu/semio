//! 🎬️ LAW: the per-window Actions pane and the per-window Search pane are REAL retained bodies on
//! this renderer, carrying React's own control ids — one `action.category.<id>` header per category,
//! one `action.<id>` row per panel-eligible action, the expanded action's staged form with React's
//! `framework.window.<segment>.action.<id>.{execute,reset}` pair, and a search line whose id is the
//! engagement's own — and neither body moves a shell SURFACE, exactly as React's two `Pane`s do not.
//!
//! Oracle: `🐚️Shell/🧫️fixtures/🎬️window-actions-search-panes/🔣️.json`, read off React's
//! `buildActionCategoryTree`/`WindowActionPane`/`Search` sources plus one measured DOM capture of the
//! live React shell (`🗑️generated/w13b-react-dom.json`), derived independently of this implementation.

use super::*;
use crate::program_bridge::window_engagements_section_tests::{engagements_section_document, fixture as engagements_fixture};
use semio_framework::{ActionArgDef, ActionDefinition, ActionKind, UtilityDefinition};

fn pane_fixture() -> Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🎬️window-actions-search-panes/🔣️.json")).expect("window actions/search pane fixture")
}

fn tree_density_fixture() -> Value {
    serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🖱️ui/🧫️fixtures/🌳️tree-row-density/🔣️.json")).expect("tree row density fixture")
}

/// 🎬️ puzzle3d's shape in miniature: one window kind opened as two instances, whose actions span
/// three categories and include one zero-arg verb, one arg-carrying verb and one framework-reserved
/// verb — the three rows every law below needs to tell apart.
pub(super) fn actions_shell() -> ShellState {
    let mut shell = super::window_pane_chrome_tests::split_pane_shell();
    let session = shell.session.as_mut().expect("the split-pane fixture carries a session");
    let kind = session.app.window_kinds.first_mut();
    kind.actions = vec![
        ActionDefinition { in_palette: true, ..ActionDefinition::new_catalog("addObjectKind", LocalizedLabel::data("Add Object Kind"), ActionKind::Mutation) },
        ActionDefinition {
            in_palette: true,
            category: Some("create".into()),
            args: vec![ActionArgDef { required: true, ..ActionArgDef::text("kind", LocalizedLabel::data("Kind")) }, ActionArgDef::number("count", LocalizedLabel::data("Count"))],
            ..ActionDefinition::new_catalog("openAddObjectDialog", LocalizedLabel::data("Add Object"), ActionKind::Mutation)
        },
        ActionDefinition { in_palette: true, category: Some("history".into()), ..ActionDefinition::new_catalog("undo", LocalizedLabel::data("Undo"), ActionKind::View) },
        ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog("worldPointerDown", LocalizedLabel::data("Pointer"), ActionKind::Interaction) },
    ];
    shell
}

fn dense_actions_shell() -> ShellState {
    let mut shell = actions_shell();
    let fixture = tree_density_fixture();
    let count = fixture["actionCount"].as_u64().expect("action count") as usize;
    let session = shell.session.as_mut().expect("the split-pane fixture carries a session");
    let kind = session.app.window_kinds.first_mut();
    kind.actions.clear();
    let mut ids = vec!["clearSelection".to_string(), "selectAll".to_string()];
    ids.extend((0..count.saturating_sub(3)).map(|index| format!("density{index:02}")));
    ids.push("engagementAbort".to_string());
    kind.actions = ids
        .into_iter()
        .map(|id| ActionDefinition { in_palette: true, category: Some("selection".into()), ..ActionDefinition::new_catalog(id.clone(), LocalizedLabel::data(id), ActionKind::View) })
        .collect();
    shell
}

fn publish_dense_actions_chrome(shell: &mut ShellState, input: &mut InputState<ActionDescriptor>) -> DrawList {
    while input.retire_hit_step() {}
    shell.screen_w = 1440.0;
    shell.screen_h = 640.0;
    let mut frame = ShellChromeFrameCursor::default();
    let mut draw = DrawList::default();
    let mut overlay = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let theme = Theme::light();
    let mut world_resources = infinite_world::world::World3dBuildContext::new(infinite_world::world::WorldCursorWakeAuthority::new());
    for _ in 0..262_144 {
        if shell.render_chrome_step(&mut frame, &mut draw, &mut overlay, &mut atlas, &icons, input, &theme, &mut world_resources) {
            return draw;
        }
    }
    panic!("dense Actions chrome walk did not reach publication");
}

#[test]
fn actions_and_search_publish_vertical_scroll_roots() {
    let mut shell = actions_shell();
    shell.window_engagements.insert(
        "pane-top".into(),
        WindowEngagement {
            session_active: Some(true),
            options: None,
            input: Some(ui_wgpu::wgpu::WindowEngagementInput {
                id: Some("pane-search".into()),
                value: Some(String::new()),
                placeholder: None,
                disabled: None,
                on_change: None,
                on_submit: None,
                on_repeat_last: None,
                on_abort: None,
            }),
            control: None,
            controls: None,
            status: None,
            possible_engagements: None,
        },
    );
    for (surface, node) in [
        (window_actions_surface_id("pane-top"), shell.build_window_actions_ui("pane-top").expect("Actions body")),
        (window_search_surface_id("pane-top"), shell.build_window_search_ui("pane-top").expect("Search body")),
    ] {
        let records = panel_ui_scroll_records(&surface, &node).expect("scroll projection");
        assert!(matches!(records[0].layout, ui_contract::LayoutSpec::Scroll(ui_contract::ScrollLayout { axes: ui_contract::ScrollAxes::Vertical, .. })), "{surface} root owns the clipped vertical viewport");
    }
}

#[test]
fn the_mounted_actions_tree_keeps_fixed_rows_clips_the_terminal_row_and_scrolls_to_it() {
    let fixture = tree_density_fixture();
    let row_height = fixture["rowHeightPx"].as_f64().expect("row height") as f32;
    let first = fixture["firstRows"].as_array().expect("first rows");
    let first_id = first[0].as_str().expect("first row");
    let second_id = first[1].as_str().expect("second row");
    let terminal_id = fixture["terminalRowId"].as_str().expect("terminal row");
    let scroll_delta = fixture["scrollDeltaPx"].as_f64().expect("scroll delta") as f32;
    let mut shell = dense_actions_shell();
    shell.sync_dock_tabs();
    shell.toggle_window_pane_chip("pane-top", WindowPaneChip::Actions);
    let windows = vec!["pane-top".to_string(), "pane-perspective".to_string()];
    let mut faults = Vec::new();
    shell.refresh_window_action_panes(&windows, &mut faults).expect("dense Actions pane documents publish");
    assert!(faults.is_empty(), "dense Actions pane publication remains fault-free: {faults:?}");
    let mut input = InputState::<ActionDescriptor>::default();

    let row = |id: &str, input: &InputState<ActionDescriptor>| {
        input
            .hits()
            .iter()
            .find(|hit| hit.kind == HitKind::TreeItem && hit.control_id.as_deref().is_some_and(|control| control.ends_with(id)))
            .unwrap_or_else(|| panic!("mounted Actions row '{id}' missing from the published viewport"))
            .clone()
    };
    let _ = publish_dense_actions_chrome(&mut shell, &mut input);
    let clear = row(first_id, &input);
    let select = row(second_id, &input);
    assert!((clear.rect.h - row_height).abs() < 0.01 && (select.rect.h - row_height).abs() < 0.01, "fixed row heights: {:?} {:?}", clear.rect, select.rect);
    assert!((select.rect.y - clear.rect.y - row_height).abs() < 0.01, "fixed row pitch: {:?} {:?}", clear.rect, select.rect);
    assert!(!input.hits().iter().any(|hit| hit.control_id.as_deref().is_some_and(|control| control.ends_with(terminal_id))), "the terminal Actions row is clipped below the initial viewport");

    let pointer = (clear.rect.x + clear.rect.w * 0.5, clear.rect.y + clear.rect.h * 0.5);
    assert!(shell.handle_pointer_wheel(pointer.0, pointer.1, scroll_delta / 24.0, &mut input), "the retained Actions viewport owns wheel input");
    let draw = publish_dense_actions_chrome(&mut shell, &mut input);
    assert!(input.hits().iter().any(|hit| hit.kind == HitKind::TreeItem && hit.control_id.as_deref().is_some_and(|control| control.ends_with(terminal_id))), "the retained viewport publication reveals the terminal Actions row");
    let abort = row(terminal_id, &input);
    assert!((abort.rect.h - row_height).abs() < 0.01, "the scrolled terminal row keeps its authored height: {:?}", abort.rect);
    assert!(!input.hits().iter().any(|hit| hit.control_id.as_deref().is_some_and(|control| control.ends_with(first_id))), "the first row leaves the clipped viewport after scrolling");
    assert!(draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).any(|instance| instance.params[2] == ui_wgpu::wgpu::draw::KIND_GLYPH && instance.rect[1] >= abort.rect.y && instance.rect[1] < abort.rect.y + abort.rect.h), "the normal chrome walk paints the terminal row in the same band it publishes for input");

    let pointer = (abort.rect.x + abort.rect.w * 0.5, abort.rect.y + abort.rect.h * 0.5);
    let _ = crate::collect_fixture_actions(&mut input);
    assert!(shell.retained_hit_window(&abort).is_some(), "the terminal row remains addressed to its retained Actions body");
    let mut capture = PointerCapture::default();
    let owner = capture.press(shell.pointer_owner_at(pointer.0, pointer.1, &input, &Theme::light()));
    assert_eq!(owner, PointerHitOwner::Chrome, "the retained Actions row owns PointerDown above the scene under its pane");
    semio_framework_async::block_on(shell.handle_pointer_button(pointer.0, pointer.1, true, 0, &mut input, &Theme::light())).expect("scrolled Abort row press");
    let _ = publish_dense_actions_chrome(&mut shell, &mut input);
    assert_eq!(capture.release(), PointerHitOwner::Chrome, "the retained Actions row keeps the captured release across its repaint");
    semio_framework_async::block_on(shell.handle_pointer_button(pointer.0, pointer.1, false, 0, &mut input, &Theme::light())).expect("scrolled Abort row release");
    let actions = crate::collect_fixture_actions(&mut input);
    assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), vec![terminal_id.strip_prefix("action.").expect("terminal action id")], "the painted scrolled row dispatches its own action exactly once");
    assert!(shell.handle_pointer_wheel(pointer.0, pointer.1, -scroll_delta / 24.0, &mut input), "reverse wheel remains owned by the retained viewport");
    let _ = publish_dense_actions_chrome(&mut shell, &mut input);
    assert!((row(first_id, &input).rect.h - row_height).abs() < 0.01, "reverse wheel restores the initial fixed row");
    assert!(!input.hits().iter().any(|hit| hit.control_id.as_deref().is_some_and(|control| control.ends_with(terminal_id))), "reverse wheel clips the terminal row again");

    let mut faults = Vec::new();
    shell.refresh_window_action_panes(&[], &mut faults).expect("dense pane owners retire");
}

/// 🧾️ Every record key the pane's published document carries, in publication order — the tail of each
/// is React's own control id, because [`panel_ui_records`] qualifies a shell-owned key with its
/// surface (two panes of one kind publish the same React ids, which is React's own duplicate-id
/// defect; a global owner map cannot carry it, so this renderer qualifies).
fn published_keys(shell: &ShellState, window_id: &str, search: bool) -> Vec<String> {
    let surface = if search { window_search_surface_id(window_id) } else { window_actions_surface_id(window_id) };
    let node = if search { shell.build_window_search_ui(window_id) } else { shell.build_window_actions_ui(window_id) }.expect("the pane publishes a body");
    panel_ui_records(&surface, &node).expect("the pane body projects").into_iter().map(|record| record.key.as_str().to_string()).collect()
}

/// 🎬️ **The census pin.** One `action.category.<id>` header per category in first-declaration order,
/// each followed by its own `action.<id>` rows — React's `buildActionCategoryTree` shape — and an
/// action the app hides from the palette is in neither renderer's pane.
#[test]
fn the_actions_pane_publishes_reacts_own_category_and_row_census() {
    let fixture = pane_fixture();
    let shell = actions_shell();
    let keys = published_keys(&shell, "pane-top", false);
    let category_row = fixture["actionsPane"]["categoryRowId"].as_str().expect("fixture category row id");
    let action_row = fixture["actionsPane"]["actionRowId"].as_str().expect("fixture action row id");
    let rows: Vec<&String> = keys.iter().filter(|key| key.contains("/action.")).collect();
    let expected: Vec<String> = [
        category_row.replace("{category}", "actions"),
        action_row.replace("{actionId}", "addObjectKind"),
        category_row.replace("{category}", "create"),
        action_row.replace("{actionId}", "openAddObjectDialog"),
        category_row.replace("{category}", "history"),
        action_row.replace("{actionId}", "undo"),
    ]
    .into_iter()
    .collect();
    assert_eq!(rows.len(), expected.len(), "🎬️ the pane publishes exactly React's rows, got {rows:?}");
    for (key, react_id) in rows.iter().zip(&expected) {
        assert!(key.ends_with(react_id.as_str()), "🎬️ '{key}' does not carry React's own id '{react_id}'");
    }
    assert!(!keys.iter().any(|key| key.ends_with("action.worldPointerDown")), "🎬️ an action the app keeps out of the palette is not a pane row");
}

/// 🎯️ **The first-row dispatch pin.** A zero-arg row fires the APP's own verb with the app's
/// controller — never a shell verb, never a second dispatch — which is what makes the first row of
/// puzzle3d's pane journal `addObjectKind` exactly as React's does.
#[test]
fn the_first_row_of_the_actions_pane_dispatches_the_apps_own_verb() {
    let shell = actions_shell();
    let node = shell.build_window_actions_ui("pane-top").expect("the pane publishes a body");
    let UiNode::Stack(stack) = &node else { panic!("🎬️ the pane body is a stack") };
    let tree = stack.children.iter().find_map(|child| if let UiNode::Tree(tree) = child { Some(tree) } else { None }).expect("the pane body carries React's own Tree");
    let first = tree.sections.first().expect("a category section").items.first().expect("a row").clone();
    assert_eq!(first.id, "action.addObjectKind", "🎯️ the first row is the app's first panel-eligible action");
    let action = first.action.expect("a zero-arg row carries its verb");
    assert_eq!(action.action, "addObjectKind", "🎯️ the row dispatches the verb it names");
    assert_eq!(action.controller_id, "pane.controller", "🎯️ and it dispatches it to the APP, not the shell");
    assert!(action.args.is_none(), "🎯️ a zero-arg verb fires bare — no staged defaults, P4");
}

/// 📝️ **The staged-form pin.** An arg-carrying row wears React's `…` suffix and toggles the
/// expansion instead of firing; the expanded action KEEPS its list row (that row is the trigger that
/// folds the form again) and gains a form carrying one `action.<id>.arg.<argId>` row per visible
/// argument plus React's own `framework.window.<segment>.action.<id>.{execute,reset}` pair, with
/// Execute refused until every required argument has an effective value.
#[test]
fn an_arg_carrying_row_opens_reacts_staged_form_and_refuses_execute_until_it_resolves() {
    let fixture = pane_fixture();
    let form = &fixture["actionsPane"]["form"];
    let mut shell = actions_shell();
    let node = shell.build_window_actions_ui("pane-top").expect("a body");
    let UiNode::Stack(stack) = &node else { panic!("🎬️ the pane body is a stack") };
    let tree = stack.children.iter().find_map(|child| if let UiNode::Tree(tree) = child { Some(tree) } else { None }).expect("a tree");
    let row = tree.sections.iter().flat_map(|section| section.items.iter()).find(|item| item.id == "action.openAddObjectDialog").expect("the arg-carrying row");
    assert!(row.label.as_str().ends_with(fixture["actionsPane"]["argCarryingLabelSuffix"].as_str().expect("fixture suffix")), "📝️ an arg-carrying row wears React's ellipsis");
    let dispatch = row.action.clone().expect("the row toggles its own expansion");
    assert_eq!((dispatch.controller_id.as_str(), dispatch.action.as_str()), ("framework", "setActionExpanded"), "📝️ an arg-carrying row opens the form and fires nothing (P3)");

    shell.action_panel_expanded.insert("pane-top".into(), "openAddObjectDialog".into());
    let keys = published_keys(&shell, "pane-top", false);
    let segment = semio_framework::element_id_segment("pane-top");
    for expected in [
        form["sectionId"].as_str().expect("form section").replace("{category}", "create"),
        form["argRowId"].as_str().expect("arg row").replace("{actionId}", "openAddObjectDialog").replace("{argId}", "kind"),
        form["argRowId"].as_str().expect("arg row").replace("{actionId}", "openAddObjectDialog").replace("{argId}", "count"),
        form["executeId"].as_str().expect("execute").replace("{windowSegment}", &segment).replace("{actionId}", "openAddObjectDialog"),
        form["resetId"].as_str().expect("reset").replace("{windowSegment}", &segment).replace("{actionId}", "openAddObjectDialog"),
    ] {
        assert!(keys.iter().any(|key| key.ends_with(&expected)), "📝️ the staged form is missing React's '{expected}' — got {keys:?}");
    }
    // 🌳️ React's ACTIONS pane does NOT drop the expanded action from its list: `buildActionCategoryTree`
    // maps EVERY `categoryActions` entry and pushes the form BESIDE the list section
    // (`🛠️ShellHelpers/🟦️.tsx:4097`-`:4145`), because the row itself is the accordion trigger that folds
    // the form again. Only `buildCommandCategoryTree` — the Command dock, a different surface — filters
    // the expanded command out, and this law was first written with that rule by mistake.
    let list_rows: Vec<&String> = keys.iter().filter(|key| key.ends_with("action.openAddObjectDialog")).collect();
    assert_eq!(list_rows.len(), 1, "📝️ the expanded action keeps exactly its one list row — it is the trigger that folds the form again, got {keys:?}");

    let execute = |shell: &ShellState| -> bool {
        let node = shell.build_window_actions_ui("pane-top").expect("a body");
        let UiNode::Stack(stack) = &node else { panic!("stack") };
        // ⚡️ React renders Execute/Reset as the form section's own `actions`; a `TreeSection` carries
        // no action row here, so the pair follows the form as its own band of the pane stack.
        let button = stack
            .children
            .iter()
            .find_map(|child| if let UiNode::Button(button) = child { button.id.as_deref().is_some_and(|id| id.ends_with("execute")).then_some(button) } else { None })
            .expect("the Execute button");
        matches!(button.presence.state, ui_wgpu::wgpu::component::ui::UiState::Disabled)
    };
    assert!(execute(&shell), "📝️ Execute is refused while a required argument is unstaged — React's `missing.length > 0`");
    shell.stage_arg("pane-top", "openAddObjectDialog", "kind", Value::String("slab".into()));
    assert!(!execute(&shell), "📝️ and admitted the moment every required argument resolves");
}

/// 🚦️ **The armed-utility pin.** A utility that declares `allowsActionsWhileActive: false` disables
/// every APP row of the pane it is armed in — and NONE of the framework's own reserved verbs, whose
/// chords keep firing regardless (`FRAMEWORK_RESERVED_ACTION_IDS`).
#[test]
fn an_armed_gating_utility_disables_every_app_row_but_not_the_frameworks_own() {
    let fixture = pane_fixture();
    let declared: Vec<&str> = fixture["actionsPane"]["reservedActionIds"].as_array().expect("fixture reserved ids").iter().map(|id| id.as_str().expect("reserved id")).collect();
    assert_eq!(declared, FRAMEWORK_RESERVED_ACTION_IDS.to_vec(), "🚦️ this renderer's reserved set is React's own, verbatim");

    let mut shell = actions_shell();
    let session = shell.session.as_mut().expect("a session");
    session.app.utilities.push(UtilityDefinition { id: "pane.gate".into(), label: LocalizedLabel::data("Gate"), icon_id: "lock".into(), group: None, keys: None, cursor: None, category: None, allows_actions_while_active: false, run: None });
    shell.active_utility_by_window.insert("pane-top".into(), "pane.gate".into());
    let node = shell.build_window_actions_ui("pane-top").expect("a body");
    let UiNode::Stack(stack) = &node else { panic!("stack") };
    let tree = stack.children.iter().find_map(|child| if let UiNode::Tree(tree) = child { Some(tree) } else { None }).expect("a tree");
    for item in tree.sections.iter().flat_map(|section| section.items.iter()) {
        let reserved = item.id == "action.undo";
        assert_eq!(matches!(item.presence.state, ui_wgpu::wgpu::component::ui::UiState::Disabled), !reserved, "🚦️ {} gating state", item.id);
        assert_eq!(item.action.is_some(), reserved, "🚦️ {} keeps a verb only while it stays pressable", item.id);
    }
}

/// 🔎️ **The search pane pin.** The line is published under the ENGAGEMENT's own input id, the
/// possibles chevron under React's `ui.windowSearch.suggestions`, and an engagement with no input
/// publishes no body at all — React's `if (!hasInput) return null`.
#[test]
fn the_search_pane_publishes_reacts_input_and_suggestion_ids() {
    let fixture = pane_fixture();
    let mut shell = actions_shell();
    assert!(shell.build_window_search_ui("pane-top").is_none(), "🔎️ an engagement with no input publishes no search body");

    let possibles: Vec<ui_wgpu::wgpu::WindowEngagementPossible> = fixture["searchRanking"]["possibles"]
        .as_array()
        .expect("fixture possibles")
        .iter()
        .map(|row| ui_wgpu::wgpu::WindowEngagementPossible {
            id: row["id"].as_str().expect("possible id").to_string(),
            label: row["label"].as_str().expect("possible label").to_string(),
            detail: row["detail"].as_str().map(ToOwned::to_owned),
            action: Some(ActionDescriptor { controller_id: "pane.controller".into(), action: "acceptSuggestion".into(), args: None }),
        })
        .collect();
    shell.window_engagements.insert(
        "pane-top".into(),
        WindowEngagement {
            session_active: Some(true),
            options: None,
            input: Some(ui_wgpu::wgpu::WindowEngagementInput {
                id: Some("pane-engagement".into()),
                value: Some(String::new()),
                placeholder: None,
                disabled: None,
                on_change: None,
                on_submit: Some(ActionDescriptor { controller_id: "pane.controller".into(), action: "engagementSubmit".into(), args: None }),
                on_repeat_last: None,
                on_abort: None,
            }),
            control: None,
            controls: None,
            status: None,
            possible_engagements: Some(possibles),
        },
    );
    let keys = published_keys(&shell, "pane-top", false);
    assert!(keys.iter().all(|key| !key.ends_with("pane-engagement")), "🔎️ the typed line belongs to the SEARCH pane, never the Actions pane");
    let keys = published_keys(&shell, "pane-top", true);
    assert!(keys.iter().any(|key| key.ends_with("pane-engagement")), "🔎️ the line carries the engagement's own id — got {keys:?}");
    let suggestions = fixture["searchPane"]["suggestionsId"].as_str().expect("fixture suggestions id");
    assert!(keys.iter().any(|key| key.ends_with(suggestions)), "🔎️ the possibles chevron carries React's id");
    assert!(!keys.iter().any(|key| key.ends_with("p.box")), "🔎️ a collapsed chevron publishes no suggestion row, React's own Popover");

    shell.search_possibles_open.insert("pane-top".into(), true);
    let keys = published_keys(&shell, "pane-top", true);
    for id in ["p.other", "p.circle", "p.boxSelect", "p.box"] {
        assert!(keys.iter().any(|key| key.ends_with(id)), "🔎️ an open chevron publishes the '{id}' row");
    }

    let unnamed = shell.window_engagements.get_mut("pane-top").expect("the engagement");
    unnamed.input.as_mut().expect("the input").id = None;
    let keys = published_keys(&shell, "pane-top", true);
    assert!(keys.iter().any(|key| key.ends_with(fixture["searchPane"]["fallbackInputId"].as_str().expect("fixture fallback id"))), "🔎️ an unnamed line falls back to React's own `ui.windowSearch.action`");
}

#[test]
fn canonical_engagements_publication_drives_search_presence_body_and_retirement() {
    let fixture = engagements_fixture();
    let document = engagements_section_document(&fixture["section"], 41);
    let mut shell = actions_shell();
    let mut faults = Vec::new();
    shell.install_window_engagements_section(document, &mut faults).expect("the canonical section installs");
    assert!(faults.is_empty(), "the authored section decodes: {faults:?}");
    for expected in fixture["windows"].as_array().expect("fixture windows") {
        let window_id = expected["windowId"].as_str().expect("window id");
        assert_eq!(shell.window_has_search_spec(window_id), expected["hasSearchSpec"].as_bool().expect("search-spec expectation"), "{window_id}: Search toggle presence");
        assert_eq!(shell.build_window_search_ui(window_id).is_some(), expected["hasSearchBody"].as_bool().expect("search-body expectation"), "{window_id}: Search body presence");
    }

    let windows = ["pane-top".to_string(), "pane-perspective".to_string()];
    shell.refresh_window_action_panes(&windows, &mut faults).expect("the canonical snapshot republishes pane owners");
    assert!(shell.window_search_documents.contains_key("pane-top"), "input-only engagement owns a Search body");
    assert!(!shell.window_search_documents.contains_key("pane-perspective"), "possibles-only engagement exposes the Search toggle without a blank body");

    shell.refresh_window_action_panes(&["pane-perspective".to_string()], &mut faults).expect("the closed input window retires its pane owner");
    assert!(shell.window_search_documents.is_empty(), "closing the input window releases its Search document");
    assert!(shell.closing_documents.terminal_is_empty(), "the retired owner reaches terminal release");
    shell.refresh_window_action_panes(&[], &mut faults).expect("remaining pane owners retire");
}

#[test]
fn malformed_engagements_retire_the_transport_lease_and_preserve_the_last_valid_snapshot() {
    let fixture = engagements_fixture();
    let mut shell = actions_shell();
    let mut faults = Vec::new();
    shell.install_window_engagements_section(engagements_section_document(&fixture["section"], 42), &mut faults).expect("valid section installs");
    let expected = shell.window_engagements.clone();
    shell
        .install_window_engagements_section(engagements_section_document(&serde_json::json!(["invalid"]), 43), &mut faults)
        .expect("malformed source ownership still retires");
    assert_eq!(shell.window_engagements, expected, "a malformed refresh preserves the last valid snapshot");
    assert_eq!(faults.len(), 1, "the producer fault is explicit");
    assert!(faults[0].2.starts_with("window engagements section parse"), "the decode fault identifies the canonical section: {:?}", faults[0]);
    assert!(shell.closing_documents.terminal_is_empty(), "the malformed source lease reaches terminal release");
}

/// 🔎️ **The ranking pin.** The window search line ranks with React's `searchPossibleRankScore`, NOT
/// with the ⌘️K palette's `rank_fuzzy_items`: label prefix beats detail prefix beats id prefix beats a
/// substring anywhere, ties keep declaration order, and an empty query keeps the host's own order.
#[test]
fn the_search_possibles_rank_the_way_reacts_own_scorer_does() {
    let fixture = pane_fixture();
    let possibles: Vec<ui_wgpu::wgpu::WindowEngagementPossible> = fixture["searchRanking"]["possibles"]
        .as_array()
        .expect("fixture possibles")
        .iter()
        .map(|row| ui_wgpu::wgpu::WindowEngagementPossible { id: row["id"].as_str().expect("id").to_string(), label: row["label"].as_str().expect("label").to_string(), detail: row["detail"].as_str().map(ToOwned::to_owned), action: None })
        .collect();
    for case in fixture["searchRanking"]["cases"].as_array().expect("fixture cases") {
        let query = case["query"].as_str().expect("case query");
        let expected: Vec<&str> = case["expected"].as_array().expect("case expectation").iter().map(|id| id.as_str().expect("expected id")).collect();
        let ranked: Vec<&str> = filter_search_possibles(query, &possibles).into_iter().map(|possible| possible.id.as_str()).collect();
        assert_eq!(ranked, expected, "🔎️ query {query:?}");
    }
    assert_eq!(normalize_engagement_action_text("move 3.5 x"), "Move3.5X", "🔎️ a decimal INSIDE a number survives React's own normalizer");
}

/// 🪟️ **The surface pin.** Publishing both pane bodies moves no shell SURFACE — React's census reports
/// windows, docked panels and dialogs, and an open `Pane` is none of those. This is what makes the
/// probe's `+s` column read empty on both renderers for a pane-chip step.
#[test]
fn the_pane_bodies_move_no_shell_surface() {
    let mut shell = actions_shell();
    let before = shell.chrome_surface_census();
    let mut faults = Vec::new();
    shell.refresh_window_action_panes(&["pane-top".to_string(), "pane-perspective".to_string()], &mut faults).expect("the panes publish");
    assert!(faults.is_empty(), "🪟️ both bodies project cleanly, got {faults:?}");
    assert_eq!(shell.window_actions_documents.len(), 2, "🪟️ each live pane owns its own Actions document");
    assert_eq!(shell.chrome_surface_census(), before, "🪟️ and neither publication is a shell surface");

    shell.refresh_window_action_panes(&["pane-top".to_string()], &mut faults).expect("the panes republish");
    assert_eq!(shell.window_actions_documents.keys().map(String::as_str).collect::<Vec<_>>(), vec!["pane-top"], "🪟️ a pane that closes retires its body");
    // 📄️ Every lease this law minted is handed back before the fixture drops — a retained document
    // dropped without retirement is the store-drop fault the framework refuses.
    shell.refresh_window_action_panes(&[], &mut faults).expect("the panes retire");
    assert!(shell.window_actions_documents.is_empty() && shell.window_search_documents.is_empty(), "🪟️ and a shell with no live pane owns no pane document");
}

/// 🎛️ **The one-fold pin.** Both bodies read the ONE `actionsFolded` React gives the Actions/Search
/// pair, so either chip opens and closes both — and a folded pane paints neither body.
#[test]
fn both_pane_bodies_read_the_one_actions_fold() {
    let mut shell = actions_shell();
    assert!(shell.window_actions_folded("pane-top"), "🎛️ React's `useState(true)` — a pane opens folded");
    shell.toggle_window_pane_chip("pane-top", WindowPaneChip::Search);
    assert!(!shell.window_actions_folded("pane-top"), "🎛️ the Search chip opens the pair");
    assert!(shell.window_actions_folded("pane-perspective"), "🎛️ and never the sibling pane's");
    shell.toggle_window_pane_chip("pane-top", WindowPaneChip::Actions);
    assert!(shell.window_actions_folded("pane-top"), "🎛️ the Actions chip closes the same fold");
}

/// 🎛️ **The engagement-control law (W14c item 2).** React's `<Engagement/>` paints a `control` and a
/// `controls` row — slider, stepper, ring, toggle group, select — above its status lines and its
/// quick-action group, each under React's own id and dispatching React's own intent. wgpu painted the
/// status lines and the options and DROPPED all five kinds
/// (`📓️w13b-actions-search-pane-bodies.md` §6 gap 3): `WindowEngagementControl` had no reader in the
/// whole renderer, so a granularity ring or a step slider simply was not there.
#[test]
fn the_engagement_body_paints_reacts_control_row_with_reacts_ids_and_intents() {
    let fixture = pane_fixture();
    let controls = &fixture["engagementControls"];
    let mut shell = actions_shell();
    let select = |action: &str| Some(ActionDescriptor { controller_id: "pane.controller".into(), action: action.into(), args: None });
    shell.window_engagements.insert(
        "pane-top".into(),
        WindowEngagement {
            session_active: Some(true),
            options: None,
            input: None,
            control: Some(ui_wgpu::wgpu::WindowEngagementControl::Slider { id: None, label: Some("Height".into()), value: 2.0, min: 0.0, max: 10.0, step: Some(0.5), unit: Some("m".into()), disabled: None, on_change: select("setHeight"), on_commit: None }),
            controls: Some(vec![
                ui_wgpu::wgpu::WindowEngagementControl::Stepper { id: None, label: None, value: 3.0, min: None, max: None, step: Some(1.0), unit: None, disabled: None, on_change: select("setCount"), on_commit: None },
                ui_wgpu::wgpu::WindowEngagementControl::Ring { id: None, label: None, value: Some("ring.b".into()), options: vec![ui_wgpu::wgpu::WindowEngagementRingOption { id: "ring.a".into(), label: "A".into(), disabled: None }, ui_wgpu::wgpu::WindowEngagementRingOption { id: "ring.b".into(), label: "B".into(), disabled: None }], disabled: None, on_select: select("pickOrb") },
                ui_wgpu::wgpu::WindowEngagementControl::ToggleGroup { id: Some("granularity".into()), label: None, value: Some("granularity.face".into()), options: vec![ui_wgpu::wgpu::WindowEngagementToggleGroupOption { id: "granularity.face".into(), label: "Face".into(), disabled: None }, ui_wgpu::wgpu::WindowEngagementToggleGroupOption { id: "granularity.edge".into(), label: "Edge".into(), disabled: None }], disabled: None, on_select: select("setGranularity") },
                ui_wgpu::wgpu::WindowEngagementControl::Select { id: None, label: None, value: Some("m".into()), placeholder: None, items: vec![ui_wgpu::wgpu::WindowEngagementSelectItem { id: "m".into(), value: "m".into(), label: "Metres".into() }], disabled: None, on_change: select("setUnit") },
            ]),
            status: Some(vec![
                ui_wgpu::wgpu::WindowEngagementStatus { id: "engagement-step".into(), text: "Pick a face".into() },
                ui_wgpu::wgpu::WindowEngagementStatus { id: "engagement-hint".into(), text: "Shift to snap".into() },
            ]),
            possible_engagements: None,
        },
    );
    let keys = published_keys(&shell, "pane-top", false);
    let fallbacks = &controls["fallbackControlIds"];
    for expected in [
        fallbacks["slider"].as_str().expect("slider fallback id").to_string(),
        fallbacks["stepper"].as_str().expect("stepper fallback id").to_string(),
        fallbacks["select"].as_str().expect("select fallback id").to_string(),
        fallbacks["ring"].as_str().expect("ring fallback id").to_string(),
        "granularity".to_string(),
        "granularity.face".to_string(),
        "granularity.edge".to_string(),
        "ring.a".to_string(),
        "ring.b".to_string(),
    ] {
        assert!(keys.iter().any(|key| key.ends_with(&format!("/{expected}"))), "🎛️ the engagement body is missing React's '{expected}' — got {keys:?}");
    }

    let node = shell.build_window_actions_ui("pane-top").expect("a body");
    let UiNode::Stack(stack) = &node else { panic!("🎬️ the pane body is a stack") };
    // 🧭️ React's order: the session's step heading, the primary control, the `controls` row, the
    // remaining status lines, the quick-action group.
    let Some(UiNode::Text(heading)) = stack.children.first() else { panic!("🎛️ the live session starts with its heading") };
    assert_eq!(heading.value.as_str(), "Pick a face", "🎛️ a LIVE session promotes `engagement-step` into the heading React renders first");
    let Some(UiNode::Field(primary)) = stack.children.get(1) else { panic!("🎛️ a labelled primary control publishes as a semantic field") };
    assert_eq!(primary.label.as_str(), controls["unitLabelFormat"].as_str().expect("unit format").replace("{label}", "Height").replace("{unit}", "m"), "🎛️ a numeric control with a unit reads React's `Label (unit)`");
    assert!(matches!(primary.child.as_ref(), UiNode::Slider(_)), "🎛️ the field's focusable child is the primary slider");
    let status_index = stack.children.iter().position(|child| matches!(child, UiNode::Text(text) if text.value.as_str() == "Shift to snap")).expect("the secondary status line");
    let options_index = stack.children.iter().position(|child| matches!(child, UiNode::Stack(group) if group.id.as_deref() == controls["optionsGroupId"].as_str())).unwrap_or(usize::MAX);
    let select_index = stack.children.iter().position(|child| matches!(child, UiNode::Select(_))).expect("the select control");
    assert!(select_index < status_index, "🎛️ every `controls` row precedes the secondary status lines");
    assert!(status_index < options_index || options_index == usize::MAX, "🎛️ and the status lines precede the quick-action group");

    // 🎯️ Intent parity: a toggle-group option dispatches the control's `onSelect` carrying the
    // OPTION's own id — React's `onClick={() => control.onSelect?.(option.id)}`.
    let face = stack
        .children
        .iter()
        .find_map(|child| if let UiNode::Stack(group) = child { group.children.iter().find_map(|row| if let UiNode::Toggle(toggle) = row { (toggle.id == "granularity.face").then_some(toggle) } else { None }) } else { None })
        .expect("the toggle group's first option");
    assert_eq!(face.on_change.action, "setGranularity", "🎯️ the option fires the control's own verb");
    assert!(face.presence.selected, "🎯️ and the selected option reads pressed, as React's `interactiveActiveFillClass` paints it");
    let Some(DslValue::Object(args)) = face.on_change.args.as_ref() else { panic!("🎯️ the option carries its own id in the intent") };
    assert_eq!(args.iter().find(|(key, _)| key == "id").map(|(_, value)| value.clone()), Some(DslValue::String("granularity.face".into())), "🎯️ React's `onSelect(option.id)`");

    // 🕳️ React's `if (!control.options.length) return null` — an empty group renders NOTHING, not an
    // empty container.
    shell.window_engagements.get_mut("pane-top").expect("the engagement").controls = Some(vec![ui_wgpu::wgpu::WindowEngagementControl::ToggleGroup { id: Some("empty".into()), label: Some("Empty".into()), value: None, options: Vec::new(), disabled: None, on_select: None }]);
    let keys = published_keys(&shell, "pane-top", false);
    assert!(!keys.iter().any(|key| key.ends_with("/empty")), "🕳️ an option-less toggle group publishes nothing at all");
}

/// ✍️ **The search-line moment law (W14c item 3).** React's line binds `onChange` on every keystroke
/// AND `onSubmit` on Enter AND `onAbort` on Escape AND `onRepeatLast` on Space over an empty idle
/// line — four moments on one field. This renderer bound ONE: it committed on blur and dispatched
/// `on_submit` there, so a program that feeds autocomplete from the typing never saw a keystroke
/// (`📓️w13b-actions-search-pane-bodies.md` §6 gap 4). The retained producers landed with this packet;
/// this pins the four bindings the published record carries.
#[test]
fn the_search_line_binds_reacts_change_submit_abort_and_repeat_moments() {
    let fixture = pane_fixture();
    let line = &fixture["searchLine"];
    let verb = |action: &str| Some(ActionDescriptor { controller_id: "pane.controller".into(), action: action.into(), args: None });
    let engagement = |session_active: bool| WindowEngagement {
        session_active: Some(session_active),
        options: None,
        input: Some(ui_wgpu::wgpu::WindowEngagementInput {
            id: Some("pane-engagement".into()),
            value: Some(String::new()),
            placeholder: None,
            disabled: None,
            on_change: verb("engagementInput"),
            on_submit: verb("engagementSubmit"),
            on_repeat_last: verb("engagementRepeatLast"),
            on_abort: verb("engagementAbort"),
        }),
        control: None,
        controls: None,
        status: None,
        possible_engagements: None,
    };
    let mut shell = actions_shell();
    shell.window_engagements.insert("pane-top".into(), engagement(false));
    let node = shell.build_window_search_ui("pane-top").expect("the search body");
    let UiNode::Stack(stack) = &node else { panic!("🔎️ the search body is a stack") };
    let Some(UiNode::Input(input)) = stack.children.first() else { panic!("🔎️ the line is the body's first row") };
    assert_eq!(input.commit.is_some(), line["commitsOnBlur"].as_bool().expect("fixture commit rule"), "✍️ the line fires `Trigger::Change` on every keystroke, never only on blur");
    assert_eq!(input.on_change.action, "engagementInput", "✍️ `onChange` is the guest's own verb");
    assert_eq!(input.on_submit.as_ref().map(|action| action.action.as_str()), Some("engagementSubmit"), "⏎️ Enter carries `onSubmit`");
    assert_eq!(input.on_abort.as_ref().map(|action| action.action.as_str()), Some("engagementAbort"), "⎋️ Escape carries `onAbort`");
    assert_eq!(input.on_repeat_last.as_ref().map(|action| action.action.as_str()), Some("engagementRepeatLast"), "🔁️ an IDLE line carries `onRepeatLast`");

    let surface = window_search_surface_id("pane-top");
    let records = panel_ui_records(&surface, &node).expect("the search body projects");
    let record = records.iter().find(|record| record.key.as_str().ends_with("pane-engagement")).expect("the line's own record");
    let bound: Vec<String> = record.bindings.iter().map(|binding| format!("{:?}", binding.trigger).to_lowercase()).collect();
    for key in ["onChange", "onSubmit", "onAbort", "onRepeatLast"] {
        let trigger = line["triggers"][key].as_str().expect("fixture trigger name").to_lowercase();
        assert!(bound.contains(&trigger), "✍️ the published record binds no {key} ({trigger}) — got {bound:?}");
    }

    // 🔁️ React routes an empty line to `onSubmit` DURING a session and to `onRepeatLast` only while
    // idle (`applySearchSpaceAction`), so a live session offers no repeat at all.
    assert!(line["repeatLastOnlyWhenIdle"].as_bool().expect("fixture repeat rule"));
    shell.window_engagements.insert("pane-top".into(), engagement(true));
    let node = shell.build_window_search_ui("pane-top").expect("the search body");
    let UiNode::Stack(stack) = &node else { panic!("stack") };
    let Some(UiNode::Input(input)) = stack.children.first() else { panic!("line") };
    assert!(input.on_repeat_last.is_none(), "🔁️ a live engagement session routes an empty line to submit, never to repeat-last");
    assert_eq!(input.on_submit.as_ref().map(|action| action.action.as_str()), Some("engagementSubmit"), "⏎️ and it keeps its submit");
}

/// 🌳️ **The form-header law (W14c item 4).** React's staged form is a `TreeDataSection`
/// (`action.category.<id>.form`) whose header is a collapsible BUTTON and whose rows carry their
/// editor as the row's own `control`. This renderer projected it as a `UiNode::Section`, and a
/// `Container(Section)` registers NO hit (`retained_hit_registration`, `📥️input/🦀️.rs:718`), so the
/// header had no pressable twin at all (`📓️w13b-actions-search-pane-bodies.md` §6 gap 5).
#[test]
fn the_staged_forms_category_header_registers_reacts_collapsible_row() {
    let fixture = pane_fixture();
    let form = &fixture["actionsPane"]["form"];
    let mut shell = actions_shell();
    shell.action_panel_expanded.insert("pane-top".into(), "openAddObjectDialog".into());
    let surface = window_actions_surface_id("pane-top");
    let node = shell.build_window_actions_ui("pane-top").expect("a body");
    let records = panel_ui_records(&surface, &node).expect("the pane body projects");
    let section_id = form["sectionId"].as_str().expect("form section").replace("{category}", "create");
    let header = records.iter().find(|record| record.key.as_str().ends_with(&section_id)).unwrap_or_else(|| panic!("🌳️ the form header '{section_id}' is published"));
    assert!(matches!(header.component, ui_contract::Component::TreeSection(_)), "🌳️ React's form header is a collapsible tree section, not a plain container — only a `TreeSection` registers `section.chevron.<id>`");

    // 🎛️ Every argument row is a `TreeItem` carrying its editor as a CHILD record, which is React's
    // `TreeDataItem.control`; the row keeps `action.<id>.arg.<argId>` and the editor keeps the bare
    // `def.id` React's `renderStagedArgControl` gives it.
    let row_id = form["argRowId"].as_str().expect("arg row").replace("{actionId}", "openAddObjectDialog").replace("{argId}", "kind");
    let row = records.iter().find(|record| record.key.as_str().ends_with(&row_id)).unwrap_or_else(|| panic!("🌳️ the '{row_id}' row is published"));
    assert!(matches!(row.component, ui_contract::Component::TreeItem(_)), "🌳️ a staged argument is a tree ROW");
    assert_eq!(row.children.len(), 1, "🎛️ and it carries exactly its editor");
    let editor = records.iter().find(|record| Some(record.id) == row.children.iter().next().copied()).expect("the row's editor record");
    assert!(matches!(editor.component, ui_contract::Component::Input(_)), "🎛️ a text argument edits through an input");
    assert!(editor.key.as_str().ends_with("/kind"), "🆔️ the editor keeps React's bare `def.id`: {}", editor.key.as_str());

    // ⚡️ React's section `actions` keep their own ids beside the form.
    let segment = semio_framework::element_id_segment("pane-top");
    for expected in [form["executeId"].as_str().expect("execute").replace("{windowSegment}", &segment).replace("{actionId}", "openAddObjectDialog"), form["resetId"].as_str().expect("reset").replace("{windowSegment}", &segment).replace("{actionId}", "openAddObjectDialog")] {
        assert!(records.iter().any(|record| record.key.as_str().ends_with(&expected)), "⚡️ the form keeps React's '{expected}'");
    }
}
