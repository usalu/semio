//! 🫧️ S Home app-transient mutation aggregate — one verb: fold one authenticated directory page into the projection.

use super::HomeTransient;

#[path = "📬️apply-directory/🦀️.rs"]
mod apply_directory_page;
pub use apply_directory_page::ApplyDirectoryPage;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslOps, dsl::Mutations)]
#[mutations(snapshot = HomeTransient, diff = HomeTransient, schema = "s.space.home.transient")]
pub enum HomeTransientMutation {
    #[dsl(key = "apply-directory-page")]
    ApplyDirectoryPage(ApplyDirectoryPage),
}

impl protocol::OpText for HomeTransientMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{keyword} ");
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
        let spec_fn = variants.iter().find(|(key, _)| key == &keyword).map(|(_, spec)| *spec).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for HomeTransientMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}

impl protocol::MutationDiff<HomeTransient> for HomeTransient {
    fn apply(&self, _base: &HomeTransient) -> protocol::MutationApplyResult<HomeTransient> {
        Ok(self.clone())
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}

//#region 🌉️TestBridge
/// 🌉️ The committed-vector report of this state lane for the language-neutral case adapter, which links only this
/// crate: production dispatch (`Mutation::diff(..).apply_to`) and the mutation's own inverse over `HomeTransient`.
///
/// @see store::os_store::test_support::mutation_report_json
pub fn home_transient_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    store::os_store::test_support::mutation_report_json::<HomeTransient, HomeTransientMutation>(base_json, mutation_json, after_json)
}
//#endregion 🌉️TestBridge
