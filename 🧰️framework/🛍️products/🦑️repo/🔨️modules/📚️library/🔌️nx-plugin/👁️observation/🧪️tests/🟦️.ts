/** 👁️ Inference observation law over `../🧫️fixtures/🔣️.json`: a persisted inference result is reused exactly while every filesystem fact the
 * computation observed still holds, identical rewrites stay hits, any observed change is a miss, and a hit always equals a fresh recomputation.
 * Oracles: Ajv validates every persisted record against `../🧬️schema/🔣️.json`; fast-glob independently lists the probed directory. */
import { expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import Ajv from "ajv";
import fastGlob from "fast-glob";

type Scenario = { id: string; mutation: { op: "none" | "write" | "replace-directory-with-file"; path?: string; content?: string }; expected: "hit" | "miss" };
const fixture = JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json", import.meta.url), "utf8")) as { schemaVersion: number; files: Record<string, string>; scenarios: Scenario[] };
const validate = new Ajv({ strict: true }).compile(JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json", import.meta.url), "utf8")));
const { observedFs, observeInference } = await import(new URL("../🟨️.mjs", import.meta.url).href) as typeof import("../🟨️.mjs");

/** 🧪️ One isolated workspace whose files are older than the racy window, so the first result is persisted. */
function workspace(): string {
  const parent = process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir();
  mkdirSync(parent, { recursive: true });
  const root = mkdtempSync(join(realpathSync(parent), "io-"));
  for (const [path, content] of Object.entries(fixture.files)) { mkdirSync(join(root, path, ".."), { recursive: true }); writeFileSync(join(root, path), content); }
  Bun.sleepSync(2200);
  return root;
}

/** 🔍️ The probe program: one read, one absence, one listing and one type probe, all through the observed filesystem. */
function probe(root: string): Readonly<Record<string, unknown>> {
  return { one: observedFs.readFileSync(join(root, "a/one.txt"), "utf8"), missing: observedFs.existsSync(join(root, "c/missing.txt")), listing: observedFs.readdirSync(join(root, "a")).sort(), bIsDirectory: observedFs.statSync(join(root, "b")).isDirectory() };
}

test("fixture is closed and versioned", () => {
  expect(fixture.schemaVersion).toBe(1);
  expect(new Set(fixture.scenarios.map((row) => row.id)).size).toBe(fixture.scenarios.length);
});

for (const scenario of fixture.scenarios) test(`observed inference reuse: ${scenario.id}`, async () => {
  const root = workspace(), cachePath = join(root, ".cache", "inference.json");
  let computations = 0;
  const compute = (): Readonly<Record<string, unknown>> => { computations++; return probe(root); };
  const first = await observeInference({ cachePath, key: ["law", scenario.id] }, compute);
  expect(computations).toBe(1);
  const header = JSON.parse(readFileSync(cachePath, "utf8").split("\n")[0]!);
  expect(validate(header), JSON.stringify(validate.errors)).toBe(true);
  expect(first.listing).toEqual((await fastGlob("*", { cwd: join(root, "a"), onlyFiles: false })).sort());
  if (scenario.mutation.op === "write") { mkdirSync(join(root, scenario.mutation.path!, ".."), { recursive: true }); writeFileSync(join(root, scenario.mutation.path!), scenario.mutation.content!); }
  if (scenario.mutation.op === "replace-directory-with-file") { rmSync(join(root, scenario.mutation.path!), { recursive: true }); writeFileSync(join(root, scenario.mutation.path!), "file"); }
  const second = await observeInference({ cachePath, key: ["law", scenario.id] }, compute);
  expect(computations).toBe(scenario.expected === "hit" ? 1 : 2);
  expect(second).toEqual(probe(root));
  console.log(`[DEBUG] inference observation ${scenario.id}: ${scenario.expected} computations=${computations} observations=${header.observations.length}`);
  rmSync(root, { recursive: true, force: true });
}, 20_000);

test("a result that is not exactly JSON-representable is never persisted", async () => {
  const root = workspace(), cachePath = join(root, ".cache", "inexact.json");
  let computations = 0;
  for (let round = 0; round < 2; round++) await observeInference({ cachePath, key: ["inexact"] }, () => { computations++; return { value: undefined, file: observedFs.readFileSync(join(root, "a/two.txt"), "utf8") }; });
  expect(computations).toBe(2);
  rmSync(root, { recursive: true, force: true });
}, 20_000);

const { memoizedFileRecord, observationsHold, observeDependencies } = await import(new URL("../🟨️.mjs", import.meta.url).href) as typeof import("../🟨️.mjs");

for (const scenario of fixture.scenarios) test(`non-blocking validation agrees with reuse: ${scenario.id}`, async () => {
  const root = workspace(), cachePath = join(root, ".cache", "inference.json");
  await observeInference({ cachePath, key: ["hold", scenario.id] }, () => probe(root));
  const header = JSON.parse(readFileSync(cachePath, "utf8").split("\n")[0]!) as { observations: unknown[] };
  if (scenario.mutation.op === "write") { mkdirSync(join(root, scenario.mutation.path!, ".."), { recursive: true }); writeFileSync(join(root, scenario.mutation.path!), scenario.mutation.content!); }
  if (scenario.mutation.op === "replace-directory-with-file") { rmSync(join(root, scenario.mutation.path!), { recursive: true }); writeFileSync(join(root, scenario.mutation.path!), "file"); }
  expect(await observationsHold(header.observations), scenario.id).toBe(scenario.expected === "hit");
  rmSync(root, { recursive: true, force: true });
}, 20_000);

test("a per-file analysis is memoized across inference misses by content digest and recomputed when the bytes change", async () => {
  const root = workspace(), cachePath = join(root, ".cache", "memo.json"), file = join(root, "a/one.txt");
  let analyses = 0;
  const analyse = (bytes: Buffer): Readonly<{ length: number; upper: string }> => { analyses++; return { length: bytes.byteLength, upper: bytes.toString("utf8").toUpperCase() }; };
  const run = (key: string) => observeInference({ cachePath, key: ["memo", key] }, () => ({ fact: memoizedFileRecord("law-analysis-v1", file, analyse) }));
  const first = await run("one");
  const second = await run("two");
  expect(second).toEqual(first);
  expect(analyses).toBe(1);
  const header = JSON.parse(readFileSync(cachePath, "utf8").split("\n")[0]!) as { observations: { kind: string; path: string }[] };
  expect(header.observations.some((row) => row.kind === "file" && row.path === file)).toBe(true);
  writeFileSync(file, "changed bytes\n");
  Bun.sleepSync(2200);
  const third = await run("three");
  expect(analyses).toBe(2);
  expect(third).toEqual({ fact: analyse(Buffer.from("changed bytes\n")) });
  rmSync(root, { recursive: true, force: true });
}, 30_000);

test("a dependency receipt names the persisted inference records this process used, and is withdrawn when an input is racy", async () => {
  const root = workspace(), cachePath = join(root, ".cache", "nodes.json"), receiptPath = join(root, ".cache", "nodes.receipt.json");
  await observeInference({ cachePath, key: ["receipt"] }, () => probe(root));
  await observeDependencies({ receiptPath }, () => observedFs.readFileSync(join(root, "a/two.txt"), "utf8"));
  const receipt = JSON.parse(readFileSync(receiptPath, "utf8")) as { pid: number; inference: [string, string][]; observations: { path: string }[] };
  expect(receipt.pid).toBe(process.pid);
  expect(receipt.inference.some(([path]) => path === cachePath)).toBe(true);
  expect(receipt.observations.some((row) => row.path === join(root, "a/two.txt"))).toBe(true);
  writeFileSync(join(root, "a/two.txt"), "fresh\n");
  await observeDependencies({ receiptPath }, () => observedFs.readFileSync(join(root, "a/two.txt"), "utf8"));
  expect(() => readFileSync(receiptPath)).toThrow();
  rmSync(root, { recursive: true, force: true });
}, 30_000);
