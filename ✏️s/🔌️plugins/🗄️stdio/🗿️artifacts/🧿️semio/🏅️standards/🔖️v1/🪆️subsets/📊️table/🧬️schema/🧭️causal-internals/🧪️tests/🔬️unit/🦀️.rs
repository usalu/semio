
use super::*;

#[test]
fn causal_error_contract_preserves_transparent_sources_and_conversions() {
    let simple = [
        (CausalError::VariableNotFound("x".into()), "variable `x` not found".to_string()),
        (CausalError::NotADag(vec![2, 0]), "edges contain a cycle through node indices [2, 0]".to_string()),
        (CausalError::ColumnType(3, "continuous"), "column 3 has wrong type: expected continuous".to_string()),
        (CausalError::DimensionMismatch("covariance".into()), "dimension mismatch: covariance".to_string()),
        (CausalError::NotIdentifiable("backdoor".into()), "effect not identifiable: backdoor".to_string()),
        (CausalError::Singular("regression"), "singular linear system in regression".to_string()),
        (CausalError::InvalidQuery("empty".into()), "invalid query: empty".to_string()),
        (CausalError::InferenceTooLarge(32, 16), "inference too large: factor would have 32 entries (limit 16)".to_string()),
    ];
    for (error, message) in simple {
        assert_eq!(error.to_string(), message);
        assert!(std::error::Error::source(&error).is_none());
    }
    let stats: CausalError = crate::standards::v1::subsets::table::schema::statistics_internals::StatisticsError::SingularMatrix.into();
    let tabular: CausalError = crate::standards::v1::subsets::table::schema::tabular_internals::TabularError::IndexOutOfBounds(5).into();
    let probability: CausalError = crate::standards::v1::subsets::table::schema::probability_internals::ProbabilityError::NoConvergence { what: "sample" }.into();
    for error in [&stats, &tabular, &probability] {
        assert!(std::error::Error::source(error).is_some());
    }
    assert_eq!(stats.to_string(), "singular matrix");
    assert_eq!(tabular.to_string(), "column index 5 out of bounds");
    assert_eq!(probability.to_string(), "no convergence in sample");
}

