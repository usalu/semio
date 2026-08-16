//#region 🦠️Mutation
/** 🦠️ set-material payload. */
import type { GltfMaterial } from '../../📸️snapshot/🟦️component.ts';
export interface SetMaterial { index: number; material: GltfMaterial }
//#endregion 🦠️Mutation

//#region 🔺️Diff
/** 🔺️ set-material sparse diff. */
import type { GltfDiff } from '../../🔺️diff/🟦️component.ts';
export type SetMaterialDiff = GltfDiff;
//#endregion 🔺️Diff

//#region ↩️Inverse
/** ↩️ SetMaterial semantic inverse. */
import type { GltfMutation } from '../../../🔨️modules/🧭️mutation-dispatch/🟦️component.ts';
export type SetMaterialInverse = GltfMutation;
//#endregion ↩️Inverse
