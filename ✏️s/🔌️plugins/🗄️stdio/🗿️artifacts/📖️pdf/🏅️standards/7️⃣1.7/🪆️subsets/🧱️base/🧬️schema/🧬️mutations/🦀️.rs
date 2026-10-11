//! 🧬️ Transparent PDF 1.7/Any mutation dispatch. Every concrete payload, diff, inverse, codec,
//! schema, and test is owned by its direct semantic folder. Ticket 26/09/18/PDF-ARTIFACT-SPEC-
//! COMPLETE widened the vocabulary from the page/COS edits to the whole typed model: page boxes
//! and user units, content operators and annotations by index, every document collection by id
//! (fonts, images, forms, graphics states, shadings, patterns, colour spaces, property lists,
//! embedded files), outlines, named destinations, page labels, output intents, the interactive
//! form, optional content, viewer settings, metadata, identity and encryption, catalog extras —
//! with the retained COS lanes' object/dict/trailer edits kept as they were.

use crate::standards::v1_7::subsets::base::schema::{diff::PdfDiff, snapshot::{ObjRef, PdfPage, PdfSnapshot}};
use semio_framework_value::{DslValue, FromValue, ToValue};
use semio_framework_plugin::{Fault, FaultCode, FaultOrigin};
use semio_s_artifact_stdio_contract::editing::{EditRules, EntityRule, InsertRule, RemoveRule, Selector, SnapshotEditEvent};

