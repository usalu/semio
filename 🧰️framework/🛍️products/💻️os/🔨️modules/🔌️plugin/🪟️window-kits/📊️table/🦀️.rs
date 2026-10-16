//! 📊️ The table window kit and its editable row windows.
use semio_framework::{ActionArgDef, ActionDefinition, ActionKind, WindowKindDefinition};
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
use serde_json::Value;
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin_window_kits::text_kit::TextDraftView;
use semio_framework_plugin_window_kits::text_kit::TextView;
use semio_framework_plugin_window_kits::text_kit::TextWindowKit;
use crate::tree_kit::TreeWindows;
use crate::tree_kit::tree_window_indexed_rows;
use crate::tree_kit::tree_window_rows;
use semio_framework_plugin_window_kits::WindowKit;
use semio_framework_plugin_interaction::UiAssemblyResult;
use semio_framework_plugin_window_transient::scene_surface;
use semio_framework_plugin_interaction::ui_assembly_error;
use semio_framework_plugin_window_transient::window_kind_definition;



/// 📊️ Flat column/row grid — cells are plain strings; typed cells (`TableCell`) are a renderer
/// concern, not this SDK-level view-model's.
#[derive(Clone, Debug, PartialEq)]
pub struct TableView {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

/// ✏️ One table column whose text cells publish a local draft through an artifact action.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EditableTableColumn {
    pub index: usize,
    pub action_id: String,
    pub row_argument: String,
    pub column_argument: Option<String>,
}

impl EditableTableColumn {
    pub fn new(index: usize, action_id: impl Into<String>) -> Self {
        Self { index, action_id: action_id.into(), row_argument: "row".into(), column_argument: Some("column".into()) }
    }

    pub fn without_column_argument(mut self) -> Self {
        self.column_argument = None;
        self
    }
}

/// 🎯️ One editable table cell with an artifact-stable address and optimistic revision payload.
#[derive(Debug, PartialEq)]
pub struct EditableTableCell {
    pub row_index: usize,
    pub column_index: usize,
    pub action_id: String,
    pub arguments: UiValue,
}

impl EditableTableCell {
    pub fn new(row_index: usize, column_index: usize, action_id: impl Into<String>, arguments: UiValue) -> Self {
        Self { row_index, column_index, action_id: action_id.into(), arguments }
    }
}

/// ✏️ One declarative table cell whose local draft commits through an artifact-owned action.
#[derive(Debug, PartialEq)]
pub struct WindowedEditableTableCell {
    pub value: String,
    pub label: String,
    pub action: Option<(String, UiValue)>,
    pub publication_revision: Option<UiPublicationRevision>,
}

impl WindowedEditableTableCell {
    pub fn new(value: impl Into<String>, label: impl Into<String>, action_id: impl Into<String>, arguments: UiValue) -> Self {
        Self { value: value.into(), label: label.into(), action: Some((action_id.into(), arguments)), publication_revision: None }
    }

    pub fn read_only(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self { value: value.into(), label: label.into(), action: None, publication_revision: None }
    }

    /// 🧾️ Pins this cell's retained input to the native document revision that authored it.
    pub fn publication_revision(mut self, publication_revision: UiPublicationRevision) -> Self {
        self.publication_revision = Some(publication_revision);
        self
    }
}

pub struct TableWindowKit;

/// ↔️ Columns shown before the host has measured the horizontal viewport.
pub const TABLE_COLUMN_WINDOW_DEFAULT: usize = 8;

/// ↔️ Maximum columns in one materialised slice; every logical column remains reachable by offset.
pub const TABLE_COLUMN_WINDOW_MAX: usize = 16;

/// 🔑️ Stable sibling window key for one table's independent column projection.
pub fn table_column_window_key(table_id: &str) -> String {
    format!("{table_id}.columns")
}

impl WindowKit for TableWindowKit {
    type ViewModel = TableView;
    const KIND_ID: &'static str = "framework.window.table";

