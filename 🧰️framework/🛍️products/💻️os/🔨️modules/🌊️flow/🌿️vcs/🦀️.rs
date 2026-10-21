//! 🌿️ Flow document VCS: operations, DSL, store, and forms bridge.

use crate::os_spr::{ColdMutation,ColdMutationDiff,ColdDiffAlgebra};
use neural_engine as neural;

use neural::{Neuron, Synapse};
use semio_framework_value_ordered::ordered::UpdateCursor as LayoutUpdate;
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};

use semio_framework_artifact_flow_flow::*;

#[path="♻️retirement/🦀️.rs"]
pub mod original_retirement;

pub use original_retirement::{FlowVcsCloseDemands,FlowVcsCloseFailure};
use original_retirement::{FlowVcsClosingOwner,FlowVcsRetirement,owner_demands,owner_step,admit_turn,retain_owner};

#[path = "../📝️forms/🧬️schema/🦀️.rs"]
pub mod forms;

//// 🪪️ Admits one caller-owned identity authority over the plugin observer and runs `operation` under it; the receipt returns only after the call finished.
#[macro_export]
#[doc(hidden)]
macro_rules! history_identity {
    (|$identity:ident| $operation:expr) => {{
        let mut observer = |_: semio_framework_value::native_encoding::NativeEncodeProgress| true;
        let observer: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_> = &mut observer;
        let mut $identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1usize << 24, observer).unwrap_or_else(|_| unreachable!("flow authoring identity ceiling is a declared nonzero constant"));
        let result = $operation;
        drop($identity.pause().unwrap_or_else(|_| unreachable!("identity authority retains its receipt through one authoring call")));
        result
    }};
}

#[path = "🔤️vocabulary-census-and-digest/🦀️.rs"]
mod vocabulary_census_and_digest;
pub use vocabulary_census_and_digest::{
    flow_vcs_host_snapshot_scalar_digest, FlowVcsAuthority, FlowVcsCensus, FlowVcsClosePhase,
    FlowVcsCredits, FlowVcsFault, FlowVcsGrant, FlowVcsHandle, FlowVcsPage, FlowVcsPoll,
    FlowVcsResourceFingerprint, FlowVcsSource, FLOW_VCS_DEADLINE_MILLISECONDS, FLOW_VCS_FEATURES,
    FLOW_VCS_MAX_BYTES, FLOW_VCS_MAX_CONTROLS, FLOW_VCS_MAX_DEPTH, FLOW_VCS_MAX_EVENTS,
    FLOW_VCS_MAX_HISTORY, FLOW_VCS_MAX_ITEMS, FLOW_VCS_MAX_OPERATIONS, FLOW_VCS_MAX_OUTPUTS,
    FLOW_VCS_MAX_PAGES,
};
use vocabulary_census_and_digest::{
    flow_vcs_fixture_census, flow_vcs_synapse_census, flow_vcs_widget_census,
};

#[path = "🪟️surface-owner-action-and-cursor/🦀️.rs"]
mod surface_owner_action_and_cursor;
pub use surface_owner_action_and_cursor::{
    FlowSurfaceOwner, FlowVcsAction, FlowVcsCursorKind, FlowVcsCursorPhase, FlowVcsStage,
};
use surface_owner_action_and_cursor::FlowVcsCursor;

#[path = "🎛️retained-vcs-state-and-begin/🦀️.rs"]
mod retained_vcs_state_and_begin;
use retained_vcs_state_and_begin::{FlowVcsDocument, FlowVcsOperation};
pub use retained_vcs_state_and_begin::{FlowFixedOwners, FlowRetainedVcs};

#[path = "📄️poll-pages-cancel-and-fault/🦀️.rs"]
mod poll_pages_cancel_and_fault;

#[path = "🏁️close-protocol/🦀️.rs"]
mod close_protocol;

#[path = "🎟️admission-and-slots/🦀️.rs"]
mod admission_and_slots;

