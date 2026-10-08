use super::*;
use parry3d::na::{Matrix3, SymmetricEigen};

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn reconstruct(eigen: &SymmetricEigen3) -> [[f64; 3]; 3] {
    let mut out = [[0.0; 3]; 3];
    for k in 0..3 {
        for i in 0..3 {
            for j in 0..3 {
                out[i][j] += eigen.values[k] * eigen.axes[k][i] * eigen.axes[k][j];
            }
        }
    }
    out
}

fn assert_matrix_close(a: [[f64; 3]; 3], b: [[f64; 3]; 3], tolerance: f64) {
    for i in 0..3 {
        for j in 0..3 {
            assert!((a[i][j] - b[i][j]).abs() <= tolerance, "[{i}][{j}] {} vs {}", a[i][j], b[i][j]);
        }
    }
}

#[test]
fn diagonal_matrix_sorts_values_and_keeps_axes_orthonormal() {
    let eigen = symmetric_eigen3([[5.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 3.0]]);
    assert_eq!(eigen.values, [1.0, 3.0, 5.0]);
    assert_eq!(eigen.axes[0], [0.0, 1.0, 0.0]);
    assert_eq!(eigen.axes[1], [0.0, 0.0, 1.0]);
    assert_eq!(eigen.axes[2], [1.0, 0.0, 0.0]);
}

#[test]
fn dense_matrices_match_nalgebra_and_reconstruct() {
    let cases = [
        [[4.0, 1.0, -2.0], [1.0, 2.0, 0.5], [-2.0, 0.5, 3.0]],
        [[10.0, -3.0, 2.0], [-3.0, 7.0, 4.0], [2.0, 4.0, 1.0]],
        [[1e-3, 2e-4, -3e-4], [2e-4, 5e-3, 1e-4], [-3e-4, 1e-4, 9e-3]],
        [[1e6, 3e5, -2e5], [3e5, 2e6, 4e5], [-2e5, 4e5, 3e6]],
    ];
    for matrix in cases {
        let eigen = symmetric_eigen3(matrix);
        let reference = SymmetricEigen::new(Matrix3::new(matrix[0][0], matrix[0][1], matrix[0][2], matrix[1][0], matrix[1][1], matrix[1][2], matrix[2][0], matrix[2][1], matrix[2][2]));
        let mut expected: Vec<f64> = reference.eigenvalues.iter().copied().collect();
        expected.sort_by(f64::total_cmp);
        let scale = expected.iter().fold(0.0_f64, |m, x| m.max(x.abs()));
        for (ours, theirs) in eigen.values.iter().zip(&expected) {
            assert!((ours - theirs).abs() <= 1e-12 * scale, "{ours} vs {theirs}");
        }
        assert_matrix_close(reconstruct(&eigen), matrix, 1e-12 * scale);
        for i in 0..3 {
            for j in 0..3 {
                let expected_dot = if i == j { 1.0 } else { 0.0 };
                assert!((dot(eigen.axes[i], eigen.axes[j]) - expected_dot).abs() < 1e-13);
            }
        }
        assert!(dot(cross(eigen.axes[0], eigen.axes[1]), eigen.axes[2]) > 0.999_999);
    }
}

#[test]
fn repeated_eigenvalues_keep_an_orthonormal_right_handed_basis() {
    let eigen = symmetric_eigen3([[2.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 7.0]]);
    assert_eq!(eigen.values, [2.0, 2.0, 7.0]);
    assert_eq!(eigen.axes[2], [0.0, 0.0, 1.0]);
    assert!((dot(eigen.axes[0], eigen.axes[1])).abs() < 1e-15);
    let sphere = symmetric_eigen3([[3.0, 0.0, 0.0], [0.0, 3.0, 0.0], [0.0, 0.0, 3.0]]);
    assert_eq!(sphere.values, [3.0, 3.0, 3.0]);
}

#[test]
fn parallel_axis_round_trips_and_matches_the_closed_form() {
    let cube = [[1.0 / 6.0, 0.0, 0.0], [0.0, 1.0 / 6.0, 0.0], [0.0, 0.0, 1.0 / 6.0]];
    let shifted = parallel_axis(cube, 1.0, [0.5, 0.5, 0.5]);
    assert_matrix_close(shifted, [[1.0 / 6.0 + 0.5, -0.25, -0.25], [-0.25, 1.0 / 6.0 + 0.5, -0.25], [-0.25, -0.25, 1.0 / 6.0 + 0.5]], 1e-15);
    assert_matrix_close(to_centroid(shifted, 1.0, [0.5, 0.5, 0.5]), cube, 1e-15);
}

#[test]
fn combining_two_cubes_equals_the_box_between_them() {
    let cube = |x: f64| InertiaPiece { mass: 1.0, centroid: [x, 0.0, 0.0], about_centroid: [[1.0 / 6.0, 0.0, 0.0], [0.0, 1.0 / 6.0, 0.0], [0.0, 0.0, 1.0 / 6.0]] };
    let both = combine(&[cube(0.5), cube(1.5)]).expect("mass");
    assert_eq!(both.mass, 2.0);
    assert_eq!(both.centroid, [1.0, 0.0, 0.0]);
    let (w, d, h, m) = (2.0, 1.0, 1.0, 2.0);
    assert_matrix_close(both.about_centroid, [[m * (d * d + h * h) / 12.0, 0.0, 0.0], [0.0, m * (w * w + h * h) / 12.0, 0.0], [0.0, 0.0, m * (w * w + d * d) / 12.0]], 1e-14);
    assert!(combine(&[]).is_none());
}
