//! 🔬️ Updates the addressed graph window's local level of detail.

use crate::editor::rewriting::window_config::{addressed, RewritingWindowConfigMutation, SetLodMode};
use crate::standards::v1::subsets::any::schema::mutations::text::RewriteRuleMutation;
use semio_framework_plugin::{Emit, Fault, FaultCode, FaultOrigin, NoConfigMutation, ViewModel};

pub(crate) fn set_lod_mode(value: &str, view: Option<&ViewModel>) -> Result<Emit<RewriteRuleMutation, NoConfigMutation>, Fault> {
    let view = view.ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("rewriting.window-required"), "Rewriting LOD requires a host window context"))?;
    if value.chars().take(65).count() > 64 {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("rewriting.lod"), "LOD identifier exceeds its 64-character bound"));
    }
    let mutation = addressed(view, RewritingWindowConfigMutation::SetLodMode(SetLodMode { value: value.into() }))?;
    Ok(Emit { window_config_mutations: vec![mutation], ..Default::default() })
}
