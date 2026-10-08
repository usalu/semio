//! 📝️ Text representation codec surface for `stdio.semio` (diff) — grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v1::subsets::base::schema::diff::*;
use crate::standards::v1::subsets::animation::schema::{diff::SemioAnimationDiff, snapshot::SemioAnimationSnapshot};
use crate::standards::v1::subsets::audio::schema::{diff::SemioAudioDiff, snapshot::SemioAudioSnapshot};
use crate::standards::v1::subsets::base::schema::snapshot::{SemioSnapshot, SemioSubsetSnapshot};
use crate::standards::v1::subsets::brep::schema::{diff::SemioBrepDiff, snapshot::SemioBrepSnapshot};
use crate::standards::v1::subsets::cad::schema::{diff::SemioCadDiff, snapshot::SemioCadSnapshot};
use crate::standards::v1::subsets::document::schema::{diff::SemioDocumentDiff, snapshot::SemioDocumentSnapshot};
use crate::standards::v1::subsets::drawing::schema::{diff::SemioDrawingDiff, snapshot::SemioDrawingSnapshot};
use crate::standards::v1::subsets::flow::schema::{diff::SemioFlowDiff, snapshot::SemioFlowSnapshot};
use crate::standards::v1::subsets::graph::schema::{diff::SemioGraphDiff, snapshot::SemioGraphSnapshot};
use crate::standards::v1::subsets::image::schema::{diff::SemioImageDiff, snapshot::SemioImageSnapshot};
use crate::standards::v1::subsets::kit::schema::{diff::SemioKitDiff, snapshot::SemioKitSnapshot};
use crate::standards::v1::subsets::mesh::schema::{diff::SemioMeshDiff, snapshot::SemioMeshSnapshot};
use crate::standards::v1::subsets::model::schema::{diff::SemioModelDiff, snapshot::SemioModelSnapshot};
use crate::standards::v1::subsets::object::schema::{diff::SemioObjectDiff, snapshot::SemioObjectSnapshot};
use crate::standards::v1::subsets::presentation::schema::{diff::SemioPresentationDiff, snapshot::SemioPresentationSnapshot};
use crate::standards::v1::subsets::table::schema::{diff::SemioTableDiff, snapshot::SemioTableSnapshot};
use crate::standards::v1::subsets::text::schema::{diff::SemioTextDiff, snapshot::SemioTextSnapshot};
use crate::standards::v1::subsets::value::schema::{diff::SemioValueTreeDiff, snapshot::SemioValueSnapshot};
use crate::standards::v1::subsets::video::schema::{diff::SemioVideoDiff, snapshot::SemioVideoSnapshot};
use protocol::command::DiffAlgebra;
use protocol::{DiffBinary,DiffCodec,DiffText};
use protocol::MutationApplyError;
use protocol::MutationDiff;
/// 🎙️ Handcrafted `protocol::DiffCodec` — one `tag:payload` line, where `payload` for the 13
/// same-kind variants is exactly that subset's OWN already-real, already-hand-rolled
/// `print_diff()`/`parse_diff()` output (genuine reuse — this module never re-derives any of the
/// 13 subsets' own bracket/triple grammars).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_semio_diff(d: &SemioDiff) -> String {
    match d {
        SemioDiff::NoChange => "noChange".to_string(),
        SemioDiff::Rejected(error) => format!("rejected:{}", enc_rejection(error)),
        SemioDiff::Brep(d) => format!("brep:{}", d.print_diff()),
        SemioDiff::Mesh(d) => format!("mesh:{}", d.print_diff()),
        SemioDiff::Model(d) => format!("model:{}", d.print_diff()),
        SemioDiff::Value(d) => format!("value:{}", d.print_diff()),
        SemioDiff::Document(d) => format!("document:{}", d.print_diff()),
        SemioDiff::Cad(d) => format!("cad:{}", d.print_diff()),
        SemioDiff::Drawing(d) => format!("drawing:{}", d.print_diff()),
        SemioDiff::Image(d) => format!("image:{}", d.print_diff()),
        SemioDiff::Video(d) => format!("video:{}", d.print_diff()),
        SemioDiff::Audio(d) => format!("audio:{}", d.print_diff()),
        SemioDiff::Animation(d) => format!("animation:{}", d.print_diff()),
        SemioDiff::Presentation(d) => format!("presentation:{}", d.print_diff()),
        SemioDiff::Flow(d) => format!("flow:{}", d.print_diff()),
        SemioDiff::Text(d) => format!("text:{}", d.print_diff()),
        SemioDiff::Table(d) => format!("table:{}", d.print_diff()),
        SemioDiff::Graph(d) => format!("graph:{}", d.print_diff()),
        SemioDiff::Object(d) => format!("object:{}", d.print_diff()),
        SemioDiff::Kit(d) => format!("kit:{}", d.print_diff()),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_semio_diff(line: &str) -> Result<SemioDiff, String> {
    if line == "noChange" {
        return Ok(SemioDiff::NoChange);
    }
    let (tag, rest) = line.split_once(':').ok_or_else(|| format!("semio diff: missing ':' in {line:?}"))?;
    match tag {
        "rejected" => Ok(SemioDiff::Rejected(dec_rejection(rest)?)),
        "brep" => Ok(SemioDiff::Brep(SemioBrepDiff::parse_diff(rest).map_err(|e| e.to_string())?)),
        "mesh" => Ok(SemioDiff::Mesh(SemioMeshDiff::parse_diff(rest).map_err(|e| e.to_string())?)),
        "model" => Ok(SemioDiff::Model(SemioModelDiff::parse_diff(rest).map_err(|e| e.to_string())?)),
        "value" => Ok(SemioDiff::Value(SemioValueTreeDiff::parse_diff(rest).map_err(|e| e.to_string())?)),
        "document" => Ok(SemioDiff::Document(SemioDocumentDiff::parse_diff(rest).map_err(|e| e.to_string())?)),
        "cad" => Ok(SemioDiff::Cad(SemioCadDiff::parse_diff(rest).map_err(|e| e.to_string())?)),
        "drawing" => Ok(SemioDiff::Drawing(SemioDrawingDiff::parse_diff(rest).map_err(|e| e.to_string())?)),
        "image" => Ok(SemioDiff::Image(SemioImageDiff::parse_diff(rest).map_err(|e| e.to_string())?)),
        "video" => Ok(SemioDiff::Video(SemioVideoDiff::parse_diff(rest).map_err(|e| e.to_string())?)),
        "audio" => Ok(SemioDiff::Audio(SemioAudioDiff::parse_diff(rest).map_err(|e| e.to_string())?)),
        "animation" => Ok(SemioDiff::Animation(SemioAnimationDiff::parse_diff(rest).map_err(|e| e.to_string())?)),
        "presentation" => Ok(SemioDiff::Presentation(SemioPresentationDiff::parse_diff(rest).map_err(|e| e.to_string())?)),
        "flow" => Ok(SemioDiff::Flow(SemioFlowDiff::parse_diff(rest).map_err(|e| e.to_string())?)),
        "text" => Ok(SemioDiff::Text(SemioTextDiff::parse_diff(rest).map_err(|e| e.to_string())?)),
        "table" => Ok(SemioDiff::Table(SemioTableDiff::parse_diff(rest).map_err(|e| e.to_string())?)),
        "graph" => Ok(SemioDiff::Graph(SemioGraphDiff::parse_diff(rest).map_err(|e| e.to_string())?)),
        "object" => Ok(SemioDiff::Object(SemioObjectDiff::parse_diff(rest).map_err(|e| e.to_string())?)),
        "kit" => Ok(SemioDiff::Kit(SemioKitDiff::parse_diff(rest).map_err(|e| e.to_string())?)),
        other => Err(format!("semio diff: unknown tag {other:?}")),
    }
}

impl protocol::DiffText for SemioDiff {
fn print_diff(&self) -> String {
    print_semio_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_semio_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_rejection(error: &MutationApplyError) -> String {
    std::iter::once(error.code.as_str())
        .chain(std::iter::once(error.message.as_str()))
        .chain(error.target.iter().map(String::as_str))
        .map(|value| value.as_bytes().iter().map(|byte| format!("{byte:02x}")).collect::<String>())
        .collect::<Vec<_>>()
        .join(",")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_rejection(payload: &str) -> Result<MutationApplyError, String> {
    let fields = payload
        .split(',')
        .map(|hex| {
            if hex.len() % 2 != 0 {
                return Err("rejected: odd hex length".to_string());
            }
            let bytes = (0..hex.len()).step_by(2).map(|index| u8::from_str_radix(&hex[index..index + 2], 16)).collect::<Result<Vec<_>, _>>().map_err(|error| format!("rejected: invalid hex: {error}"))?;
            String::from_utf8(bytes).map_err(|error| format!("rejected: utf8 decode: {error}"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    if fields.len() < 2 {
        return Err("rejected: expected code and message".to_string());
    }
    Ok(MutationApplyError { code: fields[0].clone(), message: fields[1].clone(), target: fields[2..].to_vec() })
}
}
pub use diff_codec::*;
