//#region 🦠️Mutation
/** 🦠️ set-node payload. */
import type { GltfNode } from '../../📸️snapshot/🟦️component.ts';
export interface SetNode { index: number; node: GltfNode }
//#endregion 🦠️Mutation

//#region 🔺️Diff
/** 🔺️ set-node sparse diff. */
import type { GltfDiff } from '../../🔺️diff/🟦️component.ts';
export type SetNodeDiff = GltfDiff;
//#endregion 🔺️Diff

//#region ↩️Inverse
/** ↩️ SetNode semantic inverse. */
import type { GltfMutation } from '../../../🔨️modules/🧭️mutation-dispatch/🟦️component.ts';
export type SetNodeInverse = GltfMutation;
//#endregion ↩️Inverse
