//! 🌫️ Continuous-noise diffusion sampling: noise schedules, prediction types, classifier-free guidance, solvers, img2img and inpainting.

#![allow(async_fn_in_trait)]

use semio_framework_math_sampling_vocabulary::{CounterRng, RandomSource, RandomSources, SamplingError, StreamKey, StreamPurpose};

// #region 🔖️Diffusion
/// 🌫️ A caller-owned latent buffer plus its logical shape (e.g. `[batch, channels, height,
/// width]`). Purely descriptive — solvers and the denoiser trait operate on plain `&[f32]`/🧰️framework/🔨️modules/🧮️math/🎯️sampling`&mut
/// [f32]` slices directly (avoids an awkward "mutable slice behind a shared struct reference"
/// pattern); this type exists for callers setting up or reading back a run's buffers.
pub struct LatentView<'a> {
    pub data: &'a mut [f32],
    pub shape: [usize; 4],
}

impl LatentView<'_> {
    pub async fn len(&self) -> usize {
        self.shape.iter().product()
    }

    pub async fn is_empty(&self) -> bool {
        self.len().await == 0
    }
}

async fn linspace(a: f64, b: f64, n: usize) -> Vec<f64> {
    if n <= 1 {
        return vec![a];
    }
    (0..n).map(|i| a + (b - a) * i as f64 / (n as f64 - 1.0)).collect()
}

async fn betas_to_sigmas(betas: &[f64]) -> Vec<f64> {
    let mut alpha_bar = 1.0;
    betas
        .iter()
        .map(|&beta| {
            alpha_bar *= 1.0 - beta;
            ((1.0 - alpha_bar) / alpha_bar.max(1e-12)).sqrt()
        })
        .collect()
}

/// 🌫️ Noise-level (`sigma`) schedules. All variants produce a monotonically non-increasing
/// sequence from `sigmas(steps)[0]` (most noise) down toward `0` (a clean sample).
#[derive(Clone, PartialEq, Debug)]
pub enum NoiseSchedule {
    Linear { beta_start: f64, beta_end: f64 },
    ScaledLinear { beta_start: f64, beta_end: f64 },
    Cosine { s: f64 },
    Karras { sigma_min: f64, sigma_max: f64, rho: f64 },
    Exponential { sigma_min: f64, sigma_max: f64 },
    Polynomial { sigma_min: f64, sigma_max: f64, power: f64 },
    Custom(Vec<f64>),
}

impl NoiseSchedule {
    /// 🌫️ Produces `steps` sigma values, descending.
    pub async fn sigmas(&self, steps: usize) -> Vec<f64> {
        match self {
            Self::Linear { beta_start, beta_end } => {
                let mut s = betas_to_sigmas(&linspace(*beta_start, *beta_end, steps).await).await;
                s.reverse();
                s
            }
            Self::ScaledLinear { beta_start, beta_end } => {
                let betas: Vec<f64> = linspace(beta_start.sqrt(), beta_end.sqrt(), steps).await.iter().map(|b| b * b).collect();
                let mut s = betas_to_sigmas(&betas).await;
                s.reverse();
                s
            }
            Self::Cosine { s: s_offset } => {
                let f = |t: f64| ((t + s_offset) / (1.0 + s_offset) * core::f64::consts::FRAC_PI_2).cos().powi(2);
                let f0 = f(0.0);
                let n = steps.max(1);
                let mut out: Vec<f64> = (0..n)
                    .map(|i| {
                        let alpha_bar = (f(i as f64 / (n as f64 - 1.0).max(1.0)) / f0).clamp(1e-9, 1.0);
                        ((1.0 - alpha_bar) / alpha_bar).sqrt()
                    })
                    .collect();
                out.reverse();
                out
            }
            Self::Karras { sigma_min, sigma_max, rho } => {
                let n = steps.max(1);
                (0..n)
                    .map(|i| {
                        let t = i as f64 / (n as f64 - 1.0).max(1.0);
                        (sigma_max.powf(1.0 / rho) + t * (sigma_min.powf(1.0 / rho) - sigma_max.powf(1.0 / rho))).powf(*rho)
                    })
                    .collect()
            }
            Self::Exponential { sigma_min, sigma_max } => {
                let n = steps.max(1);
                (0..n)
                    .map(|i| {
                        let t = i as f64 / (n as f64 - 1.0).max(1.0);
                        (sigma_max.ln() + t * (sigma_min.ln() - sigma_max.ln())).exp()
                    })
                    .collect()
            }
            Self::Polynomial { sigma_min, sigma_max, power } => {
                let n = steps.max(1);
                (0..n)
                    .map(|i| {
                        let t = i as f64 / (n as f64 - 1.0).max(1.0);
                        (sigma_max.powf(1.0 / power) + t * (sigma_min.powf(1.0 / power) - sigma_max.powf(1.0 / power))).powf(*power)
                    })
                    .collect()
            }
            Self::Custom(values) => values.clone(),
        }
    }
}

