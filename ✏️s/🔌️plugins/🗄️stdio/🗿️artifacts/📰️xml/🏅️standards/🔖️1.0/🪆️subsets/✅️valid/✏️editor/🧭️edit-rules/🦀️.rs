//! 🧭️ The details-pane vocabulary of the valid xml editor: which snapshot pointer raises which concrete kind. A text node edit
//! (addressed by a child-index path through the document) is a computed gesture answered by the editor itself.

use semio_s_artifact_stdio_contract::editing::{EditRules, EntityRule, InsertRule, ItemField};

/// 📚 Every kind keeps §2.8 and §2.9 closed: the document element is renamed together with the doctype, and the doctype is edited only through its two halves.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[
        EntityRule::new("/doc/root/name", "rename-document-element", "name"),
        EntityRule::new("/doc/doctype/externalId", "set-external-subset", "externalId"),
        EntityRule::new("/doc/doctype/declarations", "set-internal-subset", "declarations"),
        EntityRule::new("/doc/declaration/standalone", "set-standalone", "standalone"),
    ],
    inserts: &[InsertRule::new("/doc/doctype/declarations", "declare-entity", "").at("index").keyed(&[ItemField { payload: "parameter", field: "parameter" }, ItemField { payload: "name", field: "name" }, ItemField { payload: "value", field: "value" }])],
    removes: &[],
};
