//#region 🦠️Mutation
/** 🦠️ insert-animation payload. */
import type { GltfAnimation } from '../../📸️snapshot/🟦️component.ts';
export interface InsertAnimation { index: number; animation: GltfAnimation }
//#endregion 🦠️Mutation

//#region 🔺️Diff
/** 🔺️ insert-animation sparse diff. */
import type { GltfDiff } from '../../🔺️diff/🟦️component.ts';
export type InsertAnimationDiff = GltfDiff;
//#endregion 🔺️Diff

//#region ↩️Inverse
/** ↩️ InsertAnimation semantic inverse. */
import type { GltfMutation } from '../../../🔨️modules/🧭️mutation-dispatch/🟦️component.ts';
export type InsertAnimationInverse = GltfMutation;
//#endregion ↩️Inverse
