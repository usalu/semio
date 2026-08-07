//! 🏠️ S Home launcher artifact — document entity (constitutional: general).

use serde::{Deserialize, Serialize};

//#region 🔖️Register
/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "space.shome",
        extension: Some("shome"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::home::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::home::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::home::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::home::pack::COMPONENT_PROTOCOL_PATH),
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
        protocol: Some(crate::artifacts::home::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::home::pack::COMPONENT_PROTOCOL_PATH),
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

//#region 🔖️Types
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase")]
#[dsl(extension = "shome")]
pub struct SHomeDocument {
    pub schema: String,
    #[serde(default)]
    #[dsl(key = "gen")]
    pub catalog_generation: u64,
}

//#region 🔖️DocumentCodec
/// 📜️ Handcrafted DocumentDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::DocumentDsl for SHomeDocument {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(
            body,
            &Self::__dsl_spec(),
            &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document },
        )?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        )
        .expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted DocumentPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::DocumentPack for SHomeDocument {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        )
        .map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::DocumentDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::DocumentDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️DocumentCodec

//#endregion 🔖️Types
