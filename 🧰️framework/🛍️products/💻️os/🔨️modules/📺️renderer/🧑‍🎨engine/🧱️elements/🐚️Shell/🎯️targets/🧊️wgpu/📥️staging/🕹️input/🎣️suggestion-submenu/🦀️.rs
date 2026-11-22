//! 🎣️ The context menu's live "suggest" submenu and the floating suggestion popup — React's `🧱️elements/🎣️suggestion-submenu/🟦️.ts` and `World3dHost`'s `suggestionMenuItems`: the right-click finds the row that names a connector, the shell starts that connector's search the moment the menu opens, the row becomes a submenu over the live candidate rows (a pending row until the search echoes back), and the search is released when the menu closes. Only the world surface publishes its menu record to the shell, so a board's row stays a plain action. See [`super`].

use super::*;
use serde_json::{json, Value};

pub const SUGGESTION_SUBMENU_OPEN_ACTION: &str = "openVortexSuggestions";
pub const SUGGESTION_SUBMENU_CLOSE_ACTION: &str = "closeVortexSuggestions";

/// 🎣️ Which host publishes the menu record: a world surface lists vortex candidates, a board lists handle candidates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SuggestionKind {
    World,
    Board,
}

/// 🔦️ One candidate row's hover: the preview action (`hoverSuggestion`) and the trace record it focuses.
#[derive(Clone, Debug, PartialEq)]
pub struct SuggestionHover {
    pub row_id: String,
    pub action: ActionDescriptor,
    pub focus: Option<u64>,
}

/// 🎣️ The search or popup one open context menu stands for: the connector it targets, the controller that owns it, the window whose surface publishes the menu record, the row that became the submenu (empty for the floating popup) and the candidate row the pointer is on.
#[derive(Clone, Debug, PartialEq)]
pub struct SuggestionSearch {
    pub target: String,
    pub controller_id: String,
    pub window_id: String,
    pub row_id: String,
    pub kind: SuggestionKind,
    pub popup: bool,
    pub hovers: Vec<SuggestionHover>,
    pub hovered: Option<String>,
}

/// 📂️ One candidate row with the hover it carries.
#[derive(Clone, Debug)]
pub struct SuggestionRow {
    pub item: ContextMenuItem,
    pub hover: Option<SuggestionHover>,
}

/// 🗂️ The menu record a window's surface published, with its host.
#[derive(Clone, Debug)]
pub struct SuggestionSource {
    pub kind: SuggestionKind,
    pub menu: Option<Value>,
    pub brush_index: i64,
    pub controller_id: String,
    pub host_key: String,
}

/// 🔎️ The connector a resolved menu's "suggest" row points at: the first row, at any depth, that dispatches [`SUGGESTION_SUBMENU_OPEN_ACTION`] with a `fullId`, with the controller it dispatches to.
pub fn suggestion_submenu_target(items: &[ContextMenuItem]) -> Option<(String, String)> {
    items.iter().find_map(|item| {
        let own = item.action.as_ref().filter(|action| action.action == SUGGESTION_SUBMENU_OPEN_ACTION).and_then(|action| {
            let full_id = action.args.as_ref()?.get("fullId")?.as_str().filter(|id| !id.is_empty())?;
            Some((full_id.to_owned(), action.controller_id.clone()))
        });
        own.or_else(|| suggestion_submenu_target(&item.children))
    })
}

/// 📂️ Turns every "suggest" row into a submenu over `children` (the row stops dispatching) and keeps every other row, recursively; answers the id of the first row it turned.
pub fn with_suggestion_submenu(items: &mut [ContextMenuItem], children: &[ContextMenuItem]) -> Option<String> {
    let mut first = None;
    for item in items {
        if item.action.as_ref().is_some_and(|action| action.action == SUGGESTION_SUBMENU_OPEN_ACTION) {
            item.action = None;
            item.children = children.to_vec();
            first.get_or_insert_with(|| item.id.clone());
        } else if let Some(id) = with_suggestion_submenu(&mut item.children, children) {
            first.get_or_insert(id);
        }
    }
    first
}

