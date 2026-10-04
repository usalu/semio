//! 🖱️ Context-menu construction for artifact apps: the declared-verb [`Menu`] builder, resolved in the caller's presentation
//! axes; the localized selection count phrase over the selection-kind glossary (`🧫️fixtures/🔣️.json`, schema
//! `🧬️schema/🔣️.json`, ICU oracle `🧪️tests/🟦️.ts`); and the node-graph delete-selection row, disabled with its reason when
//! nothing is selected.
use super::*;

/// 🧱️ Ergonomic `ContextMenuItemSpec` builder for `ArtifactApp::context_menu` — resolves label/icon from this app's declared
/// `ActionDefinition`/`CommandDefinition`s (via the same `AppActionRegistry` `VcsArtifactApp` enforces the actions contract
/// with) in the caller's locale and terminology, so plugins never restate them. Shortcuts are deliberately left unset — the
/// host enriches them from the keybinding registry at menu-open time (`mapContextMenuSpecs`).
///
/// ```ignore
/// Menu::of(&registry, view_state)
///     .action("deleteSelection")
///     .separator()
///     .checked("toggleGrid", grid_on)
///     .when(has_selection, |m| m.destructive("clearSelection"))
///     .submenu("align", "Align", "align-left", |m| m.action("alignLeft").action("alignRight"))
///     .build()
/// ```
pub struct Menu<'a> {
    registry: &'a AppActionRegistry,
    locale: Locale,
    terminology: Terminology,
    items: Vec<ContextMenuItemSpec>,
}

impl<'a> Menu<'a> {
    /// 🧭️ An empty menu whose declared rows resolve their labels in `axes` (the call's `ViewModel`).
    pub fn of(registry: &'a AppActionRegistry, axes: &impl LabelAxes) -> Self {
        Self { registry, locale: axes.locale(), terminology: axes.terminology(), items: Vec::new() }
    }

