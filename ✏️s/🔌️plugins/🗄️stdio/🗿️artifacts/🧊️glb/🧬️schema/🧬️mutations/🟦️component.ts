/** 🧬️ GlbMutation union. */
export type GlbMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: import('../📸️snapshot/🟦️component.ts').GlbSnapshot };
