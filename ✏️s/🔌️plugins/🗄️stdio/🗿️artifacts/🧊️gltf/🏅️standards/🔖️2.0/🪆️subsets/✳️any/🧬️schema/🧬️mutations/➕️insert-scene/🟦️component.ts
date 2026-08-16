//#region 🦠️Mutation
/** 🦠️ insert-scene payload. */
import type { GltfScene } from '../../📸️snapshot/🟦️component.ts';
export interface InsertScene { index: number; scene: GltfScene }
//#endregion 🦠️Mutation

//#region 🔺️Diff
/** 🔺️ insert-scene sparse diff. */
import type { GltfDiff } from '../../🔺️diff/🟦️component.ts';
export type InsertSceneDiff = GltfDiff;
//#endregion 🔺️Diff

//#region ↩️Inverse
/** ↩️ InsertScene semantic inverse. */
import type { GltfMutation } from '../../../🔨️modules/🧭️mutation-dispatch/🟦️component.ts';
export type InsertSceneInverse = GltfMutation;
//#endregion ↩️Inverse
