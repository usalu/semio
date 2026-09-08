//! 🧬️ Remodeling config mutation collection.

use super::{RemodelingConfig, RemodelingWorldCamera};
#[path = "📸️replace-config/🦀️.rs"]
mod snapshot;
pub use snapshot::ReplaceConfig;
#[path = "🎥️set-camera/🦀️.rs"]
mod set_camera;
pub use set_camera::SetCamera;
#[path = "👓️set-layer-visibility/🦀️.rs"]
mod set_layer_visibility;
pub use set_layer_visibility::SetLayerVisibility;
#[path = "🎞️set-frame-cursor/🦀️.rs"]
mod set_frame_cursor;
pub use set_frame_cursor::SetFrameCursor;
#[path = "📊️set-report-table/🦀️.rs"]
mod set_report_table;
pub use set_report_table::SetReportTable;
#[path = "🧰️set-active-utility/🦀️.rs"]
mod set_active_utility;
pub use set_active_utility::SetActiveUtility;
#[path = "🗣️set-locale/🦀️.rs"]
mod set_locale;
pub use set_locale::SetLocale;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::ToValue, dsl::FromValue, dsl::DslOps, dsl::Mutations)]
#[mutations(snapshot = RemodelingConfig, diff = RemodelingConfig, schema = "remodeling.config")]
pub enum RemodelingConfigMutation {
    #[dsl(key = "replace-config")]
    ReplaceConfig(ReplaceConfig),
    #[dsl(key = "set-camera")]
    SetCamera(SetCamera),
    #[dsl(key = "set-layer-visibility")]
    SetLayerVisibility(SetLayerVisibility),
    #[dsl(key = "set-frame-cursor")]
    SetFrameCursor(SetFrameCursor),
    #[dsl(key = "set-report-table")]
    SetReportTable(SetReportTable),
    #[dsl(key = "set-active-utility")]
    SetActiveUtility(SetActiveUtility),
    #[dsl(key = "set-locale")]
    SetLocale(SetLocale),
}

impl protocol::OpText for RemodelingConfigMutation {
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

impl protocol::OpBinary for RemodelingConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { dsl::variants_binary::encode_op(self) }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> { dsl::variants_binary::decode_op(bytes) }
}

#[cfg(test)]
mod vectors {
    use super::*;
    use protocol::{Mutation, MutationDiff, OpBinary, OpText};

    #[test]
    fn mutations_match_the_json_oracle_and_restore_the_base() {
        let vectors: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔁️mutations.json")).unwrap();
        let base: RemodelingConfig = serde_json::from_value(vectors["base"].clone()).unwrap();
        for vector in vectors["cases"].as_array().unwrap() {
            let oracle: RemodelingConfigMutation = serde_json::from_value(vector["mutation"].clone()).unwrap();
            let mutation: RemodelingConfigMutation = protocol::os_pack::json::from_json_str(&vector["mutation"].to_string()).unwrap();
            assert_eq!(mutation, oracle);
            let expected: RemodelingConfig = serde_json::from_value(vector["expected"].clone()).unwrap();
            let next = mutation.diff(&base).diff().apply(&base).unwrap();
            assert_eq!(next, expected);
            let restored = mutation.inverse(&base).into_iter().fold(next, |state, inverse| inverse.diff(&state).diff().apply(&state).unwrap());
            assert_eq!(restored, base);
            assert_eq!(RemodelingConfigMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
            assert_eq!(RemodelingConfigMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
        }
    }
}
