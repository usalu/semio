/** 🧩️ The native execution-target module resolution (`📇️directory/🔌️client/🧩️execution-target-module`), replayed from the
 * language-agnostic fixture `🧫️fixtures/📇️directory/🧩️execution-target-module-resolution-v1.json` over the hub's own lease
 * corpus. Ajv and `node:crypto` are the independent oracles: Ajv holds the fixture to its schema, SHA-256 decides every
 * source and refusal from the corpus bytes — the same decision the Rust law asserts the resolver makes. */
import { describe, expect, it } from "vitest";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";

const here = (path: string) => JSON.parse(readFileSync(fileURLToPath(new URL(path, import.meta.url)), "utf8"));
const fixture = here("../../🧫️fixtures/📇️directory/🧩️execution-target-module-resolution-v1.json");
const schema = here("../../🧬️schema/🧩️execution-target-module-resolution-v1/🔣️.json");
const corpus = here(`../../../../../${fixture.corpus}`);
const hexBytes = (value: string) => Uint8Array.from(value.match(/../gu) ?? [], (pair) => Number.parseInt(pair, 16));
const sha256 = (bytes: Uint8Array) => createHash("sha256").update(bytes).digest("hex");
const flipped = (bytes: Uint8Array) => Uint8Array.from(bytes, (byte, index) => (index === 0 ? byte ^ 0xff : byte));
const component = hexBytes(corpus.componentHex);
const descriptor = hexBytes(corpus.descriptorHex);
const lease = corpus.manifest;
const holds = (bytes: Uint8Array, expected: { sha256: string; byteLength: number }) => bytes.length === expected.byteLength && sha256(bytes) === expected.sha256;

type Case = { id: string; local: string; store: string; hub: string; expected: { outcome: string; requests: string[]; steps: string[] } };

/** 🧮️ The oracle's own resolution: what a shell must do, decided from SHA-256 alone. */
function oracle(row: Case): Case["expected"] {
  const asked = (kind: string) => (row.hub === "unavailableOnce" ? [kind, kind] : [kind]);
  if (row.hub === "unavailable") return { outcome: "refused", requests: ["manifest", "manifest", "manifest"], steps: ["lease"] };
  if (row.local === "leaseComponent") return { outcome: "local", requests: asked("manifest"), steps: ["lease", "verified"] };
  const storedComponent = row.store === "empty" ? null : row.store === "tamperedComponent" ? flipped(component) : component;
  const storedDescriptor = row.store === "empty" ? null : row.store === "tamperedDescriptor" ? flipped(descriptor) : descriptor;
  const componentStored = storedComponent !== null && holds(storedComponent, lease.component);
  const descriptorStored = storedDescriptor !== null && holds(storedDescriptor, lease.descriptor);
  const requests = asked("manifest");
  const steps = ["lease"];
  if (!componentStored) {
    steps.push("component");
    if (row.hub === "cancelledAtComponent") return { outcome: "cancelled", requests, steps };
    requests.push(...asked("component"));
    if (!holds(row.hub === "wrongComponent" ? flipped(component) : component, lease.component)) return { outcome: "refused", requests, steps };
  }
  if (!descriptorStored) {
    steps.push("descriptor");
    requests.push(...asked("descriptor"));
    if (!holds(row.hub === "wrongDescriptor" ? flipped(descriptor) : descriptor, lease.descriptor)) return { outcome: "refused", requests, steps };
  }
  steps.push("verified");
  return { outcome: componentStored && descriptorStored ? "store" : "hub", requests, steps };
}

describe("🧩️ execution-target module resolution", () => {
  it("the fixture satisfies its schema", () => {
    const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  });

  it("the corpus bytes are exactly the lease's", () => {
    expect(holds(component, lease.component)).toBe(true);
    expect(holds(descriptor, lease.descriptor)).toBe(true);
  });

  for (const row of fixture.cases as Case[]) {
    it(`node:crypto decides ${row.id} as the fixture declares`, () => expect(oracle(row)).toEqual(row.expected));
  }
});