    fn window_kind() -> WindowKindDefinition {
        window_kind_definition(Self::KIND_ID, "Table", "Tabelle", SurfaceKind::Table, "table-2", Vec::new())
    }

    fn editable_window_kind() -> WindowKindDefinition {
        window_kind_definition(
            Self::KIND_ID,
            "Table",
            "Tabelle",
            SurfaceKind::Table,
            "table-2",
            vec![ActionDefinition::bounded_catalog("set-cell", LocalizedLabel::native("Set Cell", "Zelle setzen"), ActionKind::Mutation)
                .with_args(vec![
                    ActionArgDef::index("row", LocalizedLabel::native("Row", "Zeile")).required(),
                    ActionArgDef::index("column", LocalizedLabel::native("Column", "Spalte")),
                    ActionArgDef::text("value", LocalizedLabel::native("Value", "Wert")).min_length(0).required(),
                ])
                .describe(LocalizedLabel::native(
                    "Writes the given value into one table cell, addressed by row index and column, replacing what the cell held before.",
                    "Schreibt den angegebenen Wert in eine Tabellenzelle, adressiert über Zeilenindex und Spalte, und ersetzt ihren bisherigen Inhalt.",
                ))],
        )
    }

    /// 📊️ Emits the renderer table contract both hosts read: `columnsJson` as `{id, label}` records
    /// and `rowsJson` as `{id, <column id>: cell}` records, keyed by column and row position.
    ///
    /// 🧬️ M2 fallout (terra-sdk-wire): `TableScene` moved to `semio-framework-ui-scene` and its
    /// `base` constructor is now the E6 sync-by-decree shape (no suspension point) — the
    /// OUTER `build_table_scene` (unmoved, `ui_wgpu::wgpu`) stays a real `async fn`.
    fn render(view: &TableView) -> UiAssemblyResult<BuiltNode> {
        let columns: Vec<Value> = view.columns.iter().enumerate().map(|(index, label)| serde_json::json!({ "id": index.to_string(), "label": label })).collect();
        let rows: Vec<Value> = view
            .rows
            .iter()
            .enumerate()
            .map(|(index, cells)| {
                let mut record = serde_json::Map::new();
                record.insert("id".into(), Value::String(index.to_string()));
                for (column, cell) in cells.iter().enumerate() {
                    record.insert(column.to_string(), Value::String(cell.clone()));
                }
                Value::Object(record)
            })
            .collect();
        let columns_json = serde_json::to_string(&columns).unwrap_or_else(|_| "[]".into());
        let rows_json = serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into());
        let scene = semio_framework_ui_scene::TableScene::base(columns_json, rows_json);
        scene_surface(Self::KIND_ID, SurfaceKind::Table, &scene)
    }
}

