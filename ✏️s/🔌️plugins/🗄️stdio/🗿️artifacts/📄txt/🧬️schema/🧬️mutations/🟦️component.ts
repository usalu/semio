/** 🧬️ TxtMutation union. */
export type TxtMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: import('../📸️snapshot/🟦️component.ts').TxtSnapshot };
