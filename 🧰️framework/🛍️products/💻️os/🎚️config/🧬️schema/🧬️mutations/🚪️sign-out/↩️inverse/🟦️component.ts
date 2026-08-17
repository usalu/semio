/** ↩️ Inverse for `SignOut` — reads the BASE identity, never the diff: a prior session restores via
 * `SignIn`; nothing to restore ⇒ no-op (`[]`, never a sentinel) — mirrors `ClearDefaultApp`'s
 * inverse precedent (`🧬️mutations/🧹clear-default-app/↩️inverse/🦀️component.rs`). */

import type { Identity, IdentityConfigMutation } from "../../🪪️sign-in/🟦️component.ts";
import { signIn } from "../../🪪️sign-in/🦠️mutation/🟦️component.ts";

//#region 🔖️Inverse
export function inverse(base: Identity | null): IdentityConfigMutation[] {
  return base ? [signIn(base)] : [];
}
//#endregion 🔖️Inverse
