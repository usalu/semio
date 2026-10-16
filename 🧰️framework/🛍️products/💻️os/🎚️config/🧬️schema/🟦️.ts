/** 🎨️ The OS config schema documents that only the product can read: the UI-preferences JSON schema and the data class of every preference key, which the TypeScript and Rust twins both read from it. The key and value types live with the neutral config schema module. */
import uiPreferencesSchema from "./🎨️ui-preferences/🔣️.json" with { type: "json" };
import type { UiPreferenceDataClass, UiPreferenceKey } from "@semio-tech/framework/config/schema";

export const osUiPreferencesSchemaDocument = uiPreferencesSchema;

/** 🗂️ The data class of every key, read from the schema itself (`🎨️ui-preferences/🔣️.json` `$defs.DataClasses`) — the one
 * declaration the TypeScript and Rust twins both read. Rows: `🎨️ui-preferences/🧫️fixtures/🗂️data-classes/🔣️.json`. */
export const UI_PREFERENCE_DATA_CLASSES: Readonly<Record<UiPreferenceKey, UiPreferenceDataClass>> = uiPreferencesSchema.$defs.DataClasses.const as Readonly<Record<UiPreferenceKey, UiPreferenceDataClass>>;
