//! 🔎️ wgpu render functions for the Select element — extracted from `widgets` mod's inline body
//! (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired as a CRATE-ROOT sibling module of
//! `crate::wgpu::widgets` (declared `#[cfg(feature = "wgpu-engine")] #[path = "..."] mod select;` right before
//! `pub mod widgets` in lib.rs — deliberately NOT nested inside `widgets { }`, since rustc resolves a
//! nested inline-module's `#[path]` as if the parent had its own on-disk directory, which fails for a
//! genuinely inline `mod widgets { }` block). `widgets` mod pulls these back in via
//! `use crate::wgpu::select::{render_select, render_select_menu};` so its own unqualified call sites keep
//! working. `crate::wgpu::widgets::{...}` reaches the sibling items this needs (`WidgetContext`,
//! `SelectItem`, `draw_text`, `draw_text_on`); `crate::wgpu::chrome`/`crate::wgpu::input`/`crate::wgpu::theme` are the
//! other top-level engine mods `widgets` itself also depends on.

use crate::wgpu::chrome::push_control_border;
use crate::wgpu::input::{HitKind, HitTarget};
use crate::wgpu::theme::Level;
use crate::wgpu::widgets::{draw_text, draw_text_on, SelectItem, WidgetContext};

pub(crate) fn render_select<E: Clone>(id: &str, value: &str, items: &[SelectItem], placeholder: Option<&str>, bounds: crate::wgpu::geometry::Rect, ctx: &mut WidgetContext<'_, E>) {
    let open = *ctx.open_selects.get(id).unwrap_or(&false);
    let hovered = ctx.input.hovered_id.as_deref() == Some(id);
    let bg = if hovered { ctx.theme.button_hover } else { ctx.theme.input_bg };
    push_control_border(ctx.draw, bounds, ctx.theme, ctx.theme.border_normal, bg);
    let label = items.iter().find(|item| item.value == value).map_or(placeholder.unwrap_or("Select…"), |item| item.label.as_str());
    draw_text(ctx, label, bounds.x + ctx.theme.padding_standard, bounds.y + (bounds.h + ctx.theme.font_size_body) * 0.5 - 2.0, ctx.theme.font_size_body, ctx.theme.text);
    if let Some(icons) = ctx.icons {
        crate::wgpu::chrome::push_icon(
            ctx.draw,
            icons,
            "chevron-down",
            bounds.x + bounds.w - ctx.theme.padding_standard - crate::wgpu::chrome::ICON_TINY,
            bounds.y + (bounds.h - crate::wgpu::chrome::ICON_TINY) * 0.5,
            crate::wgpu::chrome::ICON_TINY,
            ctx.theme.text_element,
        );
    }
    ctx.input.register_hit(HitTarget { rect: bounds, event: None, control_id: Some(id.to_string()), kind: HitKind::Select, drag_axis: None, drag_data: None });
    if open {
        render_select_menu(id, value, items, bounds, ctx);
    }
}

pub(crate) fn render_select_menu<E: Clone>(id: &str, value: &str, items: &[SelectItem], bounds: crate::wgpu::geometry::Rect, ctx: &mut WidgetContext<'_, E>) {
    let item_h = ctx.theme.control_height;
    let menu_h = items.len() as f32 * item_h + 4.0;
    let menu = crate::wgpu::geometry::Rect::new(bounds.x, bounds.y + bounds.h + 2.0, bounds.w, menu_h);
    let mut render_rows = |draw: &mut crate::wgpu::draw::DrawList| {
        draw.push_glass([menu.x, menu.y, menu.w, menu.h], ctx.theme.border_radius, ctx.theme.glass(Level::Menu));
        for (index, item) in items.iter().enumerate() {
            let row = crate::wgpu::geometry::Rect::new(menu.x + 2.0, menu.y + 2.0 + index as f32 * item_h, menu.w - 4.0, item_h);
            let row_hovered = ctx.input.hit_at(ctx.input.pointer_x, ctx.input.pointer_y).and_then(|h| h.control_id.as_deref()) == Some(&format!("{id}.item.{}", item.value));
            if row_hovered || item.value == value {
                draw.push_rounded([row.x, row.y, row.w, row.h], ctx.theme.row_hover, ctx.theme.border_radius);
            }
            draw_text_on(draw, ctx.atlas, &item.label, row.x + 8.0, row.y + 18.0, ctx.theme.font_size_body, ctx.theme.text);
            ctx.input.register_hit(HitTarget { rect: row, event: None, control_id: Some(format!("{id}.item.{}", item.value)), kind: HitKind::DropdownItem, drag_axis: None, drag_data: None });
        }
    };
    if let Some(overlay) = ctx.overlay.as_deref_mut() {
        render_rows(overlay);
    } else {
        render_rows(ctx.draw);
    }
}
