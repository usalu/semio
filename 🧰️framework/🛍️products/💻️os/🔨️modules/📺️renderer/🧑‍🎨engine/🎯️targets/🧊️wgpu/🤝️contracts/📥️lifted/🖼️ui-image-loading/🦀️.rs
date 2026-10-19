//! 🖼️ Framework widget context: the one place a retained UI image render step builds the widget context its scenes draw with, lifted out of the Interpreter for the `contracts` crate of the renderer partition (plan section 5.2).
use super::*;

//#region UiImageLoading
/// 🪀️ One immediate-mode widget pass's context. `viewport_height` is the measured surface height in
/// logical pixels — React hands `window.innerHeight` to `resolveSelectPlacement`
/// (`🧱️elements/🔽️Select/🟦️.tsx:514`) and the kit needs the same number to flip and clamp a popup;
/// `0.0` means unmeasured and degrades to a below-the-trigger placement at natural height.
pub fn framework_widget_context<'a>(
    draw: &'a mut semio_framework_ui_draw_contract::DrawList,
    overlay: Option<&'a mut semio_framework_ui_draw_contract::DrawList>,
    atlas: &'a mut semio_framework_ui_text::FontAtlas,
    icons: Option<&'a semio_framework_ui_draw_contract::IconAtlas>,
    input: &'a mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>,
    theme: &'a Theme,
    scroll_offsets: &'a mut std::collections::HashMap<String, f32>,
    collapsed_sections: &'a mut std::collections::HashMap<String, bool>,
    open_selects: &'a mut std::collections::HashMap<String, bool>,
    interaction_maps: Option<&'a mut WidgetInteractionMaps<ActionDescriptor>>,
    viewport_height: f32,
) -> FrameworkWidgetContext<'a> {
    WidgetContext { draw, overlay, atlas, icons, input, theme, scroll_offsets, collapsed_sections, open_selects, interaction_maps, pick_clip: None, viewport_height }
}
//#endregion UiImageLoading
