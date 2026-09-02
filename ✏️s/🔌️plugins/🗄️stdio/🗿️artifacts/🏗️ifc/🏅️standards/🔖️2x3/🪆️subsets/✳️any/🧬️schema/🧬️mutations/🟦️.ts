import type { Ifc2x3Snapshot, Part21Header, Part21Instance } from "../📸️snapshot/🟦️.ts";

/** 🧬️ Ifc2x3Mutation schema. Mirrors the Rust `Ifc2x3Mutation` enum field-for-field — `snapshot`,
 * `instance`, and `header` were drifted to `unknown`; `../📸️snapshot/🟦️.ts` already
 * types them as `Ifc2x3Snapshot`/`Part21Instance`/`Part21Header`, matching the Rust leaf structs'
 * `Ifc2x3Snapshot`/`Part21Instance`/`Part21Header` payload fields. */
export type Ifc2x3Mutation =
  | { mutation: "setSnapshot"; snapshot: Ifc2x3Snapshot }
  | { mutation: "upsertInstance"; instance: Part21Instance }
  | { mutation: "removeInstance"; id: number }
  | { mutation: "setHeader"; header: Part21Header };
