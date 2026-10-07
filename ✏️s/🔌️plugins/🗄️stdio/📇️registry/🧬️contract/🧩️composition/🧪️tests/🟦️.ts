import { strict as assert } from "node:assert";
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { selectCompositionContributionsV1, admitCompositionContributionV1, resolveCompositionNativeFactoriesV1, resolveCompositionOpenTargetsV1 } from "../🟦️.ts";

/** 🧪️ Compares neutral contribution admission and selection with an independent schema oracle. */
export function runCompositionContributionChecks(): number {
  const read = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
  const corpus = read("../🧫️fixtures/🔣️.json");
  const ajv = new Ajv({ strict: true });
  const validRow = ajv.compile(read("../🧬️schema/🔣️.json"));
  const unique = ajv.compile({ type: "array", uniqueItems: true });
  for (const vector of corpus.cases) {
    const laws = vector.contributions.flatMap((row: any) => row.apps ?? []).flatMap((app: any) => app.laws ? Object.values(app.laws) : []);
    const oracle = vector.contributions.every((row: unknown) => validRow(row)) && ["artifact", "package", "order"].every((key) => unique(vector.contributions.map((row: Record<string, unknown>) => row[key]))) && unique(laws);
    let selected: string[] | undefined;
    try { selected = selectCompositionContributionsV1(vector.contributions, vector.selection).map((row) => row.artifact); } catch {}
    assert.equal(selected !== undefined, oracle, `${vector.id}: independent admission`);
    assert.equal(selected !== undefined, !vector.error, vector.id);
    if (selected) {
      const expected = vector.contributions.filter((row: { selections: string[] }) => row.selections.includes(vector.selection)).sort((a: { order: number }, b: { order: number }) => a.order - b.order).map((row: { artifact: string }) => row.artifact);
      assert.deepEqual(selected, expected, `${vector.id}: independent selection`);
      assert.deepEqual(selected, vector.expected, vector.id);
      const actualLaws = selectCompositionContributionsV1(vector.contributions, vector.selection).flatMap(row => row.apps).filter(app => app.role === "editor").map(app => ({ type: app.type, factory: app.factory, ...app.laws }));
      const expectedLaws = vector.contributions.filter((row: any) => row.selections.includes(vector.selection)).sort((a: any, b: any) => a.order - b.order).flatMap((row: any) => row.apps).filter((app: any) => app.role === "editor").map((app: any) => ({ type: app.type, factory: app.factory, ...app.laws }));
      assert.deepEqual(actualLaws, expectedLaws, `${vector.id}: each retained editor preserves both owned laws`);
    }
  }
  const validExport = ajv.compile(read("../🧬️schema/📤️native-export/🔣️.json"));
  const base = corpus.cases.find((vector: any) => !vector.error && vector.contributions.length)?.contributions[0];
  for (const vector of corpus.nativeExportCases) {
    assert(validExport(vector.export));
    const contribution = admitCompositionContributionV1({ ...base, nativeFactories: [vector.selection], openTargets: [{ factoryId: vector.selection.factoryId, role: "editor", surfaceId: "neutral.alpha.editor" }] });
    const actual = resolveCompositionNativeFactoriesV1(contribution, () => vector.export, () => vector.sourceProof);
    const independent = { artifact: base.artifact, ...JSON.parse(JSON.stringify(vector.export.factory)), descriptor_codec_id: vector.export.descriptorCodecId, protocol_source_sha256: vector.sourceProof, protocol_path: vector.selection.protocolPath, definition_path: vector.selection.definitionPath };
    assert.deepEqual(actual, [independent]); assert.deepEqual(actual, [vector.expected]);
    const target = resolveCompositionOpenTargetsV1([contribution], actual)[0]!;
    assert.equal(target.extension, vector.export.factory.extension); assert.equal(target.packSchemaHash, vector.export.factory.pack_schema_hash); assert.equal(target.protocolSourceSha256, vector.sourceProof);
    assert(!validRow({ ...base, nativeFactories: [{ ...vector.selection, packSchemaHash: "copied-stale-fact" }] }));
    assert.throws(() => admitCompositionContributionV1({ ...base, nativeFactories: [{ ...vector.selection, packSchemaHash: "copied-stale-fact" }] }));
    assert.throws(() => resolveCompositionNativeFactoriesV1(contribution, () => { throw new Error("absent owner export"); }, () => vector.sourceProof));
    assert.throws(() => resolveCompositionNativeFactoriesV1(contribution, () => ({ ...vector.export, factory: { ...vector.export.factory, factory_id: "unselected" } }), () => vector.sourceProof));
  }
  const empty = admitCompositionContributionV1({ ...base, nativeFactories: [], openTargets: [] });
  assert.deepEqual(resolveCompositionNativeFactoriesV1(empty, () => { throw new Error("must not read absent export"); }, () => { throw new Error("must not read absent source"); }), []);
  assert.deepEqual(resolveCompositionOpenTargetsV1([], []), []);
  assert.throws(() => resolveCompositionOpenTargetsV1([{ ...empty, openTargets: [{ factoryId: "absent", role: "editor", surfaceId: "neutral" }] }], []));
  return corpus.cases.length + corpus.nativeExportCases.length + 3;
}
