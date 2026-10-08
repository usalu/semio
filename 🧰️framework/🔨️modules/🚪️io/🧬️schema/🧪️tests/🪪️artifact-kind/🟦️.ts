import { readFileSync } from "node:fs";
import { join } from "node:path";
import Ajv from "ajv";
import { expect, test } from "bun:test";
import { isCanonicalArtifactKind } from "../../🟦️.ts";

test("artifact kinds admit every canonical domain and preserve exact owner segments", () => {
  const contract = JSON.parse(readFileSync(join(import.meta.dir, "../../🧬️schema/🔣️.json"), "utf8"));
  const fixture = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🪪️artifact-kind/🔣️.json"), "utf8")) as { cases: { kind: string; parts: string[] | null }[] };
  const oracle = new Ajv({ strict: true }).compile(contract.$defs.ArtifactKindId);
  for (const row of fixture.cases) {
    expect(Boolean(oracle(row.kind))).toBe(row.parts !== null);
    expect(isCanonicalArtifactKind(row.kind)).toBe(row.parts !== null);
    if (row.parts) expect(row.kind.split(".")).toEqual(row.parts);
  }
});
