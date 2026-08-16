//#region 🦠️Mutation
/** 🦠️ insert-material payload. */
import type { GltfMaterial } from '../../📸️snapshot/🟦️component.ts';
export interface InsertMaterial { index: number; material: GltfMaterial }
//#endregion 🦠️Mutation

//#region 🔺️Diff
/** 🔺️ insert-material sparse diff. */
import type { GltfDiff } from '../../🔺️diff/🟦️component.ts';
export type InsertMaterialDiff = GltfDiff;
//#endregion 🔺️Diff

//#region ↩️Inverse
/** ↩️ InsertMaterial semantic inverse. */
import type { GltfMutation } from '../../../🔨️modules/🧭️mutation-dispatch/🟦️component.ts';
export type InsertMaterialInverse = GltfMutation;
//#endregion ↩️Inverse
