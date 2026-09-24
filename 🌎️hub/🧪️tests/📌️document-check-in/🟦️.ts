// #region Header
/**
 * 📌️ Third-party oracle for the Check In contract: every language-neutral vector of
 * `📌️document-check-in-v1/🧫️fixtures` is judged by Ajv against the directory schema's
 * `DocumentCheckInV1`/`DocumentCheckInStatusV1` exports and by the TypeScript twin; the Rust twin
 * reads the same file (`📌️document-check-in-v1/🧪️tests/🔬️unit`).
 * @see ../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/📌️document-check-in-v1/🦀️.rs
 */
// #endregion Header

import { readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { hubSchemaExport } from "../../🤝️integration-harness/🟦️.ts";
import { documentCheckInCanonicalJson, parseDocumentCheckInStatusV1, parseDocumentCheckInV1 } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/📌️document-check-in-v1/🟦️.ts";

const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), "..", "..", "..");
type Vector = { name: string; source: string; schemaValid?: boolean };
const fixture = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/📌️document-check-in-v1/🧫️fixtures/🔣️.json"), "utf8")) as {
  valid: { requests: Vector[]; statuses: Vector[] };
  invalid: { requests: Vector[]; statuses: Vector[] };
};
const parse = (source: string): unknown => {
  try {
    return JSON.parse(source);
  } catch {
    return undefined;
  }
};

describe("document check-in contract", () => {
  const request = hubSchemaExport(repoRoot, "schema://os.directory/DocumentCheckInV1");
  const status = hubSchemaExport(repoRoot, "schema://os.directory/DocumentCheckInStatusV1");

  it("admits every valid vector in Ajv and the twin, which re-emits it canonically", () => {
    for (const vector of fixture.valid.requests) {
      expect(request(parse(vector.source)), vector.name).toBe(true);
      const parsed = parseDocumentCheckInV1(vector.source);
      expect(parsed, vector.name).not.toBeNull();
      expect(documentCheckInCanonicalJson(parsed!)).toBe(vector.source);
    }
    for (const vector of fixture.valid.statuses) {
      expect(status(parse(vector.source)), vector.name).toBe(true);
      expect(parseDocumentCheckInStatusV1(vector.source), vector.name).not.toBeNull();
    }
  });

  it("refuses every invalid vector in the twin and agrees with Ajv on what the schema alone can say", () => {
    for (const vector of fixture.invalid.requests) {
      expect(Boolean(request(parse(vector.source))), vector.name).toBe(vector.schemaValid);
      expect(parseDocumentCheckInV1(vector.source), vector.name).toBeNull();
    }
    for (const vector of fixture.invalid.statuses) {
      expect(Boolean(status(parse(vector.source))), vector.name).toBe(vector.schemaValid);
      expect(parseDocumentCheckInStatusV1(vector.source), vector.name).toBeNull();
    }
  });
});
