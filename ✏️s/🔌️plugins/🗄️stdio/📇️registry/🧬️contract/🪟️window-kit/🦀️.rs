//! 🪟️ Window-kit conventions of the stdio artifacts: example switching, argument readers, addressed and structural tables, revisions and media import.

use semio_framework_os_kernel as kernel;
use semio_framework_plugin::PluginAssemblyError;
use semio_framework_value as value;
use semio_s_artifact_stdio_contract_wire::{draft_splices_from_json, snapshot_edit_source, DraftSplice};

//#region 🎬️ExampleSwitch
/// 🎬️ The verb the react/wgpu shells' navbar example picker dispatches
/// (`SET_ACTIVE_EXAMPLE_ACTION_ID` in the renderer's own `🛠️ShellHelpers`). An app that does not
/// declare it is hidden from the picker outright by `appSwitchesExamples`, and the shell then never
/// announces a boot example — the pane opens the app's genesis document instead of its curated one.
pub const SET_ACTIVE_EXAMPLE_ACTION_ID: &str = semio_framework_plugin::app::CATALOGUE_EXAMPLE_ACTION_ID;

/// 📚️ Reads the example id out of a shell `{action, args}` pair. The shells spell it `exampleId`;
/// the palette, a context menu and a replayed shell command may spell it `id` or `value`, and an
/// EMPTY id means "the app's own default document", which is exactly what `fallback` of `""` says.
pub fn example_id_argument(args: Option<&semio_framework_value::DslValue>, fallback: &str) -> String {
    let entries: &[(String, semio_framework_value::DslValue)] = match args {
        Some(semio_framework_value::DslValue::Object(object)) => object.as_slice(),
        _ => &[],
    };
    for key in ["exampleId", "example_id", "id", "value"] {
        if let Some(semio_framework_value::DslValue::String(raw)) = entries.iter().find(|(name, _)| name == key).map(|(_, value)| value) {
            if !raw.is_empty() {
                return raw.clone();
            }
        }
    }
    fallback.to_string()
}

/// 🧱️ The staged value of ONE declared argument of a framework window-kit verb, as text.
///
/// The three editable kits (`TextWindowKit`/`TableWindowKit`/`TreeWindowKit`) mint `replace-text`,
/// `set-cell` and `set-node` as palette rows on every editor that composes them, and every stdio
/// editor bridges those rows through its own `command_from_action`. A rail stages a number control
/// as a `Number` and a text control as a `String`, so one reader admits both spellings; the first
/// key that carries a non-empty value wins, which is what lets a wire written before the argument
/// names were declared (`node_id`, `id`) still decode.
pub fn window_kit_text_argument(args: Option<&semio_framework_value::DslValue>, keys: &[&str], fallback: &str) -> String {
    let entries: &[(String, semio_framework_value::DslValue)] = match args {
        Some(semio_framework_value::DslValue::Object(object)) => object.as_slice(),
        _ => &[],
    };
    for key in keys {
        match entries.iter().find(|(name, _)| name == key).map(|(_, value)| value) {
            Some(semio_framework_value::DslValue::String(raw)) if !raw.is_empty() => return raw.clone(),
            Some(semio_framework_value::DslValue::Number(number)) => return number.as_u64().map(|reading| reading.to_string()).or_else(|| number.as_i64().map(|reading| reading.to_string())).unwrap_or_else(|| number.as_f64().to_string()),
            Some(semio_framework_value::DslValue::Bool(flag)) => return flag.to_string(),
            _ => {}
        }
    }
    fallback.to_string()
}

/// 🔢️ The same reader as [`window_kit_text_argument`], parsed as a grid index. A control that
/// carries no reading at all answers `fallback` rather than refusing: `set-cell`'s own handler is
/// the one place that knows whether an index addresses a live row.
pub fn window_kit_index_argument(args: Option<&semio_framework_value::DslValue>, keys: &[&str], fallback: u32) -> u32 {
    let raw = window_kit_text_argument(args, keys, "");
    raw.trim().parse::<f64>().ok().filter(|reading| reading.is_finite() && *reading >= 0.0).map_or(fallback, |reading| reading as u32)
}

