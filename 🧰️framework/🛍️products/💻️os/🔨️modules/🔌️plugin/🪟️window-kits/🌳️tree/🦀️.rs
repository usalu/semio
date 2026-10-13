//! 🌳️ Tree items, panel trees, windowed tree rows and the tree window kit.
use semio_framework_value::DslValue;
use semio_framework::{ActionArgDef, ActionDefinition, ActionKind, TreeWindowRequest, ViewModel, WindowKindDefinition, INTERACTION_SELECT_ACTION_ID};
use semio_framework_ui_contract as ui;
use semio_framework_ui_contract::BuiltNode;
/// 🧬️ SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY (`sdk-flip`, 26/08/20): the choke point. Every
/// UI-bearing plugin in the fleet imports its UI vocabulary through THIS block, so flipping it
/// here redirects all 33 at once — `ui_wgpu` leaves the guest dependency graph entirely
/// (registrar-request: drop it from this crate's `Cargo.toml`, add `semio-framework-ui-contract`
/// and `semio-framework-ui-runtime` in its place). `Ui*Node`/`UiPresence`/`UiControlNode`/
/// `ActionDescriptor` have no drop-in replacement of the same name — see
/// `📓️recipe-plugin.md` in this ticket's folder for the variant-by-variant mechanical
/// translation (`UiNode::Text{..}` → `ui::text(..)`, `ActionDescriptor` → `ActionBinding`, …).
/// `ui::*` is the contract crate's own builder DSL (`ui::stack()`, `ui::button()`, …); the bare
/// glob brings every contract type (`Component`, `UiPatch`, `Label`, `ActionId`, …) into scope
/// too, since this module already used every one of the old wgpu-target types unqualified.
use semio_framework_ui_contract::*;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::LocalizedLabel;
/// 🧾️ The body-wide node budget and its fixed-node reserve live ONCE, in the UI contract, because
/// the React host spends the same number from the other side of the wire
/// (`🌳️Tree/🟦️.tsx`'s `capTreeWindowRequests`). See
/// [`semio_framework_ui_contract::TREE_WINDOW_BODY_NODE_BUDGET`] for the cost model both sides
/// implement; [`TreeWindows`] is this side of it.
use semio_framework_ui_contract::{TREE_WINDOW_BODY_NODE_BUDGET, TREE_WINDOW_FIXED_NODE_HEADROOM, TREE_WINDOW_PATH_SEPARATOR};
use crate::window_kits::ActionFactory;
use crate::sdk_fault::PluginAssemblyError;
use crate::table_kit::activation_target;
use crate::window_kits::WindowKit;
use crate::window_surface::UiAssemblyResult;
use crate::window_surface::ui_assembly_error;
use crate::window_surface::ui_label;
use crate::window_surface::ui_text;
use crate::window_surface::window_kind_definition;
#[cfg(test)]
use crate::history_view::CommandView;
#[cfg(test)]
use crate::history_view::HistoryCommandFilter;
#[cfg(test)]
use crate::history_view::HistoryView;
#[cfg(test)]
use crate::command_history::ui_history_panel;



// 🌳️ Shared panel-tree builders — lifts the verbatim-duplicated `tree_item*`/`selection_ids` helpers
// and the `build_document_tree`/`build_inspector_tree`/`build_catalogue_tree` skeleton found across
// ~15 plugin crates (flow, procedural, layout, gis, puzzle, sequence, trinity, dag, …) into the SDK.

/// 🌳️ A bare tree item — thin wrapper over [`ui::tree_item`], `id` becoming the row's own key.
pub fn tree_item<I: AsRef<str>, L: TryInto<Label>>(id: I, label: L) -> UiAssemblyResult<BuiltNode> {
    let builder = ui::tree_item(ui_label(label, "tree-item.label")?);
    let builder = builder.try_id(id).map_err(|_| ui_assembly_error("tree-item.id"))?;
    builder.try_build().map_err(|_| ui_assembly_error("tree-item.build"))
}

/// 🌳️ A tree item with a description line.
pub fn tree_item_desc<I: AsRef<str>, L: TryInto<Label>>(id: I, label: L, description: Option<String>) -> UiAssemblyResult<BuiltNode> {
    let builder = ui::tree_item(ui_label(label, "tree-item-description.label")?);
    let mut builder = builder.try_id(id).map_err(|_| ui_assembly_error("tree-item-description.id"))?;
    if let Some(description) = description {
        builder = builder.description(ui_text(description, "tree-item-description.description")?);
    }
    builder.try_build().map_err(|_| ui_assembly_error("tree-item-description.build"))
}

/// 🌳️ A non-interactive grouping row that owns nested children — a disclosure header inside a
/// section. It binds no action and carries no argument map, so unlike an interactive row it costs
/// nothing of the process-wide `UiValue` arena (see [`TreeWindows`]): grouping a windowed listing
/// by category is free, only the leaf rows are charged.
pub fn tree_group<I: AsRef<str>, L: TryInto<Label>>(id: I, label: L, default_open: bool, children: UiFixedList<BuiltNode>) -> UiAssemblyResult<BuiltNode> {
    let builder = ui::tree_item(ui_label(label, "tree-group.label")?);
    let builder = builder.try_id(id).map_err(|_| ui_assembly_error("tree-group.id"))?.default_open(default_open);
    builder.try_children(children).map_err(|_| ui_assembly_error("tree-group.children"))?.try_build().map_err(|_| ui_assembly_error("tree-group.build"))
}

/// 🌳️ A tree item that dispatches `action` on activation while preserving its typed args — the row's ONE target,
/// the action's verb its activation.
pub fn tree_item_with_action<I: AsRef<str>, L: TryInto<Label>>(id: I, label: L, description: Option<String>, action: (ActionId, Option<UiValue>)) -> UiAssemblyResult<BuiltNode> {
    let builder = ui::tree_item(ui_label(label, "tree-item-action.label")?);
    let mut builder = builder.try_id(id).map_err(|_| ui_assembly_error("tree-item-action.id"))?.target(activation_target(action)?);
    if let Some(description) = description {
        builder = builder.description(ui_text(description, "tree-item-action.description")?);
    }
    builder.try_build().map_err(|_| ui_assembly_error("tree-item-action.build"))
}

