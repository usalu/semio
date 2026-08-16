/** 🧬 Complete semantic glTF mutation vocabulary; binary ordinals are frozen at 0–27. */
import type { NoMutation } from '../../🧬️schema/🧬️mutations/🚫️no-mutation/🟦️component.ts';
import type { SetSnapshot } from '../../🧬️schema/🧬️mutations/📄set-snapshot/🟦️component.ts';
import type { SetAsset } from '../../🧬️schema/🧬️mutations/🏷️set-asset/🟦️component.ts';
import type { InsertScene } from '../../🧬️schema/🧬️mutations/➕️insert-scene/🟦️component.ts';
import type { RemoveScene } from '../../🧬️schema/🧬️mutations/➖️remove-scene/🟦️component.ts';
import type { SetScene } from '../../🧬️schema/🧬️mutations/✏️set-scene/🟦️component.ts';
import type { InsertNode } from '../../🧬️schema/🧬️mutations/➕️insert-node/🟦️component.ts';
import type { RemoveNode } from '../../🧬️schema/🧬️mutations/➖️remove-node/🟦️component.ts';
import type { SetNode } from '../../🧬️schema/🧬️mutations/✏️set-node/🟦️component.ts';
import type { InsertMesh } from '../../🧬️schema/🧬️mutations/➕️insert-mesh/🟦️component.ts';
import type { RemoveMesh } from '../../🧬️schema/🧬️mutations/➖️remove-mesh/🟦️component.ts';
import type { SetMesh } from '../../🧬️schema/🧬️mutations/✏️set-mesh/🟦️component.ts';
import type { InsertAccessor } from '../../🧬️schema/🧬️mutations/➕️insert-accessor/🟦️component.ts';
import type { RemoveAccessor } from '../../🧬️schema/🧬️mutations/➖️remove-accessor/🟦️component.ts';
import type { SetAccessor } from '../../🧬️schema/🧬️mutations/✏️set-accessor/🟦️component.ts';
import type { InsertMaterial } from '../../🧬️schema/🧬️mutations/➕️insert-material/🟦️component.ts';
import type { RemoveMaterial } from '../../🧬️schema/🧬️mutations/➖️remove-material/🟦️component.ts';
import type { SetMaterial } from '../../🧬️schema/🧬️mutations/✏️set-material/🟦️component.ts';
import type { InsertBuffer } from '../../🧬️schema/🧬️mutations/➕️insert-buffer/🟦️component.ts';
import type { RemoveBuffer } from '../../🧬️schema/🧬️mutations/➖️remove-buffer/🟦️component.ts';
import type { SetBuffer } from '../../🧬️schema/🧬️mutations/✏️set-buffer/🟦️component.ts';
import type { InsertAnimation } from '../../🧬️schema/🧬️mutations/➕️insert-animation/🟦️component.ts';
import type { RemoveAnimation } from '../../🧬️schema/🧬️mutations/➖️remove-animation/🟦️component.ts';
import type { SetAnimation } from '../../🧬️schema/🧬️mutations/✏️set-animation/🟦️component.ts';
import type { TransformNode } from '../../🧬️schema/🧬️mutations/🔄️transform-node/🟦️component.ts';
import type { ReparentNode } from '../../🧬️schema/🧬️mutations/🌳️reparent-node/🟦️component.ts';
import type { BindNodeMesh } from '../../🧬️schema/🧬️mutations/🔗️bind-node-mesh/🟦️component.ts';
import type { BindPrimitiveMaterial } from '../../🧬️schema/🧬️mutations/🔗️bind-primitive-material/🟦️component.ts';
import type { GltfDiff } from '../../🧬️schema/🔺️diff/🟦️component.ts';

