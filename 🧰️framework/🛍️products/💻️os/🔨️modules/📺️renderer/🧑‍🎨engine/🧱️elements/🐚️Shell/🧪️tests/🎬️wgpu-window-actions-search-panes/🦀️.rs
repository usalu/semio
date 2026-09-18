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
use semio_framework::{ActionArgDef, ActionDefinition, ActionKind, UtilityDefinition};

fn pane_fixture() -> Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🎬️window-actions-search-panes/🔣️.json")).expect("window actions/search pane fixture")
}

/// 🎬️ puzzle3d's shape in miniature: one window kind opened as two instances, whose actions span
/// three categories and include one zero-arg verb, one arg-carrying verb and one framework-reserved
/// verb — the three rows every law below needs to tell apart.
fn actions_shell() -> ShellState {
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
        let section = stack.children.iter().find_map(|child| if let UiNode::Section(section) = child { Some(section) } else { None }).expect("the form section");
        let button = section
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