/// 🔄️ Replaces the children of the row `row_id` when they differ in id, label, check or enablement; `true` when it did.
pub fn set_submenu_children(items: &mut [ContextMenuItem], row_id: &str, children: &[ContextMenuItem]) -> bool {
    let same = |a: &[ContextMenuItem], b: &[ContextMenuItem]| a.len() == b.len() && a.iter().zip(b).all(|(a, b)| (&a.id, &a.label, a.checked, a.disabled) == (&b.id, &b.label, b.checked, b.disabled));
    for item in items {
        if item.id == row_id {
            if same(&item.children, children) {
                return false;
            }
            item.children = children.to_vec();
            return true;
        }
        if set_submenu_children(&mut item.children, row_id, children) {
            return true;
        }
    }
    false
}

/// 🪟️ True when this window owns the open suggestion menu: it is open and names this window or none.
pub fn suggestion_menu_owns_window(menu: &Value, window_id: &str) -> bool {
    menu.get("open") == Some(&Value::Bool(true)) && menu.get("windowId").and_then(Value::as_str).filter(|id| !id.is_empty()).is_none_or(|id| id == window_id)
}

/// 🔑️ A trace key is a u64 that travels as a JSON number or, beyond 2^53, as its decimal text.
fn trace_key(candidate: &Value) -> Option<u64> {
    let key = candidate.get("key")?;
    key.as_u64().or_else(|| key.as_str()?.parse().ok())
}

/// 📂️ The candidate rows one suggestion menu lists, as `suggestionMenuItems` and `board2dSuggestionMenuItems` build them: a pending row while the search has not echoed back (or this window does not own the menu), an empty row when it found none, otherwise one `acceptSuggestion` row per candidate, the active one checked, each carrying its `hoverSuggestion` preview.
pub fn suggestion_rows(menu: Option<&Value>, window_id: &str, source: &SuggestionSource, is_de: bool) -> Vec<SuggestionRow> {
    let board = source.kind == SuggestionKind::Board;
    let text = |key| semio_renderer_wgpu_shell_vocab::shell_chrome_string(key, is_de).to_string();
    let note = |id: &str, label: String| vec![SuggestionRow { item: ContextMenuItem { id: id.into(), label, disabled: true, ..Default::default() }, hover: None }];
    let Some(menu) = menu.filter(|menu| suggestion_menu_owns_window(menu, window_id)).filter(|menu| menu.get("pending") != Some(&Value::Bool(true))) else {
        return note("pending", text("host.checkingPlacement"));
    };
    let candidates = menu.get("candidates").and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default();
    if candidates.is_empty() {
        return note("empty", text("host.noPlacement"));
    }
    let (owner_key, owner_field, parts, icon, active) = if board { ("handleId", "handleId", ("nodeLabel", "handleLabel"), "circle-dot", menu.get("hoveredIndex").and_then(Value::as_i64).unwrap_or(0)) } else { ("vortexFullId", "fullId", ("objectLabel", "vortexLabel"), "box", source.brush_index) };
    let owner = menu.get(owner_key).and_then(Value::as_str).filter(|id| !id.is_empty());
    candidates
        .iter()
        .filter_map(|candidate| {
            let index = candidate.get("index")?.as_i64()?;
            let part = |key: &str| candidate.get(key).and_then(Value::as_str).unwrap_or_default();
            let with_owner = |mut args: Value| {
                if let Some(owner) = owner {
                    args[owner_field] = json!(owner);
                }
                args
            };
            let id = format!("suggestion-{index}");
            let hover_args = if board { with_owner(json!({ "index": index, "windowId": window_id })) } else { json!({ "index": index, "windowId": window_id }) };
            Some(SuggestionRow {
                item: ContextMenuItem {
                    id: id.clone(),
                    label: format!("{} \u{b7} {}", part(parts.0), part(parts.1)),
                    icon: Some(candidate.get("icon").and_then(Value::as_str).unwrap_or(icon).to_owned()),
                    action: Some(ActionDescriptor { controller_id: source.controller_id.clone(), action: "acceptSuggestion".into(), args: Some(semio_framework::DslValue::from(with_owner(json!({ "index": index })))) }),
                    checked: index == active,
                    ..Default::default()
                },
                hover: Some(SuggestionHover { row_id: id, action: ActionDescriptor { controller_id: source.controller_id.clone(), action: "hoverSuggestion".into(), args: Some(semio_framework::DslValue::from(hover_args)) }, focus: trace_key(candidate) }),
            })
        })
        .collect()
}

