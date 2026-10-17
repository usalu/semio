//! 🌲️ Collection and item controls, the detail tree, the details window and its rendering.

use super::*;

/// 📝️ Renders an artifact's natural file source beside its structured canvas, with an explicit
/// localized Apply/Discard draft routed through the artifact-owned whole-document action.
pub fn render_file_source_editor(
    surface_id: &str,
    source: String,
    language: &str,
    action_id: &str,
    root_node_id: &str,
    revision: &str,
    publication_revision: ui_contract::UiPublicationRevision,
    locale: Locale,
    body: BuiltNode,
) -> UiAssemblyResult<BuiltNode> {
    let labels = labels(locale);
    let source = TextWindowKit::render_draft(&TextDraftView {
        surface_id: surface_id.into(),
        text: source,
        language: Some(language.into()),
        action_id: action_id.into(),
        argument: "value".into(),
        arguments: Some(ui_args([("nodeId", ui_text_value(root_node_id)?), ("revision", ui_text_value(revision)?)])?),
        publication_revision,
        apply_label: labels.apply.into(),
        discard_label: labels.discard.into(),
        conflict_label: labels.conflict.into(),
        applying_label: labels.applying.into(),
        cancel_label: labels.cancel.into(),
        failed_label: labels.failed.into(),
        syntax_error_label: labels.syntax_error.into(),
        validation_error_label: labels.validation_error.into(),
        path_label: labels.path.into(),
        line_label: labels.line.into(),
        column_label: labels.column.into(),
    })?;
    ui::column().try_id(format!("{surface_id}-layout")).map_err(|_| error("ui.file-source.layout-id"))?.try_children(vec![source, body]).map_err(|_| error("ui.file-source.layout-children"))?.try_build().map_err(|_| error("ui.file-source.layout"))
}

fn enum_control(id: &str, path: &str, current: &SnapshotDetailValue, values: &[DslValue], labels: Labels, controller_id: &str, publication_revision: Option<ui_contract::UiPublicationRevision>) -> UiAssemblyResult<BuiltNode> {
    if values.len() <= ui_contract::UI_VALUE_MAX_ITEMS {
        if let SnapshotDetailValue::String(current) = current {
            if UiText::try_from_str(current).is_some() && values.iter().all(|value| matches!(value, DslValue::String(value) if UiText::try_from_str(value).is_some())) {
                let mut builder = ui::select(text(current)?).try_label(labels.value).map_err(|_| error("ui.snapshot-details.enum-label"))?.try_id(format!("{id}-enum")).map_err(|_| error("ui.snapshot-details.enum-id"))?;
                for value in values {
                    let DslValue::String(value) = value else { unreachable!("checked string enum") };
                    builder = builder.try_item(text(value)?, label(value)?).map_err(|_| error("ui.snapshot-details.enum-option"))?;
                }
                return bind(builder, ui::Trigger::Change, controller_id, SET_SNAPSHOT_VALUE_ACTION_ID, path_args(path)?)?.try_build().map_err(|_| error("ui.snapshot-details.enum"));
            }
        }
    }
    let controls = values.iter().enumerate().map(|(index, value)| {
        let source = snapshot_edit_source(value);
        template_control(&format!("{id}-enum-{index}"), &format!("{} {source}", labels.choose), path, value.clone(), false, labels, controller_id, publication_revision)
    });
    control_group(&format!("{id}-enum"), controls)
}

