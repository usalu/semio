/** 🦠️ remove-required-extension executable glTF command. */
import type { GltfJson, GltfSnapshot } from '../../📸️snapshot/🟦️component.ts';
import { clone, reject, run, same, type GltfLeafResult, type GltfMutationRejection } from '../../🔨️modules/🧬️mutation-support/📚️top-level/🟦️component.ts';
export const GltfUnrequireExtensionDescriptor = { id: 's.stdio.gltf.mutation.remove-required-extension.v1', version: 1, touchedPaths: ["document/extensionsRequired"], referencePolicy: 'removes only an existing requirement' } as const;
export interface GltfUnrequireExtensionPayload { extension: string }
export const validateGltfUnrequireExtension = (payload: GltfUnrequireExtensionPayload, base: GltfSnapshot): GltfMutationRejection | undefined => { if (!base.document.extensionsRequired.includes(payload.extension)) return reject('gltf.mutation.extension-absent', 'document/extensionsRequired', 'extension is not declared');  return undefined; };
export const applyGltfUnrequireExtension = (base: GltfSnapshot, payload: GltfUnrequireExtensionPayload): GltfLeafResult => run(base, payload, validateGltfUnrequireExtension, (next, payload) => { next.document.extensionsRequired = next.document.extensionsRequired.filter(value => value !== payload.extension); });
