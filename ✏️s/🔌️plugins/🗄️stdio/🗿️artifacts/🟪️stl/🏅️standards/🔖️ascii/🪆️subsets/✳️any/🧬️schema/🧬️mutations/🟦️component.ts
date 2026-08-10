/** 🧬️ StlMutation union. */
export type StlMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: import('../📸️snapshot/🟦️component.ts').StlSnapshot };
