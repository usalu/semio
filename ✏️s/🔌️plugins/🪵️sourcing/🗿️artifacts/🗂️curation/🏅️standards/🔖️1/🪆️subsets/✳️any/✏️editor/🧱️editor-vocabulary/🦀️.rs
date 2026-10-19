//! 🧱️ Sourcing editor core: the typed media I/O surface, the controller and interaction ids, the action and table factories, the UI value factories and the whole-document reset effect every other layer of the editor builds on.

use semio_framework_plugin::{ActionDescriptor, MediaClass, MediaForm, MediaType};
use pack::snapshot::ArtifactPack;
use crate::{CurationSnapshot, SOURCING_CURATION_SCHEMA};

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — the implicit document ports (keyed off
/// `SOURCING_CURATION_SCHEMA`, `MediaType{Kit,Kit}` matching the `"catalogue.sourcing"` `ArtifactKindSpec`)
/// plus the extra `catalog:out` output port: this app's `stock` (its `"catalogue.kinds"`-shaped rows)
/// mapped into the SAME `kit.catalog` JSON shape `block_3d::puzzle3d_catalog_fragment` produces, so
/// `s/plugin/puzzle`'s `kit:in` importer can consume either producer identically without knowing which
/// one it came from (see `crate::schema::inferences::sourcing_catalog_fragment`).
pub fn sourcing_curation_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        artifact_schema: SOURCING_CURATION_SCHEMA.into(),
        artifact_media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Kit },
        ports: vec![semio_framework_plugin::MediaPortSpec {
            id: "catalog:out".into(),
            label: "Catalog".into(),
            direction: semio_framework_plugin::MediaPortDirection::Out,
            media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
            kind_id: Some("kit.catalog".into()),
            required: false,
            multiplicity: semio_framework::PortMultiplicity::Many,
        }],
        export_formats: vec![],
        import_formats: vec![],
        artifact: semio_framework_plugin::ArtifactPresentation { id: "catalogue.sourcing".into(), name: "Sourcing Curation".into(), dimension: "data".into(), component_kind: "catalogue".into() },
    }
}
//#endregion 🔖️Io

//#region 🔖️Constants
/// 🎯️ Still used as a plain namespace tag for `ActionFactory`/`TableScene`/`WorldScene` addressing
/// below — NOT a trait const any more (contract §7.4: the real surface id is now derived from
/// `SOURCING_DIALECT` + `AppRole` via `surface_app_id`, never hand-written).
pub const SOURCING_CONTROLLER_ID: &str = "sourcing-curation";
pub const SOURCING_DRAG_MIME: &str = "application/x-semio-sourcing-object";
/// 🕹️ The framework interaction domain every sourcing table row and grid object is picked in.
pub const SOURCING_ROWS_DOMAIN: &str = "rows";
/// 🕹️ The granularity a stock kind is picked under in [`SOURCING_ROWS_DOMAIN`].
pub const SOURCING_OBJECT_GRANULARITY: &str = "object";
pub const DEMO_STOCK_EXAMPLE_ID: &str = crate::examples::demo::ID;
/// 🫙️ The shell's "no example" selection.
pub const EMPTY_EXAMPLE_ID: &str = "";

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// builds its `on_change`/drop actions with.
pub fn sourcing_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(SOURCING_CONTROLLER_ID).action(action, args)
}

/// 🪟️ Bridges semantic app actions into the retained window-measure transport.
pub fn sourcing_window_action(action: &str, args: Option<semio_framework_value::DslValue>) -> ActionDescriptor {
    ActionDescriptor { controller_id: SOURCING_CONTROLLER_ID.into(), action: action.into(), args }
}

/// 🎯️ The table-scene action descriptor a stepper, row button or drop target dispatches to this app.
pub fn sourcing_table_action(action: &str, object_id: Option<&str>) -> ActionDescriptor {
    let args = object_id.map(|id| semio_framework_value::DslValue::object([("objectId".to_string(), semio_framework_value::DslValue::String(id.to_string()))]));
    ActionDescriptor { controller_id: SOURCING_CONTROLLER_ID.into(), action: action.into(), args }
}

/// 🧾️ One table row record for a stock kind: its id, the `{objectId}` payload a drag carries under
/// [`SOURCING_DRAG_MIME`], and `cells` keyed by column id.
pub fn sourcing_table_row(object_id: &str, cells: Vec<(&str, semio_framework_plugin::TableCell)>) -> semio_framework_value::DslValue {
    let mut entries = vec![
        ("id".to_string(), semio_framework_value::DslValue::String(object_id.to_string())),
        ("_drag".to_string(), semio_framework_value::DslValue::object([("objectId".to_string(), semio_framework_value::DslValue::String(object_id.to_string()))])),
    ];
    entries.extend(cells.into_iter().map(|(column, cell)| (column.to_string(), semio_framework_value::ToValue::to_value(&cell))));
    semio_framework_value::DslValue::object(entries)
}

