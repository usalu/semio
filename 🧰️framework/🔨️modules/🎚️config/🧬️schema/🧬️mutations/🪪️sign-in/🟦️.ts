/** 🪪️ Authoritative direct TypeScript leaf for establishing an OS identity session. */

import type { Identity, IdentityConfigMutation, SignIn } from "../🆔️identity-config-mutation/🟦️.ts";
import { signIn, signOut } from "../🆔️identity-config-mutation/🟦️.ts";

//#region 🔖️Schema
/** 🪪️ The schema id for the identity config facet. */
export const IDENTITY_CONFIG_SCHEMA = "os.config.identity";
//#endregion 🔖️Schema

//#region 🔖️Mutation
/** 🔺️ Constructs the whole-record post-sign-in identity. */
export function diff(payload: SignIn, _base: Identity | null): Identity {
  return {
    userId: payload.userId,
    email: payload.email,
    displayName: payload.displayName,
    hubBaseUrl: payload.hubBaseUrl,
    issuedAtMs: payload.issuedAtMs,
  };
}

/** ↩️ Restores the prior identity, or signs out when there was none. */
export function inverse(_payload: SignIn, base: Identity | null): IdentityConfigMutation[] {
  return base ? [signIn(base)] : [signOut()];
}
//#endregion 🔖️Mutation
