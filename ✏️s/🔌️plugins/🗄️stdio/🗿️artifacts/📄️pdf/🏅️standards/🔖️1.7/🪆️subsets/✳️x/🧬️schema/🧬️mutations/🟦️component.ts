/** 🧬️ Transparent PDF/X mutation TypeScript union assembled from direct owners. */

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
//#endregion 🔖️Leaves

//#region 🔖️Aggregate
export type PdfXMutation =
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
  | RemoveMediaAnnotationMutation;
//#endregion 🔖️Aggregate