fn required_window_kit_argument<'a>(args: Option<&'a semio_framework_value::DslValue>, key: &str) -> Result<&'a semio_framework_value::DslValue, semio_framework_plugin::Fault> {
    let entries = match args {
        Some(semio_framework_value::DslValue::Object(entries)) => entries,
        _ => {
            return Err(semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.window-kit.arguments-required"), "the action requires an argument object"));
        }
    };
    let mut values = entries.iter().filter(|(name, _)| name == key).map(|(_, value)| value);
    let Some(value) = values.next() else {
        return Err(semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.window-kit.argument-required"), format!("the action requires argument '{key}'")));
    };
    if values.next().is_some() {
        return Err(semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.window-kit.argument-duplicate"), format!("the action argument '{key}' must occur exactly once")));
    }
    Ok(value)
}

/// 📝️ Reads one required canonical text argument without aliases, coercion, or fallback.
pub fn window_kit_required_text_argument(args: Option<&semio_framework_value::DslValue>, key: &str) -> Result<String, semio_framework_plugin::Fault> {
    match required_window_kit_argument(args, key)? {
        semio_framework_value::DslValue::String(value) => Ok(value.clone()),
        _ => Err(semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.window-kit.argument-type"), format!("the action argument '{key}' must be text"))),
    }
}

/// 📥️ Reads the change set a text window published under the `splices` edit argument (JSON list of `{offset, delete, insert}`):
/// the gesture, verbatim. Refuses a malformed list, an unknown member, and ranges that are not ascending and disjoint.
pub fn window_kit_splices_argument(args: Option<&semio_framework_value::DslValue>) -> Result<Vec<DraftSplice>, semio_framework_plugin::Fault> {
    draft_splices_from_json(&window_kit_required_text_argument(args, "splices")?)
}

/// 🔢️ Reads one required canonical non-negative `u32` index without truncation or fallback.
pub fn window_kit_required_index_argument(args: Option<&semio_framework_value::DslValue>, key: &str) -> Result<u32, semio_framework_plugin::Fault> {
    let value = required_window_kit_argument(args, key)?;
    let integer = match value {
        semio_framework_value::DslValue::Number(number) => number.as_u64().or_else(|| number.as_i64().and_then(|value| u64::try_from(value).ok())).or_else(|| {
            let value = number.as_f64();
            (value.is_finite() && value >= 0.0 && value.fract() == 0.0 && value <= u32::MAX as f64).then_some(value as u64)
        }),
        _ => None,
    };
    integer.and_then(|value| u32::try_from(value).ok()).ok_or_else(|| {
        semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.window-kit.argument-range"), format!("the action argument '{key}' must be a non-negative 32-bit integer"))
    })
}

/// 📊️ Declares the addressed editable-table contract used by stdio record formats.
pub fn addressed_table_window_kind() -> semio_framework_plugin::WindowKindDefinition {
    use semio_framework_plugin::app::{TableWindowKit, WindowKit};
    let mut definition = TableWindowKit::editable_window_kind();
    let action = definition.actions.iter_mut().find(|action| action.id == "set-cell").expect("TableWindowKit declares set-cell");
    let column = action.args.iter_mut().find(|argument| argument.id == "column").expect("set-cell declares column");
    column.required = true;
    definition
}

/// 🔐️ Declares a table cell address guarded by the rendered document revision.
pub fn revision_addressed_table_window_kind() -> semio_framework_plugin::WindowKindDefinition {
    use semio_framework_plugin::app::{TableWindowKit, WindowKit};
    use semio_framework_plugin::ActionArgDef;
    use semio_framework_ui_locale::LocalizedLabel;
    let mut definition = TableWindowKit::editable_window_kind();
    let action = definition.actions.iter_mut().find(|action| action.id == "set-cell").expect("TableWindowKit declares set-cell");
    action.args = vec![
        ActionArgDef::number("row", LocalizedLabel::native("Row", "Zeile")).required(),
        ActionArgDef::number("column", LocalizedLabel::native("Column", "Spalte")).required(),
        ActionArgDef::document_revision("revision", LocalizedLabel::native("Document revision", "Dokumentrevision")),
        ActionArgDef::text("value", LocalizedLabel::native("Value", "Wert")).min_length(0).required(),
    ];
    definition
}

/// 🔐️ Declares a table cell address guarded by its row's own revision — a target token, not the document's.
pub fn row_revision_addressed_table_window_kind() -> semio_framework_plugin::WindowKindDefinition {
    use semio_framework_plugin::ActionArgDef;
    use semio_framework_ui_locale::LocalizedLabel;
    let mut definition = revision_addressed_table_window_kind();
    let action = definition.actions.iter_mut().find(|action| action.id == "set-cell").expect("the revision-addressed table declares set-cell");
    let revision = action.args.iter_mut().find(|argument| argument.id == "revision").expect("set-cell declares revision");
    *revision = ActionArgDef::target_revision("revision", LocalizedLabel::native("Row revision", "Zeilenrevision"));
    definition
}

pub const ADD_TABLE_ROW_ACTION_ID: &str = "add-row";
pub const REMOVE_TABLE_ROW_ACTION_ID: &str = "remove-row";
pub const ADD_TABLE_COLUMN_ACTION_ID: &str = "add-column";
pub const REMOVE_TABLE_COLUMN_ACTION_ID: &str = "remove-column";
pub const SET_TABLE_HEADER_ACTION_ID: &str = "set-header";

/// 🧱️ Declares direct structural table edits beside revision-addressed cell editing — retained routes, classified
/// `Migrated` here like the kit scaffold classifies its own rows, so every app declaring the table assembles.
pub fn structural_table_window_kind() -> semio_framework_plugin::WindowKindDefinition {
    use semio_framework_plugin::ActionArgDef;
    use semio_framework_plugin::ActionDefinition;
    use semio_framework_plugin::ActionKind;
    use semio_framework_plugin::InteractiveJobClassification;
    use semio_framework_ui_locale::LocalizedLabel;
    let revision = || ActionArgDef::document_revision("revision", LocalizedLabel::native("Document revision", "Dokumentrevision"));
    let mut definition = revision_addressed_table_window_kind();
    let structural = [
        ActionDefinition::bounded_catalog(ADD_TABLE_ROW_ACTION_ID, LocalizedLabel::native("Add row", "Zeile hinzufügen"), ActionKind::Mutation).with_args(vec![revision()]).in_palette(false),
        ActionDefinition::bounded_catalog(REMOVE_TABLE_ROW_ACTION_ID, LocalizedLabel::native("Remove row", "Zeile entfernen"), ActionKind::Mutation)
            .with_args(vec![ActionArgDef::number("row", LocalizedLabel::native("Row", "Zeile")).required(), revision()])
            .in_palette(false),
        ActionDefinition::bounded_catalog(ADD_TABLE_COLUMN_ACTION_ID, LocalizedLabel::native("Add column", "Spalte hinzufügen"), ActionKind::Mutation).with_args(vec![revision()]).in_palette(false),
        ActionDefinition::bounded_catalog(REMOVE_TABLE_COLUMN_ACTION_ID, LocalizedLabel::native("Remove column", "Spalte entfernen"), ActionKind::Mutation)
            .with_args(vec![ActionArgDef::number("column", LocalizedLabel::native("Column", "Spalte")).required(), revision()])
            .in_palette(false),
        ActionDefinition::bounded_catalog(SET_TABLE_HEADER_ACTION_ID, LocalizedLabel::native("Set header", "Spaltenkopf setzen"), ActionKind::Mutation)
            .with_args(vec![ActionArgDef::number("column", LocalizedLabel::native("Column", "Spalte")).required(), revision(), ActionArgDef::text("value", LocalizedLabel::native("Header", "Spaltenkopf")).min_length(0).required()])
            .in_palette(false),
    ];
    definition.actions.extend(structural.map(|mut action| {
        action.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
        action
    }));
    definition
}

/// 🎯️ The complete ordinal address published by one revision-guarded table cell.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WindowKitRevisionedCellEdit {
    pub row: u32,
    pub column: u32,
    pub revision: String,
    pub value: String,
}

/// 🎯️ Parses a revision-guarded table cell edit without destructive defaults.
pub fn window_kit_revisioned_cell_edit(args: Option<&semio_framework_value::DslValue>) -> Result<WindowKitRevisionedCellEdit, semio_framework_plugin::Fault> {
    Ok(WindowKitRevisionedCellEdit {
        row: window_kit_required_index_argument(args, "row")?,
        column: window_kit_required_index_argument(args, "column")?,
        revision: window_kit_required_text_argument(args, "revision")?,
        value: window_kit_required_text_argument(args, "value")?,
    })
}

/// 🔐️ Computes the optimistic revision for one table projection from the typed snapshot.
pub fn window_kit_snapshot_revision<S: semio_framework_value::ToValue>(snapshot: &S) -> String {
    semio_framework_plugin::app::DocumentWindowKit::text_revision(&snapshot_edit_source(snapshot))
}

/// 🪪️ Encodes the store-owned revision already captured for a render or command operation.
pub fn window_kit_canonical_revision(revision: [u8; 32]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(64);
    for byte in revision {
        encoded.push(HEX[usize::from(byte >> 4)] as char);
        encoded.push(HEX[usize::from(byte & 0x0f)] as char);
    }
    encoded
}

/// 🧾️ Projects a canonical 32-byte hex revision onto the exact decimal u64 lane carried by
/// `AppFrame::OperationCompleted`, rejecting every malformed or non-canonical input.
pub fn window_kit_publication_revision(revision: &str) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiPublicationRevision> {
    use semio_framework_plugin::PluginAssemblyError;
    if revision.len() != 64 || !revision.bytes().all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase()) {
        return Err(PluginAssemblyError::new("stdio.table.publication-revision", "table publication revision must be 32 canonical lowercase hex bytes"));
    }
    let prefix = u64::from_str_radix(&revision[..16], 16).map_err(|_| PluginAssemblyError::new("stdio.table.publication-revision", "table publication revision prefix is invalid"))?;
    Ok(semio_framework_plugin::UiPublicationRevision(prefix))
}

/// 🎨️ Projects the exact store revision captured for a render pass onto the completion lane.
pub fn window_kit_artifact_publication_revision<S: semio_framework_value::ToValue>(doc: &semio_framework_plugin::ArtifactView<'_, S>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiPublicationRevision> {
    semio_framework_plugin::app::artifact_render_publication_revision(doc)
}

/// 🎨️ Projects one renderer-captured store authority without materializing or renumbering it.
pub fn window_kit_render_publication_revision<S: semio_framework_value::ToValue>(
    _snapshot: &S,
    render_operation: Option<semio_framework_plugin::AppRenderOperationContext>,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiPublicationRevision> {
    render_operation
        .map(semio_framework_plugin::app::render_operation_publication_revision)
        .ok_or_else(|| PluginAssemblyError::new("stdio.render.publication-revision", "editable rendering requires a render-pinned document revision"))
}

/// 🎟️ A refusal of the process-wide `UiValue` arena is the framework's capacity refusal: the row window building this
/// address ends there with a shorter run instead of refusing the editor render (`tree_window_indexed_rows`).
fn table_arena(stage: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new("ui.fixed-capacity", format!("stdio table {stage} admission failed: the UiValue arena has no free credit"))
}

/// 📍️ Builds one complete revision-guarded ordinal cell address without materializing its table.
pub fn window_kit_revisioned_cell_arguments(row: usize, column: usize, revision: &str) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    use semio_framework_plugin::{PluginAssemblyError, UiMapBuilder, UiText, UiValue};
    let mut arguments = UiMapBuilder::try_new().ok_or_else(|| table_arena("cell argument map"))?;
    arguments.try_insert("row".into(), UiValue::Number(row as f64)).map_err(|_| table_arena("row argument"))?;
    arguments.try_insert("column".into(), UiValue::Number(column as f64)).map_err(|_| table_arena("column argument"))?;
    arguments
        .try_insert("revision".into(), UiValue::Text(UiText::try_from_str(revision).ok_or_else(|| PluginAssemblyError::new("stdio.table.revision", "table revision exceeds the UI text bound"))?))
        .map_err(|_| table_arena("cell revision argument"))?;
    Ok(UiValue::Map(arguments.finish()))
}

/// 📍️ Builds one revision-only structural table address.
pub fn window_kit_revision_arguments(revision: &str) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    use semio_framework_plugin::{PluginAssemblyError, UiMapBuilder, UiText, UiValue};
    let mut arguments = UiMapBuilder::try_new().ok_or_else(|| table_arena("structure argument map"))?;
    arguments
        .try_insert("revision".into(), UiValue::Text(UiText::try_from_str(revision).ok_or_else(|| PluginAssemblyError::new("stdio.table.revision", "table revision exceeds the UI text bound"))?))
        .map_err(|_| table_arena("structure revision argument"))?;
    Ok(UiValue::Map(arguments.finish()))
}