impl TableWindowKit {
    /// ✏️ Emits typed text-input cells that commit artifact-owned actions with stable row and column ordinals.
    pub fn render_editable(view: &TableView, controller_id: &str, editable_columns: &[EditableTableColumn]) -> UiAssemblyResult<BuiltNode> {
        let columns: Vec<Value> = view.columns.iter().enumerate().map(|(index, label)| serde_json::json!({ "id": index.to_string(), "label": label })).collect();
        let rows: Vec<Value> = view
            .rows
            .iter()
            .enumerate()
            .map(|(row_index, cells)| {
                let mut record = serde_json::Map::new();
                record.insert("id".into(), Value::String(row_index.to_string()));
                for (column_index, cell) in cells.iter().enumerate() {
                    let value = editable_columns.iter().find(|editable| editable.index == column_index).map_or_else(
                        || Value::String(cell.clone()),
                        |editable| {
                            let mut args = serde_json::Map::new();
                            args.insert(editable.row_argument.clone(), serde_json::json!(row_index));
                            if let Some(argument) = &editable.column_argument {
                                args.insert(argument.clone(), serde_json::json!(column_index));
                            }
                            serde_json::json!({
                                "kind": "editableText",
                                "value": cell,
                                "action": { "controllerId": controller_id, "action": editable.action_id.as_str(), "args": args }
                            })
                        },
                    );
                    record.insert(column_index.to_string(), value);
                }
                Value::Object(record)
            })
            .collect();
        let columns_json = serde_json::to_string(&columns).unwrap_or_else(|_| "[]".into());
        let rows_json = serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into());
        let scene = semio_framework_ui_scene::TableScene::base(columns_json, rows_json);
        scene_surface(Self::KIND_ID, SurfaceKind::Table, &scene)
    }

    /// 🎯️ Emits editable cells whose static args carry stable row-specific addresses and revisions.
    pub fn render_editable_cells(view: &TableView, controller_id: &str, editable_cells: &[EditableTableCell]) -> UiAssemblyResult<BuiltNode> {
        let columns: Vec<Value> = view.columns.iter().enumerate().map(|(index, label)| serde_json::json!({ "id": index.to_string(), "label": label })).collect();
        let rows = view
            .rows
            .iter()
            .enumerate()
            .map(|(row_index, cells)| {
                let mut record = serde_json::Map::new();
                record.insert("id".into(), Value::String(row_index.to_string()));
                for (column_index, cell) in cells.iter().enumerate() {
                    let value = match editable_cells.iter().find(|editable| editable.row_index == row_index && editable.column_index == column_index) {
                        Some(editable) => serde_json::json!({
                            "kind": "editableText",
                            "value": cell,
                            "action": {
                                "controllerId": controller_id,
                                "action": editable.action_id.as_str(),
                                "args": serde_json::to_value(&editable.arguments).map_err(|_| ui_assembly_error("table-window.editable-cell-arguments"))?
                            }
                        }),
                        None => Value::String(cell.clone()),
                    };
                    record.insert(column_index.to_string(), value);
                }
                Ok(Value::Object(record))
            })
            .collect::<UiAssemblyResult<Vec<_>>>()?;
        let columns_json = serde_json::to_string(&columns).map_err(|_| ui_assembly_error("table-window.editable-cell-columns"))?;
        let rows_json = serde_json::to_string(&rows).map_err(|_| ui_assembly_error("table-window.editable-cell-rows"))?;
        let scene = semio_framework_ui_scene::TableScene::base(columns_json, rows_json);
        scene_surface(Self::KIND_ID, SurfaceKind::Table, &scene)
    }
}

/// 🎬️ One row action — its icon, accessible label and `verb` on the row's ONE [`RowTarget`] — painted on a
/// tree row or in a table's trailing actions column (`placement`), or folded into the row's menu. Enabled; `.disabled(true)`
/// paints and announces it disabled and every host refuses its dispatch ([`RowTarget::action_binding`]).
pub fn row_action(icon: &str, label: &str, verb: &str, placement: RowActionPlacement) -> UiAssemblyResult<RowAction> {
    Ok(RowAction {
        icon: UiText::try_from_str(icon).ok_or_else(|| ui_assembly_error("row-action.icon"))?,
        label: Some(Label::try_from(label).map_err(|_| ui_assembly_error("row-action.label"))?),
        verb: UiText::try_from_str(verb).ok_or_else(|| ui_assembly_error("row-action.verb"))?,
        placement,
        disabled: false,
        reason: None,
    })
}

/// 🎯️ The ONE target a row's verbs fire on: `controller_id` at version 1, `args` — an argument MAP — inherited by every
/// row action, and `activation`, the verb the row's click (Enter on a focused row) fires.
pub fn row_target(controller_id: &str, args: Option<UiValue>, activation: Option<&str>) -> UiAssemblyResult<RowTarget> {
    Ok(RowTarget {
        scope: UiText::try_from_str(controller_id).ok_or_else(|| ui_assembly_error("row-target.scope"))?,
        version: 1,
        args: match args {
            Some(UiValue::Map(args)) => Some(args),
            Some(_) => return Err(ui_assembly_error("row-target.args")),
            None => None,
        },
        activation: activation.map(|verb| UiText::try_from_str(verb).ok_or_else(|| ui_assembly_error("row-target.activation"))).transpose()?,
    })
}