//#region 🔖️Leaves
#[path = "📥️insert-page/🦀️.rs"]
pub mod insert_page;
#[path = "🗑️remove-page/🦀️.rs"]
pub mod remove_page;
#[path = "📐️set-page-media-box/🦀️.rs"]
pub mod set_page_media_box;
#[path = "✂️set-page-crop-box/🦀️.rs"]
pub mod set_page_crop_box;
#[path = "➕️append-page-content/🦀️.rs"]
pub mod append_page_content;
#[path = "ℹ️set-info/🦀️.rs"]
pub mod set_info;
#[path = "📦️insert-object/🦀️.rs"]
pub mod insert_object;
#[path = "🧹️remove-object/🦀️.rs"]
pub mod remove_object;
#[path = "🔧️set-object-value/🦀️.rs"]
pub mod set_object_value;
#[path = "🔑️set-dict-entry/🦀️.rs"]
pub mod set_dict_entry;
#[path = "🚫️remove-dict-entry/🦀️.rs"]
pub mod remove_dict_entry;
#[path = "🧳️set-trailer-entry/🦀️.rs"]
pub mod set_trailer_entry;
#[path = "🧽️remove-trailer-entry/🦀️.rs"]
pub mod remove_trailer_entry;
#[path = "🔀️move-page/🦀️.rs"]
pub mod move_page;
#[path = "✏️set-page-content/🦀️.rs"]
pub mod set_page_content;
#[path = "🔄️set-page-rotation/🦀️.rs"]
pub mod set_page_rotation;
#[path = "🖼️set-page-box/🦀️.rs"]
pub mod set_page_box;
#[path = "📏️set-page-user-unit/🦀️.rs"]
pub mod set_page_user_unit;
#[path = "🖋️insert-content/🦀️.rs"]
pub mod insert_content;
#[path = "🧻️remove-content/🦀️.rs"]
pub mod remove_content;
#[path = "🔁️replace-content/🦀️.rs"]
pub mod replace_content;
#[path = "📌️insert-annotation/🦀️.rs"]
pub mod insert_annotation;
#[path = "📍️remove-annotation/🦀️.rs"]
pub mod remove_annotation;
#[path = "📝️set-annotation/🦀️.rs"]
pub mod set_annotation;
#[path = "🔤️set-font/🦀️.rs"]
pub mod set_font;
#[path = "🅾️remove-font/🦀️.rs"]
pub mod remove_font;
#[path = "🏞️set-image/🦀️.rs"]
pub mod set_image;
#[path = "🌫️remove-image/🦀️.rs"]
pub mod remove_image;
#[path = "📄️set-form/🦀️.rs"]
pub mod set_form;
#[path = "🗞️remove-form/🦀️.rs"]
pub mod remove_form;
#[path = "🎛️set-ext-g-state/🦀️.rs"]
pub mod set_ext_g_state;
#[path = "🎚️remove-ext-g-state/🦀️.rs"]
pub mod remove_ext_g_state;
#[path = "🌅️set-shading/🦀️.rs"]
pub mod set_shading;
#[path = "🌄️remove-shading/🦀️.rs"]
pub mod remove_shading;
#[path = "🧩️set-pattern/🦀️.rs"]
pub mod set_pattern;
#[path = "🪡️remove-pattern/🦀️.rs"]
pub mod remove_pattern;
#[path = "🌈️set-color-space/🦀️.rs"]
pub mod set_color_space;
#[path = "🎨️remove-color-space/🦀️.rs"]
pub mod remove_color_space;
#[path = "🏷️set-properties/🦀️.rs"]
pub mod set_properties;
#[path = "🔖️remove-properties/🦀️.rs"]
pub mod remove_properties;
#[path = "📎️set-embedded-file/🦀️.rs"]
pub mod set_embedded_file;
#[path = "🗃️remove-embedded-file/🦀️.rs"]
pub mod remove_embedded_file;
#[path = "📑️set-outlines/🦀️.rs"]
pub mod set_outlines;
#[path = "🎯️set-named-destination/🦀️.rs"]
pub mod set_named_destination;
#[path = "🎪️remove-named-destination/🦀️.rs"]
pub mod remove_named_destination;
#[path = "🔢️set-page-labels/🦀️.rs"]
pub mod set_page_labels;
#[path = "🏳️set-output-intents/🦀️.rs"]
pub mod set_output_intents;
#[path = "📋️set-acro-form/🦀️.rs"]
pub mod set_acro_form;
#[path = "👁️set-optional-content/🦀️.rs"]
pub mod set_optional_content;
#[path = "📖️set-page-layout/🦀️.rs"]
pub mod set_page_layout;
#[path = "🖥️set-page-mode/🦀️.rs"]
pub mod set_page_mode;
#[path = "🛠️set-viewer-preferences/🦀️.rs"]
pub mod set_viewer_preferences;
#[path = "🚪️set-open-action/🦀️.rs"]
pub mod set_open_action;
#[path = "🗣️set-language/🦀️.rs"]
pub mod set_language;
#[path = "🔏️set-mark-info/🦀️.rs"]
pub mod set_mark_info;
#[path = "🧾️set-metadata/🦀️.rs"]
pub mod set_metadata;
#[path = "🆔️set-document-id/🦀️.rs"]
pub mod set_document_id;
#[path = "🔐️set-encryption/🦀️.rs"]
pub mod set_encryption;
#[path = "🗂️set-catalog-entry/🦀️.rs"]
pub mod set_catalog_entry;
#[path = "🧺️remove-catalog-entry/🦀️.rs"]
pub mod remove_catalog_entry;
#[path = "🪄️replace-page/🦀️.rs"]
pub mod replace_page;

