/** 🧬️ ObjMutation union. */
export type ObjMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: import('../📸️snapshot/🟦️component.ts').ObjSnapshot };
