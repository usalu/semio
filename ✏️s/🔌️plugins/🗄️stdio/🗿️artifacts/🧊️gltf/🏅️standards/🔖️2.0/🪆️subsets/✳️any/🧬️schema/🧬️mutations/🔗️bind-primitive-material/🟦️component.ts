//#region 🦠️Mutation
/** 🦠️ bind-primitive-material payload. */
export interface BindPrimitiveMaterial { mesh: number; primitive: number; material?: number }
//#endregion 🦠️Mutation

//#region 🔺️Diff
/** 🔺️ bind-primitive-material sparse diff. */
import type { GltfDiff } from '../../🔺️diff/🟦️component.ts';
export type BindPrimitiveMaterialDiff = GltfDiff;
//#endregion 🔺️Diff

//#region ↩️Inverse
/** ↩️ BindPrimitiveMaterial semantic inverse. */
import type { GltfMutation } from '../../../🔨️modules/🧭️mutation-dispatch/🟦️component.ts';
export type BindPrimitiveMaterialInverse = GltfMutation;
//#endregion ↩️Inverse