pub use insert_page::InsertPage;
pub use remove_page::RemovePage;
pub use set_page_media_box::SetPageMediaBox;
pub use set_page_crop_box::SetPageCropBox;
pub use append_page_content::AppendPageContent;
pub use set_info::SetInfo;
pub use insert_object::InsertObject;
pub use remove_object::RemoveObject;
pub use set_object_value::SetObjectValue;
pub use set_dict_entry::SetDictEntry;
pub use remove_dict_entry::RemoveDictEntry;
pub use set_trailer_entry::SetTrailerEntry;
pub use remove_trailer_entry::RemoveTrailerEntry;
pub use move_page::MovePage;
pub use set_page_content::SetPageContent;
pub use set_page_rotation::SetPageRotation;
pub use set_page_box::SetPageBox;
pub use set_page_user_unit::SetPageUserUnit;
pub use insert_content::InsertContent;
pub use remove_content::RemoveContent;
pub use replace_content::ReplaceContent;
pub use insert_annotation::InsertAnnotation;
pub use remove_annotation::RemoveAnnotation;
pub use set_annotation::SetAnnotation;
pub use set_font::SetFont;
pub use remove_font::RemoveFont;
pub use set_image::SetImage;
pub use remove_image::RemoveImage;
pub use set_form::SetForm;
pub use remove_form::RemoveForm;
pub use set_ext_g_state::SetExtGState;
pub use remove_ext_g_state::RemoveExtGState;
pub use set_shading::SetShading;
pub use remove_shading::RemoveShading;
pub use set_pattern::SetPattern;
pub use remove_pattern::RemovePattern;
pub use set_color_space::SetColorSpace;
pub use remove_color_space::RemoveColorSpace;
pub use set_properties::SetProperties;
pub use remove_properties::RemoveProperties;
pub use set_embedded_file::SetEmbeddedFile;
pub use remove_embedded_file::RemoveEmbeddedFile;
pub use set_outlines::SetOutlines;
pub use set_named_destination::SetNamedDestination;
pub use remove_named_destination::RemoveNamedDestination;
pub use set_page_labels::SetPageLabels;
pub use set_output_intents::SetOutputIntents;
pub use set_acro_form::SetAcroForm;
pub use set_optional_content::SetOptionalContent;
pub use set_page_layout::SetPageLayout;
pub use set_page_mode::SetPageMode;
pub use set_viewer_preferences::SetViewerPreferences;
pub use set_open_action::SetOpenAction;
pub use set_language::SetLanguage;
pub use set_mark_info::SetMarkInfo;
pub use set_metadata::SetMetadata;
pub use set_document_id::SetDocumentId;
pub use set_encryption::SetEncryption;
pub use set_catalog_entry::SetCatalogEntry;
pub use remove_catalog_entry::RemoveCatalogEntry;
pub use replace_page::ReplacePage;
//#endregion 🔖️Leaves

//#region 🔖️Aggregate
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = PdfSnapshot, diff = PdfDiff, schema = "s.stdio.pdf.1.7")]
pub enum PdfMutation {
    InsertPage(InsertPage),
    RemovePage(RemovePage),
    SetPageMediaBox(SetPageMediaBox),
    SetPageCropBox(SetPageCropBox),
    AppendPageContent(AppendPageContent),
    SetInfo(SetInfo),
    InsertObject(InsertObject),
    RemoveObject(RemoveObject),
    SetObjectValue(SetObjectValue),
    SetDictEntry(SetDictEntry),
    RemoveDictEntry(RemoveDictEntry),
    SetTrailerEntry(SetTrailerEntry),
    RemoveTrailerEntry(RemoveTrailerEntry),
    MovePage(MovePage),
    SetPageContent(SetPageContent),
    SetPageRotation(SetPageRotation),
    SetPageBox(SetPageBox),
    SetPageUserUnit(SetPageUserUnit),
    InsertContent(InsertContent),
    RemoveContent(RemoveContent),
    ReplaceContent(ReplaceContent),
    InsertAnnotation(InsertAnnotation),
    RemoveAnnotation(RemoveAnnotation),
    SetAnnotation(SetAnnotation),
    SetFont(SetFont),
    RemoveFont(RemoveFont),
    SetImage(SetImage),
    RemoveImage(RemoveImage),
    SetForm(SetForm),
    RemoveForm(RemoveForm),
    SetExtGState(SetExtGState),
    RemoveExtGState(RemoveExtGState),
    SetShading(SetShading),
    RemoveShading(RemoveShading),
    SetPattern(SetPattern),
    RemovePattern(RemovePattern),
    SetColorSpace(SetColorSpace),
    RemoveColorSpace(RemoveColorSpace),
    SetProperties(SetProperties),
    RemoveProperties(RemoveProperties),
    SetEmbeddedFile(SetEmbeddedFile),
    RemoveEmbeddedFile(RemoveEmbeddedFile),
    SetOutlines(SetOutlines),
    SetNamedDestination(SetNamedDestination),
    RemoveNamedDestination(RemoveNamedDestination),
    SetPageLabels(SetPageLabels),
    SetOutputIntents(SetOutputIntents),
    SetAcroForm(SetAcroForm),
    SetOptionalContent(SetOptionalContent),
    SetPageLayout(SetPageLayout),
    SetPageMode(SetPageMode),
    SetViewerPreferences(SetViewerPreferences),
    SetOpenAction(SetOpenAction),
    SetLanguage(SetLanguage),
    SetMarkInfo(SetMarkInfo),
    SetMetadata(SetMetadata),
    SetDocumentId(SetDocumentId),
    SetEncryption(SetEncryption),
    SetCatalogEntry(SetCatalogEntry),
    RemoveCatalogEntry(RemoveCatalogEntry),
    ReplacePage(ReplacePage),
}
//#endregion 🔖️Aggregate

