//! 🧬️ Trinity jack configuration mutation collection.

use super::{JackConfig, Camera, JackEditorSelection};
#[path = "📸️replace-config/🦀️.rs"]
mod replace_config;
pub use replace_config::ReplaceConfig;
#[path = "🎥️set-camera/🦀️.rs"]
mod set_camera;
pub use set_camera::SetCamera;
#[path = "🗿️set-active-fixture/🦀️.rs"]
mod set_active_fixture;
pub use set_active_fixture::SetActiveFixture;
#[path = "🔎️set-query/🦀️.rs"]
mod set_query;
pub use set_query::SetQuery;
#[path = "📊️set-result/🦀️.rs"]
mod set_result;
pub use set_result::SetResult;
#[path = "📝️set-editor-engagement-input/🦀️.rs"]
mod set_editor_engagement_input;
pub use set_editor_engagement_input::SetEditorEngagementInput;
#[path = "🌐️set-graph-engagement-input/🦀️.rs"]
mod set_graph_engagement_input;
pub use set_graph_engagement_input::SetGraphEngagementInput;
#[path = "📈️set-results-engagement-input/🦀️.rs"]
mod set_results_engagement_input;
pub use set_results_engagement_input::SetResultsEngagementInput;
#[path = "🔄️set-reorganize-epoch/🦀️.rs"]
mod set_reorganize_epoch;
pub use set_reorganize_epoch::SetReorganizeEpoch;
#[path = "🔤️set-editor-selection/🦀️.rs"]
mod set_editor_selection;
pub use set_editor_selection::SetEditorSelection;
#[path = "🔍️set-lod-mode/🦀️.rs"]
mod set_lod_mode;
pub use set_lod_mode::SetLodMode;
#[path = "🔢️set-revision/🦀️.rs"]
mod set_revision;
pub use set_revision::SetRevision;
#[path = "🗣️set-locale/🦀️.rs"]
mod set_locale;
pub use set_locale::SetLocale;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslOps, dsl::Mutations)]
#[mutations(snapshot = JackConfig, diff = JackConfig, schema = "trinity.jackcfg")]
pub enum JackConfigMutation {
    #[dsl(key = "replace-config")]
    ReplaceConfig(ReplaceConfig),
    #[dsl(key = "set-camera")]
    SetCamera(SetCamera),
    #[dsl(key = "set-active-fixture")]
    SetActiveFixture(SetActiveFixture),
    #[dsl(key = "set-query")]
    SetQuery(SetQuery),
    #[dsl(key = "set-result")]
    SetResult(SetResult),
    #[dsl(key = "set-editor-engagement-input")]
    SetEditorEngagementInput(SetEditorEngagementInput),
    #[dsl(key = "set-graph-engagement-input")]
    SetGraphEngagementInput(SetGraphEngagementInput),
    #[dsl(key = "set-results-engagement-input")]
    SetResultsEngagementInput(SetResultsEngagementInput),
    #[dsl(key = "set-reorganize-epoch")]
    SetReorganizeEpoch(SetReorganizeEpoch),
    #[dsl(key = "set-editor-selection")]
    SetEditorSelection(SetEditorSelection),
    #[dsl(key = "set-lod-mode")]
    SetLodMode(SetLodMode),
    #[dsl(key = "set-revision")]
    SetRevision(SetRevision),
    #[dsl(key = "set-locale")]
    SetLocale(SetLocale),
}

impl protocol::OpText for JackConfigMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for JackConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { dsl::variants_binary::encode_op(self) }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> { dsl::variants_binary::decode_op(bytes) }
}

