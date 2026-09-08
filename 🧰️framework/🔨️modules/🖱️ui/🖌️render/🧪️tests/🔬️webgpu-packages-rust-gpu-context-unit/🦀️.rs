
use super::*;

#[test]
fn known_formats_map_to_their_marker() {
    assert_eq!(surface_format_marker(wgpu::TextureFormat::Bgra8UnormSrgb), Some(SurfaceFormat::Bgra8UnormSrgb));
    assert_eq!(surface_format_marker(wgpu::TextureFormat::Rgba8UnormSrgb), Some(SurfaceFormat::Rgba8UnormSrgb));
    assert_eq!(surface_format_marker(wgpu::TextureFormat::Rgba16Float), Some(SurfaceFormat::Rgba16Float));
}

#[test]
fn unmarked_format_is_none() {
    assert_eq!(surface_format_marker(wgpu::TextureFormat::Rgba8Unorm), None);
}

#[test]
fn picks_first_non_srgb_format() {
    let formats = [wgpu::TextureFormat::Bgra8UnormSrgb, wgpu::TextureFormat::Bgra8Unorm, wgpu::TextureFormat::Rgba8Unorm];
    assert_eq!(pick_surface_format(&formats), wgpu::TextureFormat::Bgra8Unorm);
}

#[test]
fn falls_back_to_first_format_when_all_are_srgb() {
    let formats = [wgpu::TextureFormat::Bgra8UnormSrgb];
    assert_eq!(pick_surface_format(&formats), wgpu::TextureFormat::Bgra8UnormSrgb);
}

#[test]
fn srgb_view_format_adds_suffix_to_a_non_srgb_base() {
    assert_eq!(srgb_view_format(wgpu::TextureFormat::Bgra8Unorm), wgpu::TextureFormat::Bgra8UnormSrgb);
    assert_eq!(srgb_view_format(wgpu::TextureFormat::Bgra8UnormSrgb), wgpu::TextureFormat::Bgra8UnormSrgb);
}
