/** ↩️ Set-snapshot inverse captures the exact pre-mutation snapshot. */
import type { GltfMutation } from '../../🟦️component.ts';
export type GltfSetSnapshotInverse = Extract<GltfMutation, { mutation: 'setSnapshot' }>;
