/** 🧬️ PdfMutation union. */
export type PdfMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: import('../📸️snapshot/🟦️component.ts').PdfSnapshot };
