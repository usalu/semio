/** 🧬️ Transparent TypeScript dispatch for every OS config mutation vocabulary. */

import type { OpeningPreferences } from "../🟦️.ts";
import type { ClearDefaultApp, OpeningConfigMutation, SetDefaultApp } from "./📂️opening-config-mutation/🟦️.ts";
import { clearDefaultApp, setDefaultApp } from "./📂️opening-config-mutation/🟦️.ts";
import { diff as setDefaultAppDiff, inverse as setDefaultAppInverse } from "./📌️set-default-app/🟦️.ts";
import { diff as clearDefaultAppDiff, inverse as clearDefaultAppInverse } from "./🧹clear-default-app/🟦️.ts";
import type { ChangeMergePolicy, MergePolicyConfigMutation, MergePolicySetting } from "./🛡️change-merge-policy/🟦️.ts";
import { changeMergePolicy, diff as changeMergePolicyDiff, inverse as changeMergePolicyInverse } from "./🛡️change-merge-policy/🟦️.ts";
import type { AdmitLocalDocument, LocalCatalogConfigMutation, LocalDocument, LocalDocumentStorage, RetireLocalDocument } from "./🗃️local-catalog-config-mutation/🟦️.ts";
import { admitLocalDocument, retireLocalDocument } from "./🗃️local-catalog-config-mutation/🟦️.ts";
import type { LocalCatalog } from "./📥️admit-local-document/🟦️.ts";
import { diff as admitLocalDocumentDiff, inverse as admitLocalDocumentInverse, LOCAL_CATALOG_CONFIG_SCHEMA } from "./📥️admit-local-document/🟦️.ts";
import { diff as retireLocalDocumentDiff, inverse as retireLocalDocumentInverse } from "./📤️retire-local-document/🟦️.ts";
import type { AttachLocalFolder, DetachLocalFolder, LocalFolderBinding, LocalFolderRef, LocalFoldersConfigMutation } from "./📁️local-folders-config-mutation/🟦️.ts";
import { attachLocalFolder, detachLocalFolder } from "./📁️local-folders-config-mutation/🟦️.ts";
import type { LocalFolderBindings } from "./📎️attach-local-folder/🟦️.ts";
import { diff as attachLocalFolderDiff, inverse as attachLocalFolderInverse, LOCAL_FOLDERS_CONFIG_SCHEMA } from "./📎️attach-local-folder/🟦️.ts";
import { diff as detachLocalFolderDiff, inverse as detachLocalFolderInverse } from "./✂️detach-local-folder/🟦️.ts";
import type { Identity, IdentityConfigMutation, SignIn, SignOut } from "./🆔️identity-config-mutation/🟦️.ts";
import { signIn, signOut } from "./🆔️identity-config-mutation/🟦️.ts";
import { diff as signInDiff, inverse as signInInverse } from "./🪪️sign-in/🟦️.ts";
import { diff as signOutDiff, inverse as signOutInverse } from "./🚪️sign-out/🟦️.ts";
import { type UiPreferences } from "../🟦️.ts";
import type { SetAppearance, SetCustomDriver, SetCustomTheme, SetDriver, SetKeybindingOverride, SetLayout, SetLocale, SetNamedLayout, SetTerminology, SetTheme, UiPreferencesConfigMutation } from "./🎨️ui-preferences/🟦️.ts";
import { diff as uiPreferencesDiff, inverse as uiPreferencesInverse, setAppearance, setCustomDriver, setCustomTheme, setDriver, setKeybindingOverride, setLayout, setLocale, setNamedLayout, setTerminology, setTheme } from "./🎨️ui-preferences/🟦️.ts";

//#region 🔖️Opening
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
export function applyUiPreferencesConfigMutation(base: UiPreferences, mutation: UiPreferencesConfigMutation): UiPreferences {
  return uiPreferencesDiff(mutation, base);
}

export function inverseUiPreferencesConfigMutation(mutation: UiPreferencesConfigMutation, base: UiPreferences): UiPreferencesConfigMutation[] {
  return uiPreferencesInverse(mutation, base);
}
//#endregion 🔖️UiPreferences

//#region 🔖️MergePolicy
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

export { changeMergePolicy, clearDefaultApp, setAppearance, setCustomDriver, setCustomTheme, setDefaultApp, setDriver, setKeybindingOverride, setLayout, setLocale, setNamedLayout, setTerminology, setTheme, signIn, signOut };
export type { ChangeMergePolicy, ClearDefaultApp, Identity, IdentityConfigMutation, MergePolicyConfigMutation, MergePolicySetting, OpeningConfigMutation, SetAppearance, SetCustomDriver, SetCustomTheme, SetDefaultApp, SetDriver, SetKeybindingOverride, SetLayout, SetLocale, SetNamedLayout, SetTerminology, SetTheme, SignIn, SignOut, UiPreferencesConfigMutation };
//#endregion 🔖️Identity

//#region 🔖️LocalCatalog
/** 🧮️ Delegates local-catalog behavior to the direct semantic leaf named by the tag. */
export function applyLocalCatalogConfigMutation(base: LocalCatalog, mutation: LocalCatalogConfigMutation): LocalCatalog {
  return mutation.mutation === "admitLocalDocument" ? admitLocalDocumentDiff(mutation, base) : retireLocalDocumentDiff(mutation, base);
}

/** ↩️ Delegates local-catalog inverse behavior to the direct semantic leaf named by the tag. */
export function inverseLocalCatalogConfigMutation(mutation: LocalCatalogConfigMutation, base: LocalCatalog): LocalCatalogConfigMutation[] {
  return mutation.mutation === "admitLocalDocument" ? admitLocalDocumentInverse(mutation, base) : retireLocalDocumentInverse(mutation, base);
}

export { admitLocalDocument, LOCAL_CATALOG_CONFIG_SCHEMA, retireLocalDocument };
export type { AdmitLocalDocument, LocalCatalog, LocalCatalogConfigMutation, LocalDocument, LocalDocumentStorage, RetireLocalDocument };
//#endregion 🔖️LocalCatalog

//#region 🔖️LocalFolders
/** 🧮️ Delegates local-folder behavior to the direct semantic leaf named by the tag. */
export function applyLocalFoldersConfigMutation(base: LocalFolderBindings, mutation: LocalFoldersConfigMutation): LocalFolderBindings {
  return mutation.mutation === "attachLocalFolder" ? attachLocalFolderDiff(mutation, base) : detachLocalFolderDiff(mutation, base);
}

/** ↩️ Delegates local-folder inverse behavior to the direct semantic leaf named by the tag. */
export function inverseLocalFoldersConfigMutation(mutation: LocalFoldersConfigMutation, base: LocalFolderBindings): LocalFoldersConfigMutation[] {
  return mutation.mutation === "attachLocalFolder" ? attachLocalFolderInverse(mutation, base) : detachLocalFolderInverse(mutation, base);
}

export { attachLocalFolder, detachLocalFolder, LOCAL_FOLDERS_CONFIG_SCHEMA };
export type { AttachLocalFolder, DetachLocalFolder, LocalFolderBinding, LocalFolderBindings, LocalFolderRef, LocalFoldersConfigMutation };
//#endregion 🔖️LocalFolders
