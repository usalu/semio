/** 🚪️ Authoritative direct TypeScript leaf for clearing the OS identity session. */

import type { Identity, IdentityConfigMutation } from "../🆔️identity-config-mutation/🟦️.ts";
import { signIn } from "../🆔️identity-config-mutation/🟦️.ts";

//#region 🔖️Mutation
/** 🔺️ Constructs the whole-record signed-out state. */
export function diff(_base: Identity | null): null {
  return null;
}

/** ↩️ Restores the prior session, or emits no step when already signed out. */
export function inverse(base: Identity | null): IdentityConfigMutation[] {
  return base ? [signIn(base)] : [];
}
//#endregion 🔖️Mutation
