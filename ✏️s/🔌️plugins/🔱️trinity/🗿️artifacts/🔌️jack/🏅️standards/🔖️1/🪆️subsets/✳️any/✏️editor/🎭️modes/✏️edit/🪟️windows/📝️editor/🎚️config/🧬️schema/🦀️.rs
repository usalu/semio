//! 📝️ Schema for one Jack editor window's authored query.

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[dsl(extension = "trinity.jackeditorwindowcfg")]
#[dsl(layout = "lines")]
pub struct JackEditorWindowConfig {
    pub jack_query: String,
}

impl Default for JackEditorWindowConfig {
    fn default() -> Self {
        Self { jack_query: crate::editor::jack::TRINITY_JACK_DEFAULT_QUERY.into() }
    }
}
