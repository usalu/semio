/** 🪪️ Authoritative direct TypeScript leaf for establishing an OS identity session. */

import type { IdentityConfigMutation } from "../🟦️.ts";
import { signOut } from "../🚪️sign-out/🟦️.ts";

//#region 🔖️Schema
/** 🪪️ `os.config.identity` — the OS-wide signed-in session. */
export interface Identity {
  readonly userId: string;
  readonly email: string;
  readonly displayName: string;
  readonly hubBaseUrl: string;
  readonly issuedAtMs: number;
}

/** 🪪️ The schema id for the identity config facet. */
export const IDENTITY_CONFIG_SCHEMA = "os.config.identity";
//#endregion 🔖️Schema

//#region 🔖️Mutation
/** 🪪️ Establishes or replaces the OS-wide signed-in session. */
export type SignIn = Identity;

/** 🏗️ Wraps a sign-in payload in the identity dispatch union. */
export function signIn(payload: SignIn): IdentityConfigMutation {
  return { mutation: "signIn", ...payload };
}

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
