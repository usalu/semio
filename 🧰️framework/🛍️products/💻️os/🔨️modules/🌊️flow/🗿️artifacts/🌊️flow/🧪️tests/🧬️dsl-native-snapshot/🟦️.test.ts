import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import Ajv2020 from "ajv/dist/2020.js";

const root = join(import.meta.dir, "../../🧫️fixtures/🧬️dsl-native-snapshot");
const schema = JSON.parse(readFileSync(join(root, "schema.json"), "utf8"));
const corpus = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8")) as { schema: string; cases: { id: string; description: string; snapshot: any }[] };
const validate = new Ajv2020({ strict: true, allErrors: true, allowUnionTypes: true }).compile(schema);

const canonical = (value: unknown): string => JSON.stringify(value, (_, inner) => (inner && typeof inner === "object" && !Array.isArray(inner) ? Object.fromEntries(Object.entries(inner).sort(([a], [b]) => (a < b ? -1 : 1))) : inner));
const depth = (tree: any): number => 1 + Math.max(0, ...tree.neurons.map((neuron: any) => (neuron.tree ? depth(neuron.tree) : 0)));

test("the corpus covers the sizes and kinds the plan names", () => {
  expect(corpus.schema).toBe("semio.flow.dsl-native-snapshot-law/v1");
  expect(corpus.cases.map((row) => row.id)).toEqual(["empty-document", "default-three-widget-chain", "every-widget-kind", "nested-cluster-depth-3", "unicode-identifiers", "large-layout"]);
  const kinds = new Set(corpus.cases.flatMap((row) => row.snapshot.widgets.map((widget: any) => widget.kind)));
  expect([...kinds].sort()).toEqual(["cluster", "inputImage", "inputNote", "inputSlider", "neuron", "outputAction", "outputExport", "outputPreview", "variable"]);
  const nested = corpus.cases.find((row) => row.id === "nested-cluster-depth-3")!.snapshot.widgets[0].tree;
  expect(depth(nested)).toBe(3);
  expect(corpus.cases.find((row) => row.id === "large-layout")!.snapshot.widgets.length).toBeGreaterThanOrEqual(100);
});

test("every case validates against the JSON Schema oracle", () => {
  for (const row of corpus.cases) expect(validate(row.snapshot), `${row.id}: ${JSON.stringify(validate.errors)}`).toBe(true);
});

test("the schema oracle refuses a snapshot that breaks the grammar", () => {
  const base = corpus.cases.find((row) => row.id === "every-widget-kind")!.snapshot;
  const mutations: Record<string, (copy: any) => void> = {
    "unknown widget kind": (copy) => { copy.widgets[0].kind = "bogus"; },
    "synapse wire literal instead of a table row": (copy) => { copy.synapses[0] = "s.number -> n.a"; },
    "missing camera zoom": (copy) => { delete copy.camera.zoom; },
    "unknown snapshot member": (copy) => { copy.extra = 1; },
    "duplicate expanded entry": (copy) => { copy.widgets.find((widget: any) => widget.kind === "outputPreview").expanded = ["a", "a"]; },
  };
  for (const [name, mutate] of Object.entries(mutations)) {
    const copy = structuredClone(base);
    mutate(copy);
    expect(validate(copy), name).toBe(false);
  }
});

test("documents are internally consistent: unique ids, resolvable synapse endpoints and layout keys", () => {
  for (const row of corpus.cases) {
    const ids = row.snapshot.widgets.map((widget: any) => widget.id);
    expect(new Set(ids).size, row.id).toBe(ids.length);
    for (const synapse of row.snapshot.synapses) {
      expect(ids.includes(synapse.from) && ids.includes(synapse.to), `${row.id}:${synapse.id}`).toBe(true);
    }
    for (const key of Object.keys(row.snapshot.layout)) expect(ids.includes(key), `${row.id}:${key}`).toBe(true);
    expect(new Set(row.snapshot.synapses.map((synapse: any) => synapse.id)).size, row.id).toBe(row.snapshot.synapses.length);
  }
});

test("the canonical text of each case is stable under parse and re-print (independent TS reader)", () => {
  for (const row of corpus.cases) expect(canonical(JSON.parse(canonical(row.snapshot))), row.id).toBe(canonical(row.snapshot));
});
