/** 🧬️ BinaryMutation union. */
export type BinaryMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: import('../📸️snapshot/🟦️component.ts').BinarySnapshot };