/// 🌫️ What a [`Denoiser`]'s raw output represents.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PredictionType {
    Epsilon,
    VPrediction,
    Sample,
}

/// 🌫️ Converts a [`Denoiser`]'s raw prediction into a denoised (`x0`) estimate. `VPrediction` uses
/// one common (EDM-style) `sigma_data = 1` parametrization: `c_skip = 1/(sigma²+1)`, `c_out =
/// -sigma/sqrt(sigma²+1)` — other papers' exact v-prediction constants vary; this is a documented
/// choice, not a universal standard.
async fn to_denoised(prediction_type: PredictionType, x: &[f32], sigma: f64, raw: &[f32]) -> Vec<f32> {
    match prediction_type {
        PredictionType::Sample => raw.to_vec(),
        PredictionType::Epsilon => x.iter().zip(raw).map(|(&xi, &ei)| xi - (sigma as f32) * ei).collect(),
        PredictionType::VPrediction => {
            let sigma2_1 = (sigma * sigma + 1.0) as f32;
            let c_skip = 1.0 / sigma2_1;
            let c_out = -(sigma as f32) / sigma2_1.sqrt();
            x.iter().zip(raw).map(|(&xi, &vi)| c_skip * xi + c_out * vi).collect()
        }
    }
}

/// 🌫️ Which conditioning branch a [`Denoiser`] call evaluates, for classifier-free guidance.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GuidanceBranch {
    Conditional,
    Unconditional,
}

/// 🌫️ Caller-supplied model evaluation: predicts (in `prediction_type`'s parametrization) the
/// noise/sample for `latent` at noise level `sigma`. Diffusion/image sampling never runs model
/// inference itself (same non-responsibility as token sampling, § 1) — this trait is the seam.
///
/// 🔀️ Not `#[dyn_enum]`'d (O1 de-dyn, math-dedyn): this crate has zero production implementors by
/// design — real ones are always caller-supplied (a model backend this math crate cannot depend
/// on) — and its two consumers ([`eval_denoised`], [`run_diffusion`]) take `denoiser` as a
/// short-lived borrow, never store it. A closed enum would force every future caller to register
/// their type in an enum only this crate can edit; `impl Denoiser` (static generics) keeps the
/// seam genuinely open while staying fully monomorphized (see `📓️terra-math-dedyn-report.md`).
pub trait Denoiser {
    async fn prediction_type(&self) -> PredictionType;
    #[allow(clippy::too_many_arguments)]
    async fn denoise(&mut self, latent: &[f32], shape: [usize; 4], sigma: f64, step: usize, branch: GuidanceBranch, out: &mut [f32]) -> Result<(), SamplingError>;
}

/// 🌫️ Classifier-free guidance: `guided = uncond + scale * (cond - uncond)`, with optional
/// variance-rescaling (Lin et al., "Common Diffusion Noise Schedules and Sample Steps are Flawed")
/// to counter over-saturation at high `scale`.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Guidance {
    pub scale: f64,
    pub rescale: f64,
}

async fn std_dev(values: &[f32]) -> f64 {
    let n = values.len() as f64;
    if n == 0.0 {
        return 0.0;
    }
    let mean: f64 = values.iter().map(|&x| x as f64).sum::<f64>() / n;
    let var: f64 = values.iter().map(|&x| (x as f64 - mean).powi(2)).sum::<f64>() / n;
    var.sqrt()
}

impl Guidance {
    async fn combine(&self, cond: &[f32], uncond: &[f32], out: &mut [f32]) {
        for i in 0..out.len() {
            out[i] = uncond[i] + (self.scale as f32) * (cond[i] - uncond[i]);
        }
        if self.rescale > 0.0 {
            let std_cond = std_dev(cond).await;
            let std_guided = std_dev(out).await;
            if std_guided > 1e-8 {
                let factor = (std_cond / std_guided) as f32;
                let rescale = self.rescale as f32;
                for v in out.iter_mut() {
                    *v = *v * rescale * factor + *v * (1.0 - rescale);
                }
            }
        }
    }
}

