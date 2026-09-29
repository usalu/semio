/** 🧩️ The native execution-target module resolution (`📇️directory/🔌️client/🧩️execution-target-module`), replayed from the
 * language-agnostic fixture `🧫️fixtures/📇️directory/🧩️execution-target-module-resolution-v1.json` over the hub's own lease
 * corpus. Ajv and `node:crypto` are the independent oracles: Ajv holds the fixture to its schema, SHA-256 decides every
 * source and refusal from the corpus bytes — the same decision the Rust law asserts the resolver makes — and replays the
 * store's `eviction` corpus through a least-recently-used model of its own. */
import { describe, expect, it } from "vitest";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { semioSchemaAjvV1 } from "../🧬️schema-oracle/🟦️.ts";

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

type EvictionStep = { op: "admit" | "read" | "tamper" | "restart"; component?: string; hit?: boolean; resident: string[] };

/** 🧮️ The oracle's own bounded store: recency order (least recent first), which entries still verify, and the byte total —
 * admitting or reading an entry uses it, admitting evicts the least recently used others beyond the capacity, a read of an
 * entry that no longer verifies removes it, and a restart forgets nothing. */
function evictionOracle(eviction: { componentCapacityBytes: number; components: Record<string, { fill: number; byteLength: number; sha256: string }>; steps: EvictionStep[] }) {
  const order: string[] = [];
  const intact = new Map<string, boolean>();
  const use = (name: string) => {
    const index = order.indexOf(name);
    if (index >= 0) order.splice(index, 1);
    order.push(name);
  };
  const bytes = (name: string) => new Uint8Array(eviction.components[name]!.byteLength).fill(eviction.components[name]!.fill);
  return eviction.steps.map((step) => {
    let hit: boolean | undefined;
    if (step.op === "admit") {
      use(step.component!);
      intact.set(step.component!, sha256(bytes(step.component!)) === eviction.components[step.component!]!.sha256);
      let total = order.reduce((sum, name) => sum + eviction.components[name]!.byteLength, 0);
      for (const name of [...order]) {
        if (total <= eviction.componentCapacityBytes) break;
        if (name === step.component) continue;
        order.splice(order.indexOf(name), 1);
        total -= eviction.components[name]!.byteLength;
      }
    } else if (step.op === "read") {
      hit = order.includes(step.component!) && intact.get(step.component!) === true;
      if (hit) use(step.component!);
      else if (order.includes(step.component!)) order.splice(order.indexOf(step.component!), 1);
    } else if (step.op === "tamper") {
      use(step.component!);
      intact.set(step.component!, sha256(flipped(bytes(step.component!))) === eviction.components[step.component!]!.sha256);
    }
    return { op: step.op, ...(step.component === undefined ? {} : { component: step.component }), ...(hit === undefined ? {} : { hit }), resident: [...order].sort() };
  });
}

describe("🧩️ execution-target module resolution", () => {
  it("the fixture satisfies its schema", () => {
    const validate = semioSchemaAjvV1({ allErrors: true, strict: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  });

  it("the corpus bytes are exactly the lease's", () => {
    expect(holds(component, lease.component)).toBe(true);
    expect(holds(descriptor, lease.descriptor)).toBe(true);
  });

  for (const row of fixture.cases as Case[]) {
    it(`node:crypto decides ${row.id} as the fixture declares`, () => expect(oracle(row)).toEqual(row.expected));
  }

  it("node:crypto hashes every eviction component to its declared content address", () => {
    for (const component of Object.values(fixture.eviction.components) as { fill: number; byteLength: number; sha256: string }[]) {
      expect(sha256(new Uint8Array(component.byteLength).fill(component.fill))).toBe(component.sha256);
    }
  });

  it("the bounded store evicts the least recently used component and serves only verified bytes, as the fixture declares", () => {
    expect(evictionOracle(fixture.eviction)).toEqual(fixture.eviction.steps);
  });
});
