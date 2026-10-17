/** 🎚️ Runs the canonical UI preference and preference-lane suites against the renderer's UiPreferences module, wiring the product's mutation vocabulary the way a composition root does; the module holds no test hook. */
import { dirname, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import * as vitest from "vitest";
import { createMemoryStoragePort } from "@semio-tech/framework";
import { setTheme } from "@semio-tech/framework/config/mutations";
import { UI_PREFERENCE_MUTATION_KEYS, uiPreferenceMutationDataClassV1 } from "../../../../../🎚️config/🧬️schema/🧬️mutations/🟦️.ts";
import * as preferences from "../../🧱️elements/🎚️UiPreferences/🟦️.ts";
import { registerTests1 as registerCanonicalTests } from "../../🎚️UiPreferences/🧪️tests/🎚️canonical-os-ui-preferences/🟦️.ts";
import { registerTests1 as registerLaneTests } from "../../🎚️UiPreferences/🧪️tests/🌐️preference-lane/🟦️.ts";

const vocabulary: preferences.UiPreferenceVocabularyV1 = {
  knows: (mutationName) => mutationName in UI_PREFERENCE_MUTATION_KEYS,
  keyOf: (mutation) => UI_PREFERENCE_MUTATION_KEYS[mutation.mutation],
  dataClassOf: uiPreferenceMutationDataClassV1,
};
const source = { url: pathToFileURL(resolve(dirname(fileURLToPath(import.meta.url)), "../../🧱️elements/🎚️UiPreferences/🟦️.ts")).href };

await registerCanonicalTests(vitest, preferences, source);
await registerLaneTests(
  vitest,
  {
    ...preferences,
    commitUiPreferenceOnLaneV1: (storage: Parameters<typeof preferences.commitUiPreferenceOnLaneV1>[1], mutation: Parameters<typeof preferences.commitUiPreferenceOnLaneV1>[2], lane: string | null, mintId: () => string) => preferences.commitUiPreferenceOnLaneV1(vocabulary, storage, mutation, lane, mintId),
    foldUiPreferenceLanePageV1: (storage: Parameters<typeof preferences.foldUiPreferenceLanePageV1>[1], lane: string, page: Parameters<typeof preferences.foldUiPreferenceLanePageV1>[3]) => preferences.foldUiPreferenceLanePageV1(vocabulary, storage, lane, page),
    parseUiPreferenceEnvelopeV1: (schema: string, text: string) => preferences.parseUiPreferenceEnvelopeV1(vocabulary, schema, text),
  },
  source,
);

vitest.describe("preference vocabulary port", () => {
  vitest.it("keeps every preference on this device when the host was given no vocabulary", () => {
    const device = createMemoryStoragePort();
    const mutation = setTheme("night");
    const outcome = preferences.commitUiPreferenceOnLaneV1(preferences.LOCAL_ONLY_UI_PREFERENCE_VOCABULARY_V1, device, mutation, "https://hub.test#u1", () => "0".repeat(32));
    vitest.expect(outcome.queued).toBeNull();
    vitest.expect(outcome.unshared).toBe(false);
    vitest.expect(preferences.readUiPreferences(device).themeId).toBe("night");
    vitest.expect(preferences.readUiPreferenceLane(device, "https://hub.test#u1").outbox).toEqual([]);
  });

  vitest.it("queues a shared change on its lane only when the vocabulary classifies it shared", () => {
    const device = createMemoryStoragePort();
    const lane = "https://hub.test#u1";
    const queued = preferences.commitUiPreferenceOnLaneV1(vocabulary, device, setTheme("night"), lane, (() => { let n = 0; return () => String(++n).padStart(32, "0"); })());
    vitest.expect(vocabulary.dataClassOf(setTheme("night"))).toBe("persistedShared");
    vitest.expect(queued.queued?.mutation).toEqual(setTheme("night"));
    vitest.expect(preferences.readUiPreferenceLane(device, lane).outbox).toHaveLength(1);
  });
});
