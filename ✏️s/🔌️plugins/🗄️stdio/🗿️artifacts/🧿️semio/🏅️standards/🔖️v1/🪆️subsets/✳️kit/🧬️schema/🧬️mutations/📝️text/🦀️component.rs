//! ⚡️ Semio kit artifact — hand-rolled `OpText` for `SemioKitMutation`. `#[derive(dsl::Mutations)]`
//! only generates `Mutation`/`SemanticMutation` — the wire-text codec stays handcrafted here, one
//! keyword per semantic verb, grammar `keyword:arg1,arg2,...`. Reuses the snapshot facet's own
//! real hex/bracket encoders for children/links/types/pieces/connections (never re-derived).

pub use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::SemioKitMutation;

use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::split_top_level;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::{
    add_design::mutation::AddDesign, add_type::mutation::AddType, bind_representation::mutation::BindRepresentation, change_representation_pin::mutation::ChangeRepresentationPin, create_model::mutation::CreateModel,
    create_object::mutation::CreateObject, create_properties::mutation::CreateProperties, delete_model::mutation::DeleteModel, delete_object::mutation::DeleteObject, delete_properties::mutation::DeleteProperties, edit_design::mutation::EditDesign,
    remove_design::mutation::RemoveDesign, remove_type::mutation::RemoveType, rename_type::mutation::RenameType, unbind_representation::mutation::UnbindRepresentation,
};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::{dec_connection, dec_piece, dec_pin, dec_ref, dec_str, enc_connection, enc_piece, enc_pin, enc_ref, enc_str};

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️Primitives
async fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
async fn enc_pieces(pieces: &[crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitPiece]) -> String {
    format!("[{}]", pieces.iter().map(enc_piece).collect::<Vec<_>>().join(","))
}
async fn dec_pieces(s: &str) -> Result<Vec<crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitPiece>, String> {
    let inner = s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("pieces: expected brackets, got {s:?}"))?;
    split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_piece).collect()
}
async fn enc_connections(cs: &[crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitConnection]) -> String {
    format!("[{}]", cs.iter().map(enc_connection).collect::<Vec<_>>().join(","))
}
async fn dec_connections(s: &str) -> Result<Vec<crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitConnection>, String> {
    let inner = s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("connections: expected brackets, got {s:?}"))?;
    split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_connection).collect()
}
//#endregion 🔖️Primitives

//#region 🔖️OpText
async fn print_kit_mutation(m: &SemioKitMutation) -> String {
    match m {
        SemioKitMutation::CreateObject(p) => format!("createObject:{},{}", enc_str(&p.child_id), enc_ref(&p.target)),
        SemioKitMutation::DeleteObject(p) => format!("deleteObject:{}", enc_str(&p.child_id)),
        SemioKitMutation::CreateModel(p) => format!("createModel:{},{}", enc_str(&p.child_id), enc_ref(&p.target)),
        SemioKitMutation::DeleteModel(p) => format!("deleteModel:{}", enc_str(&p.child_id)),
        SemioKitMutation::CreateProperties(p) => format!("createProperties:{},{}", enc_str(&p.child_id), enc_ref(&p.target)),
        SemioKitMutation::DeleteProperties(_) => "deleteProperties".to_string(),
        SemioKitMutation::BindRepresentation(p) => format!("bindRepresentation:{},{},{}", enc_ref(&p.target), enc_pin(&p.pin), enc_str(&p.role)),
        SemioKitMutation::UnbindRepresentation(p) => format!("unbindRepresentation:{}", p.index),
        SemioKitMutation::ChangeRepresentationPin(p) => format!("changeRepresentationPin:{},{}", p.index, enc_pin(&p.pin)),
        SemioKitMutation::AddType(p) => format!("addType:{},{},{}", enc_str(&p.id), enc_str(&p.name), enc_str(&p.category)),
        SemioKitMutation::RemoveType(p) => format!("removeType:{}", enc_str(&p.id)),
        SemioKitMutation::RenameType(p) => format!("renameType:{},{}", enc_str(&p.id), enc_str(&p.new_name)),
        SemioKitMutation::AddDesign(p) => format!("addDesign:{},{}", enc_str(&p.id), enc_str(&p.name)),
        SemioKitMutation::RemoveDesign(p) => format!("removeDesign:{}", enc_str(&p.id)),
        SemioKitMutation::EditDesign(p) => format!("editDesign:{},{},{}", enc_str(&p.id), enc_pieces(&p.pieces), enc_connections(&p.connections)),
    }
}

