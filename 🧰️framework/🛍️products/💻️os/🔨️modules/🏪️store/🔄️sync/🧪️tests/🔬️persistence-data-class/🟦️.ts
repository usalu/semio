import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import {
  bindingsDataClass,
  folderPersistenceBinding,
  hubPersistenceBinding,
  wireLaneDataClass,
  type PersistenceBinding,
  type PersistenceDataClass,
} from "../../../../../🟦️.ts";

type Vitest = NonNullable<ImportMeta["vitest"]>;

type FixtureCase = {
  readonly id: string;
  readonly binding: PersistenceBinding | { readonly kind: "ephemeral"; readonly dataClass: PersistenceDataClass; readonly lane?: "preview" | "presence" };
  readonly wireLane?: { readonly lane: "command" | "preview" | "presence"; readonly dataClass: PersistenceDataClass };
  readonly expectedDataClass: PersistenceDataClass;
  readonly allowsShare: boolean;
  readonly allowsCollaboration: boolean;
  readonly durable: boolean;
};

type Fixture = {
  readonly schema: string;
  readonly cases: readonly FixtureCase[];
};

const root = join(dirname(fileURLToPath(import.meta.url)), "../../..");
const fixture = JSON.parse(readFileSync(join(root, "🧫️fixtures/persistence-data-class-v1/🔣️.json"), "utf8")) as Fixture;
const schema = JSON.parse(readFileSync(join(root, "🔄️sync/🧬️schema/persistence-data-class/🔣️.json"), "utf8"));

function classOf(binding: FixtureCase["binding"]): PersistenceDataClass {
  if (binding.kind === "folder" || binding.kind === "hub") return binding.dataClass;
  return binding.dataClass;
}

export default function persistenceDataClassSuite(vitest: Vitest): void {
  const { describe, expect, it } = vitest;
  describe("persistence-data-class fixture", () => {
    it("validates the language-agnostic fixture against the schema", () => {
      const ajv = new Ajv({ strict: true, allErrors: true });
      ajv.addSchema(schema);
      for (const caseRow of fixture.cases) {
        const validateBinding = ajv.getSchema(`${schema.$id}#/$defs/ClassifiedPersistenceBinding`);
        expect(validateBinding?.(caseRow.binding), caseRow.id).toBe(true);
        if (caseRow.wireLane) {
          const validateLane = ajv.getSchema(`${schema.$id}#/$defs/ClassifiedWireLane`);
          expect(validateLane?.(caseRow.wireLane), caseRow.id).toBe(true);
        }
      }
    });

    it("routes bindings and lanes with share/collab/durable gates", () => {
      for (const caseRow of fixture.cases) {
        const dataClass = classOf(caseRow.binding);
        expect(dataClass, caseRow.id).toBe(caseRow.expectedDataClass);
        const durable = dataClass === "persistedLocalOnly" || dataClass === "persistedShared";
        const allowsShare = dataClass === "persistedShared";
        expect(durable, caseRow.id).toBe(caseRow.durable);
        expect(allowsShare, caseRow.id).toBe(caseRow.allowsShare);
        expect(allowsShare, caseRow.id).toBe(caseRow.allowsCollaboration);
        if (caseRow.wireLane) {
          expect(wireLaneDataClass(caseRow.wireLane.lane)).toBe(caseRow.wireLane.dataClass);
          expect(wireLaneDataClass(caseRow.wireLane.lane) === "ephemeralShared").toBe(true);
        }
      }
      expect(bindingsDataClass([])).toBe("ephemeralLocalOnly");
      expect(bindingsDataClass([folderPersistenceBinding("/tmp/demo")])).toBe("persistedLocalOnly");
      expect(bindingsDataClass([hubPersistenceBinding("http://hub.test", "space-1")])).toBe("persistedShared");
    });
  });
}

if (import.meta.vitest) {
  persistenceDataClassSuite(import.meta.vitest);
}
