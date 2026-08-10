/** 🧬️ LasMutation union. */
export type LasMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: import('../📸️snapshot/🟦️component.ts').LasSnapshot };
