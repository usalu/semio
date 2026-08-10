/** 🧬️ TiffMutation union. */
export type TiffMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: import('../📸️snapshot/🟦️component.ts').TiffSnapshot };
