//! 🧩️ glTF inference DAG assembly over independently-owned leaf results.

use super::super::modules::{measurement_contracts::*, mesh_topology::Topology};
use super::{
    adjacency::GltfAdjacencyInference, area_volume::GltfAreaVolumeInference, clearance::GltfClearanceInference, compactness::GltfCompactnessInference, concavity::GltfConcavityInference, curvature::GltfCurvatureInference, geometry_core::*,
    mass_distribution::GltfMassInference, orientation::GltfOrientationInference, proportion::GltfProportionInference, roughness::GltfRoughnessInference, size::GltfSizeInference, symmetry::GltfSymmetryInference, thickness::GltfThicknessInference,
    topology::GltfTopologyInference, GltfEntityIndicators, GltfGeometricInference, GltfInferenceCounts, GltfPairInference, GltfPartInference,
};
use crate::artifacts::gltf::schema::snapshot::GltfSnapshot;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn empty_indicators(diagnostic_ids: Vec<String>) -> GltfEntityIndicators {
    GltfEntityIndicators {
        size: GltfSizeInference::unavailable(&diagnostic_ids),
        area_volume: GltfAreaVolumeInference::unavailable(&diagnostic_ids),
        compactness: GltfCompactnessInference::unavailable(&diagnostic_ids),
        proportion: GltfProportionInference::unavailable(&diagnostic_ids),
        mass: GltfMassInference::unavailable(&diagnostic_ids),
        curvature: GltfCurvatureInference::unavailable(&diagnostic_ids),
        thickness: GltfThicknessInference::unavailable(&diagnostic_ids),
        concavity: GltfConcavityInference::unavailable(&diagnostic_ids),
        clearance: GltfClearanceInference::unavailable(&diagnostic_ids),
        adjacency: GltfAdjacencyInference::unavailable(&diagnostic_ids),
        orientation: GltfOrientationInference::unavailable(&diagnostic_ids),
        symmetry: GltfSymmetryInference::unavailable(&diagnostic_ids),
        roughness: GltfRoughnessInference::unavailable(&diagnostic_ids),
        topology: GltfTopologyInference::unavailable(&diagnostic_ids),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn assemble_indicators(points: &[V3], triangles: &[[usize; 3]], policy: &GltfAnalysisPolicy) -> (GltfEntityIndicators, Topology) {
    let Some(context) = GltfGeometryContext::new(points, triangles, policy) else {
        return (empty_indicators(Vec::new()), topology_summary(points, triangles));
    };
    let topology = context.topology;
    (
        GltfEntityIndicators {
            size: GltfSizeInference::infer(&context),
            area_volume: GltfAreaVolumeInference::infer(&context),
            compactness: GltfCompactnessInference::infer(&context),
            proportion: GltfProportionInference::infer(&context),
            mass: GltfMassInference::infer(&context),
            curvature: GltfCurvatureInference::infer(&context),
            thickness: GltfThicknessInference::infer(&context),
            concavity: GltfConcavityInference::infer(&context),
            clearance: GltfClearanceInference::infer(&context),
            adjacency: GltfAdjacencyInference::infer(&context),
            orientation: GltfOrientationInference::infer(&context),
            symmetry: GltfSymmetryInference::infer(&context),
            roughness: GltfRoughnessInference::infer(&context),
            topology: GltfTopologyInference::infer(&context),
        },
        topology,
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn compute_gltf_inference(snapshot: &GltfSnapshot) -> GltfGeometricInference {
    let policy = policy();
    let mut diagnostics = Vec::new();
    let (raw_parts, node_instances) = collect_parts(snapshot, &mut diagnostics);
    let mut all_points = Vec::new();
    let mut all_triangles = Vec::new();
    for part in &raw_parts {
        let offset = all_points.len();
        all_points.extend_from_slice(&part.points);
        all_triangles.extend(part.triangles.iter().map(|face| [face[0] + offset, face[1] + offset, face[2] + offset]));
    }
    let (mut overall, overall_topology) = assemble_indicators(&all_points, &all_triangles, &policy);
    let mut parts = Vec::new();
    let mut component_count = 0;
    for raw in &raw_parts {
        let (indicators, topology) = assemble_indicators(&raw.points, &raw.triangles, &policy);
        component_count += topology.components;
        parts.push(GltfPartInference { address: raw.address.clone(), name: raw.name.clone(), indicators, diagnostic_ids: raw.diagnostic_ids.clone() });
    }
    GltfSymmetryInference::infer_assembly(&mut overall.symmetry, &parts, &policy, overall_topology);
    let mut pairs = Vec::new();
    for first in 0..raw_parts.len() {
        for second in first + 1..raw_parts.len() {
            if let Some(pair) = pair_geometry(&raw_parts[first], &raw_parts[second], &policy) {
                let (minimum_distance, clearance_distribution, interference_volume, overlap_volume) = GltfClearanceInference::infer_pair(&pair, &policy);
                pairs.push(GltfPairInference {
                    first: pair.first.clone(),
                    second: pair.second.clone(),
                    minimum_distance,
                    clearance_distribution,
                    contact_area: GltfAreaVolumeInference::infer_pair_contact(&pair),
                    interference_volume,
                    overlap_volume,
                    adjacent: GltfAdjacencyInference::infer_pair(&pair),
                    orientation_consistency: GltfOrientationInference::infer_pair(&pair),
                });
            }
        }
    }
    let distances = pairs.iter().filter_map(|pair| pair.minimum_distance.value).collect::<Vec<_>>();
    let contact_area = pairs.iter().filter_map(|pair| pair.contact_area.value).sum::<f64>();
    let contact_area_complete = pairs.iter().all(|pair| pair.contact_area.value.is_some());
    let overlap_volume = pairs.iter().filter_map(|pair| pair.overlap_volume.value).sum::<f64>();
    let overlap_complete = pairs.iter().all(|pair| pair.overlap_volume.value.is_some());
    let contacts = pairs.iter().filter(|pair| pair.adjacent.value == Some(true)).count() as u64;
    let sample_count = all_points.len();
    GltfAreaVolumeInference::infer_assembly(&mut overall.area_volume, raw_parts.len(), contact_area, contact_area_complete, sample_count, overall_topology);
    GltfAdjacencyInference::infer_assembly(&mut overall.adjacency, raw_parts.len(), contacts, sample_count, overall_topology);
    GltfOrientationInference::infer_assembly(&mut overall.orientation, raw_parts.len(), sample_count, overall_topology);
    GltfClearanceInference::infer_assembly(&mut overall.clearance, &distances, overlap_volume, overlap_complete, pairs.len(), &policy, sample_count, overall_topology);
    let valid_part_count = raw_parts.iter().filter(|part| !part.triangles.is_empty()).count() as u64;
    let invalid_part_count = raw_parts.iter().filter(|part| part.triangles.is_empty()).count() as u64 + diagnostics.iter().filter(|diagnostic| diagnostic.severity == GltfSeverity::Error).count() as u64;
    let counts = GltfInferenceCounts {
        scene_count: snapshot.document.scenes.len() as u64,
        node_instance_count: node_instances,
        mesh_count: snapshot.document.meshes.len() as u64,
        primitive_count: snapshot.document.meshes.iter().map(|mesh| mesh.primitives.len() as u64).sum(),
        vertex_count: all_points.len() as u64,
        triangle_count: all_triangles.len() as u64,
        component_count,
        surface_region_count: component_count,
        pair_count: pairs.len() as u64,
        valid_part_count,
        invalid_part_count,
    };
    let validity = if counts.invalid_part_count > 0 {
        GltfValidity::Invalid
    } else if counts.valid_part_count == 0 && !snapshot.document.meshes.is_empty() {
        GltfValidity::Indeterminate
    } else {
        GltfValidity::Valid
    };
    let mut quality = quality(GltfComputationMethod::DeterministicEstimate, all_points.len(), Some(overall_topology));
    quality.coverage = if counts.valid_part_count + counts.invalid_part_count == 0 { u8::from(snapshot.document.meshes.is_empty()) as f64 } else { counts.valid_part_count as f64 / (counts.valid_part_count + counts.invalid_part_count) as f64 };
    let mut inference_provenance = provenance(GltfCoordinateSpace::SceneWorld);
    inference_provenance.dependency_fingerprints.push(format!("canonical:{}", fingerprint(&all_points, &all_triangles)));
    inference_provenance.dependency_fingerprints.extend(snapshot.buffers.iter().enumerate().map(|(index, bytes)| format!("buffer:{index}:{}", byte_fingerprint(bytes))));
    GltfGeometricInference { schema: "s.stdio.gltf.inference".into(), schema_version: 2, policy, counts, overall, parts, pairs, diagnostics, validity, quality, provenance: inference_provenance }
}
