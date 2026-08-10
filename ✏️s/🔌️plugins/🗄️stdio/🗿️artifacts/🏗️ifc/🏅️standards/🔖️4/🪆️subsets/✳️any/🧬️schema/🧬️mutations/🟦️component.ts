/** 🧬️ IfcMutation union. */
export type IfcMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: import('../📸️snapshot/🟦️component.ts').IfcSnapshot };
