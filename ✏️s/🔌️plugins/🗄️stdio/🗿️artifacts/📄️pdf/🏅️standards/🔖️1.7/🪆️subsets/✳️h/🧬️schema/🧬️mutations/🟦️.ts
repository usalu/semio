/** 🧬️ Transparent PDF/H mutation TypeScript union assembled from direct owners. */

import type { SetInfoTitleMutation } from './🏷️set-info-title/🟦️.ts';
import type { SetInfoAuthorMutation } from './👤️set-info-author/🟦️.ts';
import type { InsertJavascriptActionMutation } from './📜️insert-javascript-action/🟦️.ts';
import type { RemoveJavascriptActionMutation } from './🚫️remove-javascript-action/🟦️.ts';
import type { InsertLaunchActionMutation } from './🚀️insert-launch-action/🟦️.ts';
import type { RemoveLaunchActionMutation } from './🛬️remove-launch-action/🟦️.ts';
import type { InsertSignatureFieldMutation } from './✒️insert-signature-field/🟦️.ts';
import type { RemoveSignatureFieldMutation } from './✂️remove-signature-field/🟦️.ts';
import type { EmbedFontFileMutation } from './🔤️embed-font-file/🟦️.ts';
import type { RemoveFontFileMutation } from './🧺️remove-font-file/🟦️.ts';

export type PdfHMutation =
  | SetInfoTitleMutation
  | SetInfoAuthorMutation
  | InsertJavascriptActionMutation
  | RemoveJavascriptActionMutation
  | InsertLaunchActionMutation
  | RemoveLaunchActionMutation
  | InsertSignatureFieldMutation
  | RemoveSignatureFieldMutation
  | EmbedFontFileMutation
  | RemoveFontFileMutation;
