//! 🏷️ Rewriting editor ids: controller, surface, body and window identifiers every window, panel and command shares.



//#region 🔖️Constants
pub const TRINITY_REWRITING_PLAY_CONTROLLER_ID: &str = "trinity-rewriting-play";
pub const TRINITY_REWRITING_PLAY_SURFACE_BEFORE: &str = "trinity.rewriting.before";
pub const TRINITY_REWRITING_PLAY_SURFACE_AFTER: &str = "trinity.rewriting.after";
pub const TRINITY_REWRITING_PLAY_SURFACE_LHS: &str = "trinity.rewriting.lhs";
pub const TRINITY_REWRITING_PLAY_SURFACE_RHS: &str = "trinity.rewriting.rhs";
pub const TRINITY_REWRITING_PLAY_SURFACE_JACK: &str = "trinity.rewriting.jack";
pub const TRINITY_REWRITING_PLAY_BODY_BEFORE: &str = "trinity.rewriting.play.before";
pub const TRINITY_REWRITING_PLAY_BODY_AFTER: &str = "trinity.rewriting.play.after";
pub const TRINITY_REWRITING_PLAY_BODY_LHS: &str = "trinity.rewriting.play.lhs";
pub const TRINITY_REWRITING_PLAY_BODY_RHS: &str = "trinity.rewriting.play.rhs";
pub const TRINITY_REWRITING_PLAY_BODY_JACK: &str = "trinity.rewriting.play.jack";
pub const TRINITY_REWRITING_PLAY_BODY_PARAMETERS: &str = "trinity.rewriting.play.parameters";
pub const TRINITY_REWRITING_PLAY_BODY_ARTIFACT: &str = "trinity.rewriting.play.artifact";
pub const TRINITY_REWRITING_PLAY_BODY_CATALOGUE: &str = "trinity.rewriting.play.catalogue";
pub const TRINITY_REWRITING_PLAY_BODY_INSPECTION: &str = "trinity.rewriting.play.inspection";
pub const TRINITY_REWRITING_PLAY_WINDOW_BEFORE: &str = "trinity-rewriting-before";
pub const TRINITY_REWRITING_PLAY_WINDOW_AFTER: &str = "trinity-rewriting-after";
pub const TRINITY_REWRITING_PLAY_WINDOW_LHS: &str = "trinity-rewriting-lhs";
pub const TRINITY_REWRITING_PLAY_WINDOW_RHS: &str = "trinity-rewriting-rhs";
pub const TRINITY_REWRITING_PLAY_WINDOW_JACK: &str = "trinity-rewriting-jack";
pub const TRINITY_REWRITING_PLAY_WINDOW_PARAMETERS: &str = "trinity-rewriting-parameters";
pub const TRINITY_REWRITING_PLAY_RULE_NAME: &str = "label-core";


pub const TRINITY_LOD_MODE_AUTOMATIC: &str = "automatic";
//#endregion 🔖️Constants
