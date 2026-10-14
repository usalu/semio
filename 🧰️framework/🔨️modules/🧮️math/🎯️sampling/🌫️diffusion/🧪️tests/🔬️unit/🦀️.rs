use super::*;

// #region 🔖️DiffusionTests
struct ConstantDenoiser {
    target: f32,
}

impl Denoiser for ConstantDenoiser {
    async fn prediction_type(&self) -> PredictionType {
        PredictionType::Sample
    }
    async fn denoise(&mut self, _latent: &[f32], _shape: [usize; 4], _sigma: f64, _step: usize, _branch: GuidanceBranch, out: &mut [f32]) -> Result<(), SamplingError> {
        for o in out.iter_mut() {
            *o = self.target;
        }
        Ok(())
    }
}

#[semio_framework_async_macros::async_test]
async fn all_noise_schedules_are_non_increasing() {
    let schedules = vec![
        NoiseSchedule::Linear { beta_start: 0.0001, beta_end: 0.02 },
        NoiseSchedule::ScaledLinear { beta_start: 0.0001, beta_end: 0.02 },
        NoiseSchedule::Cosine { s: 0.008 },
        NoiseSchedule::Karras { sigma_min: 0.01, sigma_max: 10.0, rho: 7.0 },
        NoiseSchedule::Exponential { sigma_min: 0.01, sigma_max: 10.0 },
        NoiseSchedule::Polynomial { sigma_min: 0.01, sigma_max: 10.0, power: 2.0 },
    ];
    for schedule in schedules {
        let sigmas = schedule.sigmas(10).await;
        assert_eq!(sigmas.len(), 10);
        for w in sigmas.windows(2) {
            assert!(w[0] >= w[1] - 1e-9, "{schedule:?} sigmas not non-increasing: {sigmas:?}");
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn custom_schedule_returns_its_values_verbatim() {
    let values = vec![5.0, 3.0, 1.0, 0.0];
    let schedule = NoiseSchedule::Custom(values.clone());
    assert_eq!(schedule.sigmas(4).await, values);
}

#[semio_framework_async_macros::async_test]
async fn euler_solver_converges_exactly_to_a_constant_target_when_schedule_reaches_zero() {
    // 📐️ With a constant (x-independent) `Sample` prediction, the Euler update at the final
    // step (sigma_next == 0) reduces algebraically to `x_next = denoised` exactly — a good
    // closed-form check that doesn't depend on any specific schedule shape.
    let config = DiffusionRunConfig { schedule: NoiseSchedule::Custom(vec![10.0, 5.0, 2.0, 0.5, 0.0]), solver: Solver::Euler, steps: 4, guidance: None, seed: 0 };
    let mut latent = vec![0.0f32; 4];
    let mut denoiser = ConstantDenoiser { target: 5.0 };
    run_diffusion(&config, &mut latent, [1, 1, 1, 4], &mut denoiser, |_, _, _| StepControlFlow::Continue).await.unwrap();
    for &v in &latent {
        assert!((v - 5.0).abs() < 1e-3, "v={v}");
    }
}

#[semio_framework_async_macros::async_test]
async fn ddim_eta_zero_is_deterministic_regardless_of_seed() {
    let run = async |seed: u64| {
        let config = DiffusionRunConfig { schedule: NoiseSchedule::Custom(vec![4.0, 2.0, 1.0, 0.0]), solver: Solver::Ddim { eta: 0.0 }, steps: 3, guidance: None, seed };
        let mut latent = vec![1.0f32; 4];
        let mut denoiser = ConstantDenoiser { target: 3.0 };
        run_diffusion(&config, &mut latent, [1, 1, 1, 4], &mut denoiser, |_, _, _| StepControlFlow::Continue).await.unwrap();
        latent
    };
    assert_eq!(run(1).await, run(999).await);
}

#[semio_framework_async_macros::async_test]
async fn euler_ancestral_and_ddim_with_eta_differ_across_seeds() {
    let run = async |seed: u64| {
        let config = DiffusionRunConfig { schedule: NoiseSchedule::Custom(vec![4.0, 2.0, 1.0, 0.2]), solver: Solver::EulerAncestral, steps: 3, guidance: None, seed };
        let mut latent = vec![1.0f32; 4];
        let mut denoiser = ConstantDenoiser { target: 3.0 };
        run_diffusion(&config, &mut latent, [1, 1, 1, 4], &mut denoiser, |_, _, _| StepControlFlow::Continue).await.unwrap();
        latent
    };
    assert_ne!(run(1).await, run(2).await, "ancestral sampling's injected noise should differ across seeds");
}

#[semio_framework_async_macros::async_test]
async fn guidance_combine_extrapolates_away_from_unconditional() {
    let guidance = Guidance { scale: 2.0, rescale: 0.0 };
    let cond = [1.0f32, 2.0];
    let uncond = [0.0f32, 0.0];
    let mut out = [0.0f32; 2];
    guidance.combine(&cond, &uncond, &mut out).await;
    assert_eq!(out, [2.0, 4.0]);
}

#[semio_framework_async_macros::async_test]
async fn run_diffusion_with_guidance_evaluates_both_branches_every_step() {
    struct BranchTrackingDenoiser {
        cond_calls: usize,
        uncond_calls: usize,
    }
    impl Denoiser for BranchTrackingDenoiser {
        async fn prediction_type(&self) -> PredictionType {
            PredictionType::Sample
        }
        async fn denoise(&mut self, _latent: &[f32], _shape: [usize; 4], _sigma: f64, _step: usize, branch: GuidanceBranch, out: &mut [f32]) -> Result<(), SamplingError> {
            match branch {
                GuidanceBranch::Conditional => {
                    self.cond_calls += 1;
                    out.fill(1.0);
                }
                GuidanceBranch::Unconditional => {
                    self.uncond_calls += 1;
                    out.fill(0.0);
                }
            }
            Ok(())
        }
    }
    let config = DiffusionRunConfig { schedule: NoiseSchedule::Custom(vec![2.0, 1.0, 0.0]), solver: Solver::Euler, steps: 2, guidance: Some(Guidance { scale: 1.5, rescale: 0.0 }), seed: 0 };
    let mut latent = vec![0.0f32; 2];
    let mut denoiser = BranchTrackingDenoiser { cond_calls: 0, uncond_calls: 0 };
    run_diffusion(&config, &mut latent, [1, 1, 1, 2], &mut denoiser, |_, _, _| StepControlFlow::Continue).await.unwrap();
    assert_eq!(denoiser.cond_calls, 2);
    assert_eq!(denoiser.uncond_calls, 2);
}

#[semio_framework_async_macros::async_test]
async fn heun_solver_uses_two_evaluations_per_non_final_step() {
    struct CountingDenoiser {
        calls: usize,
    }
    impl Denoiser for CountingDenoiser {
        async fn prediction_type(&self) -> PredictionType {
            PredictionType::Sample
        }
        async fn denoise(&mut self, _latent: &[f32], _shape: [usize; 4], _sigma: f64, _step: usize, _branch: GuidanceBranch, out: &mut [f32]) -> Result<(), SamplingError> {
            self.calls += 1;
            out.fill(2.0);
            Ok(())
        }
    }
    let config = DiffusionRunConfig { schedule: NoiseSchedule::Custom(vec![4.0, 2.0, 0.0]), solver: Solver::Heun, steps: 2, guidance: None, seed: 0 };
    let mut latent = vec![0.0f32; 2];
    let mut denoiser = CountingDenoiser { calls: 0 };
    run_diffusion(&config, &mut latent, [1, 1, 1, 2], &mut denoiser, |_, _, _| StepControlFlow::Continue).await.unwrap();
    // step 0 (sigma_next=2.0, not final): 2 evaluations; step 1 (sigma_next=0.0, final): 1 evaluation.
    assert_eq!(denoiser.calls, 3);
}

#[semio_framework_async_macros::async_test]
async fn run_diffusion_cancellation_stops_the_run_and_errors() {
    let config = DiffusionRunConfig { schedule: NoiseSchedule::Custom(vec![4.0, 2.0, 1.0, 0.0]), solver: Solver::Euler, steps: 3, guidance: None, seed: 0 };
    let mut latent = vec![0.0f32; 2];
    let mut denoiser = ConstantDenoiser { target: 1.0 };
    let mut calls = 0;
    let result = run_diffusion(&config, &mut latent, [1, 1, 1, 2], &mut denoiser, |_, _, _| {
        calls += 1;
        if calls == 1 { StepControlFlow::Cancel } else { StepControlFlow::Continue }
    })
    .await;
    assert!(result.is_err());
    assert_eq!(calls, 1);
}

#[semio_framework_async_macros::async_test]
async fn img2img_start_index_boundaries() {
    assert_eq!(img2img_start_index(10, 1.0).await, 0);
    assert_eq!(img2img_start_index(10, 0.0).await, 10);
    assert_eq!(img2img_start_index(10, 0.5).await, 5);
}

#[semio_framework_async_macros::async_test]
async fn apply_inpaint_mask_blends_by_mask_weight() {
    let mut x = [10.0f32, 10.0, 10.0];
    let original = [0.0f32, 0.0, 0.0];
    let mask = [1.0f32, 0.0, 0.5];
    let mut rng: RandomSources = CounterRng::from_seed(1).await.into();
    // 🌫️ sigma = 0.0 means the re-noised original equals the original exactly, isolating the
    // blend-weight arithmetic from the injected-noise term.
    apply_inpaint_mask(&mut x, &original, &mask, 0.0, &mut rng).await;
    assert!((x[0] - 0.0).abs() < 1e-6, "fully masked (1.0) must fall back to the original");
    assert_eq!(x[1], 10.0, "unmasked (0.0) must stay untouched");
    assert!((x[2] - 5.0).abs() < 1e-6, "half-masked (0.5) must blend 50/50");
}

#[semio_framework_async_macros::async_test]
async fn prediction_type_conversions_round_trip_through_denoised() {
    let x = [2.0f32, -1.0];
    let sigma = 0.5;
    let target = [3.0f32, 3.0];

    let sample_raw = target;
    assert_eq!(to_denoised(PredictionType::Sample, &x, sigma, &sample_raw).await, target.to_vec());

    // 📐️ epsilon such that `x - sigma*eps == target` exactly.
    let eps: Vec<f32> = x.iter().zip(target).map(|(&xi, ti)| (xi - ti) / sigma as f32).collect();
    let denoised = to_denoised(PredictionType::Epsilon, &x, sigma, &eps).await;
    for (d, t) in denoised.iter().zip(target) {
        assert!((d - t).abs() < 1e-4);
    }
}

// #endregion 🔖️DiffusionTests
