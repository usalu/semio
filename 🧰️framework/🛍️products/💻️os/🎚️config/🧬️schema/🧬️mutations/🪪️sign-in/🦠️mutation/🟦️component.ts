/** 🪪️ Identity mutation — `SignIn` payload: establishes (or replaces) the OS-wide signed-in
 * session. Diff/inverse delegate to the sibling `🔺️diff`/`↩️inverse` leaves. */

import type { IdentityConfigMutation } from "../🟦️component.ts";

//#region 🔖️Mutation
export interface SignIn {
  readonly userId: string;
  readonly email: string;
  readonly displayName: string;
  readonly hubBaseUrl: string;
  readonly sessionToken: string;
  readonly issuedAtMs: number;
}

/** 🏗️ Builder — wraps the payload in its dispatch variant. */
export function signIn(payload: SignIn): IdentityConfigMutation {
  return { mutation: "signIn", ...payload };
}
//#endregion 🔖️Mutation
