//! @emoji 🗣️ `s_language_bundle` — aggregates registered [`dsl::LanguageSpec`] hosts for apps (writer, OS).

extern crate semio_framework_os_kernel as dsl_lsp;
extern crate semio_framework_os_kernel as dsl;
pub use dsl_lsp::{handle_json_rpc, LanguageSession};
pub use dsl::{language, language_for_extension, register_language, LanguageRole, LanguageSpec};

/// @emoji 📖️ Opens an in-process session when the extension resolves to a registered document grammar.
pub fn session_for_uri(uri: &str, text: impl Into<String>) -> Option<LanguageSession> {
    let ext = uri.rsplit('.').next().filter(|s| !s.is_empty())?;
    let spec = language_for_extension(ext)?;
    Some(LanguageSession::open(spec, text))
}