/// 📊️ One sourcing table surface: `(id, label, sortable)` columns, [`sourcing_table_row`] records
/// draggable under [`SOURCING_DRAG_MIME`], and `drop_action` answering a row dropped onto it.
pub fn sourcing_table(surface_id: &str, columns: &[(&str, &str, bool)], rows: Vec<semio_framework_value::DslValue>, drop_action: &str, sort: Option<&crate::TableSort>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let columns = columns
        .iter()
        .map(|(id, label, sortable)| semio_framework_value::DslValue::object([("id".to_string(), semio_framework_value::DslValue::String(id.to_string())), ("label".to_string(), semio_framework_value::DslValue::String(label.to_string())), ("sortable".to_string(), semio_framework_value::DslValue::Bool(*sortable))]))
        .collect();
    let mut scene = semio_framework_plugin::TableScene::base(semio_framework_pack_json::to_json_string(&semio_framework_value::DslValue::Array(columns)), semio_framework_pack_json::to_json_string(&semio_framework_value::DslValue::Array(rows)));
    scene.row_drag_mime = Some(SOURCING_DRAG_MIME.into());
    scene.drop_action_json = Some(semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(&sourcing_table_action(drop_action, None))));
    scene.sort_json = sort.map(|sort| semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(sort)));
    scene.domain_id = Some(SOURCING_ROWS_DOMAIN.into());
    scene.domain_granularity_id = Some(SOURCING_OBJECT_GRANULARITY.into());
    semio_framework_plugin::scene_surface(surface_id, semio_framework_plugin::plugin_app_close_prelude::SurfaceKind::Table, &scene)
}

/// 🧱️ Admits one fixed UI text action value without JSON staging.
pub fn ui_value_text(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    semio_framework_plugin::UiText::try_from_str(value.as_ref())
        .map(semio_framework_plugin::UiValue::Text)
        .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI text admission failed"))
}

/// 🔘️ Admits one boolean UI action value.
pub fn ui_value_bool(value: bool) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Bool(value)
}

/// 🔢️ Admits one numeric UI action value.
pub fn ui_value_number(value: impl Into<f64>) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Number(value.into())
}

/// 📚️ Admits one fixed UI list action value without dynamic staging.
pub fn ui_value_list(values: impl IntoIterator<Item = semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiListBuilder::try_new()
        .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list admission failed"))?;
    for value in values {
        builder
            .push(value)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list item admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::List(builder.finish()))
}

/// 🗺️ Admits one ordered fixed UI map action value without JSON staging.
pub fn ui_value_map(values: impl IntoIterator<Item = (&'static str, semio_framework_plugin::UiValue)>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new()
        .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map admission failed"))?;
    for (key, value) in values {
        builder
            .push(key.to_owned(), value)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map entry admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::Map(builder.finish()))
}

//#endregion 🔖️Constants

//#region 🔖️ResetDocument
/// 🌱️ Builds a `Effect::LoadDocument` that swaps the live document to `document` OUTSIDE undo
/// history — the sanctioned non-mutation path for a whole-document replace (JSON import, load-
/// example, bulk catalogue restock). Per `📓️taxonomy.md`, the former whole-snapshot-replace variant
/// is banned outright with NO replacement mutation: whole-document replace is not expressible as an in-history `Mutation` at
/// all. Every former "replace the whole document" gesture in this app (`import_media`'s
/// `"artifact:in"` above, `commands::document::{set_active_example, load_document_json,
/// stock_from_catalogue}`) builds this effect instead of an `Emit::mutations([...])`. The spr is a
/// fresh, edit-free op-log for `document` — a genesis envelope with no history to encode.
pub fn reset_document_effect(document: &CurationSnapshot) -> semio_framework::kernel::Effect {
    let pack = <CurationSnapshot as ArtifactPack>::encode_pack(document);
    let spr = ::semio_framework_async::poll::resolve_ready(store::os_store::text_format::empty_document_spr("curation", SOURCING_CURATION_SCHEMA));
    semio_framework::kernel::Effect::LoadDocument { pack, spr }
}
//#endregion 🔖️ResetDocument

//#region 📏️ConfigCaps
/// 🧩️ The dedicated retained contributions lane. A `sourcing.module` pack is a host-pushed capability
/// blob, never something a person types, so it is priced here rather than against the 96-byte filter
/// envelope that used to refuse every real pack.
///
/// 📏️ The ceiling is MEASURED, not chosen: the host retires a retained config one
/// `ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES` (4 KiB) page per close turn, and a config carrying more than
/// ~3 KiB of contributions text never reaches its terminal-empty shell once a body has rendered
/// against it — 2 909 bytes closes, 3 302 bytes livelocks (see
/// `📓️fix-2026-09-16-sourcing-contributions-envelope.md` §3). 2 KiB keeps a full page of margin under
/// that cliff. `schema::installable_contributions` is what keeps the app inside it: the app retains
/// the modules it can act on, never the host's whole pack.
pub const SOURCING_CURATION_CONFIG_CONTRIBUTIONS_BYTES: usize = 2_048;
/// 📏️ The encoded-text envelope every retained FILTER edit is priced against — read by the window
/// nodes' tests to pin what a typed filter can and cannot carry.
pub const SOURCING_CURATION_CONFIG_TEXT_BYTES: usize = 96;
//#endregion 📏️ConfigCaps

//#region 📚️Examples
/// 📚️ The subset's registered examples — the one source the manifest picker, the `setActiveExample` argument form and its handler all resolve ids against.
pub fn examples() -> &'static [semio_framework_plugin::ExampleSource] {
    static EXAMPLES: std::sync::OnceLock<Vec<semio_framework_plugin::ExampleSource>> = std::sync::OnceLock::new();
    EXAMPLES.get_or_init(|| vec![crate::examples::demo::source()]).as_slice()
}
//#endregion 📚️Examples