/// 🌳️ A draggable tree item: `drag_data` is a JSON object whose entries become the item's
/// MIME-type -> payload drag-data map (string values are used verbatim; non-string values are
/// serialized), e.g. `json!({ "application/x-my-widget": descriptor.to_string() })`. Generalizes the
/// single-hardcoded-MIME-key pattern duplicated per app (each app previously baked its own MIME
/// constant into this function) — the caller now supplies the key(s) explicitly.
pub fn tree_item_with_action_draggable<I: AsRef<str>, L: TryInto<Label>>(id: I, label: L, description: Option<String>, action: (ActionId, Option<UiValue>), drag_data: &semio_framework_pack_json::Value) -> UiAssemblyResult<BuiltNode> {
    let builder = ui::tree_item(ui_label(label, "draggable-tree-item.label")?);
    let mut builder = builder.try_id(id).map_err(|_| ui_assembly_error("draggable-tree-item.id"))?.target(activation_target(action)?).draggable(true);
    if let Some(description) = description {
        builder = builder.description(ui_text(description, "draggable-tree-item.description")?);
    }
    if let Some(entries) = drag_data.as_object() {
        let mut fixed = UiFixedMap::default();
        let mut last = UiText::default();
        while fixed.len() < entries.len() {
            let Some((key, value)) = entries.iter().filter(|(key, _)| *key > last.as_str()).min_by_key(|(key, _)| *key) else { break };
            let key = ui_text(key.to_string(), "draggable-tree-item.drag-key")?;
            let value = ui_text(value.as_str().map_or_else(|| value.to_string(), str::to_string), "draggable-tree-item.drag-value")?;
            last = key.clone();
            fixed.try_push(key, value).map_err(|_| ui_assembly_error("draggable-tree-item.drag-data"))?;
        }
        if fixed.len() != entries.len() {
            return Err(ui_assembly_error("draggable-tree-item.drag-data"));
        }
        builder = builder.drag_data(fixed);
    }
    builder.try_build().map_err(|_| ui_assembly_error("draggable-tree-item.build"))
}

/// 🎯️ Parses a selection-action's `ids` array arg into a plain `Vec<String>` — the one selection-argument shape
/// every app's selection actions accept.
pub fn selection_ids(args: Option<&DslValue>) -> Vec<String> {
    args.and_then(|value| value.get("ids")).and_then(DslValue::as_array).and_then(|items| items.iter().map(|item| item.as_str().map(str::to_string)).collect::<Option<Vec<String>>>()).unwrap_or_default()
}

/// 🌳️ Fluent builder for the `build_document_tree`/`build_inspector_tree`/`build_catalogue_tree`
/// skeleton duplicated across plugin crates: namespaced item ids, sections (optionally substituting a
/// single "(none)" placeholder item for the empty state), a selected/highlighted id set, an
/// `interaction_domain` binding, and a drop action — ending in `.build()` -> a [`BuiltNode`] (a
/// `Component::Tree`; wrap with the crate's own `built_to_tree` + `ComponentTree::new` at the one
/// place `Present::present` actually returns, per `📓️recipe-plugin.md` §5).
///
/// ⚠️ Gap (SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY packet `sdk-helpers` — see
/// `📓️terra-sdk-helpers-report.md`): `.selected()`/`.highlighted()` used to stamp
/// `presence.selected`/`.hovered` directly onto matching items via the old `ui_tree_stamp_presence`.
/// That field no longer exists on a built node — selection/hover is now
/// `semio_framework_ui_contract::PresenceUpdate`, a separate TTL-scoped channel published through a
/// live `transact()` call this SDK layer does not have (`📓️recipe-plugin.md` §6; `sdk-flip`'s own
/// decision 1 — the reactor embeds `SurfaceReconciler` only, not the full `UiRuntime` a
/// `PresenceHub` publish needs). `selected_ids`/`highlighted_ids` are still recorded here — a
/// future packet wiring a real per-app `Present`/`HandleIntent` can read them straight off this
/// builder to publish the equivalent `PresenceUpdate` — but `.build()` cannot render them onto the
/// tree this wave. Framework-owned interaction-domain trees lose the identical capability at
/// `stamp_and_cache_interaction_ui`, for the identical reason — see that method's own doc comment.
pub struct PanelTreeBuilder {
    namespace: UiText,
    sections: UiFixedList<BuiltNode>,
    selected_ids: Option<UiFixedList<UiText>>,
    highlighted_ids: Option<UiFixedList<UiText>>,
    interaction_domain: Option<UiText>,
    interaction_select: Option<(ActionId, Option<UiValue>)>,
    drop_action: Option<ActionId>,
}

impl PanelTreeBuilder {
    /// 🌳️ `namespace` prefixes every id built via `.item_id()`, e.g. `"flow-play-document"`.
    pub fn new(namespace: impl TryInto<UiText>) -> UiAssemblyResult<Self> {
        Ok(Self { namespace: ui_text(namespace, "panel-tree.namespace")?, sections: UiFixedList::default(), selected_ids: None, highlighted_ids: None, interaction_domain: None, interaction_select: None, drop_action: None })
    }

    /// 🌳️ Builds a namespaced item id: `"{namespace}.{kind}.{id}"`.
    pub fn item_id(&self, kind: &str, id: &str) -> UiAssemblyResult<UiText> {
        UiText::try_format(format_args!("{}.{kind}.{id}", self.namespace)).ok_or_else(|| ui_assembly_error("panel-tree.item-id"))
    }

    /// 🌳️ Adds a section verbatim. ⚠️ Its rows are OUTSIDE the [`TreeWindows`] node ledger — see
    /// that type and [`TREE_WINDOW_FIXED_NODE_HEADROOM`] — so this spelling is for small fixed
    /// lists only; an entity list belongs on [`Self::window_section`].
    /// ⚠️ Decision: a `None` label becomes an empty [`Label`] —
    /// [`ui::tree_section`] has no optional-label constructor; [`TreeSectionProps::label`] itself
    /// stays `Option` on the record, so only this convenience path loses the "omit entirely"
    /// spelling.
    pub fn section<I: AsRef<str>>(mut self, id: I, label: Option<Label>, default_open: bool, items: UiFixedList<BuiltNode>) -> UiAssemblyResult<Self> {
        let builder = tree_section(label.unwrap_or_default()).default_open(default_open);
        let builder = builder.try_id(id).map_err(|_| ui_assembly_error("panel-tree.section-id"))?;
        let builder = builder.try_children(items).map_err(|_| ui_assembly_error("panel-tree.section-items"))?;
        let node = builder.try_build().map_err(|_| ui_assembly_error("panel-tree.section-build"))?;
        self.sections.try_push(node).map_err(|_| ui_assembly_error("panel-tree.sections"))?;
        Ok(self)
    }

    /// 🌳️ Adds a section, substituting a single "(none)"-style placeholder item when `items` is empty —
    /// the empty-state pattern duplicated in `build_document_tree`/`build_catalogue_tree` across apps.
    pub fn section_or_placeholder<I: AsRef<str>, L: TryInto<Label>>(self, id: I, label: Option<Label>, default_open: bool, mut items: UiFixedList<BuiltNode>, placeholder_label: L) -> UiAssemblyResult<Self> {
        if items.is_empty() {
            let empty_id = UiText::try_format(format_args!("{}.empty", id.as_ref())).ok_or_else(|| ui_assembly_error("panel-tree.placeholder-id"))?;
            items.try_push(tree_item(empty_id, placeholder_label)?).map_err(|_| ui_assembly_error("panel-tree.placeholder"))?;
        }
        self.section(id, label, default_open, items)
    }

