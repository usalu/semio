//! 👁️ 👁️ Trinity Jack app command — `text-edit`.

use crate::standards::v1::subsets::any::schema::mutations::set_query;
use crate::standards::v1::subsets::any::schema::mutations::text::TrinityGraphMutation;
use semio_framework_plugin::{Emit, Fault, FaultCode, FaultOrigin, NoConfigMutation};

/// ⌨️ The query editor's text as one `set-query` document mutation — the whole query, a single buffer. A live typing delivery
/// (`typing` argument) folds into its window's typing run, whose net leaf is the final query and which commits as ONE edit on
/// idle, a caret jump, blur or any other verb (design §13.2), so typing never spends the store's fixed applied-edit ledger one
/// keystroke at a time (ticket 26/09/23 F1); a one-shot dispatch is one edit.
pub(crate) fn text_edit(text: &str) -> Result<Emit<TrinityGraphMutation, NoConfigMutation>, Fault> {
    if text.len() > crate::JACK_QUERY_MAXIMUM_BYTES {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("jack.query-too-large"), format!("a Jack query holds at most {} bytes", crate::JACK_QUERY_MAXIMUM_BYTES)));
    }
    Ok(Emit::mutations(vec![set_query(text.to_string())]))
}
