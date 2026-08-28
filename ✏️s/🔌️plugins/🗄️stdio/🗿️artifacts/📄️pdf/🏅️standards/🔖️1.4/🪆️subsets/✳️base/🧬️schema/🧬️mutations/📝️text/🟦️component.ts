/** 📝️ One direct payload encoded as hexadecimal UTF-8 JSON. */
export type PdfMutationOpcode = "insert-page" | "remove-page" | "move-page" | "resize-page" | "replace-page-text";
export type PdfMutationText = `${PdfMutationOpcode} payload=${string}`;
