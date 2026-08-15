//! 📈️ Criterion benchmarks for `semio-framework-3d`'s hot-path BREP operations.
//!
//! Run via `nx run semio-s-plugin-stdio:bench` (`bun ./📜️script.ts bench`). Each group is
//! parameterized over a scaling value (point/edge/face count) to reveal where an
//! operation's cost curve goes superlinear, not just a single-point timing.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use semio_framework_3d::engine::Vec3;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::{Brep, GeometryHandle};

// #region 🔖️Fixtures

fn box_solid(kernel: &mut Brep) -> GeometryHandle {
    kernel.box_prim_sync(1.0, 1.0, 1.0).expect("box_prim_sync")
}

fn sphere_solid(kernel: &mut Brep) -> GeometryHandle {
    kernel.sphere_prim_sync(1.0).expect("sphere_prim_sync")
}

fn torus_solid(kernel: &mut Brep) -> GeometryHandle {
    kernel.torus_prim_sync(2.0, 0.5).expect("torus_prim_sync")
}

/// 🧱️ A single connected solid whose face count grows with `boxes` via chained,
/// overlapping fuses — used to reveal per-face-count scaling for tessellation,
/// fillet, and closest-point queries (which all require a single `SolidId`, not
/// a compound, ruling out `linear_pattern_sync` as a many-face fixture).
fn multi_box_solid(kernel: &mut Brep, boxes: usize) -> GeometryHandle {
    let mut current = box_solid(kernel);
    for i in 1..boxes {
        let next = box_solid(kernel);
        let next = kernel.translate_sync(&next, [i as f64 * 0.6, 0.0, 0.0]).expect("translate_sync");
        current = kernel.fuse_sync(&current, &next).expect("fuse_sync");
    }
    current
}

fn profile_face(kernel: &mut Brep) -> GeometryHandle {
    let wire = kernel.rectangle_wire_sync(1.0, 1.0).expect("rectangle_wire_sync");
    kernel.planar_face_from_wire_sync(&wire).expect("planar_face_from_wire_sync")
}

fn straight_path(kernel: &mut Brep) -> GeometryHandle {
    kernel.line_curve_sync([0.0, 0.0, 0.0], [0.0, 0.0, 5.0]).expect("line_curve_sync")
}

fn polyline_path(kernel: &mut Brep, segments: usize) -> GeometryHandle {
    let points: Vec<Vec3> = (0..=segments).map(|i| [0.3 * (i as f64 * 0.7).sin(), 0.3 * (i as f64 * 0.7).cos(), i as f64 * (5.0 / segments as f64)]).collect();
    kernel.polyline_wire_sync(&points).expect("polyline_wire_sync")
}

fn point_cloud(n: usize) -> Vec<Vec3> {
    (0..n).map(|i| [i as f64 * 0.1, (i as f64 * 0.3).sin(), (i as f64 * 0.17).cos()]).collect()
}

fn surface_grid(rows: usize, cols: usize) -> Vec<Vec<Vec3>> {
    (0..rows).map(|r| (0..cols).map(|c| [r as f64, c as f64, (r as f64 * 0.5 + c as f64 * 0.3).sin()]).collect()).collect()
}
// #endregion 🔖️Fixtures

// #region 🔖️Primitives
fn bench_primitives(c: &mut Criterion) {
    let mut group = c.benchmark_group("primitives");
    group.bench_function("box", |b| {
        b.iter(|| {
            let mut kernel = Brep::new();
            black_box(box_solid(&mut kernel));
        })
    });
    group.bench_function("sphere", |b| {
        b.iter(|| {
            let mut kernel = Brep::new();
            black_box(sphere_solid(&mut kernel));
        })
    });
    group.finish();
}
// #endregion 🔖️Primitives

// #region 🔖️CurvesSurfaces
fn bench_curves_surfaces(c: &mut Criterion) {
    let mut group = c.benchmark_group("curves_surfaces");
    for &n in &[10usize, 100, 500] {
        let points = point_cloud(n);
        group.bench_with_input(BenchmarkId::new("interpolate_curve", n), &points, |b, points| {
            b.iter(|| {
                let mut kernel = Brep::new();
                black_box(kernel.interpolate_curve_sync(points, 3).expect("interpolate_curve_sync"));
            })
        });
    }
    for &(rows, cols) in &[(4usize, 4usize), (20, 20), (50, 50)] {
        let grid = surface_grid(rows, cols);
        group.bench_with_input(BenchmarkId::new("nurbs_surface_from_grid", rows * cols), &grid, |b, grid| {
            b.iter(|| {
                let mut kernel = Brep::new();
                black_box(kernel.nurbs_surface_from_grid_sync(grid, 3, 3).expect("nurbs_surface_from_grid_sync"));
            })
        });
    }
    group.finish();
}
// #endregion 🔖️CurvesSurfaces

