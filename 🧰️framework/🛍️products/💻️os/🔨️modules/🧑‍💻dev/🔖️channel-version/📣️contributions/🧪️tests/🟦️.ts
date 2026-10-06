import assert from "node:assert/strict";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json";
import corpus from "../🧫️fixtures/🔣️.json";
import { resolve } from "node:path";
import * as channel from "../../🔍️census/🟦️.ts";
import { admitChannelVersionContributionsV1 } from "../🟦️.ts";

/** 📣️Proves closed outward consumer admission against independent schema and path oracles. */
export function proveChannelVersionContributionsV1(): number {
  const ajv = new Ajv({ strict: true }).addKeyword("x-semio-path-identity").addSchema(schema);
  
  const validate = ajv.getSchema(`${schema.$id}#/$defs/OwnersV1`)!;
  const failures: string[] = [];
  for (const row of corpus.cases) {
    const paths = row.input.flatMap(owner => owner.document.consumers.map(consumer => `${owner.ownerRoot}/${consumer.path}`));
    const identities = [...row.input.map(owner => owner.ownerRoot), ...row.input.flatMap(owner => owner.document.consumers.map(consumer => consumer.path))];
    const oracle = validate(row.input) && identities.every(path => path.normalize("NFC") === path) && new Set(paths.map(path => path.toLowerCase())).size === paths.length && new Set(row.input.map(owner => owner.ownerRoot.toLowerCase())).size === row.input.length;
    assert.equal(oracle, row.accepted, `${row.id}: AJV oracle`);
    let accepted = false, actual: string[] = [];
    try {
      const parse = admitChannelVersionContributionsV1;
      actual = parse(row.input).map(consumer => consumer.path);
      accepted = true;
    } catch {}
    if (accepted !== row.accepted || JSON.stringify(actual) !== JSON.stringify(row.paths)) failures.push(row.id);
  }
  assert.deepEqual(failures, [], "Outward consumer admission differs from the portable corpus");
  return corpus.cases.length;
}

/** 🔍️Checks missing-owner safety and strict current-owner census against an independent JSON oracle. */
export function proveChannelVersionContributionCensusV1(): number {
  const ajv = new Ajv({ strict: true }).addKeyword("x-semio-path-identity").addSchema(schema);
  
  for (const row of corpus.census) {
    const consumers = admitChannelVersionContributionsV1(row.owners);
    const files = row.files as Record<string, string>;
    const source = { pin: row.pin, consumers, candidates: row.candidates, readText: (path: string) => { if (!Object.hasOwn(files, path)) throw Error("Missing source"); return files[path]!; } };
    assert.deepEqual(channel.channelVersionCensus(source).findings.map(finding => finding.problem), row.expected, row.id);
    for (const consumer of consumers) {
      if (!Object.hasOwn(files, consumer.path)) continue;
      const values = Object.values(JSON.parse(files[consumer.path]!));
      const admitted = ajv.compile({ type: "array", items: { enum: [row.pin, ...(consumer.hostileValues ?? [])] } })(values);
      assert.equal(consumer.arbitrary || admitted, !row.expected.includes("drift"), `${row.id}: independent JSON/AJV pin oracle`);
      const hostile = values.filter(value => (consumer.hostileValues ?? []).includes(value as number)).length;
      assert.equal(consumer.arbitrary || hostile === (consumer.hostileOccurrences ?? 0), !row.expected.includes("hostile"), `${row.id}: independent hostile-count oracle`);
    }
  }
  return corpus.census.length;
}

/** 🟢️Executes the portable pure contract in independent esbuild and native Node. */
export async function proveIndependentChannelVersionContributionsV1(): Promise<void> {
  const { build } = await import("esbuild");
  const path = resolve(import.meta.dirname, "🟦️.ts");
  const source = `import { proveChannelVersionContributionsV1, proveChannelVersionContributionCensusV1 } from ${JSON.stringify(path)}; process.stdout.write(JSON.stringify({ admission: proveChannelVersionContributionsV1(), census: proveChannelVersionContributionCensusV1() }));`;
  const result = await build({ stdin: { contents: source, resolveDir: import.meta.dirname, loader: "ts" }, bundle: true, platform: "node", format: "esm", write: false });
  const node = Bun.spawnSync(["node", "--input-type=module"], { stdin: Buffer.from(result.outputFiles![0]!.text), stdout: "pipe", stderr: "pipe" });
  assert.equal(node.exitCode, 0, Buffer.from(node.stderr).toString());
  assert.deepEqual(JSON.parse(Buffer.from(node.stdout).toString()), { admission: corpus.cases.length, census: corpus.census.length });
}