/// 📍️ Builds one row- or column-indexed structural table address.
pub fn window_kit_indexed_revision_arguments(index_name: &str, index: usize, revision: &str) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    use semio_framework_plugin::{PluginAssemblyError, UiMapBuilder, UiText, UiValue};
    let mut arguments = UiMapBuilder::try_new().ok_or_else(|| table_arena("indexed argument map"))?;
    arguments.try_insert(index_name.into(), UiValue::Number(index as f64)).map_err(|_| table_arena("index argument"))?;
    arguments
        .try_insert("revision".into(), UiValue::Text(UiText::try_from_str(revision).ok_or_else(|| PluginAssemblyError::new("stdio.table.revision", "table revision exceeds the UI text bound"))?))
        .map_err(|_| table_arena("indexed revision argument"))?;
    Ok(UiValue::Map(arguments.finish()))
}

/// 🧱️ Adds localized structural controls and a windowed header editor around a table body. The controls are admitted
/// before `table` renders its windowed body: the body takes whatever `UiValue` arena credit is left and ends its window
/// short on the arena's capacity refusal, so a starved body never takes the controls down.
pub fn render_structural_table(
    header_count: usize,
    mut header: impl FnMut(usize) -> String,
    editable_headers: bool,
    controller_id: &str,
    revision: &str,
    publication_revision: semio_framework_plugin::UiPublicationRevision,
    locale: semio_framework_ui_locale::Locale,
    windows: &semio_framework_plugin::TreeWindows<'_>,
    table: impl FnOnce() -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode>,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    use semio_framework_plugin::app::{editable_table_window_row, row_action, row_target, TableWindowKit, WindowedEditableTableCell};
    use semio_framework_plugin::{ActionId, Buildable, HasBase, HasChildren, PluginAssemblyError, RowActionPlacement, Trigger};
    use semio_framework_ui_contract::{self as ui, Label as UiLabel};
    let labels = match locale {
        semio_framework_ui_locale::Locale::De => ("Zeile hinzufügen", "Spalte hinzufügen", "Spaltenköpfe", "Spaltenkopf", "Aktionen", "Spalte entfernen"),
        semio_framework_ui_locale::Locale::En => ("Add row", "Add column", "Headers", "Header", "Actions", "Remove column"),
    };
    let action = |name: &str| ActionId::try_v1(controller_id, name).ok_or_else(|| PluginAssemblyError::new("stdio.table.action", format!("invalid action {name}")));
    let add_row = ui::button(UiLabel::try_from(labels.0).map_err(|_| PluginAssemblyError::new("stdio.table.add-row", "button label admission"))?)
        .try_id("add-row")
        .map_err(|_| PluginAssemblyError::new("stdio.table.add-row", "button id admission"))?
        .try_on_with(Trigger::Activate, action(ADD_TABLE_ROW_ACTION_ID)?, window_kit_revision_arguments(revision)?)
        .map_err(|_| PluginAssemblyError::new("stdio.table.add-row", "button binding admission"))?
        .try_build()
        .map_err(|_| PluginAssemblyError::new("stdio.table.add-row", "button admission"))?;
    let add_column = ui::button(UiLabel::try_from(labels.1).map_err(|_| PluginAssemblyError::new("stdio.table.add-column", "button label admission"))?)
        .try_id("add-column")
        .map_err(|_| PluginAssemblyError::new("stdio.table.add-column", "button id admission"))?
        .try_on_with(Trigger::Activate, action(ADD_TABLE_COLUMN_ACTION_ID)?, window_kit_revision_arguments(revision)?)
        .map_err(|_| PluginAssemblyError::new("stdio.table.add-column", "button binding admission"))?
        .try_build()
        .map_err(|_| PluginAssemblyError::new("stdio.table.add-column", "button admission"))?;
    let toolbar = ui::row()
        .try_id("structure-actions")
        .map_err(|_| PluginAssemblyError::new("stdio.table.toolbar", "toolbar id admission"))?
        .try_children([add_row, add_column])
        .map_err(|_| PluginAssemblyError::new("stdio.table.toolbar", "toolbar child admission"))?
        .try_build()
        .map_err(|_| PluginAssemblyError::new("stdio.table.toolbar", "toolbar admission"))?;
    let mut children = vec![toolbar];
    children.push(TableWindowKit::render_indexed_rows_with_id(windows, "stdio-table-headers", labels.2, &[labels.3], Some(labels.4), header_count, |column| {
        let remove = row_action("trash-2", labels.5, REMOVE_TABLE_COLUMN_ACTION_ID, RowActionPlacement::Row)?;
        let target = row_target(controller_id, Some(window_kit_indexed_revision_arguments("column", column, revision)?), None)?;
        let value = header(column);
        let cell = if editable_headers {
            WindowedEditableTableCell::new(value, labels.3, SET_TABLE_HEADER_ACTION_ID, window_kit_indexed_revision_arguments("column", column, revision)?).publication_revision(publication_revision)
        } else {
            WindowedEditableTableCell::read_only(value, labels.3)
        };
        editable_table_window_row(&format!("header-{column}"), controller_id, locale, [cell], [remove], Some(target))
    })?);
    children.push(table()?);
    ui::column()
        .try_id("stdio-structural-table")
        .map_err(|_| PluginAssemblyError::new("stdio.table.root", "root id admission"))?
        .try_children(children)
        .map_err(|_| PluginAssemblyError::new("stdio.table.root", "root child admission"))?
        .try_build()
        .map_err(|_| PluginAssemblyError::new("stdio.table.root", "root admission"))
}

