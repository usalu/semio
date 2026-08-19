//! ⚡️ Semio drawing artifact — hand-rolled `OpText` for `SemioDrawingMutation`. `#[derive(dsl::
//! Mutations)]` only generates `Mutation`/`SemanticMutation` (see `../🦀️component.rs`) — the
//! wire-text codec stays handcrafted here, one keyword per semantic verb, `✳️text`'s own
//! `keyword:arg1,arg2,...` grammar convention (hex/bracket-encoded values, reusing the sibling
//! `📸️snapshot` facet's own real primitives rather than re-deriving a second copy).

pub use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;

use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::{split_top_level, strip_brackets};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::NodePath;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::{
    change_stroke_color::mutation::ChangeStrokeColor, change_stroke_width::mutation::ChangeStrokeWidth, create_layer::mutation::CreateLayer, create_node::mutation::CreateNode, delete_layer::mutation::DeleteLayer, delete_node::mutation::DeleteNode,
    drag_nodes::mutation::DragNodes, flatten::mutation::FlattenNode, group::mutation::GroupNodes, move_node::mutation::MoveNode, reorder_nodes::mutation::ReorderNodes, replace_fill::mutation::ReplaceFill, replace_path::mutation::ReplacePath,
    rotate::mutation::Rotate, scale::mutation::Scale, unflatten::mutation::UnflattenNode, ungroup::mutation::UngroupNode,
};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{
    dec_layer, dec_list, dec_node, dec_path_segment, dec_point2, dec_point3, dec_quaternion, dec_rgba, dec_str, dec_transform, decode_option, enc_layer, enc_list, enc_node, enc_path_segment, enc_point2, enc_point3, enc_quaternion, enc_rgba, enc_str,
    enc_transform, encode_option,
};

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️NodePathPrimitive
/// 🧭️ `[layer,[p0,p1,...]]` — mirrors `NodePath`'s own field shape.
async fn enc_node_path(np: &NodePath) -> String {
    format!("[{},{}]", np.layer, enc_list(&np.path, |i: &usize| i.to_string()))
}
async fn dec_node_path(s: &str) -> Result<NodePath, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [layer, path] = parts.as_slice() else { return Err(format!("node path: expected 2 fields, got {}", parts.len())) };
    Ok(NodePath { layer: layer.parse::<usize>().map_err(|e: std::num::ParseIntError| e.to_string())?, path: dec_list(path, |v| v.parse::<usize>().map_err(|e: std::num::ParseIntError| e.to_string()))? })
}
async fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
async fn enc_indices(indices: &[usize]) -> String {
    enc_list(indices, |i: &usize| i.to_string())
}
async fn dec_indices(s: &str) -> Result<Vec<usize>, String> {
    dec_list(s, parse_usize)
}
//#endregion 🔖️NodePathPrimitive