    /// 🪟️ Adds a windowed section — see [`tree_window_section`]. The panel hands the same
    /// [`TreeWindows`] to every container it builds, so the shared first-paint budget is spent in
    /// document order.
    pub fn window_section<T>(mut self, windows: &TreeWindows<'_>, id: &str, label: Option<Label>, default_open: bool, entries: &[T], row: impl FnMut(&T) -> UiAssemblyResult<BuiltNode>) -> UiAssemblyResult<Self> {
        let node = tree_window_section(windows, id, label.unwrap_or_default(), default_open, entries, row)?;
        self.sections.try_push(node).map_err(|_| ui_assembly_error("panel-tree.sections"))?;
        Ok(self)
    }

    /// 🪟️ Adds indexed rows without materializing the full domain list.
    pub fn window_indexed_section(mut self,windows:&TreeWindows<'_>,id:&str,label:Option<Label>,default_open:bool,total:usize,row:impl FnMut(usize)->UiAssemblyResult<BuiltNode>)->UiAssemblyResult<Self> {
        let node=tree_window_indexed_section(windows,id,label.unwrap_or_default(),default_open,total,row)?;
        self.sections.try_push(node).map_err(|_|ui_assembly_error("panel-tree.sections"))?;
        Ok(self)
    }

    /// 🪟️ [`Self::window_section`] with an empty-state placeholder row — see
    /// [`tree_window_section_or_placeholder`].
    pub fn window_section_or_placeholder<T, L: TryInto<Label>>(
        mut self,
        windows: &TreeWindows<'_>,
        id: &str,
        label: Option<Label>,
        default_open: bool,
        entries: &[T],
        row: impl FnMut(&T) -> UiAssemblyResult<BuiltNode>,
        placeholder_label: L,
    ) -> UiAssemblyResult<Self> {
        let node = tree_window_section_or_placeholder(windows, id, label.unwrap_or_default(), default_open, entries, row, placeholder_label)?;
        self.sections.try_push(node).map_err(|_| ui_assembly_error("panel-tree.sections"))?;
        Ok(self)
    }

    pub fn selected<I: IntoIterator<Item = String>>(mut self, ids: I) -> UiAssemblyResult<Self> {
        let mut admitted = UiFixedList::default();
        for id in ids {
            let id = ui_text(id, "panel-tree.selected-id")?;
            admitted.try_push(id).map_err(|_| ui_assembly_error("panel-tree.selected"))?;
        }
        self.selected_ids = Some(admitted);
        Ok(self)
    }

    pub fn highlighted<I: IntoIterator<Item = String>>(mut self, ids: I) -> UiAssemblyResult<Self> {
        let mut admitted = UiFixedList::default();
        for id in ids {
            let id = ui_text(id, "panel-tree.highlighted-id")?;
            admitted.try_push(id).map_err(|_| ui_assembly_error("panel-tree.highlighted"))?;
        }
        self.highlighted_ids = Some(admitted);
        Ok(self)
    }

    /// 🕹️ Binds the built tree to an app-declared `InteractionDefinition` domain id — the framework
    /// then owns this domain's selection/hover, replacing the deleted per-app `selection_change` action
    /// — AND stamps the one tree-level `Trigger::Activate` binding
    /// (`interactionSelect { domainId }`) every pick row in the tree dispatches through. A row marked
    /// with [`TreeItemBuilder::granularity`] therefore needs no binding and no argument map of its
    /// own: it costs zero `UiValue` arena, which is what lets a window of 128 rows exist at all.
    pub fn interaction_domain(mut self, controller_id: &'static str, domain: impl TryInto<UiText>) -> UiAssemblyResult<Self> {
        let domain = ui_text(domain, "panel-tree.interaction-domain")?;
        let mut args = UiMapBuilder::try_new().ok_or_else(|| ui_assembly_error("panel-tree.interaction-args"))?;
        args.push("domainId".to_owned(), UiValue::Text(domain.clone())).map_err(|_| ui_assembly_error("panel-tree.interaction-args"))?;
        self.interaction_select = Some(ActionFactory::new(controller_id).action(INTERACTION_SELECT_ACTION_ID, Some(UiValue::Map(args.finish())))?);
        self.interaction_domain = Some(domain);
        Ok(self)
    }

    pub fn drop_action(mut self, action: ActionId) -> Self {
        self.drop_action = Some(action);
        self
    }

    /// 🌳️ Terminal — see this struct's own doc for why `.selected()`/`.highlighted()` no longer
    /// reach the built tree this wave.
    pub fn build(self) -> UiAssemblyResult<BuiltNode> {
        let builder = tree().try_id(&self.namespace).map_err(|_| ui_assembly_error("panel-tree.root-id"))?;
        let mut builder = builder.try_children(self.sections).map_err(|_| ui_assembly_error("panel-tree.root-sections"))?;
        if let Some(domain) = self.interaction_domain {
            builder = builder.interaction_domain(domain);
        }
        if let Some((action, args)) = self.interaction_select {
            builder = match args {
                Some(args) => builder.try_on_with(Trigger::Activate, action, args).map_err(|_| ui_assembly_error("panel-tree.interaction-binding"))?,
                None => builder.try_on(Trigger::Activate, action).map_err(|_| ui_assembly_error("panel-tree.interaction-binding"))?,
            };
        }
        if let Some(action) = self.drop_action {
            builder = builder.try_on(Trigger::Drop, action).map_err(|_| ui_assembly_error("panel-tree.drop-binding"))?;
        }
        builder.try_build().map_err(|_| ui_assembly_error("panel-tree.root-build"))
    }
}

// 🪟️ Shared virtualised-panel windowing — a panel body is NOT a row per entity, and it is not a
// page with a `+N` tail either. Every container reports its FULL extent through
// `TreeWindow { total, offset }` and materialises only the rows the host asked for, so the host's
// scrollbar spans the whole document while the built tree stays inside `UI_BUILT_CHILDREN_MAX`
// children and the process-wide `UiValue` argument arena. Open state and scroll offset are
// host-owned and arrive per render through `ViewModel::tree_windows`; the guest keeps no cursor.

/// 🪟️ Rows one first paint materialises across a body's containers together, before the host has
/// measured its viewport (`ViewModel::tree_viewport_rows`).
pub const TREE_WINDOW_DEFAULT_ROWS: u32 = 48;

/// 🧾️ Items one fixed (unwindowed) record or windowed container record of a body may charge the surface
/// reconciliation — the item-side price of [`TREE_WINDOW_FIXED_NODE_HEADROOM`].
pub const TREE_WINDOW_FIXED_NODE_ITEMS: usize = 32;

/// 🧾️ The most reconciliation items a windowed row keeps back from the windows nested inside it (a quarter of what is left when less): a nested
/// window that fills the whole remaining budget would leave its enclosing row (its own controls, bindings and argument maps) unaffordable
/// and drop that row, and every row after it, from the first paint.
pub const TREE_WINDOW_ROW_OWN_ITEMS: usize = 512;

