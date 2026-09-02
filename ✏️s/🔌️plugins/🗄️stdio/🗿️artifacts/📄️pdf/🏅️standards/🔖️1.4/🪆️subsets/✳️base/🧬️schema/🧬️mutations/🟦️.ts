/** 🧬️ PDF 1.4 Any direct mutation union. */
//#region 🔖️Leaves
import type { InsertPage } from "./📥️insert-page/🟦️";
export type { InsertPage } from "./📥️insert-page/🟦️";
import type { RemovePage } from "./🗑️remove-page/🟦️";
export type { RemovePage } from "./🗑️remove-page/🟦️";
import type { MovePage } from "./🔀️move-page/🟦️";
export type { MovePage } from "./🔀️move-page/🟦️";
import type { ResizePage } from "./📐️resize-page/🟦️";
export type { ResizePage } from "./📐️resize-page/🟦️";
import type { ReplacePageText } from "./📝️replace-page-text/🟦️";
export type { ReplacePageText } from "./📝️replace-page-text/🟦️";
//#endregion 🔖️Leaves

//#region 🔖️Aggregate
export type PdfMutation =
  | { mutation: "insert-page"; payload: InsertPage }
  | { mutation: "remove-page"; payload: RemovePage }
  | { mutation: "move-page"; payload: MovePage }
  | { mutation: "resize-page"; payload: ResizePage }
  | { mutation: "replace-page-text"; payload: ReplacePageText };
//#endregion 🔖️Aggregate
