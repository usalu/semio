//! 🫧️ Ephemeral local-only state of ONE exact Remodeling window: the streamed import in flight (design §15, ticket
//! 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING). An import spans one dispatch per picked file or decoded video frame, so its
//! tool state lives here between them, in the window whose dispatch started it: the stream its first decodable tick
//! minted and its progress. A tick arriving after its import ended — committed, aborted, or aborted from another window —
//! is dropped, and the window's closing aborts its import with zero trace. Tool state is never config and never history;
//! only the committed import edit is.

use crate::editor::remodeling::modes::{analyze, capture, model};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️State
/// 📥️ One streamed import in flight: the stream its first decodable tick minted (`None` before), the ticks seen, the
/// ticks the host announced (`0`: unknown, a decoded video) and the sharpness scores of the last admitted frames the blur
/// gate compares the next one against (at most the engine's `BLUR_GATE_ROLLING_WINDOW`, so a tick never re-decodes a stored frame).
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct RemodelingImport {
    pub stream_id: Option<String>,
    pub done: u32,
    pub total: u32,
    pub rolling_scores: Vec<f32>,
}

/// 🫧️ A Remodeling window's transient partition: its import in flight, `None` at rest.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct RemodelingWindowTransient {
    pub import: Option<RemodelingImport>,
}

semio_framework_value::artifact_retire_struct!(RemodelingImport { stream_id, done, total, rolling_scores });
semio_framework_value::artifact_retire_struct!(RemodelingWindowTransient { import });
//#endregion 🔖️State

//#region 🔖️Owner
semio_framework_plugin::transient_root! {
    state: RemodelingWindowTransient,
    mutation: RemodelingWindowTransientMutation,
    owner: "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient",
    kind: "set-window-transient",
    display_name: "Set Remodeling Window Transient",
    payload_schema: "remodeling.windowtransient",
    envelope: "s.remodel.remodeling.windowtransient",
    extension: "remodelingwindowtransient",
}

semio_framework_plugin::window_transient_owners! {
    state: RemodelingWindowTransient,
    mutation: RemodelingWindowTransientMutation,
    windows: {
        RemodelingModelWindowTransientOwner => model::windows::model::REMODELING_PLAY_WINDOW_MAIN,
        RemodelingFramesWindowTransientOwner => capture::windows::frames::REMODELING_PLAY_WINDOW_FRAMES,
        RemodelingReportWindowTransientOwner => analyze::windows::report::REMODELING_PLAY_WINDOW_REPORT,
    },
}
//#endregion 🔖️Owner