/// 🧾️ The body-wide ITEM budget windowed rows spend: the surface reconciler's item ceiling
/// ([`semio_framework_ui_runtime::SURFACE_RECONCILE_MAX_ITEMS`], less its one root seed) less what the fixed
/// records may charge. Rows carrying their own argument maps reach this long before the node budget — 28
/// Home rows with five row actions each do — so [`TreeWindows`] prices every materialised row with
/// [`semio_framework_ui_runtime::surface_subtree_items`] and ends a window early rather than letting the
/// reconciler refuse the whole surface (ticket 26/09/23 U5: 138 Home spaces faulted `items 4098 > 4097`).
pub const TREE_WINDOW_BODY_ITEM_BUDGET: usize = semio_framework_ui_runtime::SURFACE_RECONCILE_MAX_ITEMS - 1 - TREE_WINDOW_FIXED_NODE_HEADROOM * TREE_WINDOW_FIXED_NODE_ITEMS;

/// 🪟️ What one container materialises this render: `len` rows starting at `offset` out of `total`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TreeSlice {
    pub open: bool,
    pub offset: usize,
    pub len: usize,
    pub total: usize,
}

/// 🪟️ The host's windows for one panel body, read once per render off `ViewModel::tree_windows`.
/// Containers the host has never seen share `budget` — the first-paint allowance spent in
/// document order, nested containers included — so a cold render draws about one viewport and
/// stops, instead of the whole document.
///
/// 🧾️ `nodes` is the body-wide **node ledger**, and it is the reason per-container clamps are not
/// enough. The reconciler admits at most [`semio_framework_ui_contract::UI_DOCUMENT_NODES`] node
/// records per PRESENTED BODY — the tree root, every section node, every group item, every row and
/// every placeholder counts once (`SurfaceReconcileLimits::max_nodes`, enforced per admitted node
/// in `🧠️runtime/♻️reconcile/🦀️.rs`). A body whose containers each stayed inside
/// `UI_BUILT_CHILDREN_MAX` could still sum past that ceiling and fault the whole refresh — fem3d
/// House did, at `nodes: 129 > max_nodes: 128`.
///
/// 🧾️ **Cost model — one budget, one arithmetic, both sides of the wire.** The ledger opens on
/// [`semio_framework_ui_contract::TREE_WINDOW_BODY_NODE_BUDGET`], the SAME number the React host
/// caps its requests with, and one windowed container costs `1 + its materialised rows`. Every
/// node is charged EXACTLY ONCE: a nested [`tree_window_item`] built as a row of an enclosing
/// window is the row its parent already paid for, so it does not charge its own `1` again (the
/// helpers track that they are inside a parent's row closure). So a request the host is allowed to
/// file — it caps `Σ(1 + rows) ≤ TREE_WINDOW_BODY_NODE_BUDGET` — is always one this side can
/// honour in full.
///
/// 🥇️ **Request priority — order independence.** A first-paint default must never eat the records
/// a container the user actually scrolled to needs, and the host's request order is its node keys
/// sorted alphabetically (wire byte-stability), NOT document order. So the ledger is split the
/// moment it is built, before any container exists: each host request reserves
/// `1 + min(rows, UI_BUILT_CHILDREN_MAX)`, in request order, until the budget is spent — a host
/// that over-asks is clamped there, deterministically, tail last. A container the host has NOT
/// addressed is then served out of the UNRESERVED remainder only, and additionally out of the
/// shared viewport `budget`, in document order. A container the host HAS addressed releases its
/// own reservation when the render reaches it and is served out of the whole remaining ledger,
/// wherever it sits in the body. `rows: 0` with `open: true` is a valid request — a measured but
/// off-screen container — and materialises nothing; it never falls back to the first-paint default.
/// A stale `offset` past the end clamps to the LAST window (`total - rows`), never to an empty
/// slice of a non-empty container.
///
/// 🔑️ **A container is addressed by its window PATH, never by its bare node key.** The path is the
/// enclosing windowed containers' keys, outermost first, then its own key, joined by
/// [`semio_framework_ui_contract::TREE_WINDOW_PATH_SEPARATOR`] — see [`Self::path_of`]. A
/// top-level section's path IS its key, so a flat body is byte-identical to a bare-key one; a
/// nested one is `"…load-cases\u{1f}dead"`. This is what makes container identity collision-free
/// BY CONSTRUCTION without touching node keys or pick target ids: a load case and a combination
/// both called `uls`, or one object id repeated across four cad pane sections, differ in their
/// parents and therefore in their paths, and keep independent open and window state.
///
/// 🔑️ Two containers can then collide only as TRUE SIBLINGS under one parent — which the UI
/// document itself already refuses (`DuplicateSiblingKey`). The second sibling to claim a path is
/// refused here first, with `ui.tree-window.duplicate-key`, so the author sees the cause rather
/// than the symptom. A container with no entries the host has never addressed publishes no window,
/// is not addressable, and therefore claims nothing: a tree whose leaves are built through
/// [`tree_window_item`] (a JSON document, an AST) is bound by this rule only at the nodes that
/// actually have children.
///
/// 🧾️ Running out is not a fault. A container the ledger cannot seat still builds and still stamps
/// its full `TreeWindow { total, offset }` with a shorter — possibly empty — materialised run, so
/// the host's scrollbar keeps spanning the whole document and the next scroll streams the rows in.
///
/// ⚠️ Rows built any other way are OUTSIDE the ledger: [`PanelTreeBuilder::section`],
/// [`PanelTreeBuilder::section_or_placeholder`], bare [`tree_item`]/[`tree_section`] children and
/// [`ui_node_list`] admissions are invisible to it, because they carry no window and no node key
/// for the host to address. Those are the small fixed lists apps legitimately still build, and
/// [`TREE_WINDOW_FIXED_NODE_HEADROOM`] is what pays for them. A panel that builds MORE than the
/// headroom in fixed rows must move them onto a window. A container's own node is debited
/// unconditionally — it is not optional once the panel decided to build it — so an overdrawn
/// ledger simply grants nothing to everything after it.
pub struct TreeWindows<'a> {
    requests: Vec<(&'a TreeWindowRequest, std::cell::Cell<usize>)>,
    budget: std::cell::Cell<u32>,
    nodes: std::cell::Cell<isize>,
    items: std::cell::Cell<usize>,
    reserved: std::cell::Cell<usize>,
    claimed: std::cell::RefCell<Vec<String>>,
    path: std::cell::RefCell<Vec<String>>,
}

impl<'a> TreeWindows<'a> {
    /// 🪟️ Every request the host filed for `body_key`, plus its measured viewport budget.
    pub fn for_body(view: &'a ViewModel, body_key: &str) -> Self {
        Self::seated(view.tree_windows.iter().filter(|request| request.body_key == body_key), view.tree_viewport_rows.unwrap_or(TREE_WINDOW_DEFAULT_ROWS))
    }

    /// 🪟️ No host state at all (tests, a body the host has never rendered): author defaults and
    /// the default first-paint budget.
    pub fn unhosted() -> Self {
        Self::seated(std::iter::empty(), TREE_WINDOW_DEFAULT_ROWS)
    }