// #region 🔖️Sweeps
fn bench_sweeps(c: &mut Criterion) {
    let mut group = c.benchmark_group("sweeps");
    group.bench_function("sweep_straight", |b| {
        b.iter(|| {
            let mut kernel = Brep::new();
            let profile = profile_face(&mut kernel);
            let path = straight_path(&mut kernel);
            black_box(kernel.sweep_sync(&profile, &path).expect("sweep_sync"));
        })
    });
    for &segments in &[5usize, 20, 50] {
        group.bench_with_input(BenchmarkId::new("sweep_polyline", segments), &segments, |b, &segments| {
            b.iter(|| {
                let mut kernel = Brep::new();
                let profile = profile_face(&mut kernel);
                let path = polyline_path(&mut kernel, segments);
                black_box(kernel.sweep_sync(&profile, &path).expect("sweep_sync"));
            })
        });
    }
    for &turns in &[1.0f64, 10.0, 50.0] {
        group.bench_with_input(BenchmarkId::new("helical_sweep", turns as u64), &turns, |b, &turns| {
            b.iter(|| {
                let mut kernel = Brep::new();
                let profile = profile_face(&mut kernel);
                black_box(kernel.helical_sweep_sync(&profile, [0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 2.0, 0.5, turns).expect("helical_sweep_sync"));
            })
        });
    }
    group.finish();
}
// #endregion 🔖️Sweeps

// #region 🔖️Booleans

/// 🍩️ `fuse_box_torus_mesh_fallback` isolates the cost of `boolean_mesh_sync`'s coarse-tessellation
/// fallback path (`kernel/3d/brep/rs/lib.rs:236-253`), which a torus operand forces.
///
/// 🔁️ `repeated_cut_same_torus_x10` runs repeated cuts against the *same* static torus operand — the
/// slider-drag motivating case for a coarse-mesh cache in `boolean_mesh_sync`.
fn bench_booleans(c: &mut Criterion) {
    let mut group = c.benchmark_group("booleans");
    group.bench_function("fuse_box_box", |b| {
        b.iter(|| {
            let mut kernel = Brep::new();
            let a = box_solid(&mut kernel);
            let b_solid = box_solid(&mut kernel);
            let b_solid = kernel.translate_sync(&b_solid, [0.5, 0.0, 0.0]).expect("translate_sync");
            black_box(kernel.fuse_sync(&a, &b_solid).expect("fuse_sync"));
        })
    });
    group.bench_function("cut_box_sphere", |b| {
        b.iter(|| {
            let mut kernel = Brep::new();
            let a = box_solid(&mut kernel);
            let s = sphere_solid(&mut kernel);
            black_box(kernel.cut_sync(&a, &s).expect("cut_sync"));
        })
    });
    group.bench_function("fuse_box_torus_mesh_fallback", |b| {
        b.iter(|| {
            let mut kernel = Brep::new();
            let a = box_solid(&mut kernel);
            let t = torus_solid(&mut kernel);
            black_box(kernel.fuse_sync(&a, &t).expect("fuse_sync"));
        })
    });
    group.bench_function("repeated_cut_same_torus_x10", |b| {
        b.iter(|| {
            let mut kernel = Brep::new();
            let t = torus_solid(&mut kernel);
            for i in 0..10 {
                let a = box_solid(&mut kernel);
                let a = kernel.translate_sync(&a, [i as f64 * 0.01, 0.0, 0.0]).expect("translate_sync");
                black_box(kernel.cut_sync(&a, &t).expect("cut_sync"));
            }
        })
    });
    group.finish();
}
// #endregion 🔖️Booleans

// #region 🔖️Transforms
fn bench_transforms(c: &mut Criterion) {
    let mut group = c.benchmark_group("transforms");
    for &boxes in &[1usize, 20, 60] {
        group.bench_with_input(BenchmarkId::new("translate", boxes), &boxes, |b, &boxes| {
            b.iter(|| {
                let mut kernel = Brep::new();
                let shape = multi_box_solid(&mut kernel, boxes);
                black_box(kernel.translate_sync(&shape, [1.0, 2.0, 3.0]).expect("translate_sync"));
            })
        });
    }
    group.finish();
}
// #endregion 🔖️Transforms

// #region 🔖️Features
fn bench_features(c: &mut Criterion) {
    let mut group = c.benchmark_group("features");
    for &boxes in &[1usize, 5, 15] {
        group.bench_with_input(BenchmarkId::new("fillet_all_edges", boxes), &boxes, |b, &boxes| {
            b.iter(|| {
                let mut kernel = Brep::new();
                let shape = multi_box_solid(&mut kernel, boxes);
                black_box(kernel.fillet_sync(&shape, 0.05).expect("fillet_sync"));
            })
        });
        group.bench_with_input(BenchmarkId::new("chamfer_all_edges", boxes), &boxes, |b, &boxes| {
            b.iter(|| {
                let mut kernel = Brep::new();
                let shape = multi_box_solid(&mut kernel, boxes);
                black_box(kernel.chamfer_sync(&shape, 0.05).expect("chamfer_sync"));
            })
        });
    }
    group.finish();
}
// #endregion 🔖️Features

// #region 🔖️IntersectMeasure
fn bench_intersect_measure(c: &mut Criterion) {
    let mut group = c.benchmark_group("intersect_measure");
    for &boxes in &[1usize, 20, 60] {
        group.bench_with_input(BenchmarkId::new("closest_point", boxes), &boxes, |b, &boxes| {
            b.iter(|| {
                let mut kernel = Brep::new();
                let shape = multi_box_solid(&mut kernel, boxes);
                black_box(kernel.closest_point_sync(&shape, [100.0, 100.0, 100.0]).expect("closest_point_sync"));
            })
        });
        group.bench_with_input(BenchmarkId::new("classify_point", boxes), &boxes, |b, &boxes| {
            b.iter(|| {
                let mut kernel = Brep::new();
                let shape = multi_box_solid(&mut kernel, boxes);
                black_box(kernel.classify_point_sync(&shape, [0.5, 0.5, 0.5]).expect("classify_point_sync"));
            })
        });
    }
    for &control_points in &[10usize, 200] {
        let points = point_cloud(control_points);
        group.bench_with_input(BenchmarkId::new("curve_curve_intersect", control_points), &points, |b, points| {
            b.iter(|| {
                let mut kernel = Brep::new();
                let a = kernel.interpolate_curve_sync(points, 3).expect("interpolate_curve_sync");
                let shifted: Vec<Vec3> = points.iter().map(|p| [p[0] + 0.05, p[1], p[2]]).collect();
                let bcurve = kernel.interpolate_curve_sync(&shifted, 3).expect("interpolate_curve_sync");
                black_box(kernel.curve_curve_intersect_sync(&a, &bcurve, 1e-3).expect("curve_curve_intersect_sync"));
            })
        });
    }
    group.finish();
}
// #endregion 🔖️IntersectMeasure

// #region 🔖️Tessellation
fn bench_tessellation(c: &mut Criterion) {
    let mut group = c.benchmark_group("tessellation");
    for &tolerance in &[0.5f64, 0.1, 0.01] {
        group.bench_with_input(BenchmarkId::new("box_tolerance", format!("{tolerance}")), &tolerance, |b, &tolerance| {
            let mut kernel = Brep::new();
            let shape = box_solid(&mut kernel);
            b.iter(|| black_box(kernel.tessellate_sync(&shape, tolerance).expect("tessellate_sync")))
        });
        group.bench_with_input(BenchmarkId::new("sphere_tolerance", format!("{tolerance}")), &tolerance, |b, &tolerance| {
            let mut kernel = Brep::new();
            let shape = sphere_solid(&mut kernel);
            b.iter(|| black_box(kernel.tessellate_sync(&shape, tolerance).expect("tessellate_sync")))
        });
    }
    for &boxes in &[1usize, 20, 60] {
        group.bench_with_input(BenchmarkId::new("multi_box_faces", boxes), &boxes, |b, &boxes| {
            let mut kernel = Brep::new();
            let shape = multi_box_solid(&mut kernel, boxes);
            b.iter(|| black_box(kernel.tessellate_sync(&shape, 0.1).expect("tessellate_sync")))
        });
    }
    group.finish();
}
// #endregion 🔖️Tessellation

// #region 🔖️Patterns
fn bench_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("patterns");
    for &count in &[5usize, 50, 200] {
        group.bench_with_input(BenchmarkId::new("linear_pattern", count), &count, |b, &count| {
            b.iter(|| {
                let mut kernel = Brep::new();
                let shape = box_solid(&mut kernel);
                black_box(kernel.linear_pattern_sync(&shape, [2.0, 0.0, 0.0], 2.0, count).expect("linear_pattern_sync"));
            })
        });
        group.bench_with_input(BenchmarkId::new("circular_pattern", count), &count, |b, &count| {
            b.iter(|| {
                let mut kernel = Brep::new();
                let shape = box_solid(&mut kernel);
                black_box(kernel.circular_pattern_sync(&shape, [0.0, 0.0, 1.0], count).expect("circular_pattern_sync"));
            })
        });
    }
    group.finish();
}
// #endregion 🔖️Patterns

criterion_group!(kernel_benches, bench_primitives, bench_curves_surfaces, bench_sweeps, bench_booleans, bench_transforms, bench_features, bench_intersect_measure, bench_tessellation, bench_patterns);
criterion_main!(kernel_benches);