/// 📊️ Binds every present table cell to its row, column, and shared snapshot revision.
pub fn window_kit_revisioned_editable_cells(rows: &[Vec<String>], action_id: &str, revision: &str) -> semio_framework_plugin::UiAssemblyResult<Vec<semio_framework_plugin::app::EditableTableCell>> {
    rows.iter()
        .enumerate()
        .flat_map(|(row, cells)| cells.iter().enumerate().map(move |(column, _)| (row, column)))
        .map(|(row, column)| Ok(semio_framework_plugin::app::EditableTableCell::new(row, column, action_id, window_kit_revisioned_cell_arguments(row, column, revision)?)))
        .collect()
}

/// 🎯️ Declares a stable, revision-guarded spreadsheet cell address.
pub fn stable_addressed_table_window_kind() -> semio_framework_plugin::WindowKindDefinition {
    use semio_framework_plugin::app::{TableWindowKit, WindowKit};
    use semio_framework_plugin::ActionArgDef;
    use semio_framework_ui_locale::LocalizedLabel;
    let mut definition = TableWindowKit::editable_window_kind();
    let action = definition.actions.iter_mut().find(|action| action.id == "set-cell").expect("TableWindowKit declares set-cell");
    action.args = vec![
        ActionArgDef::text("sheetName", LocalizedLabel::native("Worksheet", "Arbeitsblatt")).required(),
        ActionArgDef::number("row", LocalizedLabel::native("Row", "Zeile")).required(),
        ActionArgDef::number("column", LocalizedLabel::native("Column", "Spalte")).required(),
        ActionArgDef::target_revision("revision", LocalizedLabel::native("Cell revision", "Zellrevision")),
        ActionArgDef::text("value", LocalizedLabel::native("Value", "Wert")).min_length(0).required(),
    ];
    definition
}

