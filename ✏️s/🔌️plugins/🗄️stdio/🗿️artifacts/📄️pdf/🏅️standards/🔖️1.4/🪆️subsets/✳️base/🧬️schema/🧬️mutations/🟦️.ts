/** 🧬️ PDF 1.4 Any direct mutation union. */
//#region 🔖️Leaves
import type { InsertPage } from "./📥️insert-page/🟦️component";
export type { InsertPage } from "./📥️insert-page/🟦️component";
import type { RemovePage } from "./🗑️remove-page/🟦️component";
export type { RemovePage } from "./🗑️remove-page/🟦️component";
import type { MovePage } from "./🔀️move-page/🟦️component";
export type { MovePage } from "./🔀️move-page/🟦️component";
import type { ResizePage } from "./📐️resize-page/🟦️component";
export type { ResizePage } from "./📐️resize-page/🟦️component";
import type { ReplacePageText } from "./📝️replace-page-text/🟦️component";
export type { ReplacePageText } from "./📝️replace-page-text/🟦️component";
//#endregion 🔖️Leaves

//#region 🔖️Aggregate
export type PdfMutation =
  | { mutation: "insert-page"; payload: InsertPage }
  | { mutation: "remove-page"; payload: RemovePage }
  | { mutation: "move-page"; payload: MovePage }
  | { mutation: "resize-page"; payload: ResizePage }
  | { mutation: "replace-page-text"; payload: ReplacePageText };
//#endregion 🔖️Aggregate
