//! 🔮️ Third-party oracle: `fdg-sim` Fruchterman-Reingold from the same seeded initial positions. Layout quality is
//! compared by normalized stress (scale-invariant: the Euclidean distances are optimally rescaled against the
//! graph-theoretic distances before the squared relative error is averaged over all node pairs).

use super::testing::{layout_run_fdg_layout, layout_run_hop_distances, layout_run_normalized_stress, LayoutRunOracleParameters};
use super::tests::{fixture, generate, new_job, run_to, Driven, Ledger};
use super::*;
use semio_framework_job::INTERACTIVE_LANE_FUEL;

#[test]
fn layout_run_stress_matches_the_fdg_sim_fruchterman_reingold_oracle() {
    let fixture = fixture();
    let oracle = &fixture["oracle"];
    let config: LayoutRunConfig = serde_json::from_value(fixture["defaultConfig"].clone()).expect("default config");
    let parameters: LayoutRunOracleParameters = serde_json::from_value(oracle.clone()).expect("oracle parameters");
    let ratio_max = oracle["stressRatioMax"].as_f64().expect("ratio");
    let improvement_min = oracle["improvementMin"].as_f64().expect("improvement");
    for spec in oracle["cases"].as_array().expect("oracle cases") {
        let graph = generate(spec);
        let hops = layout_run_hop_distances(&graph);
        let mut job = new_job(&graph, config);
        let initial = job.positions();
        let initial_positions: Vec<[f64; 2]> = initial.iter().map(|point| [point.x, point.y]).collect();
        run_to(&mut job, &mut Ledger::default(), INTERACTIVE_LANE_FUEL, Driven::Complete);
        let ours: Vec<[f64; 2]> = job.positions().iter().map(|point| [point.x, point.y]).collect();
        let theirs = layout_run_fdg_layout(&graph, &initial, parameters);
        assert!(theirs.iter().all(|point| point[0].is_finite() && point[1].is_finite()), "the oracle layout is finite");
        let (start, mine, reference) = (layout_run_normalized_stress(&initial_positions, &hops), layout_run_normalized_stress(&ours, &hops), layout_run_normalized_stress(&theirs, &hops));
        let report = format!("{spec}: initial {start:.4}, layout run {mine:.4} after {} iterations, fdg-sim {reference:.4}", job.iteration());
        assert!(reference < start, "the oracle improves the seeded layout ({report})");
        assert!(mine <= start * (1.0 - improvement_min), "the layout run improves the seeded stress by at least {improvement_min} ({report})");
        assert!(mine <= reference * ratio_max, "the layout run's stress stays within {ratio_max}× the oracle's ({report})");
    }
}