/// 🎯️ The complete stable address published by one editable spreadsheet cell.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WindowKitStableCellEdit {
    pub sheet_name: String,
    pub row: u32,
    pub column: u32,
    pub revision: String,
    pub value: String,
}

/// 🎯️ Parses a spreadsheet cell edit without ordinal aliases or destructive defaults.
pub fn window_kit_stable_cell_edit(args: Option<&semio_framework_value::DslValue>) -> Result<WindowKitStableCellEdit, semio_framework_plugin::Fault> {
    Ok(WindowKitStableCellEdit {
        sheet_name: window_kit_required_text_argument(args, "sheetName")?,
        row: window_kit_required_index_argument(args, "row")?,
        column: window_kit_required_index_argument(args, "column")?,
        revision: window_kit_required_text_argument(args, "revision")?,
        value: window_kit_required_text_argument(args, "value")?,
    })
}

/// 📄️ The complete canonical payload published by an editable document draft.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WindowKitDocumentTextEdit {
    pub page: u32,
    pub item: u32,
    pub revision: String,
    pub text: String,
}

/// 📄️ Parses a document draft payload without aliases, coercion, or destructive defaults.
pub fn window_kit_document_text_edit(args: Option<&semio_framework_value::DslValue>) -> Result<WindowKitDocumentTextEdit, semio_framework_plugin::Fault> {
    Ok(WindowKitDocumentTextEdit {
        page: window_kit_required_index_argument(args, "page")?,
        item: window_kit_required_index_argument(args, "item")?,
        revision: window_kit_required_text_argument(args, "revision")?,
        text: window_kit_required_text_argument(args, "text")?,
    })
}

