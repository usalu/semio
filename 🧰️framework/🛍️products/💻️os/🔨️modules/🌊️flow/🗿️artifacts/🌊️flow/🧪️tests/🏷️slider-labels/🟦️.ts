/** 🧱️ Retains the fixture law at its concrete product contract owner. */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";

const root = resolve(import.meta.dir, "../../../../../../../../..");
const fixture = <T = Record<string, unknown>>(path: string): T => JSON.parse(readFileSync(resolve(root, path), "utf8"));

test("slider descriptors require authored labels across schema and native contracts", () => {
  const schema = fixture<{ $defs: { FlowInputSliderDescriptorV1: object } }>("🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧬️schema/🔣️.json");
  const vectors = fixture<{ cases: { widget: Record<string, unknown>; expectedDagName: string }[] }>("🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧬️schema/📸️snapshot/🧫️fixtures/🏷️slider-labels.json");
  const validate = new Ajv().compile(schema.$defs.FlowInputSliderDescriptorV1);
  for (const row of vectors.cases) {
    expect(validate(row.widget)).toBe(true);
    const missing = { ...row.widget };
    delete missing.label;
    expect(validate(missing)).toBe(false);
  }
});
