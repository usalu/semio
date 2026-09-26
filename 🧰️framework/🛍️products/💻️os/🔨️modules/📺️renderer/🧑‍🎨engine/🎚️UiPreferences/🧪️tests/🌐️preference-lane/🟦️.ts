import Ajv from "ajv";
import { createMemoryStoragePort, createScopedStoragePort, type StoragePort } from "@semio-tech/framework";
import { UI_PREFERENCE_MUTATION_KEYS, uiPreferenceMutationDataClassV1, type UiPreferencesConfigMutation } from "../../../../../../🎚️config/🧬️schema/🧬️mutations/🟦️.ts";
import { UI_PREFERENCE_DATA_CLASSES, type UiPreferences } from "../../../../../../🎚️config/🧬️schema/🟦️.ts";
import mutationSchema from "../../../../../../🎚️config/🧬️schema/🧬️mutations/🎨️ui-preferences/🧬️schema/🔣️.json" with { type: "json" };
import dataClassFixture from "../../../../../../🎚️config/🧬️schema/🎨️ui-preferences/🧫️fixtures/🗂️data-classes/🔣️.json" with { type: "json" };
import directorySchema from "../../../../../../🔨️modules/📇️directory/🧬️schema/🔣️.json" with { type: "json" };
import { validUserPreferenceRecordV1 } from "../../../../../../🔨️modules/📇️directory/🧬️schema/🟦️.ts";
import fixture from "../../🧫️fixtures/🌐️preference-lane/🔣️.json" with { type: "json" };
import recordFixture from "../../../../../../🔨️modules/📇️directory/🧬️schema/🧫️fixtures/🎚️user-preference-record/🔣️.json" with { type: "json" };

type LaneStep = { readonly commit?: { readonly device: "A" | "B"; readonly mutation: UiPreferencesConfigMutation }; readonly deliver?: "A" | "B"; readonly fold?: "A" | "B" };
type LaneScenario = { readonly id: string; readonly steps: readonly LaneStep[]; readonly expected: Readonly<Record<"A" | "B", Partial<UiPreferences>>> & { readonly hubEvents: number } };
type HubEvent = { readonly seq: number; readonly requestId: string; readonly schema: string; readonly mutation: string };

/** 🌐️ Laws of the per-user preference lane: data classes (schema-declared, Ajv-checked), and every scenario of
 * `🧫️fixtures/🌐️preference-lane/🔣️.json` played on two in-memory devices and an in-memory hub log — each device must
 * equal the fixture AND an independent oracle (the hub log replayed in seq order, then the device's unacknowledged
 * changes); every recorded event must satisfy the directory's own JSON schema under Ajv. */
