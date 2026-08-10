/** 🧬️ DxfMutation union. */
export type DxfMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: import('../📸️snapshot/🟦️component.ts').DxfSnapshot };
