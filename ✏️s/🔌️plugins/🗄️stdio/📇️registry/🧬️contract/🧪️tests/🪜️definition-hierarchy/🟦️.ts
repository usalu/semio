import { readFileSync } from "node:fs";
import { strict as assert } from "node:assert";
import Ajv from "ajv";

/** 🪜️ Independent category grammar oracle and physical policy ownership witness. */
export function runDefinitionHierarchyChecks(): number {
  const read = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
  const corpus = read("../../🧫️fixtures/🪜️definition-hierarchy/🔣️.json");
  const ajv = new Ajv({ strict: true });
  assert.equal(corpus.cases.length, 9);
  const grammars: Record<string, string> = { standard: "standard\\.[^.]+", profile: "standard\\.[^.]+\\.profile\\.[^.]+", codec: "standard\\.[^.]+\\.codec\\.[^.]+\\.v[0-9]+", mutation: "mutation\\.[^.]+\\.v[0-9]+" };
  for (const row of corpus.cases) {
    const pattern = grammars[row.category];
    const validate = ajv.compile({ type: "string", ...(pattern ? { pattern: "^s\\.stdio\\.ifc\\." + pattern + "$" } : {}) });
    assert.equal(validate(row.identity), row.accepted, row.identity);
  }
  for (const row of corpus.runtimeCases) {
    const suffix = row.category === "codec" ? "codec\\.[^.]+\\.v[0-9]+" : "representation\\.[^.]+";
    const validate = ajv.compile({ type: "string", anyOf: row.standards.map((standard: string) => ({ pattern: "^" + standard.replaceAll(".", "\\.") + "\\." + suffix + "$" })) });
    assert.equal(validate(row.identity), row.accepted, row.identity);
  }
  const source = readFileSync(new URL("../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs", import.meta.url), "utf8");
  assert(!/stdio_artifact|is_stdio_|stdio_capability/.test(source), "general SDK contains Stdio-owned policy");
  return corpus.cases.length + corpus.runtimeCases.length + 2;
}
