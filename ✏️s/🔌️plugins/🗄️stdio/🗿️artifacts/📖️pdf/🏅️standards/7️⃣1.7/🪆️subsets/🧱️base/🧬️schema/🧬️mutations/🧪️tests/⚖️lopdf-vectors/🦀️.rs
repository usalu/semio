//! ⚖️ The lopdf specification-vector laws of the resource, catalog, page and identity kinds that
//! ticket 26/09/18/PDF-ARTIFACT-SPEC-COMPLETE added without evidence. Each committed pair under
//! `../../../../🧫️fixtures/<kind>/` was written by `lopdf` 0.44 alone (`🏭️generator/🔁️codec`):
//! `⬅️before.pdf` is the arranged seed and `➡️after.pdf` is lopdf's own COS edit of it.
//!
//! Every law reads both files through this subset's reader, derives the kind's payload from what
//! lopdf changed, applies it to the decoded before-document, WRITES the result with this subset's
//! writer and reads it back, and requires the typed model to equal lopdf's after-document — so the
//! mutation, the writer and the reader are held to bytes an independent writer produced. The
//! inverse steps must then write back to the before-document. The retained COS carrier
//! (`objects`, `trailer`) is left out of both comparisons: object numbers are writer freedom.
//!
//! @see ../../../../🏭️generator/🔁️codec/🦀️.rs

use super::*;
use crate::standards::v1_7::subsets::base::io::{decode_pdf, encode_pdf};
use crate::standards::v1_7::subsets::base::schema::diff::PdfPageBox;
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::{Mutation, MutationDiff};

//#region 🔖️Harness
/// 🧬️ The typed model alone, without the retained COS carrier.
fn lanes(snapshot: &PdfSnapshot) -> PdfSnapshot {
    let mut lanes = snapshot.clone();
    lanes.objects.clear();
    lanes.trailer.clear();
    lanes
}

fn applied(base: &PdfSnapshot, mutation: &PdfMutation) -> PdfSnapshot {
    let outcome = mutation.diff(base);
    MutationDiff::apply(outcome.diff(), base).unwrap_or_else(|error| panic!("{mutation:?} must apply: {error:?}"))
}

/// 💾️ Writes the snapshot with this subset's writer and reads the bytes back.
fn written(snapshot: &PdfSnapshot) -> PdfSnapshot {
    decode_pdf(&encode_pdf(snapshot).expect("the subset's writer encodes the mutated snapshot")).expect("the subset's reader decodes the subset's own bytes")
}

/// 🪜️ One kind's edit as the step sequence that reproduces it: a single mutation, or the few a
/// kind needs when lopdf's one COS edit spans two typed collections.
trait Steps {
    fn steps(self) -> Vec<PdfMutation>;
}

impl Steps for PdfMutation {
    fn steps(self) -> Vec<PdfMutation> {
        vec![self]
    }
}

impl<const N: usize> Steps for [PdfMutation; N] {
    fn steps(self) -> Vec<PdfMutation> {
        self.into()
    }
}

/// ⚖️ The forward and inverse laws for one committed lopdf pair, reproduced by `steps` applied in
/// order; the inverse undoes them in reverse, each step's own inverse read off the state it ran on.
fn holds<S: Steps>(kind: &str, before: &[u8], after: &[u8], derive: impl Fn(&PdfSnapshot, &PdfSnapshot) -> S) {
    let base = decode_pdf(before).unwrap_or_else(|error| panic!("{kind}: lopdf's before-document must decode: {error:?}"));
    let expected = decode_pdf(after).unwrap_or_else(|error| panic!("{kind}: lopdf's after-document must decode: {error:?}"));
    assert_ne!(lanes(&base), lanes(&expected), "{kind}: the reader sees no difference between lopdf's before- and after-document");
    let steps = derive(&base, &expected).steps();
    let mut states = vec![base.clone()];
    for step in &steps {
        let current = states.last().expect("the before-document starts the chain");
        let outcome = step.diff(current);
        assert!(outcome.messages().is_empty(), "{kind}: {step:?}, derived from lopdf's edit, must apply cleanly, but raised {:?}", outcome.messages());
        states.push(applied(current, step));
    }
    let forward = states.last().expect("the chain holds the forward result").clone();
    assert_eq!(lanes(&written(&forward)), lanes(&expected), "{kind}: applying {steps:?} and writing the result does not reproduce lopdf's after-document");
    let undone = steps.iter().zip(&states).rev().fold(forward, |current, (step, prior)| step.inverse(prior).iter().fold(current, |state, undo| applied(&state, undo)));
    assert_eq!(lanes(&written(&undone)), lanes(&base), "{kind}: the inverse of {steps:?} does not write back to lopdf's before-document");
}

