mod tests {
    use super::*;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn matmul(a: &[f64], b: &[f64], m: usize, k: usize, n: usize) -> Vec<f64> {
        let mut out = vec![0.0_f64; m * n];
        for i in 0..m {
            for j in 0..n {
                let mut sum = 0.0;
                for t in 0..k {
                    sum += a[i * k + t] * b[t * n + j];
                }
                out[i * n + j] = sum;
            }
        }
        out
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn transpose(a: &[f64], rows: usize, cols: usize) -> Vec<f64> {
        let mut out = vec![0.0_f64; rows * cols];
        for i in 0..rows {
            for j in 0..cols {
                out[j * rows + i] = a[i * cols + j];
            }
        }
        out
    }

    #[test]
    fn jacobi_matches_diagonal_matrix_eigenvalues() {
        let n = 3;
        let a = vec![5.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 9.0];
        let (eigenvalues, _) = jacobi_eigen_symmetric(&a, n).unwrap();
        assert!((eigenvalues[0] - 9.0).abs() < 1e-9);
        assert!((eigenvalues[1] - 5.0).abs() < 1e-9);
        assert!((eigenvalues[2] - 2.0).abs() < 1e-9);
    }

    #[test]
    fn jacobi_reconstructs_symmetric_matrix() {
        let n = 3;
        let a = vec![4.0, 1.0, 2.0, 1.0, 3.0, 0.5, 2.0, 0.5, 5.0];
        let (eigenvalues, eigenvectors) = jacobi_eigen_symmetric(&a, n).unwrap();
        let mut d = vec![0.0_f64; n * n];
        for i in 0..n {
            d[i * n + i] = eigenvalues[i];
        }
        let vt = transpose(&eigenvectors, n, n);
        let reconstructed = matmul(&matmul(&eigenvectors, &d, n, n, n), &vt, n, n, n);
        for i in 0..n * n {
            assert!((reconstructed[i] - a[i]).abs() < 1e-7, "index {i}: {} vs {}", reconstructed[i], a[i]);
        }
    }

    #[test]
    fn jacobi_hand_3x3_matches_known_eigenvalues() {
        // 🔐️ A = [[2,-1,0],[-1,2,-1],[0,-1,2]] has eigenvalues 2, 2±sqrt(2).
        let a = vec![2.0, -1.0, 0.0, -1.0, 2.0, -1.0, 0.0, -1.0, 2.0];
        let (eigenvalues, _) = jacobi_eigen_symmetric(&a, 3).unwrap();
        let expected = {
            let mut v = vec![2.0 + 2.0_f64.sqrt(), 2.0, 2.0 - 2.0_f64.sqrt()];
            v.sort_by(|a, b| b.partial_cmp(a).unwrap());
            v
        };
        for (a, b) in eigenvalues.iter().zip(expected.iter()) {
            assert!((a - b).abs() < 1e-8);
        }
    }

    #[test]
    fn cholesky_reconstructs_positive_definite_matrix() {
        let n = 3;
        let a = vec![4.0, 2.0, 2.0, 2.0, 5.0, 1.0, 2.0, 1.0, 6.0];
        let l = cholesky(&a, n).unwrap();
        let lt = transpose(&l, n, n);
        let reconstructed = matmul(&l, &lt, n, n, n);
        for i in 0..n * n {
            assert!((reconstructed[i] - a[i]).abs() < 1e-9);
        }
    }

    #[test]
    fn cholesky_regularizes_near_singular_matrix() {
        let a = vec![1.0, 1.0, 1.0, 1.0]; // rank-1, singular
        let result = cholesky(&a, 2);
        assert!(result.is_ok());
    }

    #[test]
    fn log_det_matches_known_determinant() {
        let a = vec![4.0, 0.0, 0.0, 9.0];
        let ld = log_det(&a, 2).unwrap();
        assert!((ld - 36.0_f64.ln()).abs() < 1e-9);
    }

    #[test]
    fn svd_reconstructs_matrix() {
        let rows = 4;
        let cols = 3;
        let a = vec![
            1.0, 0.0, 0.0, //
            0.0, 2.0, 0.0, //
            0.0, 0.0, 3.0, //
            1.0, 1.0, 1.0,
        ];
        let (u, s, v) = svd_jacobi(&a, rows, cols).unwrap();
        let mut sigma = vec![0.0_f64; cols * cols];
        for i in 0..cols {
            sigma[i * cols + i] = s[i];
        }
        let vt = transpose(&v, cols, cols);
        let reconstructed = matmul(&matmul(&u, &sigma, rows, cols, cols), &vt, rows, cols, cols);
        for i in 0..rows * cols {
            assert!((reconstructed[i] - a[i]).abs() < 1e-6, "index {i}");
        }
    }

    #[test]
    fn svd_entropy_of_equal_singular_values_is_maximal() {
        // 🔐️ identity-like: all singular values equal -> normalized entropy = 1.
        let a = vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
        let est = svd_entropy(&a, 3, 3, LogBase::Bits).unwrap();
        assert!((est.value - 3.0_f64.log2()).abs() < 1e-6);
    }

    #[test]
    fn svd_entropy_of_rank_one_matrix_is_zero() {
        let a = vec![1.0, 2.0, 3.0, 2.0, 4.0, 6.0]; // rank 1, 2x3
        let est = svd_entropy(&a, 2, 3, LogBase::Bits).unwrap();
        assert!(est.value.abs() < 1e-6);
    }

    #[test]
    fn von_neumann_entropy_of_pure_state_is_zero() {
        let density = vec![1.0, 0.0, 0.0, 0.0];
        let est = von_neumann_entropy(&density, 2, LogBase::Nats).unwrap();
        assert!(est.value.abs() < 1e-9);
    }

    #[test]
    fn von_neumann_entropy_of_maximally_mixed_state_is_log_n() {
        let n = 4;
        let mut density = vec![0.0_f64; n * n];
        for i in 0..n {
            density[i * n + i] = 1.0 / n as f64;
        }
        let est = von_neumann_entropy(&density, n, LogBase::Nats).unwrap();
        assert!((est.value - (n as f64).ln()).abs() < 1e-9);
    }

    #[test]
    fn von_neumann_clips_tiny_negative_eigenvalues() {
        let density = vec![0.5 + 1e-16, 0.5, 0.5, 0.5 - 1e-16];
        // 🔐️ near-singular; should not error, should clip.
        let result = von_neumann_entropy(&density, 2, LogBase::Nats);
        assert!(result.is_ok());
    }
}
