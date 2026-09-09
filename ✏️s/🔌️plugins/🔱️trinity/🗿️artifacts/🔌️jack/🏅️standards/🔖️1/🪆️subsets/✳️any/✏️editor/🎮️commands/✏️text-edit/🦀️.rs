//! 👁️ 👁️ Trinity Jack app command — `text-edit`.

use crate::editor::jack::query_window_config::{self, JackEditorWindowConfigMutation, SetQuery};
use crate::standards::v1::subsets::any::schema::mutations::text::TrinityGraphMutation;
use semio_framework_plugin::{Emit, Fault, NoConfigMutation, ViewModel};

pub(crate) fn text_edit(text: &str, view: Option<&ViewModel>) -> Result<Emit<TrinityGraphMutation, NoConfigMutation>, Fault> {
    let view = view.ok_or_else(|| Fault::from("Jack text edit requires an exact editor window"))?;
    let mutation = query_window_config::addressed(view, JackEditorWindowConfigMutation::SetQuery(SetQuery { value: text.to_string() }))?;
    Ok(Emit { window_config_mutations: vec![mutation], ..Default::default() })
}
