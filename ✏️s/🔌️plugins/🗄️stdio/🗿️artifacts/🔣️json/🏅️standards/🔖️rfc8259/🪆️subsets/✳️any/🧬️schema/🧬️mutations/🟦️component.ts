/** 🧬️ JsonMutation union. */
export type JsonMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: import('../📸️snapshot/🟦️component.ts').JsonSnapshot };