export interface GltfMutationRejection { code: string; path: string; detail: string }
/** 🛂 Application is total and typed: rejection never degrades to an empty diff. */
export type GltfMutationApplication = { accepted: true; diff: GltfDiff } | { accepted: false; rejection: GltfMutationRejection };
export type GltfTouchedRegion = 'asset' | 'scene' | 'scenes' | 'nodes' | 'meshes' | 'accessors' | 'bufferViews' | 'buffers' | 'bufferBytes' | 'materials' | 'textures' | 'images' | 'samplers' | 'skins' | 'animations' | 'cameras' | 'extensionsUsed' | 'extensionsRequired' | 'extensions' | 'extras' | 'sourceForm';
/** ↩️ `inverse` restores the exact base and `diff` is the accepted sparse state transition. */
export interface GltfMutationDerivation { mutation: GltfMutation; diff: GltfDiff; inverse: GltfMutation; touchedPaths: string[]; touchedRegions: GltfTouchedRegion[]; referenceRules: GltfReferenceRule[] }
export interface GltfReferenceRule { family: 'scene' | 'node' | 'mesh' | 'accessor' | 'material' | 'buffer'; pathPattern: string; validate: boolean; remapOnInsert: boolean; remapOnRemove: boolean }
/** 🔗 Includes primitive attributes, indices, and every morph-target semantic accessor reference. */
export const GLTF_ACCESSOR_REFERENCE_RULES: readonly GltfReferenceRule[] = [
  { family: 'accessor', pathPattern: 'document/meshes/{mesh}/primitives/{primitive}/attributes/{semantic}', validate: true, remapOnInsert: true, remapOnRemove: true },
  { family: 'accessor', pathPattern: 'document/meshes/{mesh}/primitives/{primitive}/targets/{target}/{semantic}', validate: true, remapOnInsert: true, remapOnRemove: true },
  { family: 'accessor', pathPattern: 'document/meshes/{mesh}/primitives/{primitive}/indices', validate: true, remapOnInsert: true, remapOnRemove: true },
  { family: 'accessor', pathPattern: 'document/skins/{skin}/inverseBindMatrices', validate: true, remapOnInsert: true, remapOnRemove: true },
  { family: 'accessor', pathPattern: 'document/animations/{animation}/samplers/{sampler}/{input|output}', validate: true, remapOnInsert: true, remapOnRemove: true },
];
export const GLTF_STRUCTURAL_REFERENCE_RULES: readonly GltfReferenceRule[] = [
  { family: 'node', pathPattern: 'incomingInsertNode/children/{slot}', validate: true, remapOnInsert: true, remapOnRemove: false },
  { family: 'buffer', pathPattern: 'document/buffers|snapshot/buffers', validate: true, remapOnInsert: true, remapOnRemove: true },
];

export type GltfMutation =
  | ({ mutation: 'noMutation' } & NoMutation)
  | ({ mutation: 'setSnapshot' } & SetSnapshot)
  | ({ mutation: 'setAsset' } & SetAsset)
  | ({ mutation: 'insertScene' } & InsertScene)
  | ({ mutation: 'removeScene' } & RemoveScene)
  | ({ mutation: 'setScene' } & SetScene)
  | ({ mutation: 'insertNode' } & InsertNode)
  | ({ mutation: 'removeNode' } & RemoveNode)
  | ({ mutation: 'setNode' } & SetNode)
  | ({ mutation: 'insertMesh' } & InsertMesh)
  | ({ mutation: 'removeMesh' } & RemoveMesh)
  | ({ mutation: 'setMesh' } & SetMesh)
  | ({ mutation: 'insertAccessor' } & InsertAccessor)
  | ({ mutation: 'removeAccessor' } & RemoveAccessor)
  | ({ mutation: 'setAccessor' } & SetAccessor)
  | ({ mutation: 'insertMaterial' } & InsertMaterial)
  | ({ mutation: 'removeMaterial' } & RemoveMaterial)
  | ({ mutation: 'setMaterial' } & SetMaterial)
  | ({ mutation: 'insertBuffer' } & InsertBuffer)
  | ({ mutation: 'removeBuffer' } & RemoveBuffer)
  | ({ mutation: 'setBuffer' } & SetBuffer)
  | ({ mutation: 'insertAnimation' } & InsertAnimation)
  | ({ mutation: 'removeAnimation' } & RemoveAnimation)
  | ({ mutation: 'setAnimation' } & SetAnimation)
  | ({ mutation: 'transformNode' } & TransformNode)
  | ({ mutation: 'reparentNode' } & ReparentNode)
  | ({ mutation: 'bindNodeMesh' } & BindNodeMesh)
  | ({ mutation: 'bindPrimitiveMaterial' } & BindPrimitiveMaterial);

