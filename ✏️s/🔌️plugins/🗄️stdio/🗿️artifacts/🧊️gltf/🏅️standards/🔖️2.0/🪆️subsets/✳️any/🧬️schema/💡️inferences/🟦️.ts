/** 💡️ Public glTF inference assembly and atomic-leaf DAG. */
import type { GltfMeasure, GltfScalarMeasure, GltfStatisticsMeasure, GltfAnalysisPolicy, GltfDiagnostic, GltfEntityAddress, GltfValidity, GltfQuality, GltfProvenance } from '../../🔨️modules/🧾️measurement-contracts/🟦️.ts';
import type { GltfSizeIndicators } from './📦️size/🟦️.ts';
import type { GltfAreaVolumeIndicators } from './🧱️area-volume/🟦️.ts';
import type { GltfCompactnessIndicators } from './⚪️compactness/🟦️.ts';
import type { GltfProportionIndicators } from './📏️proportion/🟦️.ts';
import type { GltfMassIndicators } from './⚖️mass-distribution/🟦️.ts';
import type { GltfCurvatureIndicators } from './🌀️curvature/🟦️.ts';
import type { GltfThicknessIndicators } from './↕️thickness/🟦️.ts';
import type { GltfConcavityIndicators } from './🕳️concavity/🟦️.ts';
import type { GltfClearanceIndicators } from './↔️clearance/🟦️.ts';
import type { GltfAdjacencyIndicators } from './🔗️adjacency/🟦️.ts';
import type { GltfOrientationIndicators } from './🧭️orientation/🟦️.ts';
import type { GltfSymmetryIndicators } from './🪞️symmetry/🟦️.ts';
import type { GltfRoughnessIndicators } from './🌊️roughness/🟦️.ts';
import type { GltfTopologyIndicators } from './🕸️topology/🟦️.ts';