fn added<T: Clone + PartialEq>(before: &[T], after: &[T]) -> T {
    after.iter().find(|item| !before.contains(item)).cloned().expect("lopdf's edit adds exactly this item")
}

fn removed<T: Clone + PartialEq>(before: &[T], after: &[T]) -> T {
    before.iter().find(|item| !after.contains(item)).cloned().expect("lopdf's edit removes exactly this item")
}

/// 🖋️ The first page's operator edit as `(at, removed, inserted)`: the common prefix, the operators
/// lopdf dropped after it, and the operators it wrote in their place.
fn content_edit(before: &PdfSnapshot, after: &PdfSnapshot) -> (usize, usize, Vec<PdfOp>) {
    let (old, new) = (&before.pages[0].content, &after.pages[0].content);
    let at = old.iter().zip(new).take_while(|(left, right)| left == right).count();
    let tail = old[at..].iter().rev().zip(new[at..].iter().rev()).take_while(|(left, right)| left == right).count();
    (at, old.len() - at - tail, new[at..new.len() - tail].to_vec())
}

macro_rules! lopdf_vector {
    ($name:ident, $directory:literal, $derive:expr) => {
        #[test]
        fn $name() {
            holds($directory, include_bytes!(concat!("../../../../🧫️fixtures/", $directory, "/⬅️before.pdf")), include_bytes!(concat!("../../../../🧫️fixtures/", $directory, "/➡️after.pdf")), $derive);
        }
    };
}
//#endregion 🔖️Harness

//#region 🔖️PageKinds
lopdf_vector!(set_page_box, "🖼️set-page-box", |_, a| PdfMutation::SetPageBox(SetPageBox { index: 0, kind: PdfPageBox::Trim, rect: a.pages[0].trim_box }));
lopdf_vector!(set_page_user_unit, "📏️set-page-user-unit", |_, a| PdfMutation::SetPageUserUnit(SetPageUserUnit { index: 0, user_unit: a.pages[0].user_unit }));
lopdf_vector!(insert_content, "🖋️insert-content", |b, a| {
    let (at, _, content) = content_edit(b, a);
    PdfMutation::InsertContent(InsertContent { index: 0, at, content })
});
lopdf_vector!(remove_content, "🧻️remove-content", |b, a| {
    let (at, count, _) = content_edit(b, a);
    PdfMutation::RemoveContent(RemoveContent { index: 0, at, count })
});
lopdf_vector!(replace_content, "🔁️replace-content", |b, a| {
    let (at, count, inserted) = content_edit(b, a);
    assert_eq!((count, inserted.len()), (1, 1), "replace-content: lopdf's edit replaces exactly one operator");
    PdfMutation::ReplaceContent(ReplaceContent { index: 0, at, op: inserted[0].clone() })
});
lopdf_vector!(insert_annotation, "📌️insert-annotation", |_, a| PdfMutation::InsertAnnotation(InsertAnnotation { index: 0, at: 0, annotation: a.pages[0].annotations[0].clone() }));
lopdf_vector!(remove_annotation, "📍️remove-annotation", |_, _| PdfMutation::RemoveAnnotation(RemoveAnnotation { index: 0, at: 0 }));
lopdf_vector!(set_annotation, "📝️set-annotation", |_, a| PdfMutation::SetAnnotation(SetAnnotation { index: 0, at: 0, annotation: a.pages[0].annotations[0].clone() }));
//#endregion 🔖️PageKinds

