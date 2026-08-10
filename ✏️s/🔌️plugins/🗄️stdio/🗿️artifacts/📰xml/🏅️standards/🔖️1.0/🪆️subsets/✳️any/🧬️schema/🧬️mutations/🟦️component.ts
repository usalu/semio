/** 🧬️ XmlMutation union. */
export type XmlMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: import('../📸️snapshot/🟦️component.ts').XmlSnapshot };