fn scalar_control(
    id: &str,
    path: &str,
    value: SnapshotDetailValue,
    template: Option<DslValue>,
    enum_values: &[DslValue],
    allows_untyped_creation: bool,
    labels: Labels,
    controller_id: &str,
    publication_revision: Option<ui_contract::UiPublicationRevision>,
) -> UiAssemblyResult<BuiltNode> {
    if !enum_values.is_empty() {
        return enum_control(id, path, &value, enum_values, labels, controller_id, publication_revision);
    }
    match value {
        SnapshotDetailValue::Bool(value) => {
            let builder = ui::toggle(value).try_label(labels.value).map_err(|_| error("ui.snapshot-details.toggle-label"))?.try_id(format!("{id}-toggle")).map_err(|_| error("ui.snapshot-details.toggle-id"))?;
            bind(builder, ui::Trigger::Change, controller_id, SET_SNAPSHOT_VALUE_ACTION_ID, path_args(path)?)?.try_build().map_err(|_| error("ui.snapshot-details.toggle"))
        }
        SnapshotDetailValue::Number(value) => {
            let value = snapshot_edit_source(&DslValue::Number(value));
            set_input(&format!("{id}-number"), path, ui::InputKind::Text, &value, labels.value, labels, controller_id, true, publication_revision)
        }
        SnapshotDetailValue::String(value) if UiText::try_from_str(&value).is_some() => set_input(&format!("{id}-text"), path, ui::InputKind::Text, &value, labels.value, labels, controller_id, false, publication_revision),
        SnapshotDetailValue::String(value) => text_draft(&format!("{id}-draft"), value, Some(path), labels, publication_revision),
        SnapshotDetailValue::Null => {
            let candidates = [(labels.add_text, "\"\""), (labels.add_number, "0"), (labels.add_true, "true"), (labels.add_false, "false"), (labels.add_object, "{}"), (labels.add_list, "[]")];
            let mut controls = Vec::new();
            if let Some(template) = template.filter(|template| !matches!(template, DslValue::Null)) {
                controls.push(template_control(&format!("{id}-null-template"), labels.create, path, template, false, labels, controller_id, publication_revision)?);
            }
            if allows_untyped_creation {
                for (index, (title, value)) in candidates.into_iter().enumerate() {
                    controls.push(button(&format!("{id}-null-{index}"), title, "plus", controller_id, SET_SNAPSHOT_VALUE_ACTION_ID, static_value_args(path, value)?)?);
                }
            }
            control_group(&format!("{id}-null"), controls.into_iter().map(Ok))
        }
        SnapshotDetailValue::Bytes { len } => read_only_value(&format!("{id}-octets"), SnapshotDetailValue::Bytes { len }),
        SnapshotDetailValue::Array | SnapshotDetailValue::Object => Err(error("ui.snapshot-details.scalar-kind")),
    }
}

fn rename_control(id: &str, path: &str, key: &str, labels: Labels, controller_id: &str, publication_revision: Option<ui_contract::UiPublicationRevision>) -> UiAssemblyResult<BuiltNode> {
    let draft_target = input_draft_target(controller_id, RENAME_SNAPSHOT_KEY_ACTION_ID, path);
    if UiText::try_from_str(key).is_none() || draft_target.is_none() {
        return action_text_draft(&format!("{id}-key-draft"), key.into(), None, RENAME_SNAPSHOT_KEY_ACTION_ID, Some(path_args(path)?), labels, publication_revision);
    }
    let mut builder = ui::input(ui::InputKind::Text)
        .value(text(key)?)
        .commit(text("blur")?)
        .draft_target(draft_target.expect("checked bounded semantic target"))
        .try_label(labels.key)
        .map_err(|_| error("ui.snapshot-details.key-label"))?
        .try_id(format!("{id}-key"))
        .map_err(|_| error("ui.snapshot-details.key-id"))?;
    if let Some(publication_revision) = publication_revision {
        builder = builder.publication_revision(publication_revision);
    }
    bind(builder, ui::Trigger::Commit, controller_id, RENAME_SNAPSHOT_KEY_ACTION_ID, path_args(path)?)?.try_build().map_err(|_| error("ui.snapshot-details.key"))
}

fn collection_insert_path<P: SnapshotDetailsProvider + ?Sized>(provider: &P, path: &[SnapshotDetailPathSegment], value: &SnapshotDetailValue) -> Option<String> {
    let base = pointer(path);
    if matches!(value, SnapshotDetailValue::Array) {
        return Some(format!("{base}/-"));
    }
    provider.collection_insertion_key(path).map(|key| format!("{base}/{}", key.replace('~', "~0").replace('/', "~1")))
}

