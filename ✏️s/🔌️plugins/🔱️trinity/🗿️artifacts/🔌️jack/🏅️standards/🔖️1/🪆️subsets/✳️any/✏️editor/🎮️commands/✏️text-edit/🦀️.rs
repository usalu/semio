//! 👁️ 👁️ Trinity Jack app command — `text-edit`.

use crate::editor::jack::query_window_config::{self, JackEditorWindowConfigMutation, SetQuery};
use crate::standards::v1::subsets::any::schema::mutations::text::TrinityGraphMutation;
use semio_framework_plugin::{Emit, Fault, NoConfigMutation, ViewModel};

/// ⌨️ The coalesce key of a typing burst in the query editor: every keystroke amends the one window-config edit the burst
/// opened, so typing is one undo step and never spends the store's fixed applied-edit ledger one keystroke at a time (ticket
/// 26/09/23 F1: the 65th typed character was refused with `batched publication requires preinstalled fixed applied and
/// revision capacity` and the query stopped saving).
pub(crate) const JACK_QUERY_TYPING_COALESCE_KEY: &str = "jack-query-typing";

pub(crate) fn text_edit(text: &str, view: Option<&ViewModel>) -> Result<Emit<TrinityGraphMutation, NoConfigMutation>, Fault> {
    let view = view.ok_or_else(|| Fault::from("Jack text edit requires an exact editor window"))?;
    let mutation = query_window_config::addressed(view, JackEditorWindowConfigMutation::SetQuery(SetQuery { value: text.to_string() }))?;
    Ok(Emit { window_config_mutations: vec![mutation], coalesce_key: Some(JACK_QUERY_TYPING_COALESCE_KEY.into()), ..Default::default() })
}
