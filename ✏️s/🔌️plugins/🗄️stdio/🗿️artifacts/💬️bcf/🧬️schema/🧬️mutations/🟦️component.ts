/** 🧬️ BcfMutation union. */
export type BcfMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: import('../📸️snapshot/🟦️component.ts').BcfSnapshot };