/// 🏷️ The popup's title, React's `ContextMenuController title`: a labelled separator, which paints as a heading and no arrow key reaches.
pub fn suggestion_popup_title(is_de: bool) -> ContextMenuItem {
    ContextMenuItem { id: "suggestion-title".into(), label: semio_renderer_wgpu_shell_vocab::shell_chrome_string("surfaceContextMenu.placementSuggestions", is_de).to_string(), separator: true, ..Default::default() }
}

/// 🪟️ True when `menu` is open, owned by `window_id` and floats as its own popup rather than as the context menu's submenu.
pub fn suggestion_popup_owns_window(menu: &Value, window_id: &str) -> bool {
    suggestion_menu_owns_window(menu, window_id) && menu.get("submenu") != Some(&Value::Bool(true))
}

impl ShellInputState {
    /// 🎣️ The surface of `window_id` that publishes suggestion menus: a world surface first, then a board.
    pub fn suggestion_source(&self, surfaces: &SurfaceState, window_id: &str) -> Option<SuggestionSource> {
        if let Some(state) = surfaces.world3d_state_for_window(window_id) {
            let session = &state.session;
            return Some(SuggestionSource { kind: SuggestionKind::World, menu: session.suggestion_menu_json.as_deref().and_then(|json| serde_json::from_str(json).ok()), brush_index: session.brush_candidate_index, controller_id: session.controller_id.clone(), host_key: session.surface_id.clone() });
        }
        let (host_key, board) = surfaces.board2d_states.iter().find(|(_, board)| board.window_id == window_id)?;
        Some(SuggestionSource { kind: SuggestionKind::Board, menu: board.suggestion_menu_json.as_deref().and_then(|json| serde_json::from_str(json).ok()), brush_index: 0, controller_id: board.controller_id.clone(), host_key: host_key.clone() })
    }

