/** 🧾 Shared GLTF inference values, measures, policy, diagnostics, and entity identity. */
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

