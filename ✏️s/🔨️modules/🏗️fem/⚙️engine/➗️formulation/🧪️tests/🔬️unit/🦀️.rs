use super::*;

#[test]
fn gauss_1d_weights_sum_to_two() {
    for n in 1..=4 {
        let sum: f64 = gauss_1d(n).iter().map(|(_, w)| w).sum();
        assert!((sum - 2.0).abs() < 1e-12, "n={n} sum={sum}");
    }
}

#[test]
fn gauss_tri_weights_sum_to_half() {
    for n in [1, 3, 7] {
        let sum: f64 = gauss_tri(n).iter().map(|(_, _, w)| w).sum();
        assert!((sum - 0.5).abs() < 1e-12, "n={n} sum={sum}");
    }
}

#[test]
fn gauss_quad_weights_sum_to_four() {
    for n in 1..=4 {
        let sum: f64 = gauss_quad(n).iter().map(|(_, _, w)| w).sum();
        assert!((sum - 4.0).abs() < 1e-9, "n={n} sum={sum}");
    }
}

#[test]
fn shape_tri3_partition_of_unity_and_node_values() {
    let (n, _) = shape_tri3(0.2, 0.3);
    assert!((n[0] + n[1] + n[2] - 1.0).abs() < 1e-12);
    let nodes = [(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)];
    for (i, &(xi, eta)) in nodes.iter().enumerate() {
        let (n, _) = shape_tri3(xi, eta);
        for (j, &nj) in n.iter().enumerate() {
            let expected = if i == j { 1.0 } else { 0.0 };
            assert!((nj - expected).abs() < 1e-12, "tri3 node {i} shape {j} = {nj}");
        }
    }
}

#[test]
fn shape_tri6_partition_of_unity_and_node_values() {
    let (n, _) = shape_tri6(0.15, 0.35);
    let sum: f64 = n.iter().sum();
    assert!((sum - 1.0).abs() < 1e-12);
    let nodes = [(0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (0.5, 0.0), (0.5, 0.5), (0.0, 0.5)];
    for (i, &(xi, eta)) in nodes.iter().enumerate() {
        let (n, _) = shape_tri6(xi, eta);
        for (j, &nj) in n.iter().enumerate() {
            let expected = if i == j { 1.0 } else { 0.0 };
            assert!((nj - expected).abs() < 1e-10, "tri6 node {i} shape {j} = {nj}");
        }
    }
}

#[test]
fn shape_quad4_partition_of_unity_and_node_values() {
    let (n, _) = shape_quad4(0.3, -0.4);
    let sum: f64 = n.iter().sum();
    assert!((sum - 1.0).abs() < 1e-12);
    let nodes = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];
    for (i, &(xi, eta)) in nodes.iter().enumerate() {
        let (n, _) = shape_quad4(xi, eta);
        for (j, &nj) in n.iter().enumerate() {
            let expected = if i == j { 1.0 } else { 0.0 };
            assert!((nj - expected).abs() < 1e-12, "quad4 node {i} shape {j} = {nj}");
        }
    }
}

#[test]
fn shape_quad8_partition_of_unity_and_node_values() {
    let (n, _) = shape_quad8(0.25, -0.6);
    let sum: f64 = n.iter().sum();
    assert!((sum - 1.0).abs() < 1e-10);
    let nodes = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0), (0.0, -1.0), (1.0, 0.0), (0.0, 1.0), (-1.0, 0.0)];
    for (i, &(xi, eta)) in nodes.iter().enumerate() {
        let (n, _) = shape_quad8(xi, eta);
        for (j, &nj) in n.iter().enumerate() {
            let expected = if i == j { 1.0 } else { 0.0 };
            assert!((nj - expected).abs() < 1e-10, "quad8 node {i} shape {j} = {nj}");
        }
    }
}

#[test]
fn jacobian_of_axis_aligned_rectangle_is_diagonal() {
    let (lx, ly) = (4.0, 6.0);
    let coords = [[0.0, 0.0], [lx, 0.0], [lx, ly], [0.0, ly]];
    let (_, d_n) = shape_quad4(0.3, -0.2);
    let (j, det_j, _) = jacobian_2d(&coords, &d_n);
    assert!(j[0][1].abs() < 1e-12);
    assert!(j[1][0].abs() < 1e-12);
    assert!((j[0][0] - lx / 2.0).abs() < 1e-12);
    assert!((j[1][1] - ly / 2.0).abs() < 1e-12);
    assert!((det_j - lx * ly / 4.0).abs() < 1e-9);
}

#[test]
#[should_panic(expected = "gauss_1d: unsupported order")]
fn gauss_1d_panics_on_unsupported_order() {
    gauss_1d(5);
}

#[test]
#[should_panic(expected = "gauss_tri: unsupported order")]
fn gauss_tri_panics_on_unsupported_order() {
    gauss_tri(2);
}
