//! 🧱️ Studio foundation: the persisted space configuration, presence, terminology and the identifiers every studio layer shares.

#![allow(async_fn_in_trait)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;
use semio_s_space_core::*;

#[path = "."]
pub mod config {
    #[path = "../🎚️config/🦀️.rs"]
    mod component;
    pub use component::*;

    #[path = "../🎚️config/🧬️schema/🦀️.rs"]
    pub mod schema;
}

#[path = "."]
pub mod presence {
    #[path = "../👥️presence/🦀️.rs"]
    mod component;
    pub use component::*;

    #[path = "../👥️presence/🧬️schema/🦀️.rs"]
    pub mod schema;
}

#[path = "../🗣️terminology/🦀️.rs"]
pub mod terminology;

//#region 🔖️Constants
/// 🪪️ Canonical surface id (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET §1: every
/// `App::builder(...)` id must parse via `semio_framework::parse_surface_app_id` as
/// `<artifact_kind>@<standard>/<subset>#<role>`) — was the bare `"studio"`, which stopped parsing the
/// moment that migration's validation landed in `PluginBuilder::document_app`'s `build_definition`.
pub const S_PLAY_APP_ID: &str = "s.space.studio@1/*#editor";

pub const S_PLAY_CONTROLLER_ID: &str = "s.space.studio@1/*#editor";

pub const S_PLAY_CATALOGUE_TAB_ID: &str = "s-play-catalogue";

pub const S_PLAY_PARAMETERS_TAB_ID: &str = "s-play-parameters";

pub const S_PLAY_INSPECTOR_TAB_ID: &str = "s-play-inspector";

pub const S_PLAY_CATALOGUE_BODY_KEY: &str = "s.play.catalogue";

pub const S_PLAY_PARAMETERS_BODY_KEY: &str = "s.play.parameters";

pub const S_PLAY_INSPECTOR_BODY_KEY: &str = "s.play.inspector";

pub const S_PLAY_CATALOGUE_DRAG_MIME: &str = "application/x-semio-catalogue-item";

pub const S_STUDIO_EXAMPLES: &[(&str, &str)] = &[("demo", "Demo Studio")];

/// 🕹️ The `graph` interaction domain id (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) —
/// one domain over the workflow node graph, shared by both the "instance" and "media-node" granularities.
pub const S_PLAY_INTERACTION_DOMAIN: &str = "graph";
//#endregion 🔖️Constants

//#region 🔖️Constants
pub const S_PLAY_WINDOW_WORKFLOW: &str = "s-workflow";

pub const S_PLAY_BODY_WORKFLOW: &str = "s.play.workflow";

pub const S_PLAY_SURFACE_WORKFLOW: &str = "s.play.workflow";
//#endregion 🔖️Constants
