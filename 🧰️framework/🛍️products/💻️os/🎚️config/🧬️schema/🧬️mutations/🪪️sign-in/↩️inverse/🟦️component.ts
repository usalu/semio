/** ↩️ Inverse for `SignIn` — reads the BASE identity, never the diff: a prior session (a "switch
 * account" sign-in) restores via `SignIn`; no prior session restores via `SignOut` — mirrors
 * `SetDefaultApp`'s inverse precedent (`🧬️mutations/📌️set-default-app/↩️inverse/🦀️component.rs`). */

import type { SignIn } from "../🦠️mutation/🟦️component.ts";
import { signIn } from "../🦠️mutation/🟦️component.ts";
import type { Identity, IdentityConfigMutation } from "../🟦️component.ts";
import { signOut } from "../../🚪️sign-out/🦠️mutation/🟦️component.ts";

//#region 🔖️Inverse
export function inverse(_payload: SignIn, base: Identity | null): IdentityConfigMutation[] {
  return base ? [signIn(base)] : [signOut()];
}
//#endregion 🔖️Inverse
