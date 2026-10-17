//! 🧩️ Clustering and multicriteria decision making: fuzzy c-means, Gustafson-Kessel clustering and the fuzzy AHP, TOPSIS and VIKOR methods. See [`super`].

use super::*;

// #region 🔖️FuzzyCMeans
/// 🎯️ Fuzzy c-means clustering result.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct FcmResult {
    pub centers: Vec<Vec<f64>>,
    pub membership: Vec<Vec<f64>>,
    pub objective: f64,
}

/// 🎯️ Standard fuzzy c-means clustering.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn fuzzy_c_means(data: &[Vec<f64>], k: usize, m: f64, max_iter: usize, tol: f64) -> FuzzyResult<FcmResult> {
    if data.is_empty() || k == 0 {
        return Err(FuzzyError::InvalidDomain("fcm data".into()));
    }
    let n = data.len();
    let dim = data[0].len();
    let mut membership = vec![vec![1.0 / k as f64; k]; n];
    let mut centers = data[..k.min(n)].to_vec();
    while centers.len() < k {
        centers.push(data[centers.len() % n].clone());
    }
    let mut objective = f64::INFINITY;
    for _ in 0..max_iter {
        for j in 0..k {
            for d in 0..dim {
                let mut num = 0.0;
                let mut den = 0.0;
                for i in 0..n {
                    let _dist2: f64 = (0..dim).map(|dd| (data[i][dd] - centers[j][dd]).powi(2)).sum();
                    let w = membership[i][j].powf(m);
                    num += w * data[i][d];
                    den += w;
                }
                centers[j][d] = if den.abs() < 1e-12 { centers[j][d] } else { num / den };
            }
        }
        let mut new_obj = 0.0;
        for i in 0..n {
            for j in 0..k {
                let dist2: f64 = (0..dim).map(|d| (data[i][d] - centers[j][d]).powi(2)).sum::<f64>().max(1e-12);
                membership[i][j] = 1.0
                    / (0..k)
                        .map(|c| {
                            let dist2c: f64 = (0..dim).map(|d| (data[i][d] - centers[c][d]).powi(2)).sum::<f64>().max(1e-12);
                            (dist2 / dist2c).powf(1.0 / (m - 1.0))
                        })
                        .sum::<f64>();
                new_obj += membership[i][j].powf(m) * dist2;
            }
        }
        if (objective - new_obj).abs() < tol {
            objective = new_obj;
            break;
        }
        objective = new_obj;
    }
    Ok(FcmResult { centers, membership, objective })
}

/// 🎯️ Gustafson–Kessel fuzzy clustering with adaptive covariance per cluster.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn gustafson_kessel(data: &[Vec<f64>], k: usize, m: f64, max_iter: usize) -> FuzzyResult<FcmResult> {
    let mut result = fuzzy_c_means(data, k, m, max_iter, 1e-5)?;
    let n = data.len();
    let dim = data[0].len();
    for _ in 0..max_iter {
        let mut covariances = vec![MatD::identity(dim); k];
        for j in 0..k {
            let mut cov = MatD::zeros(dim, dim);
            let mut den = 0.0;
            for i in 0..n {
                let w = result.membership[i][j].powf(m);
                den += w;
                for a in 0..dim {
                    for b in 0..dim {
                        cov.add_at(a, b, w * (data[i][a] - result.centers[j][a]) * (data[i][b] - result.centers[j][b]));
                    }
                }
            }
            if den > 1e-12 {
                for a in 0..dim {
                    for b in 0..dim {
                        covariances[j].set(a, b, cov.get(a, b) / den + if a == b { 1e-6 } else { 0.0 });
                    }
                }
            }
        }
        for i in 0..n {
            for j in 0..k {
                let mut dist2 = 0.0;
                let diff: Vec<f64> = (0..dim).map(|d| data[i][d] - result.centers[j][d]).collect();
                for a in 0..dim {
                    for b in 0..dim {
                        dist2 += diff[a] * covariances[j].get(a, b) * diff[b];
                    }
                }
                result.membership[i][j] = 1.0
                    / (0..k)
                        .map(|c| {
                            let mut d2 = 0.0;
                            let diff: Vec<f64> = (0..dim).map(|d| data[i][d] - result.centers[c][d]).collect();
                            for a in 0..dim {
                                for b in 0..dim {
                                    d2 += diff[a] * covariances[c].get(a, b) * diff[b];
                                }
                            }
                            (dist2.max(1e-12) / d2.max(1e-12)).powf(1.0 / (m - 1.0))
                        })
                        .sum::<f64>();
            }
        }
    }
    Ok(result)
}
// #endregion 🔖️FuzzyCMeans

// #region 🔖️MulticriteriaDecision
/// 📊️ Fuzzy AHP pairwise comparison matrix.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct FuzzyAhp {
    pub matrix: Vec<Vec<FuzzyNumber>>,
}