export * from './📦️size/overall-size/🟦️.ts';
export * from './📦️size/axis-aligned-bounds/🟦️.ts';
export * from './📦️size/oriented-bounds/🟦️.ts';
export * from './📦️size/bounding-box-dimensions/🟦️.ts';
export * from './📦️size/characteristic-length/🟦️.ts';
export * from './📦️size/footprint-area/🟦️.ts';
export * from './📦️size/projected-area/🟦️.ts';
export * from './🧱️area-volume/surface-area/🟦️.ts';
export * from './🧱️area-volume/total-area/🟦️.ts';
export * from './🧱️area-volume/exposed-area/🟦️.ts';
export * from './🧱️area-volume/contact-area/🟦️.ts';
export * from './🧱️area-volume/volume/🟦️.ts';
export * from './🧱️area-volume/enclosed-volume/🟦️.ts';
export * from './🧱️area-volume/material-volume/🟦️.ts';
export * from './🧱️area-volume/void-volume/🟦️.ts';
export * from './⚪️compactness/compactness/🟦️.ts';
export * from './⚪️compactness/surface-to-volume-ratio/🟦️.ts';
export * from './⚪️compactness/sphericity/🟦️.ts';
export * from './⚪️compactness/compactness-index/🟦️.ts';
export * from './⚪️compactness/hull-fill-ratio/🟦️.ts';
export * from './📏️proportion/aspect-ratios/🟦️.ts';
export * from './📏️proportion/slenderness/🟦️.ts';
export * from './📏️proportion/flatness/🟦️.ts';
export * from './📏️proportion/elongation/🟦️.ts';
export * from './⚖️mass-distribution/centroid/🟦️.ts';
export * from './⚖️mass-distribution/principal-frame/🟦️.ts';
export * from './⚖️mass-distribution/principal-axes/🟦️.ts';
export * from './⚖️mass-distribution/moments-of-inertia/🟦️.ts';
export * from './⚖️mass-distribution/inertia-tensor/🟦️.ts';
export * from './🌀️curvature/mean-curvature/🟦️.ts';
export * from './🌀️curvature/gaussian-curvature/🟦️.ts';
export * from './🌀️curvature/curvature-histogram/🟦️.ts';
export * from './🌀️curvature/sharp-feature-proportion/🟦️.ts';
export * from './↕️thickness/mean-thickness/🟦️.ts';
export * from './↕️thickness/minimum-thickness/🟦️.ts';
export * from './↕️thickness/thickness-variability/🟦️.ts';
export * from './↕️thickness/thickness-distribution/🟦️.ts';
export * from './🕳️concavity/convex-hull-gap/🟦️.ts';
export * from './🕳️concavity/reentrant-area/🟦️.ts';
export * from './🕳️concavity/reentrant-volume/🟦️.ts';
export * from './🕳️concavity/concavity-index/🟦️.ts';
export * from './↔️clearance/minimum-distance-to-neighbors/🟦️.ts';
export * from './↔️clearance/clearance-distribution/🟦️.ts';
export * from './↔️clearance/interference-volume/🟦️.ts';
export * from './↔️clearance/overlap-volume/🟦️.ts';
export * from './🔗️adjacency/number-of-contacts/🟦️.ts';
export * from './🔗️adjacency/contact-graph-degree/🟦️.ts';
export * from './🔗️adjacency/connected-components/🟦️.ts';
export * from './🧭️orientation/main-axis-direction/🟦️.ts';
export * from './🧭️orientation/face-normal-distribution/🟦️.ts';
export * from './🧭️orientation/orientation-consistency/🟦️.ts';
export * from './🪞️symmetry/reflection-symmetry-score/🟦️.ts';
export * from './🪞️symmetry/rotational-symmetry-score/🟦️.ts';
export * from './🪞️symmetry/reflection-symmetries/🟦️.ts';
export * from './🪞️symmetry/rotational-symmetries/🟦️.ts';
export * from './🪞️symmetry/repetition-ratio/🟦️.ts';
export * from './🪞️symmetry/modularity-ratio/🟦️.ts';
export * from './🌊️roughness/deviation-from-ideal/🟦️.ts';
export * from './🌊️roughness/deviation-from-smoothed-geometry/🟦️.ts';
export * from './🌊️roughness/normal-variation/🟦️.ts';
export * from './🌊️roughness/surface-waviness/🟦️.ts';
export * from './🌊️roughness/irregularity/🟦️.ts';
export * from './🕸️topology/holes/🟦️.ts';
export * from './🕸️topology/handles/🟦️.ts';
export * from './🕸️topology/boundary-loops/🟦️.ts';
export * from './🕸️topology/euler-characteristic/🟦️.ts';
export * from './🕸️topology/genus/🟦️.ts';

