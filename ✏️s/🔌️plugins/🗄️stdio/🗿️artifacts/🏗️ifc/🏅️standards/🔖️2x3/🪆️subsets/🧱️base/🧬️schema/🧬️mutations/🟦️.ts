import type { Ifc2x3Snapshot, Part21Header, Part21Instance } from "../📸️snapshot/🟦️.ts";
import type { SnapshotPatch } from '../../../../../../../../📇️registry/🧬️contract/✏️editing/🩹️patch/🟦️.ts';

/** 🧬️ Ifc2x3Mutation schema. Mirrors the Rust `Ifc2x3Mutation` enum field-for-field — `snapshot`,
 * `instance`, and `header` were drifted to `unknown`; `../📸️snapshot/🟦️.ts` already
 * types them as `Ifc2x3Snapshot`/`Part21Instance`/`Part21Header`, matching the Rust leaf structs'
 * `Ifc2x3Snapshot`/`Part21Instance`/`Part21Header` payload fields. */
export type Ifc2x3Mutation =
  | { mutation: "setSnapshot"; snapshot: Ifc2x3Snapshot }
  | { readonly mutation: 'patchSnapshot'; readonly patch: SnapshotPatch }
  | { mutation: "upsertInstance"; instance: Part21Instance }
  | { mutation: "removeInstance"; id: Part21Instance["id"] }
  | { mutation: "setHeader"; header: Part21Header };