impl FuzzyAhp {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(matrix: Vec<Vec<FuzzyNumber>>) -> Self {
        Self { matrix }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn weights(&self) -> Vec<f64> {
        let n = self.matrix.len();
        let mut scores = vec![0.0; n];
        for i in 0..n {
            for j in 0..n {
                scores[i] += self.matrix[i][j].defuzzify_centroid(32);
            }
        }
        let sum: f64 = scores.iter().sum();
        if sum.abs() < 1e-12 {
            vec![1.0 / n as f64; n]
        } else {
            scores.into_iter().map(|s| s / sum).collect()
        }
    }
}

/// 📊️ Fuzzy TOPSIS decision over alternatives.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct FuzzyTopsis {
    pub alternatives: Vec<String>,
    pub criteria: Vec<String>,
    pub decision_matrix: Vec<Vec<FuzzyNumber>>,
    pub weights: Vec<f64>,
    pub benefit: Vec<bool>,
}

impl FuzzyTopsis {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn rank(&self) -> Vec<(usize, f64)> {
        let m = self.alternatives.len();
        let n = self.criteria.len();
        let mut crisp = vec![vec![0.0; n]; m];
        for i in 0..m {
            for j in 0..n {
                crisp[i][j] = self.decision_matrix[i][j].defuzzify_centroid(32);
            }
        }
        let mut norm = vec![vec![0.0; n]; m];
        for j in 0..n {
            let denom = (0..m).map(|i| crisp[i][j] * crisp[i][j]).sum::<f64>().sqrt().max(1e-12);
            for i in 0..m {
                norm[i][j] = crisp[i][j] / denom * self.weights[j];
            }
        }
        let ideal_pos: Vec<f64> = (0..n).map(|j| if self.benefit[j] { (0..m).map(|i| norm[i][j]).fold(f64::NEG_INFINITY, f64::max) } else { (0..m).map(|i| norm[i][j]).fold(f64::INFINITY, f64::min) }).collect();
        let ideal_neg: Vec<f64> = (0..n).map(|j| if self.benefit[j] { (0..m).map(|i| norm[i][j]).fold(f64::INFINITY, f64::min) } else { (0..m).map(|i| norm[i][j]).fold(f64::NEG_INFINITY, f64::max) }).collect();
        let mut scores = Vec::new();
        for i in 0..m {
            let d_pos: f64 = (0..n).map(|j| (norm[i][j] - ideal_pos[j]).powi(2)).sum::<f64>().sqrt();
            let d_neg: f64 = (0..n).map(|j| (norm[i][j] - ideal_neg[j]).powi(2)).sum::<f64>().sqrt();
            scores.push((i, d_neg / (d_pos + d_neg + 1e-12)));
        }
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scores
    }
}

/// 📊️ Fuzzy VIKOR compromise ranking.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct FuzzyVikor {
    pub alternatives: Vec<String>,
    pub criteria: Vec<String>,
    pub decision_matrix: Vec<Vec<FuzzyNumber>>,
    pub weights: Vec<f64>,
    pub benefit: Vec<bool>,
    pub v: f64,
}

impl FuzzyVikor {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn rank(&self) -> Vec<(usize, f64)> {
        let m = self.alternatives.len();
        let n = self.criteria.len();
        let crisp = self.decision_matrix.iter().map(|row| row.iter().map(|cell| cell.defuzzify_centroid(32)).collect::<Vec<_>>()).collect::<Vec<_>>();
        let f_star: Vec<f64> = (0..n).map(|j| if self.benefit[j] { (0..m).map(|i| crisp[i][j]).fold(f64::NEG_INFINITY, f64::max) } else { (0..m).map(|i| crisp[i][j]).fold(f64::INFINITY, f64::min) }).collect();
        let _f_minus: Vec<f64> = (0..n).map(|j| if self.benefit[j] { (0..m).map(|i| crisp[i][j]).fold(f64::INFINITY, f64::min) } else { (0..m).map(|i| crisp[i][j]).fold(f64::NEG_INFINITY, f64::max) }).collect();
        let mut s = vec![0.0_f64; m];
        let mut r = vec![0.0_f64; m];
        for i in 0..m {
            for j in 0..n {
                let d = if self.benefit[j] { (f_star[j] - crisp[i][j]).max(0.0) } else { (crisp[i][j] - f_star[j]).max(0.0) };
                s[i] += self.weights[j] * d;
                r[i] = r[i].max(self.weights[j] * d);
            }
        }
        let s_star = s.iter().copied().fold(f64::INFINITY, f64::min);
        let s_minus = s.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let r_star = r.iter().copied().fold(f64::INFINITY, f64::min);
        let r_minus = r.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let mut q = Vec::new();
        for i in 0..m {
            let qi = self.v * (s[i] - s_star) / (s_minus - s_star + 1e-12) + (1.0 - self.v) * (r[i] - r_star) / (r_minus - r_star + 1e-12);
            q.push((i, qi));
        }
        q.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        q
    }
}
// #endregion 🔖️MulticriteriaDecision
