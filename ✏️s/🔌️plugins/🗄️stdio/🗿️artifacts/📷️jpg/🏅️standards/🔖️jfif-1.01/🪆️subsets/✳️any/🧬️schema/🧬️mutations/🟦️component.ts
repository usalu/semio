/** 🧬️ JpgMutation union. */
export type JpgMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: import('../📸️snapshot/🟦️component.ts').JpgSnapshot };
