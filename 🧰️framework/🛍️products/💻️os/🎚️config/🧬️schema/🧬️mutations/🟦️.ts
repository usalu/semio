/** 🗂️ The UI-preferences mutation schema authority that only the product can read: which preference key each mutation writes (the mutation schema `$defs.PreferenceKeys`) and the data class that key follows. The mutation vocabulary and its dispatch live with the neutral config mutation module. */
import uiPreferencesMutationSchema from "./🎨️ui-preferences/🧬️schema/🔣️.json" with { type: "json" };
import type { UiPreferenceDataClass, UiPreferenceKey } from "@semio-tech/framework/config/schema";
import type { UiPreferencesConfigMutation } from "@semio-tech/framework/config/schema/mutations/ui-preferences";
import { UI_PREFERENCE_DATA_CLASSES } from "../🟦️.ts";

//#region 🔖️UiPreferences
/** 🗂️ The one key each UiPreferences mutation writes, read from the mutation schema (`🎨️ui-preferences/🧬️schema/🔣️.json`
 * `$defs.PreferenceKeys`). */
export const UI_PREFERENCE_MUTATION_KEYS: Readonly<Record<UiPreferencesConfigMutation["mutation"], UiPreferenceKey>> = uiPreferencesMutationSchema.$defs.PreferenceKeys.const as Readonly<Record<UiPreferencesConfigMutation["mutation"], UiPreferenceKey>>;

/** 🗂️ Whether a mutation follows the user to every device (`persistedShared`) or stays on this one — its key's data class. */
export function uiPreferenceMutationDataClassV1(mutation: UiPreferencesConfigMutation): UiPreferenceDataClass {
  return UI_PREFERENCE_DATA_CLASSES[UI_PREFERENCE_MUTATION_KEYS[mutation.mutation]];
}
//#endregion 🔖️UiPreferences