    /// 🥇️ Seats the host's requests on the ledger in request order — see this type's doc. A key the
    /// host filed twice is seated once: only the first is ever read back, so reserving for the
    /// second would hold records nothing can release.
    fn seated(requests: impl IntoIterator<Item = &'a TreeWindowRequest>, viewport_rows: u32) -> Self {
        let mut unreserved = TREE_WINDOW_BODY_NODE_BUDGET;
        let mut seen: Vec<&str> = Vec::new();
        let requests: Vec<_> = requests
            .into_iter()
            .map(|request| {
                let want = match request.open {
                    Some(false) => 0,
                    _ if seen.contains(&request.node_key.as_str()) => 0,
                    _ => (1 + (request.rows as usize).min(UI_BUILT_CHILDREN_MAX)).min(unreserved),
                };
                seen.push(request.node_key.as_str());
                unreserved -= want;
                (request, std::cell::Cell::new(want))
            })
            .collect();
        Self {
            requests,
            budget: std::cell::Cell::new(viewport_rows),
            nodes: std::cell::Cell::new(TREE_WINDOW_BODY_NODE_BUDGET as isize),
            items: std::cell::Cell::new(TREE_WINDOW_BODY_ITEM_BUDGET),
            reserved: std::cell::Cell::new(TREE_WINDOW_BODY_NODE_BUDGET - unreserved),
            claimed: std::cell::RefCell::new(Vec::new()),
            path: std::cell::RefCell::new(Vec::new()),
        }
    }

    /// 🔑️ The **window path** `node_key` is addressed by from where the render currently stands:
    /// the enclosing windowed containers' keys, outermost first, then `node_key`, joined by
    /// [`semio_framework_ui_contract::TREE_WINDOW_PATH_SEPARATOR`]. At the top level this is
    /// `node_key` itself, so a flat body's requests and laws are byte-identical to before.
    ///
    /// 🔑️ A path is a **view-context identifier**: printable, at most 256 code points, because it
    /// crosses the process boundary inside `ViewModel::tree_windows` and both sides admit it with
    /// the same law (`parseResolvedPluginViewState`, `🛂️manifest/🟦️.ts`). A path longer than that
    /// is one the host never files a request for, so the container it names simply renders as
    /// UNREQUESTED — the author's default plus the shared first-paint budget — which is already
    /// what an absent seat means here.
    pub fn path_of(&self, node_key: &str) -> String {
        let path = self.path.borrow();
        if path.is_empty() {
            return node_key.to_owned();
        }
        let mut joined = String::new();
        for segment in path.iter() {
            joined.push_str(segment);
            joined.push_str(TREE_WINDOW_PATH_SEPARATOR);
        }
        joined.push_str(node_key);
        joined
    }

    /// 🧾️ Whether the render currently stands inside an enclosing window's row closure — the
    /// container being built there is that row, already charged by its parent.
    fn is_nested(&self) -> bool {
        !self.path.borrow().is_empty()
    }

    /// 🧾️ Node records this body may still materialise — every container and every row this
    /// [`TreeWindows`] has served has already been charged. Zero once the ledger is spent, including
    /// when unconditional container debits drove it past zero.
    pub fn nodes_remaining(&self) -> usize {
        self.nodes.get().max(0) as usize
    }

    /// 🥇️ Records still held back for host requests this render has not reached yet.
    pub fn nodes_reserved(&self) -> usize {
        self.reserved.get()
    }

    /// 🧾️ Reconciliation items windowed rows may still charge — see [`TREE_WINDOW_BODY_ITEM_BUDGET`].
    pub fn items_remaining(&self) -> usize {
        self.items.get()
    }

    /// 🧾️ Gives back the node records of `rows` a window was granted but did not materialise.
    fn refund(&self, rows: usize) {
        self.nodes.set(self.nodes.get() + rows as isize);
    }

    /// 🔑️ Registers `node_key` as this body's, refusing the second container to claim it — see
    /// this type's doc. Only a container that PUBLISHES a window claims: one with no entries the
    /// host has never addressed stamps nothing, so it is not addressable and two of them cannot
    /// steer each other. That is what keeps a tree of leaf rows (a JSON document, an AST) out of
    /// the uniqueness requirement — it binds containers, not rows.
    /// 🔑️ A container key carrying the path separator would make its own path ambiguous — it could
    /// not be told from a parent/child pair — so it is refused at assembly rather than mis-addressed.
    pub(crate) fn admit_key(node_key: &str) -> UiAssemblyResult<()> {
        if node_key.contains(TREE_WINDOW_PATH_SEPARATOR) {
            return Err(PluginAssemblyError::new("ui.tree-window.separator-in-key", format!("[tree-window] node key {node_key:?} contains the window-path separator {TREE_WINDOW_PATH_SEPARATOR:?}, which would make its path ambiguous")));
        }
        Ok(())
    }

    pub(crate) fn claim_window(&self, path: &str, entries: usize) -> UiAssemblyResult<()> {
        if entries == 0 && self.seat(path).is_none() {
            return Ok(());
        }
        self.claim(path)
    }

    fn claim(&self, node_key: &str) -> UiAssemblyResult<()> {
        let mut claimed = self.claimed.borrow_mut();
        if claimed.iter().any(|key| key == node_key) {
            return Err(PluginAssemblyError::new(
                "ui.tree-window.duplicate-key",
                format!("[tree-window] duplicate key {node_key:?} in this panel body — two windowed containers share one window and one open state; give every windowed container its own node key"),
            ));
        }
        claimed.push(node_key.to_owned());
        Ok(())
    }

    /// 🧾️ Takes `nodes` records off the ledger whether or not it can afford them — a container's
    /// own node is not optional once the panel decided to build it.
    fn debit(&self, nodes: usize) {
        self.nodes.set(self.nodes.get() - nodes as isize);
    }

    /// 🧾️ Charges the node of a container that is about to be built — nothing when the container is
    /// itself a row of an enclosing window, which already paid for it (charged exactly once).
    pub(crate) fn debit_container(&self) {
        if !self.is_nested() {
            self.debit(1);
        }
    }

    /// 🧾️ Takes up to `want` records off the ledger, returning what it could actually afford.
    fn grant(&self, want: usize) -> usize {
        let spent = want.min(self.nodes_remaining());
        self.debit(spent);
        spent
    }

    /// 🥇️ [`Self::grant`] out of the records no host request is holding — the first-paint path.
    fn grant_unreserved(&self, want: usize) -> usize {
        let spent = want.min(self.nodes_remaining().saturating_sub(self.reserved.get()));
        self.debit(spent);
        spent
    }

    fn seat(&self, node_key: &str) -> Option<&(&'a TreeWindowRequest, std::cell::Cell<usize>)> {
        self.requests.iter().find(|(request, _)| request.node_key == node_key)
    }

