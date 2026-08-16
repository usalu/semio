//#region 🦠️Mutation
/** 🦠️ set-animation payload. */
import type { GltfAnimation } from '../../📸️snapshot/🟦️component.ts';
export interface SetAnimation { index: number; animation: GltfAnimation }
//#endregion 🦠️Mutation

//#region 🔺️Diff
/** 🔺️ set-animation sparse diff. */
import type { GltfDiff } from '../../🔺️diff/🟦️component.ts';
export type SetAnimationDiff = GltfDiff;
//#endregion 🔺️Diff

//#region ↩️Inverse
/** ↩️ SetAnimation semantic inverse. */
import type { GltfMutation } from '../../../🔨️modules/🧭️mutation-dispatch/🟦️component.ts';
export type SetAnimationInverse = GltfMutation;
//#endregion ↩️Inverse
