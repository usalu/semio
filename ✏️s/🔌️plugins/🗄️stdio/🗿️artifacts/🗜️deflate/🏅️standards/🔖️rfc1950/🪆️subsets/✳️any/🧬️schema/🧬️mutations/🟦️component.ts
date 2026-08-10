/** 🧬️ DeflateMutation union. */
export type DeflateMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: import('../📸️snapshot/🟦️component.ts').DeflateSnapshot };