    fn nested(&self) -> Menu<'a> {
        Menu { registry: self.registry, locale: self.locale, terminology: self.terminology, items: Vec::new() }
    }

    /// 📌️ Appends a row for a declared action id, resolving `label`/`icon` from the app's `ActionDefinition`. An unresolvable
    /// id is dropped with a debug-mode panic (a construction-time typo, the same enforcement style as
    /// `AppBuilder::build_definition`'s ref validation); release builds skip it so a stale reference never crashes.
    pub fn action(self, action_id: impl Into<String>) -> Self {
        self.action_with_args(action_id, None)
    }

    pub fn action_args(self, action_id: impl Into<String>, args: DslValue) -> Self {
        self.action_with_args(action_id, Some(args))
    }

    fn action_with_args(mut self, action_id: impl Into<String>, args: Option<DslValue>) -> Self {
        let action_id = action_id.into();
        match self.registry.get(&action_id) {
            Some(definition) => {
                self.items.push(ContextMenuItemSpec {
                    id: action_id.clone(),
                    label: Some(definition.label.resolve(self.terminology, self.locale).to_string()),
                    icon: Some(definition.icon_id.as_str().to_string()),
                    action: Some(action_id),
                    args,
                    ..Default::default()
                });
            }
            None => debug_assert!(false, "Menu::action: unknown action id {action_id:?}"),
        }
        self
    }

    /// 🎛️ Appends a row for a declared command id (os/plugin/app/mode-scoped) — same resolution discipline as `action`,
    /// against `AppActionRegistry::get_command`.
    pub fn command(mut self, command_id: impl Into<String>) -> Self {
        let command_id = command_id.into();
        match self.registry.get_command(&command_id) {
            Some(definition) => {
                self.items.push(ContextMenuItemSpec {
                    id: command_id.clone(),
                    label: Some(definition.label.resolve(self.terminology, self.locale).to_string()),
                    icon: Some(definition.icon_id.as_str().to_string()),
                    action: Some(command_id),
                    ..Default::default()
                });
            }
            None => debug_assert!(false, "Menu::command: unknown command id {command_id:?}"),
        }
        self
    }

    /// ☑️ Same as `action`, with `checked` set — for a toggleable verb (e.g. "Show grid").
    pub fn checked(mut self, action_id: impl Into<String>, checked: bool) -> Self {
        self = self.action(action_id);
        if let Some(last) = self.items.last_mut() {
            last.checked = Some(checked);
        }
        self
    }

    /// 💥️ Same as `action`, with `destructive` set — sorts visually distinct and last in `ContextMenuController`'s rendering.
    pub fn destructive(mut self, action_id: impl Into<String>) -> Self {
        self = self.action(action_id);
        if let Some(last) = self.items.last_mut() {
            last.destructive = Some(true);
        }
        self
    }

    /// 🚫️ Keeps the most recent row visible but inert with the reason it cannot run (discoverability): the row stays
    /// focusable, never dispatches, and every host announces `reason` (resolved in this menu's axes) as its description —
    /// `.destructive("clearSelection").when(nothing_selected, |m| m.disabled_because(&nothing_selected()))`.
    pub fn disabled_because(mut self, reason: &LocalizedLabel) -> Self {
        if let Some(last) = self.items.pop() {
            self.items.push(last.disabled_because(reason.resolve(self.terminology, self.locale).to_string()));
        }
        self
    }

    /// 🧩️ Appends a fully custom row (dynamic label/hover actions/data-driven lists — the escape hatch for rows that don't map
    /// onto one declared action, e.g. a per-candidate suggestion row).
    pub fn item(mut self, item: ContextMenuItemSpec) -> Self {
        self.items.push(item);
        self
    }

    pub fn separator(mut self) -> Self {
        self.items.push(ContextMenuItemSpec { id: format!("separator-{}", self.items.len()), separator: Some(true), ..Default::default() });
        self
    }

    /// 🌿️ Appends a nested submenu, its rows built by a fresh `Menu` sharing this app's registry and axes.
    pub fn submenu(mut self, id: impl Into<String>, label: impl Into<String>, icon_id: impl Into<IconName>, build: impl FnOnce(Menu<'a>) -> Menu<'a>) -> Self {
        let children = build(self.nested()).build();
        self.items.push(ContextMenuItemSpec { id: id.into(), label: Some(label.into()), icon: Some(icon_id.into().as_str().to_string()), children: (!children.is_empty()).then_some(children), ..Default::default() });
        self
    }

    /// 🗂️ Appends a taxonomy group row (`menu.group.<category>`, `label: None` — the host resolves the localized label via
    /// `ui_wgpu::wgpu::ribbon_parent_label`) built by a fresh `Menu` sharing this app's registry and axes. Unlike `submenu`, a
    /// group's id/label are not bespoke: `organize_context_menu` (run at the `VcsArtifactApp::context_menu` funnel) merges
    /// every row sharing the same category across the whole level, dedupes their children by id, and orders groups by the
    /// canonical `RIBBON_PARENT_CATEGORIES` taxonomy.
    pub fn group(mut self, category: impl Into<String>, build: impl FnOnce(Menu<'a>) -> Menu<'a>) -> Self {
        let children = build(self.nested()).build();
        self.items.push(ContextMenuItemSpec { id: format!("menu.group.{}", category.into()), label: None, children: (!children.is_empty()).then_some(children), ..Default::default() });
        self
    }

    /// 🔀️ Conditionally applies `build` to the menu so far — the idiomatic way to gate a section on a guard (selection kind,
    /// hover target, ...) without breaking the fluent chain.
    pub fn when(self, condition: bool, build: impl FnOnce(Self) -> Self) -> Self {
        if condition {
            build(self)
        } else {
            self
        }
    }

    pub fn build(self) -> Vec<ContextMenuItemSpec> {
        self.items
    }
}

/// 🏷️ A selection kind a count phrase names; its words are the `kinds` glossary of `🧫️fixtures/🔣️.json`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SelectionKind {
    Node,
    Edge,
    Frame,
    Item,
    Part,
}

impl SelectionKind {
    pub const ALL: [Self; 5] = [Self::Node, Self::Edge, Self::Frame, Self::Item, Self::Part];

    /// 🔑️ The glossary key.
    pub fn key(self) -> &'static str {
        match self {
            Self::Node => "node",
            Self::Edge => "edge",
            Self::Frame => "frame",
            Self::Item => "item",
            Self::Part => "part",
        }
    }

    /// 📖️ The CLDR cardinal `one` and `other` forms of this kind in `locale`.
    pub fn words(self, locale: Locale) -> (&'static str, &'static str) {
        match (self, locale) {
            (Self::Node, Locale::En) => ("node", "nodes"),
            (Self::Node, Locale::De) => ("Knoten", "Knoten"),
            (Self::Edge, Locale::En) => ("edge", "edges"),
            (Self::Edge, Locale::De) => ("Kante", "Kanten"),
            (Self::Frame, Locale::En) => ("frame", "frames"),
            (Self::Frame, Locale::De) => ("Rahmen", "Rahmen"),
            (Self::Item, Locale::En) => ("item", "items"),
            (Self::Item, Locale::De) => ("Element", "Elemente"),
            (Self::Part, Locale::En) => ("part", "parts"),
            (Self::Part, Locale::De) => ("Teil", "Teile"),
        }
    }
}

