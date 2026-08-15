//! 🪞 GLTF symmetry indicators.

use super::geometric_analysis::{GltfGeometryContext};
use super::super::super::modules::{inference_measures::{estimate, unavailable}, mesh_topology::Topology};
use super::super::super::modules::measurement_contracts::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfSymmetryIndicators {
    pub reflection_symmetry_score: GltfMeasure<f64>,
    pub rotational_symmetry_score: GltfMeasure<f64>,
    pub reflection_symmetries: GltfMeasure<Vec<GltfDirectionScore>>,
    pub rotational_symmetries: GltfMeasure<Vec<GltfDirectionScore>>,
    pub repetition_ratio: GltfMeasure<f64>,
    pub modularity_ratio: GltfMeasure<f64>,
}

fn symmetry_score(points: &[[f64; 3]], centroid: [f64; 3], axis: [f64; 3], scale: f64, rotation: bool, budget: usize) -> f64 {
    if points.is_empty() || scale <= 1e-15 {
        return 1.0;
    }
    let step = (points.len() / budget.max(1)).max(1);
    let mut err = 0.0;
    let mut count = 0;
    for p in points.iter().step_by(step) {
        let rel = [p[0] - centroid[0], p[1] - centroid[1], p[2] - centroid[2]];
        let dot = rel[0] * axis[0] + rel[1] * axis[1] + rel[2] * axis[2];
        let target = if rotation { [2.0 * dot * axis[0] - rel[0], 2.0 * dot * axis[1] - rel[1], 2.0 * dot * axis[2] - rel[2]] } else { [rel[0] - 2.0 * dot * axis[0], rel[1] - 2.0 * dot * axis[1], rel[2] - 2.0 * dot * axis[2]] };
        let closest = points
            .iter()
            .map(|q| {
                let qrel = [q[0] - centroid[0], q[1] - centroid[1], q[2] - centroid[2]];
                (target[0] - qrel[0]).powi(2) + (target[1] - qrel[1]).powi(2) + (target[2] - qrel[2]).powi(2)
            })
            .fold(f64::INFINITY, f64::min)
            .sqrt();
        err += (closest / scale).min(1.0);
        count += 1;
    }
    if count == 0 {
        1.0
    } else {
        (1.0 - err / count as f64).max(0.0)
    }
}

pub struct GltfSymmetryInference;

impl GltfSymmetryInference {
    pub(crate) fn infer_assembly(indicators: &mut GltfSymmetryIndicators, parts: &[super::geometric_analysis::GltfPartInference], policy: &GltfAnalysisPolicy, topology: Topology) {
        if parts.is_empty() {
            return;
        }
        let signature = |part: &super::geometric_analysis::GltfPartInference| {
            let mut dimensions = part.indicators.size.oriented_bounds.value.as_ref().map(|bounds| bounds.dimensions.array()).unwrap_or([0.0; 3]);
            dimensions.sort_by(f64::total_cmp);
            let quantum = policy.absolute_length_tolerance.max(1e-9);
            let area_quantum = quantum * quantum;
            let volume_quantum = area_quantum * quantum;
            format!(
                "{},{},{},{},{}",
                (dimensions[0] / quantum).round() as i64,
                (dimensions[1] / quantum).round() as i64,
                (dimensions[2] / quantum).round() as i64,
                (part.indicators.area_volume.surface_area.value.unwrap_or(0.0) / area_quantum).round() as i64,
                (part.indicators.area_volume.volume.value.unwrap_or(0.0) / volume_quantum).round() as i64
            )
        };
        let mut signatures = std::collections::BTreeMap::<String, usize>::new();
        for part in parts {
            *signatures.entry(signature(part)).or_default() += 1;
        }
        let repeated_members = signatures.values().filter(|count| **count > 1).sum::<usize>();
        let repeated_excess = signatures.values().map(|count| count.saturating_sub(1)).sum::<usize>();
        indicators.repetition_ratio = estimate(repeated_excess as f64 / parts.len() as f64, GltfUnit::Unitless, parts.len(), Some(topology));
        indicators.modularity_ratio = estimate(repeated_members as f64 / parts.len() as f64, GltfUnit::Unitless, parts.len(), Some(topology));
    }
}

impl GltfInferenceStage<GltfGeometryContext<'_>> for GltfSymmetryInference {
    type Output = GltfSymmetryIndicators;

    fn infer(context: &GltfGeometryContext<'_>) -> Self::Output {
        let score = |axis: GltfVec3, rotation| symmetry_score(&context.points, context.centroid, axis.array(), context.diagonal, rotation, context.policy.sampling_budget as usize);
        Self::Output {
            reflection_symmetry_score: estimate(score(context.principal_frame.axes[0], false), GltfUnit::Unitless, context.sample_count, Some(context.topology)),
            rotational_symmetry_score: estimate(score(context.principal_frame.axes[0], true), GltfUnit::Unitless, context.sample_count, Some(context.topology)),
            reflection_symmetries: estimate(context.principal_frame.axes.iter().map(|axis| GltfDirectionScore { direction: *axis, score: score(*axis, false), order: None }).collect(), GltfUnit::Unitless, context.sample_count, Some(context.topology)),
            rotational_symmetries: estimate(
                context.principal_frame.axes.iter().map(|axis| GltfDirectionScore { direction: *axis, score: score(*axis, true), order: Some(2) }).collect(),
                GltfUnit::Unitless,
                context.sample_count,
                Some(context.topology),
            ),
            repetition_ratio: unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, Vec::new(), context.sample_count, Some(context.topology)),
            modularity_ratio: unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, Vec::new(), context.sample_count, Some(context.topology)),
        }
    }

    fn unavailable(diagnostic_ids: &[String]) -> Self::Output {
        let unavail = || unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None);
        Self::Output {
            reflection_symmetry_score: unavail(),
            rotational_symmetry_score: unavail(),
            reflection_symmetries: unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None),
            rotational_symmetries: unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None),
            repetition_ratio: unavail(),
            modularity_ratio: unavail(),
        }
    }
}