export const gltfInferenceLeafDescriptors = [
  { id: 's.stdio.gltf.inference.overall-size.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.overall-size.v1:geometry-v2', target: 'geometry.overall.size.overallSize' },
  { id: 's.stdio.gltf.inference.axis-aligned-bounds.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.axis-aligned-bounds.v1:geometry-v2', target: 'geometry.overall.size.axisAlignedBounds' },
  { id: 's.stdio.gltf.inference.oriented-bounds.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.oriented-bounds.v1:geometry-v2', target: 'geometry.overall.size.orientedBounds' },
  { id: 's.stdio.gltf.inference.bounding-box-dimensions.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.bounding-box-dimensions.v1:geometry-v2', target: 'geometry.overall.size.boundingBoxDimensions' },
  { id: 's.stdio.gltf.inference.characteristic-length.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.characteristic-length.v1:geometry-v2', target: 'geometry.overall.size.characteristicLength' },
  { id: 's.stdio.gltf.inference.footprint-area.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.footprint-area.v1:geometry-v2', target: 'geometry.overall.size.footprintArea' },
  { id: 's.stdio.gltf.inference.projected-area.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.projected-area.v1:geometry-v2', target: 'geometry.overall.size.projectedArea' },
  { id: 's.stdio.gltf.inference.surface-area.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.surface-area.v1:geometry-v2', target: 'geometry.overall.areaVolume.surfaceArea' },
  { id: 's.stdio.gltf.inference.total-area.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.total-area.v1:geometry-v2', target: 'geometry.overall.areaVolume.totalArea' },
  { id: 's.stdio.gltf.inference.exposed-area.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.exposed-area.v1:geometry-v2', target: 'geometry.overall.areaVolume.exposedArea' },
  { id: 's.stdio.gltf.inference.contact-area.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.contact-area.v1:geometry-v2', target: 'geometry.overall.areaVolume.contactArea' },
  { id: 's.stdio.gltf.inference.volume.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.volume.v1:geometry-v2', target: 'geometry.overall.areaVolume.volume' },
  { id: 's.stdio.gltf.inference.enclosed-volume.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.enclosed-volume.v1:geometry-v2', target: 'geometry.overall.areaVolume.enclosedVolume' },
  { id: 's.stdio.gltf.inference.material-volume.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.material-volume.v1:geometry-v2', target: 'geometry.overall.areaVolume.materialVolume' },
  { id: 's.stdio.gltf.inference.void-volume.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.void-volume.v1:geometry-v2', target: 'geometry.overall.areaVolume.voidVolume' },
  { id: 's.stdio.gltf.inference.compactness.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.compactness.v1:geometry-v2', target: 'geometry.overall.compactness.compactness' },
  { id: 's.stdio.gltf.inference.surface-to-volume-ratio.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.surface-to-volume-ratio.v1:geometry-v2', target: 'geometry.overall.compactness.surfaceToVolumeRatio' },
  { id: 's.stdio.gltf.inference.sphericity.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.sphericity.v1:geometry-v2', target: 'geometry.overall.compactness.sphericity' },
  { id: 's.stdio.gltf.inference.compactness-index.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.compactness-index.v1:geometry-v2', target: 'geometry.overall.compactness.compactnessIndex' },
  { id: 's.stdio.gltf.inference.hull-fill-ratio.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.hull-fill-ratio.v1:geometry-v2', target: 'geometry.overall.compactness.hullFillRatio' },
  { id: 's.stdio.gltf.inference.aspect-ratios.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.aspect-ratios.v1:geometry-v2', target: 'geometry.overall.proportion.aspectRatios' },
  { id: 's.stdio.gltf.inference.slenderness.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.slenderness.v1:geometry-v2', target: 'geometry.overall.proportion.slenderness' },
  { id: 's.stdio.gltf.inference.flatness.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.flatness.v1:geometry-v2', target: 'geometry.overall.proportion.flatness' },
  { id: 's.stdio.gltf.inference.elongation.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.elongation.v1:geometry-v2', target: 'geometry.overall.proportion.elongation' },
  { id: 's.stdio.gltf.inference.centroid.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.centroid.v1:geometry-v2', target: 'geometry.overall.mass.centroid' },
  { id: 's.stdio.gltf.inference.principal-frame.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.principal-frame.v1:geometry-v2', target: 'geometry.overall.mass.principalFrame' },
  { id: 's.stdio.gltf.inference.principal-axes.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.principal-axes.v1:geometry-v2', target: 'geometry.overall.mass.principalAxes' },
  { id: 's.stdio.gltf.inference.moments-of-inertia.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.moments-of-inertia.v1:geometry-v2', target: 'geometry.overall.mass.momentsOfInertia' },
  { id: 's.stdio.gltf.inference.inertia-tensor.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.inertia-tensor.v1:geometry-v2', target: 'geometry.overall.mass.inertiaTensor' },
  { id: 's.stdio.gltf.inference.mean-curvature.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.mean-curvature.v1:geometry-v2', target: 'geometry.overall.curvature.meanCurvature' },
  { id: 's.stdio.gltf.inference.gaussian-curvature.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.gaussian-curvature.v1:geometry-v2', target: 'geometry.overall.curvature.gaussianCurvature' },
  { id: 's.stdio.gltf.inference.curvature-histogram.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.curvature-histogram.v1:geometry-v2', target: 'geometry.overall.curvature.curvatureHistogram' },
  { id: 's.stdio.gltf.inference.sharp-feature-proportion.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.sharp-feature-proportion.v1:geometry-v2', target: 'geometry.overall.curvature.sharpFeatureProportion' },
  { id: 's.stdio.gltf.inference.mean-thickness.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.mean-thickness.v1:geometry-v2', target: 'geometry.overall.thickness.meanThickness' },
  { id: 's.stdio.gltf.inference.minimum-thickness.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.minimum-thickness.v1:geometry-v2', target: 'geometry.overall.thickness.minimumThickness' },
  { id: 's.stdio.gltf.inference.thickness-variability.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.thickness-variability.v1:geometry-v2', target: 'geometry.overall.thickness.thicknessVariability' },
  { id: 's.stdio.gltf.inference.thickness-distribution.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.thickness-distribution.v1:geometry-v2', target: 'geometry.overall.thickness.thicknessDistribution' },
  { id: 's.stdio.gltf.inference.convex-hull-gap.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.convex-hull-gap.v1:geometry-v2', target: 'geometry.overall.concavity.convexHullGap' },
  { id: 's.stdio.gltf.inference.reentrant-area.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.reentrant-area.v1:geometry-v2', target: 'geometry.overall.concavity.reentrantArea' },
  { id: 's.stdio.gltf.inference.reentrant-volume.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.reentrant-volume.v1:geometry-v2', target: 'geometry.overall.concavity.reentrantVolume' },
  { id: 's.stdio.gltf.inference.concavity-index.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.concavity-index.v1:geometry-v2', target: 'geometry.overall.concavity.concavityIndex' },
  { id: 's.stdio.gltf.inference.minimum-distance-to-neighbors.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.minimum-distance-to-neighbors.v1:geometry-v2', target: 'geometry.overall.clearance.minimumDistanceToNeighbors' },
  { id: 's.stdio.gltf.inference.clearance-distribution.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.clearance-distribution.v1:geometry-v2', target: 'geometry.overall.clearance.clearanceDistribution' },
  { id: 's.stdio.gltf.inference.interference-volume.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.interference-volume.v1:geometry-v2', target: 'geometry.overall.clearance.interferenceVolume' },
  { id: 's.stdio.gltf.inference.overlap-volume.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.overlap-volume.v1:geometry-v2', target: 'geometry.overall.clearance.overlapVolume' },
  { id: 's.stdio.gltf.inference.number-of-contacts.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.number-of-contacts.v1:geometry-v2', target: 'geometry.overall.adjacency.numberOfContacts' },
  { id: 's.stdio.gltf.inference.contact-graph-degree.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.contact-graph-degree.v1:geometry-v2', target: 'geometry.overall.adjacency.contactGraphDegree' },
  { id: 's.stdio.gltf.inference.connected-components.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.connected-components.v1:geometry-v2', target: 'geometry.overall.adjacency.connectedComponents' },
  { id: 's.stdio.gltf.inference.main-axis-direction.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.main-axis-direction.v1:geometry-v2', target: 'geometry.overall.orientation.mainAxisDirection' },
  { id: 's.stdio.gltf.inference.face-normal-distribution.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.face-normal-distribution.v1:geometry-v2', target: 'geometry.overall.orientation.faceNormalDistribution' },
  { id: 's.stdio.gltf.inference.orientation-consistency.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.orientation-consistency.v1:geometry-v2', target: 'geometry.overall.orientation.orientationConsistency' },
  { id: 's.stdio.gltf.inference.reflection-symmetry-score.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.reflection-symmetry-score.v1:geometry-v2', target: 'geometry.overall.symmetry.reflectionSymmetryScore' },
  { id: 's.stdio.gltf.inference.rotational-symmetry-score.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.rotational-symmetry-score.v1:geometry-v2', target: 'geometry.overall.symmetry.rotationalSymmetryScore' },
  { id: 's.stdio.gltf.inference.reflection-symmetries.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.reflection-symmetries.v1:geometry-v2', target: 'geometry.overall.symmetry.reflectionSymmetries' },
  { id: 's.stdio.gltf.inference.rotational-symmetries.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.rotational-symmetries.v1:geometry-v2', target: 'geometry.overall.symmetry.rotationalSymmetries' },
  { id: 's.stdio.gltf.inference.repetition-ratio.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.repetition-ratio.v1:geometry-v2', target: 'geometry.overall.symmetry.repetitionRatio' },
  { id: 's.stdio.gltf.inference.modularity-ratio.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.modularity-ratio.v1:geometry-v2', target: 'geometry.overall.symmetry.modularityRatio' },
  { id: 's.stdio.gltf.inference.deviation-from-ideal.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.deviation-from-ideal.v1:geometry-v2', target: 'geometry.overall.roughness.deviationFromIdeal' },
  { id: 's.stdio.gltf.inference.deviation-from-smoothed-geometry.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.deviation-from-smoothed-geometry.v1:geometry-v2', target: 'geometry.overall.roughness.deviationFromSmoothedGeometry' },
  { id: 's.stdio.gltf.inference.normal-variation.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.normal-variation.v1:geometry-v2', target: 'geometry.overall.roughness.normalVariation' },
  { id: 's.stdio.gltf.inference.surface-waviness.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.surface-waviness.v1:geometry-v2', target: 'geometry.overall.roughness.surfaceWaviness' },
  { id: 's.stdio.gltf.inference.irregularity.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.irregularity.v1:geometry-v2', target: 'geometry.overall.roughness.irregularity' },
  { id: 's.stdio.gltf.inference.holes.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.holes.v1:geometry-v2', target: 'geometry.overall.topology.holes' },
  { id: 's.stdio.gltf.inference.handles.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.handles.v1:geometry-v2', target: 'geometry.overall.topology.handles' },
  { id: 's.stdio.gltf.inference.boundary-loops.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.boundary-loops.v1:geometry-v2', target: 'geometry.overall.topology.boundaryLoops' },
  { id: 's.stdio.gltf.inference.euler-characteristic.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.euler-characteristic.v1:geometry-v2', target: 'geometry.overall.topology.eulerCharacteristic' },
  { id: 's.stdio.gltf.inference.genus.v1', algorithmVersion: 1, cacheKey: 's.stdio.gltf.inference.genus.v1:geometry-v2', target: 'geometry.overall.topology.genus' },
] as const;