fn collection_controls<P: SnapshotDetailsProvider + ?Sized>(
    provider: &P,
    path: &[SnapshotDetailPathSegment],
    value: &SnapshotDetailValue,
    id: &str,
    labels: Labels,
    controller_id: &str,
    windows: &TreeWindows<'_>,
    publication_revision: Option<ui_contract::UiPublicationRevision>,
) -> UiAssemblyResult<BuiltNode> {
    let insert_path = collection_insert_path(provider, path, value);
    let candidates = [(labels.add_text, "\"\""), (labels.add_number, "0"), (labels.add_true, "true"), (labels.add_false, "false"), (labels.add_object, "{}"), (labels.add_list, "[]"), (labels.add_null, "null")];
    let mut builder = ui::column().try_id(format!("{id}-collection-controls")).map_err(|_| error("ui.snapshot-details.collection-controls-id"))?;
    let variant_count = provider.variant_count(path, labels.locale);
    if variant_count > 0 {
        let variants_id = format!("{id}-variants");
        let variants = tree_window_indexed_section(windows, &variants_id, label(labels.switch_to)?, true, variant_count, |index| {
            let variant = provider.variant(path, labels.locale, index).ok_or_else(|| error("ui.snapshot-details.variant"))?;
            let title = format!("{} {}", labels.switch_to, variant.label);
            variant_control(&format!("{id}-variant-{index}"), &title, &pointer(path), variant.value, labels, controller_id, publication_revision)
        })?;
        builder = builder.try_child(variants).map_err(|_| error("ui.snapshot-details.variant-controls"))?;
    }
    let mut controls = Vec::new();
    if provider.allows_collection_insert(path) {
        if let (Some(insert_path), Some(template)) = (insert_path.as_deref().filter(|path| path_is_bindable(path)), provider.creation_template(path, true)) {
            controls.push(template_control(&format!("{id}-add-template"), labels.create, insert_path, template, true, labels, controller_id, publication_revision)?);
        }
        let property_count = provider.missing_property_count(path);
        if property_count > 0 {
            let properties_id = format!("{id}-missing-properties");
            let properties = tree_window_indexed_section(windows, &properties_id, label(labels.add)?, true, property_count, |index| {
                let (key, template) = provider.missing_property_template(path, index).ok_or_else(|| error("ui.snapshot-details.missing-property"))?;
                let mut property = path.to_vec();
                property.push(SnapshotDetailPathSegment::Key(key.clone()));
                let property_path = pointer(&property);
                if !path_is_bindable(&property_path) {
                    return text_draft(&format!("{id}-add-property-source-{index}"), provider.source().ok_or_else(|| error("ui.snapshot-details.unbindable-property-without-source"))?, None, labels, publication_revision);
                }
                let property_label = provider.presentation(&property, labels.locale).map_or_else(|| key.clone(), |presentation| presentation.label);
                let title = format!("{} {property_label}", labels.add);
                template_control(&format!("{id}-add-property-{index}"), &title, &property_path, template, true, labels, controller_id, publication_revision)
            })?;
            builder = builder.try_child(properties).map_err(|_| error("ui.snapshot-details.missing-property-controls"))?;
        }
    }
    if provider.allows_collection_insert(path) && provider.allows_untyped_creation(path) && insert_path.as_deref().is_some_and(path_is_bindable) {
        let insert_path = insert_path.as_deref().expect("checked bindable insertion path");
        for (index, (title, value)) in candidates.into_iter().enumerate() {
            controls.push(button(&format!("{id}-add-{index}"), title, "plus", controller_id, INSERT_SNAPSHOT_VALUE_ACTION_ID, static_value_args(insert_path, value)?)?);
        }
    }
    if !controls.is_empty() {
        builder = builder.try_child(control_group(&format!("{id}-add"), controls.into_iter().map(Ok))?).map_err(|_| error("ui.snapshot-details.add-controls"))?;
    }
    builder.try_build().map_err(|_| error("ui.snapshot-details.collection-controls"))
}

