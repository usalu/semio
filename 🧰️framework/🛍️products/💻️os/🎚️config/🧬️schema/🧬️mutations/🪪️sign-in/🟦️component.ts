/** 🪪️ OS-level identity config facet — `os.config.identity`: the signed-in user's session,
 * event-sourced over `sign-in`/`sign-out` (contract-freeze §C3 of ticket 26/08/16/HUB-SPACES-LIVE-
 * PRESENCE-AND-COLLABORATIVE-STUDIOS). Schema + dispatch enum folded into THIS file (rather than
 * split across a parent `🧬️schema`/`🧬️mutations` pair the way `OpeningPreferences` is) because this
 * triad's lease is scoped to `🧬️mutations/🪪️sign-in/**`/`🚪️sign-out/**` only — the sibling
 * `📌️set-default-app`/`🧹clear-default-app` triads and their shared parent files
 * (`../../🔣️component.json`/`🦀️component.rs`/`🟦️component.ts`, `../🦀️component.rs`) belong to the
 * concurrent `ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET` ticket and are never touched here (see
 * `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS/
 * 📋️ownership-and-handoffs.md` §A). Matches the `🛡️change-merge-policy` precedent — same
 * self-containment, same reason (`../🛡️change-merge-policy/🦀️component.rs`'s header doc).
 *
 * Persisted through the FOLDER lane (`S_DATA_DIR/os`), never the hub — unlike `OpeningPreferences`'s
 * `bindings: []`, so a reload keeps the session token. See `identityActorConfig` in
 * `../../../../../🟦️backbone-worker.ts`'s `🔖️ConfigLane` region. */

import type { SignIn } from "./🦠️mutation/🟦️component.ts";
import { signIn } from "./🦠️mutation/🟦️component.ts";
import { diff as signInDiff } from "./🔺️diff/🟦️component.ts";
import { inverse as signInInverse } from "./↩️inverse/🟦️component.ts";
import type { SignOut } from "../🚪️sign-out/🦠️mutation/🟦️component.ts";
import { signOut } from "../🚪️sign-out/🦠️mutation/🟦️component.ts";
import { diff as signOutDiff } from "../🚪️sign-out/🔺️diff/🟦️component.ts";
import { inverse as signOutInverse } from "../🚪️sign-out/↩️inverse/🟦️component.ts";

//#region 🔖️Schema
/** 🪪️ `os.config.identity` — the OS-wide signed-in session, or `null` when signed out. */
export interface Identity {
  readonly userId: string;
  readonly email: string;
  readonly displayName: string;
  readonly hubBaseUrl: string;
  readonly sessionToken: string;
  readonly issuedAtMs: number;
}

/** 🪪️ The schema id this facet is registered under. */
export const IDENTITY_CONFIG_SCHEMA = "os.config.identity";
//#endregion 🔖️Schema

//#region 🔖️Mutations
/** @emoji 🪪️ Typed, invertible identity mutation vocabulary. */
export type IdentityConfigMutation = ({ readonly mutation: "signIn" } & SignIn) | ({ readonly mutation: "signOut" } & SignOut);

/** 🧮️ Real handcrafted diff, delegating per variant to the sibling triads' `🔺️diff` leaves — both
 * already return the full post-op value (`Identity` or `null`), matching `OpeningPreferences`'s
 * whole-record precedent, so `apply` below never merges. */
export function diffIdentityConfigMutation(mutation: IdentityConfigMutation, base: Identity | null): Identity | null {
  return mutation.mutation === "signIn" ? signInDiff(mutation, base) : signOutDiff(base);
}

export function inverseIdentityConfigMutation(mutation: IdentityConfigMutation, base: Identity | null): IdentityConfigMutation[] {
  return mutation.mutation === "signIn" ? signInInverse(mutation, base) : signOutInverse(base);
}

/** 🧮️ Diff-first apply — `diff(mutation, base)`, matching `apply_opening_config_mutation`'s
 * precedent (`🧬️mutations/🦀️component.rs`). */
export function applyIdentityConfigMutation(base: Identity | null, mutation: IdentityConfigMutation): Identity | null {
  return diffIdentityConfigMutation(mutation, base);
}

export { signIn, signOut };
export type { SignIn, SignOut };
//#endregion 🔖️Mutations
