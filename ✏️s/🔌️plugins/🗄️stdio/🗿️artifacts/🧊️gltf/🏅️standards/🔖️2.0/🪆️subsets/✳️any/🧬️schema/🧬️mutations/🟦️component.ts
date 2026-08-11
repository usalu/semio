/** 🧬️ GltfMutation union — real named variants (highest-value arrays get Insert/Remove/Set
 * triads; the rest are reachable only via `setSnapshot` in this wave). */
import type { GltfSnapshot, GltfAsset, GltfScene, GltfNode, GltfMesh, GltfAccessor, GltfMaterial, GltfBuffer, GltfAnimation } from '../📸️snapshot/🟦️component.ts';

export type GltfMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: GltfSnapshot }
  | { mutation: 'setAsset'; asset: GltfAsset }
  | { mutation: 'insertScene'; index: number; scene: GltfScene }
  | { mutation: 'removeScene'; index: number }
  | { mutation: 'setScene'; index: number; scene: GltfScene }
  | { mutation: 'insertNode'; index: number; node: GltfNode }
  | { mutation: 'removeNode'; index: number }
  | { mutation: 'setNode'; index: number; node: GltfNode }
  | { mutation: 'insertMesh'; index: number; mesh: GltfMesh }
  | { mutation: 'removeMesh'; index: number }
  | { mutation: 'setMesh'; index: number; mesh: GltfMesh }
  | { mutation: 'insertAccessor'; index: number; accessor: GltfAccessor }
  | { mutation: 'removeAccessor'; index: number }
  | { mutation: 'setAccessor'; index: number; accessor: GltfAccessor }
  | { mutation: 'insertMaterial'; index: number; material: GltfMaterial }
  | { mutation: 'removeMaterial'; index: number }
  | { mutation: 'setMaterial'; index: number; material: GltfMaterial }
  | { mutation: 'insertBuffer'; index: number; buffer: GltfBuffer; bytes: number[] }
  | { mutation: 'removeBuffer'; index: number }
  | { mutation: 'setBuffer'; index: number; buffer: GltfBuffer; bytes: number[] }
  | { mutation: 'insertAnimation'; index: number; animation: GltfAnimation }
  | { mutation: 'removeAnimation'; index: number }
  | { mutation: 'setAnimation'; index: number; animation: GltfAnimation };
