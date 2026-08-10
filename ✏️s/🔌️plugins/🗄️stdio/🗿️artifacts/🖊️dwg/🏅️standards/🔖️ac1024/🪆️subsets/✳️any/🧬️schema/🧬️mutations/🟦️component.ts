/** 🧬️ DwgMutation union. */
export type DwgMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: import('../📸️snapshot/🟦️component.ts').DwgSnapshot };
