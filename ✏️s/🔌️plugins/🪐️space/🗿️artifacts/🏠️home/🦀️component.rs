//! 🏠️ S Home launcher artifact — document entity (constitutional: general).

pub const S_HOME_DOCUMENT_SCHEMA: &str = "s.home";
pub use crate::artifacts::home::schema::SHomeArtifact;
pub use crate::artifacts::home::snapshot::schema::SHomeSnapshot;

//#region 🔖️Register
/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    crate::artifacts::home::engine::register_artifact_schema();
    dsl::register_language(dsl::LanguageSpec {
        id: "space.shome",
        extension: Some("shome"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::home::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::home::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::home::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::home::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("space.shome"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "space.shome.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::home::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::home::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::home::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::home::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("space.shome.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "space.shome.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::home::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::home::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("space.shome.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "home.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::home::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::home::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("home.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "home.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::home::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::home::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("home.spr"),
    });
}
//#endregion 🔖️Register
