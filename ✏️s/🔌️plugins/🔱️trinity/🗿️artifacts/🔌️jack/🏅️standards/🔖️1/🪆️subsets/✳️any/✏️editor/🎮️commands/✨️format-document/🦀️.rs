//! 🔎️ 🔎️ Trinity Jack app command — `format-document`.

use crate::standards::v1::subsets::any::schema::language_service;
use crate::standards::v1::subsets::any::schema::mutations::set_query;
use crate::standards::v1::subsets::any::schema::mutations::TrinityGraphMutation;
use semio_framework_plugin::{Emit, NoConfigMutation};

/// ✨️ Reformats the document's query as one undoable `set-query`; a query that does not parse, or is already formatted,
/// changes nothing.
pub fn format_document(jack_query: &str) -> Emit<TrinityGraphMutation, NoConfigMutation> {
    match language_service::format(jack_query) {
        Ok(formatted) if formatted != jack_query && formatted.len() <= crate::standards::v1::subsets::any::schema::graph::JACK_QUERY_MAXIMUM_BYTES => Emit { artifact_mutations: vec![set_query(formatted)], ..Default::default() },
        _ => Emit::default(),
    }
}