/// 🎯️ The target of a row whose ONLY verb is its activation `(action, args)` — the action's scope, version and argument
/// map, the action's verb as the activation.
pub fn activation_target(action: (ActionId, Option<UiValue>)) -> UiAssemblyResult<RowTarget> {
    let (action, args) = action;
    let mut target = row_target(action.scope.as_str(), args, None)?;
    target.version = action.version;
    target.activation = Some(action.name);
    Ok(target)
}

/// 📊️ One windowed table row: record key `key` (the row's stable identity, e.g. `"space:<id>"`),
/// `cells` positional to the table's columns, `actions` in its actions column and `target` the ONE target its
/// actions and activation (Enter on the focused row) fire on. One node record however many cells and actions it
/// carries: row actions travel only as [`RowAction`] verbs, the one representation every renderer paints in the
/// trailing actions column.
pub fn table_window_row(key: &str, cells: &[&str], actions: impl IntoIterator<Item = RowAction>, target: Option<RowTarget>) -> UiAssemblyResult<BuiltNode> {
    let mut row_cells = UiFixedList::default();
    for cell in cells {
        row_cells.try_push(UiText::try_from_str(cell).ok_or_else(|| ui_assembly_error("table-window.cell"))?).map_err(|_| ui_assembly_error("table-window.cells"))?;
    }
    let mut builder = table_row(row_cells).try_id(key).map_err(|_| ui_assembly_error("table-window.row-id"))?;
    for action in actions {
        builder = builder.try_row_action(action).map_err(|_| ui_assembly_error("table-window.row-actions"))?;
    }
    if let Some(target) = target {
        builder = builder.target(target);
    }
    builder.try_build().map_err(|_| ui_assembly_error("table-window.row-build"))
}

/// ✏️ One windowed table row whose cells are native declarative inputs. Short values use inline
/// commit-on-blur controls; long values use the paged text-draft scene with explicit Apply/Discard. `actions` are
/// verbs on the row's ONE `target`.
pub fn editable_table_window_row(key: &str, controller_id: &str, locale: Locale, cells: impl IntoIterator<Item = WindowedEditableTableCell>, actions: impl IntoIterator<Item = RowAction>, target: Option<RowTarget>) -> UiAssemblyResult<BuiltNode> {
    editable_table_window_row_at(key, controller_id, locale, 0, cells, actions, target)
}

/// 🎯️ Stable semantic identity for one table-cell draft, independent of action arguments such as revisions.
fn editable_table_cell_draft_target(controller_id: &str, row_key: &str, column: usize, action_id: &str) -> UiAssemblyResult<UiText> {
    let target = format!("table-cell.v1/{}/{}/{}/{}/{}/{}/{}", controller_id.len(), controller_id, row_key.len(), row_key, action_id.len(), action_id, column);
    UiText::try_from_str(&target).ok_or_else(|| ui_assembly_error("table-window.editable-row-draft-target"))
}