/// 🔐️ Rejects a stale document text target before a mutation can be published.
pub fn require_window_kit_document_revision(text: &str, revision: &str, code: &'static str) -> Result<(), semio_framework_plugin::Fault> {
    if semio_framework_plugin::app::DocumentWindowKit::text_revision(text) == revision {
        return Ok(());
    }
    Err(semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new(code), "the saved document text changed while the local draft was open"))
}

/// 🎬️ Projects the exact live app/document/revision/generation authority into a media transport resource.
pub fn media_transport_resource<P>(doc: &semio_framework_plugin::ArtifactView<'_, P>, controller_id: &str) -> Option<semio_framework_plugin::app::MediaResource> {
    let operation = doc.render_operation()?;
    Some(semio_framework_plugin::app::MediaResource {
        controller_id: controller_id.to_string(),
        app_instance_id: operation.app_instance_id,
        parent_document_id: doc.parent_document_id()?.to_string(),
        output_port: semio_framework_plugin::app::MEDIA_PLAYBACK_OUTPUT_PORT.to_string(),
        revision: operation.base_revision.0.to_string(),
        generation: operation.generation.0.to_string(),
    })
}

/// ⏱️ Converts a positive finite inferred duration into the host's exact safe millisecond domain.
pub fn media_duration_ms(duration_seconds: f64) -> Option<u64> {
    const MAX_SAFE_INTEGER: f64 = 9_007_199_254_740_991.0;
    let milliseconds = duration_seconds * 1_000.0;
    (milliseconds.is_finite() && milliseconds > 0.0 && milliseconds <= MAX_SAFE_INTEGER).then(|| milliseconds.round().max(1.0) as u64)
}

