/** 🧬️ DocxMutation union. */
export type DocxMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: import('../📸️snapshot/🟦️component.ts').DocxSnapshot };
