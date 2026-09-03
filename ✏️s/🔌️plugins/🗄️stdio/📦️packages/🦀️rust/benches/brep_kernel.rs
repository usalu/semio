//! 📈️ Owned deterministic-iteration benchmarks for the stdio BREP kernel.
//!
//! Run with `nx run semio-s-plugin-stdio:bench`. Every case uses a fixed warm-up and sample
//! count, prints the median elapsed time, and requires no benchmark runtime dependency.

use std::hint::black_box;
use std::time::Instant;

use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::{Brep, GeometryHandle, Vec3};

const SAMPLES: usize = 3;

//#region 🧰️Harness
fn measure(label: &str, mut operation: impl FnMut()) {
    operation();
    let mut samples = [0u128; SAMPLES];
    for sample in &mut samples {
        let started = Instant::now();
        operation();
        *sample = started.elapsed().as_nanos();
    }
    samples.sort_unstable();
    println!("[DEBUG] benchmark={label} samples={SAMPLES} median_ns={}", samples[SAMPLES / 2]);
}
//#endregion 🧰️Harness

//#region 🔖️Fixtures
fn box_solid(kernel: &mut Brep) -> GeometryHandle {
    kernel.box_prim_sync(1.0, 1.0, 1.0).expect("box")
}

fn sphere_solid(kernel: &mut Brep) -> GeometryHandle {
    kernel.sphere_prim_sync(1.0).expect("sphere")
}

fn torus_solid(kernel: &mut Brep) -> GeometryHandle {
    kernel.torus_prim_sync(2.0, 0.5).expect("torus")
}

fn multi_box_solid(kernel: &mut Brep, boxes: usize) -> GeometryHandle {
    let mut current = box_solid(kernel);
    for index in 1..boxes {
        let next = box_solid(kernel);
        let next = kernel.translate_sync(&next, [index as f64 * 0.6, 0.0, 0.0]).expect("translate");
        current = kernel.fuse_sync(&current, &next).expect("fuse");
    }
    current
}

fn profile_face(kernel: &mut Brep) -> GeometryHandle {
    let wire = kernel.rectangle_wire_sync(1.0, 1.0).expect("rectangle");
    kernel.planar_face_from_wire_sync(&wire).expect("face")
}

fn polyline_path(kernel: &mut Brep, segments: usize) -> GeometryHandle {
    let points = (0..=segments).map(|index| [0.3 * (index as f64 * 0.7).sin(), 0.3 * (index as f64 * 0.7).cos(), index as f64 * (5.0 / segments as f64)]).collect::<Vec<_>>();
    kernel.polyline_wire_sync(&points).expect("polyline")
}

fn point_cloud(count: usize) -> Vec<Vec3> {
    (0..count).map(|index| [index as f64 * 0.1, (index as f64 * 0.3).sin(), (index as f64 * 0.17).cos()]).collect()
}

fn surface_grid(rows: usize, columns: usize) -> Vec<Vec<Vec3>> {
    (0..rows).map(|row| (0..columns).map(|column| [row as f64, column as f64, (row as f64 * 0.5 + column as f64 * 0.3).sin()]).collect()).collect()
}
//#endregion 🔖️Fixtures

//#region 📈️Cases
fn primitives() {
    measure("primitives/box", || {
        black_box(box_solid(&mut Brep::new()));
    });
    measure("primitives/sphere", || {
        black_box(sphere_solid(&mut Brep::new()));
    });
}

fn curves_and_surfaces() {
    for count in [10, 100, 500] {
        let points = point_cloud(count);
        measure(&format!("curves/interpolate/{count}"), || {
            black_box(Brep::new().interpolate_curve_sync(&points, 3).expect("curve"));
        });
    }
    for (rows, columns) in [(4, 4), (20, 20), (50, 50)] {
        let grid = surface_grid(rows, columns);
        measure(&format!("surfaces/grid/{}", rows * columns), || {
            black_box(Brep::new().nurbs_surface_from_grid_sync(&grid, 3, 3).expect("surface"));
        });
    }
}