/// 🧬️ The whole-document load an example switch hands the host. Whole-document replacement is not an
/// in-history mutation (no stdio subset mints a whole-snapshot edit for it), so the switch emits an
/// `Effect::LoadDocument` carrying the example's own pack plus an edit-free `.spr` log — the shape
/// `dag`/`reasoning.wires` already use, and the reason the route publishes on the HostOnly lane.
pub fn load_example_effect<P: pack::snapshot::ArtifactPack>(document: &P, schema: &'static str) -> semio_framework_plugin::Effect {
    let pack = document.encode_pack();
    let spr = ::semio_framework_async::poll::resolve_ready(kernel::os_store::text_format::empty_document_spr(schema, schema));
    semio_framework_plugin::Effect::LoadDocument { pack, spr }
}

/// 📂️ A natural-file or document-pack import as the whole-document load it is: the imported snapshot replaces the open document
/// through [`load_example_effect`] (no stdio subset mints a whole-snapshot mutation), decoded by the editor's own natural-file
/// codec on the reserved port or from the structured base64 pack on `artifact:in`. Any other port is not implemented.
pub fn import_media_as_load<E>(port: &str, media: &semio_framework_plugin::app::Media) -> Result<semio_framework_plugin::Emit<E::Mutation, E::ConfigMutation, E::DraftMutation>, semio_framework_plugin::MediaError>
where
    E: semio_framework_plugin::ArtifactEditor,
    E::Snapshot: pack::snapshot::ArtifactPack,
{
    use semio_framework_plugin::app::MediaPayload;
    use semio_framework_plugin::MediaError;
    let snapshot = if port == semio_framework_plugin::NATURAL_FILE_PORT {
        let codec = E::natural_file_codec().ok_or(MediaError::NotImplemented)?;
        let MediaPayload::Intrinsic { schema, value: value::DslValue::Bytes(bytes) } = &media.payload else {
            return Err(MediaError::Payload(port.to_string(), "natural file importer requires intrinsic owned bytes".into()));
        };
        if schema != codec.format_kind {
            return Err(MediaError::Payload(port.to_string(), format!("natural file schema mismatch: expected {}, found {schema}", codec.format_kind)));
        }
        E::decode_natural_file(bytes)?
    } else if port == "artifact:in" {
        let MediaPayload::Structured { json, .. } = &media.payload else {
            return Err(MediaError::Payload(port.to_string(), "document importer only accepts a Structured (base64 pack) payload".into()));
        };
        let bytes = kernel::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
        <E::Snapshot as pack::snapshot::ArtifactPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?
    } else {
        return Err(MediaError::NotImplemented);
    };
    Ok(semio_framework_plugin::Emit { effects: vec![load_example_effect(&snapshot, E::DOCUMENT_SCHEMA)], ..Default::default() })
}

