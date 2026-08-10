/** 🧬️ CsvMutation union. */
export type CsvMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: import('../📸️snapshot/🟦️component.ts').CsvSnapshot };
