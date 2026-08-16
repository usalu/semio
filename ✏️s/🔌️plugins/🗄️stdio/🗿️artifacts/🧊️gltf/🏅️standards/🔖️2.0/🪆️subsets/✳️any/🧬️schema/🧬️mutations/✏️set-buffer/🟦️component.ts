//#region 🦠️Mutation
/** 🦠️ set-buffer payload. */
import type { GltfBuffer } from '../../📸️snapshot/🟦️component.ts';
export interface SetBuffer { index: number; buffer: GltfBuffer; bytes: number[] }
//#endregion 🦠️Mutation

//#region 🔺️Diff
/** 🔺️ set-buffer sparse diff. */
import type { GltfDiff } from '../../🔺️diff/🟦️component.ts';
export type SetBufferDiff = GltfDiff;
//#endregion 🔺️Diff

//#region ↩️Inverse
/** ↩️ SetBuffer semantic inverse. */
import type { GltfMutation } from '../../../🔨️modules/🧭️mutation-dispatch/🟦️component.ts';
export type SetBufferInverse = GltfMutation;
//#endregion ↩️Inverse
