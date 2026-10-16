//! 🌐️ Generic 3D scene math — camera, mesh instances, screen picking and draw descriptors — re-exported name for name from the math producers `-algebra`, `-camera`, `-mesh`, `-pass` and `-picking`.
//!
//! Generic 3D scene math, orbit camera, mesh instances, screen picking, and draw descriptors —
//! the `math` region of `semio-framework-ui-scene`. Relocated verbatim from the old
//! `🖱️ui/🎬️scene/🦀️.rs` (which `ui_wgpu` used to mount directly via a relative `#[path]`,
//! ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave MESH) into this
//! standalone crate, per ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME packet
//! `scene-surface`. `ui_wgpu` now re-exports this module instead of mounting the file.
//!
//! 🚫️async: E6 sync frame construction — every fn here arrived `async` from an old blind codemod
//! with zero internal `.await` call sites (none of these fns actually suspend), which is also why
//! the pre-existing file did not compile under `--features wgpu-engine`. Stripped to plain sync
//! `fn` wholesale rather than tagging each one individually.

pub use semio_framework_ui_scene_algebra::{Mat4, Mat4Math, Vec3, Vec3Math};
pub use semio_framework_ui_scene_camera::{
    adaptive_orbit_camera_far, axis_rotate_angle, camera_grid_fade_distance, camera_grid_visible_radius, default_projection_spec, floating_origin_rebase, frame_distance_for_radius, frame_orbit_to_bounds, frame_projection_orbit_to_bounds,
    frame_projection_orbit_to_spec_bounds, grid_placement_anchor, gumball_axis_drag_plane_normal, gumball_extent, gumball_eye, gumball_project_ray_onto_axis, lod_from_camera_distance, lod_grid_band_steps_world, lod_grid_fade_alpha,
    lod_grid_step_world, lod_orbit_distance_for_camera, lod_progressive_grid_layer_key, lod_progressive_grid_layers, pick_closest_lod, pick_closest_mesh_url, projection_spec_family, projection_spec_fov_degrees, projection_spec_matrix,
    projection_spec_oblique_shear, projection_spec_orientation_look, projection_spec_project_ndc, projection_spec_project_point, projection_spec_project_segment, projection_spec_ray_from_screen, projection_spec_unproject_ndc,
    projection_spec_view_proj, quat_from_basis, ray_plane_point, ray_segment_closest, ray_segment_distance, rotate_vector, screen_half_extent, vec3_from_f64, world_projection_matched_ortho_zoom, world_projection_matched_perspective_distance,
    world_projection_ortho_zoom, world_projection_view_half_extent, Camera3d, CameraProjection3d, OrbitController, ProjectedSegment3d, RaySegmentClosest, WorldCardinalView, WorldProjectionOrientation, LOD_GRID_LAYER_OPACITY, LOD_GRID_MAJOR_QUANTUM,
    LOD_GRID_MEDIUM_QUANTUM, LOD_GRID_MICRO_QUANTUM, LOD_GRID_SMALL_QUANTUM, WORLD_FRAME_BOUNDS_MARGIN, WORLD_LOD_GRID_BASE_LOD, WORLD_LOD_GRID_COVERAGE_MARGIN, WORLD_LOD_GRID_FADE_HEIGHT_FACTOR, WORLD_LOD_GRID_FADE_STRENGTH, WORLD_LOD_GRID_MAX_LOD,
    WORLD_LOD_GRID_MEDIUM_MAX_LOD, WORLD_LOD_GRID_MICRO_MAX_LOD, WORLD_LOD_GRID_MIN_FADE_CELLS, WORLD_LOD_GRID_SMALL_MAX_LOD, WORLD_LOD_REFERENCE_FOV_DEG, WORLD_ORBIT_CAMERA_FAR_DISTANCE_FACTOR, WORLD_ORBIT_CAMERA_MIN_FAR, WORLD_ORBIT_CAMERA_NEAR,
    WORLD_ORBIT_PARALLEL_DEFAULT_ZOOM, WORLD_ORBIT_PARALLEL_ZOOM_MAX, WORLD_ORBIT_PARALLEL_ZOOM_MIN, WORLD_PROJECTION_FRAME_PADDING,
};
pub use semio_framework_ui_scene_mesh::{
    mesh3d_abort, mesh3d_abort_demands, mesh3d_abort_step, mesh3d_allocate_capacity_byte_demand, mesh3d_allocate_step, mesh3d_begin, mesh3d_begin_capacity_byte_demand, mesh3d_begin_close, mesh3d_close_demands, mesh3d_close_step,
    mesh3d_component_admission_step, mesh3d_component_capacity_byte_demand, mesh3d_move_component_references, mesh3d_read_write_u32, mesh3d_read_write_vec3, mesh3d_seal, mesh3d_terminal_is_empty, mesh3d_update_vec3, mesh3d_write_edge,
    mesh3d_write_u32, mesh3d_write_vec2, mesh3d_write_vec3, mesh3d_write_vec4, ComponentReferenceTable, ComponentSource3d, Instance3d, Mesh3dComponentAdmissionStep, Mesh3dFault, Mesh3dField, Mesh3dItem, Mesh3dItemCursor, Mesh3dLease,
    Mesh3dOwnershipStep, Mesh3dPageCursor, Mesh3dReceivingStep, Mesh3dResourceIdentity, Mesh3dResourceLoan, Mesh3dResourceOwner, Mesh3dResourceReceiving, Mesh3dSchema, Mesh3dWriteToken, SceneColorSource3d, SceneInstanceMaterial3d,
    SceneSectionRole3d, MESH3D_AUTHORITY_CAPACITY, MESH3D_AUTHORITY_PAGE_CAPACITY, MESH3D_OWNER_BYTE_CAPACITY, MESH3D_OWNER_PAGE_CAPACITY, MESH3D_PAGE_BYTES, MESH3D_RESOURCE_COMPONENT_COUNT_CAPACITY,
};
pub use semio_framework_ui_scene_pass::{
    aabb_intersects_frustum, directional_shadow_frustum_planes, directional_shadow_texture_matrix, directional_shadow_view_projection, frustum_planes, transform_aabb, FrustumPlane, LineDraw3d, LineVertex3d, ProceduralGrid3d, SceneAuthoredMaterial3d,
    SceneCurvilinear3d, SceneDraw3d, SceneLighting3d, SceneMaterial3d, SceneMaterialAlpha3d, SceneMaterialDraw3d, SceneMaterialKind3d, ScenePass3d, SceneRenderProfile3d, SceneShadow3d, SceneShadowRole3d, SceneTextureFilter3d, SceneTextureSampler3d,
    SceneTextureWrap3d, SceneViewportMask3d, TexturedDraw3d, TexturedInstance3d, ICON_SHADOW_MAP_SIZE, PROCEDURAL_GRID_CELL_THICKNESS, PROCEDURAL_GRID_FADE_STRENGTH, PROCEDURAL_GRID_SECTION_THICKNESS, WORLD_SHADOW_FAR, WORLD_SHADOW_HALF_EXTENT,
    WORLD_SHADOW_LIGHT_DISTANCE, WORLD_SHADOW_MAP_SIZE, WORLD_SHADOW_NEAR,
};
pub use semio_framework_ui_scene_picking::{
    interpolate_mesh_uv, marquee_is_crossing, marquee_is_crossing_from_path, point_in_polygon, project_point, projected_aabb_bounds, ray_aabb_slab, ray_pick_instance, ray_pick_mesh_detail, ray_triangle, rect_contains, screen_segment_distance,
    screen_select_components, screen_select_instances, RayMeshHit, SELECTION_DRAG_DIRECTION_THRESHOLD_PX,
};
