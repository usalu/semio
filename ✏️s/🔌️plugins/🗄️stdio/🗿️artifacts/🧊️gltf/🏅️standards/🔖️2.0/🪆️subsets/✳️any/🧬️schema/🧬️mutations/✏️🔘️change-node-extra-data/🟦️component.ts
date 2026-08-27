/** 🦠️ change-node-extra-data is an atomic, typed glTF 2.0 command. */
import type { GltfJson, GltfSnapshot } from '../../📸️snapshot/🟦️component.ts';
import { reject, run, same, type GltfLeafResult, type GltfMutationRejection } from '../../🔨️modules/🧬️mutation-support/📚️top-level/🟦️component.ts';
import { itemIndex } from '../../🔨️modules/🧬️mutation-support/🧱️structure-geometry/🟦️component.ts';
export const GltfChangeNodeExtraDataDescriptor = { id: 's.stdio.gltf.mutation.change-node-extra-data.v1', version: 1, kind: 'change', touchedPaths: ["document/nodes/*/extras"], referencePolicy: 'none' } as const;
export type GltfDataPresence = { state: 'absent' } | { state: 'present'; value: GltfJson };
export interface GltfChangeNodeExtraDataPayload { node: number; data: GltfDataPresence }
export type GltfChangeNodeExtraDataResult = GltfLeafResult;
export const validateGltfChangeNodeExtraData = (payload: GltfChangeNodeExtraDataPayload, base: GltfSnapshot): GltfMutationRejection | undefined => { const node = itemIndex(payload.node, base.document.nodes.length, 'document/nodes'); if (node) return node; const before = base.document.nodes[payload.node]!.extras; const unchanged = payload.data.state === 'absent' ? before === undefined : before !== undefined && same(before, payload.data.value); return unchanged ? reject('gltf.mutation.no-observable-change', `document/nodes/${payload.node}/extras`, 'extras already has the requested presence and value') : undefined; };
export const applyGltfChangeNodeExtraData = (base: GltfSnapshot, payload: GltfChangeNodeExtraDataPayload): GltfChangeNodeExtraDataResult => run(base, payload, validateGltfChangeNodeExtraData, (next, payload) => { next.document.nodes[payload.node]!.extras = payload.data.state === 'present' ? structuredClone(payload.data.value) : undefined; });