async fn parse_kit_mutation(line: &str) -> Result<SemioKitMutation, String> {
    if line == "deleteProperties" {
        return Ok(SemioKitMutation::DeleteProperties(DeleteProperties {}));
    }
    let (tag, rest) = line.split_once(':').ok_or_else(|| format!("kit mutation: missing ':' in {line:?}"))?;
    match tag {
        "createObject" => {
            let (child_id, target) = rest.split_once(',').ok_or_else(|| "createObject: missing comma".to_string())?;
            Ok(SemioKitMutation::CreateObject(CreateObject { child_id: dec_str(child_id).await?, target: dec_ref(target).await? }))
        }
        "deleteObject" => Ok(SemioKitMutation::DeleteObject(DeleteObject { child_id: dec_str(rest).await? })),
        "createModel" => {
            let (child_id, target) = rest.split_once(',').ok_or_else(|| "createModel: missing comma".to_string())?;
            Ok(SemioKitMutation::CreateModel(CreateModel { child_id: dec_str(child_id).await?, target: dec_ref(target).await? }))
        }
        "deleteModel" => Ok(SemioKitMutation::DeleteModel(DeleteModel { child_id: dec_str(rest).await? })),
        "createProperties" => {
            let (child_id, target) = rest.split_once(',').ok_or_else(|| "createProperties: missing comma".to_string())?;
            Ok(SemioKitMutation::CreateProperties(CreateProperties { child_id: dec_str(child_id).await?, target: dec_ref(target).await? }))
        }
        "bindRepresentation" => {
            let parts = split_top_level(rest, ',').await;
            let [target, pin, role] = parts.as_slice() else { return Err(format!("bindRepresentation: expected 3 fields, got {}", parts.len())) };
            Ok(SemioKitMutation::BindRepresentation(BindRepresentation { target: dec_ref(target).await?, pin: dec_pin(pin).await?, role: dec_str(role).await? }))
        }
        "unbindRepresentation" => Ok(SemioKitMutation::UnbindRepresentation(UnbindRepresentation { index: parse_usize(rest).await? })),
        "changeRepresentationPin" => {
            let (index, pin) = rest.split_once(',').ok_or_else(|| "changeRepresentationPin: missing comma".to_string())?;
            Ok(SemioKitMutation::ChangeRepresentationPin(ChangeRepresentationPin { index: parse_usize(index).await?, pin: dec_pin(pin).await? }))
        }
        "addType" => {
            let parts = split_top_level(rest, ',').await;
            let [id, name, category] = parts.as_slice() else { return Err(format!("addType: expected 3 fields, got {}", parts.len())) };
            Ok(SemioKitMutation::AddType(AddType { id: dec_str(id).await?, name: dec_str(name).await?, category: dec_str(category).await? }))
        }
        "removeType" => Ok(SemioKitMutation::RemoveType(RemoveType { id: dec_str(rest).await? })),
        "renameType" => {
            let (id, new_name) = rest.split_once(',').ok_or_else(|| "renameType: missing comma".to_string())?;
            Ok(SemioKitMutation::RenameType(RenameType { id: dec_str(id).await?, new_name: dec_str(new_name).await? }))
        }
        "addDesign" => {
            let (id, name) = rest.split_once(',').ok_or_else(|| "addDesign: missing comma".to_string())?;
            Ok(SemioKitMutation::AddDesign(AddDesign { id: dec_str(id).await?, name: dec_str(name).await? }))
        }
        "removeDesign" => Ok(SemioKitMutation::RemoveDesign(RemoveDesign { id: dec_str(rest).await? })),
        "editDesign" => {
            let parts = split_top_level(rest, ',').await;
            let [id, pieces, connections] = parts.as_slice() else { return Err(format!("editDesign: expected 3 fields, got {}", parts.len())) };
            Ok(SemioKitMutation::EditDesign(EditDesign { id: dec_str(id).await?, pieces: dec_pieces(pieces).await?, connections: dec_connections(connections).await? }))
        }
        other => Err(format!("kit mutation: unknown keyword {other:?}")),
    }
}

impl protocol::OpText for SemioKitMutation {
    async fn print_op(&self) -> String {
        print_kit_mutation(self).await
    }
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_kit_mutation(line).await.map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}
//#endregion 🔖️OpText

//#region 🔖️DemoCases
/// 🌱 One representative value per variant — single source of truth for
/// `ops_grammar_conformance_law`/`protocol_walk_law` in `🚪️io/🦀️component.rs`.
#[cfg(test)]
pub(crate) async fn demo_mutation_cases() -> Vec<SemioKitMutation> {
    let ref_of = |subset: &str, id: &str| store::os_io::ArtifactRef { artifact_id: id.into(), dialect: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: subset.into() } };
    vec![
        SemioKitMutation::CreateObject(CreateObject { child_id: "o1".into(), target: ref_of("object", "t1") }),
        SemioKitMutation::DeleteObject(DeleteObject { child_id: "o1".into() }),
        SemioKitMutation::CreateModel(CreateModel { child_id: "m1".into(), target: ref_of("model", "t2") }),
        SemioKitMutation::DeleteModel(DeleteModel { child_id: "m1".into() }),
        SemioKitMutation::CreateProperties(CreateProperties { child_id: "p1".into(), target: ref_of("value", "t3") }),
        SemioKitMutation::DeleteProperties(DeleteProperties {}),
        SemioKitMutation::BindRepresentation(BindRepresentation { target: ref_of("mesh", "t4"), pin: store::LinkPin::Head, role: "chair".into() }),
        SemioKitMutation::UnbindRepresentation(UnbindRepresentation { index: 0 }),
        SemioKitMutation::ChangeRepresentationPin(ChangeRepresentationPin { index: 0, pin: store::LinkPin::Checkpoint { id: "cp1".into() } }),
        SemioKitMutation::AddType(AddType { id: "chair".into(), name: "Chair".into(), category: "furniture".into() }),
        SemioKitMutation::RemoveType(RemoveType { id: "chair".into() }),
        SemioKitMutation::RenameType(RenameType { id: "chair".into(), new_name: "Armchair".into() }),
        SemioKitMutation::AddDesign(AddDesign { id: "d1".into(), name: "Design One".into() }),
        SemioKitMutation::RemoveDesign(RemoveDesign { id: "d1".into() }),
        SemioKitMutation::EditDesign(EditDesign { id: "d1".into(), pieces: vec![], connections: vec![] }),
    ]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::OpText;

    #[semio_framework_async_macros::async_test]
    async fn op_text_roundtrip_law() {
        for mutation in demo_mutation_cases() {
            let printed = mutation.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = <SemioKitMutation as OpText>::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch (printed {printed:?})");
        }
    }
}
//#endregion 🧪️Tests
