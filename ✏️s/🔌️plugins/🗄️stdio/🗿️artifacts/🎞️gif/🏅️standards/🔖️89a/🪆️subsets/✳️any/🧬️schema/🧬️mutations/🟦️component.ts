/** 🧬️ GifMutation union. */
export type GifMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: import('../📸️snapshot/🟦️component.ts').GifSnapshot };