    fn request(&self, node_key: &str) -> Option<&'a TreeWindowRequest> {
        self.seat(node_key).map(|(request, _)| *request)
    }

    /// 🔽️ Whether `node_key` is expanded — a host `open` wins over the author's default. The key
    /// is resolved against the current [`Self::path_of`] scope, so callers pass the plain id.
    pub fn is_open(&self, node_key: &str, default_open: bool) -> bool {
        self.request(&self.path_of(node_key)).and_then(|request| request.open).unwrap_or(default_open)
    }

    /// 🪟️ The slice `node_key` materialises this render out of `total` logical entries, charged to
    /// the body-wide node ledger. A container the host asked for is served out of its own
    /// reservation and cannot be starved by the containers before it, wherever it sits in the body;
    /// a container on its first paint gets only what no request is holding, and only as much as the
    /// shared viewport budget still allows.
    pub fn slice(&self, node_key: &str, default_open: bool, total: usize) -> TreeSlice {
        self.sliced(&self.path_of(node_key), default_open, total)
    }

    pub(crate) fn sliced(&self, path: &str, default_open: bool, total: usize) -> TreeSlice {
        self.sliced_capped(path, default_open, total, usize::MAX)
    }

    pub(crate) fn sliced_capped(&self, path: &str, default_open: bool, total: usize, maximum: usize) -> TreeSlice {
        let seat = self.seat(path);
        if let Some((_, reservation)) = seat {
            self.reserved.set(self.reserved.get().saturating_sub(reservation.replace(0)));
        }
        let request = seat.map(|(request, _)| *request);
        if !request.and_then(|request| request.open).unwrap_or(default_open) {
            return TreeSlice { open: false, offset: 0, len: 0, total };
        }
        let Some(request) = request else {
            let len = self.grant_unreserved(total.min(maximum).min(UI_BUILT_CHILDREN_MAX).min(self.budget.get() as usize));
            self.budget.set(self.budget.get() - len as u32);
            return TreeSlice { open: true, offset: 0, len, total };
        };
        let rows = (request.rows as usize).min(maximum).min(UI_BUILT_CHILDREN_MAX);
        let offset = (request.offset as usize).min(total.saturating_sub(rows.max(1)));
        let len = self.grant(rows.min(total - offset));
        TreeSlice { open: true, offset, len, total }
    }

    pub(crate) fn projection_slice(&self, path: &str, default_open: bool, total: usize, first_paint: usize, maximum: usize) -> TreeSlice {
        let seat = self.seat(path);
        if let Some((_, reservation)) = seat {
            self.reserved.set(self.reserved.get().saturating_sub(reservation.replace(0)));
        }
        let request = seat.map(|(request, _)| *request);
        if !request.and_then(|request| request.open).unwrap_or(default_open) {
            return TreeSlice { open: false, offset: 0, len: 0, total };
        }
        let rows = request.map_or(first_paint, |request| request.rows as usize).min(maximum).min(UI_BUILT_CHILDREN_MAX).min(total);
        let offset = request.map_or(0, |request| request.offset as usize).min(total.saturating_sub(rows.max(1)));
        TreeSlice { open: true, offset, len: rows.min(total - offset), total }
    }

    /// 🪟️ The contract stamp a slice publishes to the host.
    pub fn window(slice: &TreeSlice) -> TreeWindow {
        TreeWindow { total: slice.total as u32, offset: slice.offset as u32, row_extent: TreeWindowRowExtent::Standard }
    }

    pub(crate) fn stamp(&self, path: &str, slice: &TreeSlice) -> Option<TreeWindow> {
        (slice.total > 0 || self.seat(path).is_some()).then(|| Self::window(slice))
    }
}