//#region 🔖️OpText
async fn print_drawing_mutation(m: &SemioDrawingMutation) -> String {
    match m {
        SemioDrawingMutation::CreateLayer(p) => format!("createLayer:{},{}", p.index, enc_layer(&p.layer)),
        SemioDrawingMutation::DeleteLayer(p) => format!("deleteLayer:{}", enc_str(&p.id)),
        SemioDrawingMutation::CreateNode(p) => format!("createNode:{},{},{}", enc_node_path(&p.parent), p.index, enc_node(&p.node)),
        SemioDrawingMutation::DeleteNode(p) => format!("deleteNode:{}", enc_node_path(&p.at)),
        SemioDrawingMutation::MoveNode(p) => format!("moveNode:{},{}", enc_node_path(&p.at), enc_point2(&p.new_origin)),
        SemioDrawingMutation::DragNodes(p) => format!("dragNodes:{},{}", enc_list(&p.ats, enc_node_path), enc_point2(&p.offset)),
        SemioDrawingMutation::Rotate(p) => format!("rotate:{},{}", enc_node_path(&p.at), enc_quaternion(&p.new_rotation)),
        SemioDrawingMutation::Scale(p) => format!("scale:{},{}", enc_node_path(&p.at), enc_point3(&p.new_scale)),
        SemioDrawingMutation::ReorderNodes(p) => format!("reorderNodes:{},{},{}", enc_node_path(&p.parent), p.from, p.to),
        SemioDrawingMutation::Group(p) => format!("group:{},{},{}", enc_node_path(&p.parent), enc_indices(&p.indices), enc_transform(&p.transform)),
        SemioDrawingMutation::Ungroup(p) => format!("ungroup:{}", enc_node_path(&p.at)),
        SemioDrawingMutation::Flatten(p) => format!("flatten:{}", enc_node_path(&p.at)),
        SemioDrawingMutation::Unflatten(p) => format!("unflatten:{},{}", enc_node_path(&p.at), enc_node(&p.original)),
        SemioDrawingMutation::ReplacePath(p) => format!("replacePath:{},{}", enc_node_path(&p.at), enc_list(&p.new_segments, enc_path_segment)),
        SemioDrawingMutation::ReplaceFill(p) => format!("replaceFill:{},{}", enc_str(&p.style_name), encode_option(&p.new_fill, enc_rgba)),
        SemioDrawingMutation::ChangeStrokeColor(p) => format!("changeStrokeColor:{},{}", enc_str(&p.style_name), encode_option(&p.new_color, enc_rgba)),
        SemioDrawingMutation::ChangeStrokeWidth(p) => format!("changeStrokeWidth:{},{}", enc_str(&p.style_name), encode_option(&p.new_width, |v: &f64| v.to_string())),
    }
}

