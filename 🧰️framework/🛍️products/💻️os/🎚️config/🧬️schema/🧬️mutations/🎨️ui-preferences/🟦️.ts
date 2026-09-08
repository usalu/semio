/** 🎨️ Direct TypeScript mutation leaves for persisted local OS UI preferences. */

import type { UiAppearance, UiChromeLayout, UiDriver, UiLocale, UiPreferences, UiTheme } from "../../🟦️.ts";
import type { UiPreferencesConfigMutation } from "../🟦️.ts";

export interface SetAppearance { readonly appearance: UiAppearance | null }
export interface SetLayout { readonly layout: UiChromeLayout | null }
export interface SetDriver { readonly driverId: string | null }
export interface SetCustomDriver { readonly driverId: string; readonly driver: UiDriver | null }
export interface SetLocale { readonly locale: UiLocale | null }
export interface SetTerminology { readonly terminology: string | null }
export interface SetTheme { readonly themeId: string | null }
export interface SetCustomTheme { readonly themeId: string; readonly theme: UiTheme | null }
export interface SetKeybindingOverride { readonly controlId: string; readonly keys: string | null }

export const setAppearance = (appearance: UiAppearance | null): UiPreferencesConfigMutation => ({ mutation: "setAppearance", appearance });
export const setLayout = (layout: UiChromeLayout | null): UiPreferencesConfigMutation => ({ mutation: "setLayout", layout });
export const setDriver = (driverId: string | null): UiPreferencesConfigMutation => ({ mutation: "setDriver", driverId });
export const setCustomDriver = (driverId: string, driver: UiDriver | null): UiPreferencesConfigMutation => ({ mutation: "setCustomDriver", driverId, driver });
export const setLocale = (locale: UiLocale | null): UiPreferencesConfigMutation => ({ mutation: "setLocale", locale });
export const setTerminology = (terminology: string | null): UiPreferencesConfigMutation => ({ mutation: "setTerminology", terminology });
export const setTheme = (themeId: string | null): UiPreferencesConfigMutation => ({ mutation: "setTheme", themeId });
export const setCustomTheme = (themeId: string, theme: UiTheme | null): UiPreferencesConfigMutation => ({ mutation: "setCustomTheme", themeId, theme });
export const setKeybindingOverride = (controlId: string, keys: string | null): UiPreferencesConfigMutation => ({ mutation: "setKeybindingOverride", controlId, keys });

export function diff(mutation: UiPreferencesConfigMutation, base: UiPreferences): UiPreferences {
  switch (mutation.mutation) {
    case "setAppearance": return { ...base, appearance: mutation.appearance };
    case "setLayout": return { ...base, layout: mutation.layout };
    case "setDriver": return { ...base, driverId: mutation.driverId };
    case "setCustomDriver": {
      const customDrivers = { ...base.customDrivers };
      if (mutation.driver === null) delete customDrivers[mutation.driverId];
      else customDrivers[mutation.driverId] = mutation.driver;
      return { ...base, customDrivers };
    }
    case "setLocale": return { ...base, locale: mutation.locale };
    case "setTerminology": return { ...base, terminology: mutation.terminology };
    case "setTheme": return { ...base, themeId: mutation.themeId };
    case "setCustomTheme": {
      const customThemes = { ...base.customThemes };
      if (mutation.theme === null) delete customThemes[mutation.themeId];
      else customThemes[mutation.themeId] = mutation.theme;
      return { ...base, customThemes };
    }
    case "setKeybindingOverride": {
      const keybindingOverrides = { ...base.keybindingOverrides };
      if (mutation.keys === null) delete keybindingOverrides[mutation.controlId];
      else keybindingOverrides[mutation.controlId] = mutation.keys;
      return { ...base, keybindingOverrides };
    }
  }
}

export function inverse(mutation: UiPreferencesConfigMutation, base: UiPreferences): UiPreferencesConfigMutation[] {
  switch (mutation.mutation) {
    case "setAppearance": return [setAppearance(base.appearance)];
    case "setLayout": return [setLayout(base.layout)];
    case "setDriver": return [setDriver(base.driverId)];
    case "setCustomDriver": return [setCustomDriver(mutation.driverId, base.customDrivers[mutation.driverId] ?? null)];
    case "setLocale": return [setLocale(base.locale)];
    case "setTerminology": return [setTerminology(base.terminology)];
    case "setTheme": return [setTheme(base.themeId)];
    case "setCustomTheme": return [setCustomTheme(mutation.themeId, base.customThemes[mutation.themeId] ?? null)];
    case "setKeybindingOverride": return [setKeybindingOverride(mutation.controlId, base.keybindingOverrides[mutation.controlId] ?? null)];
  }
}
