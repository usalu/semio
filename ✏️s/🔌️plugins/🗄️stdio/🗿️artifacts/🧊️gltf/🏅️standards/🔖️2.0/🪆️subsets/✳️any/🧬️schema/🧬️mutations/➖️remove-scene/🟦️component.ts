//#region 🦠️Mutation
/** 🦠️ remove-scene payload. */
export interface RemoveScene { index: number }
//#endregion 🦠️Mutation

//#region 🔺️Diff
/** 🔺️ remove-scene sparse diff. */
import type { GltfDiff } from '../../🔺️diff/🟦️component.ts';
export type RemoveSceneDiff = GltfDiff;
//#endregion 🔺️Diff

//#region ↩️Inverse
/** ↩️ RemoveScene semantic inverse. */
import type { GltfMutation } from '../../../🔨️modules/🧭️mutation-dispatch/🟦️component.ts';
export type RemoveSceneInverse = GltfMutation;
//#endregion ↩️Inverse
