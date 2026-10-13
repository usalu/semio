//! 📤️ The SDK's own plugin fault constructor.
use semio_framework::{Fault, FaultCode, FaultOrigin};



// 🗣️ Shared two-axis (locale × terminology) label resolution — replaces the ~25x hand-rolled
// `struct XLabels { .. }` + `const X_LABELS_EN/DE` + `fn x_labels(view_state) -> &'static XLabels`
// pattern duplicated per app, AND the 4-crate hand-rolled `NATIVE_EN/NATIVE_DE/REUSE_EN/REUSE_DE` +
// non-exhaustive `match (terminology, is_de) { ..., (_, true) => ... }` terminology resolvers this
// SDK never covered. `app_labels!` now declares all four cells per field and resolves them via an
// exhaustive match on the generated `Locale`/`Terminology` enums — no catch-all arm is possible, so
// adding a locale or terminology to `🖱️ui/🎚️axes/🔣️.json` breaks every invocation until it supplies that
// cell. See ticket 26/08/03/COMPILE-TIME-CHECKED-UI-LABELS-ACROSS-LOCALE-TERMINOLOGY-AND-BRAND.

// 🚫️async: E1 pure error constructor consumed pervasively by sync closures
// (`.map_err`/`.ok_or_else`) — see R9.
pub(crate) fn plugin_sdk_fault(message: impl Into<String>) -> Fault {
    Fault::new(FaultOrigin::Plugin, FaultCode::new("plugin.internal"), message)
}

/// 🚨️ Typed failure at the plugin assembly boundary.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PluginAssemblyError {
    pub code: String,
    pub message: String,
}

// 🚫️async: E1 pure error constructor consumed pervasively by sync closures
// (`.map_err`/`.ok_or_else`) — see R9.
impl PluginAssemblyError {
    /// 🏗️ Creates a codebase-owned assembly failure.
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self { code: code.into(), message: message.into() }
    }

    /// 🚫️async: E1 pure error constructor consumed pervasively by `.map_err(PluginAssemblyError::definition)` — see R9.
    pub fn definition(error: ArtifactDefinitionError) -> Self {
        Self::new(error.code, error.message)
    }
}

impl std::fmt::Display for PluginAssemblyError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for PluginAssemblyError {}




// 🗣️ Shared two-axis (locale × terminology) label resolution — replaces the ~25x hand-rolled
// `struct XLabels { .. }` + `const X_LABELS_EN/DE` + `fn x_labels(view_state) -> &'static XLabels`
// pattern duplicated per app, AND the 4-crate hand-rolled `NATIVE_EN/NATIVE_DE/REUSE_EN/REUSE_DE` +
// non-exhaustive `match (terminology, is_de) { ..., (_, true) => ... }` terminology resolvers this
// SDK never covered. `app_labels!` now declares all four cells per field and resolves them via an
// exhaustive match on the generated `Locale`/`Terminology` enums — no catch-all arm is possible, so
// adding a locale or terminology to `🖱️ui/🎚️axes/🔣️.json` breaks every invocation until it supplies that
// cell. See ticket 26/08/03/COMPILE-TIME-CHECKED-UI-LABELS-ACROSS-LOCALE-TERMINOLOGY-AND-BRAND.

/// 🧾️ A validation failure in the artifact-definition contract.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactDefinitionError {
    code: &'static str,
    message: String,
}

// 🚫️async: E1 pure error constructor consumed pervasively by sync closures
// (`.map_err`/`.ok_or_else`) — see R9.
impl ArtifactDefinitionError {
    /// 🧾️ Creates a validation error at an artifact-definition boundary.
    pub fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self { code, message: message.into() }
    }

    /// 🏷️ Stable machine-readable failure identity.
    pub fn code(&self) -> &'static str {
        self.code
    }

    /// 📝 Human-readable failure detail.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for ArtifactDefinitionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for ArtifactDefinitionError {}
