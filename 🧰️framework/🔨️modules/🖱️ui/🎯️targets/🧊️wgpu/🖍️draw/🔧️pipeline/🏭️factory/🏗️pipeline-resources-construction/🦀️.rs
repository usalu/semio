//! 🏗️ Pipeline resources construction: the single `PipelineResources::new` that builds every pipeline, atlas, target and bind group. See [`super`].

use super::*;

#[path = "🔰️base-resources-and-ui-pipelines/🦀️.rs"]
mod base_resources_and_ui_pipelines;
use base_resources_and_ui_pipelines::{build_base_resources, BaseResources, build_ui_pipelines, UiPipelineSet};

#[path = "🌍️world-geometry-pipelines/🦀️.rs"]
mod world_geometry_pipelines;
use world_geometry_pipelines::{build_world_layouts_and_targets, WorldLayouts, build_world_geometry_pipelines, WorldGeometryPipelines};

#[path = "🎭️material-pipelines/🦀️.rs"]
mod material_pipelines;
use material_pipelines::{build_material_layouts_and_shaders, MaterialLayouts, build_material_pipelines, MaterialPipelines};

#[path = "🖼️textured-grid-and-post-pipelines/🦀️.rs"]
mod textured_grid_and_post_pipelines;
use textured_grid_and_post_pipelines::{build_textured_resources, TexturedResources, build_grid_resources, GridResources, build_post_processing_resources, PostProcessingResources};

#[path = "📏️vertex-layouts-and-material-depth/🦀️.rs"]
mod vertex_layouts_and_material_depth;
use vertex_layouts_and_material_depth::{world_vertex_layout, world_instance_layout, world_surface_vertex_layout, world_surface_instance_layout, celebration_instance_layout, material_depth};

impl PipelineResources {
    pub fn new(device: &gpu_device::Device, queue: &gpu_device::Queue, format: gpu_device::TextureFormat) -> Self {
        let BaseResources { world_encoded_format, globals_bind_group_layout, ui_shader, vector_shader, world_shader, world_lines_shader, depth_state, overlay_depth_state, quad_vertex_buffer, globals_buffer, glyph_texture, glyph_sampler, icon_texture, icon_sampler, glyph_bind_group } = build_base_resources(device, format);
        let UiPipelineSet { #[cfg(any(test,feature="testkit"))] mask_pipeline, ui_pipeline, vector_pipeline } = build_ui_pipelines(device, format, &globals_bind_group_layout, &ui_shader, &vector_shader, overlay_depth_state);
        let WorldLayouts { world_bind_group_layout, world_globals_ring, world_shadow_bind_group_layout, world_shadow_target, world_pipeline_layout, world_mesh_pipeline_layout } = build_world_layouts_and_targets(device);
        let WorldGeometryPipelines { world_pipeline, world_pipeline_translucent, world_section_stencil_pipeline, world_section_cap_pipeline, world_shadow_pipeline, world_line_pipeline } = build_world_geometry_pipelines(device, format, world_encoded_format, &world_shader, &world_lines_shader, depth_state, &world_pipeline_layout, &world_mesh_pipeline_layout);
        let MaterialLayouts { blur_shader, scene_blit_shader, glass_shader, world_postprocess_shader, blur_bind_group_layout, postprocess_bind_group_layout, scene_bind_group_layout, world_authored_bind_group_layout, world_authored_white, world_authored_shader, world_authored_pipeline_layout, world_painted_shader, world_celebration_shader, world_painted_pipeline_layout, world_celebration_pipeline_layout } = build_material_layouts_and_shaders(device, format, queue, &world_bind_group_layout, &world_shadow_bind_group_layout);
        let MaterialPipelines { world_standard_translucent_pipeline, world_authored_front_pipeline, world_authored_double_translucent_pipeline, world_painted_pipeline, world_authored_painted_double_pipeline, world_authored_painted_double_translucent_pipeline, world_authored_painted_front_pipeline, world_authored_painted_front_translucent_pipeline, world_painted_pipeline_translucent, world_celebration_pipeline, world_celebration_pipeline_translucent } = build_material_pipelines(device, format, world_encoded_format, &world_shader, &world_mesh_pipeline_layout, &world_authored_shader, &world_authored_pipeline_layout, &world_painted_shader, &world_celebration_shader, &world_painted_pipeline_layout, &world_celebration_pipeline_layout);
        let TexturedResources { world_grid_shader, world_plane_vertex_buffer, world_plane_sampler, world_material_samplers, world_textured_pipeline } = build_textured_resources(device, format, world_encoded_format, &world_bind_group_layout, &scene_bind_group_layout);
        let GridResources { world_grid_uniform_buffer, world_grid_bind_group, world_grid_pipeline } = build_grid_resources(device, format, world_encoded_format, &world_bind_group_layout, &world_grid_shader);
        let PostProcessingResources { blur_globals_buffer, postprocess_globals_buffer, world_postprocess_pipeline, blur_downsample_pipeline, scene_blit_pipeline, glass_pipeline } = build_post_processing_resources(device, format, world_encoded_format, &globals_bind_group_layout, &blur_shader, &scene_blit_shader, &glass_shader, &world_postprocess_shader, &blur_bind_group_layout, &postprocess_bind_group_layout, &scene_bind_group_layout);
        let _ = queue;
        Self {
            #[cfg(any(test,feature="testkit"))]
            mask_pipeline,
            ui_pipeline,
            vector_pipeline,
            world_pipeline,
            world_pipeline_translucent,
            world_section_stencil_pipeline,
            world_section_cap_pipeline,
            world_authored_front_pipeline,
            world_authored_double_translucent_pipeline,
            world_standard_translucent_pipeline,
            world_painted_pipeline,
            world_painted_pipeline_translucent,
            world_authored_painted_double_pipeline,
            world_authored_painted_double_translucent_pipeline,
            world_authored_painted_front_pipeline,
            world_authored_painted_front_translucent_pipeline,
            world_celebration_pipeline,
            world_celebration_pipeline_translucent,
            world_shadow_pipeline,
            world_line_pipeline,
            world_grid_pipeline,
            world_textured_pipeline,
            blur_downsample_pipeline,
            scene_blit_pipeline,
            glass_pipeline,
            world_postprocess_pipeline,
            quad_vertex_buffer,
            world_plane_vertex_buffer,
            world_plane_sampler,
            world_material_samplers,
            world_grid_uniform_buffer,
            world_grid_bind_group,
            globals_buffer,
            blur_globals_buffer,
            postprocess_globals_buffer,
            world_globals_ring,
            world_bind_group_layout,
            world_shadow_bind_group_layout,
            world_shadow_target,
            blur_bind_group_layout,
            postprocess_bind_group_layout,
            scene_bind_group_layout,
            world_authored_bind_group_layout,
            world_authored_white,
            glyph_texture,
            glyph_sampler,
            icon_texture,
            icon_sampler,
            glyph_bind_group,
            bind_group_layout: globals_bind_group_layout,
            surface_scale: 1.0,
        }
    }

}