//#region 🔖️ResourceKinds
lopdf_vector!(set_font, "🔤️set-font", |b, a| PdfMutation::SetFont(SetFont { font: added(&b.fonts, &a.fonts) }));
lopdf_vector!(remove_font, "🅾️remove-font", |b, a| PdfMutation::RemoveFont(RemoveFont { id: removed(&b.fonts, &a.fonts).id }));
lopdf_vector!(set_image, "🏞️set-image", |b, a| PdfMutation::SetImage(SetImage { image: added(&b.images, &a.images) }));
lopdf_vector!(remove_image, "🌫️remove-image", |b, a| PdfMutation::RemoveImage(RemoveImage { id: removed(&b.images, &a.images).id }));
lopdf_vector!(set_form, "📄️set-form", |b, a| PdfMutation::SetForm(SetForm { form: added(&b.forms, &a.forms) }));
lopdf_vector!(remove_form, "🗞️remove-form", |b, a| PdfMutation::RemoveForm(RemoveForm { id: removed(&b.forms, &a.forms).id }));
lopdf_vector!(set_ext_g_state, "🎛️set-ext-g-state", |b, a| PdfMutation::SetExtGState(SetExtGState { state: added(&b.ext_g_states, &a.ext_g_states) }));
lopdf_vector!(remove_ext_g_state, "🎚️remove-ext-g-state", |b, a| PdfMutation::RemoveExtGState(RemoveExtGState { id: removed(&b.ext_g_states, &a.ext_g_states).id }));
lopdf_vector!(set_shading, "🌅️set-shading", |b, a| PdfMutation::SetShading(SetShading { shading: added(&b.shadings, &a.shadings) }));
lopdf_vector!(remove_shading, "🌄️remove-shading", |b, a| PdfMutation::RemoveShading(RemoveShading { id: removed(&b.shadings, &a.shadings).id }));
lopdf_vector!(set_pattern, "🧩️set-pattern", |b: &PdfSnapshot, a: &PdfSnapshot| [PdfMutation::SetShading(SetShading { shading: added(&b.shadings, &a.shadings) }), PdfMutation::SetPattern(SetPattern { pattern: added(&b.patterns, &a.patterns) })]);
lopdf_vector!(remove_pattern, "🪡️remove-pattern", |b: &PdfSnapshot, a: &PdfSnapshot| [PdfMutation::RemovePattern(RemovePattern { id: removed(&b.patterns, &a.patterns).id }), PdfMutation::RemoveShading(RemoveShading { id: removed(&b.shadings, &a.shadings).id })]);
lopdf_vector!(set_color_space, "🌈️set-color-space", |b, a| PdfMutation::SetColorSpace(SetColorSpace { color_space: added(&b.color_spaces, &a.color_spaces) }));
lopdf_vector!(remove_color_space, "🎨️remove-color-space", |b, a| PdfMutation::RemoveColorSpace(RemoveColorSpace { name: removed(&b.color_spaces, &a.color_spaces).name }));
lopdf_vector!(set_properties, "🏷️set-properties", |b, a| PdfMutation::SetProperties(SetProperties { properties: added(&b.properties, &a.properties) }));
lopdf_vector!(remove_properties, "🔖️remove-properties", |b, a| PdfMutation::RemoveProperties(RemoveProperties { name: removed(&b.properties, &a.properties).name }));
//#endregion 🔖️ResourceKinds

