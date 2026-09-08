/** 🧬️ Transparent PDF/E mutation TypeScript union assembled from direct owners. */

import type { InsertEncryptionDictionaryMutation } from './🔒️insert-encryption-dictionary/🟦️.ts';
import type { RemoveEncryptionDictionaryMutation } from './🔓️remove-encryption-dictionary/🟦️.ts';
import type { InsertJavascriptActionMutation } from './📜️insert-javascript-action/🟦️.ts';
import type { RemoveJavascriptActionMutation } from './🚫️remove-javascript-action/🟦️.ts';
import type { InsertLaunchActionMutation } from './🚀️insert-launch-action/🟦️.ts';
import type { RemoveLaunchActionMutation } from './🛬️remove-launch-action/🟦️.ts';
import type { InsertMediaAnnotationMutation } from './🎬️insert-media-annotation/🟦️.ts';
import type { RemoveMediaAnnotationMutation } from './⏹️remove-media-annotation/🟦️.ts';
import type { SetOutputIntentMutation } from './🏳️set-output-intent/🟦️.ts';
import type { RemoveOutputIntentMutation } from './🧽️remove-output-intent/🟦️.ts';
import type { EmbedFontFileMutation } from './🔤️embed-font-file/🟦️.ts';
import type { RemoveFontFileMutation } from './🧺️remove-font-file/🟦️.ts';

export type PdfEMutation =
  | InsertEncryptionDictionaryMutation
  | RemoveEncryptionDictionaryMutation
  | InsertJavascriptActionMutation
  | RemoveJavascriptActionMutation
  | InsertLaunchActionMutation
  | RemoveLaunchActionMutation
  | InsertMediaAnnotationMutation
  | RemoveMediaAnnotationMutation
  | SetOutputIntentMutation
  | RemoveOutputIntentMutation
  | EmbedFontFileMutation
  | RemoveFontFileMutation;
