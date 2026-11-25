/** 🧺️ Action sets over `🧫️fixtures/🧫️action-sets/🔣️.json` — TypeScript twin of the Rust laws beside this file (`PluginManifest::resolved`, `PluginManifest::validate_action_sets`); oracle: Ajv (strict) checks the shape of every resolution case before it is used. */
import { describe, expect, test } from "bun:test";
import Ajv from "ajv";
import fixture from "../🧫️fixtures/🧫️action-sets/🔣️.json";
import { actionSetsFault, resolveManifestActionSets } from "../../🧺️action-sets/🟦️.ts";
import type { ActionDefinition } from "../../🎬️action-types-and-semantics/🟦️.ts";

const ids = { type: "array", items: { type: "string" } } as const;
const caseSchema = {
  type: "object",
  additionalProperties: false,
  required: ["case", "apps", "actionSets", "resolved", "fault"],
  properties: {
    case: { type: "string" },
    apps: { type: "object", additionalProperties: ids },
    actionSets: { type: "array", items: { type: "object", additionalProperties: false, required: ["id", "appIds", "actions"], properties: { id: { type: "string" }, appIds: ids, actions: ids } } },
    resolved: { oneOf: [{ type: "null" }, { type: "object", additionalProperties: ids }] },
    fault: { oneOf: [{ type: "null" }, { type: "string" }] },
  },
} as const;
const validCase = new Ajv({ strict: true }).compile(caseSchema);
const row = (id: string): ActionDefinition => ({ id }) as unknown as ActionDefinition;

describe("action sets resolve and validate the way every language does", () => {
  for (const entry of fixture.resolution) {
    test(entry.case, () => {
      expect(validCase(entry), JSON.stringify(validCase.errors)).toBe(true);
      const manifest = {
        apps: Object.entries(entry.apps).map(([id, actions]) => ({ id, actions: actions.map(row) })),
        actionSets: entry.actionSets.map((set) => ({ id: set.id, appIds: set.appIds, actions: set.actions.map(row) })),
      };
      expect(actionSetsFault(manifest) ?? null).toBe(entry.fault);
      if (entry.resolved === null) return;
      const resolved = resolveManifestActionSets(manifest);
      expect(resolved.actionSets ?? []).toEqual([]);
      expect(Object.fromEntries(resolved.apps.map((app) => [app.id, (app.actions ?? []).map((action) => action.id)]))).toEqual(entry.resolved);
    });
  }
});
