//! 🧬️ Transparent PDF 1.7/Any mutation dispatch. Every concrete payload, diff, inverse, codec,
//! schema, and test is owned by its direct semantic folder. Ticket 26/09/18/PDF-ARTIFACT-SPEC-
//! COMPLETE widened the vocabulary from the page/COS edits to the whole typed model: page boxes
//! and user units, content operators and annotations by index, every document collection by id
//! (fonts, images, forms, graphics states, shadings, patterns, colour spaces, property lists,
//! embedded files), outlines, named destinations, page labels, output intents, the interactive
//! form, optional content, viewer settings, metadata, identity and encryption, catalog extras —
//! with the retained COS lanes' object/dict/trailer edits kept as they were.

use crate::standards::v1_7::subsets::base::schema::{diff::PdfDiff, snapshot::{PdfPage, PdfSnapshot}};

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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
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

//#region 🔖️Net
/// 🧮️ The keyed upsert/remove script carrying `before` to `after` for a lane whose items are addressed by key: removals first,
/// then each changed survivor in place and each new item at its final position, ascending. A lane whose surviving keys changed
/// relative order is rebuilt whole, so the replayed result is always the lane `after` holds.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn keyed_script<T: PartialEq, K: PartialEq>(before: &[T], after: &[T], key: impl Fn(&T) -> K, remove: impl Fn(K) -> PdfMutation, set: impl Fn(&T, Option<usize>) -> PdfMutation, leaves: &mut Vec<PdfMutation>) {
    let kept_before: Vec<K> = before.iter().map(&key).filter(|candidate| after.iter().any(|item| key(item) == *candidate)).collect();
    let kept_after: Vec<K> = after.iter().map(&key).filter(|candidate| before.iter().any(|item| key(item) == *candidate)).collect();
    let rebuilt = kept_before != kept_after;
    for item in before {
        let name = key(item);
        if rebuilt || !after.iter().any(|candidate| key(candidate) == name) {
            leaves.push(remove(name));
        }
    }
    for (index, item) in after.iter().enumerate() {
        let name = key(item);
        match before.iter().find(|candidate| key(candidate) == name).filter(|_| !rebuilt) {
            Some(previous) if previous == item => {}
            Some(_) => leaves.push(set(item, None)),
            None => leaves.push(set(item, Some(index))),
        }
    }
}

/// 📄️ The page leaves carrying `before` to `after`: a single moved page as one move, otherwise the pages that changed in place as
/// replacements, then the surplus pages removed from the end or inserted at their final positions.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pages_script(before: &[PdfPage], after: &[PdfPage], leaves: &mut Vec<PdfMutation>) {
    let prefix = before.iter().zip(after).take_while(|(left, right)| left == right).count();
    let suffix = before[prefix..].iter().rev().zip(after[prefix..].iter().rev()).take_while(|(left, right)| left == right).count();
    let (old, new) = (&before[prefix..before.len() - suffix], &after[prefix..after.len() - suffix]);
    if old.len() == new.len() && old.len() >= 2 {
        let last = old.len() - 1;
        if old[1..] == new[..last] && old[0] == new[last] {
            leaves.push(PdfMutation::MovePage(MovePage { from: prefix, to: prefix + last }));
            return;
        }
        if old[..last] == new[1..] && old[last] == new[0] {
            leaves.push(PdfMutation::MovePage(MovePage { from: prefix + last, to: prefix }));
            return;
        }
    }
    let paired = old.len().min(new.len());
    for offset in (0..paired).filter(|offset| old[*offset] != new[*offset]) {
        leaves.push(PdfMutation::ReplacePage(ReplacePage { index: prefix + offset, page: new[offset].clone() }));
    }
    for index in (prefix + paired..prefix + old.len()).rev() {
        leaves.push(PdfMutation::RemovePage(RemovePage { index }));
    }
    for offset in paired..new.len() {
        leaves.push(PdfMutation::InsertPage(InsertPage { index: prefix + offset, page: new[offset].clone() }));
    }
}

