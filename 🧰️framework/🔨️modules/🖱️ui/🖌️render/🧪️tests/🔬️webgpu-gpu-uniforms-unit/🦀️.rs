use super::*;

#[test]
fn ui_globals_is_sixteen_bytes() {
    assert_eq!(std::mem::size_of::<UiGlobals>(), 16);
}

#[test]
fn blur_globals_is_thirty_two_bytes() {
    assert_eq!(std::mem::size_of::<BlurGlobals>(), 32);
}

#[test]
fn world3d_globals_is_two_hundred_forty_bytes() {
    assert_eq!(std::mem::size_of::<World3dGlobals>(), 240);
}

#[test]
fn world3d_globals_pack_the_complete_environment() {
    let pass = ui_render::SurfacePass {
        camera_position: [4.0, -3.0, 8.0],
        light_dir: [-0.4, 0.8, 0.42],
        lighting: ui_render::SurfaceLighting { ambient_color: [0.1, 0.2, 0.3], ambient_intensity: 0.42, sun_color: [1.0, 0.7, 0.3], sun_intensity: 1.7, sun_enabled: true },
        neutral_material: ui_render::SurfaceMaterial { metalness: 0.63, roughness: 0.27, emissive: [0.02, 0.01, 0.06], emissive_intensity: 2.4 },
        shadow: ui_render::SurfaceShadow { enabled: true, opacity: 0.65, softness: 1.75, view_proj: [0.25; 16] },
        ..Default::default()
    };
    let globals = World3dGlobals::from_pass(&pass);
    assert_eq!(globals.camera_position, [4.0, -3.0, 8.0, 0.0]);
    assert_eq!(globals.ambient, [0.1, 0.2, 0.3, 0.42]);
    assert_eq!(globals.sun, [1.0, 0.7, 0.3, 1.7]);
    assert_eq!(globals.material, [0.63, 0.27, 2.4, 1.0]);
    assert_eq!(globals.material_emissive, [0.02, 0.01, 0.06, 0.0]);
    assert_eq!(globals.shadow_view_proj, [0.25; 16]);
    assert_eq!(globals.shadow, [1.0, 0.65, 1.75, 0.0]);
}

#[test]
fn world3d_gpu_instance_is_ninety_six_bytes() {
    assert_eq!(std::mem::size_of::<World3dGpuInstance>(), 96);
}

#[test]
fn from_instance_packs_row_major_model_and_flags() {
    let mut model = [0.0f32; 16];
    for (index, value) in model.iter_mut().enumerate() {
        *value = index as f32;
    }
    let instance = ui_render::MeshInstance { model, color: [1.0, 0.0, 0.0, 1.0], preserve_vertex_color: true, emissive_intensity: 0.2, metalness: 0.63, roughness: 0.27 };
    let gpu = World3dGpuInstance::from_instance(&instance);
    assert_eq!(gpu.model0, [0.0, 1.0, 2.0, 3.0]);
    assert_eq!(gpu.model3, [12.0, 13.0, 14.0, 15.0]);
    assert_eq!(gpu.flags, [1.0, 0.2, 0.63, 0.27]);
}
