import type { ArtifactDialect } from "../../../../🔨️modules/🚪️io/🧬️schema/🟦️.ts";
import type { AppRole, AppRef } from "../../../../🔨️modules/🛂️manifest/🧬️schema/🟦️.ts";
/** 🧬️ Canonical OS configuration schemas. */

import uiPreferencesSchema from "./🎨️ui-preferences/🔣️.json" with { type: "json" };

export type UiAppearance = "system" | "light" | "dark";
export type UiChromeLayout = "desktop" | "tablet";
export type UiLocale = "en" | "de";

export interface UiDriver {
  driverId: string;
  label: string;
  config: unknown;
}

export interface UiTheme {
  themeId: string;
  label: string;
  config: unknown;
}

export interface UiPreferences {
  appearance: UiAppearance | null;
  layout: UiChromeLayout | null;
  driverId: string | null;
  customDrivers: Record<string, UiDriver>;
  locale: UiLocale | null;
  terminology: string | null;
  themeId: string | null;
  customThemes: Record<string, UiTheme>;
  keybindingOverrides: Record<string, string>;
}

export const OS_UI_PREFERENCES_SCHEMA_ID = "https://semio.tech/schema/os/config/ui-preferences.json";
export const osUiPreferencesSchemaDocument = uiPreferencesSchema;

export class OsConfigSchemaError extends Error {
  constructor(readonly exportId: string, readonly problems: readonly string[]) {
    super(`${exportId} does not satisfy ${OS_UI_PREFERENCES_SCHEMA_ID}: ${problems.join("; ")}`);
    this.name = "OsConfigSchemaError";
  }
}

const isRecord = (value: unknown): value is Record<string, unknown> => typeof value === "object" && value !== null && !Array.isArray(value);
const owns = (value: Record<string, unknown>, key: string): boolean => Object.prototype.hasOwnProperty.call(value, key);

function parseEnum<T extends string>(exportId: string, value: unknown, values: readonly T[]): T {
  if (typeof value === "string" && values.includes(value as T)) return value as T;
  throw new OsConfigSchemaError(exportId, [`value must be one of ${JSON.stringify(values)}`]);
}

function parseRecord<T>(exportId: string, value: unknown, parseValue: (entry: unknown) => T): Record<string, T> {
  if (!isRecord(value)) throw new OsConfigSchemaError(exportId, ["value must be an object"]);
  const result: Record<string, T> = {};
  for (const [key, entry] of Object.entries(value)) result[key] = parseValue(entry);
  return result;
}

function parseUiExtension<T extends "driverId" | "themeId">(exportId: string, id: T, value: unknown): Record<T, string> & { label: string; config: unknown } {
  if (!isRecord(value)) throw new OsConfigSchemaError(exportId, ["value must be an object"]);
  const allowed = new Set([id, "label", "config"]);
  const problems = Object.keys(value).filter((key) => !allowed.has(key)).map((key) => `value has unexpected property '${key}'`);
  if (typeof value[id] !== "string") problems.push(`value.${id} must be a string`);
  if (typeof value.label !== "string") problems.push("value.label must be a string");
  if (!owns(value, "config")) problems.push("value is missing required property 'config'");
  if (problems.length > 0) throw new OsConfigSchemaError(exportId, problems);
  return value as Record<T, string> & { label: string; config: unknown };
}

export const parseUiAppearance = (value: unknown): UiAppearance => parseEnum("UiAppearance", value, ["system", "light", "dark"]);
export const parseUiChromeLayout = (value: unknown): UiChromeLayout => parseEnum("UiChromeLayout", value, ["desktop", "tablet"]);
export const parseUiLocale = (value: unknown): UiLocale => parseEnum("UiLocale", value, ["en", "de"]);
export const parseUiDriver = (value: unknown): UiDriver => parseUiExtension("UiDriver", "driverId", value) as UiDriver;
export const parseUiTheme = (value: unknown): UiTheme => parseUiExtension("UiTheme", "themeId", value) as UiTheme;

export function parseUiPreferences(value: unknown): UiPreferences {
  if (!isRecord(value)) throw new OsConfigSchemaError("UiPreferences", ["value must be an object"]);
  const required = ["appearance", "layout", "driverId", "customDrivers", "locale", "terminology", "themeId", "customThemes", "keybindingOverrides"] as const;
  const problems = Object.keys(value).filter((key) => !required.includes(key as (typeof required)[number])).map((key) => `value has unexpected property '${key}'`);
  for (const key of required) if (!owns(value, key)) problems.push(`value is missing required property '${key}'`);
  const nullableString = (key: "driverId" | "terminology" | "themeId") => {
    if (value[key] !== null && typeof value[key] !== "string") problems.push(`value.${key} must be a string or null`);
  };
  nullableString("driverId");
  nullableString("terminology");
  nullableString("themeId");
  try { if (value.appearance !== null) parseUiAppearance(value.appearance); } catch { problems.push("value.appearance is invalid"); }
  try { if (value.layout !== null) parseUiChromeLayout(value.layout); } catch { problems.push("value.layout is invalid"); }
  try { if (value.locale !== null) parseUiLocale(value.locale); } catch { problems.push("value.locale is invalid"); }
  try { parseRecord("UiPreferences.customDrivers", value.customDrivers, parseUiDriver); } catch { problems.push("value.customDrivers is invalid"); }
  try { parseRecord("UiPreferences.customThemes", value.customThemes, parseUiTheme); } catch { problems.push("value.customThemes is invalid"); }
  try { parseRecord("UiPreferences.keybindingOverrides", value.keybindingOverrides, (entry) => {
    if (typeof entry !== "string") throw new Error();
    return entry;
  }); } catch { problems.push("value.keybindingOverrides is invalid"); }
  if (problems.length > 0) throw new OsConfigSchemaError("UiPreferences", problems);
  return value as unknown as UiPreferences;
}

export interface DefaultApp {
  dialect: ArtifactDialect;
  role: AppRole;
  app: AppRef;
}

export interface OpeningPreferences {
  /** @state config */
  defaults: DefaultApp[];
}
