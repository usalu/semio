/** 🧬️ Transparent PDF/UA mutation TypeScript union assembled from direct owners. */

//#region 🔖️Leaves
import type { SetMarkInfoMutation } from './🏷️set-mark-info/🟦️component.ts';
import type { RemoveMarkInfoMutation } from './🗑️remove-mark-info/🟦️component.ts';
import type { SetStructTreeRootMutation } from './🌲️set-struct-tree-root/🟦️component.ts';
import type { RemoveStructTreeRootMutation } from './🪓️remove-struct-tree-root/🟦️component.ts';
import type { SetLangMutation } from './🗣️set-lang/🟦️component.ts';
import type { RemoveLangMutation } from './🤐️remove-lang/🟦️component.ts';
import type { SetDisplayDocTitleMutation } from './🪧️set-display-doc-title/🟦️component.ts';
import type { RemoveDisplayDocTitleMutation } from './🚫️remove-display-doc-title/🟦️component.ts';
import type { SetInfoTitleMutation } from './🏷️set-info-title/🟦️component.ts';
import type { EmbedFontFileMutation } from './🔤️embed-font-file/🟦️component.ts';
import type { RemoveFontFileMutation } from './🧺️remove-font-file/🟦️component.ts';
//#endregion 🔖️Leaves

//#region 🔖️Aggregate
export type PdfUaMutation =
  | SetMarkInfoMutation
  | RemoveMarkInfoMutation
  | SetStructTreeRootMutation
  | RemoveStructTreeRootMutation
  | SetLangMutation
  | RemoveLangMutation
  | SetDisplayDocTitleMutation
  | RemoveDisplayDocTitleMutation
  | SetInfoTitleMutation
  | EmbedFontFileMutation
  | RemoveFontFileMutation;
//#endregion 🔖️Aggregate