/// 📇️ The picker's own action declaration — one row per stdio editor, so the nine editors cannot
/// drift on label, kind or icon. Its label is the framework's own localized label of the load-example verb
/// (`app::runtime_verb_label`, design §20.6 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING), so the history row a load
/// records never reads a hand-written description. The caller still chains `.action_args(..)`, `.action_destructive(..)`
/// and `.action_interactive_job(.., Migrated)`: only `Migrated` survives UI dispatch, and only a
/// destructive row makes the MCP gateway ask before replacing a document.
pub fn set_active_example_action() -> semio_framework_plugin::ActionDefinition {
    let label = semio_framework_plugin::app::runtime_verb_label(SET_ACTIVE_EXAMPLE_ACTION_ID).expect("the framework labels its load-example verb");
    semio_framework_plugin::ActionDefinition::new(SET_ACTIVE_EXAMPLE_ACTION_ID, label, semio_framework_plugin::ActionKind::Mutation, "file")
}

/// 💬️ The agent-facing description of every stdio editor's `setActiveExample` — one sentence for all formats,
/// beside the argument form every stdio editor already shares.
pub fn set_active_example_description() -> semio_framework_ui_locale::LocalizedLabel {
    semio_framework_ui_locale::LocalizedLabel::native(
        "Replaces the whole open document with one of the bundled examples for its format, by example id.",
        "Ersetzt das gesamte offene Dokument durch eines der mitgelieferten Beispiele für sein Format, anhand der Beispiel-Id.",
    )
}

/// 📝️ The picker's typed argument: one option per example this editor's subset publishes, defaulting
/// to the one the pane boots.
pub fn set_active_example_args(options: &[(&str, semio_framework_ui_locale::LocalizedLabel)], default_example_id: &str) -> Vec<semio_framework_plugin::ActionArgDef> {
    vec![semio_framework_plugin::ActionArgDef::select(
        "exampleId",
        semio_framework_ui_locale::LocalizedLabel::native("Example", "Beispiel"),
        options.iter().map(|(id, label)| semio_framework_plugin::ActionArgOption::new(*id, label.clone())).collect(),
    )
    .default_value(&default_example_id.to_string())]
}
//#endregion 🎬️ExampleSwitch

#[cfg(test)]
#[path = "../🧪️tests/🪟️window-kit/🦀️.rs"]
mod tests;