#[path = "🪜️action-cursor-impl/🦀️.rs"]
mod action_cursor_impl;
use action_cursor_impl::flow_vcs_rediscovered_handle;

#[path = "🧮️cursor-steps/🦀️.rs"]
mod cursor_steps;
use cursor_steps::{
    flow_vcs_step_document_replacement, flow_vcs_step_mutation, flow_vcs_step_rollback,
    flow_vcs_step_scan, flow_vcs_step_shift,
};

// #region 🔖️FormsBridge
pub mod forms_bridge {
    use super::{FlowHostSnapshot, Widget};

    /// 🫴️ Hands each original widget's fields directly to the selected actual producer.
    pub fn flow_host_snapshot_to_form_spec<P:crate::forms::FlowFormProducer>(host_snapshot:&FlowHostSnapshot,producer:&P)->P::Form {
     let fields=host_snapshot.widgets.iter().filter_map(|widget|match widget {
      Widget::InputSlider {id,label,value,min,max,step}=>Some(producer.slider(id,label,*value,*min,*max,*step)),
      Widget::InputNote {id,text}=>Some(producer.note(id,text)),
      Widget::InputImage {id,src}=>Some(producer.image(id,src)),
      Widget::Variable {id,name,schema}=>Some(producer.variable(id,name,schema)),
      _=>None,
     }).collect();
     producer.finish(fields)
    }

    /// 🏷️ Widget "kind" tags recognized when patching a single generation value into a raw fixture-JSON widget.
    enum WidgetPatchKind {
        InputSlider,
        InputNote,
        InputImage,
        Variable,
    }

    impl WidgetPatchKind {
        fn parse(kind: &str) -> Option<Self> {
            match kind {
                "inputSlider" => Some(Self::InputSlider),
                "inputNote" => Some(Self::InputNote),
                "inputImage" => Some(Self::InputImage),
                "variable" => Some(Self::Variable),
                _ => None,
            }
        }
    }

    pub fn apply_generation_values_to_host_snapshot(snapshot_json: &str, values: &semio_framework_pack_json::Object) -> String {
        let Ok(mut root) = semio_framework_pack_json::parse(snapshot_json, semio_framework_pack_json::JsonMemberPolicy::Reject) else {
            return snapshot_json.to_string();
        };
        let Some(widgets) = root.get_mut("widgets").and_then(|entry| entry.as_array_mut()) else {
            return snapshot_json.to_string();
        };
        for widget in widgets.iter_mut() {
            let Some(id) = widget.get("id").and_then(|entry| entry.as_str()) else {
                continue;
            };
            let Some(value) = values.get(id) else {
                continue;
            };
            let kind = widget.get("kind").and_then(|entry| entry.as_str()).unwrap_or_default();
            let patch_kind = WidgetPatchKind::parse(kind);
            let Some(object) = widget.as_object_mut() else {
                continue;
            };
            match patch_kind {
                Some(WidgetPatchKind::InputSlider) => {
                    if let Some(number) = value.as_f64() {
                        object.insert("value", semio_framework_pack_json::Value::Number(number.into()));
                    }
                }
                Some(WidgetPatchKind::InputNote) => {
                    if let Some(text) = value.as_str() {
                        object.insert("text", semio_framework_pack_json::Value::String(text.to_string()));
                    }
                }
                Some(WidgetPatchKind::InputImage) => {
                    if let Some(src) = value.as_str() {
                        object.insert("src", semio_framework_pack_json::Value::String(src.to_string()));
                    }
                }
                Some(WidgetPatchKind::Variable) => {
                    if let Some(text) = value.as_str() {
                        object.insert("name", semio_framework_pack_json::Value::String(text.to_string()));
                    }
                }
                None => {}
            }
        }
        semio_framework_pack_json::to_string(&root)
    }
}
// #endregion 🔖️FormsBridge

#[cfg(test)]
#[path = "🧪️tests/🔬️flow-canonical/🦀️.rs"]
mod flow_canonical_tests;
// #endregion 🔖️ArtifactVcs
