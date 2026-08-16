/** 🦠️ unbind-node-child is an atomic, typed glTF 2.0 command. */
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { reject, run, type GltfLeafResult, type GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
import { itemIndex } from '../../🔒️structure-geometry-private/🟦️component.ts';
export const GltfUnbindNodeChildDescriptor = { id: 's.stdio.gltf.mutation.unbind-node-child.v1', version: 1, kind: 'unbind', touchedPaths: ["document/nodes/*/children"], referencePolicy: 'removes only the explicit parent-child relationship' } as const;
export interface GltfUnbindNodeChildPayload { parent: number; child: number }
export type GltfUnbindNodeChildResult = GltfLeafResult;
export const validateGltfUnbindNodeChild = (payload: GltfUnbindNodeChildPayload, base: GltfSnapshot): GltfMutationRejection | undefined => { const parent = itemIndex(payload.parent, base.document.nodes.length, 'document/nodes'); if (parent) return parent; const child = itemIndex(payload.child, base.document.nodes.length, 'document/nodes'); if (child) return child; if (!base.document.nodes[payload.parent]!.children.includes(payload.child)) return reject('gltf.mutation.relation-absent', `document/nodes/${payload.parent}/children`, 'child is not linked to parent'); return undefined; };
export const applyGltfUnbindNodeChild = (base: GltfSnapshot, payload: GltfUnbindNodeChildPayload): GltfUnbindNodeChildResult => run(base, payload, validateGltfUnbindNodeChild, (next, payload) => { const position = next.document.nodes[payload.parent]!.children.indexOf(payload.child); next.document.nodes[payload.parent]!.children.splice(position, 1); });