fn item_controls(
    id: &str,
    path: &str,
    key: Option<&str>,
    array_index: Option<(usize, usize)>,
    value: Option<SnapshotDetailValue>,
    template: Option<DslValue>,
    enum_values: &[DslValue],
    allows_untyped_creation: bool,
    capabilities: SnapshotDetailItemCapabilities,
    labels: Labels,
    controller_id: &str,
    publication_revision: Option<ui_contract::UiPublicationRevision>,
) -> UiAssemblyResult<BuiltNode> {
    let mut controls = Vec::new();
    if capabilities.rename {
        let key = key.ok_or_else(|| error("ui.snapshot-details.rename-without-key"))?;
        controls.push(rename_control(id, path, key, labels, controller_id, publication_revision));
    }
    if capabilities.edit {
        if let Some(value) = value {
            controls.push(scalar_control(id, path, value, template, enum_values, allows_untyped_creation, labels, controller_id, publication_revision));
        }
    }
    if capabilities.reorder {
        let (index, length) = array_index.ok_or_else(|| error("ui.snapshot-details.reorder-without-index"))?;
        if index > 0 {
            let parent = path.rsplit_once('/').map_or("", |(parent, _)| parent);
            let destination = format!("{parent}/{}", index - 1);
            if path_is_bindable(&destination) {
                controls.push(button(&format!("{id}-up"), labels.move_up, "arrow-up", controller_id, MOVE_SNAPSHOT_VALUE_ACTION_ID, ui_args([pointer_argument(path, "from", "fromChunks")?, pointer_argument(&destination, "path", "pathChunks")?])?));
            }
        }
        if index + 1 < length {
            let parent = path.rsplit_once('/').map_or("", |(parent, _)| parent);
            let destination = format!("{parent}/{}", index + 1);
            if path_is_bindable(&destination) {
                controls.push(button(
                    &format!("{id}-down"),
                    labels.move_down,
                    "arrow-down",
                    controller_id,
                    MOVE_SNAPSHOT_VALUE_ACTION_ID,
                    ui_args([pointer_argument(path, "from", "fromChunks")?, pointer_argument(&destination, "path", "pathChunks")?])?,
                ));
            }
        }
    }
    if capabilities.remove {
        controls.push(button(&format!("{id}-remove"), labels.remove, "trash-2", controller_id, REMOVE_SNAPSHOT_VALUE_ACTION_ID, path_args(path)?));
    }
    control_group(&format!("{id}-controls"), controls)
}

fn detail_node<P: SnapshotDetailsProvider + ?Sized>(
    provider: &P,
    path: &[SnapshotDetailPathSegment],
    title: &str,
    key: Option<&str>,
    array_index: Option<(usize, usize)>,
    labels: Labels,
    controller_id: &str,
    windows: &TreeWindows<'_>,
    publication_revision: Option<ui_contract::UiPublicationRevision>,
) -> UiAssemblyResult<BuiltNode> {
    let value = provider.value(path).ok_or_else(|| error("ui.snapshot-details.value-missing"))?;
    let id = if path.is_empty() { ROOT_PATH_ID.to_string() } else { path_id(path) };
    let path_pointer = pointer(path);
    let is_collection = matches!(value, SnapshotDetailValue::Array | SnapshotDetailValue::Object);
    let scalar = (!is_collection).then(|| value.clone());
    let template = provider.creation_template(path, false);
    let enum_values = provider.enum_values(path);
    let allows_untyped_creation = provider.allows_untyped_creation(path);
    let capabilities = provider.item_capabilities(path);
    let controls = if path_is_bindable(&path_pointer) {
        item_controls(&id, &path_pointer, key, array_index, scalar, template, &enum_values, allows_untyped_creation, capabilities, labels, controller_id, publication_revision)?
    } else {
        text_draft(&format!("{id}-source-fallback"), provider.source().ok_or_else(|| error("ui.snapshot-details.unbindable-path-without-source"))?, None, labels, publication_revision)?
    };
    let presentation = provider.presentation(path, labels.locale);
    let title = presentation.as_ref().map_or(title, |presentation| presentation.label.as_str());
    let mut builder = ui_contract::tree_item(label(title)?).default_open(is_collection).try_id(&id).map_err(|_| error("ui.snapshot-details.item-id"))?.try_child(controls).map_err(|_| error("ui.snapshot-details.item-controls"))?;
    if let Some(description) = presentation.and_then(|presentation| presentation.description) {
        builder = builder.description(UiText::clipped(&description));
    }
    if is_collection {
        if path_is_bindable(&path_pointer) {
            builder = builder.try_child(collection_controls(provider, path, &value, &id, labels, controller_id, windows, publication_revision)?).map_err(|_| error("ui.snapshot-details.collection-controls"))?;
        }
        let count = provider.child_count(path);
        let children_id = format!("{id}-children");
        return tree_window_indexed_item(windows, builder, &children_id, true, count, |index| detail_child(provider, path, &value, index, count, labels, controller_id, windows, publication_revision));
    }
    builder.try_build().map_err(|_| error("ui.snapshot-details.item"))
}

