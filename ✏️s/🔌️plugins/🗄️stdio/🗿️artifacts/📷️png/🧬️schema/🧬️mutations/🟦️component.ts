/** 🧬️ PngMutation union. */
export type PngMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: import('../📸️snapshot/🟦️component.ts').PngSnapshot };
