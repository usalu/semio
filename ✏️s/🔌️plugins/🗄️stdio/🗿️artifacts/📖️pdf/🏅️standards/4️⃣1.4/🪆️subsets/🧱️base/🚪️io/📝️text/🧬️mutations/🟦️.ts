/** 📝️ Original five hex payloads, readable own14 snapshot DSL and original patch text. */
export type PdfMutationOpcode = "insert-page" | "remove-page" | "move-page" | "resize-page" | "replace-page-text";
export type PdfMutationText = `${PdfMutationOpcode} payload=${string}`;