//#region 🔖️Edit
/// ✋️ One path-scoped edit applied to a sub-value of a lane.
enum Edit {
    Set(DslValue),
    Insert(DslValue),
    Remove,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn refused(code: &'static str, message: impl Into<String>) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new(code), message)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pointer_segments(path: &str) -> Result<Vec<String>, Fault> {
    if path.is_empty() {
        return Ok(Vec::new());
    }
    let Some(rest) = path.strip_prefix('/') else { return Err(refused("pdf-edit.invalid-pointer", format!("'{path}' is not an RFC 6901 pointer"))) };
    Ok(rest.split('/').map(|raw| raw.replace("~1", "/").replace("~0", "~")).collect())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn position(segment: &str, length: usize, insert: bool) -> Result<usize, Fault> {
    if insert && segment == "-" {
        return Ok(length);
    }
    match segment.parse::<usize>() {
        Ok(index) if index < length || (insert && index == length) => Ok(index),
        _ => Err(refused("pdf-edit.invalid-index", format!("'{segment}' addresses no position of a list of {length}"))),
    }
}

/// ✋️ Applies `edit` at `rest` below `value`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn edit_at(value: &mut DslValue, rest: &[String], edit: Edit) -> Result<(), Fault> {
    let Some((first, tail)) = rest.split_first() else {
        return match edit {
            Edit::Set(next) => {
                *value = next;
                Ok(())
            }
            Edit::Insert(_) | Edit::Remove => Err(refused("pdf-edit.root-operation", "insert and remove address a member, not the value itself")),
        };
    };
    match value {
        DslValue::Object(entries) => {
            let found = entries.iter().position(|(key, _)| key == first);
            match (tail.is_empty(), edit, found) {
                (false, edit, Some(at)) => edit_at(&mut entries[at].1, tail, edit),
                (true, Edit::Set(next), Some(at)) => {
                    entries[at].1 = next;
                    Ok(())
                }
                (true, Edit::Insert(next) | Edit::Set(next), None) => {
                    entries.push((first.clone(), next));
                    Ok(())
                }
                (true, Edit::Remove, Some(at)) => {
                    entries.remove(at);
                    Ok(())
                }
                _ => Err(refused("pdf-edit.path-missing", format!("object key '{first}' cannot take this edit"))),
            }
        }
        DslValue::Array(items) => {
            if !tail.is_empty() {
                let at = position(first, items.len(), false)?;
                return edit_at(&mut items[at], tail, edit);
            }
            match edit {
                Edit::Set(next) => {
                    let at = position(first, items.len(), false)?;
                    items[at] = next;
                }
                Edit::Insert(next) => {
                    let at = position(first, items.len(), true)?;
                    items.insert(at, next);
                }
                Edit::Remove => {
                    let at = position(first, items.len(), false)?;
                    items.remove(at);
                }
            }
            Ok(())
        }
        _ => Err(refused("pdf-edit.not-container", format!("path segment '{first}' has a scalar parent"))),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode<T: FromValue>(value: DslValue) -> Result<T, Fault> {
    T::from_value(value).map_err(|error| refused("pdf-edit.schema-invalid", error.to_string()))
}

/// 🎯️ A whole-value lane (`/language`, `/info/title`, …) edited below `rest`: its own `set-*` kind carries the edited value.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn whole_lane<T: ToValue + FromValue + PartialEq>(current: &T, rest: &[String], edit: Edit, make: impl FnOnce(T) -> PdfMutation) -> Result<Vec<PdfMutation>, Fault> {
    let mut value = current.to_value();
    match (rest.is_empty(), edit) {
        (true, Edit::Remove) => value = DslValue::Null,
        (true, Edit::Insert(next) | Edit::Set(next)) => value = next,
        (_, edit) => edit_at(&mut value, rest, edit)?,
    }
    let next: T = decode(value)?;
    Ok((next != *current).then(|| make(next)).into_iter().collect())
}

/// 🎯️ A list lane edited at `/<index>[/…]`: the touched item's own `set-*` / `remove-*` kind, never the whole lane.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn item_lane<T: ToValue + FromValue + PartialEq, K: PartialEq>(items: &[T], rest: &[String], edit: Edit, key: impl Fn(&T) -> K, remove: impl Fn(K) -> PdfMutation, set: impl Fn(T, Option<usize>) -> PdfMutation) -> Result<Vec<PdfMutation>, Fault> {
    let Some((first, tail)) = rest.split_first() else { return Err(refused("pdf-edit.lane-operation", "edit the items of this list one by one")) };
    if tail.is_empty() {
        return match edit {
            Edit::Remove => {
                let at = position(first, items.len(), false)?;
                Ok(vec![remove(key(&items[at]))])
            }
            Edit::Insert(value) => {
                let at = position(first, items.len(), true)?;
                Ok(vec![set(decode(value)?, Some(at))])
            }
            Edit::Set(value) => replace_item(items, position(first, items.len(), false)?, decode(value)?, key, remove, set),
        };
    }
    let at = position(first, items.len(), false)?;
    let mut value = items[at].to_value();
    edit_at(&mut value, tail, edit)?;
    replace_item(items, at, decode(value)?, key, remove, set)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn replace_item<T: PartialEq, K: PartialEq>(items: &[T], at: usize, next: T, key: impl Fn(&T) -> K, remove: impl Fn(K) -> PdfMutation, set: impl Fn(T, Option<usize>) -> PdfMutation) -> Result<Vec<PdfMutation>, Fault> {
    if items[at] == next {
        return Ok(Vec::new());
    }
    if key(&items[at]) == key(&next) {
        return Ok(vec![set(next, None)]);
    }
    Ok(vec![remove(key(&items[at])), set(next, Some(at))])
}

/// 🧭️ The details-pane edit table: which JSON-pointer edit raises which ONE concrete kind. A set of an entity, or any edit inside it,
/// raises the entity's kind carrying its whole new value; an insert into a list raises the list's set kind at the position; a remove
/// raises the list's remove kind addressing the row by its key (pages by position). Whole-value lanes edited as a whole, the retained
/// COS lanes (`objects`, `trailer`, `catalogExtra`, whose kinds take the row's fields apart) and page moves are answered by [`special_edit`].
pub static EDIT_RULES: EditRules = EditRules {
    entities: &[
        EntityRule::new("/info", "set-info", "info"),
        EntityRule::new("/outlines", "set-outlines", "outlines"),
        EntityRule::new("/pageLabels", "set-page-labels", "labels"),
        EntityRule::new("/outputIntents", "set-output-intents", "intents"),
        EntityRule::new("/acroForm", "set-acro-form", "form"),
        EntityRule::new("/optionalContent", "set-optional-content", "content"),
        EntityRule::new("/pageLayout", "set-page-layout", "layout"),
        EntityRule::new("/pageMode", "set-page-mode", "mode"),
        EntityRule::new("/viewerPreferences", "set-viewer-preferences", "preferences"),
        EntityRule::new("/openAction", "set-open-action", "action"),
        EntityRule::new("/language", "set-language", "language"),
        EntityRule::new("/markInfo", "set-mark-info", "info"),
        EntityRule::new("/metadata", "set-metadata", "xmp"),
        EntityRule::new("/documentId", "set-document-id", "id"),
        EntityRule::new("/encryption", "set-encryption", "encryption"),
        EntityRule::new("/pages/*", "replace-page", "page").selecting(&[Selector::Index("index")]),
        EntityRule::new("/fonts/*", "set-font", "font"),
        EntityRule::new("/images/*", "set-image", "image"),
        EntityRule::new("/forms/*", "set-form", "form"),
        EntityRule::new("/extGStates/*", "set-ext-g-state", "state"),
        EntityRule::new("/shadings/*", "set-shading", "shading"),
        EntityRule::new("/patterns/*", "set-pattern", "pattern"),
        EntityRule::new("/colorSpaces/*", "set-color-space", "colorSpace"),
        EntityRule::new("/properties/*", "set-properties", "properties"),
        EntityRule::new("/embeddedFiles/*", "set-embedded-file", "file"),
        EntityRule::new("/namedDestinations/*", "set-named-destination", "destination"),
    ],
    inserts: &[
        InsertRule::new("/pages", "insert-page", "page").at("index"),
        InsertRule::new("/fonts", "set-font", "font").at("index"),
        InsertRule::new("/images", "set-image", "image").at("index"),
        InsertRule::new("/forms", "set-form", "form").at("index"),
        InsertRule::new("/extGStates", "set-ext-g-state", "state").at("index"),
        InsertRule::new("/shadings", "set-shading", "shading").at("index"),
        InsertRule::new("/patterns", "set-pattern", "pattern").at("index"),
        InsertRule::new("/colorSpaces", "set-color-space", "colorSpace").at("index"),
        InsertRule::new("/properties", "set-properties", "properties").at("index"),
        InsertRule::new("/embeddedFiles", "set-embedded-file", "file").at("index"),
        InsertRule::new("/namedDestinations", "set-named-destination", "destination").at("index"),
    ],
    removes: &[
        RemoveRule::by_index("/pages", "remove-page", "index"),
        RemoveRule::by_key("/fonts", "remove-font", "id", "id"),
        RemoveRule::by_key("/images", "remove-image", "id", "id"),
        RemoveRule::by_key("/forms", "remove-form", "id", "id"),
        RemoveRule::by_key("/extGStates", "remove-ext-g-state", "id", "id"),
        RemoveRule::by_key("/shadings", "remove-shading", "id", "id"),
        RemoveRule::by_key("/patterns", "remove-pattern", "id", "id"),
        RemoveRule::by_key("/colorSpaces", "remove-color-space", "name", "name"),
        RemoveRule::by_key("/properties", "remove-properties", "name", "name"),
        RemoveRule::by_key("/embeddedFiles", "remove-embedded-file", "id", "id"),
        RemoveRule::by_key("/namedDestinations", "remove-named-destination", "name", "name"),
    ],
};

/// 🏷️ A keyed document lane edited at `/<index>/<key field>` or as a whole row: an item whose key changed keeps its position under the new key,
/// so the edit is the removal of the old key plus the set of the renamed item at the same position; an unchanged key is one set.
macro_rules! rename_in_place {
    ($items:expr, $rest:expr, $edit:expr, $key:ident, $remove:ident, $set:ident, $payload:ident) => {
        item_lane($items, $rest, $edit, |item| item.$key.clone(), |key| PdfMutation::$remove($remove { $key: key }), |item, index| PdfMutation::$set($set { $payload: item, index }))
    };
}

/// 🎯️ The edits the table cannot express: a whole-value lane set, inserted or removed as a whole (an absent optional lane has no
/// pointer for the table to read), the retained COS lanes whose kinds take the row apart, and a page moved to another position.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn special_edit(event: &SnapshotEditEvent, snapshot: &PdfSnapshot) -> Result<Option<Vec<PdfMutation>>, Fault> {
    let (path, edit) = match event {
        SnapshotEditEvent::SetValue { path, value } => (path, Edit::Set(value.clone())),
        SnapshotEditEvent::InsertValue { path, value } => (path, Edit::Insert(value.clone())),
        SnapshotEditEvent::RemoveValue { path } => (path, Edit::Remove),
        SnapshotEditEvent::MoveValue { from, path } => return move_page(from, path, snapshot),
        SnapshotEditEvent::RenameKey { .. } | SnapshotEditEvent::ReplaceSource { .. } => return Ok(None),
    };
    let segments = pointer_segments(path)?;
    let Some((lane, rest)) = segments.split_first() else { return Ok(None) };
    let lane_level = rest.is_empty();
    let leaves = match lane.as_str() {
        "info" if lane_level => whole_lane(&snapshot.info, rest, edit, |info| PdfMutation::SetInfo(SetInfo { info })),
        "outlines" if lane_level => whole_lane(&snapshot.outlines, rest, edit, |outlines| PdfMutation::SetOutlines(SetOutlines { outlines })),
        "pageLabels" if lane_level => whole_lane(&snapshot.page_labels, rest, edit, |labels| PdfMutation::SetPageLabels(SetPageLabels { labels })),
        "outputIntents" if lane_level => whole_lane(&snapshot.output_intents, rest, edit, |intents| PdfMutation::SetOutputIntents(SetOutputIntents { intents })),
        "acroForm" if lane_level => whole_lane(&snapshot.acro_form, rest, edit, |form| PdfMutation::SetAcroForm(SetAcroForm { form })),
        "optionalContent" if lane_level => whole_lane(&snapshot.optional_content, rest, edit, |content| PdfMutation::SetOptionalContent(SetOptionalContent { content })),
        "pageLayout" if lane_level => whole_lane(&snapshot.page_layout, rest, edit, |layout| PdfMutation::SetPageLayout(SetPageLayout { layout })),
        "pageMode" if lane_level => whole_lane(&snapshot.page_mode, rest, edit, |mode| PdfMutation::SetPageMode(SetPageMode { mode })),
        "viewerPreferences" if lane_level => whole_lane(&snapshot.viewer_preferences, rest, edit, |preferences| PdfMutation::SetViewerPreferences(SetViewerPreferences { preferences })),
        "openAction" if lane_level => whole_lane(&snapshot.open_action, rest, edit, |action| PdfMutation::SetOpenAction(SetOpenAction { action })),
        "language" if lane_level => whole_lane(&snapshot.language, rest, edit, |language| PdfMutation::SetLanguage(SetLanguage { language })),
        "markInfo" if lane_level => whole_lane(&snapshot.mark_info, rest, edit, |info| PdfMutation::SetMarkInfo(SetMarkInfo { info })),
        "metadata" if lane_level => whole_lane(&snapshot.metadata, rest, edit, |xmp| PdfMutation::SetMetadata(SetMetadata { xmp })),
        "documentId" if lane_level => whole_lane(&snapshot.document_id, rest, edit, |id| PdfMutation::SetDocumentId(SetDocumentId { id })),
        "encryption" if lane_level => whole_lane(&snapshot.encryption, rest, edit, |encryption| PdfMutation::SetEncryption(SetEncryption { encryption })),
        "objects" => item_lane(&snapshot.objects, rest, edit, |item| item.id, |id| PdfMutation::RemoveObject(RemoveObject { id, admitted_stream_roles: None }), |item, index| PdfMutation::SetObjectValue(SetObjectValue { id: item.id, value: item.value, index, admitted_stream_roles: None })),
        "trailer" => item_lane(&snapshot.trailer, rest, edit, |item| item.key.clone(), |key| PdfMutation::RemoveTrailerEntry(RemoveTrailerEntry { key }), |item, index| PdfMutation::SetTrailerEntry(SetTrailerEntry { key: item.key, value: item.value, index })),
        "catalogExtra" => item_lane(&snapshot.catalog_extra, rest, edit, |item| item.key.clone(), |key| PdfMutation::RemoveCatalogEntry(RemoveCatalogEntry { key }), |item, index| PdfMutation::SetCatalogEntry(SetCatalogEntry { key: item.key, value: item.value, index })),
        "fonts" if renames(rest, &edit, "id") => rename_in_place!(&snapshot.fonts, rest, edit, id, RemoveFont, SetFont, font),
        "images" if renames(rest, &edit, "id") => rename_in_place!(&snapshot.images, rest, edit, id, RemoveImage, SetImage, image),
        "forms" if renames(rest, &edit, "id") => rename_in_place!(&snapshot.forms, rest, edit, id, RemoveForm, SetForm, form),
        "extGStates" if renames(rest, &edit, "id") => rename_in_place!(&snapshot.ext_g_states, rest, edit, id, RemoveExtGState, SetExtGState, state),
        "shadings" if renames(rest, &edit, "id") => rename_in_place!(&snapshot.shadings, rest, edit, id, RemoveShading, SetShading, shading),
        "patterns" if renames(rest, &edit, "id") => rename_in_place!(&snapshot.patterns, rest, edit, id, RemovePattern, SetPattern, pattern),
        "embeddedFiles" if renames(rest, &edit, "id") => rename_in_place!(&snapshot.embedded_files, rest, edit, id, RemoveEmbeddedFile, SetEmbeddedFile, file),
        "colorSpaces" if renames(rest, &edit, "name") => rename_in_place!(&snapshot.color_spaces, rest, edit, name, RemoveColorSpace, SetColorSpace, color_space),
        "properties" if renames(rest, &edit, "name") => rename_in_place!(&snapshot.properties, rest, edit, name, RemoveProperties, SetProperties, properties),
        "namedDestinations" if renames(rest, &edit, "name") => rename_in_place!(&snapshot.named_destinations, rest, edit, name, RemoveNamedDestination, SetNamedDestination, destination),
        _ => return Ok(None),
    }?;
    Ok(Some(leaves))
}

/// 🏷️ Whether the edit can change the key of a list row: the key `field` itself (`<index>/<field>`) or the whole row (`<index>`) set anew.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn renames(rest: &[String], edit: &Edit, field: &str) -> bool {
    matches!(rest, [_, name] if name == field) || (rest.len() == 1 && matches!(edit, Edit::Set(_)))
}

