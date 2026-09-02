/** 🚪️ Authoritative direct TypeScript leaf for clearing the OS identity session. */

import type { IdentityConfigMutation } from "../🟦️.ts";
import type { Identity } from "../🪪️sign-in/🟦️.ts";
import { signIn } from "../🪪️sign-in/🟦️.ts";

//#region 🔖️Mutation
/** 🚪️ Empty payload for clearing the OS-wide signed-in session. */
export type SignOut = Record<never, never>;

/** 🏗️ Wraps the empty sign-out payload in the identity dispatch union. */
export function signOut(): IdentityConfigMutation {
  return { mutation: "signOut" };
}

/** 🔺️ Constructs the whole-record signed-out state. */
export function diff(_base: Identity | null): null {
  return null;
}

/** ↩️ Restores the prior session, or emits no step when already signed out. */
export function inverse(base: Identity | null): IdentityConfigMutation[] {
  return base ? [signIn(base)] : [];
}
//#endregion 🔖️Mutation