export interface GltfEntityIndicators { size: GltfSizeIndicators; areaVolume: GltfAreaVolumeIndicators; compactness: GltfCompactnessIndicators; proportion: GltfProportionIndicators; mass: GltfMassIndicators; curvature: GltfCurvatureIndicators; thickness: GltfThicknessIndicators; concavity: GltfConcavityIndicators; clearance: GltfClearanceIndicators; adjacency: GltfAdjacencyIndicators; orientation: GltfOrientationIndicators; symmetry: GltfSymmetryIndicators; roughness: GltfRoughnessIndicators; topology: GltfTopologyIndicators }
export interface GltfPartInference { address: GltfEntityAddress; name?: string; indicators: GltfEntityIndicators; diagnosticIds: string[] }
export interface GltfPairInference { first: GltfEntityAddress; second: GltfEntityAddress; minimumDistance: GltfScalarMeasure; clearanceDistribution: GltfStatisticsMeasure; contactArea: GltfScalarMeasure; interferenceVolume: GltfScalarMeasure; overlapVolume: GltfScalarMeasure; adjacent: GltfMeasure<boolean>; orientationConsistency: GltfScalarMeasure }
export interface GltfInferenceCounts { sceneCount: number; nodeInstanceCount: number; meshCount: number; primitiveCount: number; vertexCount: number; triangleCount: number; componentCount: number; surfaceRegionCount: number; pairCount: number; validPartCount: number; invalidPartCount: number }
export interface GltfGeometricInference { schema: 's.stdio.gltf.inference'; schemaVersion: number; policy: GltfAnalysisPolicy; counts: GltfInferenceCounts; overall: GltfEntityIndicators; parts: GltfPartInference[]; pairs: GltfPairInference[]; diagnostics: GltfDiagnostic[]; validity: GltfValidity; quality: GltfQuality; provenance: GltfProvenance }
export interface GltfInference { geometry: GltfGeometricInference }