thread_local! {
    /// 🎟️ Rows the windows of the render in progress left unbuilt because the process-wide `UiValue` arena had no
    /// credit for them — see [`take_arena_unbuilt_rows`].
    static ARENA_UNBUILT_ROWS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// 🎟️ Takes (and resets) the rows the renders since the last call left unbuilt for want of `UiValue` arena credit.
/// The reactor reads it after every surface render and renders that surface once more when the arena can price
/// them again; a law reads it to prove a short window was cut by the arena, not by the host's request.
pub fn take_arena_unbuilt_rows() -> usize {
    ARENA_UNBUILT_ROWS.with(|rows| rows.replace(0))
}

/// 🪟️ Pushes a slice's rows straight into a container builder's children. A row refused with
/// `ui.fixed-capacity` — the process-global argument arena taken by another panel mid-build, or a
/// full `BuiltChildren` — or one whose whole subtree the remaining [`TREE_WINDOW_BODY_ITEM_BUDGET`]
/// cannot pay for ends the window early with a shorter materialised run, and gives back the records
/// and items it and the unbuilt rest held; any other error propagates. A row the process-wide `UiValue`
/// arena can no longer price is not started at all: [`semio_framework_ui_contract::ui_value_headroom`] must
/// still admit one row of [`semio_framework_ui_contract::UI_VALUE_ROW_COLLECTIONS`] collections, because the
/// arena is ONE page shared by every panel and by the previous generations the reactor is still retiring, and
/// a window that stops there stamps its full extent like any other short run. Rows are built with the
/// ledger's `nested` flag raised, so a row that is itself a windowed container does not charge its own
/// node twice, and its subtree's item price replaces what its nested windows charged.
pub(crate) fn tree_window_indexed_rows<B: HasChildren>(mut builder: B, windows: &TreeWindows<'_>, id: &str, slice: &TreeSlice, mut row: impl FnMut(usize) -> UiAssemblyResult<BuiltNode>) -> UiAssemblyResult<B> {
    for (built, index) in (slice.offset..slice.offset + slice.len).enumerate() {
        let (nodes, items) = (windows.nodes.get(), windows.items.get());
        let unbuilt = || {
            windows.nodes.set(nodes);
            windows.items.set(items);
            windows.refund(slice.len - built);
        };
        if ui_value_headroom().rows() == 0 {
            windows.refund(slice.len - built);
            ARENA_UNBUILT_ROWS.with(|rows| rows.set(rows.get() + slice.len - built));
            break;
        }
        windows.path.borrow_mut().push(id.to_owned());
        windows.items.set(items - (items / 4).min(TREE_WINDOW_ROW_OWN_ITEMS));
        let materialised = row(index);
        windows.path.borrow_mut().pop();
        let node = match materialised {
            Ok(node) => node,
            Err(error) if error.code == "ui.fixed-capacity" => {
                unbuilt();
                if ui_value_headroom().rows() == 0 {
                    ARENA_UNBUILT_ROWS.with(|rows| rows.set(rows.get() + slice.len - built));
                }
                break;
            }
            Err(error) => return Err(error),
        };
        let Some(cost) = semio_framework_ui_runtime::surface_subtree_items(&node).ok().filter(|cost| *cost <= items) else {
            unbuilt();
            break;
        };
        windows.items.set(items - cost);
        builder = match builder.try_child(node) {
            Ok(builder) => builder,
            Err((builder, _)) => {
                unbuilt();
                return Ok(builder);
            }
        };
    }
    Ok(builder)
}

pub(crate) fn tree_window_rows<B: HasChildren, T>(builder: B, windows: &TreeWindows<'_>, id: &str, entries: &[T], slice: &TreeSlice, mut row: impl FnMut(&T) -> UiAssemblyResult<BuiltNode>) -> UiAssemblyResult<B> {
    tree_window_indexed_rows(builder, windows, id, slice, |index| row(&entries[index]))
}

/// 🪟️ One windowed section node: only the host's slice is built, `window` carries the full extent,
/// never a `+N` continuation row. The section node itself is charged to the body-wide node ledger
/// before its rows are — see [`TreeWindows`].
pub fn tree_window_section<T>(windows: &TreeWindows<'_>, id: &str, label: Label, default_open: bool, entries: &[T], row: impl FnMut(&T) -> UiAssemblyResult<BuiltNode>) -> UiAssemblyResult<BuiltNode> {
    TreeWindows::admit_key(id)?;
    let path = windows.path_of(id);
    windows.claim_window(&path, entries.len())?;
    windows.debit_container();
    let slice = windows.sliced(&path, default_open, entries.len());
    let builder = tree_section(label).default_open(default_open).try_id(id).map_err(|_| ui_assembly_error("tree-window.section-id"))?;
    let builder = tree_window_rows(builder, windows, id, entries, &slice, row)?;
    let builder = match windows.stamp(&path, &slice) {
        Some(window) => builder.window(window),
        None => builder,
    };
    builder.try_build().map_err(|_| ui_assembly_error("tree-window.section-build"))
}

/// 🪟️ One windowed section whose logical rows are addressed by ordinal, keeping large lazy
/// projections reachable without allocating a temporary entry collection.
pub fn tree_window_indexed_section(windows: &TreeWindows<'_>, id: &str, label: Label, default_open: bool, total: usize, row: impl FnMut(usize) -> UiAssemblyResult<BuiltNode>) -> UiAssemblyResult<BuiltNode> {
    TreeWindows::admit_key(id)?;
    let path = windows.path_of(id);
    windows.claim_window(&path, total)?;
    windows.debit_container();
    let slice = windows.sliced(&path, default_open, total);
    let builder = tree_section(label).default_open(default_open).try_id(id).map_err(|_| ui_assembly_error("tree-window.section-id"))?;
    let builder = tree_window_indexed_rows(builder, windows, id, &slice, row)?;
    let builder = match windows.stamp(&path, &slice) {
        Some(window) => builder.window(window),
        None => builder,
    };
    builder.try_build().map_err(|_| ui_assembly_error("tree-window.section-build"))
}

/// 🪟️ [`tree_window_section`], substituting one "(none)"-style placeholder row when `entries` is
/// empty — an empty container has no window to publish.
pub fn tree_window_section_or_placeholder<T, L: TryInto<Label>>(
    windows: &TreeWindows<'_>,
    id: &str,
    label: Label,
    default_open: bool,
    entries: &[T],
    row: impl FnMut(&T) -> UiAssemblyResult<BuiltNode>,
    placeholder_label: L,
) -> UiAssemblyResult<BuiltNode> {
    if !entries.is_empty() {
        return tree_window_section(windows, id, label, default_open, entries, row);
    }
    TreeWindows::admit_key(id)?;
    windows.debit_container();
    windows.debit(1);
    let empty_id = UiText::try_format(format_args!("{id}.empty")).ok_or_else(|| ui_assembly_error("tree-window.placeholder-id"))?;
    let builder = tree_section(label).default_open(default_open).try_id(id).map_err(|_| ui_assembly_error("tree-window.section-id"))?;
    builder.try_child(tree_item(empty_id, placeholder_label)?).map_err(|_| ui_assembly_error("tree-window.placeholder"))?.try_build().map_err(|_| ui_assembly_error("tree-window.section-build"))
}

/// 🪟️ A windowed group row (object › vortices, load case › loads) — `item` already carries its id,
/// label and icon, `id` is the node key the host addresses it by. Built as a row of an enclosing
/// window, the group item's own node is the row its parent already charged; built standalone it
/// charges its own. Either way its rows come off the same body-wide ledger as the sections around
/// it — see [`TreeWindows`].
pub fn tree_window_item<T>(windows: &TreeWindows<'_>, item: TreeItemBuilder, id: &str, default_open: bool, entries: &[T], row: impl FnMut(&T) -> UiAssemblyResult<BuiltNode>) -> UiAssemblyResult<BuiltNode> {
    TreeWindows::admit_key(id)?;
    let path = windows.path_of(id);
    windows.claim_window(&path, entries.len())?;
    windows.debit_container();
    let slice = windows.sliced(&path, default_open, entries.len());
    let builder = tree_window_rows(item.default_open(default_open), windows, id, entries, &slice, row)?;
    let builder = match windows.stamp(&path, &slice) {
        Some(window) => builder.window(window),
        None => builder,
    };
    builder.try_build().map_err(|_| ui_assembly_error("tree-window.item-build"))
}

/// 🪟️ One windowed group whose logical rows are addressed by ordinal without first allocating a
/// collection of row records. Lazy artifact projections use this for byte, pixel, vertex, and
/// document collections whose complete cardinality must remain reachable without materialisation.
pub fn tree_window_indexed_item(windows: &TreeWindows<'_>, item: TreeItemBuilder, id: &str, default_open: bool, total: usize, row: impl FnMut(usize) -> UiAssemblyResult<BuiltNode>) -> UiAssemblyResult<BuiltNode> {
    TreeWindows::admit_key(id)?;
    let path = windows.path_of(id);
    windows.claim_window(&path, total)?;
    windows.debit_container();
    let slice = windows.sliced(&path, default_open, total);
    let builder = tree_window_indexed_rows(item.default_open(default_open), windows, id, &slice, row)?;
    let builder = match windows.stamp(&path, &slice) {
        Some(window) => builder.window(window),
        None => builder,
    };
    builder.try_build().map_err(|_| ui_assembly_error("tree-window.item-build"))
}

/// 🌳️ Admits fallibly assembled UI nodes into fixed child storage — the fleet's one copy, replacing
/// the per-app duplicates. ⚠️ These rows are OUTSIDE the [`TreeWindows`] node ledger: they carry no
/// window and no node key, so only [`TREE_WINDOW_FIXED_NODE_HEADROOM`] pays for them. Use it for the
/// small fixed lists a panel genuinely owns, never for an entity list.
pub fn ui_node_list(values: impl IntoIterator<Item = UiAssemblyResult<BuiltNode>>) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let mut nodes = UiFixedList::default();
    for value in values {
        nodes.try_push(value?).map_err(|_| ui_assembly_error("ui-node-list.admission"))?;
    }
    Ok(nodes)
}

#[cfg(test)]
include!("../../🧪️tests/🔬️app-panel-kit/🦀️.rs");

/// 🌳️ One recursive labeled node; `TreeView.roots` seeds a single unlabeled tree section.
#[derive(Clone, Debug, PartialEq)]
pub struct TreeNodeView {
    pub id: String,
    pub label: String,
    pub children: Vec<TreeNodeView>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TreeView {
    pub roots: Vec<TreeNodeView>,
}

/// ✏️ One materialized tree row's inline text draft and complete static artifact address.
#[derive(Debug, PartialEq)]
pub struct EditableTreeNode {
    pub value: String,
    pub action_id: String,
    pub arguments: UiValue,
}

impl EditableTreeNode {
    pub fn new(value: impl Into<String>, action_id: impl Into<String>, arguments: UiValue) -> Self {
        Self { value: value.into(), action_id: action_id.into(), arguments }
    }
}

pub struct TreeWindowKit;

impl WindowKit for TreeWindowKit {
    type ViewModel = TreeView;
    const KIND_ID: &'static str = "framework.window.tree";

