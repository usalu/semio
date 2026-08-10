/** 🧬️ XlsxMutation union. */
export type XlsxMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: import('../📸️snapshot/🟦️component.ts').XlsxSnapshot };
