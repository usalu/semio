/** 🧬️ Transparent PDF/E mutation TypeScript union assembled from direct owners. */

import type { InsertEncryptionDictionaryMutation } from './🔒️insert-encryption-dictionary/🟦️component.ts';
import type { RemoveEncryptionDictionaryMutation } from './🔓️remove-encryption-dictionary/🟦️component.ts';
import type { InsertJavascriptActionMutation } from './📜️insert-javascript-action/🟦️component.ts';
import type { RemoveJavascriptActionMutation } from './🚫️remove-javascript-action/🟦️component.ts';
import type { InsertLaunchActionMutation } from './🚀️insert-launch-action/🟦️component.ts';
import type { RemoveLaunchActionMutation } from './🛬️remove-launch-action/🟦️component.ts';
import type { InsertMediaAnnotationMutation } from './🎬️insert-media-annotation/🟦️component.ts';
import type { RemoveMediaAnnotationMutation } from './⏹️remove-media-annotation/🟦️component.ts';
import type { SetOutputIntentMutation } from './🏳️set-output-intent/🟦️component.ts';
import type { RemoveOutputIntentMutation } from './🧽️remove-output-intent/🟦️component.ts';
import type { EmbedFontFileMutation } from './🔤️embed-font-file/🟦️component.ts';
import type { RemoveFontFileMutation } from './🧺️remove-font-file/🟦️component.ts';

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