fn sweeps() {
    measure("sweeps/straight", || {
        let mut kernel = Brep::new();
        let profile = profile_face(&mut kernel);
        let path = kernel.line_curve_sync([0.0, 0.0, 0.0], [0.0, 0.0, 5.0]).expect("line");
        black_box(kernel.sweep_sync(&profile, &path).expect("sweep"));
    });
    for segments in [5, 20, 50] {
        measure(&format!("sweeps/polyline/{segments}"), || {
            let mut kernel = Brep::new();
            let profile = profile_face(&mut kernel);
            let path = polyline_path(&mut kernel, segments);
            black_box(kernel.sweep_sync(&profile, &path).expect("sweep"));
        });
    }
}

fn booleans() {
    measure("booleans/fuse-box-box", || {
        let mut kernel = Brep::new();
        let left = box_solid(&mut kernel);
        let right = box_solid(&mut kernel);
        let right = kernel.translate_sync(&right, [0.5, 0.0, 0.0]).expect("translate");
        black_box(kernel.fuse_sync(&left, &right).expect("fuse"));
    });
    measure("booleans/cut-box-sphere", || {
        let mut kernel = Brep::new();
        let left = box_solid(&mut kernel);
        let right = sphere_solid(&mut kernel);
        black_box(kernel.cut_sync(&left, &right).expect("cut"));
    });
    measure("booleans/fuse-box-torus", || {
        let mut kernel = Brep::new();
        let left = box_solid(&mut kernel);
        let right = torus_solid(&mut kernel);
        black_box(kernel.fuse_sync(&left, &right).expect("fuse"));
    });
}

fn transforms_and_features() {
    for boxes in [1, 20, 60] {
        measure(&format!("transforms/translate/{boxes}"), || {
            let mut kernel = Brep::new();
            let shape = multi_box_solid(&mut kernel, boxes);
            black_box(kernel.translate_sync(&shape, [1.0, 2.0, 3.0]).expect("translate"));
        });
    }
    for boxes in [1, 5, 15] {
        measure(&format!("features/fillet/{boxes}"), || {
            let mut kernel = Brep::new();
            let shape = multi_box_solid(&mut kernel, boxes);
            black_box(kernel.fillet_sync(&shape, 0.05).expect("fillet"));
        });
        measure(&format!("features/chamfer/{boxes}"), || {
            let mut kernel = Brep::new();
            let shape = multi_box_solid(&mut kernel, boxes);
            black_box(kernel.chamfer_sync(&shape, 0.05).expect("chamfer"));
        });
    }
}

fn queries_and_tessellation() {
    for boxes in [1, 20, 60] {
        measure(&format!("queries/closest/{boxes}"), || {
            let mut kernel = Brep::new();
            let shape = multi_box_solid(&mut kernel, boxes);
            black_box(kernel.closest_point_sync(&shape, [100.0, 100.0, 100.0]).expect("closest"));
        });
        measure(&format!("queries/classify/{boxes}"), || {
            let mut kernel = Brep::new();
            let shape = multi_box_solid(&mut kernel, boxes);
            black_box(kernel.classify_point_sync(&shape, [0.5, 0.5, 0.5]).expect("classify"));
        });
    }
    for tolerance in [0.5, 0.1, 0.01] {
        measure(&format!("tessellation/sphere/{tolerance}"), || {
            let mut kernel = Brep::new();
            let shape = sphere_solid(&mut kernel);
            black_box(kernel.tessellate_sync(&shape, tolerance).expect("tessellate"));
        });
    }
}

fn patterns() {
    for count in [5, 50, 200] {
        measure(&format!("patterns/linear/{count}"), || {
            let mut kernel = Brep::new();
            let shape = box_solid(&mut kernel);
            black_box(kernel.linear_pattern_sync(&shape, [2.0, 0.0, 0.0], 2.0, count).expect("linear pattern"));
        });
        measure(&format!("patterns/circular/{count}"), || {
            let mut kernel = Brep::new();
            let shape = box_solid(&mut kernel);
            black_box(kernel.circular_pattern_sync(&shape, [0.0, 0.0, 1.0], count).expect("circular pattern"));
        });
    }
}
//#endregion 📈️Cases

fn main() {
    primitives();
    curves_and_surfaces();
    sweeps();
    booleans();
    transforms_and_features();
    queries_and_tessellation();
    patterns();
}