/// ↔️ Column-windowed counterpart whose child keys and draft surfaces retain logical ordinals.
pub fn editable_table_window_row_at(
    key: &str,
    controller_id: &str,
    locale: Locale,
    column_offset: usize,
    cells: impl IntoIterator<Item = WindowedEditableTableCell>,
    actions: impl IntoIterator<Item = RowAction>,
    target: Option<RowTarget>,
) -> UiAssemblyResult<BuiltNode> {
    let mut row_cells = UiFixedList::default();
    let mut children = Vec::new();
    for (index, cell) in cells.into_iter().enumerate() {
        let logical_column = column_offset + index;
        let preview = UiText::try_from_str(&cell.value).unwrap_or_else(|| UiText::try_from_str("…").expect("static table overflow marker fits"));
        row_cells.try_push(preview).map_err(|_| ui_assembly_error("table-window.editable-row-cells"))?;
        let child_id = format!("cell-{logical_column}");
        let child = if let Some((action_id, arguments)) = cell.action {
            if let Some(value) = UiText::try_from_str(&cell.value) {
                let draft_target = editable_table_cell_draft_target(controller_id, key, logical_column, &action_id)?;
                let action = ActionId::try_v1(controller_id, &action_id).ok_or_else(|| ui_assembly_error("table-window.editable-row-action"))?;
                let mut builder = input(InputKind::Text)
                    .value(value)
                    .commit(UiText::try_from_str("blur").ok_or_else(|| ui_assembly_error("table-window.editable-row-commit"))?)
                    .draft_target(draft_target)
                    .try_label(&cell.label)
                    .map_err(|_| ui_assembly_error("table-window.editable-row-label"))?
                    .try_id(&child_id)
                    .map_err(|_| ui_assembly_error("table-window.editable-row-id"))?;
                if let Some(publication_revision) = cell.publication_revision {
                    builder = builder.publication_revision(publication_revision);
                }
                builder.try_on_with(Trigger::Commit, action, arguments).map_err(|_| ui_assembly_error("table-window.editable-row-binding"))?.try_build().map_err(|_| ui_assembly_error("table-window.editable-row-input"))?
            } else {
                let (apply_label, discard_label, conflict_label, applying_label, cancel_label, failed_label) = match locale {
                    Locale::De => ("Anwenden", "Verwerfen", "Der gespeicherte Zellwert hat sich geändert. Bitte neu abgleichen.", "Wird angewendet …", "Abbrechen", "Änderung fehlgeschlagen. Der Entwurf bleibt erhalten."),
                    Locale::En => ("Apply", "Discard", "The saved cell value changed. Reconcile before applying.", "Applying…", "Cancel", "The edit failed. Your draft is preserved."),
                };
                let (syntax_error_label, validation_error_label, path_label, line_label, column_label) = match locale {
                    Locale::De => ("Der Quelltext enthält ungültige Syntax.", "Der Entwurf entspricht nicht dem Dokumentformat.", "Pfad", "Zeile", "Spalte"),
                    Locale::En => ("The source contains invalid syntax.", "The draft does not match the document format.", "Path", "Line", "Column"),
                };
                TextWindowKit::render_draft(&TextDraftView {
                    surface_id: format!("table-draft-{key}-{logical_column}"),
                    text: cell.value,
                    language: None,
                    action_id,
                    argument: "value".into(),
                    arguments: Some(arguments),
                    publication_revision: cell.publication_revision.ok_or_else(|| ui_assembly_error("table-window.editable-row-publication-revision"))?,
                    apply_label: apply_label.into(),
                    discard_label: discard_label.into(),
                    conflict_label: conflict_label.into(),
                    applying_label: applying_label.into(),
                    cancel_label: cancel_label.into(),
                    failed_label: failed_label.into(),
                    syntax_error_label: syntax_error_label.into(),
                    validation_error_label: validation_error_label.into(),
                    path_label: path_label.into(),
                    line_label: line_label.into(),
                    column_label: column_label.into(),
                })?
            }
        } else if let Some(value) = UiText::try_from_str(&cell.value) {
            text(Label::try_from(value.as_str()).map_err(|_| ui_assembly_error("table-window.read-only-cell-label"))?)
                .try_id(&child_id)
                .map_err(|_| ui_assembly_error("table-window.read-only-cell-id"))?
                .try_build()
                .map_err(|_| ui_assembly_error("table-window.read-only-cell"))?
        } else {
            TextWindowKit::render_read_only(&format!("table-value-{key}-{logical_column}"), &TextView { text: cell.value, language: None })?
        };
        children.push(child);
    }
    let mut builder = table_row(row_cells).try_id(key).map_err(|_| ui_assembly_error("table-window.editable-row-key"))?;
    builder = builder.try_children(children).map_err(|_| ui_assembly_error("table-window.editable-row-children"))?;
    for action in actions {
        builder = builder.try_row_action(action).map_err(|_| ui_assembly_error("table-window.editable-row-actions"))?;
    }
    if let Some(target) = target {
        builder = builder.target(target);
    }
    builder.try_build().map_err(|_| ui_assembly_error("table-window.editable-row-build"))
}

