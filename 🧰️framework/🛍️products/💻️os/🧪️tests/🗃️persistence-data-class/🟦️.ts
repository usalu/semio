import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import {
  bindingsDataClass,
  ephemeralLocalPersistenceBinding,
  ephemeralSharedPersistenceBinding,
  folderPersistenceBinding,
  hubPersistenceBinding,
  wireLaneDataClass,
  type PersistenceBinding,
  type PersistenceDataClass,
} from "../../🟦️.ts";
import { semioSchemaAjvV1 } from "@semio-tech/schema-ts/oracles/validator";
import { dataClassV1FixtureUrl, persistenceDataClassSchemaUrl } from "../../🔨️modules/🏪️store/🧪️tests/🧫️fixtures/🟦️.ts";

const here = dirname(fileURLToPath(import.meta.url));
const fixture = JSON.parse(
  readFileSync(dataClassV1FixtureUrl, "utf8"),
);
const schema = JSON.parse(
  readFileSync(persistenceDataClassSchemaUrl, "utf8"),
);

describe("persistence data class routing", () => {
  it("classifies every durable binding independently of peer order", () => {
    const ajv = semioSchemaAjvV1({ strict: false });
    const shared = ajv.compile({ type: "array", contains: { type: "object", required: ["dataClass"], properties: { dataClass: { const: "persistedShared" } } } });
    const durable = ajv.compile({ type: "array", contains: { type: "object", required: ["dataClass"], properties: { dataClass: { const: "persistedLocalOnly" } } } });
    const ephemeral = ajv.compile({ type: "array", contains: { type: "object", required: ["dataClass"], properties: { dataClass: { const: "ephemeralShared" } } } });
    for (const row of [...fixture.bindingSets, ...fixture.ephemeralBindingSets]) {
      const bindings = row.members.map((id: string) => fixture.cases.find((candidate: { id: string }) => candidate.id === id).binding) as PersistenceBinding[];
      const oracle = shared(bindings) ? "persistedShared" : durable(bindings) ? "persistedLocalOnly" : ephemeral(bindings) ? "ephemeralShared" : "ephemeralLocalOnly";
      expect(oracle, row.id).toBe(row.expected);
      expect(bindingsDataClass(bindings), row.id).toBe(oracle);
      expect(bindingsDataClass([...bindings].reverse()), row.id).toBe(oracle);
    }
  });

  it("validates the language-agnostic fixture against schema", () => {
    const ajv = semioSchemaAjvV1({ strict: false, allErrors: true });
    ajv.addSchema(schema);
    const validateCase = ajv.getSchema(`${schema.$id}#/$defs/ClassifiedPersistenceBinding`);
    const validateLane = ajv.getSchema(`${schema.$id}#/$defs/ClassifiedWireLane`);
    expect(validateCase).toBeTruthy();
    expect(validateLane).toBeTruthy();
    for (const c of fixture.cases) {
      expect(validateCase!(c.binding), JSON.stringify(validateCase!.errors)).toBe(true);
      if (c.wireLane) {
        expect(validateLane!(c.wireLane), JSON.stringify(validateLane!.errors)).toBe(true);
      }
    }
  });

  it("routes bindings and lanes to the four data classes", () => {
    for (const c of fixture.cases) {
      const expected = c.expectedDataClass as PersistenceDataClass;
      let binding: PersistenceBinding | null = null;
      if (c.binding.kind === "folder") binding = folderPersistenceBinding(c.binding.path);
      if (c.binding.kind === "hub") binding = hubPersistenceBinding(c.binding.baseUrl, c.binding.spaceId);
      if (c.binding.kind === "ephemeral" && c.binding.dataClass === "ephemeralLocalOnly") {
        binding = ephemeralLocalPersistenceBinding();
      }
      if (c.binding.kind === "ephemeral" && c.binding.dataClass === "ephemeralShared") {
        binding = ephemeralSharedPersistenceBinding(c.binding.lane ?? "preview");
      }
      expect(binding, c.id).toBeTruthy();
      expect(binding!.dataClass).toBe(expected);
      if (binding!.kind === "folder" || binding!.kind === "hub") {
        expect(bindingsDataClass([binding!])).toBe(expected);
      } else if (binding!.dataClass === "ephemeralLocalOnly") {
        expect(bindingsDataClass([])).toBe("ephemeralLocalOnly");
      }
      if (c.wireLane) {
        expect(wireLaneDataClass(c.wireLane.lane)).toBe(c.wireLane.dataClass);
        expect(wireLaneDataClass(c.wireLane.lane)).toBe("ephemeralShared");
      }
      expect(binding!.dataClass === "persistedShared").toBe(c.allowsShare);
      expect(binding!.dataClass === "persistedShared").toBe(c.allowsCollaboration);
      expect(binding!.dataClass === "persistedLocalOnly" || binding!.dataClass === "persistedShared").toBe(c.durable);
    }
    expect(fixture.homeUnion.hubSpace.origin).toBe("hub");
    expect(fixture.homeUnion.hubSpace.dataClass).toBe("persistedShared");
    expect(fixture.homeUnion.ephemeralStudio.dataClass).toBe("ephemeralLocalOnly");
    expect(fixture.homeUnion.ephemeralStudio.backbone).toBeNull();
  });
});
