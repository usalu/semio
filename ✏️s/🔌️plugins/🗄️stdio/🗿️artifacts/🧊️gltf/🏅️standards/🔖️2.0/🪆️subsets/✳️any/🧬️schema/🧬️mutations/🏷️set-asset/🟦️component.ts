//#region 🦠️Mutation
/** 🦠️ set-asset payload. */
import type { GltfAsset } from '../../📸️snapshot/🟦️component.ts';
export interface SetAsset { asset: GltfAsset }
//#endregion 🦠️Mutation

//#region 🔺️Diff
/** 🔺️ set-asset sparse diff. */
import type { GltfDiff } from '../../🔺️diff/🟦️component.ts';
export type SetAssetDiff = GltfDiff;
//#endregion 🔺️Diff

//#region ↩️Inverse
/** ↩️ SetAsset semantic inverse. */
import type { GltfMutation } from '../../../🔨️modules/🧭️mutation-dispatch/🟦️component.ts';
export type SetAssetInverse = GltfMutation;
//#endregion ↩️Inverse