//#region 🔖️CatalogKinds
lopdf_vector!(set_embedded_file, "📎️set-embedded-file", |b, a| PdfMutation::SetEmbeddedFile(SetEmbeddedFile { file: added(&b.embedded_files, &a.embedded_files) }));
lopdf_vector!(remove_embedded_file, "🗃️remove-embedded-file", |b, a| PdfMutation::RemoveEmbeddedFile(RemoveEmbeddedFile { id: removed(&b.embedded_files, &a.embedded_files).id }));
lopdf_vector!(set_outlines, "📑️set-outlines", |_, a| PdfMutation::SetOutlines(SetOutlines { outlines: a.outlines.clone() }));
lopdf_vector!(set_named_destination, "🎯️set-named-destination", |b, a| PdfMutation::SetNamedDestination(SetNamedDestination { destination: added(&b.named_destinations, &a.named_destinations) }));
lopdf_vector!(remove_named_destination, "🎪️remove-named-destination", |b, a| PdfMutation::RemoveNamedDestination(RemoveNamedDestination { name: removed(&b.named_destinations, &a.named_destinations).name }));
lopdf_vector!(set_page_labels, "🔢️set-page-labels", |_, a| PdfMutation::SetPageLabels(SetPageLabels { labels: a.page_labels.clone() }));
lopdf_vector!(set_output_intents, "🏳️set-output-intents", |_, a| PdfMutation::SetOutputIntents(SetOutputIntents { intents: a.output_intents.clone() }));
lopdf_vector!(set_acro_form, "📋️set-acro-form", |_, a| PdfMutation::SetAcroForm(SetAcroForm { form: a.acro_form.clone() }));
lopdf_vector!(set_optional_content, "👁️set-optional-content", |_, a| PdfMutation::SetOptionalContent(SetOptionalContent { content: a.optional_content.clone() }));
lopdf_vector!(set_page_layout, "📖️set-page-layout", |_, a| PdfMutation::SetPageLayout(SetPageLayout { layout: a.page_layout }));
lopdf_vector!(set_page_mode, "🖥️set-page-mode", |_, a| PdfMutation::SetPageMode(SetPageMode { mode: a.page_mode }));
lopdf_vector!(set_viewer_preferences, "🛠️set-viewer-preferences", |_, a| PdfMutation::SetViewerPreferences(SetViewerPreferences { preferences: a.viewer_preferences.clone() }));
lopdf_vector!(set_open_action, "🚪️set-open-action", |_, a| PdfMutation::SetOpenAction(SetOpenAction { action: a.open_action.clone() }));
lopdf_vector!(set_language, "🗣️set-language", |_, a| PdfMutation::SetLanguage(SetLanguage { language: a.language.clone() }));
lopdf_vector!(set_mark_info, "🔏️set-mark-info", |_, a| PdfMutation::SetMarkInfo(SetMarkInfo { info: a.mark_info.clone() }));
lopdf_vector!(set_metadata, "🧾️set-metadata", |_, a| PdfMutation::SetMetadata(SetMetadata { xmp: a.metadata.clone() }));
lopdf_vector!(set_catalog_entry, "🗂️set-catalog-entry", |b, a| {
    let entry = added(&b.catalog_extra, &a.catalog_extra);
    PdfMutation::SetCatalogEntry(SetCatalogEntry { key: entry.key, value: entry.value })
});
lopdf_vector!(remove_catalog_entry, "🧺️remove-catalog-entry", |b, a| PdfMutation::RemoveCatalogEntry(RemoveCatalogEntry { key: removed(&b.catalog_extra, &a.catalog_extra).key }));
//#endregion 🔖️CatalogKinds

//#region 🔖️IdentityKinds
lopdf_vector!(set_document_id, "🆔️set-document-id", |_, a| PdfMutation::SetDocumentId(SetDocumentId { id: a.document_id.clone() }));
lopdf_vector!(set_encryption, "🔐️set-encryption", |_, a| PdfMutation::SetEncryption(SetEncryption { encryption: a.encryption.clone() }));
//#endregion 🔖️IdentityKinds
