//! 🔬️ Trinity Jack app command — set the addressed graph window level of detail.

use crate::editor::jack::window_config::{addressed, JackGraphWindowConfigMutation, SetLodMode};
use crate::standards::v1::subsets::any::schema::mutations::text::TrinityGraphMutation;
use semio_framework_plugin::{Emit, Fault, FaultCode, FaultOrigin, ViewModel};

pub(crate) fn set_lod_mode(value: &str, view: Option<&ViewModel>) -> Result<Emit<TrinityGraphMutation, crate::editor::jack::config::JackConfigMutation>, Fault> {
    let view = view.ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("jack.window-required"), "Jack LOD requires a host window context"))?;
    if value.len() > 64 {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("jack.lod"), "LOD identifier exceeds its 64-byte bound"));
    }
    let mutation = addressed(view, JackGraphWindowConfigMutation::SetLodMode(SetLodMode { value: value.into() }))?;
    Ok(Emit { window_config_mutations: vec![mutation], ..Default::default() })
}
