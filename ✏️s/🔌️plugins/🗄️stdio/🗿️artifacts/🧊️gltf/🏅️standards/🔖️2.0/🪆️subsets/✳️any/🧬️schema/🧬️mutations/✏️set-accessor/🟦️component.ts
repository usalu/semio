//#region 🦠️Mutation
/** 🦠️ set-accessor payload. */
import type { GltfAccessor } from '../../📸️snapshot/🟦️component.ts';
export interface SetAccessor { index: number; accessor: GltfAccessor }
//#endregion 🦠️Mutation

//#region 🔺️Diff
/** 🔺️ set-accessor sparse diff. */
import type { GltfDiff } from '../../🔺️diff/🟦️component.ts';
export type SetAccessorDiff = GltfDiff;
//#endregion 🔺️Diff

//#region ↩️Inverse
/** ↩️ SetAccessor semantic inverse. */
import type { GltfMutation } from '../../../🔨️modules/🧭️mutation-dispatch/🟦️component.ts';
export type SetAccessorInverse = GltfMutation;
//#endregion ↩️Inverse
