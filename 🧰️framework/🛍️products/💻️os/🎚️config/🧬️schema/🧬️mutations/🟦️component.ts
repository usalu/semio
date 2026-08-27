/** 🧬️ Transparent TypeScript dispatch for every OS config mutation vocabulary. */

import type { OpeningPreferences } from "../🟦️component.ts";
import type { SetDefaultApp } from "./📌️set-default-app/🟦️component.ts";
import { diff as setDefaultAppDiff, inverse as setDefaultAppInverse, setDefaultApp } from "./📌️set-default-app/🟦️component.ts";
import type { ClearDefaultApp } from "./🧹clear-default-app/🟦️component.ts";
import { clearDefaultApp, diff as clearDefaultAppDiff, inverse as clearDefaultAppInverse } from "./🧹clear-default-app/🟦️component.ts";
import type { ChangeMergePolicy, MergePolicySetting } from "./🛡️change-merge-policy/🟦️component.ts";
import { changeMergePolicy, diff as changeMergePolicyDiff, inverse as changeMergePolicyInverse } from "./🛡️change-merge-policy/🟦️component.ts";
import type { Identity, SignIn } from "./🪪️sign-in/🟦️component.ts";
import { diff as signInDiff, inverse as signInInverse, signIn } from "./🪪️sign-in/🟦️component.ts";
import type { SignOut } from "./🚪️sign-out/🟦️component.ts";
import { diff as signOutDiff, inverse as signOutInverse, signOut } from "./🚪️sign-out/🟦️component.ts";

//#region 🔖️Opening
/** 🎚️ Typed, invertible opening-preferences mutation vocabulary. */
export type OpeningConfigMutation = ({ readonly mutation: "setDefaultApp" } & SetDefaultApp) | ({ readonly mutation: "clearDefaultApp" } & ClearDefaultApp);

/** 🧮️ Delegates opening-preference behavior to the direct semantic leaf named by the tag. */
export function applyOpeningConfigMutation(base: OpeningPreferences, mutation: OpeningConfigMutation): OpeningPreferences {
  return mutation.mutation === "setDefaultApp" ? setDefaultAppDiff(mutation, base) : clearDefaultAppDiff(mutation, base);
}

/** ↩️ Delegates opening-preference inverse behavior to the direct semantic leaf named by the tag. */
export function inverseOpeningConfigMutation(mutation: OpeningConfigMutation, base: OpeningPreferences): OpeningConfigMutation[] {
  return mutation.mutation === "setDefaultApp" ? setDefaultAppInverse(mutation, base) : clearDefaultAppInverse(mutation, base);
}
//#endregion 🔖️Opening

//#region 🔖️MergePolicy
/** 🛡️ Typed, invertible merge-policy mutation vocabulary. */
export type MergePolicyConfigMutation = { readonly mutation: "changeMergePolicy" } & ChangeMergePolicy;

/** 🧮️ Delegates merge-policy behavior to its direct semantic leaf. */
export function applyMergePolicyConfigMutation(base: MergePolicySetting, mutation: MergePolicyConfigMutation): MergePolicySetting {
  return changeMergePolicyDiff(mutation, base);
}

/** ↩️ Delegates merge-policy inverse behavior to its direct semantic leaf. */
export function inverseMergePolicyConfigMutation(mutation: MergePolicyConfigMutation, base: MergePolicySetting): MergePolicyConfigMutation[] {
  return changeMergePolicyInverse(mutation, base);
}
//#endregion 🔖️MergePolicy

//#region 🔖️Identity
/** 🪪️ Typed, invertible identity mutation vocabulary. */
export type IdentityConfigMutation = ({ readonly mutation: "signIn" } & SignIn) | ({ readonly mutation: "signOut" } & SignOut);

/** 🧮️ Delegates diff behavior to the direct semantic leaf named by the tag. */
export function diffIdentityConfigMutation(mutation: IdentityConfigMutation, base: Identity | null): Identity | null {
  return mutation.mutation === "signIn" ? signInDiff(mutation, base) : signOutDiff(base);
}

/** ↩️ Delegates inverse behavior to the direct semantic leaf named by the tag. */
export function inverseIdentityConfigMutation(mutation: IdentityConfigMutation, base: Identity | null): IdentityConfigMutation[] {
  return mutation.mutation === "signIn" ? signInInverse(mutation, base) : signOutInverse(base);
}

/** 🧮️ Applies an identity mutation's whole-record diff. */
export function applyIdentityConfigMutation(base: Identity | null, mutation: IdentityConfigMutation): Identity | null {
  return diffIdentityConfigMutation(mutation, base);
}

export { changeMergePolicy, clearDefaultApp, setDefaultApp, signIn, signOut };
export type { ChangeMergePolicy, ClearDefaultApp, Identity, MergePolicySetting, SetDefaultApp, SignIn, SignOut };
//#endregion 🔖️Identity
