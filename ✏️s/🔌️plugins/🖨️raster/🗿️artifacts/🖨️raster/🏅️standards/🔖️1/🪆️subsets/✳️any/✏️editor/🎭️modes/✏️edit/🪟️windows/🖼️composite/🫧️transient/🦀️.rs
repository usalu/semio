//! 🫧️ Ephemeral local-only state of ONE exact raster Composite window: the brush or eraser stroke in flight. A streamed
//! stroke spans many dispatches (one per pointer batch), so its tool state — the statechart configuration and the open
//! `ToolTransaction` holding the one provisional `paint-stroke` leaf — lives here between them, and the window paints the
//! committed document with that leaf applied. Tool state is never config and never history; only the committed leaf is
//! (design §5, §17.2, ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING).

use semio_framework_value_derive::{FromValue, ToValue};
use std::mem::size_of;

#[derive(Clone,Debug,semio_framework_value::RetireOwned)]
pub struct RasterStrokePreview{pub base_revision:[u8;32],pub snapshot:std::sync::Arc<semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot>}
impl PartialEq for RasterStrokePreview{fn eq(&self,other:&Self)->bool{self.base_revision==other.base_revision&&std::sync::Arc::ptr_eq(&self.snapshot,&other.snapshot)}}
impl semio_framework_value::ToValue for RasterStrokePreview{fn to_value(&self)->semio_framework_value::DslValue{semio_framework_value::DslValue::object(vec![("baseRevision".into(),semio_framework_value::ToValue::to_value(&self.base_revision)),("ready".into(),semio_framework_value::ToValue::to_value(&true))])}}
impl semio_framework_value::FromValue for RasterStrokePreview{fn from_value(_value:semio_framework_value::DslValue)->Result<Self,semio_framework_value::ValueError>{Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"stroke preview requires its original retained semantic candidate"))}}

//#region 🔖️State
/// 🖌️ A window's in-flight stroke: the press it follows, the statechart configuration by stable ids, the admission seed
/// the transaction was minted under, the open `TransactionRef`, and the typed provisional `paint-stroke` leaf.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct RasterStrokeToolState {
    pub gesture: String,
    pub states: Vec<String>,
    pub authoring_seed: semio_framework_value::SharedUtf8,
    pub transaction: protocol::TransactionRef,
    pub stroke: semio_s_artifact_raster_raster_mutations_pixels::paint_stroke::PaintStroke,
    #[value(default)]
    pub preview:Option<RasterStrokePreview>,
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

semio_framework_value::artifact_retire_struct!(RasterStrokeToolState { gesture, states, authoring_seed, transaction, stroke, preview });
semio_framework_value::artifact_retire_struct!(RasterCompositeWindowTransient { stroke, closed });
//#endregion 🔖️State

//#region 🔖️Owner
semio_plugin::transient_root! {
    state: RasterCompositeWindowTransient,
    mutation: RasterCompositeWindowTransientMutation,
    diff: RasterCompositeWindowTransientDiff,
    owner: "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️composite/🫧️transient",
    kind: "set-window-transient",
    display_name: "Set Raster Composite Window Transient",
    payload_schema: "raster.compositewindowtransient",
    envelope: "s.raster.raster.compositewindowtransient",
    extension: "rastercompositewindowtransient",
    fields: { stroke: Option<Box<RasterStrokeToolState>>, closed: Option<String> },
}

impl RasterCompositeWindowTransientMutation{
    pub fn footprint(&self)->Result<protocol::one_item::ArtifactStoreOneItemFootprint,String>{
        let Self::Snapshot{transient}=self;let mut bytes=size_of::<RasterCompositeWindowTransient>();
        if transient.closed.as_ref().is_some_and(|value|value.len()>4096){return Err("raster stroke closed identity exceeds its field bound".into());}
        bytes+=transient.closed.as_ref().map_or(0,String::capacity);
        if let Some(state)=transient.stroke.as_ref(){
            let leaf=&state.stroke;
            if state.gesture.len()>4096||state.authoring_seed.len()>4096||state.transaction.id.len()>64||state.transaction.tool.len()>4096||state.states.len()>2||state.states.iter().any(|state|state.len()>64)||leaf.layer_id.len()>4096||leaf.target.len()>16||leaf.tool.len()>16||leaf.points.len()>semio_s_artifact_raster_raster_mutations_pixels::paint_stroke::RASTER_STROKE_MAXIMUM_POINTS||leaf.points.capacity()>semio_s_artifact_raster_raster_mutations_pixels::paint_stroke::RASTER_STROKE_MAXIMUM_POINTS||leaf.brush.color.len()!=4||leaf.brush.color.capacity()>4{return Err("raster stroke transient exceeds its typed field bounds".into());}
            bytes+=size_of::<RasterStrokeToolState>()+state.gesture.capacity()+state.authoring_seed.original_allocation_bytes()+state.transaction.id.capacity()+state.transaction.tool.capacity()+state.states.capacity()*size_of::<String>()+state.states.iter().map(String::capacity).sum::<usize>()+leaf.layer_id.capacity()+leaf.target.capacity()+leaf.tool.capacity()+leaf.points.capacity()*size_of::<semio_s_artifact_raster_raster_mutations_pixels::paint_stroke::RasterStrokePoint>()+leaf.brush.color.capacity()*size_of::<f64>();
        }
        let footprint=protocol::one_item::ArtifactStoreOneItemFootprint::for_ephemeral_item(bytes);footprint.is_admissible().then_some(footprint).ok_or_else(||"raster stroke transient exceeds its physical root envelope".into())
    }
    pub fn into_state(self)->RasterCompositeWindowTransient{let Self::Snapshot{transient}=self;transient}
}

semio_plugin::window_transient_registry! {
    state: RasterCompositeWindowTransient,
    mutation: RasterCompositeWindowTransientMutation,
    windows: {
        RasterCompositeWindowTransientOwner => super::RASTER_PLAY_WINDOW_COMPOSITE,
    },
}
//#endregion 🔖️Owner

pub fn cached_stroke_preview<'a>(window:&'a RasterCompositeWindowTransient,base_revision:&[u8;32])->Option<&'a semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot>{let preview=window.stroke.as_deref()?.preview.as_ref()?;(preview.base_revision==*base_revision).then_some(preview.snapshot.as_ref())}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
