/** 🧬️ BmpMutation union. */
export type BmpMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: import('../📸️snapshot/🟦️component.ts').BmpSnapshot };
