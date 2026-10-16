/** 🔨️ Typed, non-public glTF geometry kernel shared by TypeScript inference leaves. */
import type {
  GltfAvailability,
  GltfComputationMethod,
  GltfMeasure,
  GltfProvenance,
  GltfQuality,
  GltfUnit,
} from '../../../🔨️modules/🧾️measurement-contracts/🟦️.ts';

export type GltfVector3 = readonly [number, number, number];
export type GltfTriangle = readonly [number, number, number];

export interface GltfTsGeometryContext {
  points: readonly GltfVector3[];
  triangles: readonly GltfTriangle[];
  sampleCount: number;
  valid: boolean;
  diagnostics: readonly string[];
  topology?: {
    watertight: boolean;
    manifold: boolean;
    consistentlyOriented: boolean;
    boundaryLoops?: number;
    eulerCharacteristic?: number;
    genus?: number;
  };
}

export interface GltfTsBounds3 {
  min: GltfVector3;
  max: GltfVector3;
  dimensions: GltfVector3;
}

export type GltfTsMeasure<T> = GltfMeasure<T>;

const subtract = (left: GltfVector3, right: GltfVector3): [number, number, number] => [
  left[0] - right[0],
  left[1] - right[1],
  left[2] - right[2],
];

const dot = (left: GltfVector3, right: GltfVector3): number =>
  left[0] * right[0] + left[1] * right[1] + left[2] * right[2];

const cross = (left: GltfVector3, right: GltfVector3): [number, number, number] => [
  left[1] * right[2] - left[2] * right[1],
  left[2] * right[0] - left[0] * right[2],
  left[0] * right[1] - left[1] * right[0],
];

const length = (value: GltfVector3): number => Math.sqrt(dot(value, value));

export const bounds = (points: readonly GltfVector3[]): GltfTsBounds3 | undefined => {
  if (points.length === 0) return undefined;
  const min: [number, number, number] = [...points[0]];
  const max: [number, number, number] = [...points[0]];
  for (const point of points) {
    for (let axis = 0; axis < 3; axis += 1) {
      min[axis] = Math.min(min[axis], point[axis]);
      max[axis] = Math.max(max[axis], point[axis]);
    }
  }
  return { min, max, dimensions: subtract(max, min) };
};

export const sortedExtents = (context: GltfTsGeometryContext): GltfVector3 | undefined => {
  const dimensions = bounds(context.points)?.dimensions;
  if (!dimensions) return undefined;
  return [...dimensions].sort((left, right) => right - left) as [number, number, number];
};

export const surfaceArea = (context: GltfTsGeometryContext): number =>
  context.triangles.reduce((sum, [a, b, c]) => {
    const first = context.points[a];
    const second = context.points[b];
    const third = context.points[c];
    return first && second && third
      ? sum + length(cross(subtract(second, first), subtract(third, first))) / 2
      : sum;
  }, 0);

export const signedVolume = (context: GltfTsGeometryContext): number =>
  context.triangles.reduce((sum, [a, b, c]) => {
    const first = context.points[a];
    const second = context.points[b];
    const third = context.points[c];
    return first && second && third ? sum + dot(first, cross(second, third)) / 6 : sum;
  }, 0);

const quality = (context: GltfTsGeometryContext, method: GltfComputationMethod, coverage: number): GltfQuality => ({
  method,
  coverage,
  sampleCount: context.sampleCount,
  watertight: context.topology?.watertight ?? false,
  manifold: context.topology?.manifold ?? true,
  consistentlyOriented: context.topology?.consistentlyOriented ?? true,
  warnings: [],
});

const provenance: GltfProvenance = {
  algorithm: 's.stdio.gltf.geometry',
  algorithmVersion: 2,
  dependencyFingerprints: [],
  coordinateSpace: 'sceneWorld',
  toleranceFingerprint: 'gltf-geometry-policy-v2-1e-9-1e-7-4096',
  samplingSeed: 's.stdio.gltf.geometry.v2',
  pose: 'static-node-and-mesh-morph-weights;skinning-unapplied',
};

export const exact = <T>(context: GltfTsGeometryContext, value: T, unit: GltfUnit): GltfTsMeasure<T> => ({
  value,
  unit,
  availability: 'available',
  validity: 'valid',
  diagnosticIds: [...context.diagnostics],
  quality: quality(context, 'exact', context.sampleCount === 0 ? 0 : 1),
  provenance,
});

export const unavailable = <T>(
  context: GltfTsGeometryContext,
  unit: GltfUnit,
  availability: GltfAvailability = 'unavailable',
): GltfTsMeasure<T> => ({
  unit,
  availability,
  validity: availability === 'invalidInput' ? 'invalid' : 'indeterminate',
  diagnosticIds: [...context.diagnostics],
  quality: quality(context, 'exact', 0),
  provenance,
});
