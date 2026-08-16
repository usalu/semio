//#region 🦠️Mutation
/** 🦠️ set-scene payload. */
import type { GltfScene } from '../../📸️snapshot/🟦️component.ts';
export interface SetScene { index: number; scene: GltfScene }
//#endregion 🦠️Mutation

//#region 🔺️Diff
/** 🔺️ set-scene sparse diff. */
import type { GltfDiff } from '../../🔺️diff/🟦️component.ts';
export type SetSceneDiff = GltfDiff;
//#endregion 🔺️Diff

//#region ↩️Inverse
/** ↩️ SetScene semantic inverse. */
import type { GltfMutation } from '../../../🔨️modules/🧭️mutation-dispatch/🟦️component.ts';
export type SetSceneInverse = GltfMutation;
//#endregion ↩️Inverse
