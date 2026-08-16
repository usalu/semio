//! 💡️ Shared inference-measure constructors.

use super::measurement_contracts::*;
use super::mesh_topology::Topology;

//#region 💡️Measures
fn provenance() -> GltfProvenance {
    GltfProvenance {
        algorithm: "s.stdio.gltf.geometry".into(),
        algorithm_version: 2,
        dependency_fingerprints: Vec::new(),
        coordinate_space: GltfCoordinateSpace::SceneWorld,
        tolerance_fingerprint: "gltf-geometry-policy-v2-1e-9-1e-7-4096".into(),
        sampling_seed: Some("s.stdio.gltf.geometry.v2".into()),
        pose: Some("static-node-and-mesh-morph-weights;skinning-unapplied".into()),
    }
}

fn quality(method: GltfComputationMethod, sample_count: usize, topology: Option<Topology>) -> GltfQuality {
    let topology = topology.unwrap_or(Topology { components: 0, boundary_loops: 0, chi: 0, genus: None, manifold: true, watertight: false, oriented: true });
    GltfQuality {
        method,
        coverage: if sample_count == 0 { 0.0 } else { 1.0 },
        absolute_error: None,
        relative_error: None,
        sample_count: sample_count as u64,
        watertight: topology.watertight,
        manifold: topology.manifold,
        consistently_oriented: topology.oriented,
        warnings: Vec::new(),
    }
}

fn measure<T>(value: T, unit: GltfUnit, method: GltfComputationMethod, sample_count: usize, topology: Option<Topology>) -> GltfMeasure<T> {
    GltfMeasure {
        value: Some(value),
        unit,
        availability: if method == GltfComputationMethod::Exact { GltfAvailability::Available } else { GltfAvailability::Approximate },
        validity: GltfValidity::Valid,
        diagnostic_ids: Vec::new(),
        quality: quality(method, sample_count, topology),
        provenance: provenance(),
    }
}

pub(crate) fn unavailable<T>(unit: GltfUnit, availability: GltfAvailability, diagnostic_ids: Vec<String>, sample_count: usize, topology: Option<Topology>) -> GltfMeasure<T> {
    let validity = if availability == GltfAvailability::InvalidInput { GltfValidity::Invalid } else { GltfValidity::Indeterminate };
    GltfMeasure { value: None, unit, availability, validity, diagnostic_ids, quality: quality(GltfComputationMethod::Exact, sample_count, topology), provenance: provenance() }
}

pub(crate) fn exact<T>(value: T, unit: GltfUnit, sample_count: usize, topology: Option<Topology>) -> GltfMeasure<T> {
    measure(value, unit, GltfComputationMethod::Exact, sample_count, topology)
}
pub(crate) fn estimate<T>(value: T, unit: GltfUnit, sample_count: usize, topology: Option<Topology>) -> GltfMeasure<T> {
    measure(value, unit, GltfComputationMethod::DeterministicEstimate, sample_count, topology)
}
//#endregion 💡️Measures
