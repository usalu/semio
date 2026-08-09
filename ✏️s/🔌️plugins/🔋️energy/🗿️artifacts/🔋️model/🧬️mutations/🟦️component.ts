/** 🧩 energy-model mutations facade. */
export type EnergyModelMutation =
  | { mutation: "noMutation" }
  | { mutation: "setSnapshot"; snapshot: { schema: string; modelJson: string } };
