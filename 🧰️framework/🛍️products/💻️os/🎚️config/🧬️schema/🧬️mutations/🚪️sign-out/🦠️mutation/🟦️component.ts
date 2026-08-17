/** 🚪️ Identity mutation — `SignOut` payload: clears the OS-wide signed-in session. Diff/inverse
 * delegate to the sibling `🔺️diff`/`↩️inverse` leaves. */

import type { IdentityConfigMutation } from "../../🪪️sign-in/🟦️component.ts";

//#region 🔖️Mutation
/** 🚪️ No-payload mutation — kept as a named type (not `Record<string, never>`) so a future field
 * lands as an additive shape change. */
export type SignOut = Record<string, never>;

/** 🏗️ Builder — wraps the (empty) payload in its dispatch variant. */
export function signOut(): IdentityConfigMutation {
  return { mutation: "signOut" };
}
//#endregion 🔖️Mutation
