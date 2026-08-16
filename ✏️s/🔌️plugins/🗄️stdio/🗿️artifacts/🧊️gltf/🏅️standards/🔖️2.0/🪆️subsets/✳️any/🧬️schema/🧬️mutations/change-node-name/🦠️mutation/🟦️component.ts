/** 🦠️ change-node-name is an atomic, typed glTF 2.0 command. */
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { reject, run, type GltfLeafResult, type GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
import { itemIndex } from '../../🔒️structure-geometry-private/🟦️component.ts';
export const GltfChangeNodeNameDescriptor = { id: 's.stdio.gltf.mutation.change-node-name.v1', version: 1, kind: 'change', touchedPaths: ["document/nodes/*/name"], referencePolicy: 'none' } as const;
export interface GltfChangeNodeNamePayload { node: number; value: string | null }
export type GltfChangeNodeNameResult = GltfLeafResult;
export const validateGltfChangeNodeName = (payload: GltfChangeNodeNamePayload, base: GltfSnapshot): GltfMutationRejection | undefined => { const node = itemIndex(payload.node, base.document.nodes.length, 'document/nodes'); if (node) return node; const before = base.document.nodes[payload.node]!.name ?? null; return before === payload.value ? reject('gltf.mutation.no-observable-change', `document/nodes/${payload.node}/name`, 'name already has the requested presence and value') : undefined; };
export const applyGltfChangeNodeName = (base: GltfSnapshot, payload: GltfChangeNodeNamePayload): GltfChangeNodeNameResult => run(base, payload, validateGltfChangeNodeName, (next, payload) => { next.document.nodes[payload.node]!.name = payload.value ?? undefined; });
