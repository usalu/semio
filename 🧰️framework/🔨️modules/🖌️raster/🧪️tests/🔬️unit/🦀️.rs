
use super::*;

/// 🔢️ Constant-seeded LCG (`next_u32`), deterministic across platforms — never `rand`.
struct Lcg(u32);
impl Lcg {
    fn next_u32(&mut self) -> u32 {
        self.0 = self.0.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        self.0
    }
    fn next_unit(&mut self) -> f32 {
        (self.next_u32() % 1000) as f32 / 1000.0
    }
}

fn seeded_square_scene(seed: u32) -> (VectorScene, [f32; 4]) {
    let mut rng = Lcg(seed);
    let mut path = BezPath::new();
    path.move_to((8.0, 8.0));
    path.line_to((24.0, 8.0));
    path.line_to((24.0, 24.0));
    path.line_to((8.0, 24.0));
    path.close_path();
    let color = [rng.next_unit(), rng.next_unit(), rng.next_unit(), 1.0];
    let mut scene = VectorScene::new();
    scene.fill(path, Affine::IDENTITY, color);
    (scene, [0.0, 0.0, 0.0, 1.0])
}

#[test]
fn vector_scene_push_order_is_stable() {
    let (scene, _) = seeded_square_scene(7);
    assert_eq!(scene.ops.len(), 1);
}

/// 🔬️ Pure fixture, no GPU: `width=32` gives an unpadded row of `4*32=128` bytes, which is NOT
/// a multiple of `wgpu::COPY_BYTES_PER_ROW_ALIGNMENT` (256) — this is exactly the case that
/// made `scene_rasterizer_renders_expected_pixel_count` fail with wgpu's own validation error
/// ("Bytes per row does not respect COPY_BYTES_PER_ROW_ALIGNMENT") before `read_pixels` learned
/// to pad the copy and strip the padding back out. `width=64` stays aligned (`256 == 256`).
/// Exercises `align_bytes_per_row`, itself gated with `SceneRasterizer` — native/host-only.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
#[test]
fn align_bytes_per_row_pads_to_wgpu_alignment() {
    assert_eq!(align_bytes_per_row(32 * 4), 256);
    assert_eq!(align_bytes_per_row(64 * 4), 256);
    assert_eq!(align_bytes_per_row(65 * 4), 512);
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
#[semio_framework_async_macros::async_test]
async fn scene_rasterizer_renders_expected_pixel_count() {
    let Ok(mut rasterizer) = SceneRasterizer::new(32, 32).await else {
        eprintln!("[DEBUG] no wgpu adapter in this environment — skipping GPU assertion");
        return;
    };
    let (scene, background) = seeded_square_scene(42);
    let pixels = rasterizer.render(&scene, background).expect("render");
    assert_eq!(pixels.len(), 32 * 32 * 4);
    let center = (16 * 32 + 16) * 4;
    assert!(pixels[center + 3] > 0, "expected the filled square to cover the center pixel");
}
