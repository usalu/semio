//! 🪞 GLTF symmetry indicators.

#[path = "modularity-ratio/🦀️component.rs"]
pub mod modularity_ratio;
#[path = "reflection-symmetries/🦀️component.rs"]
pub mod reflection_symmetries;
#[path = "reflection-symmetry-score/🦀️component.rs"]
pub mod reflection_symmetry_score;
#[path = "repetition-ratio/🦀️component.rs"]
pub mod repetition_ratio;
#[path = "rotational-symmetries/🦀️component.rs"]
pub mod rotational_symmetries;
#[path = "rotational-symmetry-score/🦀️component.rs"]
pub mod rotational_symmetry_score;

use super::super::modules::measurement_contracts::*;
use super::super::modules::mesh_topology::Topology;
use super::{geometry_core::GltfGeometryContext, GltfPartInference};
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

async fn symmetry_score(points: &[[f64; 3]], centroid: [f64; 3], axis: [f64; 3], scale: f64, rotation: bool, budget: usize) -> f64 {
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

pub(crate) struct GltfSymmetryRaw {
    pub(crate) reflection_score: f64,
    pub(crate) rotation_score: f64,
    pub(crate) reflections: Vec<GltfDirectionScore>,
    pub(crate) rotations: Vec<GltfDirectionScore>,
}

pub(crate) async fn raw(context: &GltfGeometryContext<'_>) -> GltfSymmetryRaw {
    let score = |axis: GltfVec3, rotation| symmetry_score(&context.points, context.centroid, axis.array(), context.diagonal, rotation, context.policy.sampling_budget as usize);
    let axes = &context.principal_frame.axes;
    GltfSymmetryRaw {
        reflection_score: score(axes[0], false),
        rotation_score: score(axes[0], true),
        reflections: axes.iter().map(|axis| GltfDirectionScore { direction: *axis, score: score(*axis, false), order: None }).collect(),
        rotations: axes.iter().map(|axis| GltfDirectionScore { direction: *axis, score: score(*axis, true), order: Some(2) }).collect(),
    }
}

pub(crate) async fn assembly_ratios(parts: &[GltfPartInference], policy: &GltfAnalysisPolicy) -> Option<(f64, f64)> {
    if parts.is_empty() {
        return None;
    }
    let signature = |part: &GltfPartInference| {
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
    Some((repeated_excess as f64 / parts.len() as f64, repeated_members as f64 / parts.len() as f64))
}

impl GltfSymmetryInference {
    pub(crate) async fn infer_assembly(indicators: &mut GltfSymmetryIndicators, parts: &[GltfPartInference], policy: &GltfAnalysisPolicy, topology: Topology) {
        if let Some(measure) = repetition_ratio::from_assembly(parts, policy, topology) {
            indicators.repetition_ratio = measure;
        }
        if let Some(measure) = modularity_ratio::from_assembly(parts, policy, topology) {
            indicators.modularity_ratio = measure;
        }
    }
}

impl GltfInferenceStage<GltfGeometryContext<'_>> for GltfSymmetryInference {
    type Output = GltfSymmetryIndicators;

    async fn infer(context: &GltfGeometryContext<'_>) -> Self::Output {
        let raw = raw(context);
        Self::Output {
            reflection_symmetry_score: reflection_symmetry_score::from_raw(context, &raw),
            rotational_symmetry_score: rotational_symmetry_score::from_raw(context, &raw),
            reflection_symmetries: reflection_symmetries::from_raw(context, &raw),
            rotational_symmetries: rotational_symmetries::from_raw(context, &raw),
            repetition_ratio: repetition_ratio::infer(context),
            modularity_ratio: modularity_ratio::infer(context),
        }
    }

    async fn unavailable(diagnostic_ids: &[String]) -> Self::Output {
        Self::Output {
            reflection_symmetry_score: reflection_symmetry_score::unavailable_measure(diagnostic_ids),
            rotational_symmetry_score: rotational_symmetry_score::unavailable_measure(diagnostic_ids),
            reflection_symmetries: reflection_symmetries::unavailable_measure(diagnostic_ids),
            rotational_symmetries: rotational_symmetries::unavailable_measure(diagnostic_ids),
            repetition_ratio: repetition_ratio::unavailable_measure(diagnostic_ids),
            modularity_ratio: modularity_ratio::unavailable_measure(diagnostic_ids),
        }
    }
}
