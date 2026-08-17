/** 🧩 Common create-scene descriptor for the open glTF mutation registry. */
import type { GltfSnapshot } from '../../📸️snapshot/🟦️component.ts';
import type { GltfMutationLeafApplication, GltfMutationLeafDescriptor, GltfMutationLeafError, GltfMutationLeafPlan, GltfMutationLeafResult } from '../🟦️component.ts';
import { applyGltfCreateScene, decodeGltfCreateScenePayload } from './🦠️mutation/🟦️component.ts';
import { applyGltfCreateSceneDiff, deriveGltfCreateSceneDiff, type GltfCreateSceneDiff } from './🔺️diff/🟦️component.ts';
import { applyGltfCreateSceneInverse, deriveGltfCreateSceneInverse, type GltfCreateSceneInverse } from './↩️inverse/🟦️component.ts';

const accepted = <T>(value: T): GltfMutationLeafResult<T> => ({ accepted: true, value });
const rejected = <T>(rejection: GltfMutationLeafError): GltfMutationLeafResult<T> => ({ accepted: false, rejection });
const error = (code: string, path: string, detail: string): GltfMutationLeafError => ({ code, path, detail });
const fromRejection = (rejection: GltfMutationLeafError): GltfMutationLeafResult<never> => rejected(rejection);
const encode = (value: unknown, path: string): GltfMutationLeafResult<string> => {
  try { return accepted(JSON.stringify(value)); } catch (cause) { return rejected(error('gltf.mutation.encode-failed', path, String(cause))); }
};
const decode = <T>(payload: string, path: string): GltfMutationLeafResult<T> => {
  try {
    const value: unknown = JSON.parse(payload);
    return value && typeof value === 'object' && !Array.isArray(value) ? accepted(value as T) : rejected(error('gltf.mutation.malformed-payload', path, 'payload must be a JSON object'));
  } catch (cause) { return rejected(error('gltf.mutation.malformed-payload', path, String(cause))); }
};

const plan = (payload: string, base: GltfSnapshot): GltfMutationLeafResult<GltfMutationLeafPlan> => {
  const decoded = decodeGltfCreateScenePayload(payload);
  if (!decoded.accepted) return fromRejection(decoded.rejection);
  const mutation = applyGltfCreateScene(base, decoded.payload);
  if (!mutation.accepted) return fromRejection(mutation.rejection);
  const diff = deriveGltfCreateSceneDiff(base, decoded.payload);
  if (!diff.accepted) return fromRejection(diff.rejection);
  const inverse = deriveGltfCreateSceneInverse(base, decoded.payload);
  if (!inverse.accepted) return fromRejection(inverse.rejection);
  const diffPayload = encode(diff.diff, 'diff');
  if (!diffPayload.accepted) return diffPayload;
  const inversePayload = encode(inverse.inverse, 'inverse');
  return inversePayload.accepted ? accepted({ diffPayload: diffPayload.value, inversePayload: inversePayload.value, touchedPaths: diff.touchedPaths }) : inversePayload;
};

const planInverse = (payload: string, base: GltfSnapshot): GltfMutationLeafResult<GltfMutationLeafPlan> => {
  const inverse = decode<GltfCreateSceneInverse>(payload, 'inverse/payload');
  if (!inverse.accepted) return inverse;
  const applied = applyGltfCreateSceneInverse(base, inverse.value);
  return applied.accepted ? accepted({ diffPayload: payload, inversePayload: '', touchedPaths: inverse.value.touchedPaths }) : fromRejection(applied.rejection);
};

const applyDiff = (payload: string, base: GltfSnapshot): GltfMutationLeafResult<GltfMutationLeafApplication> => {
  const diff = decode<GltfCreateSceneDiff>(payload, 'diff/payload');
  if (!diff.accepted) return diff;
  const applied = applyGltfCreateSceneDiff(base, diff.value);
  return applied.accepted ? accepted({ snapshot: applied.snapshot, touchedPaths: diff.value.touchedPaths }) : fromRejection(applied.rejection);
};

const applyInverse = (payload: string, base: GltfSnapshot): GltfMutationLeafResult<GltfMutationLeafApplication> => {
  const inverse = decode<GltfCreateSceneInverse>(payload, 'inverse/payload');
  if (!inverse.accepted) return inverse;
  const applied = applyGltfCreateSceneInverse(base, inverse.value);
  return applied.accepted ? accepted({ snapshot: applied.snapshot, touchedPaths: inverse.value.touchedPaths }) : fromRejection(applied.rejection);
};

export const GltfCreateSceneLeafDescriptor: GltfMutationLeafDescriptor = {
  commandId: 's.stdio.gltf.mutation.create-scene.v1', version: 1, plan, planInverse, applyDiff, applyInverse,
};
