/** 🧬️ Transparent TypeScript dispatch for every OS config mutation vocabulary. */

import type { OpeningPreferences } from "../🟦️.ts";
import type { SetDefaultApp } from "./📌️set-default-app/🟦️.ts";
import { diff as setDefaultAppDiff, inverse as setDefaultAppInverse, setDefaultApp } from "./📌️set-default-app/🟦️.ts";
import type { ClearDefaultApp } from "./🧹clear-default-app/🟦️.ts";
import { clearDefaultApp, diff as clearDefaultAppDiff, inverse as clearDefaultAppInverse } from "./🧹clear-default-app/🟦️.ts";
import type { ChangeMergePolicy, MergePolicySetting } from "./🛡️change-merge-policy/🟦️.ts";
import { changeMergePolicy, diff as changeMergePolicyDiff, inverse as changeMergePolicyInverse } from "./🛡️change-merge-policy/🟦️.ts";
import type { Identity, SignIn } from "./🪪️sign-in/🟦️.ts";
import { diff as signInDiff, inverse as signInInverse, signIn } from "./🪪️sign-in/🟦️.ts";
import type { SignOut } from "./🚪️sign-out/🟦️.ts";
import { diff as signOutDiff, inverse as signOutInverse, signOut } from "./🚪️sign-out/🟦️.ts";
import type { UiPreferences } from "../🟦️.ts";
import type { SetAppearance, SetCustomDriver, SetCustomTheme, SetDriver, SetKeybindingOverride, SetLayout, SetLocale, SetTerminology, SetTheme } from "./🎨️ui-preferences/🟦️.ts";
import { diff as uiPreferencesDiff, inverse as uiPreferencesInverse, setAppearance, setCustomDriver, setCustomTheme, setDriver, setKeybindingOverride, setLayout, setLocale, setTerminology, setTheme } from "./🎨️ui-preferences/🟦️.ts";

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

//#region 🔖️UiPreferences
/** 🎨️ Typed, invertible OS UI-preferences mutation vocabulary. */
export type UiPreferencesConfigMutation =
  | ({ readonly mutation: "setAppearance" } & SetAppearance)
  | ({ readonly mutation: "setLayout" } & SetLayout)
  | ({ readonly mutation: "setDriver" } & SetDriver)
  | ({ readonly mutation: "setCustomDriver" } & SetCustomDriver)
  | ({ readonly mutation: "setLocale" } & SetLocale)
  | ({ readonly mutation: "setTerminology" } & SetTerminology)
  | ({ readonly mutation: "setTheme" } & SetTheme)
  | ({ readonly mutation: "setCustomTheme" } & SetCustomTheme)
  | ({ readonly mutation: "setKeybindingOverride" } & SetKeybindingOverride);

export function applyUiPreferencesConfigMutation(base: UiPreferences, mutation: UiPreferencesConfigMutation): UiPreferences {
  return uiPreferencesDiff(mutation, base);
}

export function inverseUiPreferencesConfigMutation(mutation: UiPreferencesConfigMutation, base: UiPreferences): UiPreferencesConfigMutation[] {
  return uiPreferencesInverse(mutation, base);
}
//#endregion 🔖️UiPreferences

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

export { changeMergePolicy, clearDefaultApp, setAppearance, setCustomDriver, setCustomTheme, setDefaultApp, setDriver, setKeybindingOverride, setLayout, setLocale, setTerminology, setTheme, signIn, signOut };
export type { ChangeMergePolicy, ClearDefaultApp, Identity, MergePolicySetting, SetAppearance, SetCustomDriver, SetCustomTheme, SetDefaultApp, SetDriver, SetKeybindingOverride, SetLayout, SetLocale, SetTerminology, SetTheme, SignIn, SignOut };
//#endregion 🔖️Identity