async fn normal_std(rng: &mut RandomSources) -> f32 {
    let u1 = rng.next_f64_open01().await;
    let u2 = rng.next_f64().await;
    ((-2.0 * u1.ln()).sqrt() * (2.0 * core::f64::consts::PI * u2).cos()) as f32
}

async fn euler_step(x: &mut [f32], denoised: &[f32], sigma: f64, sigma_next: f64) {
    let sigma_f = sigma.max(1e-10) as f32;
    let dt = (sigma_next - sigma) as f32;
    for i in 0..x.len() {
        let d = (x[i] - denoised[i]) / sigma_f;
        x[i] += d * dt;
    }
}

async fn euler_ancestral_step(x: &mut [f32], denoised: &[f32], sigma: f64, sigma_next: f64, rng: &mut RandomSources) {
    let sigma_up = if sigma > 1e-10 { (sigma_next.powi(2) * (sigma.powi(2) - sigma_next.powi(2)) / sigma.powi(2)).max(0.0).sqrt() } else { 0.0 };
    let sigma_down = (sigma_next.powi(2) - sigma_up.powi(2)).max(0.0).sqrt();
    euler_step(x, denoised, sigma, sigma_down).await;
    if sigma_up > 1e-10 {
        for xi in x.iter_mut() {
            *xi += (sigma_up as f32) * normal_std(rng).await;
        }
    }
}

async fn heun_correct(x: &mut [f32], denoised0: &[f32], x_euler: &[f32], denoised1: &[f32], sigma: f64, sigma_next: f64) {
    let sigma_f = sigma.max(1e-10) as f32;
    let sigma_next_f = sigma_next.max(1e-10) as f32;
    let dt = (sigma_next - sigma) as f32;
    for i in 0..x.len() {
        let d0 = (x[i] - denoised0[i]) / sigma_f;
        let d1 = (x_euler[i] - denoised1[i]) / sigma_next_f;
        x[i] += (d0 + d1) * 0.5 * dt;
    }
}

async fn ddim_step(x: &mut [f32], denoised: &[f32], sigma: f64, sigma_next: f64, eta: f64, rng: &mut RandomSources) {
    let alpha_bar = 1.0 / (1.0 + sigma * sigma);
    let alpha_bar_next = 1.0 / (1.0 + sigma_next * sigma_next);
    let sigma_ddim = if alpha_bar < 1.0 { eta * ((1.0 - alpha_bar_next) / (1.0 - alpha_bar) * (1.0 - alpha_bar / alpha_bar_next)).max(0.0).sqrt() } else { 0.0 };
    let sigma_f = sigma.max(1e-10) as f32;
    let dir_coeff = ((1.0 - alpha_bar_next - sigma_ddim * sigma_ddim).max(0.0)).sqrt() as f32;
    let alpha_bar_next_sqrt = alpha_bar_next.sqrt() as f32;
    for i in 0..x.len() {
        let eps = (x[i] - denoised[i]) / sigma_f;
        x[i] = alpha_bar_next_sqrt * denoised[i] + dir_coeff * eps;
    }
    if sigma_ddim > 1e-10 {
        for xi in x.iter_mut() {
            *xi += (sigma_ddim as f32) * normal_std(rng).await;
        }
    }
}

/// 🌫️ Which ODE/SDE solver [`run_diffusion`] uses to step from one noise level to the next.
/// Covers the most commonly used solvers; PLMS/DPM/DPM++/UniPC-style multistep solvers are a
/// documented scope cut for this delivery (they need retained solver-state history across steps,
/// which [`Solver`] doesn't yet carry).
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Solver {
    Euler,
    EulerAncestral,
    Heun,
    Ddim { eta: f64 },
}

/// 🌫️ One [`run_diffusion`] configuration.
pub struct DiffusionRunConfig {
    pub schedule: NoiseSchedule,
    pub solver: Solver,
    pub steps: usize,
    pub guidance: Option<Guidance>,
    pub seed: u64,
}

/// 🌫️ Whether [`run_diffusion`]'s step callback wants to continue or cancel the run.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StepControlFlow {
    Continue,
    Cancel,
}

