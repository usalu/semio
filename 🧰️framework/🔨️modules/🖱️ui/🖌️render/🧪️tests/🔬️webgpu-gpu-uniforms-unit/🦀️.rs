
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
fn world3d_globals_is_eighty_bytes() {
    assert_eq!(std::mem::size_of::<World3dGlobals>(), 80);
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
    let instance = ui_render::MeshInstance { model, color: [1.0, 0.0, 0.0, 1.0], selected: true, hovered: false };
    let gpu = World3dGpuInstance::from_instance(&instance);
    assert_eq!(gpu.model0, [0.0, 1.0, 2.0, 3.0]);
    assert_eq!(gpu.model3, [12.0, 13.0, 14.0, 15.0]);
    assert_eq!(gpu.flags, [1.0, 0.0, 0.0, 0.0]);
}
