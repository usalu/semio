/** 🧬️ ZipMutation union. */
export type ZipMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: import('../📸️snapshot/🟦️component.ts').ZipSnapshot };
