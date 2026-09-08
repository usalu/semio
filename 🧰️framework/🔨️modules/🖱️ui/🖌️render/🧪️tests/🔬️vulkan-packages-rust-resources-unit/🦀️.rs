
use super::*;

#[test]
fn one_byte_per_pixel_routes_to_the_glyph_atlas() {
    assert_eq!(classify_atlas_upload(4, 4, 16).expect("classify"), AtlasFormat::Glyph);
}

#[test]
fn four_bytes_per_pixel_routes_to_the_icon_atlas() {
    assert_eq!(classify_atlas_upload(4, 4, 64).expect("classify"), AtlasFormat::Icon);
}

#[test]
fn an_unsupported_byte_density_is_rejected_cleanly() {
    let error = classify_atlas_upload(4, 4, 48).expect_err("3 bytes/pixel is not a supported atlas density");
    assert!(matches!(error, VulkanGraphicsError::UnsupportedAtlasChannels(3)));
}

#[test]
fn a_zero_area_request_defaults_to_glyph_without_dividing_by_zero() {
    assert_eq!(classify_atlas_upload(0, 0, 0).expect("classify"), AtlasFormat::Glyph);
}

#[test]
fn glyph_and_icon_formats_map_to_the_expected_vk_formats() {
    assert_eq!(AtlasFormat::Glyph.vk_format(), vk::Format::R8_UNORM);
    assert_eq!(AtlasFormat::Icon.vk_format(), vk::Format::R8G8B8A8_SRGB);
}

#[test]
fn a_fresh_gpu_resources_table_knows_nothing_about_an_interned_but_unapplied_texture() {
    let mut registry = ui_render::ResourceRegistry::default();
    let id = registry.intern_texture("never_applied");
    let resources = GpuResources::default();
    assert!(!resources.knows_texture(id));
}
