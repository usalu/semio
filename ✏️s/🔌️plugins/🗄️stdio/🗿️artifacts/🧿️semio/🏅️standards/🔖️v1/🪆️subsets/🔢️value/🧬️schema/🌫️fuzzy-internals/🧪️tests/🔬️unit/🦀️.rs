
use super::*;

#[test]
fn fuzzy_error_contract_is_owned_and_stable() {
    let errors = [
        (FuzzyError::InvalidDomain("range".into()), "invalid domain: range".to_string()),
        (FuzzyError::EmptyRuleBase, "empty rule base".to_string()),
        (FuzzyError::EmptyUniverse, "empty universe".to_string()),
        (FuzzyError::SingularSystem, "singular linear system".to_string()),
        (FuzzyError::DimensionMismatch("matrix".into()), "dimension mismatch: matrix".to_string()),
        (FuzzyError::InvalidIntuitionistic, "invalid intuitionistic set: membership + non-membership exceeds 1".to_string()),
        (FuzzyError::NoFiredRules, "no fired rules".to_string()),
        (FuzzyError::InvalidParameterCount { expected: 4, got: 2 }, "invalid parameter count: expected 4, got 2".to_string()),
    ];
    for (error, message) in errors {
        assert_eq!(error.to_string(), message);
        assert!(std::error::Error::source(&error).is_none());
    }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn temp_speed_system() -> MimoSystem {
    let temp_univ = Universe::new(0.0, 40.0, 41).unwrap();
    let speed_univ = Universe::new(0.0, 100.0, 41).unwrap();
    let temp_var = LinguisticVariable::new("temperature", temp_univ, vec![FuzzySet::new("low", MembershipFunction::triangular(0.0, 0.0, 20.0)), FuzzySet::new("high", MembershipFunction::triangular(15.0, 40.0, 40.0))]);
    let speed_var = LinguisticVariable::new("fan_speed", speed_univ, vec![FuzzySet::new("slow", MembershipFunction::triangular(0.0, 0.0, 50.0)), FuzzySet::new("fast", MembershipFunction::triangular(40.0, 100.0, 100.0))]);
    MimoSystem {
        inputs: vec![temp_var],
        outputs: vec![speed_var],
        rules: RuleBase::new(vec![
            Rule { id: 0, antecedents: vec![AntecedentClause { input: 0, term: 1, hedge: None }], consequent: Consequent::Mamdani { output: 0, term: 1 }, weight: 1.0, confidence: 1.0 },
            Rule { id: 1, antecedents: vec![AntecedentClause { input: 0, term: 0, hedge: None }], consequent: Consequent::Mamdani { output: 0, term: 0 }, weight: 1.0, confidence: 1.0 },
        ]),
        model: InferenceModel::Mamdani,
        tnorm: TNorm::Min,
        tconorm: TConorm::Max,
        defuzzifier: Defuzzifier::Centroid,
    }
}

#[test]
fn triangular_membership_peaks_at_center() {
    let mf = MembershipFunction::triangular(0.0, 5.0, 10.0);
    assert!((mf.eval(5.0) - 1.0).abs() < 1e-9);
    assert_eq!(mf.eval(-1.0), 0.0);
}

#[test]
fn tnorm_product_is_stricter_than_min() {
    assert!(TNorm::Product.apply(0.6, 0.6) < TNorm::Min.apply(0.6, 0.6));
}

#[test]
fn fuzzy_number_alpha_cut_contains_peak() {
    let n = FuzzyNumber::triangular(1.0, 3.0, 5.0);
    let (lo, hi) = n.alpha_cut(1.0);
    assert!(lo <= 3.0 && hi >= 3.0);
}

#[test]
fn relation_composition_max_min() {
    let mut r1 = FuzzyRelation::new(2, 2);
    r1.set(0, 0, 0.8);
    r1.set(0, 1, 0.3);
    r1.set(1, 0, 0.2);
    r1.set(1, 1, 0.9);
    let composed = r1.compose_max_min(&r1).unwrap();
    assert!(composed.get(0, 0) > 0.0);
}

#[test]
fn mamdani_high_temperature_yields_fast_fan() {
    let system = temp_speed_system();
    let (out, explanation) = system.infer(&[35.0]).unwrap();
    assert!(out[0] > 50.0);
    assert!(!explanation.traces.is_empty());
}

#[test]
fn sugeno_inference_weighted_average() {
    let mut system = temp_speed_system();
    system.model = InferenceModel::Sugeno;
    system.rules = RuleBase::new(vec![
        Rule { id: 0, antecedents: vec![AntecedentClause { input: 0, term: 1, hedge: None }], consequent: Consequent::SugenoConstant { output: 0, value: 90.0 }, weight: 1.0, confidence: 1.0 },
        Rule { id: 1, antecedents: vec![AntecedentClause { input: 0, term: 0, hedge: None }], consequent: Consequent::SugenoConstant { output: 0, value: 10.0 }, weight: 1.0, confidence: 1.0 },
    ]);
    let (out, _) = system.infer(&[35.0]).unwrap();
    assert!(out[0] > 70.0);
}

#[test]
fn defuzzifier_centroid_on_triangle() {
    let univ = Universe::new(0.0, 10.0, 101).unwrap();
    let membership: Vec<f64> = univ.samples.iter().map(|x| MembershipFunction::triangular(0.0, 5.0, 10.0).eval(*x)).collect();
    let c = Defuzzifier::Centroid.apply(&univ, &membership, None, None).unwrap();
    assert!((c - 5.0).abs() < 0.2);
}

#[test]
fn fuzzy_c_means_two_cluster_toy() {
    let data = vec![vec![0.0, 0.0], vec![0.1, 0.0], vec![5.0, 5.0], vec![5.1, 5.0]];
    let result = fuzzy_c_means(&data, 2, 2.0, 50, 1e-4).unwrap();
    assert_eq!(result.centers.len(), 2);
    assert_eq!(result.membership.len(), 4);
}

#[test]
fn fuzzy_ahp_produces_normalized_weights() {
    let matrix = vec![vec![FuzzyNumber::triangular(1.0, 1.0, 1.0), FuzzyNumber::triangular(2.0, 3.0, 4.0)], vec![FuzzyNumber::triangular(0.25, 0.33, 0.5), FuzzyNumber::triangular(1.0, 1.0, 1.0)]];
    let weights = FuzzyAhp::new(matrix).weights();
    assert!((weights.iter().sum::<f64>() - 1.0).abs() < 1e-6);
}

#[test]
fn fuzzy_topsis_ranks_better_alternative_first() {
    let topsis = FuzzyTopsis {
        alternatives: vec!["a".into(), "b".into()],
        criteria: vec!["c".into()],
        decision_matrix: vec![vec![FuzzyNumber::triangular(8.0, 9.0, 10.0)], vec![FuzzyNumber::triangular(1.0, 2.0, 3.0)]],
        weights: vec![1.0],
        benefit: vec![true],
    };
    let rank = topsis.rank();
    assert_eq!(rank[0].0, 0);
}

#[test]
fn interval_type2_centroid_between_bounds() {
    let it2 = IntervalType2Set::new("temp", MembershipFunction::gaussian(20.0, 1.0), MembershipFunction::gaussian(20.0, 3.0));
    let c = it2.type_reduced_centroid(&linspace(10.0, 30.0, 41));
    assert!((c - 20.0).abs() < 2.0);
}

#[test]
fn fan_controller_learns_from_sensor_data() {
    let temps: Vec<f64> = (0..20).map(|i| i as f64 * 2.0).collect();
    let speeds: Vec<f64> = temps.iter().map(|t| t * 2.5).collect();
    let mut controller = FanController::from_sensor_data(&temps, &speeds).unwrap();
    let (speed, explanation) = controller.decide(30.0).unwrap();
    assert!(speed >= 0.0);
    assert!(!explanation.rationale.is_empty());
}

#[test]
fn trapezoidal_membership_flat_top_and_edges() {
    let mf = MembershipFunction::trapezoidal(0.0, 2.0, 6.0, 8.0);
    assert_eq!(mf.eval(-1.0), 0.0);
    assert_eq!(mf.eval(0.0), 0.0);
    assert!((mf.eval(1.0) - 0.5).abs() < 1e-9);
    assert_eq!(mf.eval(4.0), 1.0);
    assert!((mf.eval(7.0) - 0.5).abs() < 1e-9);
    assert_eq!(mf.eval(8.0), 0.0);
}

#[test]
fn gaussian_membership_symmetric_and_peaks_at_mean() {
    let mf = MembershipFunction::gaussian(10.0, 2.0);
    assert_eq!(mf.eval(10.0), 1.0);
    assert!((mf.eval(12.0) - mf.eval(8.0)).abs() < 1e-12);
    assert!((mf.eval(12.0) - 0.6065306597126334).abs() < 1e-9);
}

#[test]
fn generalized_bell_membership_symmetric_peak() {
    let mf = MembershipFunction::generalized_bell(2.0, 4.0, 5.0);
    assert_eq!(mf.eval(5.0), 1.0);
    assert!((mf.eval(7.0) - mf.eval(3.0)).abs() < 1e-9);
    assert!((mf.eval(7.0) - 0.5).abs() < 1e-9);
}

#[test]
fn sigmoid_membership_is_monotonic_increasing() {
    let mf = MembershipFunction::sigmoid(1.0, 0.0);
    assert!((mf.eval(0.0) - 0.5).abs() < 1e-9);
    assert!(mf.eval(-1.0) < mf.eval(0.0));
    assert!(mf.eval(0.0) < mf.eval(1.0));
}

#[test]
fn singleton_membership_exact_match_only() {
    let mf = MembershipFunction::singleton(5.0);
    assert_eq!(mf.eval(5.0), 1.0);
    assert_eq!(mf.eval(5.001), 0.0);
}

#[test]
fn piecewise_linear_membership_interpolates_and_edge_cases() {
    assert_eq!(MembershipFunction::piecewise_linear(vec![]).eval(0.0), 0.0);
    assert_eq!(MembershipFunction::piecewise_linear(vec![(2.0, 0.7)]).eval(99.0), 0.7);
    let mf = MembershipFunction::piecewise_linear(vec![(0.0, 0.0), (5.0, 1.0), (10.0, 0.0)]);
    assert_eq!(mf.eval(-1.0), 0.0);
    assert_eq!(mf.eval(11.0), 0.0);
    assert!((mf.eval(2.5) - 0.5).abs() < 1e-9);
    assert!((mf.eval(7.5) - 0.5).abs() < 1e-9);
}

#[test]
fn membership_function_parameters_roundtrip() {
    let mut tri = MembershipFunction::triangular(0.0, 1.0, 2.0);
    assert_eq!(tri.parameters(), vec![0.0, 1.0, 2.0]);
    tri.set_parameters(&[1.0, 2.0, 3.0]).unwrap();
    assert_eq!(tri.parameters(), vec![1.0, 2.0, 3.0]);
    assert_eq!(tri.set_parameters(&[1.0, 2.0]), Err(FuzzyError::InvalidParameterCount { expected: 3, got: 2 }));

    let mut trap = MembershipFunction::trapezoidal(0.0, 1.0, 2.0, 3.0);
    trap.set_parameters(&[1.0, 2.0, 3.0, 4.0]).unwrap();
    assert_eq!(trap.parameters(), vec![1.0, 2.0, 3.0, 4.0]);
    assert!(trap.set_parameters(&[1.0]).is_err());

    let mut gauss = MembershipFunction::gaussian(0.0, 1.0);
    gauss.set_parameters(&[2.0, 1e-20]).unwrap();
    assert_eq!(gauss.parameters(), vec![2.0, 1e-12]);
    assert!(gauss.set_parameters(&[1.0]).is_err());

    let mut bell = MembershipFunction::generalized_bell(1.0, 1.0, 0.0);
    bell.set_parameters(&[1e-20, 1e-20, 5.0]).unwrap();
    assert_eq!(bell.parameters(), vec![1e-12, 1e-6, 5.0]);
    assert!(bell.set_parameters(&[1.0, 2.0]).is_err());

    let mut sig = MembershipFunction::sigmoid(1.0, 0.0);
    sig.set_parameters(&[2.0, 3.0]).unwrap();
    assert_eq!(sig.parameters(), vec![2.0, 3.0]);
    assert!(sig.set_parameters(&[]).is_err());

    let mut single = MembershipFunction::singleton(1.0);
    single.set_parameters(&[9.0]).unwrap();
    assert_eq!(single.parameters(), vec![9.0]);
    assert!(single.set_parameters(&[1.0, 2.0]).is_err());

    let mut pw = MembershipFunction::piecewise_linear(vec![(0.0, 0.0)]);
    pw.set_parameters(&[0.0, 0.0, 1.0, 1.0]).unwrap();
    assert_eq!(pw.parameters(), vec![0.0, 0.0, 1.0, 1.0]);
    assert!(pw.set_parameters(&[0.0, 0.0, 1.0]).is_err());
}

#[test]
fn membership_function_support_bounds() {
    assert_eq!(MembershipFunction::triangular(1.0, 2.0, 3.0).support_min(), 1.0);
    assert_eq!(MembershipFunction::triangular(1.0, 2.0, 3.0).support_max(), 3.0);
    assert_eq!(MembershipFunction::trapezoidal(1.0, 2.0, 3.0, 4.0).support_min(), 1.0);
    assert_eq!(MembershipFunction::trapezoidal(1.0, 2.0, 3.0, 4.0).support_max(), 4.0);
    let g = MembershipFunction::gaussian(10.0, 2.0);
    assert_eq!(g.support_min(), 2.0);
    assert_eq!(g.support_max(), 18.0);
    let bell = MembershipFunction::generalized_bell(2.0, 4.0, 5.0);
    assert_eq!(bell.support_min(), -3.0);
    assert_eq!(bell.support_max(), 13.0);
    let sig = MembershipFunction::sigmoid(1.0, 5.0);
    assert_eq!(sig.support_min(), -5.0);
    assert_eq!(sig.support_max(), 15.0);
    let single = MembershipFunction::singleton(7.0);
    assert_eq!(single.support_min(), 7.0);
    assert_eq!(single.support_max(), 7.0);
    assert_eq!(MembershipFunction::piecewise_linear(vec![]).support_min(), 0.0);
    let pw = MembershipFunction::piecewise_linear(vec![(1.0, 0.0), (9.0, 1.0)]);
    assert_eq!(pw.support_min(), 1.0);
    assert_eq!(pw.support_max(), 9.0);
}

#[test]
fn intuitionistic_set_grades_rejects_over_unity() {
    let set = IntuitionisticSet::new("x", MembershipFunction::triangular(0.0, 5.0, 10.0), MembershipFunction::triangular(0.0, 5.0, 10.0));
    assert_eq!(set.grades(5.0), Err(FuzzyError::InvalidIntuitionistic));
}

#[test]
fn intuitionistic_set_grades_computes_hesitation() {
    let set = IntuitionisticSet::new("x", MembershipFunction::triangular(0.0, 5.0, 10.0), MembershipFunction::singleton(100.0));
    let (mu, nu, hesitation) = set.grades(2.0).unwrap();
    assert!((mu - 0.4).abs() < 1e-9);
    assert_eq!(nu, 0.0);
    assert!((hesitation - 0.6).abs() < 1e-9);
}

#[test]
fn tnorm_lukasiewicz_and_drastic_variants() {
    assert!((TNorm::Lukasiewicz.apply(0.6, 0.6) - 0.2).abs() < 1e-9);
    assert_eq!(TNorm::Lukasiewicz.apply(0.3, 0.3), 0.0);
    assert_eq!(TNorm::Drastic.apply(0.5, 1.0), 0.5);
    assert_eq!(TNorm::Drastic.apply(1.0, 0.5), 0.5);
    assert_eq!(TNorm::Drastic.apply(0.5, 0.5), 0.0);
}

#[test]
fn tconorm_variants_apply_correctly() {
    assert_eq!(TConorm::Max.apply(0.3, 0.7), 0.7);
    assert!((TConorm::ProbSum.apply(0.5, 0.5) - 0.75).abs() < 1e-9);
    assert_eq!(TConorm::Lukasiewicz.apply(0.6, 0.6), 1.0);
    assert_eq!(TConorm::NilpotentMax.apply(0.3, 0.3), 0.0);
    assert_eq!(TConorm::NilpotentMax.apply(0.6, 0.6), 0.6);
}

#[test]
fn tnorm_tconorm_fold_over_iterator() {
    let vals = [0.9, 0.8, 0.7];
    assert!((TNorm::Min.fold(vals.iter().copied()) - 0.7).abs() < 1e-9);
    assert!((TConorm::Max.fold(vals.iter().copied()) - 0.9).abs() < 1e-9);
}

#[test]
fn hedge_variants_apply() {
    assert!((Hedge::Very.apply(0.8) - 0.64).abs() < 1e-9);
    assert!((Hedge::Somewhat.apply(0.64) - 0.8).abs() < 1e-9);
    assert!((Hedge::Extremely.apply(0.5) - 0.125).abs() < 1e-9);
    assert!((Hedge::MoreOrLess.apply(0.5) - 0.5).abs() < 1e-9);
}

#[test]
fn complement_concentration_dilation_basic() {
    assert!((complement(0.3) - 0.7).abs() < 1e-9);
    assert!((concentration(0.5) - 0.25).abs() < 1e-9);
    assert!((dilation(0.25) - 0.5).abs() < 1e-9);
}

#[test]
fn fuzzy_number_sub_and_scale() {
    let n1 = FuzzyNumber::triangular(1.0, 3.0, 5.0);
    let n2 = FuzzyNumber::triangular(1.0, 2.0, 3.0);
    let diff = n1.sub(n2);
    assert_eq!((diff.a, diff.b, diff.c), (-2.0, 1.0, 4.0));
    let scaled = n1.scale(-2.0);
    assert_eq!((scaled.a, scaled.b, scaled.c), (-10.0, -6.0, -2.0));
    let scaled_pos = n1.scale(2.0);
    assert_eq!((scaled_pos.a, scaled_pos.b, scaled_pos.c), (2.0, 6.0, 10.0));
}

#[test]
fn fuzzy_number_defuzzify_centroid_matches_peak() {
    let n = FuzzyNumber::triangular(0.0, 5.0, 10.0);
    assert!((n.defuzzify_centroid(101) - 5.0).abs() < 0.1);
}

#[test]
fn fuzzy_add_sums_components() {
    let a = FuzzyNumber::triangular(1.0, 2.0, 3.0);
    let b = FuzzyNumber::triangular(1.0, 1.0, 1.0);
    let sum = fuzzy_add(a, b);
    assert_eq!((sum.a, sum.b, sum.c), (2.0, 3.0, 4.0));
}

#[test]
fn fuzzy_mul_interval_at_peak_alpha() {
    let a = FuzzyNumber::triangular(1.0, 2.0, 3.0);
    let b = FuzzyNumber::triangular(2.0, 3.0, 4.0);
    let (lo, hi) = fuzzy_mul_interval(a, b, 1.0);
    assert!((lo - 6.0).abs() < 1e-9);
    assert!((hi - 6.0).abs() < 1e-9);
}

#[test]
fn relation_composition_max_product() {
    let mut r1 = FuzzyRelation::new(2, 2);
    r1.set(0, 0, 0.5);
    r1.set(0, 1, 0.5);
    r1.set(1, 0, 0.5);
    r1.set(1, 1, 0.5);
    let composed = r1.compose_max_product(&r1).unwrap();
    assert!((composed.get(0, 0) - 0.25).abs() < 1e-9);
}

#[test]
fn relation_composition_dimension_mismatch_errors() {
    let r1 = FuzzyRelation::new(2, 3);
    let r2 = FuzzyRelation::new(2, 2);
    assert_eq!(r1.compose_max_min(&r2), Err(FuzzyError::DimensionMismatch("relation composition".into())));
    assert_eq!(r1.compose_max_product(&r2), Err(FuzzyError::DimensionMismatch("relation composition".into())));
}

#[test]
fn possibility_and_necessity_measures() {
    let pm = PossibilityMeasure::new(vec![1.0, 2.0, 3.0, 4.0, 5.0], vec![0.2, 0.5, 0.8, 0.3, 0.1]).unwrap();
    assert!((pm.possibility(|x| x >= 3.0) - 0.8).abs() < 1e-9);
    assert!((pm.necessity(|x| x >= 3.0) - 0.5).abs() < 1e-9);
}

#[test]
fn possibility_measure_from_scores_normalizes_by_max() {
    let pm = PossibilityMeasure::from_scores(vec![1.0, 2.0, 3.0], vec![2.0, 4.0, 1.0]).unwrap();
    assert_eq!(pm.membership, vec![0.5, 1.0, 0.25]);
}

#[test]
fn possibility_measure_rejects_mismatched_lengths() {
    assert_eq!(PossibilityMeasure::new(vec![1.0, 2.0], vec![0.5]), Err(FuzzyError::InvalidDomain("possibility universe".into())));
}

#[test]
fn universe_rejects_invalid_bounds() {
    assert_eq!(Universe::new(5.0, 1.0, 10), Err(FuzzyError::InvalidDomain("universe bounds".into())));
    assert_eq!(Universe::new(0.0, 10.0, 0), Err(FuzzyError::InvalidDomain("universe bounds".into())));
}

#[test]
fn universe_len_sample_and_is_empty() {
    let u = Universe::new(0.0, 10.0, 5).unwrap();
    assert_eq!(u.len(), 5);
    assert!(!u.is_empty());
    assert_eq!(u.sample(0), 0.0);
    assert_eq!(u.sample(4), 10.0);
}

#[test]
fn linguistic_variable_fuzzify_and_term_index() {
    let univ = Universe::new(0.0, 10.0, 11).unwrap();
    let var = LinguisticVariable::new("x", univ, vec![FuzzySet::new("low", MembershipFunction::triangular(0.0, 0.0, 5.0)), FuzzySet::new("high", MembershipFunction::triangular(5.0, 10.0, 10.0))]);
    assert_eq!(var.term_index("high"), Some(1));
    assert_eq!(var.term_index("missing"), None);
    let grades = var.fuzzify(2.5);
    assert_eq!(grades.len(), 2);
    assert!(grades[0] > 0.0);
}

#[test]
fn rule_firing_strength_applies_hedge() {
    let univ = Universe::new(0.0, 10.0, 11).unwrap();
    let var = LinguisticVariable::new("x", univ, vec![FuzzySet::new("mid", MembershipFunction::triangular(0.0, 5.0, 10.0))]);
    let rule = Rule { id: 0, antecedents: vec![AntecedentClause { input: 0, term: 0, hedge: Some(Hedge::Very) }], consequent: Consequent::Mamdani { output: 0, term: 0 }, weight: 1.0, confidence: 1.0 };
    let strength = rule.firing_strength(&[var], &[2.5], TNorm::Min);
    assert!((strength - 0.25).abs() < 1e-9);
}

#[test]
fn rule_base_is_empty_reports_correctly() {
    assert!(RuleBase::new(vec![]).is_empty());
    let rule = Rule { id: 0, antecedents: vec![], consequent: Consequent::SugenoConstant { output: 0, value: 1.0 }, weight: 1.0, confidence: 1.0 };
    assert!(!RuleBase::new(vec![rule]).is_empty());
}

#[test]
fn defuzzifier_empty_universe_errors() {
    let empty = Universe { min: 0.0, max: 1.0, samples: vec![] };
    assert_eq!(Defuzzifier::Centroid.apply(&empty, &[], None, None), Err(FuzzyError::EmptyUniverse));
}

#[test]
fn defuzzifier_weighted_average_and_height_match() {
    let univ = Universe::new(0.0, 1.0, 3).unwrap();
    let membership = vec![0.1, 0.1, 0.1];
    let heights = vec![0.5, 0.8];
    let values = vec![10.0, 20.0];
    let wa = Defuzzifier::WeightedAverage.apply(&univ, &membership, Some(&heights), Some(&values)).unwrap();
    let ht = Defuzzifier::Height.apply(&univ, &membership, Some(&heights), Some(&values)).unwrap();
    assert!((wa - 16.153846153846153).abs() < 1e-9);
    assert_eq!(wa, ht);
}

#[test]
fn defuzzifier_weighted_average_requires_heights_and_values() {
    let univ = Universe::new(0.0, 1.0, 3).unwrap();
    let membership = vec![0.1, 0.1, 0.1];
    assert!(Defuzzifier::WeightedAverage.apply(&univ, &membership, None, None).is_err());
    assert!(Defuzzifier::Height.apply(&univ, &membership, None, None).is_err());
}

#[test]
fn defuzzifier_bisector_mom_som_lom() {
    let univ = Universe::new(0.0, 5.0, 6).unwrap();
    let membership = vec![0.0, 0.2, 0.9, 0.9, 0.9, 0.1];
    let bisector = Defuzzifier::Bisector.apply(&univ, &membership, None, None).unwrap();
    assert!((bisector - 3.0).abs() < 1e-9);
    let mom = Defuzzifier::Mom.apply(&univ, &membership, None, None).unwrap();
    assert!((mom - 3.0).abs() < 1e-9);
    // 🔍️ argmax breaks ties on the *last* max (std max_by semantics), so Som and Lom
    // coincide here even though "smallest of maximum" would conventionally pick x=2.
    let som = Defuzzifier::Som.apply(&univ, &membership, None, None).unwrap();
    let lom = Defuzzifier::Lom.apply(&univ, &membership, None, None).unwrap();
    assert!((som - 4.0).abs() < 1e-9);
    assert!((lom - 4.0).abs() < 1e-9);
}

#[test]
fn mimo_infer_rejects_bad_input_and_empty_rules() {
    let system = temp_speed_system();
    assert_eq!(system.infer(&[1.0, 2.0]), Err(FuzzyError::DimensionMismatch("input count".into())));
    let mut empty_system = system;
    empty_system.rules = RuleBase::new(vec![]);
    assert_eq!(empty_system.infer(&[20.0]), Err(FuzzyError::EmptyRuleBase));
}

#[test]
fn larsen_inference_model_produces_output() {
    let mut system = temp_speed_system();
    system.model = InferenceModel::Larsen;
    let (out, explanation) = system.infer(&[35.0]).unwrap();
    assert!(out[0] >= 0.0);
    assert!(explanation.traces.iter().any(|t| t.description.contains("Larsen")));
}

#[test]
fn tsukamoto_inference_model_inverts_monotonic_consequent() {
    let temp_univ = Universe::new(0.0, 40.0, 41).unwrap();
    let temp_var = LinguisticVariable::new("temperature", temp_univ, vec![FuzzySet::new("high", MembershipFunction::triangular(0.0, 40.0, 40.0))]);
    let out_univ = Universe::new(0.0, 100.0, 101).unwrap();
    let out_var = LinguisticVariable::new("speed", out_univ, vec![FuzzySet::new("level", MembershipFunction::sigmoid(0.2, 50.0))]);
    let system = MimoSystem {
        inputs: vec![temp_var],
        outputs: vec![out_var],
        rules: RuleBase::new(vec![Rule { id: 0, antecedents: vec![AntecedentClause { input: 0, term: 0, hedge: None }], consequent: Consequent::Tsukamoto { output: 0, term: 0 }, weight: 1.0, confidence: 1.0 }]),
        model: InferenceModel::Tsukamoto,
        tnorm: TNorm::Min,
        tconorm: TConorm::Max,
        defuzzifier: Defuzzifier::Centroid,
    };
    let (out, _) = system.infer(&[30.0]).unwrap();
    assert!(out[0] > 0.0 && out[0] < 100.0);
}

#[test]
fn soft_constraint_consequent_scales_by_preference() {
    let system = temp_speed_system();
    let mut soft_system = system;
    soft_system.rules = RuleBase::new(vec![Rule { id: 0, antecedents: vec![AntecedentClause { input: 0, term: 1, hedge: None }], consequent: Consequent::SoftConstraint { output: 0, term: 1, preference: 0.5 }, weight: 1.0, confidence: 1.0 }]);
    let (out, explanation) = soft_system.infer(&[35.0]).unwrap();
    assert!(out[0] >= 0.0);
    assert!(explanation.traces.iter().any(|t| t.description.contains("soft constraint")));
}

#[test]
fn tsukamoto_inverse_handles_zero_alpha_and_solves_monotonic() {
    let mf = MembershipFunction::sigmoid(1.0, 0.0);
    assert_eq!(tsukamoto_inverse(&mf, 0.0), mf.support_min());
    let x = tsukamoto_inverse(&mf, 0.5);
    assert!((mf.eval(x) - 0.5).abs() < 1e-6);
}

#[test]
fn adaptive_membership_fit_updates_parameters() {
    let mf = MembershipFunction::triangular(0.0, 3.0, 10.0);
    let samples: Vec<(f64, f64)> = (0..10)
        .map(|i| {
            let x = i as f64;
            (x, MembershipFunction::triangular(0.0, 5.0, 10.0).eval(x))
        })
        .collect();
    let mut adaptive = AdaptiveMembership::new(mf, 0.05);
    let initial_params = adaptive.mf.parameters();
    let loss = adaptive.fit(&samples, 10);
    assert!(loss.is_finite() && loss >= 0.0);
    assert_ne!(adaptive.mf.parameters(), initial_params);
}

#[test]
fn anfis_default_forward_and_rule_count() {
    let anfis = Anfis::new(2, 2, &[(0.0, 1.0), (0.0, 1.0)]);
    assert_eq!(anfis.rule_count(), 4);
    assert_eq!(anfis.forward(&[0.5, 0.5]), 0.0);
}

#[test]
fn wang_mendel_rules_generates_one_rule_per_sample() {
    let univ = Universe::new(0.0, 10.0, 11).unwrap();
    let input_var = LinguisticVariable::new("x", univ.clone(), vec![FuzzySet::new("low", MembershipFunction::triangular(0.0, 0.0, 10.0)), FuzzySet::new("high", MembershipFunction::triangular(0.0, 10.0, 10.0))]);
    let output_var = LinguisticVariable::new("y", univ, vec![FuzzySet::new("low", MembershipFunction::triangular(0.0, 0.0, 10.0)), FuzzySet::new("high", MembershipFunction::triangular(0.0, 10.0, 10.0))]);
    let data = vec![(vec![1.0], 2.0), (vec![9.0], 8.0)];
    let rules = wang_mendel_rules(&[input_var], &output_var, &data, InferenceModel::Mamdani);
    assert_eq!(rules.rules.len(), 2);
    assert!(matches!(rules.rules[0].consequent, Consequent::Mamdani { .. }));
}

#[test]
fn wang_mendel_rules_sugeno_uses_constant_consequent() {
    let univ = Universe::new(0.0, 10.0, 11).unwrap();
    let input_var = LinguisticVariable::new("x", univ.clone(), vec![FuzzySet::new("low", MembershipFunction::triangular(0.0, 0.0, 10.0))]);
    let output_var = LinguisticVariable::new("y", univ, vec![FuzzySet::new("low", MembershipFunction::triangular(0.0, 0.0, 10.0))]);
    let data = vec![(vec![1.0], 3.5)];
    let rules = wang_mendel_rules(&[input_var], &output_var, &data, InferenceModel::Sugeno);
    assert!(matches!(rules.rules[0].consequent, Consequent::SugenoConstant { value, .. } if (value - 3.5).abs() < 1e-9));
}

#[test]
fn prune_rules_filters_below_thresholds() {
    let rules = RuleBase::new(vec![
        Rule { id: 0, antecedents: vec![], consequent: Consequent::SugenoConstant { output: 0, value: 1.0 }, weight: 0.9, confidence: 0.9 },
        Rule { id: 1, antecedents: vec![], consequent: Consequent::SugenoConstant { output: 0, value: 1.0 }, weight: 0.1, confidence: 0.9 },
    ]);
    let pruned = prune_rules(rules, 0.5, 0.5);
    assert_eq!(pruned.rules.len(), 1);
    assert_eq!(pruned.rules[0].id, 0);
}

#[test]
fn weight_rules_by_fit_updates_confidence_from_error() {
    let mut system = temp_speed_system();
    let data = vec![(vec![35.0], vec![90.0]), (vec![2.0], vec![5.0])];
    weight_rules_by_fit(&mut system, &data);
    for rule in &system.rules.rules {
        assert!((0.0..=1.0).contains(&rule.confidence));
    }
}

#[test]
fn subtractive_cluster_centers_finds_clusters() {
    let data = vec![vec![0.0, 0.0], vec![0.2, 0.1], vec![10.0, 10.0], vec![10.1, 9.9]];
    let centers = subtractive_cluster_centers(&data, 1.0, 0.5, 0.15);
    assert!(!centers.is_empty());
    assert!(centers.len() <= data.len());
}

#[test]
fn subtractive_cluster_centers_empty_data_returns_empty() {
    assert!(subtractive_cluster_centers(&[], 1.0, 0.5, 0.15).is_empty());
}

#[test]
fn gustafson_kessel_clusters_two_groups() {
    let data = vec![vec![0.0, 0.0], vec![0.1, 0.0], vec![5.0, 5.0], vec![5.1, 5.0]];
    let result = gustafson_kessel(&data, 2, 2.0, 5).unwrap();
    assert_eq!(result.centers.len(), 2);
    assert_eq!(result.membership.len(), 4);
}

#[test]
fn fuzzy_vikor_ranks_compromise_solution() {
    let vikor = FuzzyVikor {
        alternatives: vec!["a".into(), "b".into()],
        criteria: vec!["c".into()],
        decision_matrix: vec![vec![FuzzyNumber::triangular(8.0, 9.0, 10.0)], vec![FuzzyNumber::triangular(1.0, 2.0, 3.0)]],
        weights: vec![1.0],
        benefit: vec![true],
        v: 0.5,
    };
    let rank = vikor.rank();
    assert_eq!(rank[0].0, 0);
}

#[test]
fn temporal_evaluator_recently_and_frequently() {
    let eval = TemporalEvaluator { now: 100.0, recent_window: 10.0, frequent_window: 5.0 };
    assert!((eval.recently(100.0) - 1.0).abs() < 1e-9);
    assert_eq!(eval.recently(90.0), 0.0);
    assert!((eval.frequently(&[99.0, 98.0, 97.0]) - 0.6).abs() < 1e-9);
}

#[test]
fn spatial_evaluator_near_and_slowly() {
    let eval = SpatialEvaluator { anchor: vec![0.0, 0.0], near_radius: 10.0 };
    assert!((eval.near(&[0.0, 0.0]) - 1.0).abs() < 1e-9);
    assert!(eval.near(&[20.0, 0.0]) <= 0.0);
    assert!((eval.slowly(0.0, 10.0) - 1.0).abs() < 1e-9);
    assert_eq!(eval.slowly(10.0, 10.0), 0.0);
}

#[test]
fn probabilistic_to_membership_blends_by_certainty() {
    assert!((probabilistic_to_membership(0.8, 1.0) - 0.8).abs() < 1e-9);
    assert!((probabilistic_to_membership(0.8, 0.0) - 0.5).abs() < 1e-9);
}

#[test]
fn possibility_to_membership_passes_through_values() {
    let pm = PossibilityMeasure::new(vec![1.0, 2.0], vec![0.3, 0.7]).unwrap();
    assert_eq!(possibility_to_membership(&pm), vec![0.3, 0.7]);
}

#[test]
fn hybrid_fuse_weighted_combination() {
    assert!((hybrid_fuse(0.8, 0.2, 1.0) - 0.8).abs() < 1e-9);
    assert!((hybrid_fuse(0.8, 0.2, 0.0) - 0.2).abs() < 1e-9);
    assert!((hybrid_fuse(0.8, 0.2, 0.5) - 0.5).abs() < 1e-9);
}

mod long {
    use super::*;

    #[test]
    fn anfis_learns_nonlinear_surface() {
        let data: Vec<(Vec<f64>, f64)> = (0..30)
            .flat_map(|i| {
                (0..30).map(move |j| {
                    let x = i as f64 / 10.0;
                    let y = j as f64 / 10.0;
                    (vec![x, y], (x * y).sin())
                })
            })
            .collect();
        let mut anfis = Anfis::new(2, 3, &[(0.0, 3.0), (0.0, 3.0)]);
        let loss = anfis.fit_hybrid(&data, 5);
        let pred = anfis.forward(&[1.5, 1.5]);
        assert!(loss.is_finite());
        assert!(pred.is_finite());
    }

    #[test]
    fn genetic_optimizer_improves_quadratic() {
        let opt = GeneticOptimizer { population_size: 20, generations: 30, mutation_rate: 0.2, crossover_rate: 0.7, bounds: vec![(-5.0, 5.0)], seed: 42 };
        let (best, fit) = opt.optimize(|x| (x[0] - 2.0).powi(2));
        assert!(fit < 1.0);
        assert!((best[0] - 2.0).abs() < 1.5);
    }

    #[test]
    fn pso_optimizer_finds_minimum() {
        let opt = PsoOptimizer { swarm_size: 15, iterations: 40, inertia: 0.7, cognitive: 1.4, social: 1.4, bounds: vec![(-3.0, 3.0), (-3.0, 3.0)], seed: 7 };
        let (best, fit) = opt.optimize(|x| x[0] * x[0] + x[1] * x[1]);
        assert!(fit < 0.5);
        assert!(best[0].abs() < 1.0 && best[1].abs() < 1.0);
    }

    #[test]
    fn hierarchical_system_chains_layers() {
        let layer1 = temp_speed_system();
        let layer2_univ = Universe::new(0.0, 100.0, 21).unwrap();
        let layer2 = MimoSystem {
            inputs: vec![LinguisticVariable::new("fan_speed", layer2_univ, vec![FuzzySet::new("any", MembershipFunction::triangular(0.0, 50.0, 100.0))])],
            outputs: vec![LinguisticVariable::new("power", Universe::new(0.0, 1.0, 21).unwrap(), vec![FuzzySet::new("high", MembershipFunction::triangular(0.5, 1.0, 1.0))])],
            rules: RuleBase::new(vec![Rule { id: 0, antecedents: vec![AntecedentClause { input: 0, term: 0, hedge: None }], consequent: Consequent::SugenoConstant { output: 0, value: 0.9 }, weight: 1.0, confidence: 1.0 }]),
            model: InferenceModel::Sugeno,
            tnorm: TNorm::Min,
            tconorm: TConorm::Max,
            defuzzifier: Defuzzifier::Centroid,
        };
        let hier = HierarchicalSystem::new(vec![layer1, layer2]);
        let (out, explanations) = hier.infer(&[35.0]).unwrap();
        assert_eq!(out.len(), 1);
        assert_eq!(explanations.len(), 2);
    }

    #[test]
    fn evolving_system_adapts_over_stream() {
        let system = temp_speed_system();
        let mut evolving = EvolvingFuzzySystem::new(system, 0.1, 0.05, 10);
        for t in (20..30).map(|x| x as f64) {
            let _ = evolving.observe(&[t], &[80.0]);
        }
        assert!(!evolving.system.rules.rules.is_empty());
    }
}
