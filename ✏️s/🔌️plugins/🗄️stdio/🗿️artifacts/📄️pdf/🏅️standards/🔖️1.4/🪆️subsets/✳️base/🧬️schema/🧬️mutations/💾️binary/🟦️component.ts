/** 💾️ Native payload behind the common format-byte/tag-byte header. */
export const PDF_MUTATION_TAGS = {
  "insert-page": 0,
  "remove-page": 1,
  "move-page": 2,
  "resize-page": 3,
  "replace-page-text": 4,
} as const;
export type PdfMutationBinary = Uint8Array;
