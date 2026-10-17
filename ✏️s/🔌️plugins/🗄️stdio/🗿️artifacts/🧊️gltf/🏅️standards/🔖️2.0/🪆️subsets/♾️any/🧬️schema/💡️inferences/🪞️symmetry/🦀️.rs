//! 🪞 GLTF symmetry indicators.

pub use semio_s_artifact_stdio_gltf_inferences_footprint_area::standards::v2_0::subsets::any::schema::inferences::symmetry::modularity_ratio;
#[path = "🪞️reflection-symmetries/🦀️.rs"]
pub mod reflection_symmetries;
#[path = "⚖️reflection-symmetry-score/🦀️.rs"]
pub mod reflection_symmetry_score;
pub use semio_s_artifact_stdio_gltf_inferences_repetition_ratio::standards::v2_0::subsets::any::schema::inferences::symmetry::repetition_ratio;
#[path = "🔁️rotational-symmetries/🦀️.rs"]
pub mod rotational_symmetries;
#[path = "🔄️rotational-symmetry-score/🦀️.rs"]
pub mod rotational_symmetry_score;

use crate::standards::v2_0::subsets::any::schema::modules::measurement_contracts::*;
use crate::standards::v2_0::subsets::any::schema::inferences::geometry_core::GltfGeometryContext;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfSymmetryIndicators {
    pub reflection_symmetry_score: GltfMeasure<f64>,
    pub rotational_symmetry_score: GltfMeasure<f64>,
    pub reflection_symmetries: GltfMeasure<Vec<GltfDirectionScore>>,
    pub rotational_symmetries: GltfMeasure<Vec<GltfDirectionScore>>,
    pub repetition_ratio: GltfMeasure<f64>,
    pub modularity_ratio: GltfMeasure<f64>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

pub struct GltfSymmetryRaw {
    pub reflection_score: f64,
    pub rotation_score: f64,
    pub reflections: Vec<GltfDirectionScore>,
    pub rotations: Vec<GltfDirectionScore>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn raw(context: &GltfGeometryContext<'_>) -> GltfSymmetryRaw {
    let score = |axis: GltfVec3, rotation| symmetry_score(&context.points, context.centroid, axis.array(), context.diagonal, rotation, context.policy.sampling_budget as usize);
    let axes = &context.principal_frame.axes;
    GltfSymmetryRaw {
        reflection_score: score(axes[0], false),
        rotation_score: score(axes[0], true),
        reflections: axes.iter().map(|axis| GltfDirectionScore { direction: *axis, score: score(*axis, false), order: None }).collect(),
        rotations: axes.iter().map(|axis| GltfDirectionScore { direction: *axis, score: score(*axis, true), order: Some(2) }).collect(),
    }
}

impl GltfInferenceStage<GltfGeometryContext<'_>> for GltfSymmetryInference {
    type Output = GltfSymmetryIndicators;

    fn infer(context: &GltfGeometryContext<'_>) -> Self::Output {
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

    fn unavailable(diagnostic_ids: &[String]) -> Self::Output {
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
