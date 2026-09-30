/** 🧱️ Shared contract witnesses stay with the framework that owns their semantics. */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { hierarchy } from "d3-hierarchy";
import Ajv from "ajv";

const root = resolve(import.meta.dir, "../../..");
const fixture = <T = Record<string, unknown>>(path: string): T => JSON.parse(readFileSync(resolve(root, path), "utf8"));
type Layer = { id: string; locked: boolean; children?: Layer[] };
type Protection = { locked: boolean; inherited: boolean; descendant: boolean; editable: boolean; structural: boolean; canChangeLock: boolean };

test("shared pixel lock vectors match the independent hierarchy oracle", () => {
  const vectors = fixture<{ layers: Layer[]; cases: { id: string; expected: Protection }[] }>("🧰️framework/🔨️modules/🗺️surface/🎨️paint/🧫️fixtures/🔒️protection/🔣️.json");
  const tree = hierarchy<Layer>({ id: "root", locked: false, children: vectors.layers }, node => node.children);
  for (const row of vectors.cases) {
    const node = tree.descendants().find(node => node.data.id === row.id)!;
    const locked = node.data.locked;
    const inherited = node.ancestors().slice(1).some(node => node.data.locked);
    const descendant = node.descendants().slice(1).some(node => node.data.locked);
    const editable = !locked && !inherited;
    expect({ locked, inherited, descendant, editable, structural: editable && !descendant, canChangeLock: !inherited }).toEqual(row.expected);
  }
});

test("the lease corpus binds both peers to one framework contract witness", () => {
  const vectors = fixture<{ schema: string; plan: { package: Record<string, string> }; manifest: { package: Record<string, string> } }>("🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json");
  expect(vectors.schema).toBe("semio.os.document-execution-target-lease-corpus/v1");
  expect(vectors.plan.package.componentSha256).toBe(vectors.manifest.package.componentSha256);
  expect(vectors.plan.package.descriptorByteSha256).toBe(vectors.manifest.package.descriptorByteSha256);
});

test("neutral job pages agree with the independent JSON Schema oracle", () => {
  const path = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🧫️fixtures/🗳️job-client";
  const schema = fixture<{ $defs: { EventPage: object } }>(`${path}/🧬️schema/🔣️.json`);
  const vectors = fixture<{ cases: { response: { jobId: string }; requestedJobId: string; expectedError: string | null }[] }>(`${path}/🔣️.json`);
  const ajv = new Ajv();
  expect(ajv.compile(schema)(vectors)).toBe(true);
  const validate = ajv.compile(schema.$defs.EventPage);
  for (const row of vectors.cases) expect(validate(row.response) && row.response.jobId === row.requestedJobId).toBe(row.expectedError === null);
});

test("approval intents are closed under the framework-owned contract", () => {
  const schema = fixture("🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧬️schema/✅️approval-request/🔣️.json");
  const validate = new Ajv().compile(schema);
  const intent = { schema: "semio.hub.inference-approval/v1", version: 1, jobId: "ab".repeat(16), proposalHash: "cd".repeat(32) };
  expect(validate(intent)).toBe(true);
  expect(validate({ ...intent, proposal: "private bytes" })).toBe(false);
  expect(validate({ ...intent, jobId: "invalid" })).toBe(false);
});

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
