//! ⚡️ Semio kit artifact — hand-rolled `OpText` for `SemioKitMutation`. `#[derive(dsl::Mutations)]`
//! only generates `Mutation`/`SemanticMutation` — the wire-text codec stays handcrafted here, one
//! keyword per semantic verb, grammar `keyword:arg1,arg2,...`. Reuses the snapshot facet's own
//! real hex/bracket encoders for children/links/types/pieces/connections (never re-derived).

use crate::standards::v1::subsets::kit::schema::mutations::SemioKitMutation;

use crate::standards::v1::subsets::base::io::text::snapshot::{split_top_level};
use crate::standards::v1::subsets::kit::schema::mutations::{
    add_design::AddDesign, add_type::AddType, bind_representation::BindRepresentation, change_representation_pin::ChangeRepresentationPin, create_model::CreateModel, create_object::CreateObject, create_properties::CreateProperties,
    delete_model::DeleteModel, delete_object::DeleteObject, delete_properties::DeleteProperties, edit_design::EditDesign, remove_design::RemoveDesign, remove_type::RemoveType, rename_type::RenameType, unbind_representation::UnbindRepresentation,
};
use crate::standards::v1::subsets::kit::io::text::snapshot::{dec_connection};
use crate::standards::v1::subsets::kit::io::text::snapshot::{enc_connection};
use crate::standards::v1::subsets::kit::io::text::snapshot::{dec_piece};
use crate::standards::v1::subsets::kit::io::text::snapshot::{enc_piece};
use crate::standards::v1::subsets::kit::io::text::snapshot::{dec_pin};
use crate::standards::v1::subsets::kit::io::text::snapshot::{enc_pin};
use crate::standards::v1::subsets::kit::io::text::snapshot::{dec_ref};
use crate::standards::v1::subsets::kit::io::text::snapshot::{enc_ref};
use crate::standards::v1::subsets::kit::io::text::snapshot::{dec_str};
use crate::standards::v1::subsets::kit::io::text::snapshot::{enc_str};

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️Primitives
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_pieces(pieces: &[crate::standards::v1::subsets::kit::schema::snapshot::SemioKitPiece]) -> String {
    format!("[{}]", pieces.iter().map(enc_piece).collect::<Vec<_>>().join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_pieces(s: &str) -> Result<Vec<crate::standards::v1::subsets::kit::schema::snapshot::SemioKitPiece>, String> {
    let inner = s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("pieces: expected brackets, got {s:?}"))?;
    split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_piece).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_connections(cs: &[crate::standards::v1::subsets::kit::schema::snapshot::SemioKitConnection]) -> String {
    format!("[{}]", cs.iter().map(enc_connection).collect::<Vec<_>>().join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_connections(s: &str) -> Result<Vec<crate::standards::v1::subsets::kit::schema::snapshot::SemioKitConnection>, String> {
    let inner = s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("connections: expected brackets, got {s:?}"))?;
    split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_connection).collect()
}
//#endregion 🔖️Primitives

//#region 🔖️OpText
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn hex_encode(bytes: &[u8]) -> String { bytes.iter().map(|byte| format!("{byte:02x}")).collect() }
fn hex_decode(value: &str) -> Result<Vec<u8>, String> {
    if !value.len().is_multiple_of(2) { return Err("snapshot payload has odd hexadecimal length".into()); }
    (0..value.len()).step_by(2).map(|index| u8::from_str_radix(&value[index..index + 2], 16).map_err(|error| error.to_string())).collect()
}

fn enc_at(at: Option<usize>) -> String {
    at.map(|at| format!(",{at}")).unwrap_or_default()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_at(parts: &[&str], required: usize, tag: &str) -> Result<Option<usize>, String> {
    match parts.len() {
        n if n == required => Ok(None),
        n if n == required + 1 => parse_usize(parts[required]).map(Some),
        n => Err(format!("{tag}: expected {required} or {} fields, got {n}", required + 1)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_kit_mutation(m: &SemioKitMutation) -> String {
    match m {
        SemioKitMutation::CreateObject(p) => format!("createObject:{},{}{}", enc_str(&p.child_id), enc_ref(&p.target), enc_at(p.at)),
        SemioKitMutation::DeleteObject(p) => format!("deleteObject:{}", enc_str(&p.child_id)),
        SemioKitMutation::CreateModel(p) => format!("createModel:{},{}{}", enc_str(&p.child_id), enc_ref(&p.target), enc_at(p.at)),
        SemioKitMutation::DeleteModel(p) => format!("deleteModel:{}", enc_str(&p.child_id)),
        SemioKitMutation::CreateProperties(p) => format!("createProperties:{},{}", enc_str(&p.child_id), enc_ref(&p.target)),
        SemioKitMutation::DeleteProperties(_) => "deleteProperties".to_string(),
        SemioKitMutation::BindRepresentation(p) => format!("bindRepresentation:{},{},{}{}", enc_ref(&p.target), enc_pin(&p.pin), enc_str(&p.role), enc_at(p.at)),
        SemioKitMutation::UnbindRepresentation(p) => format!("unbindRepresentation:{}", p.index),
        SemioKitMutation::ChangeRepresentationPin(p) => format!("changeRepresentationPin:{},{}", p.index, enc_pin(&p.pin)),
        SemioKitMutation::AddType(p) => format!("addType:{},{},{}{}", enc_str(&p.id), enc_str(&p.name), enc_str(&p.category), enc_at(p.at)),
        SemioKitMutation::RemoveType(p) => format!("removeType:{}", enc_str(&p.id)),
        SemioKitMutation::RenameType(p) => format!("renameType:{},{}", enc_str(&p.id), enc_str(&p.new_name)),
        SemioKitMutation::AddDesign(p) => format!("addDesign:{},{}{}", enc_str(&p.id), enc_str(&p.name), enc_at(p.at)),
        SemioKitMutation::RemoveDesign(p) => format!("removeDesign:{}", enc_str(&p.id)),
        SemioKitMutation::EditDesign(p) => format!("editDesign:{},{},{}", enc_str(&p.id), enc_pieces(&p.pieces), enc_connections(&p.connections)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_kit_mutation(line: &str) -> Result<SemioKitMutation, String> {
    if line == "deleteProperties" {
        return Ok(SemioKitMutation::DeleteProperties(DeleteProperties {}));
    }
    let (tag, rest) = line.split_once(':').ok_or_else(|| format!("kit mutation: missing ':' in {line:?}"))?;
    match tag {
        "createObject" => {
            let parts = split_top_level(rest, ',');
            let at = dec_at(&parts, 2, "createObject")?;
            Ok(SemioKitMutation::CreateObject(CreateObject { child_id: dec_str(parts[0])?, target: dec_ref(parts[1])?, at }))
        }
        "deleteObject" => Ok(SemioKitMutation::DeleteObject(DeleteObject { child_id: dec_str(rest)? })),
        "createModel" => {
            let parts = split_top_level(rest, ',');
            let at = dec_at(&parts, 2, "createModel")?;
            Ok(SemioKitMutation::CreateModel(CreateModel { child_id: dec_str(parts[0])?, target: dec_ref(parts[1])?, at }))
        }
        "deleteModel" => Ok(SemioKitMutation::DeleteModel(DeleteModel { child_id: dec_str(rest)? })),
        "createProperties" => {
            let (child_id, target) = rest.split_once(',').ok_or_else(|| "createProperties: missing comma".to_string())?;
            Ok(SemioKitMutation::CreateProperties(CreateProperties { child_id: dec_str(child_id)?, target: dec_ref(target)? }))
        }
        "bindRepresentation" => {
            let parts = split_top_level(rest, ',');
            let at = dec_at(&parts, 3, "bindRepresentation")?;
            Ok(SemioKitMutation::BindRepresentation(BindRepresentation { target: dec_ref(parts[0])?, pin: dec_pin(parts[1])?, role: dec_str(parts[2])?, at }))
        }
        "unbindRepresentation" => Ok(SemioKitMutation::UnbindRepresentation(UnbindRepresentation { index: parse_usize(rest)? })),
        "changeRepresentationPin" => {
            let (index, pin) = rest.split_once(',').ok_or_else(|| "changeRepresentationPin: missing comma".to_string())?;
            Ok(SemioKitMutation::ChangeRepresentationPin(ChangeRepresentationPin { index: parse_usize(index)?, pin: dec_pin(pin)? }))
        }
        "addType" => {
            let parts = split_top_level(rest, ',');
            let at = dec_at(&parts, 3, "addType")?;
            Ok(SemioKitMutation::AddType(AddType { id: dec_str(parts[0])?, name: dec_str(parts[1])?, category: dec_str(parts[2])?, at }))
        }
        "removeType" => Ok(SemioKitMutation::RemoveType(RemoveType { id: dec_str(rest)? })),
        "renameType" => {
            let (id, new_name) = rest.split_once(',').ok_or_else(|| "renameType: missing comma".to_string())?;
            Ok(SemioKitMutation::RenameType(RenameType { id: dec_str(id)?, new_name: dec_str(new_name)? }))
        }
        "addDesign" => {
            let parts = split_top_level(rest, ',');
            let at = dec_at(&parts, 2, "addDesign")?;
            Ok(SemioKitMutation::AddDesign(AddDesign { id: dec_str(parts[0])?, name: dec_str(parts[1])?, at }))
        }
        "removeDesign" => Ok(SemioKitMutation::RemoveDesign(RemoveDesign { id: dec_str(rest)? })),
        "editDesign" => {
            let parts = split_top_level(rest, ',');
            let [id, pieces, connections] = parts.as_slice() else { return Err(format!("editDesign: expected 3 fields, got {}", parts.len())) };
            Ok(SemioKitMutation::EditDesign(EditDesign { id: dec_str(id)?, pieces: dec_pieces(pieces)?, connections: dec_connections(connections)? }))
        }
        other => Err(format!("kit mutation: unknown keyword {other:?}")),
    }
}

impl protocol::OpText for SemioKitMutation {
    fn print_op(&self) -> String {
        print_kit_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        parse_kit_mutation(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}
//#endregion 🔖️OpText

//#region 🔖️DemoCases
/// 🌱 One representative value per variant — single source of truth for
/// `ops_grammar_conformance_law`/`protocol_walk_law` in `🚪️io/🦀️.rs`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<SemioKitMutation> {
    let ref_of = |subset: &str, id: &str| semio_framework_artifact_reference::ArtifactRef { artifact_id: id.into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: subset.into() } };
    vec![
        SemioKitMutation::CreateObject(CreateObject { child_id: "o1".into(), target: ref_of("object", "t1"), at: None }),
        SemioKitMutation::DeleteObject(DeleteObject { child_id: "o1".into() }),
        SemioKitMutation::CreateModel(CreateModel { child_id: "m1".into(), target: ref_of("model", "t2"), at: None }),
        SemioKitMutation::DeleteModel(DeleteModel { child_id: "m1".into() }),
        SemioKitMutation::CreateProperties(CreateProperties { child_id: "p1".into(), target: ref_of("value", "t3") }),
        SemioKitMutation::DeleteProperties(DeleteProperties {}),
        SemioKitMutation::BindRepresentation(BindRepresentation { target: ref_of("mesh", "t4"), pin: store::LinkPin::Head, role: "chair".into(), at: None }),
        SemioKitMutation::UnbindRepresentation(UnbindRepresentation { index: 0 }),
        SemioKitMutation::ChangeRepresentationPin(ChangeRepresentationPin { index: 0, pin: store::LinkPin::Checkpoint { id: "cp1".into() } }),
        SemioKitMutation::AddType(AddType { id: "chair".into(), name: "Chair".into(), category: "furniture".into(), at: None }),
        SemioKitMutation::AddType(AddType { id: "stool".into(), name: "Stool".into(), category: "furniture".into(), at: Some(1) }),
        SemioKitMutation::RemoveType(RemoveType { id: "chair".into() }),
        SemioKitMutation::RenameType(RenameType { id: "chair".into(), new_name: "Armchair".into() }),
        SemioKitMutation::AddDesign(AddDesign { id: "d1".into(), name: "Design One".into(), at: None }),
        SemioKitMutation::RemoveDesign(RemoveDesign { id: "d1".into() }),
        SemioKitMutation::EditDesign(EditDesign { id: "d1".into(), pieces: vec![], connections: vec![] }),
    ]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::kit::schema::mutations::*;
use crate::standards::v1::subsets::kit::schema::diff::SemioKitDiff;
use crate::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;
use crate::standards::v1::subsets::kit::schema::mutations::add_design;
use crate::standards::v1::subsets::kit::schema::mutations::add_type;
use crate::standards::v1::subsets::kit::schema::mutations::bind_representation;
use crate::standards::v1::subsets::kit::schema::mutations::change_representation_pin;
use crate::standards::v1::subsets::kit::schema::mutations::create_model;
use crate::standards::v1::subsets::kit::schema::mutations::create_object;
use crate::standards::v1::subsets::kit::schema::mutations::create_properties;
use crate::standards::v1::subsets::kit::schema::mutations::delete_model;
use crate::standards::v1::subsets::kit::schema::mutations::delete_object;
use crate::standards::v1::subsets::kit::schema::mutations::delete_properties;
use crate::standards::v1::subsets::kit::schema::mutations::edit_design;
use crate::standards::v1::subsets::kit::schema::mutations::remove_design;
use crate::standards::v1::subsets::kit::schema::mutations::remove_type;
use crate::standards::v1::subsets::kit::schema::mutations::rename_type;
use crate::standards::v1::subsets::kit::schema::mutations::unbind_representation;

/// 📥️ Decodes this subset's own default-derived JSON projection — the exact shape the committed
/// `<kind>/🧪️tests/<fixture>/🦠️mutation/🔣️.json` specification-vector fixtures carry
/// (externally tagged by variant name, snake_case payload fields — no `#[value(rename_all)]` on
/// this enum or its payload structs) — into a real `SemioKitMutation`. Same rationale as
/// `../📸️snapshot/🦀️.rs`'s `decode_kit_snapshot_json`.
pub fn decode_kit_mutation_json(text: &str) -> Result<SemioKitMutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}
}
pub use mutations_codec::*;
