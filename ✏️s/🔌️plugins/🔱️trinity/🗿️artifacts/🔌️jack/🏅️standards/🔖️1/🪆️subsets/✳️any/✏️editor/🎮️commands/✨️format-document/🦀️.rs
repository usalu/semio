//! 🔎️ 🔎️ Trinity Jack app command — `format-document`.

use crate::core;
use crate::editor::jack::query_window_config::{self, JackEditorWindowConfigMutation, SetQuery};
use crate::standards::v1::subsets::any::schema::mutations::text::TrinityGraphMutation;
use semio_framework_plugin::{Emit, Fault, NoConfigMutation, ViewModel};

pub(crate) fn format_document(jack_query: &str, view: Option<&ViewModel>) -> Result<Emit<TrinityGraphMutation, NoConfigMutation>, Fault> {
    let view = view.ok_or_else(|| Fault::from("Jack query formatting requires an exact editor window"))?;
    match core::format(jack_query) {
        Ok(formatted) => Ok(Emit { window_config_mutations: vec![query_window_config::addressed(view, JackEditorWindowConfigMutation::SetQuery(SetQuery { value: formatted }))?], ..Default::default() }),
        Err(_) => Ok(Emit::default()),
    }
}
