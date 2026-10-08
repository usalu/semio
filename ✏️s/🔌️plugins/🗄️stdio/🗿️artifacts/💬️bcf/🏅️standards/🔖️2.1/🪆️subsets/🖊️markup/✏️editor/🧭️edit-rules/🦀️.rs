//! 🧭️ The details-pane vocabulary of the bcf editor: which snapshot pointer raises which concrete kind.

use semio_s_artifact_stdio_contract::editing::{EditRules, EntityRule, InsertRule, RemoveRule, Selector};

const TOPIC: &[Selector] = &[Selector::Field { payload: "guid", field: "guid" }];
const COMMENT: &[Selector] = &[Selector::Field { payload: "topicGuid", field: "guid" }, Selector::Field { payload: "guid", field: "guid" }];
const VIEWPOINT: &[Selector] = COMMENT;
const IN_TOPIC: &[Selector] = &[Selector::Field { payload: "topicGuid", field: "guid" }];

/// 📚 Topics, comments and viewpoints are addressed by their guids; each markup field is its own sparse payload field.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[
        EntityRule::new("/version", "set-version", "version"),
        EntityRule::new("/parts", "set-parts", "parts"),
        EntityRule::new("/topics/*/title", "set-topic-markup", "title").selecting(TOPIC),
        EntityRule::new("/topics/*/description", "set-topic-markup", "description").selecting(TOPIC),
        EntityRule::new("/topics/*/status", "set-topic-markup", "status").selecting(TOPIC),
        EntityRule::new("/topics/*/priority", "set-topic-markup", "priority").selecting(TOPIC),
        EntityRule::new("/topics/*/labels", "set-topic-markup", "labels").selecting(TOPIC),
        EntityRule::new("/topics/*/creationDate", "set-topic-markup", "creationDate").selecting(TOPIC),
        EntityRule::new("/topics/*/creationAuthor", "set-topic-markup", "creationAuthor").selecting(TOPIC),
        EntityRule::new("/topics/*/comments/*/date", "set-comment", "date").selecting(COMMENT),
        EntityRule::new("/topics/*/comments/*/author", "set-comment", "author").selecting(COMMENT),
        EntityRule::new("/topics/*/comments/*/text", "set-comment", "text").selecting(COMMENT),
        EntityRule::new("/topics/*/comments/*/viewpointRef", "set-comment", "viewpointRef").selecting(COMMENT),
        EntityRule::new("/topics/*/viewpoints/*/camera", "set-viewpoint-camera", "camera").selecting(VIEWPOINT),
        EntityRule::new("/topics/*/viewpoints/*/components", "set-viewpoint-components", "components").selecting(VIEWPOINT),
        EntityRule::new("/topics/*/viewpoints/*/snapshot", "set-viewpoint-snapshot", "snapshot").selecting(VIEWPOINT),
    ],
    inserts: &[
        InsertRule::new("/topics", "insert-topic", "topic").at("index"),
        InsertRule::new("/topics/*/comments", "insert-comment", "comment").at("index").selecting(IN_TOPIC),
        InsertRule::new("/topics/*/viewpoints", "insert-viewpoint", "viewpoint").at("index").selecting(IN_TOPIC),
    ],
    removes: &[
        RemoveRule::by_key("/topics", "remove-topic", "guid", "guid"),
        RemoveRule::by_key("/topics/*/comments", "remove-comment", "guid", "guid").selecting(IN_TOPIC),
        RemoveRule::by_key("/topics/*/viewpoints", "remove-viewpoint", "guid", "guid").selecting(IN_TOPIC),
    ],
};
