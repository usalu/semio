/** 📐 Authoritative glTF 2.0 universal geometric-inference value contract. */
export type GltfUnit = 'unitless' | 'metre' | 'squareMetre' | 'cubicMetre' | 'radian' | 'inverseMetre' | 'inverseSquareMetre';
export type GltfCoordinateSpace = 'meshLocal' | 'nodeLocal' | 'sceneWorld';
export type GltfAvailability = 'available' | 'approximate' | 'unavailable' | 'invalidInput' | 'unsupportedPrimitive' | 'openSurface' | 'nonManifold' | 'degenerate' | 'unresolvedResource';
export type GltfValidity = 'valid' | 'invalid' | 'indeterminate';
export type GltfSeverity = 'info' | 'warning' | 'error';
export type GltfComputationMethod = 'exact' | 'deterministicEstimate';
export interface GltfVec3 { x: number; y: number; z: number }
export interface GltfBounds3 { min: GltfVec3; max: GltfVec3; dimensions: GltfVec3 }
export interface GltfHistogram { edges: number[]; counts: number[]; weights: number[] }
export interface GltfStatistics { minimum?: number; maximum?: number; mean?: number; variance?: number; standardDeviation?: number; median?: number; quantiles: number[]; histogram?: GltfHistogram }
export interface GltfDirectionScore { direction: GltfVec3; score: number; order?: number }
export interface GltfPrincipalFrame { centroid: GltfVec3; axes: [GltfVec3, GltfVec3, GltfVec3]; eigenvalues: [number, number, number] }
export interface GltfQuality { method: GltfComputationMethod; coverage: number; absoluteError?: number; relativeError?: number; sampleCount: number; watertight: boolean; manifold: boolean; consistentlyOriented: boolean; warnings: string[] }
export interface GltfProvenance { algorithm: string; algorithmVersion: number; dependencyFingerprints: string[]; coordinateSpace: GltfCoordinateSpace; toleranceFingerprint: string; samplingSeed?: string; pose?: string }
export interface GltfMeasure<T> { value?: T; unit: GltfUnit; availability: GltfAvailability; validity: GltfValidity; diagnosticIds: string[]; quality: GltfQuality; provenance: GltfProvenance }
export type GltfScalarMeasure = GltfMeasure<number>;
export type GltfCountMeasure = GltfMeasure<number>;
export type GltfVectorMeasure = GltfMeasure<GltfVec3>;
export type GltfBoundsMeasure = GltfMeasure<GltfBounds3>;
export type GltfStatisticsMeasure = GltfMeasure<GltfStatistics>;
export type GltfPrincipalFrameMeasure = GltfMeasure<GltfPrincipalFrame>;
export type GltfDirectionsMeasure = GltfMeasure<GltfDirectionScore[]>;
export interface GltfAnalysisPolicy { schemaVersion: number; absoluteLengthTolerance: number; relativeTolerance: number; angularToleranceRadians: number; contactTolerance: number; sharpFeatureAngleRadians: number; histogramEdges: number[]; samplingBudget: number; samplingSeed: string; staticPose: boolean; unitDensity: boolean; fingerprint: string }
export interface GltfDiagnostic { id: string; severity: GltfSeverity; code: string; message: string; paths: string[] }
export interface GltfEntityAddress { scope: 'document' | 'scene' | 'nodeInstance' | 'mesh' | 'primitive' | 'component' | 'surfaceRegion'; scene?: number; nodePath: number[]; mesh?: number; primitive?: number; component?: number; surfaceRegion?: number; contentFingerprint: string }
export interface GltfSizeIndicators { overallSize: GltfScalarMeasure; axisAlignedBounds: GltfBoundsMeasure; orientedBounds: GltfBoundsMeasure; boundingBoxDimensions: GltfVectorMeasure; characteristicLength: GltfScalarMeasure; footprintArea: GltfScalarMeasure; projectedArea: GltfStatisticsMeasure }
export interface GltfAreaVolumeIndicators { surfaceArea: GltfScalarMeasure; totalArea: GltfScalarMeasure; exposedArea: GltfScalarMeasure; contactArea: GltfScalarMeasure; volume: GltfScalarMeasure; enclosedVolume: GltfScalarMeasure; materialVolume: GltfScalarMeasure; voidVolume: GltfScalarMeasure }
export interface GltfCompactnessIndicators { compactness: GltfScalarMeasure; surfaceToVolumeRatio: GltfScalarMeasure; sphericity: GltfScalarMeasure; compactnessIndex: GltfScalarMeasure; hullFillRatio: GltfScalarMeasure }
export interface GltfProportionIndicators { aspectRatios: GltfVectorMeasure; slenderness: GltfScalarMeasure; flatness: GltfScalarMeasure; elongation: GltfScalarMeasure }
export interface GltfMassIndicators { centroid: GltfVectorMeasure; principalFrame: GltfPrincipalFrameMeasure; principalAxes: GltfDirectionsMeasure; momentsOfInertia: GltfVectorMeasure; inertiaTensor: GltfMeasure<number[]> }
export interface GltfCurvatureIndicators { meanCurvature: GltfStatisticsMeasure; gaussianCurvature: GltfStatisticsMeasure; curvatureHistogram: GltfStatisticsMeasure; sharpFeatureProportion: GltfScalarMeasure }
export interface GltfThicknessIndicators { meanThickness: GltfScalarMeasure; minimumThickness: GltfScalarMeasure; thicknessVariability: GltfScalarMeasure; thicknessDistribution: GltfStatisticsMeasure }
export interface GltfConcavityIndicators { convexHullGap: GltfScalarMeasure; reentrantArea: GltfScalarMeasure; reentrantVolume: GltfScalarMeasure; concavityIndex: GltfScalarMeasure }
export interface GltfClearanceIndicators { minimumDistanceToNeighbors: GltfScalarMeasure; clearanceDistribution: GltfStatisticsMeasure; interferenceVolume: GltfScalarMeasure; overlapVolume: GltfScalarMeasure }
export interface GltfAdjacencyIndicators { numberOfContacts: GltfCountMeasure; contactGraphDegree: GltfCountMeasure; connectedComponents: GltfCountMeasure }
export interface GltfOrientationIndicators { mainAxisDirection: GltfVectorMeasure; faceNormalDistribution: GltfStatisticsMeasure; orientationConsistency: GltfScalarMeasure }
export interface GltfSymmetryIndicators { reflectionSymmetryScore: GltfScalarMeasure; rotationalSymmetryScore: GltfScalarMeasure; reflectionSymmetries: GltfDirectionsMeasure; rotationalSymmetries: GltfDirectionsMeasure; repetitionRatio: GltfScalarMeasure; modularityRatio: GltfScalarMeasure }
export interface GltfRoughnessIndicators { deviationFromIdeal: GltfStatisticsMeasure; deviationFromSmoothedGeometry: GltfStatisticsMeasure; normalVariation: GltfStatisticsMeasure; surfaceWaviness: GltfStatisticsMeasure; irregularity: GltfScalarMeasure }
export interface GltfTopologyIndicators { holes: GltfCountMeasure; handles: GltfCountMeasure; boundaryLoops: GltfCountMeasure; eulerCharacteristic: GltfCountMeasure; genus: GltfCountMeasure }
export interface GltfEntityIndicators { size: GltfSizeIndicators; areaVolume: GltfAreaVolumeIndicators; compactness: GltfCompactnessIndicators; proportion: GltfProportionIndicators; mass: GltfMassIndicators; curvature: GltfCurvatureIndicators; thickness: GltfThicknessIndicators; concavity: GltfConcavityIndicators; clearance: GltfClearanceIndicators; adjacency: GltfAdjacencyIndicators; orientation: GltfOrientationIndicators; symmetry: GltfSymmetryIndicators; roughness: GltfRoughnessIndicators; topology: GltfTopologyIndicators }
export interface GltfPartInference { address: GltfEntityAddress; name?: string; indicators: GltfEntityIndicators; diagnosticIds: string[] }
export interface GltfPairInference { first: GltfEntityAddress; second: GltfEntityAddress; minimumDistance: GltfScalarMeasure; clearanceDistribution: GltfStatisticsMeasure; contactArea: GltfScalarMeasure; interferenceVolume: GltfScalarMeasure; overlapVolume: GltfScalarMeasure; adjacent: GltfMeasure<boolean>; orientationConsistency: GltfScalarMeasure }
export interface GltfInferenceCounts { sceneCount: number; nodeInstanceCount: number; meshCount: number; primitiveCount: number; vertexCount: number; triangleCount: number; componentCount: number; surfaceRegionCount: number; pairCount: number; validPartCount: number; invalidPartCount: number }
/** 🧭 Complete authoritative result. Bounds live at `overall.size`, counts at `counts`. */
export interface GltfGeometricInference { schema: 's.stdio.gltf.inference'; schemaVersion: number; policy: GltfAnalysisPolicy; counts: GltfInferenceCounts; overall: GltfEntityIndicators; parts: GltfPartInference[]; pairs: GltfPairInference[]; diagnostics: GltfDiagnostic[]; validity: GltfValidity; quality: GltfQuality; provenance: GltfProvenance }