export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: Record<string, any>, _source: { readonly url: string }): Promise<void> {
  const { describe, expect, it } = vitest;
  const { commitUiPreferenceOnLaneV1, foldUiPreferenceLanePageV1, readUiPreferenceLane, readUiPreferences, replayUiPreferenceEvents, uiPreferenceEnvelopeTextV1, parseUiPreferenceEnvelopeV1, UI_PREFERENCES_LANE_SCHEMA_V1, uiPreferenceLaneKeyV1 } = dependencies;

  describe("per-user preference lane", () => {
    it("declares a data class for every key and a key for every mutation, in the schemas, equal to the fixture", () => {
      const validate = new Ajv({ strict: true }).compile(mutationSchema);
      expect(Object.keys(UI_PREFERENCE_MUTATION_KEYS).sort()).toEqual(dataClassFixture.mutations.map((row) => row.mutation.mutation).sort());
      for (const row of dataClassFixture.mutations) {
        const mutation = row.mutation as UiPreferencesConfigMutation;
        expect(validate(mutation), `${mutation.mutation}: ${JSON.stringify(validate.errors)}`).toBe(true);
        expect(UI_PREFERENCE_MUTATION_KEYS[mutation.mutation], mutation.mutation).toBe(row.key);
        expect(UI_PREFERENCE_DATA_CLASSES[row.key as keyof UiPreferences], row.key).toBe(row.dataClass);
        expect(uiPreferenceMutationDataClassV1(mutation), mutation.mutation).toBe(row.dataClass);
      }
    });

    it("plays every scenario to the fixture's projections and to the hub-order oracle, with schema-valid recorded events", () => {
      const ajv = new Ajv({ strict: false });
      ajv.addSchema(directorySchema, "directory");
      const validBody = ajv.compile({ $ref: "directory#/$defs/DirectoryEventBody" });
      const validMutation = new Ajv({ strict: true }).compile(mutationSchema);
      const lane = uiPreferenceLaneKeyV1("http://127.0.0.1:7800/", "u1");
      expect(lane).toBe("http://127.0.0.1:7800#u1");
      for (const scenario of fixture.scenarios as unknown as readonly LaneScenario[]) {
        const physical = createMemoryStoragePort();
        const devices: Record<"A" | "B", StoragePort> = { A: createScopedStoragePort(physical, "device-a"), B: createScopedStoragePort(physical, "device-b") };
        const localOnly: Record<"A" | "B", UiPreferencesConfigMutation[]> = { A: [], B: [] };
        const hub: HubEvent[] = [];
        let minted = 0;
        const mint = (): string => (++minted).toString(16).padStart(32, "0");
        for (const step of scenario.steps) {
          if (step.commit) {
            const { queued } = commitUiPreferenceOnLaneV1(devices[step.commit.device], step.commit.mutation, lane, mint);
            if (queued === null) localOnly[step.commit.device].push(step.commit.mutation);
          }
          if (step.deliver) {
            for (const entry of readUiPreferenceLane(devices[step.deliver], lane).outbox) {
              if (hub.some((event) => event.requestId === entry.requestId)) continue;
              const mutation = uiPreferenceEnvelopeTextV1(entry);
              hub.push({ seq: hub.length + 1, requestId: entry.requestId, schema: UI_PREFERENCES_LANE_SCHEMA_V1, mutation });
            }
          }
          if (step.fold) {
            const device = devices[step.fold];
            const after = readUiPreferenceLane(device, lane).afterSeq;
            foldUiPreferenceLanePageV1(device, lane, { throughSeqInclusive: hub.length, events: hub.filter((event) => event.seq > after) });
          }
        }
        expect(hub.length, `${scenario.id}: hub events`).toBe(scenario.expected.hubEvents);
        for (const event of hub) {
          expect(validUserPreferenceRecordV1(event.schema, event.mutation), `${scenario.id}: record bound`).toBe(true);
          expect(validBody({ kind: "user.preference-recorded", userId: "u1", schema: event.schema, mutation: event.mutation }), `${scenario.id}: ${JSON.stringify(validBody.errors)}`).toBe(true);
          const envelope = parseUiPreferenceEnvelopeV1(event.schema, event.mutation);
          expect(envelope !== null && validMutation(envelope.mutation), `${scenario.id}: envelope mutation`).toBe(true);
        }
        for (const name of ["A", "B"] as const) {
          const projection = readUiPreferences(devices[name]) as UiPreferences;
          const oracle = replayUiPreferenceEvents([...localOnly[name], ...hub.map((event) => parseUiPreferenceEnvelopeV1(event.schema, event.mutation)!.mutation), ...readUiPreferenceLane(devices[name], lane).outbox.map((entry: { mutation: UiPreferencesConfigMutation }) => entry.mutation)]) as UiPreferences;
          for (const [key, value] of Object.entries(scenario.expected[name])) {
            expect(projection[key as keyof UiPreferences], `${scenario.id}: ${name}.${key}`).toEqual(value);
            expect(oracle[key as keyof UiPreferences], `${scenario.id}: oracle ${name}.${key}`).toEqual(value);
          }
          for (const key of Object.keys(UI_PREFERENCE_DATA_CLASSES) as (keyof UiPreferences)[]) {
            if (UI_PREFERENCE_DATA_CLASSES[key] === "persistedShared") expect(projection[key], `${scenario.id}: ${name}.${key} equals the hub-order oracle`).toEqual(oracle[key]);
          }
        }
      }
    });

    it("refuses a recorded change that is foreign, device-local or malformed", () => {
      const own = uiPreferenceEnvelopeTextV1({ id: "a".repeat(32), mutation: { mutation: "setLocale", locale: "de" } });
      expect(parseUiPreferenceEnvelopeV1(UI_PREFERENCES_LANE_SCHEMA_V1, own)).toEqual({ id: "a".repeat(32), mutation: { mutation: "setLocale", locale: "de" } });
      expect(parseUiPreferenceEnvelopeV1("other.vocabulary.v1", own)).toBeNull();
      expect(parseUiPreferenceEnvelopeV1(UI_PREFERENCES_LANE_SCHEMA_V1, uiPreferenceEnvelopeTextV1({ id: "a".repeat(32), mutation: { mutation: "setLayout", layout: "tablet" } }))).toBeNull();
      expect(parseUiPreferenceEnvelopeV1(UI_PREFERENCES_LANE_SCHEMA_V1, uiPreferenceEnvelopeTextV1({ id: "not-hex", mutation: { mutation: "setLocale", locale: "de" } }))).toBeNull();
      expect(parseUiPreferenceEnvelopeV1(UI_PREFERENCES_LANE_SCHEMA_V1, JSON.stringify({ id: "a".repeat(32), mutation: { mutation: "setLocale", locale: "fr" } }))).toBeNull();
      const ajv = new Ajv({ strict: false });
      ajv.addSchema(directorySchema, "directory");
      const validCommand = ajv.compile({ $ref: "directory#/$defs/DirectoryCommand" });
      expect(recordFixture.rows.length).toBeGreaterThanOrEqual(15);
      for (const row of recordFixture.rows) {
        expect(validUserPreferenceRecordV1(row.schema, row.mutation), row.id).toBe(row.valid);
        const withinSchema = validCommand({ kind: "record-user-preference", schema: row.schema, mutation: row.mutation });
        if (row.valid) expect(withinSchema, `${row.id}: Ajv admits every admitted record`).toBe(true);
        if (!withinSchema) expect(row.valid, `${row.id}: Ajv refuses only refused records`).toBe(false);
      }
    });
  });
}
