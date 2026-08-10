/** 🧬️ SvgMutation union. */
export type SvgMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: import('../📸️snapshot/🟦️component.ts').SvgSnapshot };