    fn window_kind() -> WindowKindDefinition {
        window_kind_definition(Self::KIND_ID, "Tree", "Baum", SurfaceKind::BlockList, "list-tree", Vec::new())
    }

    fn editable_window_kind() -> WindowKindDefinition {
        window_kind_definition(
            Self::KIND_ID,
            "Tree",
            "Baum",
            SurfaceKind::BlockList,
            "list-tree",
            vec![ActionDefinition::bounded_catalog("set-node", LocalizedLabel::native("Set Node", "Knoten setzen"), ActionKind::Mutation)
                .with_args(vec![
                    ActionArgDef::text("nodeId", LocalizedLabel::native("Node", "Knoten")).required(),
                    ActionArgDef::document_revision("revision", LocalizedLabel::native("Revision", "Revision")),
                    ActionArgDef::text("value", LocalizedLabel::native("Value", "Wert")).min_length(0).required(),
                ])
                .describe(LocalizedLabel::native(
                    "Writes the given value into one node of the document tree, addressed by node id, replacing the node's previous value.",
                    "Schreibt den angegebenen Wert in einen Knoten des Dokumentbaums, adressiert über die Knoten-Id, und ersetzt dessen bisherigen Wert.",
                ))],
        )
    }

    fn render(view: &TreeView) -> UiAssemblyResult<BuiltNode> {
        Self::render_windowed(view, &TreeWindows::unhosted())
    }
}

impl TreeWindowKit {
    /// 🪟️ One node of a `TreeView`, windowed at EVERY level: a node's children are materialised only
    /// while the host holds it open, and only for the rows its window names. An archive, JSON array or
    /// XML element with hundreds of siblings anywhere in its structure renders its viewport instead of
    /// refusing the whole editor with `ui.fixed-capacity` — the fan-out a document format reaches is
    /// ordinary, not an edge case.
    fn windowed_node(windows: &TreeWindows<'_>, node: &TreeNodeView, depth: usize) -> UiAssemblyResult<BuiltNode> {
        let depth = depth.checked_sub(1).ok_or_else(|| ui_assembly_error("tree-window.depth"))?;
        let builder = ui::tree_item(ui_label(node.label.clone(), "tree-window.item-label")?);
        let builder = builder.try_id(&node.id).map_err(|_| ui_assembly_error("tree-window.item-id"))?;
        tree_window_item(windows, builder, &node.id, !node.children.is_empty(), &node.children, |child| Self::windowed_node(windows, child, depth))
    }

    /// 🪟️ The `TreeView` a host reads, against that body's known open/scroll state. `WindowKit::render`
    /// is this with [`TreeWindows::unhosted`] — author defaults and one viewport of rows.
    pub fn render_windowed(view: &TreeView, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
        let root_id = UiText::try_format(format_args!("{}-root", Self::KIND_ID)).ok_or_else(|| ui_assembly_error("tree-window.section-id"))?;
        let section = tree_window_section(windows, root_id.as_str(), Label::default(), true, &view.roots, |node| Self::windowed_node(windows, node, semio_framework_ui_runtime::COMPONENT_TREE_PRODUCER_DEPTH))?;
        let tree_builder = tree().try_id(Self::KIND_ID).map_err(|_| ui_assembly_error("tree-window.id"))?;
        tree_builder.try_child(section).map_err(|_| ui_assembly_error("tree-window.section"))?.try_build().map_err(|_| ui_assembly_error("tree-window.build"))
    }

    fn editable_windowed_node<F>(windows: &TreeWindows<'_>, node: &TreeNodeView, path: &str, depth: usize, controller_id: &str, editable: &mut F) -> UiAssemblyResult<BuiltNode>
    where
        F: FnMut(&str, &TreeNodeView) -> Option<EditableTreeNode>,
    {
        let depth = depth.checked_sub(1).ok_or_else(|| ui_assembly_error("tree-window.depth"))?;
        let mut builder = ui::tree_item(ui_label(node.label.clone(), "tree-window.item-label")?).try_id(&node.id).map_err(|_| ui_assembly_error("tree-window.item-id"))?;
        if let Some(edit) = editable(path, node) {
            let action = ActionId::try_v1(controller_id, &edit.action_id).ok_or_else(|| ui_assembly_error("tree-window.edit-action-id"))?;
            let input = input(InputKind::Text)
                .value(UiText::try_from_str(&edit.value).ok_or_else(|| ui_assembly_error("tree-window.edit-value"))?)
                .commit(UiText::try_from_str("blur").ok_or_else(|| ui_assembly_error("tree-window.edit-commit"))?)
                .try_label(&node.label)
                .map_err(|_| ui_assembly_error("tree-window.edit-label"))?
                .try_id("edit")
                .map_err(|_| ui_assembly_error("tree-window.edit-id"))?
                .try_on_with(Trigger::Commit, action, edit.arguments)
                .map_err(|_| ui_assembly_error("tree-window.edit-binding"))?
                .try_build()
                .map_err(|_| ui_assembly_error("tree-window.edit-input"))?;
            builder = builder.try_child(input).map_err(|_| ui_assembly_error("tree-window.edit-child"))?;
        }
        tree_window_item(windows, builder, &node.id, !node.children.is_empty(), &node.children, |child| {
            let child_path = format!("{path}{}{id}", TREE_WINDOW_PATH_SEPARATOR, id = child.id);
            Self::editable_windowed_node(windows, child, &child_path, depth, controller_id, editable)
        })
    }

    /// ✏️ Renders only materialized rows with inline text drafts carrying caller-authored stable
    /// addresses and revisions; the committed text is merged as the canonical `value` argument.
    pub fn render_editable_nodes_windowed<F>(view: &TreeView, windows: &TreeWindows<'_>, controller_id: &str, mut editable: F) -> UiAssemblyResult<BuiltNode>
    where
        F: FnMut(&str, &TreeNodeView) -> Option<EditableTreeNode>,
    {
        let root_id = UiText::try_format(format_args!("{}-root", Self::KIND_ID)).ok_or_else(|| ui_assembly_error("tree-window.section-id"))?;
        let section = tree_window_section(windows, root_id.as_str(), Label::default(), true, &view.roots, |node| {
            Self::editable_windowed_node(windows, node, &node.id, semio_framework_ui_runtime::COMPONENT_TREE_PRODUCER_DEPTH, controller_id, &mut editable)
        })?;
        let tree_builder = tree().try_id(Self::KIND_ID).map_err(|_| ui_assembly_error("tree-window.id"))?;
        tree_builder.try_child(section).map_err(|_| ui_assembly_error("tree-window.section"))?.try_build().map_err(|_| ui_assembly_error("tree-window.build"))
    }
}