/// 🗣️ The localized conjunction of every non-zero `(count, kind)`, or `None` when nothing is counted. English and German
/// both take the CLDR cardinal `one` form for exactly 1 and `other` otherwise; counts are not digit-grouped; the list joins
/// as "a and b" / "a, b, and c" or "a und b" / "a, b und c" — the phrases ICU's `Intl.ListFormat` produces.
pub fn selection_count_phrase(locale: Locale, counts: &[(usize, SelectionKind)]) -> Option<String> {
    let parts: Vec<String> = counts
        .iter()
        .filter(|(count, _)| *count > 0)
        .map(|(count, kind)| {
            let (one, other) = kind.words(locale);
            format!("{count} {}", if *count == 1 { one } else { other })
        })
        .collect();
    let (pair, serial) = match locale {
        Locale::En => (" and ", ", and "),
        Locale::De => (" und ", " und "),
    };
    match parts.as_slice() {
        [] => None,
        [only] => Some(only.clone()),
        [first, second] => Some(format!("{first}{pair}{second}")),
        [head @ .., last] => Some(format!("{}{serial}{last}", head.join(", "))),
    }
}

/// ✂️ The framework's delete-selection verb label (glossary `labels.deleteSelection`).
pub fn delete_selection() -> LocalizedLabel {
    LocalizedLabel::native("Delete selection", "Auswahl löschen")
}

/// 🫥️ Why a selection-scoped row is disabled when nothing is selected (glossary `labels.nothingSelected`).
pub fn nothing_selected() -> LocalizedLabel {
    LocalizedLabel::native("Nothing selected", "Nichts ausgewählt")
}

/// 🎯️ Node and edge ids from a context-menu surface snapshot, falling back to runtime selection.
pub fn selection_domains_from_surface(surface: Option<&ContextMenuSurfaceTarget>, fallback_nodes: &[String], fallback_edges: &[String]) -> (Vec<String>, Vec<String>) {
    let groups = surface.map_or(&[][..], |target| target.selection.as_slice());
    let mut nodes: Vec<String> = groups.iter().filter(|group| group.domain == "node").flat_map(|group| group.ids.iter().cloned()).collect();
    let mut edges: Vec<String> = groups.iter().filter(|group| group.domain == "edge").flat_map(|group| group.ids.iter().cloned()).collect();
    if nodes.is_empty() && edges.is_empty() {
        nodes = fallback_nodes.to_vec();
        edges = fallback_edges.to_vec();
    }
    (nodes, edges)
}

/// 🚦️ How delete-selection is dispatched from a node-graph context menu row.
#[derive(Clone, Copy)]
pub enum NodeGraphDeleteDispatch {
    /// ➡️ The `deleteSelection` view action (flow and similar).
    Direct,
    /// 🕸️ `nodeGraphEdit` with ONE `delete` row naming the selection's node and edge ids (design §13.3; dag, sequence,
    /// procedural, trinity).
    ViaNodeGraphEdit,
}

/// 🗑️ Delete-selection row labelled `delete_label` with the count phrase over the selection `node_ids` / `edge_ids` resolved
/// when the menu opens, in `axes` (the call's `ViewModel`). Via `nodeGraphEdit` it carries those ids as the
/// `delete {nodeIds, synapseIds}` row (`🧰️framework/🔨️modules/🛠️tool-machine/🧬️schema/🔣️node-graph-edit-rows`). An empty
/// selection keeps the row visible but disabled with the reason [`nothing_selected`], and without any action, so no row ever
/// dispatches an empty or ambient delete.
pub fn node_graph_delete_selection_spec(delete_label: &str, axes: &impl LabelAxes, node_ids: &[String], edge_ids: &[String], dispatch: NodeGraphDeleteDispatch) -> ContextMenuItemSpec {
    let row = ContextMenuItemSpec { id: "delete-selection".into(), label: Some(delete_label.to_string()), icon: Some("trash".into()), destructive: Some(true), ..Default::default() };
    let Some(phrase) = selection_count_phrase(axes.locale(), &[(node_ids.len(), SelectionKind::Node), (edge_ids.len(), SelectionKind::Edge)]) else {
        return row.disabled_because(nothing_selected().resolve(axes.terminology(), axes.locale()).to_string());
    };
    let (action, args) = match dispatch {
        NodeGraphDeleteDispatch::Direct => ("deleteSelection".to_string(), None),
        NodeGraphDeleteDispatch::ViaNodeGraphEdit => ("nodeGraphEdit".to_string(), Some(DslValue::from(&serde_json::json!({ "operations": [{ "operation": "delete", "nodeIds": node_ids, "synapseIds": edge_ids }] })))),
    };
    ContextMenuItemSpec { label: Some(format!("{delete_label} ({phrase})")), action: Some(action), args, ..row }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
