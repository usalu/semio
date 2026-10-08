//! 🧭️ The details-pane vocabulary of the obj editor: which snapshot pointer raises which concrete kind.
//! Renaming a group or an object (a remove then a set at the same position) is a computed gesture answered by the editor itself.

use semio_s_artifact_stdio_contract::editing::{EditRules, EntityRule, InsertRule, ItemField, RemoveRule, Selector};

const BY_NAME: &[Selector] = &[Selector::Field { payload: "name", field: "name" }];
const NAME_AND_FACES: &[ItemField] = &[ItemField { payload: "name", field: "name" }, ItemField { payload: "faces", field: "faces" }];

/// 📚 Vertices, texture coordinates, normals and faces are addressed by position; groups and objects by name; ranges and statements are collection setters.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[
        EntityRule::new("/vertices/*", "set-vertex", "vertex").selecting(&[Selector::Index("index")]),
        EntityRule::new("/texcoords/*", "set-texcoord", "texcoord").selecting(&[Selector::Index("index")]),
        EntityRule::new("/normals/*", "set-normal", "normal").selecting(&[Selector::Index("index")]),
        EntityRule::new("/faces/*", "set-face", "face").selecting(&[Selector::Index("index")]),
        EntityRule::new("/groups/*/faces", "set-group", "faces").selecting(BY_NAME),
        EntityRule::new("/objects/*/faces", "set-object", "faces").selecting(BY_NAME),
        EntityRule::new("/mtllib", "set-mtllib", "mtllib"),
        EntityRule::new("/usemtl", "set-usemtl", "usemtl"),
        EntityRule::new("/smoothingGroups", "set-smoothing-groups", "smoothingGroups"),
        EntityRule::new("/unknownStatements", "set-unknown-statements", "unknownStatements"),
    ],
    inserts: &[
        InsertRule::new("/vertices", "insert-vertex", "vertex").at("index"),
        InsertRule::new("/texcoords", "insert-texcoord", "texcoord").at("index"),
        InsertRule::new("/normals", "insert-normal", "normal").at("index"),
        InsertRule::new("/faces", "insert-face", "face").at("index"),
        InsertRule::new("/groups", "set-group", "").at("index").keyed(NAME_AND_FACES),
        InsertRule::new("/objects", "set-object", "").at("index").keyed(NAME_AND_FACES),
    ],
    removes: &[
        RemoveRule::by_index("/vertices", "remove-vertex", "index"),
        RemoveRule::by_index("/texcoords", "remove-texcoord", "index"),
        RemoveRule::by_index("/normals", "remove-normal", "index"),
        RemoveRule::by_index("/faces", "remove-face", "index"),
        RemoveRule::by_key("/groups", "remove-group", "name", "name"),
        RemoveRule::by_key("/objects", "remove-object", "name", "name"),
    ],
};
