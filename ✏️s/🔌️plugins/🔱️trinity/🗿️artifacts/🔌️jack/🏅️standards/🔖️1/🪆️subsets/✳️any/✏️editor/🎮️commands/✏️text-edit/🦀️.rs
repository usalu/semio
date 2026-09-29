//! 👁️ 👁️ Trinity Jack app command — `text-edit`.

use crate::standards::v1::subsets::any::schema::mutations::set_query;
use crate::standards::v1::subsets::any::schema::mutations::text::TrinityGraphMutation;
use semio_framework_plugin::{Emit, Fault, FaultCode, FaultOrigin, NoConfigMutation};

/// ⌨️ The coalesce key of a typing burst in the query editor: every keystroke amends the one document edit the burst opened,
/// so a typing run is ONE undo step and never spends the store's fixed applied-edit ledger one keystroke at a time (ticket
/// 26/09/23 F1: the 65th typed character was refused with `batched publication requires preinstalled fixed applied and
/// revision capacity` and the query stopped saving).
pub(crate) const JACK_QUERY_TYPING_COALESCE_KEY: &str = "jack-query-typing";

/// ⌨️ One keystroke of the query editor: the whole query text as one `set-query` document mutation, coalesced into the run.
pub(crate) fn text_edit(text: &str) -> Result<Emit<TrinityGraphMutation, NoConfigMutation>, Fault> {
    if text.len() > crate::JACK_QUERY_MAXIMUM_BYTES {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("jack.query-too-large"), format!("a Jack query holds at most {} bytes", crate::JACK_QUERY_MAXIMUM_BYTES)));
    }
    Ok(Emit { artifact_mutations: vec![set_query(text.to_string())], coalesce_key: Some(JACK_QUERY_TYPING_COALESCE_KEY.into()), ..Default::default() })
}
