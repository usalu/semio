/** 🧬️ Ifc2x3Mutation schema. */
export type Ifc2x3Mutation =
  | { mutation: "noMutation" }
  | { mutation: "setSnapshot"; snapshot: unknown }
  | { mutation: "upsertInstance"; instance: unknown }
  | { mutation: "removeInstance"; id: number }
  | { mutation: "setHeader"; header: unknown };