async fn eval_denoised(denoiser: &mut impl Denoiser, x: &[f32], shape: [usize; 4], sigma: f64, guidance: Option<&Guidance>, step: usize) -> Result<Vec<f32>, SamplingError> {
    let prediction_type = denoiser.prediction_type().await;
    let mut raw_cond = vec![0.0f32; x.len()];
    denoiser.denoise(x, shape, sigma, step, GuidanceBranch::Conditional, &mut raw_cond).await?;
    let raw = if let Some(g) = guidance {
        let mut raw_uncond = vec![0.0f32; x.len()];
        denoiser.denoise(x, shape, sigma, step, GuidanceBranch::Unconditional, &mut raw_uncond).await?;
        let mut combined = vec![0.0f32; x.len()];
        g.combine(&raw_cond, &raw_uncond, &mut combined).await;
        combined
    } else {
        raw_cond
    };
    Ok(to_denoised(prediction_type, x, sigma, &raw).await)
}

/// 🌫️ Runs the reverse diffusion process on `latent` in place, from `schedule.sigmas(steps + 1)`'s
/// most-noisy value down to its last (typically `0`) — the "Text-to-image latent initialization"
/// and "Partial denoising" generation modes (see [`img2img_start_index`] and [`apply_inpaint_mask`]
/// for image-to-image / inpainting on top of this same loop). `step_callback(step, sigma_next,
/// latent)` fires after every step — it can preview, modify (e.g. blend in [`apply_inpaint_mask`]),
/// or cancel (returning [`StepControlFlow::Cancel`], which surfaces as [`SamplingError::Cancelled`]).
pub async fn run_diffusion(config: &DiffusionRunConfig, latent: &mut [f32], shape: [usize; 4], denoiser: &mut impl Denoiser, mut step_callback: impl FnMut(usize, f64, &mut [f32]) -> StepControlFlow) -> Result<(), SamplingError> {
    let sigmas = config.schedule.sigmas(config.steps + 1).await;
    let mut rng: RandomSources = CounterRng::from_root(config.seed, StreamKey { request: 0, sequence: 0, beam: 0, candidate: 0, purpose: StreamPurpose::Diffusion }).await.into();

    for i in 0..config.steps {
        let sigma = sigmas[i];
        let sigma_next = sigmas.get(i + 1).copied().unwrap_or(0.0);
        let denoised = eval_denoised(denoiser, latent, shape, sigma, config.guidance.as_ref(), i).await?;

        match config.solver {
            Solver::Euler => euler_step(latent, &denoised, sigma, sigma_next).await,
            Solver::EulerAncestral => euler_ancestral_step(latent, &denoised, sigma, sigma_next, &mut rng).await,
            Solver::Heun => {
                if sigma_next <= 1e-10 {
                    euler_step(latent, &denoised, sigma, sigma_next).await;
                } else {
                    let mut x_euler = latent.to_vec();
                    euler_step(&mut x_euler, &denoised, sigma, sigma_next).await;
                    let denoised_next = eval_denoised(denoiser, &x_euler, shape, sigma_next, config.guidance.as_ref(), i).await?;
                    heun_correct(latent, &denoised, &x_euler, &denoised_next, sigma, sigma_next).await;
                }
            }
            Solver::Ddim { eta } => ddim_step(latent, &denoised, sigma, sigma_next, eta, &mut rng).await,
        }

        if let StepControlFlow::Cancel = step_callback(i, sigma_next, latent) {
            return Err(SamplingError::Cancelled);
        }
    }
    Ok(())
}

/// 🌫️ Image-to-image: the sigma-schedule index to start denoising from, given `strength` in
/// `[0, 1]` (`1.0` = full generation from pure noise, `0.0` = skip denoising entirely, returning
/// the original image essentially unchanged).
pub async fn img2img_start_index(sigma_count: usize, strength: f64) -> usize {
    let strength = strength.clamp(0.0, 1.0);
    (((1.0 - strength) * sigma_count as f64).round() as usize).min(sigma_count)
}

/// 🌫️ Inpainting: re-noises `original` to the current `sigma` and blends it into `x` wherever
/// `mask[i] > 0` (`1.0` = fully keep the (re-noised) original, `0.0` = fully keep the freely
/// generated content), matching the "hold the known region, regenerate the rest" contract.
pub async fn apply_inpaint_mask(x: &mut [f32], original: &[f32], mask: &[f32], sigma: f64, rng: &mut RandomSources) {
    for i in 0..x.len() {
        if mask[i] > 0.0 {
            let noisy_original = original[i] + (sigma as f32) * normal_std(rng).await;
            x[i] = mask[i] * noisy_original + (1.0 - mask[i]) * x[i];
        }
    }
}
// #endregion 🔖️Diffusion

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
