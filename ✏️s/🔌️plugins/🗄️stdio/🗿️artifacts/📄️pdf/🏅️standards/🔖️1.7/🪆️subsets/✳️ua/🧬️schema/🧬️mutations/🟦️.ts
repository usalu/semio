/** 🧬️ Transparent PDF/UA mutation TypeScript union assembled from direct owners. */

//#region 🔖️Leaves
import type { SetMarkInfoMutation } from './🏷️set-mark-info/🟦️.ts';
import type { RemoveMarkInfoMutation } from './🗑️remove-mark-info/🟦️.ts';
import type { SetStructTreeRootMutation } from './🌲️set-struct-tree-root/🟦️.ts';
import type { RemoveStructTreeRootMutation } from './🪓️remove-struct-tree-root/🟦️.ts';
import type { SetLangMutation } from './🗣️set-lang/🟦️.ts';
import type { RemoveLangMutation } from './🤐️remove-lang/🟦️.ts';
import type { SetDisplayDocTitleMutation } from './🪧️set-display-doc-title/🟦️.ts';
import type { RemoveDisplayDocTitleMutation } from './🚫️remove-display-doc-title/🟦️.ts';
import type { SetInfoTitleMutation } from './🏷️set-info-title/🟦️.ts';
import type { EmbedFontFileMutation } from './🔤️embed-font-file/🟦️.ts';
import type { RemoveFontFileMutation } from './🧺️remove-font-file/🟦️.ts';
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
