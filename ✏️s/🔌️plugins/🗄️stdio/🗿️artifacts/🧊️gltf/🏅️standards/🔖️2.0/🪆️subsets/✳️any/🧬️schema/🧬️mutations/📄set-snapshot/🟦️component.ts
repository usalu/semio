//#region 🦠️Mutation
/** 🦠️ set-snapshot payload. */
import type { GltfSnapshot } from '../../📸️snapshot/🟦️component.ts';
export interface SetSnapshot { snapshot: GltfSnapshot }
//#endregion 🦠️Mutation

//#region 🔺️Diff
/** 🔺️ set-snapshot sparse diff. */
import type { GltfDiff } from '../../🔺️diff/🟦️component.ts';
export type SetSnapshotDiff = GltfDiff;
//#endregion 🔺️Diff

//#region ↩️Inverse
/** ↩️ set-snapshot semantic inverse. */
import type { GltfMutation } from '../../../🔨️modules/🧭️mutation-dispatch/🟦️component.ts';
export type SetSnapshotInverse = GltfMutation;
//#endregion ↩️Inverse
