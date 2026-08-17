/** 🔺️ Diff fragment yielded by `SignIn` — real handcrafted construction, never apply-then-capture:
 * replaces any existing session wholesale (whole-record diff, matching `OpeningPreferences`'s
 * precedent — `os.config.opening`'s `🔺️diff` leaves, `🎚️config/🧬️schema/🧬️mutations/🦀️component.rs`
 * doc). `base` is unused — a sign-in never merges into a prior session. */

import type { SignIn } from "../🦠️mutation/🟦️component.ts";
import type { Identity } from "../🟦️component.ts";

//#region 🔖️Diff
export function diff(payload: SignIn, _base: Identity | null): Identity {
  return {
    userId: payload.userId,
    email: payload.email,
    displayName: payload.displayName,
    hubBaseUrl: payload.hubBaseUrl,
    sessionToken: payload.sessionToken,
    issuedAtMs: payload.issuedAtMs,
  };
}
//#endregion 🔖️Diff
