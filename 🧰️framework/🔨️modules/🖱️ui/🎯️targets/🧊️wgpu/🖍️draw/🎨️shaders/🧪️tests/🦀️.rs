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
