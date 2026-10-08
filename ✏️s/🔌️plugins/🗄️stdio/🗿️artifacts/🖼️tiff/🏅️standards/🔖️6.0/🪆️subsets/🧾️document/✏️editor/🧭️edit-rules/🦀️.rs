//! 🧭️ The details-pane vocabulary of the tiff document editor: which snapshot pointer raises which concrete kind.
//! A tag's values raise `replace-tag` addressed by directory position and tag number; inserting or removing a tag entry raises `replace-tag` / `remove-tag`; inserting or removing a directory raises its positional kind.
//! A single sample of a block is a computed gesture answered by the editor itself as one `replace-samples` row; block geometry and tag numbers have no kind and are refused naming the path.

use semio_s_artifact_stdio_contract::editing::{EditRules, EntityRule, InsertRule, ItemField, RemoveRule, Selector};

const DIRECTORY: &[Selector] = &[Selector::Index("ifdIndex")];
const TAG_OF_DIRECTORY: &[Selector] = &[Selector::Index("ifdIndex"), Selector::Field { payload: "tag", field: "tag" }];

/// 📚 Tag values by (directory position, tag number); directories and tag entries insert and remove positionally or by tag number.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[EntityRule::new("/ifds/*/entries/*/values", "replace-tag", "values").selecting(TAG_OF_DIRECTORY)],
    inserts: &[
        InsertRule::new("/ifds", "insert-ifd", "ifd").at("index"),
        InsertRule::new("/ifds/*/entries", "replace-tag", "").selecting(DIRECTORY).keyed(&[ItemField { payload: "tag", field: "tag" }, ItemField { payload: "values", field: "values" }]),
    ],
    removes: &[RemoveRule::by_index("/ifds", "remove-ifd", "index"), RemoveRule::by_key("/ifds/*/entries", "remove-tag", "tag", "tag").selecting(DIRECTORY)],
};
