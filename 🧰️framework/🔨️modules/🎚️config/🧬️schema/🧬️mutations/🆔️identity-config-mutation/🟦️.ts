/** 🆔️ Identity record, payloads, dispatch union and constructors of the identity mutation vocabulary, shared by its two direct leaves. */

//#region 🔖️Schema
/** 🪪️ `os.config.identity` — the OS-wide signed-in session. */
export interface Identity {
  readonly userId: string;
  readonly email: string;
  readonly displayName: string;
  readonly hubBaseUrl: string;
  readonly issuedAtMs: number;
}
//#endregion 🔖️Schema

//#region 🔖️Payloads
/** 🔑️ Establishes or replaces the OS-wide signed-in session. */
export type SignIn = Identity;

/** 🚪️ Empty payload for clearing the OS-wide signed-in session. */
export type SignOut = Record<never, never>;
//#endregion 🔖️Payloads

//#region 🔖️Dispatch
/** 🎚️ Typed, invertible identity mutation vocabulary. */
export type IdentityConfigMutation = ({ readonly mutation: "signIn" } & SignIn) | ({ readonly mutation: "signOut" } & SignOut);

/** 🏗️ Wraps a sign-in payload in the identity dispatch union. */
export function signIn(payload: SignIn): IdentityConfigMutation {
  return { mutation: "signIn", ...payload };
}

/** 🔨️ Wraps the empty sign-out payload in the identity dispatch union. */
export function signOut(): IdentityConfigMutation {
  return { mutation: "signOut" };
}
//#endregion 🔖️Dispatch


export interface IdentitySessionEdit {
  readonly value?: Identity;
}


export interface IdentityDiff {
  readonly session?: IdentitySessionEdit;
}