fn detail_child<P: SnapshotDetailsProvider + ?Sized>(
    provider: &P,
    path: &[SnapshotDetailPathSegment],
    value: &SnapshotDetailValue,
    index: usize,
    count: usize,
    labels: Labels,
    controller_id: &str,
    windows: &TreeWindows<'_>,
    publication_revision: Option<ui_contract::UiPublicationRevision>,
) -> UiAssemblyResult<BuiltNode> {
    let (segment, title, key, array_index) = if matches!(value, SnapshotDetailValue::Object) {
        let key = provider.object_key(path, index).ok_or_else(|| error("ui.snapshot-details.object-key"))?;
        (SnapshotDetailPathSegment::Key(key.clone()), key.clone(), Some(key), None)
    } else {
        (SnapshotDetailPathSegment::Index(index), format!("{} {}", labels.item, index + 1), None, Some((index, count)))
    };
    let mut child_path = path.to_vec();
    child_path.push(segment);
    detail_node(provider, &child_path, &title, key.as_deref(), array_index, labels, controller_id, windows, publication_revision)
}

pub(super) fn source_row(source: String, labels: Labels, publication_revision: Option<ui_contract::UiPublicationRevision>) -> UiAssemblyResult<BuiltNode> {
    let controls = text_draft("stdio-snapshot-details-source-draft", source, None, labels, publication_revision)?;
    ui_contract::tree_item(label(labels.source)?)
        .try_id(SOURCE_ROW_ID)
        .map_err(|_| error("ui.snapshot-details.source-id"))?
        .try_child(controls)
        .map_err(|_| error("ui.snapshot-details.source-controls"))?
        .try_build()
        .map_err(|_| error("ui.snapshot-details.source"))
}

pub fn snapshot_details_window_definition() -> WindowKindDefinition {
    let mut definition = TreeWindowKit::window_kind();
    definition.id = SNAPSHOT_DETAILS_WINDOW_KIND_ID.into();
    definition.body_key = SNAPSHOT_DETAILS_BODY_KEY.into();
    definition.label = LocalizedLabel::native("Details", "Details");
    definition.icon_id = "list-tree".into();
    definition.actions = snapshot_edit_actions();
    for action in &mut definition.actions {
        action.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
    }
    definition
}

pub fn snapshot_details_split_layout(main_window_kind_id: &str, main_title: &str) -> WindowLayout {
    let stack = |size, window_kind_id: &str, title: &str| {
        WindowLayoutChild::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size: Some(size),
            active_window_kind_id: None,
            children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: window_kind_id.into(), title: Some(title.into()), instance_id: None, template_id: None, corner: None }],
        })
    };
    WindowLayout { root: WindowLayoutRoot::Axis(WindowLayoutAxisNode { kind: "row".into(), size: None, children: vec![stack(0.68, main_window_kind_id, main_title), stack(0.32, SNAPSHOT_DETAILS_WINDOW_KIND_ID, "Details")] }) }
}

