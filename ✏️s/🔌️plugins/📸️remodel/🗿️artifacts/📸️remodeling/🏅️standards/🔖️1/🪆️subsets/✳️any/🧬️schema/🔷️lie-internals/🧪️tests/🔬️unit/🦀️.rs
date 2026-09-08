
use super::*;
use crate::algebra::VecD;
use std::f64::consts::PI;

fn lcg(state: &mut u64) -> f64 {
    *state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    ((*state >> 11) as f64 / (1_u64 << 53) as f64) * 2.0 - 1.0
}

fn vec3_close(a: [f64; 3], b: [f64; 3], tol: f64) -> bool {
    (0..3).all(|k| (a[k] - b[k]).abs() < tol)
}

fn vecn_close(a: &[f64], b: &[f64], tol: f64) -> bool {
    a.iter().zip(b).all(|(x, y)| (x - y).abs() < tol)
}

fn mat_close(a: &Mat3d, b: &Mat3d, tol: f64) -> bool {
    (0..3).all(|c| (0..3).all(|r| (a.cols[c][r] - b.cols[c][r]).abs() < tol))
}

#[test]
fn so3_exp_log_round_trips_small_moderate_and_near_pi() {
    let axis = vec3d_normalize([1.0, 2.0, -2.0]);
    let cases = [[1e-9, -2e-9, 1.5e-9], [0.3, -0.4, 0.5], [1.2, 0.7, -0.9], vec3_scale(axis, PI - 1e-4), vec3_scale(axis, PI - 1e-6)];
    for w in cases {
        assert!(vec3_close(So3::exp(w).log(), w, 1e-9), "failed for {w:?}");
    }
}

#[test]
fn so3_log_matrix_round_trip_at_pi() {
    let axis = vec3d_normalize([0.3, -0.5, 0.81]);
    let r = So3::exp(vec3_scale(axis, PI));
    let back = So3::exp(r.log());
    assert!(mat_close(&r.0, &back.0, 1e-9));
}

#[test]
fn so3_compose_inverse_and_hat_vee_round_trip() {
    let r = So3::exp([0.4, -0.9, 0.2]);
    assert!(mat_close(&r.semio_compose_rs(&r.inverse()).0, &Mat3d::IDENTITY, 1e-12));
    let w = [0.7, -0.2, 1.4];
    assert!(vec3_close(So3::vee(&So3::hat(w)), w, 0.0_f64.max(1e-15)));
    let v = [0.5, -1.0, 2.0];
    assert!(vec3_close(So3::hat(w).mul_vec3(v), vec3d_cross(w, v), 1e-12));
}

#[test]
fn quat_matrix_round_trip_covers_all_shepperd_branches() {
    let axes = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0], vec3d_normalize([0.6, 0.64, 0.48])];
    for axis in axes {
        for angle in [0.2, PI - 1e-3] {
            let half = angle / 2.0;
            let q = Quatd { w: half.cos(), x: half.sin() * axis[0], y: half.sin() * axis[1], z: half.sin() * axis[2] };
            let m = q.to_mat3d();
            assert!(mat_close(&m, &So3::exp(vec3_scale(axis, angle)).0, 1e-9));
            let q2 = Quatd::from_mat3d(&m);
            assert!((q.dot(q2).abs() - 1.0).abs() < 1e-9, "failed for axis {axis:?} angle {angle}");
        }
    }
}

#[test]
fn quat_rotate_matches_matrix_action() {
    let q = So3::exp([0.5, -0.3, 0.9]).to_quat();
    let v = [1.2, -0.7, 0.4];
    assert!(vec3_close(q.rotate(v), q.to_mat3d().mul_vec3(v), 1e-12));
    assert!(vec3_close(q.conjugate().rotate(q.rotate(v)), v, 1e-12));
}

#[test]
fn quat_slerp_hits_endpoints_and_stays_unit() {
    let a = So3::exp([0.2, 0.0, 0.0]).to_quat();
    let b = So3::exp([0.0, 1.1, 0.4]).to_quat();
    assert!((Quatd::slerp(a, b, 0.0).dot(a).abs() - 1.0).abs() < 1e-12);
    assert!((Quatd::slerp(a, b, 1.0).dot(b).abs() - 1.0).abs() < 1e-12);
    let mid = Quatd::slerp(a, b, 0.5);
    assert!((mid.dot(mid) - 1.0).abs() < 1e-12);
    assert!((mid.dot(a).abs() - mid.dot(b).abs()).abs() < 1e-9);
    let near = So3::exp([0.2 + 1e-9, 0.0, 0.0]).to_quat();
    let lerped = Quatd::slerp(a, near, 0.5);
    assert!((lerped.dot(lerped) - 1.0).abs() < 1e-12);
}

