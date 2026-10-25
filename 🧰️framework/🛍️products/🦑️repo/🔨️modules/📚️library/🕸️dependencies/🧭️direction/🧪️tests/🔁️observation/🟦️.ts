/** 🔁️ Contract: the dependency direction lane takes a new observation when a peer moves a source under a resolver pass, bounded by the churn schema, and lets every other failure stand; an independent Python oracle replays the same worlds. */
import { expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, realpathSync, rmSync, unlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import fixture from "../🧫️fixtures/🔁️observation/🔣️.json";
import schema from "../../🧬️schema/🔁️observation/🔣️.json";
import { ConcurrentEditErrorV1, churnProtocolV1, type ChurnRetryEventV1 } from "@semio-tech/framework-filesystem/observation/churn";
import { testPythonV1 } from "@semio-tech/process/testing/toolchain";
import { dependencyDirectionSourceInventory, type DependencyDirectionGraphScope } from "../../🟦️.ts";
import { observeDependencyDirectionV1 } from "../../🔁️observation/🟦️.ts";

const here = dirname(fileURLToPath(import.meta.url)), python = testPythonV1();
const oracle = JSON.parse(Bun.spawnSync([python!, "-I", join(here, "🐍️.py"), join(here, "..", "🧫️fixtures", "🔁️observation", "🔣️.json"), Bun.resolveSync("@semio-tech/framework-filesystem/observation/churn/schema.json", here)]).stdout.toString()) as Record<string, Record<string, unknown>>;
type Event = Readonly<{ observe?: readonly string[]; observeFails?: string; path?: string; run?: string }>;
const nobody = async (): Promise<void> => {};

async function replay(events: readonly Event[]): Promise<{ verdict: Record<string, unknown>; announcements: ChurnRetryEventV1[] }> {
  const queue = [...events], announcements: ChurnRetryEventV1[] = [];
  let runs = 0;
  const observe = (): { expectedSources: readonly string[] } => {
    const event = queue.shift()!;
    if (event.observeFails) throw Object.assign(new Error(event.observeFails), { code: event.observeFails, path: event.path });
    return { expectedSources: event.observe! };
  };
  const run = async (): Promise<string> => { const event = queue.shift()!; runs++; if (event.run !== "ok") throw new RangeError("forbidden edge"); return "ok"; };
  try {
    await observeDependencyDirectionV1(observe, run, { sleep: nobody, onRetry: (event) => announcements.push(event), environment: {} });
    return { verdict: { result: "ok", runs }, announcements };
  } catch (error) {
    if (error instanceof ConcurrentEditErrorV1) return { verdict: { result: "exhausted", runs, reason: error.reason, path: error.path }, announcements };
    return { verdict: { result: "failed", runs }, announcements };
  }
}

test("scenario table is closed by its schema", () => {
  const validate = new Ajv({ strict: false }).compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  expect(new Set(fixture.scenarios.map((row) => row.id)).size).toBe(fixture.scenarios.length);
});

test("the lane re-observes bounded concurrent moves and lets other failures stand, as the independent oracle does", async () => {
  expect(Object.keys(oracle).sort()).toEqual(fixture.scenarios.map((row) => row.id).sort());
  for (const row of fixture.scenarios) {
    const { verdict } = await replay(row.events as readonly Event[]);
    expect(verdict, row.id).toEqual(row.expected);
    expect(oracle[row.id], row.id).toEqual(row.expected);
  }
});

test("every retry is announced in both languages and the exhausted edit names its source", async () => {
  const row = fixture.scenarios.find((entry) => entry.id === "moves-persist-until-exhausted")!;
  const { announcements } = await replay(row.events as readonly Event[]);
  expect(announcements.length).toBe(churnProtocolV1().profiles.build.maximumAttempts - 1);
  for (const event of announcements) { expect(event.profile).toBe("build"); expect(event.path).toBe("b.ts"); expect(event.text.split("\n").length).toBe(churnProtocolV1().languages.length); }
});

test("a source that a peer deletes while the resolver runs is re-inventoried on the real file system", async () => {
  const base = realpathSync(process.env.SEMIO_TEST_ARTIFACT_DIR || tmpdir());
  mkdirSync(base, { recursive: true });
  const root = realpathSync(mkdtempSync(join(base, "dependency-observation-")));
  try {
    for (const path of ["🧰️framework/a.ts", "🧰️framework/b.ts", "🧰️framework/c/d.ts"]) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), "export {};\n"); }
    const scope: DependencyDirectionGraphScope = { workspaceRoots: ["🧰️framework"], workspacePackages: [], excludedPaths: [], nonFollowedPaths: [], expectedSources: [] };
    const seen: (readonly string[])[] = [];
    const result = await observeDependencyDirectionV1(() => ({ expectedSources: dependencyDirectionSourceInventory(root, ["🧰️framework"], scope) }), async ({ expectedSources }) => {
      seen.push(expectedSources);
      if (seen.length === 1) { unlinkSync(join(root, "🧰️framework/b.ts")); throw new Error("ENOENT: no such file or directory, open 'b.ts'"); }
      return expectedSources;
    }, { sleep: nobody, onRetry: () => {}, environment: {} });
    expect(seen.map((sources) => sources.length)).toEqual([3, 2]);
    expect(result).toEqual(["🧰️framework/a.ts", "🧰️framework/c/d.ts"]);
    rmSync(join(root, "🧰️framework/c"), { recursive: true });
    await expect(observeDependencyDirectionV1(() => ({ expectedSources: dependencyDirectionSourceInventory(root, ["🧰️framework", "🧰️framework/c"], scope) }), async () => "never", { sleep: nobody, onRetry: () => {}, environment: {} })).rejects.toThrow();
  } finally { rmSync(root, { recursive: true, force: true }); }
});
