//! 🧬️ Remodeling presence mutation collection.

use super::{RemodelingPresence};
#[path = "👥️replace-presence/🦀️.rs"]
mod snapshot;
pub use snapshot::ReplacePresence;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::ToValue, dsl::FromValue, dsl::DslOps, dsl::Mutations)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[mutations(snapshot = RemodelingPresence, diff = RemodelingPresence, schema = "remodeling.presence")]
pub enum RemodelingPresenceMutation {
    #[dsl(key = "replace-presence")]
    ReplacePresence(ReplacePresence),
}

impl protocol::OpText for RemodelingPresenceMutation {
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

impl protocol::OpBinary for RemodelingPresenceMutation {
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
        let base: RemodelingPresence = serde_json::from_value(vectors["base"].clone()).unwrap();
        for vector in vectors["cases"].as_array().unwrap() {
            let oracle: RemodelingPresenceMutation = serde_json::from_value(vector["mutation"].clone()).unwrap();
            let mutation: RemodelingPresenceMutation = protocol::os_pack::json::from_json_str(&vector["mutation"].to_string()).unwrap();
            assert_eq!(mutation, oracle);
            let expected: RemodelingPresence = serde_json::from_value(vector["expected"].clone()).unwrap();
            let next = mutation.diff(&base).diff().apply(&base).unwrap();
            assert_eq!(next, expected);
            let restored = mutation.inverse(&base).into_iter().fold(next, |state, inverse| inverse.diff(&state).diff().apply(&state).unwrap());
            assert_eq!(restored, base);
            assert_eq!(RemodelingPresenceMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
            assert_eq!(RemodelingPresenceMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
        }
    }
}
