/** 📐 Aggregate GLTF geometric inference contract. */
import type { GltfMeasure, GltfScalarMeasure, GltfStatisticsMeasure, GltfAnalysisPolicy, GltfDiagnostic, GltfEntityAddress, GltfValidity, GltfQuality, GltfProvenance } from '../../../🔨️modules/🧾️measurement-contracts/🟦️component.ts';
import type { GltfSizeIndicators } from '../📦️size/🟦️component.ts';
import type { GltfAreaVolumeIndicators } from '../🧱️area-volume/🟦️component.ts';
import type { GltfCompactnessIndicators } from '../⚪️compactness/🟦️component.ts';
import type { GltfProportionIndicators } from '../📏️proportion/🟦️component.ts';
import type { GltfMassIndicators } from '../⚖️mass-distribution/🟦️component.ts';
import type { GltfCurvatureIndicators } from '../🌀️curvature/🟦️component.ts';
import type { GltfThicknessIndicators } from '../↕️thickness/🟦️component.ts';
import type { GltfConcavityIndicators } from '../🕳️concavity/🟦️component.ts';
import type { GltfClearanceIndicators } from '../↔️clearance/🟦️component.ts';
import type { GltfAdjacencyIndicators } from '../🔗️adjacency/🟦️component.ts';
import type { GltfOrientationIndicators } from '../🧭️orientation/🟦️component.ts';
import type { GltfSymmetryIndicators } from '../🪞️symmetry/🟦️component.ts';
import type { GltfRoughnessIndicators } from '../🌊️roughness/🟦️component.ts';
import type { GltfTopologyIndicators } from '../🕸️topology/🟦️component.ts';

export interface GltfEntityIndicators { size: GltfSizeIndicators; areaVolume: GltfAreaVolumeIndicators; compactness: GltfCompactnessIndicators; proportion: GltfProportionIndicators; mass: GltfMassIndicators; curvature: GltfCurvatureIndicators; thickness: GltfThicknessIndicators; concavity: GltfConcavityIndicators; clearance: GltfClearanceIndicators; adjacency: GltfAdjacencyIndicators; orientation: GltfOrientationIndicators; symmetry: GltfSymmetryIndicators; roughness: GltfRoughnessIndicators; topology: GltfTopologyIndicators }
export interface GltfPartInference { address: GltfEntityAddress; name?: string; indicators: GltfEntityIndicators; diagnosticIds: string[] }
export interface GltfPairInference { first: GltfEntityAddress; second: GltfEntityAddress; minimumDistance: GltfScalarMeasure; clearanceDistribution: GltfStatisticsMeasure; contactArea: GltfScalarMeasure; interferenceVolume: GltfScalarMeasure; overlapVolume: GltfScalarMeasure; adjacent: GltfMeasure<boolean>; orientationConsistency: GltfScalarMeasure }
export interface GltfInferenceCounts { sceneCount: number; nodeInstanceCount: number; meshCount: number; primitiveCount: number; vertexCount: number; triangleCount: number; componentCount: number; surfaceRegionCount: number; pairCount: number; validPartCount: number; invalidPartCount: number }
/** 🧮 Complete authoritative result. Bounds live at `overall.size`, counts at `counts`. */
export interface GltfGeometricInference { schema: 's.stdio.gltf.inference'; schemaVersion: number; policy: GltfAnalysisPolicy; counts: GltfInferenceCounts; overall: GltfEntityIndicators; parts: GltfPartInference[]; pairs: GltfPairInference[]; diagnostics: GltfDiagnostic[]; validity: GltfValidity; quality: GltfQuality; provenance: GltfProvenance }
