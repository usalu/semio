/** 🧭 Open glTF mutation descriptor registry and generic envelopes. */
import type { GltfSnapshot } from '../../🧬️schema/📸️snapshot/🟦️component.ts';
import { gltfMutationLeafDescriptors, type GltfMutationLeafApplication, type GltfMutationLeafDescriptor, type GltfMutationLeafError, type GltfMutationLeafPlan, type GltfMutationLeafResult } from '../../🧬️schema/🧬️mutations/🟦️component.ts';

export const GLTF_MUTATION_MAX_COMMAND_ID_BYTES = 160;
export const GLTF_MUTATION_MAX_PAYLOAD_BYTES = 64 * 1024;
export const GLTF_MUTATION_MAX_TOUCHED_PATHS = 64;
export const GLTF_MUTATION_MAX_TOUCHED_PATH_BYTES = 512;
const U32_MAX = 0xffff_ffff;
const utf8 = new TextEncoder();
const isU32 = (value: number): boolean => Number.isInteger(value) && value > 0 && value <= U32_MAX;
const reject = (code: string, path: string, detail: string): GltfMutationRejection => ({ code, path, detail });
const accepted = <T>(value: T): GltfMutationResult<T> => ({ accepted: true, value });
const rejected = <T>(rejection: GltfMutationRejection): GltfMutationResult<T> => ({ accepted: false, rejection });

export type GltfMutationPhase = 'mutation' | 'diff' | 'inverse';
export interface GltfMutationRejection { readonly code: string; readonly path: string; readonly detail: string }
export type GltfMutationResult<T> = { readonly accepted: true; readonly value: T } | { readonly accepted: false; readonly rejection: GltfMutationRejection };
export interface GltfMutation { readonly commandId: string; readonly version: number; readonly phase: 'mutation' | 'inverse'; readonly payload: string }
export interface GltfDiffEnvelope { readonly commandId: string; readonly version: number; readonly phase: 'diff' | 'inverse'; readonly payload: string; readonly touchedPaths: readonly string[] }

const leaf = <T>(result: GltfMutationLeafResult<T>): GltfMutationResult<T> => result.accepted ? accepted(result.value) : rejected(result.rejection);

export class GltfMutationDescriptorRegistry {
  readonly #descriptors = new Map<string, GltfMutationLeafDescriptor>();

  constructor(descriptors: readonly GltfMutationLeafDescriptor[]) {
    for (const descriptor of descriptors) {
      if (!descriptor.commandId || utf8.encode(descriptor.commandId).length > GLTF_MUTATION_MAX_COMMAND_ID_BYTES || !isU32(descriptor.version)) throw new Error('invalid glTF mutation descriptor');
      if (this.#descriptors.has(descriptor.commandId)) throw new Error(`duplicate glTF mutation descriptor ${descriptor.commandId}`);
      this.#descriptors.set(descriptor.commandId, descriptor);
    }
  }

  resolve(commandId: string, version: number): GltfMutationResult<GltfMutationLeafDescriptor> {
    const descriptor = this.#descriptors.get(commandId);
    if (!descriptor) return rejected(reject('gltf.mutation.unknown-command', 'commandId', 'command is not registered'));
    return descriptor.version === version ? accepted(descriptor) : rejected(reject('gltf.mutation.stale-version', 'version', 'command version does not match its descriptor'));
  }

  commandIds(): readonly string[] { return [...this.#descriptors.keys()]; }
}

export const gltfMutationDescriptorRegistry = new GltfMutationDescriptorRegistry(gltfMutationLeafDescriptors);
export const registeredGltfMutationCommandIds = (): readonly string[] => gltfMutationDescriptorRegistry.commandIds();

const validateEnvelope = (envelope: { readonly commandId: string; readonly version: number; readonly payload: string }): GltfMutationRejection | undefined => {
  if (!envelope.commandId || utf8.encode(envelope.commandId).length > GLTF_MUTATION_MAX_COMMAND_ID_BYTES) return reject('gltf.mutation.budget-exceeded', 'commandId', 'command id exceeds its byte budget');
  if (!isU32(envelope.version)) return reject('gltf.mutation.invalid-version', 'version', 'version must be a nonzero u32');
  return typeof envelope.payload === 'string' && utf8.encode(envelope.payload).length <= GLTF_MUTATION_MAX_PAYLOAD_BYTES ? undefined : reject('gltf.mutation.budget-exceeded', 'payload', 'payload exceeds its byte budget');
};

export const validateGltfMutationEnvelope = (envelope: GltfMutation): GltfMutationRejection | undefined => {
  const invalid = validateEnvelope(envelope);
  if (invalid) return invalid;
  const descriptor = gltfMutationDescriptorRegistry.resolve(envelope.commandId, envelope.version);
  return descriptor.accepted ? undefined : descriptor.rejection;
};

export const planGltfMutation = (envelope: GltfMutation, base: GltfSnapshot): GltfMutationResult<GltfMutationLeafPlan> => {
  const invalid = validateEnvelope(envelope);
  if (invalid) return rejected(invalid);
  const descriptor = gltfMutationDescriptorRegistry.resolve(envelope.commandId, envelope.version);
  if (!descriptor.accepted) return descriptor;
  return leaf(envelope.phase === 'mutation' ? descriptor.value.plan(envelope.payload, base) : descriptor.value.planInverse(envelope.payload, base));
};

export const applyGltfMutationEnvelope = (envelope: GltfDiffEnvelope, base: GltfSnapshot): GltfMutationResult<GltfMutationLeafApplication> => {
  const invalid = validateEnvelope(envelope);
  if (invalid) return rejected(invalid);
  if (envelope.touchedPaths.length > GLTF_MUTATION_MAX_TOUCHED_PATHS || envelope.touchedPaths.some(path => utf8.encode(path).length > GLTF_MUTATION_MAX_TOUCHED_PATH_BYTES)) return rejected(reject('gltf.mutation.budget-exceeded', 'touchedPaths', 'touched paths exceed their byte budget'));
  const descriptor = gltfMutationDescriptorRegistry.resolve(envelope.commandId, envelope.version);
  if (!descriptor.accepted) return descriptor;
  const applied = leaf(envelope.phase === 'diff' ? descriptor.value.applyDiff(envelope.payload, base) : descriptor.value.applyInverse(envelope.payload, base));
  if (!applied.accepted) return applied;
  return JSON.stringify(applied.value.touchedPaths) === JSON.stringify(envelope.touchedPaths) ? applied : rejected(reject('gltf.mutation.invalid-touched-paths', 'touchedPaths', 'envelope paths do not match the descriptor payload'));
};
