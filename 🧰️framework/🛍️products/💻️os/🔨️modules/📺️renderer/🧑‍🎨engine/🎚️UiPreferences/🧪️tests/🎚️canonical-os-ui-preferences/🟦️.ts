import Ajv from "ajv";
import { createMemoryStoragePort, createScopedStoragePort } from "@semio-tech/framework";
import type { UiPreferencesConfigMutation } from "../../../../../../🎚️config/🧬️schema/🧬️mutations/🟦️.ts";
import shellSchema from "../../../../../🖥️shell/🧬️schema/🔣️.json" with { type: "json" };
import fixture from "./🧫️fixtures/🔁️event-replay.json" with { type: "json" };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: Record<string, any>, _source: { readonly url: string }): Promise<void> {
  const { describe, expect, it } = vitest;
  const { commitUiPreferencesConfigMutation, readUiPreferenceEvents, readUiPreferences, replayUiPreferenceEvents, resolveUiPreferences, subscribeUiPreferences, UI_PREFERENCES_CONFIG_SCHEMA } = dependencies;

  describe("canonical OS UI preference host", () => {
    it("replays the language-agnostic event fixture and propagates it across shells sharing one OS store", () => {
      const physical = createMemoryStoragePort();
      const first = createScopedStoragePort(physical, "shared");
      const peer = createScopedStoragePort(physical, "shared");
      const isolated = createScopedStoragePort(physical, "isolated");
      const observed: unknown[] = [];
      const unsubscribe = subscribeUiPreferences(peer, (preferences: unknown) => observed.push(preferences));
      for (const event of fixture.events as UiPreferencesConfigMutation[]) commitUiPreferencesConfigMutation(first, event);
      unsubscribe();

      const reloaded = readUiPreferences(first);
      expect(reloaded).toEqual(fixture.expected);
      expect(replayUiPreferenceEvents(readUiPreferenceEvents(peer))).toEqual(fixture.expected);
      expect(observed).toHaveLength(fixture.events.length);
      expect(observed.at(-1)).toEqual(fixture.expected);
      expect(readUiPreferences(isolated)).toMatchObject({ appearance: null, layout: null, locale: null, terminology: null });
      expect(first.get("ui.chrome.locale")).toBeNull();
      const config = JSON.parse(physical.get("semio.shell.shared.semio.os.config") ?? "null");
      expect(JSON.parse(config.preferences[UI_PREFERENCES_CONFIG_SCHEMA])).toEqual({ version: 1, events: fixture.events });
      const validate = new Ajv({ strict: true }).addSchema(shellSchema).getSchema(`${shellSchema.$id}#/$defs/UiPreferences`);
      expect(validate?.(reloaded), JSON.stringify(validate?.errors)).toBe(true);
      expect(resolveUiPreferences(reloaded, { appearance: "system", layout: "desktop", driverId: "default", locale: "en", terminology: "native", themeId: "semio" }).locale).toBe("de");
      console.info("[DEBUG] canonical OS UI preference events replayed, cross-shell propagated, isolated, and schema-validated");
    });
  });
}