/// 🧮️ The concrete leaves carrying `base` to `next`, one per changed lane: the retained COS graph first (a graph edit re-reads the
/// typed lanes it touches), then the keyed document collections, the page list and every whole-value lane. Replaying them through
/// the central applier is the proof the net is exact; a change in a lane no leaf addresses (the schema and version markers) is
/// left out, so the replay then refuses the edit.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn net_mutations(base: &PdfSnapshot, next: &PdfSnapshot) -> Vec<PdfMutation> {
    let mut leaves = Vec::new();
    keyed_script(&base.objects, &next.objects, |item| item.id, |id| PdfMutation::RemoveObject(RemoveObject { id }), |item, index| PdfMutation::SetObjectValue(SetObjectValue { id: item.id, value: item.value.clone(), index }), &mut leaves);
    keyed_script(&base.trailer, &next.trailer, |item| item.key.clone(), |key| PdfMutation::RemoveTrailerEntry(RemoveTrailerEntry { key }), |item, index| PdfMutation::SetTrailerEntry(SetTrailerEntry { key: item.key.clone(), value: item.value.clone(), index }), &mut leaves);
    keyed_script(&base.catalog_extra, &next.catalog_extra, |item| item.key.clone(), |key| PdfMutation::RemoveCatalogEntry(RemoveCatalogEntry { key }), |item, index| PdfMutation::SetCatalogEntry(SetCatalogEntry { key: item.key.clone(), value: item.value.clone(), index }), &mut leaves);
    keyed_script(&base.fonts, &next.fonts, |item| item.id.clone(), |id| PdfMutation::RemoveFont(RemoveFont { id }), |item, index| PdfMutation::SetFont(SetFont { font: item.clone(), index }), &mut leaves);
    keyed_script(&base.images, &next.images, |item| item.id.clone(), |id| PdfMutation::RemoveImage(RemoveImage { id }), |item, index| PdfMutation::SetImage(SetImage { image: item.clone(), index }), &mut leaves);
    keyed_script(&base.forms, &next.forms, |item| item.id.clone(), |id| PdfMutation::RemoveForm(RemoveForm { id }), |item, index| PdfMutation::SetForm(SetForm { form: item.clone(), index }), &mut leaves);
    keyed_script(&base.ext_g_states, &next.ext_g_states, |item| item.id.clone(), |id| PdfMutation::RemoveExtGState(RemoveExtGState { id }), |item, index| PdfMutation::SetExtGState(SetExtGState { state: item.clone(), index }), &mut leaves);
    keyed_script(&base.shadings, &next.shadings, |item| item.id.clone(), |id| PdfMutation::RemoveShading(RemoveShading { id }), |item, index| PdfMutation::SetShading(SetShading { shading: item.clone(), index }), &mut leaves);
    keyed_script(&base.patterns, &next.patterns, |item| item.id.clone(), |id| PdfMutation::RemovePattern(RemovePattern { id }), |item, index| PdfMutation::SetPattern(SetPattern { pattern: item.clone(), index }), &mut leaves);
    keyed_script(&base.color_spaces, &next.color_spaces, |item| item.name.clone(), |name| PdfMutation::RemoveColorSpace(RemoveColorSpace { name }), |item, index| PdfMutation::SetColorSpace(SetColorSpace { color_space: item.clone(), index }), &mut leaves);
    keyed_script(&base.properties, &next.properties, |item| item.name.clone(), |name| PdfMutation::RemoveProperties(RemoveProperties { name }), |item, index| PdfMutation::SetProperties(SetProperties { properties: item.clone(), index }), &mut leaves);
    keyed_script(&base.embedded_files, &next.embedded_files, |item| item.id.clone(), |id| PdfMutation::RemoveEmbeddedFile(RemoveEmbeddedFile { id }), |item, index| PdfMutation::SetEmbeddedFile(SetEmbeddedFile { file: item.clone(), index }), &mut leaves);
    keyed_script(&base.named_destinations, &next.named_destinations, |item| item.name.clone(), |name| PdfMutation::RemoveNamedDestination(RemoveNamedDestination { name }), |item, index| PdfMutation::SetNamedDestination(SetNamedDestination { destination: item.clone(), index }), &mut leaves);
    pages_script(&base.pages, &next.pages, &mut leaves);
    if base.info != next.info {
        leaves.push(PdfMutation::SetInfo(SetInfo { info: next.info.clone() }));
    }
    if base.outlines != next.outlines {
        leaves.push(PdfMutation::SetOutlines(SetOutlines { outlines: next.outlines.clone() }));
    }
    if base.page_labels != next.page_labels {
        leaves.push(PdfMutation::SetPageLabels(SetPageLabels { labels: next.page_labels.clone() }));
    }
    if base.output_intents != next.output_intents {
        leaves.push(PdfMutation::SetOutputIntents(SetOutputIntents { intents: next.output_intents.clone() }));
    }
    if base.acro_form != next.acro_form {
        leaves.push(PdfMutation::SetAcroForm(SetAcroForm { form: next.acro_form.clone() }));
    }
    if base.optional_content != next.optional_content {
        leaves.push(PdfMutation::SetOptionalContent(SetOptionalContent { content: next.optional_content.clone() }));
    }
    if base.page_layout != next.page_layout {
        leaves.push(PdfMutation::SetPageLayout(SetPageLayout { layout: next.page_layout.clone() }));
    }
    if base.page_mode != next.page_mode {
        leaves.push(PdfMutation::SetPageMode(SetPageMode { mode: next.page_mode.clone() }));
    }
    if base.viewer_preferences != next.viewer_preferences {
        leaves.push(PdfMutation::SetViewerPreferences(SetViewerPreferences { preferences: next.viewer_preferences.clone() }));
    }
    if base.open_action != next.open_action {
        leaves.push(PdfMutation::SetOpenAction(SetOpenAction { action: next.open_action.clone() }));
    }
    if base.language != next.language {
        leaves.push(PdfMutation::SetLanguage(SetLanguage { language: next.language.clone() }));
    }
    if base.mark_info != next.mark_info {
        leaves.push(PdfMutation::SetMarkInfo(SetMarkInfo { info: next.mark_info.clone() }));
    }
    if base.metadata != next.metadata {
        leaves.push(PdfMutation::SetMetadata(SetMetadata { xmp: next.metadata.clone() }));
    }
    if base.document_id != next.document_id {
        leaves.push(PdfMutation::SetDocumentId(SetDocumentId { id: next.document_id.clone() }));
    }
    if base.encryption != next.encryption {
        leaves.push(PdfMutation::SetEncryption(SetEncryption { encryption: next.encryption.clone() }));
    }
    leaves
}
//#endregion 🔖️Net

//#region 🔖️Delegation
/// 🛡️ Applies `outcome` to `snapshot` atomically through the central applier and converts an apply rejection into a fatal outcome.
pub fn apply_outcome(outcome: protocol::MutationOutcome<PdfDiff>, snapshot: &mut PdfSnapshot) -> protocol::MutationOutcome<PdfDiff> {
    let (diff, messages) = outcome.into_parts();
    match protocol::apply_diff(&diff, snapshot) {
        Ok(next) => {
            *snapshot = next;
            protocol::MutationOutcome::new(diff).absorb_messages(messages)
        }
        Err(error) => protocol::MutationOutcome::new(PdfDiff::default()).absorb_messages(messages).absorb_messages([protocol::MutationMessage::fatal(error.code, error.message).at(error.target)]),
    }
}

/// ▶️ Applies one mutation through its leaf-owned diff.
pub fn apply_pdf_mutation(snapshot: &mut PdfSnapshot, mutation: &PdfMutation) -> protocol::MutationOutcome<PdfDiff> {
    use protocol::Mutation;
    let outcome = mutation.diff(snapshot);
    apply_outcome(outcome, snapshot)
}

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
