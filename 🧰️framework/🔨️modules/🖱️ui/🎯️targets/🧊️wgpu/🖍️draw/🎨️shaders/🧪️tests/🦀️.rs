use super::*;
/// 🎨️ LAW (ticket 26/09/23 session 14d, WG11): every edit that turns the lit mesh shader into the painted one finds its anchor
/// EXACTLY once — the emissive anchor once lost its match when the lit shader gained `+ in.emissive_cutoff.rgb`, the
/// `lit_color` declaration was silently dropped, and every GPU context refused `world3d_painted_shader`.
#[test]
fn every_painted_shader_edit_finds_its_anchor_in_the_lit_shader_once() {
    for (anchor, _) in WORLD3D_PAINTED_SHADER_EDITS {
        assert_eq!(WORLD3D_SHADER.matches(anchor).count(), 1, "painted shader anchor: {anchor}");
    }
    let painted = world3d_painted_shader();
    assert!(painted.contains("let lit_color = in.color.rgb * paint_color;") && painted.contains("out.uv = vertex.uv;"), "every painted lane is applied");
}

/// ☁️ The point-sprite shader draws six corners per instance and its extent law matches three's `PointsMaterial` sizes.
#[test]
fn point_sprite_shader_and_extent_follow_the_points_material_law() {
    assert!(WORLD3D_POINTS_SHADER.contains("fn vs_main") && WORLD3D_POINTS_SHADER.contains("fn fs_main") && WORLD3D_POINTS_SHADER.contains("array<vec2<f32>, 6>"));
    assert_eq!(world3d_point_extent(2.0, true, 200.0, 100.0), [0.5, 1.0, 0.0, 0.0]);
    assert_eq!(world3d_point_extent(4.0, false, 200.0, 100.0), [0.02, 0.04, 1.0, 0.0]);
}
