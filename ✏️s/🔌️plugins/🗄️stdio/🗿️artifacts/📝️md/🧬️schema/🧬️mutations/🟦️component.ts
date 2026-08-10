/** 🧬️ MdMutation union. */
export type MdMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: import('../📸️snapshot/🟦️component.ts').MdSnapshot };