/// 🔀️ A page dragged from `/pages/<from>` to `/pages/<to>`; any other move is the table's.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn move_page(from: &str, to: &str, snapshot: &PdfSnapshot) -> Result<Option<Vec<PdfMutation>>, Fault> {
    match (pointer_segments(from)?.as_slice(), pointer_segments(to)?.as_slice()) {
        ([lane_from, source], [lane_to, destination]) if lane_from == "pages" && lane_to == "pages" => {
            let (from, to) = (position(source, snapshot.pages.len(), false)?, position(destination, snapshot.pages.len(), true)?.min(snapshot.pages.len().saturating_sub(1)));
            Ok(Some((from != to).then(|| PdfMutation::MovePage(MovePage { from, to })).into_iter().collect()))
        }
        _ => Ok(None),
    }
}
//#endregion 🔖️Edit

//#region 🔖️Delegation
/// 🧾️ Returns the derive-owned identity table in declaration and binary-tag order.
pub fn pdf_mutation_kinds() -> &'static [protocol::SemanticDescriptor] {
    use protocol::SemanticMutation;
    PdfMutation::kinds()
}

//#endregion 🔖️Delegation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/⚖️lopdf-vectors/🦀️.rs"]
mod tests_lopdf_vectors;
//#endregion 🧪️Tests
