/** 🧬️ PlyMutation union. */
export type PlyMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: import('../📸️snapshot/🟦️component.ts').PlySnapshot };
