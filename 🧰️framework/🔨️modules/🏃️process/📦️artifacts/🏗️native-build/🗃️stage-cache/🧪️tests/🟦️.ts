import { expect, test } from "bun:test";
import Ajv from "ajv";
import { sha256 } from "@noble/hashes/sha2.js";
import { existsSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { STAGE_CACHE_POLICY_V1, stageCacheV1, stageKeyCanonicalV1, stageKeyDigestV1, type StageKeyV1 } from "../🟦️.ts";

const fixture = JSON.parse(readFileSync(new URL("🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
const schema = JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json", import.meta.url), "utf8"));
const artifacts = process.env.SEMIO_TEST_ARTIFACT_DIR;
if (!artifacts) throw Error("Caller-owned SEMIO_TEST_ARTIFACT_DIR is required");
const ajv = new Ajv({ strict: false });
ajv.addSchema(schema);
const admitKey = ajv.getSchema(`${schema.$id}#/definitions/StageKeyV1`)!;
const admitEntry = ajv.getSchema(`${schema.$id}#/definitions/StageEntryV1`)!;
const noble = (text: string): string => Buffer.from(sha256(new TextEncoder().encode(text))).toString("hex");

test("stage keys serialize to the Python oracle's canonical bytes and digest equally under Noble SHA-256", () => {
  expect(fixture.keys.length).toBeGreaterThan(3);
  for (const row of fixture.keys) {
    expect(admitKey(row.key), JSON.stringify(admitKey.errors)).toBe(true);
    expect(stageKeyCanonicalV1(row.key)).toBe(row.canonical);
    expect(stageKeyDigestV1(row.key)).toBe(row.sha256);
    expect(noble(row.canonical)).toBe(row.sha256);
    const reordered = { tools: row.key.tools, inputs: Object.fromEntries(Object.entries(row.key.inputs).reverse()), schemaVersion: row.key.schemaVersion, stage: row.key.stage } as StageKeyV1;
    expect(stageKeyDigestV1(reordered)).toBe(row.sha256);
  }
});

test("malformed keys are refused by the first-party validator and by Ajv alike", () => {
  for (const row of fixture.refusals) {
    expect(admitKey(row.key), row.name).toBe(false);
    expect(() => stageKeyDigestV1(row.key), row.name).toThrow("Invalid StageKeyV1");
  }
});

test("a miss produces once, a hit restores the same bytes without producing, and a changed tool identity misses", async () => {
  const root = mkdtempSync(join(artifacts, "stage-cache-")), cache = stageCacheV1(join(root, "store"));
  const key: StageKeyV1 = fixture.keys[0].key;
  let produced = 0;
  const produce = async (directory: string): Promise<void> => {
    produced++;
    for (const file of fixture.files) { mkdirSync(dirname(join(directory, file.path)), { recursive: true }); writeFileSync(join(directory, file.path), file.text); }
  };
  const first = await cache.run(key, join(root, "out-1"), produce);
  const second = await cache.run(key, join(root, "out-2"), produce);
  expect([first.hit, second.hit, produced]).toEqual([false, true, 1]);
  expect(second.files).toEqual(first.files);
  for (const file of fixture.files) {
    expect(readFileSync(join(root, "out-2", file.path), "utf8")).toBe(file.text);
    expect(first.files.find((row) => row.path === file.path)!.sha256).toBe(noble(file.text));
  }
  const entry = JSON.parse(readFileSync(join(root, "store", key.stage, first.digest, STAGE_CACHE_POLICY_V1.entryFile), "utf8"));
  expect(admitEntry(entry), JSON.stringify(admitEntry.errors)).toBe(true);
  const third = await cache.run({ ...key, tools: { ...key.tools, emitter: "c".repeat(64) } }, join(root, "out-3"), produce);
  expect([third.hit, produced]).toEqual([false, 2]);
  expect(cache.stats()).toMatchObject({ hits: 1, misses: 2 });
});

test("concurrent producers of one key publish one entry and leave no temporary directory", async () => {
  const root = mkdtempSync(join(artifacts, "stage-race-")), cache = stageCacheV1(join(root, "store")), key: StageKeyV1 = fixture.keys[1].key;
  const results = await Promise.all([0, 1, 2, 3].map((index) => cache.run(key, join(root, `out-${index}`), async (directory) => { await Bun.sleep(20); writeFileSync(join(directory, "module.js"), "export default 1;\n"); })));
  expect(new Set(results.map((row) => row.digest)).size).toBe(1);
  expect(readdirSync(join(root, "store", key.stage))).toEqual([results[0]!.digest]);
  for (const index of [0, 1, 2, 3]) expect(readFileSync(join(root, `out-${index}`, "module.js"), "utf8")).toBe("export default 1;\n");
});

test("a failed or cancelled producer stores nothing", async () => {
  const root = mkdtempSync(join(artifacts, "stage-fail-")), cache = stageCacheV1(join(root, "store")), key: StageKeyV1 = fixture.keys[2].key;
  await expect(cache.run(key, join(root, "out"), async () => { throw Error("tool failed"); })).rejects.toThrow("tool failed");
  const controller = new AbortController();
  await expect(cache.run(key, join(root, "out"), async (directory) => { writeFileSync(join(directory, "x"), "x"); controller.abort(Error("cancelled")); }, controller.signal)).rejects.toThrow("cancelled");
  expect(readdirSync(join(root, "store", key.stage))).toEqual([]);
  expect(existsSync(join(root, "out", "x"))).toBe(false);
});
