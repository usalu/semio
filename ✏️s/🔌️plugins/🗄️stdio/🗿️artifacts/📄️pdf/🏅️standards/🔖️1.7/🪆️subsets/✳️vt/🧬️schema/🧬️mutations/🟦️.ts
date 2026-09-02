/** 🧬️ Transparent PDF/VT mutation TypeScript union assembled from direct owners. */

//#region 🔖️Leaves
import type { InsertEncryptionDictionaryMutation } from './🔒️insert-encryption-dictionary/🟦️.ts';
import type { RemoveEncryptionDictionaryMutation } from './🔓️remove-encryption-dictionary/🟦️.ts';
import type { SetOutputIntentMutation } from './🏳️set-output-intent/🟦️.ts';
import type { RemoveOutputIntentMutation } from './🧽️remove-output-intent/🟦️.ts';
import type { SetTrimBoxMutation } from './📐️set-trim-box/🟦️.ts';
import type { RemoveTrimBoxMutation } from './🧽️remove-trim-box/🟦️.ts';
import type { EmbedFontFileMutation } from './🔤️embed-font-file/🟦️.ts';
import type { RemoveFontFileMutation } from './🧺️remove-font-file/🟦️.ts';
import type { InsertJavascriptActionMutation } from './📜️insert-javascript-action/🟦️.ts';
import type { RemoveJavascriptActionMutation } from './🚫️remove-javascript-action/🟦️.ts';
import type { InsertLaunchActionMutation } from './🚀️insert-launch-action/🟦️.ts';
import type { RemoveLaunchActionMutation } from './🛬️remove-launch-action/🟦️.ts';
import type { InsertMediaAnnotationMutation } from './🎬️insert-media-annotation/🟦️.ts';
import type { RemoveMediaAnnotationMutation } from './⏹️remove-media-annotation/🟦️.ts';
import type { SetDpartRootMutation } from './🗂️set-dpart-root/🟦️.ts';
import type { RemoveDpartRootMutation } from './🧹️remove-dpart-root/🟦️.ts';
import type { SetDpartMetadataMutation } from './🏷️set-dpart-metadata/🟦️.ts';
import type { RemoveDpartMetadataMutation } from './🗑️remove-dpart-metadata/🟦️.ts';
//#endregion 🔖️Leaves

//#region 🔖️Aggregate
export type PdfVtMutation =
  | InsertEncryptionDictionaryMutation
  | RemoveEncryptionDictionaryMutation
  | SetOutputIntentMutation
  | RemoveOutputIntentMutation
  | SetTrimBoxMutation
  | RemoveTrimBoxMutation
  | EmbedFontFileMutation
  | RemoveFontFileMutation
  | InsertJavascriptActionMutation
  | RemoveJavascriptActionMutation
  | InsertLaunchActionMutation
  | RemoveLaunchActionMutation
  | InsertMediaAnnotationMutation
  | RemoveMediaAnnotationMutation
  | SetDpartRootMutation
  | RemoveDpartRootMutation
  | SetDpartMetadataMutation
  | RemoveDpartMetadataMutation;
//#endregion 🔖️Aggregate
