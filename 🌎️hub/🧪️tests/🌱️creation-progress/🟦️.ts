// #region Header
/**
 * 🌱️ Third-party oracle (Ajv) for creation-status progress (`SpaceArtifactCreationProgress`): every status row of the
 * language-agnostic creation fixture is judged by Ajv against the directory schema and by the TypeScript twin parser;
 * both agree with the row's verdict, except where the row says JSON Schema cannot express the rule (`schemaValid`).
 */
// #endregion Header

import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { parseSpaceArtifactCreationStatusJsonV1 } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts";

const schemaDir = join(dirname(fileURLToPath(import.meta.url)), "../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema");
const module = JSON.parse(readFileSync(join(schemaDir, "🔣️.json"), "utf8"));
const fixture = JSON.parse(readFileSync(join(schemaDir, "🌱️space-artifact-creation-v1", "🔣️.json"), "utf8"));

describe("creation status progress oracle", () => {
  it("Ajv and the TypeScript twin judge every status row as the fixture does", () => {
    const ajv = new Ajv({ allErrors: true, strict: false });
    ajv.addSchema({ ...module, $id: "urn:semio:directory" });
    const validate = ajv.getSchema("urn:semio:directory#/$defs/SpaceArtifactCreationStatus")!;
    const progressRows = fixture.statuses.filter((row: any) => row.value?.progress !== undefined);
    expect(progressRows.length).toBeGreaterThanOrEqual(9);
    for (const row of fixture.statuses) {
      expect(validate(row.value), `${row.id}: ${JSON.stringify(validate.errors)}`).toBe(row.schemaValid ?? row.accepted);
      let parsed = true;
      try {
        parseSpaceArtifactCreationStatusJsonV1(JSON.stringify(row.value));
      } catch {
        parsed = false;
      }
      expect(parsed, row.id).toBe(row.accepted);
    }
  });
});