    /// 🎣️ The floating popup some window's surface asks for: `(window, source, x, y)` of the first open menu that is owned and not a submenu.
    pub fn suggestion_popup_request(&self, surfaces: &SurfaceState) -> Option<(String, SuggestionSource, f32, f32)> {
        let worlds = surfaces.world3d_states.iter().filter_map(|(_, state)| state.session.window_instance_id.clone());
        let boards = surfaces.board2d_states.iter().map(|(_, board)| board.window_id.clone());
        worlds.chain(boards).find_map(|window_id| {
            let source = self.suggestion_source(surfaces, &window_id)?;
            let menu = source.menu.as_ref().filter(|menu| suggestion_popup_owns_window(menu, &window_id))?;
            let at = |key| menu.get(key).and_then(Value::as_f64).unwrap_or_default() as f32;
            let (x, y) = (at("x"), at("y"));
            Some((window_id, source, x, y))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(id: &str, action: Option<&str>, children: Vec<ContextMenuItem>) -> ContextMenuItem {
        let action = action.map(|name| ActionDescriptor { controller_id: "app".into(), action: name.into(), args: Some(semio_framework::DslValue::from(json!({ "fullId": "v/1" }))) });
        ContextMenuItem { id: id.into(), label: id.into(), action, children, ..Default::default() }
    }

    fn source(kind: SuggestionKind, brush_index: i64) -> SuggestionSource {
        SuggestionSource { kind, menu: None, brush_index, controller_id: "app".into(), host_key: "h".into() }
    }

    fn items(rows: Vec<SuggestionRow>) -> Vec<ContextMenuItem> {
        rows.into_iter().map(|row| row.item).collect()
    }

    #[test]
    fn the_suggest_row_is_found_at_any_depth_and_becomes_a_submenu() {
        let mut menu = vec![row("a", Some("other"), vec![]), row("group", None, vec![row("suggest", Some(SUGGESTION_SUBMENU_OPEN_ACTION), vec![])])];
        assert_eq!(suggestion_submenu_target(&menu), Some(("v/1".into(), "app".into())));
        let rows = items(suggestion_rows(None, "w1", &source(SuggestionKind::World, 0), false));
        assert_eq!(with_suggestion_submenu(&mut menu, &rows).as_deref(), Some("suggest"));
        assert!(menu[1].children[0].action.is_none() && menu[1].children[0].children[0].id == "pending");
        assert_eq!(suggestion_submenu_target(&menu), None);
    }

    #[test]
    fn a_menu_of_another_window_lists_the_pending_row_and_an_owned_one_lists_its_candidates() {
        let menu = json!({ "open": true, "windowId": "w1", "pending": false, "vortexFullId": "v/1", "candidates": [{ "index": 0, "objectLabel": "Box", "vortexLabel": "Top", "key": 7 }, { "index": 2, "objectLabel": "Pipe", "vortexLabel": "End", "icon": "tube", "key": "18446744073709551615" }] });
        let world = source(SuggestionKind::World, 2);
        assert_eq!(suggestion_rows(Some(&menu), "w2", &world, false)[0].item.id, "pending");
        let rows = suggestion_rows(Some(&menu), "w1", &world, false);
        assert_eq!(rows.iter().map(|row| (row.item.label.as_str(), row.item.checked)).collect::<Vec<_>>(), [("Box \u{b7} Top", false), ("Pipe \u{b7} End", true)]);
        assert_eq!(rows[1].item.icon.as_deref(), Some("tube"));
        assert_eq!(rows.iter().map(|row| row.hover.as_ref().and_then(|hover| hover.focus)).collect::<Vec<_>>(), [Some(7), Some(u64::MAX)]);
        assert_eq!(rows[0].hover.as_ref().map(|hover| hover.action.action.as_str()), Some("hoverSuggestion"));
        assert_eq!(suggestion_rows(Some(&json!({ "open": true, "pending": false, "candidates": [] })), "w1", &world, true)[0].item.id, "empty");
    }

    #[test]
    fn a_board_menu_lists_handle_candidates_checked_by_hovered_index() {
        let menu = json!({ "open": true, "handleId": "h1", "hoveredIndex": 1, "candidates": [{ "index": 0, "nodeLabel": "A", "handleLabel": "in" }, { "index": 1, "nodeLabel": "B", "handleLabel": "out" }] });
        let rows = suggestion_rows(Some(&menu), "w1", &source(SuggestionKind::Board, 0), false);
        assert_eq!(rows.iter().map(|row| (row.item.label.as_str(), row.item.checked, row.item.icon.as_deref())).collect::<Vec<_>>(), [("A \u{b7} in", false, Some("circle-dot")), ("B \u{b7} out", true, Some("circle-dot"))]);
        let args = rows[0].item.action.as_ref().and_then(|action| action.args.as_ref()).and_then(|args| args.get("handleId")).and_then(|id| id.as_str());
        assert_eq!(args, Some("h1"));
    }

    #[test]
    fn a_popup_is_an_owned_open_menu_that_is_not_a_submenu() {
        assert!(suggestion_popup_owns_window(&json!({ "open": true }), "w1"));
        assert!(!suggestion_popup_owns_window(&json!({ "open": true, "submenu": true }), "w1"));
        assert!(!suggestion_popup_owns_window(&json!({ "open": true, "windowId": "w2" }), "w1"));
    }

    #[test]
    fn a_refresh_replaces_only_changed_children() {
        let mut menu = vec![row("suggest", None, vec![row("pending", None, vec![])])];
        assert!(!set_submenu_children(&mut menu, "suggest", &[row("pending", None, vec![])]));
        assert!(set_submenu_children(&mut menu, "suggest", &[row("suggestion-0", None, vec![])]));
    }
}
