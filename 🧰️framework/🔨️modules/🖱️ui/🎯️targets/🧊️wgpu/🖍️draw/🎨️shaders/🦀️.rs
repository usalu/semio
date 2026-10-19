// #region shaders
//! 🧊️ WGSL shader sources for the raw wgpu UI renderer.

#[path = "🖼️ui-vector-glass-and-raster-mip-shaders/🦀️.rs"]
mod ui_vector_glass_and_raster_mip_shaders;
pub use ui_vector_glass_and_raster_mip_shaders::{
    GLASS_SHADER, RASTER_MIP_SHADER, UI_SHADER, VECTOR_SHADER,
};

#[path = "🌍️world3d-shader-and-edits/🦀️.rs"]
mod world3d_shader_and_edits;
pub use world3d_shader_and_edits::{
    world3d_authored_shader, world3d_painted_shader, WORLD3D_PAINTED_SHADER_EDITS, WORLD3D_SHADER,
};

#[path = "✨️world3d-effect-shaders/🦀️.rs"]
mod world3d_effect_shaders;
pub use world3d_effect_shaders::{
    BLUR_DOWNSAMPLE_SHADER, SCENE_BLIT_SHADER, WORLD3D_CELEBRATION_SHADER, WORLD3D_GRID_SHADER,
    WORLD3D_LINES_SHADER, WORLD3D_POSTPROCESS_SHADER, WORLD3D_TEXTURED_SHADER,
};

// #endregion shaders

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
