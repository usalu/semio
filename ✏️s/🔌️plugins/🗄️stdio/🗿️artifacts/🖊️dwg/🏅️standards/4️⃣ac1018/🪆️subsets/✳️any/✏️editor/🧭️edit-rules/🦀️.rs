//! 🧭️ The details-pane vocabulary of the dwg editor: which snapshot pointer raises which concrete kind. The schema stamp has no kind, so an edit of it is refused.

use semio_s_artifact_stdio_contract::editing::{Carried, EditRules, EntityRule};

/// 📚 One entity per container block; the preamble triple raises `set-version-info` carrying its two unedited fields.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[
        EntityRule::new("/version", "set-version-info", "version").carrying(&[Carried { payload: "maintenanceVersion", pointer: "/maintenanceVersion" }, Carried { payload: "codepage", pointer: "/codepage" }]),
        EntityRule::new("/maintenanceVersion", "set-version-info", "maintenanceVersion").carrying(&[Carried { payload: "version", pointer: "/version" }, Carried { payload: "codepage", pointer: "/codepage" }]),
        EntityRule::new("/codepage", "set-version-info", "codepage").carrying(&[Carried { payload: "version", pointer: "/version" }, Carried { payload: "maintenanceVersion", pointer: "/maintenanceVersion" }]),
        EntityRule::new("/drawing", "set-drawing", "drawing"),
        EntityRule::new("/header", "set-header", "header"),
        EntityRule::new("/classes", "set-classes", "classes"),
        EntityRule::new("/dependencies", "set-dependencies", "dependencies"),
        EntityRule::new("/summary", "set-summary", "summary"),
        EntityRule::new("/application", "set-application", "application"),
        EntityRule::new("/template", "set-template", "template"),
        EntityRule::new("/auxiliaryHeader", "set-auxiliary-header", "auxiliaryHeader"),
        EntityRule::new("/revisionHistory", "set-revision-history", "revisionHistory"),
        EntityRule::new("/preview", "set-preview", "preview"),
        EntityRule::new("/applicationHistory", "set-application-history", "applicationHistory"),
    ],
    inserts: &[],
    removes: &[],
};
