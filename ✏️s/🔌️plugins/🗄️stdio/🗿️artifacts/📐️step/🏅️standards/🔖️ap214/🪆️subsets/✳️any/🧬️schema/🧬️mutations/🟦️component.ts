/** 🧬️ StepMutation union. */
export type StepMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: import('../📸️snapshot/🟦️component.ts').StepSnapshot };