#[test]
fn jl_times_jl_inv_is_identity() {
    for w in [[1e-6, -2e-6, 1e-6], [0.4, -0.2, 0.7], [1.5, 0.9, -1.1]] {
        let product = So3::jl(w).mul(So3::jl_inv(w));
        assert!(mat_close(&product, &Mat3d::IDENTITY, 1e-9), "failed for {w:?}");
    }
}

#[test]
fn se3_exp_log_round_trips() {
    let axis = vec3d_normalize([-0.2, 0.9, 0.4]);
    let near_pi = vec3_scale(axis, PI - 1e-4);
    let cases = [[1e-9, -2e-9, 1.5e-9, 2e-9, 1e-9, -1e-9], [0.5, -0.3, 0.8, 0.4, 0.2, -0.6], [-1.2, 0.7, 0.3, 1.1, -0.8, 0.9], [0.6, -0.4, 1.0, near_pi[0], near_pi[1], near_pi[2]]];
    for xi in cases {
        assert!(vecn_close(&Se3::exp(xi).log(), &xi, 1e-9), "failed for {xi:?}");
    }
    let g = Se3::exp(cases[1]);
    let round = g.semio_compose_rs(&g.inverse());
    assert!(mat_close(&round.r.0, &Mat3d::IDENTITY, 1e-12) && vec3_close(round.t, [0.0; 3], 1e-12));
}

#[test]
fn sim3_exp_log_round_trips() {
    let axis = vec3d_normalize([0.5, -0.1, 0.86]);
    let near_pi = vec3_scale(axis, PI - 1e-4);
    let cases = [[1e-9, 2e-9, -1e-9, -2e-9, 1e-9, 1e-9, 1e-9], [0.5, -0.3, 0.8, 0.4, 0.2, -0.6, 0.0], [0.5, -0.3, 0.8, 0.4, 0.2, -0.6, 0.3], [-0.9, 0.6, 0.2, 1.2, -0.5, 0.7, -0.4], [0.3, 0.8, -0.5, near_pi[0], near_pi[1], near_pi[2], 0.25]];
    for xi in cases {
        assert!(vecn_close(&Sim3::exp(xi).log(), &xi, 1e-9), "failed for {xi:?}");
    }
    let g = Sim3::exp(cases[2]);
    let round = g.semio_compose_rs(&g.inverse());
    assert!((round.s - 1.0).abs() < 1e-12 && mat_close(&round.r.0, &Mat3d::IDENTITY, 1e-12) && vec3_close(round.t, [0.0; 3], 1e-12));
}

#[test]
fn se3_adjoint_matches_conjugation() {
    let g = Se3::exp([0.4, -0.2, 0.3, 0.5, 0.2, -0.4]);
    let xi = [0.3, 0.1, -0.2, 0.25, -0.15, 0.2];
    let adj_xi = g.adjoint().mul_vec(&VecD::from_vec(xi.to_vec()));
    let mut mapped = [0.0; 6];
    for (k, slot) in mapped.iter_mut().enumerate() {
        *slot = adj_xi.get(k);
    }
    let lhs = Se3::exp(mapped);
    let rhs = g.semio_compose_rs(&Se3::exp(xi)).semio_compose_rs(&g.inverse());
    assert!(mat_close(&lhs.r.0, &rhs.r.0, 1e-8));
    assert!(vec3_close(lhs.t, rhs.t, 1e-8));
}

#[test]
fn umeyama_recovers_planted_similarity() {
    let mut state = 12345_u64;
    let src: Vec<[f64; 3]> = (0..10).map(|_| [lcg(&mut state), lcg(&mut state), lcg(&mut state)]).collect();
    let truth = Sim3 { s: 1.7, r: So3::exp([0.3, -0.7, 0.5]), t: [0.4, -1.2, 2.5] };
    let dst: Vec<[f64; 3]> = src.iter().map(|p| truth.act(*p)).collect();
    let sim = umeyama(&src, &dst, true).expect("well-posed alignment");
    assert!((sim.s - truth.s).abs() < 1e-9);
    assert!(mat_close(&sim.r.0, &truth.r.0, 1e-9));
    assert!(vec3_close(sim.t, truth.t, 1e-9));
    for (s, d) in src.iter().zip(&dst) {
        assert!(vec3_close(sim.act(*s), *d, 1e-9));
    }
}

