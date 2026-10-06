//! 🖼️ Raster play app — the composite window: the main paintable 2D surface.

use crate::editor::raster::config::RasterConfig;
use crate::editor::raster::modes::edit::windows::composite::options;
use crate::editor::raster::raster_scene;
use crate::RasterSnapshot as RasterDocument;
use semio_framework_plugin::plugin_app_close_prelude::SurfaceKind as ContractSurfaceKind;
use semio_framework_plugin::scene_surface;
use semio_framework_plugin::BuiltNode;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::SurfaceKind;
use semio_framework_plugin::UiAssemblyResult;
use semio_framework_plugin::WindowKindDefinition;
use semio_framework_plugin::WindowMeasure;
use semio_framework_plugin::WindowOptions;

//#region 🔖️Constants
pub const RASTER_PLAY_WINDOW_COMPOSITE: &str = "raster-composite";
pub const RASTER_PLAY_BODY_COMPOSITE: &str = "raster.play.composite";
const RASTER_PLAY_SURFACE_COMPOSITE: &str = "raster.play.composite";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::raster::create_raster_app`. `options.measures`
/// stays empty here on purpose: raster's measures are config-derived and rebuilt per frame by
/// [`window_measures`], not frozen into the manifest.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        initial_utility_id: None,
        id: RASTER_PLAY_WINDOW_COMPOSITE.into(),
        label: LocalizedLabel::native("Composite", "Komposit"),
        body_key: RASTER_PLAY_BODY_COMPOSITE.into(),
        surface_kind: SurfaceKind::Paint2d,
        icon_id: "image".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
        interactions: Vec::new(),
    }
}

/// 🎚️ The live chrome measures for this window, collected from its `☑️options/*` components.
pub fn window_measures(config: &RasterConfig, labels: &crate::editor::raster::terminology::RasterPlayLabels) -> Vec<WindowMeasure> {
    let mut measures=vec![options::brush::measure(config,labels),options::eraser::measure(config,labels),options::bucket::measure(config,labels)];
    for measure in &mut measures {if let WindowMeasure::Group{id,children,..}=measure{
        children.push(WindowMeasure::Select{id:format!("{id}-target"),label:Some(labels.paint_target.as_str().into()),value:config.paint_target.clone(),items:[("pixels",labels.paint_pixels.as_str()),("mask",labels.paint_mask.as_str())].into_iter().map(|(value,label)|semio_framework_plugin::MeasureSelectItem{id:value.into(),value:value.into(),label:label.into()}).collect(),on_change:crate::editor::raster::raster_measure_action("setPaintTarget")});
        children.push(WindowMeasure::Slider{id:format!("{id}-mask-value"),label:Some(labels.mask_value.as_str().into()),value:f64::from(config.mask_value),min:0.0,max:255.0,step:Some(1.0),ready:None,loading:None,waiting:None,disabled:Some(config.paint_target!="mask"),on_change:crate::editor::raster::raster_measure_action("setMaskValue")});
    }}
    measures
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🎬️ Encodes the shared `Paint2dScene` behind the semantic surface contract — the app's controller is
/// resolved by the host from the owning app instance now, so the surface node carries only the scene.
pub fn render(document: &RasterDocument, config: &RasterConfig, active_utility: &str, selected_ids: &[String], hovered_id: Option<&str>) -> UiAssemblyResult<BuiltNode> {
    scene_surface(RASTER_PLAY_SURFACE_COMPOSITE, ContractSurfaceKind::Paint2d, &raster_scene(document, config, active_utility, "composite", selected_ids, hovered_id))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
