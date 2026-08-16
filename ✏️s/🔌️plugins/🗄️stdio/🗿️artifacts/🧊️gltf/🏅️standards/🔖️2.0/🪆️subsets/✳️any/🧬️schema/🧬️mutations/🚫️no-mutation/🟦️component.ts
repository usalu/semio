//#region 🦠️Mutation
/** 🦠️ no-mutation payload. */
export interface NoMutation {  }
//#endregion 🦠️Mutation

//#region 🔺️Diff
/** 🔺️ no-mutation sparse diff. */
import type { GltfDiff } from '../../🔺️diff/🟦️component.ts';
export type NoMutationDiff = GltfDiff;
//#endregion 🔺️Diff

//#region ↩️Inverse
/** ↩️ NoMutation semantic inverse. */
import type { GltfMutation } from '../../../🔨️modules/🧭️mutation-dispatch/🟦️component.ts';
export type NoMutationInverse = GltfMutation;
//#endregion ↩️Inverse