impl TableWindowKit {
    /// 📊️ One windowed table as the window body's growing child: the header (`columns`) and accessible
    /// name (`label`) live on the `Table` node, and only the host's slice of `entries` is built — each a
    /// single [`table_window_row`] record — so the table costs `1 + materialised rows` on the SAME
    /// body-wide [`TreeWindows`] ledger the tree panels spend, and its `window` stamp is the contract the
    /// host virtualises trees with. Any row count stays inside `UI_DOCUMENT_NODES`: the host streams the
    /// rows its viewport shows, the first paint serves what the unreserved ledger and the viewport budget
    /// allow.
    pub fn render_rows<T>(windows: &TreeWindows<'_>, label: &str, columns: &[&str], actions_label: Option<&str>, entries: &[T], row: impl FnMut(&T) -> UiAssemblyResult<BuiltNode>) -> UiAssemblyResult<BuiltNode> {
        TreeWindows::admit_key(Self::KIND_ID)?;
        let path = windows.path_of(Self::KIND_ID);
        windows.claim_window(&path, entries.len())?;
        windows.debit_container();
        let slice = windows.sliced(&path, true, entries.len());
        let mut headers = UiFixedList::default();
        for column in columns {
            headers.try_push(Label::try_from(*column).map_err(|_| ui_assembly_error("table-window.column"))?).map_err(|_| ui_assembly_error("table-window.columns"))?;
        }
        let builder = table(Label::try_from(label).map_err(|_| ui_assembly_error("table-window.label"))?, headers).grow(true).try_id(Self::KIND_ID).map_err(|_| ui_assembly_error("table-window.id"))?;
        let builder = match actions_label {
            Some(actions_label) => builder.actions_label(Label::try_from(actions_label).map_err(|_| ui_assembly_error("table-window.actions-label"))?),
            None => builder,
        };
        let builder = tree_window_rows(builder, windows, Self::KIND_ID, entries, &slice, row)?;
        let builder = match windows.stamp(&path, &slice) {
            Some(window) => builder.window(window),
            None => builder,
        };
        builder.try_build().map_err(|_| ui_assembly_error("table-window.build"))
    }

    /// 📊️ Windowed table projection addressed by logical row ordinal, without allocating a
    /// temporary row collection proportional to the document.
    pub fn render_indexed_rows(windows: &TreeWindows<'_>, label: &str, columns: &[&str], actions_label: Option<&str>, total: usize, row: impl FnMut(usize) -> UiAssemblyResult<BuiltNode>) -> UiAssemblyResult<BuiltNode> {
        Self::render_indexed_rows_with_id(windows, Self::KIND_ID, label, columns, actions_label, total, row)
    }

