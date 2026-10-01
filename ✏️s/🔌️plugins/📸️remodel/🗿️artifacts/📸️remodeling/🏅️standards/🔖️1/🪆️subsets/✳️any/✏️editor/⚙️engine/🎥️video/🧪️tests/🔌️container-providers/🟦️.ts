import assert from "node:assert/strict";
import Ajv from "ajv";
import corpus from "../../🧫️fixtures/🔌️container-providers/🔣️.json";
import schema from "../../🧬️schema/🔌️container-providers/🔣️.json";

/** 🔌️Checks the shared native selection corpus against strict AJV and independent metadata filtering. */
export function proveVideoContainerProviderOracleV1(): number {
  const ajv = new Ajv({ strict: true }).addSchema(schema);
  assert.equal(ajv.getSchema(schema.$id + "#/$defs/CasesV1")!(corpus), true);
  const inventory = ajv.getSchema(schema.$id + "#/$defs/InventoryV1")!;
  for (const row of corpus.cases) {
    const selected = row.providers.filter(provider => provider.matches);
    const invalid = !inventory(row.providers) || new Set(row.providers.map(provider => provider.id)).size !== row.providers.length;
    const actual = invalid ? { status: "invalid" } : selected.length === 0 ? { status: "missing" } : selected.length > 1 ? { status: "ambiguous" } : { status: "selected", id: selected[0]!.id };
    assert.deepEqual(actual, row.expected, row.id);
  }
  const probe = ajv.getSchema(schema.$id + "#/$defs/ProbeV1")!;
  for (const row of corpus.probes) {
    const valid = probe(row.probe) && row.probe.container === row.provider && row.probe.frame_count === row.probe.samples.length;
    assert.equal(valid, row.expected, row.id);
  }
  return corpus.cases.length + corpus.probes.length;
}
