//! 🧬️ Mutations of one Jack results-window execution state.

use super::{JackResultsWindowTransient, JackResultsWindowTransientDiff};
#[path = "📊️replace-query-result/🦀️.rs"]
mod replace_query_result;
pub use replace_query_result::ReplaceQueryResult;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::RetireOwned)]
#[value(tag = "kind", rename_all = "kebab-case")]
#[mutations(snapshot = JackResultsWindowTransient, diff = JackResultsWindowTransientDiff, schema = "trinity.jackresultswindowtransient")]
pub enum JackResultsWindowTransientMutation {
    #[dsl(key = "replace-query-result")]
    ReplaceQueryResult(ReplaceQueryResult),
}





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
