//! 🔎️ 🔎️ Trinity Jack app command — `format-document`.

use crate::core;
use crate::standards::v1::subsets::any::schema::mutations::set_query;
use crate::standards::v1::subsets::any::schema::mutations::TrinityGraphMutation;
use semio_framework_plugin::{Emit, NoConfigMutation};

/// ✨️ Reformats the document's query as one undoable `set-query`; a query that does not parse, or is already formatted,
/// changes nothing.
pub(crate) fn format_document(jack_query: &str) -> Emit<TrinityGraphMutation, NoConfigMutation> {
    match core::format(jack_query) {
        Ok(formatted) if formatted != jack_query && formatted.len() <= crate::JACK_QUERY_MAXIMUM_BYTES => Emit { artifact_mutations: vec![set_query(formatted)], ..Default::default() },
        _ => Emit::default(),
    }
}
