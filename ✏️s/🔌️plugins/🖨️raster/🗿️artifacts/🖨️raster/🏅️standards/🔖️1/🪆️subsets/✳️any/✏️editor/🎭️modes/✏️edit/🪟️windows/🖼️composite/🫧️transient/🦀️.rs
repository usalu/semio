//! 🫧️ Ephemeral local-only state of ONE exact raster Composite window: the brush or eraser stroke in flight. A streamed
//! stroke spans many dispatches (one per pointer batch), so its tool state — the statechart configuration and the open
//! `ToolTransaction` holding the one provisional `paint-stroke` leaf — lives here between them, and the window paints the
//! committed document with that leaf applied. Tool state is never config and never history; only the committed leaf is
//! (design §5, §17.2, ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING).

use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️State
/// 🖌️ A window's in-flight stroke: the press it follows, the statechart configuration by stable ids, the admission seed
/// the transaction was minted under, the open `TransactionRef`, and the one provisional `paint-stroke` leaf in value form.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct RasterStrokeToolState {
    pub gesture: String,
    pub states: Vec<String>,
    pub authoring_seed: String,
    pub transaction: protocol::TransactionRef,
    pub stroke: semio_framework_value::DslValue,
}

/// 🫧️ The Composite window's transient partition: the stroke in flight (`None` at rest) and the press the window last
/// closed, so a late tick of a committed or cancelled stroke leaves zero trace. Boxed: a gesture state exceeds the
/// ephemeral ownership transfer's inline bound.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct RasterCompositeWindowTransient {
    pub stroke: Option<Box<RasterStrokeToolState>>,
    pub closed: Option<String>,
}

semio_framework_value::artifact_retire_struct!(RasterStrokeToolState { gesture, states, authoring_seed, transaction, stroke });
semio_framework_value::artifact_retire_struct!(RasterCompositeWindowTransient { stroke, closed });
//#endregion 🔖️State

//#region 🔖️Owner
semio_framework_plugin::transient_root! {
    state: RasterCompositeWindowTransient,
    mutation: RasterCompositeWindowTransientMutation,
    owner: "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️composite/🫧️transient",
    kind: "set-window-transient",
    display_name: "Set Raster Composite Window Transient",
    payload_schema: "raster.compositewindowtransient",
    envelope: "s.raster.raster.compositewindowtransient",
    extension: "rastercompositewindowtransient",
}

semio_framework_plugin::window_transient_owners! {
    state: RasterCompositeWindowTransient,
    mutation: RasterCompositeWindowTransientMutation,
    windows: {
        RasterCompositeWindowTransientOwner => super::RASTER_PLAY_WINDOW_COMPOSITE,
    },
}
//#endregion 🔖️Owner
