//! 🔎️ Pure dictionary/resource projection consumes admitted native stream meanings.
#[path = "🎨️colour/🦀️.rs"]
mod colour;
use crate::standards::v1_7::subsets::base::schema::stream_roles::PdfStreamRoleKind;
fn dict_get<'a>(entries: &'a [PdfDictEntry], key: &str) -> Option<&'a PdfObject> { entries.iter().find(|entry| entry.key == key).map(|entry| &entry.value) }
fn dict_i64(entries: &[PdfDictEntry], key: &str) -> Option<i64> { dict_get(entries, key).and_then(PdfObject::as_i64) }
fn dict_f64(entries: &[PdfDictEntry], key: &str) -> Option<f64> { dict_get(entries, key).and_then(PdfObject::as_f64) }
fn dict_name<'a>(entries: &'a [PdfDictEntry], key: &str) -> Option<&'a str> { dict_get(entries, key).and_then(PdfObject::as_name) }
fn role_words(source: &mut dyn ObjectSource, value: &PdfObject) -> Vec<u8> { match source.semantic_role(value, PdfStreamRoleKind::AttachmentBytes) { Some(PdfStreamRoleValue::AttachmentBytes { bytes }) => bytes, _ => Vec::new() } }
fn role_text(source: &mut dyn ObjectSource, value: &PdfObject) -> String { match source.semantic_role(value, PdfStreamRoleKind::MetadataText) { Some(PdfStreamRoleValue::MetadataText { text }) => text, _ => String::new() } }
fn base_encoding_from_name(name: &str) -> Option<PdfBaseEncoding> { match name { "StandardEncoding" => Some(PdfBaseEncoding::Standard), "WinAnsiEncoding" => Some(PdfBaseEncoding::WinAnsi), "MacRomanEncoding" => Some(PdfBaseEncoding::MacRoman), "MacExpertEncoding" => Some(PdfBaseEncoding::MacExpert), _ => None } }
use colour::{array_n, extra_entries, lift_colour_space, lift_ext_g_state, lift_shading, matrix_of, numbers_of, rect_of};
#[cfg(test)] use colour::lift_function;
use crate::standards::v1_7::subsets::base::schema::graph_source::ObjectSource;
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use std::collections::{HashMap, HashSet};
use crate::standards::v1_7::subsets::base::schema::stream_roles::{PdfAdmittedStreamRole, PdfGraphIdentity, PdfGraphPath, PdfStreamRoleValue};
use crate::standards::v1_7::subsets::base::schema::content_mapping::{ResourceMap, rename_content};

/// 🔎️ Projects dictionary/resource structure exclusively through admitted semantic roles.
pub fn project_graph(snapshot: &PdfSnapshot) -> Result<PdfSnapshot, String> {
    let mut source = crate::standards::v1_7::subsets::base::schema::graph_source::GraphSource::with_roles(&snapshot.objects, &snapshot.admitted_stream_roles);
    let projected = lift_document(&snapshot.trailer, &snapshot.declared_version, &mut source);
    if let Some((identity, kind)) = source.missing_roles.first() { return Err(format!("missing admitted {kind:?} role at {} {}", identity.owner.num, identity.owner.gen)); }
    Ok(projected)
}

/// 🪢️ Carries only the semantic lanes whose logical graph projection changed.
pub fn carry_graph_edit(base: &PdfSnapshot, next: &mut PdfSnapshot) -> Result<(), String> {
    if base.objects == next.objects && base.trailer == next.trailer && base.admitted_stream_roles == next.admitted_stream_roles { return Ok(()); }
    let before = project_graph(base)?;
    let after = project_graph(next)?;
    macro_rules! carry {
        ($($lane:ident),* $(,)?) => { $(if before.$lane != after.$lane { next.$lane = after.$lane; })* };
    }
    carry!(declared_version, pages, fonts, images, forms, ext_g_states, shadings, patterns, color_spaces, properties, outlines, named_destinations, page_labels, embedded_files, output_intents, acro_form, optional_content, page_layout, page_mode, viewer_preferences, open_action, language, mark_info, metadata, document_id, info, catalog_extra);
    Ok(())
}

#[path = "🧩️lifter-and-document/🦀️.rs"]
mod lifter_and_document;
pub use lifter_and_document::*;

#[path = "🪟️resources-and-fonts/🦀️.rs"]
mod resources_and_fonts;
use resources_and_fonts::*;

#[path = "🧭️navigation-and-annotations/🦀️.rs"]
mod navigation_and_annotations;
use navigation_and_annotations::*;

#[path = "📚️catalog-and-page-mode/🦀️.rs"]
mod catalog_and_page_mode;
pub use catalog_and_page_mode::*;