#[test]
fn umeyama_rigid_without_scale() {
    let mut state = 777_u64;
    let src: Vec<[f64; 3]> = (0..10).map(|_| [lcg(&mut state), lcg(&mut state), lcg(&mut state)]).collect();
    let truth = Sim3 { s: 1.0, r: So3::exp([-0.6, 0.2, 0.9]), t: [1.5, 0.3, -0.8] };
    let dst: Vec<[f64; 3]> = src.iter().map(|p| truth.act(*p)).collect();
    let sim = umeyama(&src, &dst, false).expect("well-posed alignment");
    assert!((sim.s - 1.0).abs() < 1e-15);
    assert!(mat_close(&sim.r.0, &truth.r.0, 1e-9));
    assert!(vec3_close(sim.t, truth.t, 1e-9));
}

#[test]
fn umeyama_rejects_degenerate_inputs() {
    assert!(umeyama(&[[0.0; 3], [1.0; 3]], &[[0.0; 3], [1.0; 3]], true).is_none());
    let line: Vec<[f64; 3]> = (0..10).map(|k| vec3_scale([1.0, 1.0, 1.0], k as f64)).collect();
    let shifted: Vec<[f64; 3]> = line.iter().map(|p| vec3_add(*p, [0.5, -0.2, 0.1])).collect();
    assert!(umeyama(&line, &shifted, true).is_none());
    let coincident = vec![[2.0, -1.0, 0.5]; 5];
    assert!(umeyama(&coincident, &coincident, true).is_none());
}

#[test]
fn project_to_so3_restores_perturbed_rotation() {
    let r_true = So3::exp([0.4, -0.8, 0.3]);
    assert!(mat_close(&So3::project_to_so3(&r_true.0).0, &r_true.0, 1e-9));
    let mut state = 99_u64;
    let noisy = Mat3d { cols: std::array::from_fn(|c| std::array::from_fn(|r| r_true.0.cols[c][r] + 0.05 * lcg(&mut state))) };
    let projected = So3::project_to_so3(&noisy);
    assert!(mat_close(&projected.0.transpose().mul(projected.0), &Mat3d::IDENTITY, 1e-9));
    assert!((mat3_det(projected.0) - 1.0).abs() < 1e-9);
    assert!(mat_close(&projected.0, &r_true.0, 0.15));
}

#[test]
fn se3_lerp_endpoints_and_midpoint() {
    let a = Se3::exp([0.5, -0.3, 0.8, 0.4, 0.2, -0.6]);
    let b = Se3::exp([-0.9, 0.6, 0.2, 1.2, -0.5, 0.7]);
    let at0 = se3_lerp(&a, &b, 0.0);
    let at1 = se3_lerp(&a, &b, 1.0);
    assert!(mat_close(&at0.r.0, &a.r.0, 1e-9) && vec3_close(at0.t, a.t, 1e-9));
    assert!(mat_close(&at1.r.0, &b.r.0, 1e-9) && vec3_close(at1.t, b.t, 1e-9));
    let mid = se3_lerp(&a, &b, 0.5);
    let rel_full = a.inverse().semio_compose_rs(&b).log();
    let rel_half = a.inverse().semio_compose_rs(&mid).log();
    assert!(vecn_close(&rel_half, &xi6_scale(rel_full, 0.5), 1e-9));
}

#[test]
fn se3_spline_tracks_constant_velocity() {
    let v = [0.1, -0.05, 0.2, 0.3, 0.1, -0.2];
    let poses: Vec<(f64, Se3)> = (0..7).map(|k| (k as f64, Se3::exp(xi6_scale(v, k as f64)))).collect();
    for t in [1.0, 2.5, 4.7, 5.0] {
        let spline = se3_spline(&poses, t).expect("enough poses");
        let exact = Se3::exp(xi6_scale(v, t));
        assert!(mat_close(&spline.r.0, &exact.r.0, 1e-8), "failed at t = {t}");
        assert!(vec3_close(spline.t, exact.t, 1e-8), "failed at t = {t}");
    }
    let clamped = se3_spline(&poses, -3.0).expect("enough poses");
    let start = Se3::exp(v);
    assert!(mat_close(&clamped.r.0, &start.r.0, 1e-8) && vec3_close(clamped.t, start.t, 1e-8));
    assert!(se3_spline(&poses[0..3], 1.0).is_none());
}