// #region 🔖️Fixtures
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn chain3() -> CausalDag {
    // x -> y -> z
    CausalDag::from_named_edges(vec!["x".into(), "y".into(), "z".into()], &[("x", "y"), ("y", "z")]).unwrap()
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn fork3() -> CausalDag {
    // x <- y -> z
    CausalDag::from_named_edges(vec!["x".into(), "y".into(), "z".into()], &[("y", "x"), ("y", "z")]).unwrap()
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn collider3() -> CausalDag {
    // x -> z <- y
    CausalDag::from_named_edges(vec!["x".into(), "y".into(), "z".into()], &[("x", "z"), ("y", "z")]).unwrap()
}
// #endregion 🔖️Fixtures

// #region 🔖️DSeparationTests
#[test]
fn chain_d_separation_pattern() {
    let dag = chain3();
    assert!(d_separated(&dag, &[0], &[2], &[1]), "x _||_ z | y should hold on a chain");
    assert!(!d_separated(&dag, &[0], &[2], &[]), "x _||_ z should not hold marginally on a chain");
}

#[test]
fn fork_d_separation_pattern() {
    let dag = fork3();
    assert!(d_separated(&dag, &[0], &[2], &[1]), "x _||_ z | y should hold on a fork");
    assert!(!d_separated(&dag, &[0], &[2], &[]), "x _||_ z should not hold marginally on a fork");
}

#[test]
fn collider_d_separation_pattern() {
    let dag = collider3();
    assert!(d_separated(&dag, &[0], &[1], &[]), "x _||_ y should hold marginally on a collider");
    assert!(!d_separated(&dag, &[0], &[1], &[2]), "x _||_ y | z should not hold: conditioning on the collider opens the path");
}

#[test]
fn moralization_marries_coparents() {
    let dag = collider3();
    let moral = dag.moralize();
    assert!(moral.contains(&(0, 1)), "co-parents 0 and 1 of the collider should be married");
}

#[test]
fn implied_independencies_hold_by_d_separation() {
    let dag = chain3();
    for stmt in implied_independencies(&dag) {
        assert!(d_separated(&dag, &[stmt.x], &[stmt.y], &stmt.z), "implied statement {stmt:?} should be d-separated");
    }
}
// #endregion 🔖️DSeparationTests

// #region 🔖️CpdagTests
#[test]
fn cpdag_from_chain_is_fully_undirected() {
    let cpdag = Cpdag::from_dag(&chain3());
    assert!(cpdag.is_undirected(0, 1));
    assert!(cpdag.is_undirected(1, 2));
    assert!(cpdag.directed_edges().is_empty());
}

#[test]
fn cpdag_from_collider_keeps_both_arrows() {
    let cpdag = Cpdag::from_dag(&collider3());
    assert!(cpdag.is_directed(0, 2));
    assert!(cpdag.is_directed(1, 2));
    assert!(!cpdag.has_edge(0, 1), "collider parents should not be adjacent");
}

#[test]
fn cpdag_to_dag_round_trip_preserves_skeleton_and_v_structures() {
    let dag = collider3();
    let cpdag = Cpdag::from_dag(&dag);
    let extended = cpdag.to_dag().expect("collider CPDAG is extendable");
    assert_eq!(extended.edges().len(), dag.edges().len());
    assert!(extended.parents(2).contains(&0) && extended.parents(2).contains(&1));
}

#[test]
fn cpdag_to_dag_round_trip_on_chain_is_acyclic_and_same_skeleton() {
    let dag = chain3();
    let cpdag = Cpdag::from_dag(&dag);
    let extended = cpdag.to_dag().expect("chain CPDAG is extendable");
    assert_eq!(extended.edges().len(), 2);
}

#[test]
fn meek_rule1_orients_to_avoid_new_collider() {
    // a -> b, b - c undirected, a and c not adjacent => b -> c.
    let mut cpdag = Cpdag::new(vec!["a".into(), "b".into(), "c".into()]);
    cpdag.directed.insert((0, 1));
    cpdag.undirected.insert((1, 2));
    apply_meek_rules(&mut cpdag);
    assert!(cpdag.is_directed(1, 2));
}

#[test]
fn meek_rule2_orients_to_avoid_cycle() {
    // a -> b -> c, a - c undirected => a -> c.
    let mut cpdag = Cpdag::new(vec!["a".into(), "b".into(), "c".into()]);
    cpdag.directed.insert((0, 1));
    cpdag.directed.insert((1, 2));
    cpdag.undirected.insert((0, 2));
    apply_meek_rules(&mut cpdag);
    assert!(cpdag.is_directed(0, 2));
}

#[test]
fn meek_rule3_orients_via_two_directed_co_parents() {
    // a-b, a-c, a-d undirected; c->b, d->b directed; c,d not adjacent => a->b.
    let mut cpdag = Cpdag::new(vec!["a".into(), "b".into(), "c".into(), "d".into()]);
    cpdag.undirected.insert((0, 1));
    cpdag.undirected.insert((0, 2));
    cpdag.undirected.insert((0, 3));
    cpdag.directed.insert((2, 1));
    cpdag.directed.insert((3, 1));
    apply_meek_rules(&mut cpdag);
    assert!(cpdag.is_directed(0, 1));
}

// #endregion 🔖️CpdagTests

// #region 🔖️CiTestTests
#[test]
fn fisher_z_ci_test_via_causal_dag_columns() {
    let mut table = crate::standards::v1::subsets::table::schema::tabular_internals::Table::new();
    let n = 200;
    let mut rng = semio_framework_geometry::random::Rng::from_seed(7);
    let x: Vec<f64> = (0..n).map(|_| crate::standards::v1::subsets::table::schema::probability_internals::Normal::STANDARD.sample(&mut rng)).collect();
    let y: Vec<f64> = x.iter().map(|&xi| xi * 0.8 + crate::standards::v1::subsets::table::schema::probability_internals::Normal::STANDARD.sample(&mut rng) * 0.2).collect();
    let z: Vec<f64> = y.iter().map(|&yi| yi * 0.8 + crate::standards::v1::subsets::table::schema::probability_internals::Normal::STANDARD.sample(&mut rng) * 0.2).collect();
    table.push_continuous("x", x).unwrap();
    table.push_continuous("y", y).unwrap();
    table.push_continuous("z", z).unwrap();
    let ci = FisherZ::for_table(&table, &[0, 1, 2]).unwrap();
    let marginal = ci.test(&table, 0, 2, &[]).unwrap();
    let conditional = ci.test(&table, 0, 2, &[1]).unwrap();
    assert!(marginal.p_value < 0.05, "x,z should look dependent marginally");
    assert!(conditional.p_value > 0.05, "x,z should look independent given y on a chain");
}
// #endregion 🔖️CiTestTests

// #region 🔖️DiscoveryTests
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn linear_chain_scm() -> LinearGaussianScm {
    // x -> m -> y, coefficients 2.0 and 1.5.
    let dag = CausalDag::from_named_edges(vec!["x".into(), "m".into(), "y".into()], &[("x", "m"), ("m", "y")]).unwrap();
    let mut weights = crate::standards::v1::subsets::value::schema::algebra_internals::MatD::zeros(3, 3);
    weights.set(1, 0, 2.0);
    weights.set(2, 1, 1.5);
    LinearGaussianScm {
        dag,
        weights,
        intercepts: crate::standards::v1::subsets::value::schema::algebra_internals::VecD::from_vec(vec![0.0, 0.0, 0.0]),
        noise_var: crate::standards::v1::subsets::value::schema::algebra_internals::VecD::from_vec(vec![1.0, 0.25, 0.25]),
    }
}

#[test]
fn local_bic_prefers_the_true_parent_set() {
    let scm = linear_chain_scm();
    let mut rng = semio_framework_geometry::random::Rng::from_seed(99);
    let mut data = scm.simulate(500, &mut rng).unwrap();
    // An exogenous, causally unrelated column — unlike `y` (a downstream descendant of `m`
    // in the chain, and thus itself strongly correlated with `m`), this one has no relationship
    // to `m` at all, so it is a genuine negative control for "does BIC reward a real parent".
    let unrelated: Vec<f64> = (0..data.n_rows()).map(|_| crate::standards::v1::subsets::table::schema::probability_internals::Normal::STANDARD.sample(&mut rng)).collect();
    data.push_continuous("unrelated", unrelated).unwrap();
    let with_true_parent = local_bic(&data, 1, &[0]).unwrap();
    let with_no_parent = local_bic(&data, 1, &[]).unwrap();
    let with_unrelated_covariate = local_bic(&data, 1, &[3]).unwrap();
    assert!(with_true_parent > with_no_parent, "true parent should score higher than no parent");
    assert!(with_true_parent > with_unrelated_covariate, "true parent should score higher than an unrelated covariate");
}
// #endregion 🔖️DiscoveryTests

// #region 🔖️IdentificationTests
#[test]
fn backdoor_minimal_set_on_confounder_triangle() {
    // z -> x, z -> y, x -> y.
    let dag = CausalDag::from_named_edges(vec!["x".into(), "y".into(), "z".into()], &[("z", "x"), ("z", "y"), ("x", "y")]).unwrap();
    let sets = minimal_backdoor_sets(&dag, 0, 1, 3);
    assert_eq!(sets, vec![vec![2]]);
}

#[test]
fn backdoor_m_bias_graph_allows_empty_adjustment() {
    // a -> x, b -> y, a -> m, b -> m (m is a collider, not on any backdoor path): x -> y direct edge.
    let dag = CausalDag::from_named_edges(vec!["x".into(), "y".into(), "a".into(), "b".into(), "m".into()], &[("a", "x"), ("b", "y"), ("a", "m"), ("b", "m"), ("x", "y")]).unwrap();
    assert!(backdoor_satisfied(&dag, 0, 1, &[]), "no backdoor path from x to y should exist here");
}

#[test]
fn frontdoor_satisfied_on_classic_mediator_graph() {
    // u -> x, u -> y (unobserved confounder u), x -> m -> y.
    let dag = CausalDag::from_named_edges(vec!["x".into(), "y".into(), "m".into(), "u".into()], &[("u", "x"), ("u", "y"), ("x", "m"), ("m", "y")]).unwrap();
    assert!(frontdoor_satisfied(&dag, 0, 1, &[2]));
}

#[test]
fn identify_returns_no_confounding_for_a_direct_unconfounded_edge() {
    let dag = CausalDag::from_named_edges(vec!["x".into(), "y".into()], &[("x", "y")]).unwrap();
    assert_eq!(identify(&dag, 0, 1).unwrap(), Identification::NoConfounding);
}

#[test]
fn identify_returns_backdoor_for_a_confounded_edge() {
    let dag = CausalDag::from_named_edges(vec!["x".into(), "y".into(), "z".into()], &[("z", "x"), ("z", "y"), ("x", "y")]).unwrap();
    assert_eq!(identify(&dag, 0, 1).unwrap(), Identification::Backdoor { adjustment: vec![2] });
}
// #endregion 🔖️IdentificationTests

// #region 🔖️ScmLinearTests
#[test]
fn linear_scm_total_effect_matches_analytic_path_product() {
    let scm = linear_chain_scm();
    assert!((scm.total_effect(0, 2) - 3.0).abs() < 1e-9);
    assert!((scm.ate(0, 2) - 3.0).abs() < 1e-9);
}

#[test]
fn linear_scm_implied_covariance_matches_hand_computation() {
    let scm = linear_chain_scm();
    let cov = scm.implied_covariance().unwrap();
    // Var(x) = 1.0; Var(m) = 4*1.0 + 0.25 = 4.25; Var(y) = 1.5^2*4.25 + 0.25 = 9.8125; Cov(x,y) = 2.0*1.5*Var(x) = 3.0.
    assert!((cov.get(0, 0) - 1.0).abs() < 1e-9);
    assert!((cov.get(1, 1) - 4.25).abs() < 1e-9);
    assert!((cov.get(2, 2) - 9.8125).abs() < 1e-9);
    assert!((cov.get(0, 2) - 3.0).abs() < 1e-9);
}

#[test]
fn linear_scm_counterfactual_shifts_by_exact_path_product_preserving_noise() {
    let scm = linear_chain_scm();
    let observed = [1.0, 2.5, 5.0]; // noise_m = 2.5 - 2.0*1.0 = 0.5; noise_y = 5.0 - 1.5*2.5 = 1.25
    let cf = scm.counterfactual(&observed, &[(0, 2.0)]).unwrap();
    // do(x=2.0): m = 2.0*2.0 + 0.5 = 4.5; y = 1.5*4.5 + 1.25 = 8.0
    assert!((cf.get(1) - 4.5).abs() < 1e-9);
    assert!((cf.get(2) - 8.0).abs() < 1e-9);
    // shift in y should equal exactly a*b*(delta x) = 2.0*1.5*1.0 = 3.0
    assert!((cf.get(2) - observed[2] - 3.0).abs() < 1e-9);
}

#[test]
fn what_if_query_level2_vs_level3() {
    let scm = linear_chain_scm();
    let interventional = scm.query(2, &WhatIf::new().do_(0, 2.0)).unwrap();
    // E[y | do(x=2)] = 1.5 * (2.0 * 2.0) = 6.0 (all intercepts are 0 in this fixture).
    assert!((interventional - 6.0).abs() < 1e-9);
    let observed = [1.0, 2.5, 5.0];
    let what_if = WhatIf::new().do_(0, 2.0).given(0, observed[0]).given(1, observed[1]).given(2, observed[2]);
    let counterfactual = scm.query(2, &what_if).unwrap();
    assert!((counterfactual - 8.0).abs() < 1e-9);
}
// #endregion 🔖️ScmLinearTests

// #region 🔖️ScmDiscreteTests
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sprinkler_scm() -> DiscreteScm {
    // Classic sprinkler net: Cloudy -> {Sprinkler, Rain} -> Wet. All variables binary (0=false, 1=true).
    let dag = CausalDag::from_named_edges(vec!["cloudy".into(), "sprinkler".into(), "rain".into(), "wet".into()], &[("cloudy", "sprinkler"), ("cloudy", "rain"), ("sprinkler", "wet"), ("rain", "wet")]).unwrap();
    let cardinalities = vec![2, 2, 2, 2];
    let cloudy = Cpt { node: 0, parents: vec![], cardinality: 2, probs: vec![0.5, 0.5] };
    // sprinkler | cloudy: P(sprinkler=1|cloudy=0)=0.5, P(sprinkler=1|cloudy=1)=0.1
    let sprinkler = Cpt { node: 1, parents: vec![0], cardinality: 2, probs: vec![0.5, 0.5, 0.9, 0.1] };
    // rain | cloudy: P(rain=1|cloudy=0)=0.2, P(rain=1|cloudy=1)=0.8
    let rain = Cpt { node: 2, parents: vec![0], cardinality: 2, probs: vec![0.8, 0.2, 0.2, 0.8] };
    // wet | sprinkler, rain (config mixed-radix: sprinkler fastest)
    let wet = Cpt {
        node: 3,
        parents: vec![1, 2],
        cardinality: 2,
        probs: vec![
            1.0, 0.0, // sprinkler=0,rain=0
            0.1, 0.9, // sprinkler=1,rain=0
            0.1, 0.9, // sprinkler=0,rain=1
            0.01, 0.99, // sprinkler=1,rain=1
        ],
    };
    DiscreteScm { dag, cardinalities, cpts: vec![cloudy, sprinkler, rain, wet] }
}

#[test]
fn sprinkler_posterior_vs_interventional_distribution_contrast() {
    let scm = sprinkler_scm();
    let posterior_rain = scm.posterior(2, &[(3, 1)]).unwrap();
    let prior_rain = scm.posterior(2, &[]).unwrap();
    assert!(posterior_rain[1] > prior_rain[1], "observing wet grass should raise P(rain)");

    let interventional_rain = scm.interventional_distribution(2, &[(1, 1)], &[]).unwrap();
    assert!((interventional_rain[1] - prior_rain[1]).abs() < 1e-9, "do(sprinkler=1) should not change the marginal of rain: intervening breaks the backdoor path through cloudy");

    let conditional_on_sprinkler = scm.posterior(2, &[(1, 1)]).unwrap();
    assert!((conditional_on_sprinkler[1] - prior_rain[1]).abs() > 1e-9, "merely conditioning on sprinkler=1 (not intervening) should shift P(rain) via the cloudy backdoor path");
}

// #endregion 🔖️ScmDiscreteTests

// #region 🔖️EstimationTests
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn confounded_dataset(true_ate: f64, n: usize, seed: u64) -> crate::standards::v1::subsets::table::schema::tabular_internals::Table {
    let mut rng = semio_framework_geometry::random::Rng::from_seed(seed);
    let z: Vec<f64> = (0..n).map(|_| crate::standards::v1::subsets::table::schema::probability_internals::Normal::STANDARD.sample(&mut rng)).collect();
    let t: Vec<f64> = z.iter().map(|&zi| f64::from(u8::from(zi + crate::standards::v1::subsets::table::schema::probability_internals::Normal::STANDARD.sample(&mut rng) > 0.0))).collect();
    let y: Vec<f64> = z.iter().zip(&t).map(|(&zi, &ti)| 2.0 * zi + true_ate * ti + crate::standards::v1::subsets::table::schema::probability_internals::Normal::STANDARD.sample(&mut rng) * 0.5).collect();
    crate::standards::v1::subsets::table::schema::tabular_internals::Table::from_f64_columns(vec!["z".into(), "t".into(), "y".into()], vec![z, t, y]).unwrap()
}

// #endregion 🔖️EstimationTests

// #region 🔖️ErrorPathTests
#[test]
fn cyclic_edges_are_rejected() {
    let err = CausalDag::new(vec!["a".into(), "b".into()], &[(0, 1), (1, 0)]).unwrap_err();
    assert!(matches!(err, CausalError::NotADag(_)));
}

#[test]
fn wrong_column_type_errors() {
    let mut table = crate::standards::v1::subsets::table::schema::tabular_internals::Table::new();
    table.push_continuous("x", vec![1.0, 2.0]).unwrap();
    let err = GSquared.test(&table, 0, 0, &[]).unwrap_err();
    assert!(matches!(err, CausalError::Tabular(_)));
}

#[test]
fn oversized_variable_elimination_is_rejected() {
    // 20 independent binary roots all feeding one "sink" child: the sink's own CPT factor
    // already has 2^21 entries (21 binary variables), well past the 1e6-entry guard, and
    // eliminating any root multiplies straight into that factor.
    let n_parents = 20;
    let mut names = vec!["sink".to_string()];
    names.extend((0..n_parents).map(|i| format!("p{i}")));
    let edges: Vec<(usize, usize)> = (0..n_parents).map(|i| (i + 1, 0usize)).collect();
    let dag = CausalDag::new(names, &edges).unwrap();
    let cardinalities = vec![2usize; n_parents + 1];
    let mut cpts = vec![Cpt { node: 0, parents: (1..=n_parents).collect(), cardinality: 2, probs: vec![0.5; 2usize.pow(n_parents as u32) * 2] }];
    for i in 0..n_parents {
        cpts.push(Cpt { node: i + 1, parents: vec![], cardinality: 2, probs: vec![0.5, 0.5] });
    }
    let scm = DiscreteScm { dag, cardinalities, cpts };
    let result = scm.posterior(1, &[]);
    assert!(matches!(result, Err(CausalError::InferenceTooLarge(_, _))), "posterior over a 2^21-entry join should exceed the guard");
}
// #endregion 🔖️ErrorPathTests

// #region 🔖️QuickTests
// 🐢️ Tests that simulate/fit at a scale (thousands of rows, hundreds of bootstrap replicates)
// needed for statistical power, too slow for the 15s "fundamental" budget — see
// `repo/lib/js/index.ts`'s `runCargoTestBudgeted`/`TEST_LEVEL_BUDGET_MS`.
mod quick {
    use super::*;

    #[test]
    fn pc_stable_recovers_known_cpdag_from_simulated_chain() {
        let scm = linear_chain_scm();
        let mut rng = semio_framework_geometry::random::Rng::from_seed(2024);
        let data = scm.simulate(2000, &mut rng).unwrap();
        let ci = FisherZ::for_table(&data, &[0, 1, 2]).unwrap();
        let result = pc_stable(&data, &ci, PcOptions { alpha: 0.01, max_cond_size: 2 }).unwrap();
        let truth = Cpdag::from_dag(&scm.dag);
        assert_eq!(result.cpdag.directed_edges(), truth.directed_edges());
        assert_eq!(result.cpdag.undirected_edges(), truth.undirected_edges());
    }

    #[test]
    fn ges_recovers_known_cpdag_from_simulated_chain() {
        let scm = linear_chain_scm();
        let mut rng = semio_framework_geometry::random::Rng::from_seed(4242);
        let data = scm.simulate(2000, &mut rng).unwrap();
        let found = ges(&data).unwrap();
        let truth = Cpdag::from_dag(&scm.dag);
        assert_eq!(found.directed_edges(), truth.directed_edges());
        assert_eq!(found.undirected_edges(), truth.undirected_edges());
    }

    #[test]
    fn direct_lingam_recovers_causal_order_on_uniform_noise_sem() {
        // x -> y with uniform (non-Gaussian) noise.
        let n = 3000;
        let mut rng = semio_framework_geometry::random::Rng::from_seed(55);
        let x: Vec<f64> = (0..n).map(|_| crate::standards::v1::subsets::table::schema::probability_internals::Uniform::new(-1.0, 1.0).unwrap().sample(&mut rng)).collect();
        let y: Vec<f64> = x.iter().map(|&xi| 2.0 * xi + crate::standards::v1::subsets::table::schema::probability_internals::Uniform::new(-1.0, 1.0).unwrap().sample(&mut rng)).collect();
        let table = crate::standards::v1::subsets::table::schema::tabular_internals::Table::from_f64_columns(vec!["x".into(), "y".into()], vec![x, y]).unwrap();
        let result = direct_lingam(&table, 0.05).unwrap();
        assert_eq!(result.order[0], 0, "x should be recovered as the earlier (more exogenous) variable");
        assert!(result.dag.parents(1).contains(&0), "y should have x as a parent after pruning");
    }

    #[test]
    fn linear_scm_simulate_then_fit_recovers_coefficients() {
        let scm = linear_chain_scm();
        let mut rng = semio_framework_geometry::random::Rng::from_seed(321);
        let data = scm.simulate(5000, &mut rng).unwrap();
        let refit = LinearGaussianScm::fit(&scm.dag, &data).unwrap();
        assert!((refit.weights.get(1, 0) - 2.0).abs() < 0.1);
        assert!((refit.weights.get(2, 1) - 1.5).abs() < 0.1);
    }

    #[test]
    fn discrete_scm_fit_recovers_cpts_from_simulated_data() {
        let scm = sprinkler_scm();
        let mut rng = semio_framework_geometry::random::Rng::from_seed(17);
        let data = scm.simulate(20_000, &mut rng).unwrap();
        let refit = DiscreteScm::fit(&scm.dag, &data, 1.0).unwrap();
        let cloudy_fit = &refit.cpts[0];
        assert!((cloudy_fit.probs[1] - 0.5).abs() < 0.05, "recovered P(cloudy=1) should be near 0.5, got {}", cloudy_fit.probs[1]);
    }

    #[test]
    fn naive_difference_is_biased_while_adjusted_estimators_recover_true_ate() {
        let true_ate = 1.5;
        let data = confounded_dataset(true_ate, 4000, 42);
        let opts = EstimationOptions::default();
        let naive = naive_difference(&data, 1, 2, &opts).unwrap();
        let g_formula = g_formula_ate(&data, 1, 2, &[0], &opts).unwrap();
        let ipw = ipw_ate(&data, 1, 2, &[0], &opts).unwrap();
        assert!((naive.estimate - true_ate).abs() > 0.3, "naive estimate {} should be visibly biased away from {true_ate}", naive.estimate);
        assert!((g_formula.estimate - true_ate).abs() < 0.2, "g-formula estimate {} should be close to {true_ate}", g_formula.estimate);
        // IPW carries more finite-sample variance than g-formula (propensity-weight reweighting
        // amplifies noise), so it gets a looser tolerance for the same sample size.
        assert!((ipw.estimate - true_ate).abs() < 0.35, "IPW estimate {} should be close to {true_ate}", ipw.estimate);
    }

    #[test]
    fn bootstrap_ci_contains_true_ate() {
        let true_ate = 1.5;
        let data = confounded_dataset(true_ate, 5000, 7);
        let opts = EstimationOptions { bootstrap: Some(BootstrapOptions { replicates: 300, seed: 11, level: 0.95 }) };
        let g_formula = g_formula_ate(&data, 1, 2, &[0], &opts).unwrap();
        let (lo, hi) = g_formula.ci.expect("bootstrap CI requested");
        assert!(lo <= g_formula.estimate && g_formula.estimate <= hi);
        assert!(lo <= true_ate && true_ate <= hi, "95% CI [{lo}, {hi}] should contain the true ATE {true_ate}");
    }
}
// #endregion 🔖️QuickTests
