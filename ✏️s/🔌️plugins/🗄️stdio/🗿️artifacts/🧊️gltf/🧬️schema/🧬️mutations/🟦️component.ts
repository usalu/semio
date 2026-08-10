/** 🧬️ GltfMutation union. */
export type GltfMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: import('../📸️snapshot/🟦️component.ts').GltfSnapshot };
