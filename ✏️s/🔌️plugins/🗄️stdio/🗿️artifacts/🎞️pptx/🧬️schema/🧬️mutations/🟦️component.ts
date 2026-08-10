/** 🧬️ PptxMutation union. */
export type PptxMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: import('../📸️snapshot/🟦️component.ts').PptxSnapshot };
