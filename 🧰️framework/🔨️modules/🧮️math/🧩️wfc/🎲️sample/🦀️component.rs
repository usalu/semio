//! 🎲️ Value sampling: which pattern a decision assigns from the selected node's live domain.

use crate::wfc::domain::Domain;
use crate::wfc::ids::PatternId;
use crate::wfc::model::CompiledModel;
use crate::random::Rng;

// #region 🔖️Sampler
/// 🎲️ How one pattern is chosen from an unresolved domain.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ValueSampler {
    /// 🎲️ Probability proportional to pattern weight.
    #[default]
    WeightedRoulette,
    /// 🎲️ Every live pattern equally likely, ignoring weight.
    Uniform,
}

/// 🎲️ Draws one pattern from `domain` (must be non-empty) according to `sampler`.
pub(crate) fn sample_pattern(sampler: ValueSampler, domain: &Domain, model: &CompiledModel, rng: &mut Rng) -> PatternId {
    debug_assert!(domain.cardinality() > 0, "sample_pattern: domain must be non-empty");
    match sampler {
        ValueSampler::Uniform => {
            let k = rng.next_range(0, domain.cardinality() as u64) as usize;
            domain.bits().iter_ones().nth(k).expect("domain non-empty per precondition")
        }
        ValueSampler::WeightedRoulette => {
            let total = domain.sum_w();
            if total <= 0.0 {
                let k = rng.next_range(0, domain.cardinality() as u64) as usize;
                return domain.bits().iter_ones().nth(k).expect("domain non-empty per precondition");
            }
            let target = rng.next_f64() * total;
            let mut acc = 0.0;
            let mut last = None;
            for p in domain.bits().iter_ones() {
                acc += model.weights().w(p);
                last = Some(p);
                if acc >= target {
                    return p;
                }
            }
            last.expect("domain non-empty per precondition")
        }
    }
}
// #endregion 🔖️Sampler

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::wfc::model::ModelBuilder;

    fn model_and_domain(weights: &[f64]) -> (CompiledModel, Domain) {
        let mut b = ModelBuilder::new();
        for &w in weights {
            b.add_pattern(w);
        }
        b.add_relation("r");
        let model = b.compile().unwrap();
        let domain = Domain::new_full(model.weights());
        (model, domain)
    }

    #[test]
    fn uniform_only_ever_returns_live_patterns() {
        let (model, domain) = model_and_domain(&[1.0, 1.0, 1.0]);
        let mut rng = Rng::from_seed(1);
        for _ in 0..50 {
            let p = sample_pattern(ValueSampler::Uniform, &domain, &model, &mut rng);
            assert!(domain.bits().get(p));
        }
    }

    #[test]
    fn weighted_roulette_only_ever_returns_live_patterns() {
        let (model, domain) = model_and_domain(&[1.0, 5.0, 10.0]);
        let mut rng = Rng::from_seed(2);
        for _ in 0..50 {
            let p = sample_pattern(ValueSampler::WeightedRoulette, &domain, &model, &mut rng);
            assert!(domain.bits().get(p));
        }
    }

    #[test]
    fn weighted_roulette_is_biased_toward_heavier_pattern() {
        let (model, domain) = model_and_domain(&[1.0, 99.0]);
        let mut rng = Rng::from_seed(3);
        let mut counts = [0u32; 2];
        for _ in 0..2000 {
            let p = sample_pattern(ValueSampler::WeightedRoulette, &domain, &model, &mut rng);
            counts[p.index()] += 1;
        }
        assert!(counts[1] > counts[0] * 10);
    }

    #[test]
    fn same_seed_produces_same_sequence() {
        let (model, domain) = model_and_domain(&[1.0, 2.0, 3.0]);
        let mut r1 = Rng::from_seed(42);
        let mut r2 = Rng::from_seed(42);
        for _ in 0..20 {
            let p1 = sample_pattern(ValueSampler::WeightedRoulette, &domain, &model, &mut r1);
            let p2 = sample_pattern(ValueSampler::WeightedRoulette, &domain, &model, &mut r2);
            assert_eq!(p1, p2);
        }
    }
}
// #endregion 🔖️Tests