async fn parse_drawing_mutation(line: &str) -> Result<SemioDrawingMutation, String> {
    let (tag, rest) = line.split_once(':').ok_or_else(|| format!("drawing mutation: missing ':' in {line:?}"))?;
    match tag {
        "createLayer" => {
            let (idx, layer) = rest.split_once(',').ok_or_else(|| "createLayer: missing comma".to_string())?;
            Ok(SemioDrawingMutation::CreateLayer(CreateLayer { index: parse_usize(idx)?, layer: dec_layer(layer)? }))
        }
        "deleteLayer" => Ok(SemioDrawingMutation::DeleteLayer(DeleteLayer { id: dec_str(rest)? })),
        "createNode" => {
            let parts = split_top_level(rest, ',');
            let [parent, index, node] = parts.as_slice() else { return Err(format!("createNode: expected 3 fields, got {}", parts.len())) };
            Ok(SemioDrawingMutation::CreateNode(CreateNode { parent: dec_node_path(parent)?, index: parse_usize(index)?, node: dec_node(node)? }))
        }
        "deleteNode" => Ok(SemioDrawingMutation::DeleteNode(DeleteNode { at: dec_node_path(rest)? })),
        "moveNode" => {
            let parts = split_top_level(rest, ',');
            let [at, new_origin] = parts.as_slice() else { return Err(format!("moveNode: expected 2 fields, got {}", parts.len())) };
            Ok(SemioDrawingMutation::MoveNode(MoveNode { at: dec_node_path(at)?, new_origin: dec_point2(new_origin)? }))
        }
        "dragNodes" => {
            let parts = split_top_level(rest, ',');
            let [ats, offset] = parts.as_slice() else { return Err(format!("dragNodes: expected 2 fields, got {}", parts.len())) };
            Ok(SemioDrawingMutation::DragNodes(DragNodes { ats: dec_list(ats, dec_node_path)?, offset: dec_point2(offset)? }))
        }
        "rotate" => {
            let parts = split_top_level(rest, ',');
            let [at, new_rotation] = parts.as_slice() else { return Err(format!("rotate: expected 2 fields, got {}", parts.len())) };
            Ok(SemioDrawingMutation::Rotate(Rotate { at: dec_node_path(at)?, new_rotation: dec_quaternion(new_rotation)? }))
        }
        "scale" => {
            let parts = split_top_level(rest, ',');
            let [at, new_scale] = parts.as_slice() else { return Err(format!("scale: expected 2 fields, got {}", parts.len())) };
            Ok(SemioDrawingMutation::Scale(Scale { at: dec_node_path(at)?, new_scale: dec_point3(new_scale)? }))
        }
        "reorderNodes" => {
            let parts = split_top_level(rest, ',');
            let [parent, from, to] = parts.as_slice() else { return Err(format!("reorderNodes: expected 3 fields, got {}", parts.len())) };
            Ok(SemioDrawingMutation::ReorderNodes(ReorderNodes { parent: dec_node_path(parent)?, from: parse_usize(from)?, to: parse_usize(to)? }))
        }
        "group" => {
            let parts = split_top_level(rest, ',');
            let [parent, indices, transform] = parts.as_slice() else { return Err(format!("group: expected 3 fields, got {}", parts.len())) };
            Ok(SemioDrawingMutation::Group(GroupNodes { parent: dec_node_path(parent)?, indices: dec_indices(indices)?, transform: dec_transform(transform)? }))
        }
        "ungroup" => Ok(SemioDrawingMutation::Ungroup(UngroupNode { at: dec_node_path(rest)? })),
        "flatten" => Ok(SemioDrawingMutation::Flatten(FlattenNode { at: dec_node_path(rest)? })),
        "unflatten" => {
            let parts = split_top_level(rest, ',');
            let [at, original] = parts.as_slice() else { return Err(format!("unflatten: expected 2 fields, got {}", parts.len())) };
            Ok(SemioDrawingMutation::Unflatten(UnflattenNode { at: dec_node_path(at)?, original: dec_node(original)? }))
        }
        "replacePath" => {
            let parts = split_top_level(rest, ',');
            let [at, new_segments] = parts.as_slice() else { return Err(format!("replacePath: expected 2 fields, got {}", parts.len())) };
            Ok(SemioDrawingMutation::ReplacePath(ReplacePath { at: dec_node_path(at)?, new_segments: dec_list(new_segments, dec_path_segment)? }))
        }
        "replaceFill" => {
            let parts = split_top_level(rest, ',');
            let [name, fill] = parts.as_slice() else { return Err(format!("replaceFill: expected 2 fields, got {}", parts.len())) };
            Ok(SemioDrawingMutation::ReplaceFill(ReplaceFill { style_name: dec_str(name)?, new_fill: decode_option(fill, dec_rgba)? }))
        }
        "changeStrokeColor" => {
            let parts = split_top_level(rest, ',');
            let [name, color] = parts.as_slice() else { return Err(format!("changeStrokeColor: expected 2 fields, got {}", parts.len())) };
            Ok(SemioDrawingMutation::ChangeStrokeColor(ChangeStrokeColor { style_name: dec_str(name)?, new_color: decode_option(color, dec_rgba)? }))
        }
        "changeStrokeWidth" => {
            let parts = split_top_level(rest, ',');
            let [name, width] = parts.as_slice() else { return Err(format!("changeStrokeWidth: expected 2 fields, got {}", parts.len())) };
            Ok(SemioDrawingMutation::ChangeStrokeWidth(ChangeStrokeWidth { style_name: dec_str(name)?, new_width: decode_option(width, |v| v.parse::<f64>().map_err(|e: std::num::ParseFloatError| e.to_string()))? }))
        }
        other => Err(format!("drawing mutation: unknown keyword {other:?}")),
    }
}

impl protocol::OpText for SemioDrawingMutation {
    async fn print_op(&self) -> String {
        print_drawing_mutation(self)
    }
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_drawing_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}
//#endregion 🔖️OpText

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::demo_mutation_cases;
    use protocol::OpText;

    #[semio_framework_async_macros::async_test]
    async fn op_text_roundtrip_law() {
        for mutation in demo_mutation_cases() {
            let printed = mutation.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = <SemioDrawingMutation as OpText>::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch (printed {printed:?})");
        }
    }
}
//#endregion 🧪️Tests