    /// 📊️ Named counterpart used when one body owns more than one independently windowed table.
    pub fn render_indexed_rows_with_id(windows: &TreeWindows<'_>, id: &str, label: &str, columns: &[&str], actions_label: Option<&str>, total: usize, row: impl FnMut(usize) -> UiAssemblyResult<BuiltNode>) -> UiAssemblyResult<BuiltNode> {
        TreeWindows::admit_key(id)?;
        let path = windows.path_of(id);
        windows.claim_window(&path, total)?;
        windows.debit_container();
        let slice = windows.sliced(&path, true, total);
        let mut headers = UiFixedList::default();
        for column in columns {
            headers.try_push(Label::try_from(*column).map_err(|_| ui_assembly_error("table-window.column"))?).map_err(|_| ui_assembly_error("table-window.columns"))?;
        }
        let builder = table(Label::try_from(label).map_err(|_| ui_assembly_error("table-window.label"))?, headers).grow(true).try_id(id).map_err(|_| ui_assembly_error("table-window.id"))?;
        let builder = match actions_label {
            Some(actions_label) => builder.actions_label(Label::try_from(actions_label).map_err(|_| ui_assembly_error("table-window.actions-label"))?),
            None => builder,
        };
        let builder = tree_window_indexed_rows(builder, windows, id, &slice, row)?;
        let builder = match windows.stamp(&path, &slice) {
            Some(window) => builder.window(window),
            None => builder,
        };
        builder.try_build().map_err(|_| ui_assembly_error("table-window.build"))
    }

    /// ↔️ Projects one logical matrix through independent row and column windows. Header and cell
    /// closures receive logical indices, so a nonzero column offset never rewrites action addresses.
    pub fn render_indexed_matrix_with_id(
        windows: &TreeWindows<'_>,
        id: &str,
        label: &str,
        row_label: &str,
        column_label: &str,
        column_total: usize,
        mut column: impl FnMut(usize) -> UiAssemblyResult<Label>,
        actions_label: Option<&str>,
        row_total: usize,
        mut row: impl FnMut(usize, std::ops::Range<usize>) -> UiAssemblyResult<BuiltNode>,
    ) -> UiAssemblyResult<BuiltNode> {
        TreeWindows::admit_key(id)?;
        let column_id = table_column_window_key(id);
        TreeWindows::admit_key(&column_id)?;
        let row_path = windows.path_of(id);
        let column_path = windows.path_of(&column_id);
        windows.claim_window(&row_path, row_total)?;
        windows.claim_window(&column_path, column_total)?;
        windows.debit_container();
        let column_slice = windows.projection_slice(&column_path, true, column_total, TABLE_COLUMN_WINDOW_DEFAULT, TABLE_COLUMN_WINDOW_MAX);
        let per_row_records = column_slice.len.saturating_add(2).max(1);
        let row_cap = (windows.nodes_remaining() / per_row_records).max(1);
        let row_slice = windows.sliced_capped(&row_path, true, row_total, row_cap);
        let mut headers = UiFixedList::default();
        for index in column_slice.offset..column_slice.offset + column_slice.len {
            headers.try_push(column(index)?).map_err(|_| ui_assembly_error("table-window.columns"))?;
        }
        let builder = table(Label::try_from(label).map_err(|_| ui_assembly_error("table-window.label"))?, headers)
            .axis_labels(Label::try_from(row_label).map_err(|_| ui_assembly_error("table-window.row-label"))?, Label::try_from(column_label).map_err(|_| ui_assembly_error("table-window.column-label"))?)
            .grow(true)
            .try_id(id)
            .map_err(|_| ui_assembly_error("table-window.id"))?;
        let builder = match actions_label {
            Some(actions_label) => builder.actions_label(Label::try_from(actions_label).map_err(|_| ui_assembly_error("table-window.actions-label"))?),
            None => builder,
        };
        let columns = column_slice.offset..column_slice.offset + column_slice.len;
        let builder = tree_window_indexed_rows(builder, windows, id, &row_slice, |index| row(index, columns.clone()))?;
        let builder = match windows.stamp(&row_path, &row_slice) {
            Some(window) => builder.window(window),
            None => builder,
        };
        let builder = match windows.stamp(&column_path, &column_slice) {
            Some(window) => builder.column_window(window),
            None => builder,
        };
        builder.try_build().map_err(|_| ui_assembly_error("table-window.build"))
    }
}
