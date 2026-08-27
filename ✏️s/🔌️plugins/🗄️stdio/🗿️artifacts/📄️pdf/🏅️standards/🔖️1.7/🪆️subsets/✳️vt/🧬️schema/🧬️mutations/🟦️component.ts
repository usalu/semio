/** 🧬️ Transparent PDF/VT mutation TypeScript union assembled from direct owners. */

//#region 🔖️Leaves
import type { InsertEncryptionDictionaryMutation } from './🔒️insert-encryption-dictionary/🟦️component.ts';
import type { RemoveEncryptionDictionaryMutation } from './🔓️remove-encryption-dictionary/🟦️component.ts';
import type { SetOutputIntentMutation } from './🏳️set-output-intent/🟦️component.ts';
import type { RemoveOutputIntentMutation } from './🧽️remove-output-intent/🟦️component.ts';
import type { SetTrimBoxMutation } from './📐️set-trim-box/🟦️component.ts';
import type { RemoveTrimBoxMutation } from './🧽️remove-trim-box/🟦️component.ts';
import type { EmbedFontFileMutation } from './🔤️embed-font-file/🟦️component.ts';
import type { RemoveFontFileMutation } from './🧺️remove-font-file/🟦️component.ts';
import type { InsertJavascriptActionMutation } from './📜️insert-javascript-action/🟦️component.ts';
import type { RemoveJavascriptActionMutation } from './🚫️remove-javascript-action/🟦️component.ts';
import type { InsertLaunchActionMutation } from './🚀️insert-launch-action/🟦️component.ts';
import type { RemoveLaunchActionMutation } from './🛬️remove-launch-action/🟦️component.ts';
import type { InsertMediaAnnotationMutation } from './🎬️insert-media-annotation/🟦️component.ts';
import type { RemoveMediaAnnotationMutation } from './⏹️remove-media-annotation/🟦️component.ts';
import type { SetDpartRootMutation } from './🗂️set-dpart-root/🟦️component.ts';
import type { RemoveDpartRootMutation } from './🧹️remove-dpart-root/🟦️component.ts';
import type { SetDpartMetadataMutation } from './🏷️set-dpart-metadata/🟦️component.ts';
import type { RemoveDpartMetadataMutation } from './🗑️remove-dpart-metadata/🟦️component.ts';
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