fn render_snapshot_details_provider_at_revision<P: SnapshotDetailsProvider + ?Sized>(
    provider: &P,
    locale: Locale,
    controller_id: &str,
    windows: &TreeWindows<'_>,
    publication_revision: Option<ui_contract::UiPublicationRevision>,
) -> UiAssemblyResult<BuiltNode> {
    let labels = labels(locale);
    let mut builder = ui_contract::tree().try_id(ROOT_ID).map_err(|_| error("ui.snapshot-details.root-id"))?;
    if provider.has_source() {
        let source = semio_framework_plugin::tree_window_section(windows, SOURCE_ID, label(labels.source)?, false, &[()], |_| {
            let source = provider.source().ok_or_else(|| error("ui.snapshot-details.source-unavailable"))?;
            source_row(source, labels, publication_revision)
        })?;
        builder = builder.try_child(source).map_err(|_| error("ui.snapshot-details.source-section"))?;
    }
    let value = provider.value(&[]).ok_or_else(|| error("ui.snapshot-details.root-value"))?;
    let fields = if matches!(value, SnapshotDetailValue::Object | SnapshotDetailValue::Array) {
        let controls = provider.variant_count(&[], locale) > 0
            || (provider.allows_collection_insert(&[])
                && (provider.missing_property_count(&[]) > 0 || (collection_insert_path(provider, &[], &value).is_some() && (provider.allows_untyped_creation(&[]) || provider.creation_template(&[], true).is_some()))));
        let count = provider.child_count(&[]);
        tree_window_indexed_section(windows, ROOT_SECTION_ID, label(labels.details)?, true, count + usize::from(controls), |index| {
            if controls && index == 0 {
                let controls = collection_controls(provider, &[], &value, ROOT_PATH_ID, labels, controller_id, windows, publication_revision)?;
                return ui_contract::tree_item(label(labels.details)?)
                    .default_open(true)
                    .try_id(ROOT_PATH_ID)
                    .map_err(|_| error("ui.snapshot-details.root-controls-id"))?
                    .try_child(controls)
                    .map_err(|_| error("ui.snapshot-details.root-controls"))?
                    .try_build()
                    .map_err(|_| error("ui.snapshot-details.root-controls"));
            }
            detail_child(provider, &[], &value, index - usize::from(controls), count, labels, controller_id, windows, publication_revision)
        })?
    } else {
        semio_framework_plugin::tree_window_section(windows, ROOT_SECTION_ID, label(labels.details)?, true, &[()], |_| detail_node(provider, &[], labels.details, None, None, labels, controller_id, windows, publication_revision))?
    };
    builder.try_child(fields).map_err(|_| error("ui.snapshot-details.fields"))?.try_build().map_err(|_| error("ui.snapshot-details.root"))
}

pub fn render_snapshot_details_provider_revisioned<P: SnapshotDetailsProvider + ?Sized>(
    provider: &P,
    publication_revision: ui_contract::UiPublicationRevision,
    locale: Locale,
    controller_id: &str,
    windows: &TreeWindows<'_>,
) -> UiAssemblyResult<BuiltNode> {
    render_snapshot_details_provider_at_revision(provider, locale, controller_id, windows, Some(publication_revision))
}

pub fn render_snapshot_details<S: ArtifactDsl + ToValue>(doc: &semio_framework_plugin::ArtifactView<'_, S>, locale: Locale, controller_id: &str, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let publication_revision = window_kit_artifact_publication_revision(doc)?;
    render_snapshot_details_provider_at_revision(&DslSnapshotDetailsProvider::new_with_schema_hint(doc.snapshot, Some(controller_id)), locale, controller_id, windows, Some(publication_revision))
}
