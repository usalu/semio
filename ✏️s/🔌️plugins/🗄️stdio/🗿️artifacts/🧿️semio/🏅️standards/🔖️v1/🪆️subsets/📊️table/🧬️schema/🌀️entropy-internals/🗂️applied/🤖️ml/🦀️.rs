//! 🤖️ Machine-learning uncertainty measures: per-sample predictive entropy over classifier
//! outputs, BALD (epistemic) mutual information from ensemble predictions, and expected
//! calibration error.

use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::numeric::x_ln_x;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::ConfidenceInterval;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::EntropyError;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::Estimate;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::LogBase;

// #region 🔖️Predictive
/// 🤖️ Shannon entropy of each row of a row-major `[n_samples x n_classes]` probability batch.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn predictive_entropy(probs: &[f64], n_classes: usize, base: LogBase) -> Result<Vec<Estimate>, EntropyError> {
    base.validate()?;
    if n_classes == 0 {
        return Err(EntropyError::InvalidConfig { field: "n_classes", reason: "must be at least 1" });
    }
    if probs.is_empty() || !probs.len().is_multiple_of(n_classes) {
        return Err(EntropyError::ShapeMismatch { what: "probs", expected: n_classes, actual: probs.len() % n_classes.max(1) });
    }
    let n_samples = probs.len() / n_classes;
    let mut out = Vec::with_capacity(n_samples);
    for i in 0..n_samples {
        let row = &probs[i * n_classes..(i + 1) * n_classes];
        let p = semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::counts::validate_probabilities(row, semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::Tolerances::default())?;
        let nats = -p.iter().map(|&v| x_ln_x(v)).sum::<f64>();
        out.push(Estimate { value: base.from_nats(nats), base, method: "predictive_entropy", n: n_classes, n_effective: n_classes as f64, std_error: None, ci: None::<ConfidenceInterval>, warnings: Vec::new(), diagnostics: Vec::new() });
    }
    Ok(out)
}
// #endregion 🔖️Predictive

// #region 🔖️Bald
/// 🤖️ BALD mutual information per sample: `H(mean_over_members(p)) - mean_over_members(H(p))`,
/// splitting total predictive uncertainty into epistemic (this value) and aleatoric (the
/// subtracted mean-member-entropy term) components. `ensemble_probs` is row-major
/// `[n_samples][n_members][n_classes]` flattened.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn bald_mutual_information(ensemble_probs: &[f64], n_members: usize, n_classes: usize, base: LogBase) -> Result<Vec<Estimate>, EntropyError> {
    base.validate()?;
    if n_members == 0 || n_classes == 0 {
        return Err(EntropyError::InvalidConfig { field: "n_members/n_classes", reason: "must be at least 1" });
    }
    let per_sample_len = n_members * n_classes;
    if ensemble_probs.is_empty() || !ensemble_probs.len().is_multiple_of(per_sample_len) {
        return Err(EntropyError::ShapeMismatch { what: "ensemble_probs", expected: per_sample_len, actual: ensemble_probs.len() % per_sample_len.max(1) });
    }
    let n_samples = ensemble_probs.len() / per_sample_len;
    let mut out = Vec::with_capacity(n_samples);
    for s in 0..n_samples {
        let sample = &ensemble_probs[s * per_sample_len..(s + 1) * per_sample_len];
        let mut mean_probs = vec![0.0_f64; n_classes];
        let mut mean_member_entropy_nats = 0.0_f64;
        for m in 0..n_members {
            let member = &sample[m * n_classes..(m + 1) * n_classes];
            let p = semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::counts::validate_probabilities(member, semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::Tolerances::default())?;
            for c in 0..n_classes {
                mean_probs[c] += p[c] / n_members as f64;
            }
            mean_member_entropy_nats += -p.iter().map(|&v| x_ln_x(v)).sum::<f64>() / n_members as f64;
        }
        let mean_probs = semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::counts::validate_probabilities(&mean_probs, semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::Tolerances::default())?;
        let predictive_nats = -mean_probs.iter().map(|&v| x_ln_x(v)).sum::<f64>();
        let bald_nats = semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::numeric::clamp_near_zero(predictive_nats - mean_member_entropy_nats, 1e-9);

        out.push(Estimate {
            value: base.from_nats(bald_nats),
            base,
            method: "bald_mutual_information",
            n: n_members,
            n_effective: n_members as f64,
            std_error: None,
            ci: None::<ConfidenceInterval>,
            warnings: Vec::new(),
            diagnostics: vec![("predictive_entropy", base.from_nats(predictive_nats)), ("mean_member_entropy", base.from_nats(mean_member_entropy_nats))],
        });
    }
    Ok(out)
}
// #endregion 🔖️Bald

// #region 🔖️Calibration
/// 🤖️ Expected calibration error: bins predictions by confidence into `n_bins` equal-width bins
/// in `[0,1]`, and reports the confidence-weighted average `|accuracy - confidence|` per bin.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn expected_calibration_error(confidences: &[f64], correct: &[bool], n_bins: usize) -> Result<f64, EntropyError> {
    if confidences.len() != correct.len() {
        return Err(EntropyError::LengthMismatch { expected: confidences.len(), actual: correct.len() });
    }
    if confidences.is_empty() {
        return Err(EntropyError::EmptyInput { what: "confidences" });
    }
    if n_bins == 0 {
        return Err(EntropyError::InvalidConfig { field: "n_bins", reason: "must be at least 1" });
    }
    for (i, &c) in confidences.iter().enumerate() {
        if !(0.0..=1.0).contains(&c) {
            return Err(EntropyError::InvalidProbability { index: i, value: c });
        }
    }
    let mut bin_conf_sum = vec![0.0_f64; n_bins];
    let mut bin_correct_sum = vec![0.0_f64; n_bins];
    let mut bin_count = vec![0.0_f64; n_bins];
    for (&c, &ok) in confidences.iter().zip(correct.iter()) {
        let bin = ((c * n_bins as f64) as usize).min(n_bins - 1);
        bin_conf_sum[bin] += c;
        bin_correct_sum[bin] += if ok { 1.0 } else { 0.0 };
        bin_count[bin] += 1.0;
    }
    let n = confidences.len() as f64;
    let mut ece = 0.0_f64;
    for b in 0..n_bins {
        if bin_count[b] <= 0.0 {
            continue;
        }
        let avg_conf = bin_conf_sum[b] / bin_count[b];
        let accuracy = bin_correct_sum[b] / bin_count[b];
        ece += (bin_count[b] / n) * (accuracy - avg_conf).abs();
    }
    Ok(ece)
}
// #endregion 🔖️Calibration

// #region 🔖️Tests
#[cfg(test)]
include!("🧪️tests/🔬️unit/🦀️.rs");
// #endregion 🔖️Tests
