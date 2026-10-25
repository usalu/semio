import { afterAll, expect, test } from "bun:test";
import { spawn, spawnSync } from "node:child_process";
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { testPythonV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import corpus from "./🔣️.json" with { type: "json" };

type Run = Readonly<{ stdout: string; stderr: string; status: number | null }>;

const entry = resolve(import.meta.dir, "../../📜️script.ts"), noise = resolve(import.meta.dir, "🎭️noise/🟦️.ts"), waiting = resolve(import.meta.dir, "🎭️wait/🟦️.ts");
const roots: string[] = [], fresh = (): string => { const root = mkdtempSync(join(tmpdir(), "output-contract-")); roots.push(root); return root; };
afterAll(() => { for (const root of roots) rmSync(root, { recursive: true, force: true }); });
const lines = (text: string): string[] => text.split("\n").filter((line) => line !== "");
const parses = (text: string): boolean => {
  const answer = spawnSync(testPythonV1() ?? "python3", ["-I", "-c", "import json,sys\njson.loads(sys.stdin.read())"], { input: text, encoding: "utf8" });
  return answer.status === 0;
};
const start = (argv: readonly string[], env: Record<string, string> = {}, watch?: (stderr: string) => void): Promise<Run> => new Promise((accept, reject) => {
  const child = spawn(process.execPath, argv, { env: { ...process.env, ...env }, stdio: ["ignore", "pipe", "pipe"] });
  let stdout = "", stderr = "";
  child.stdout.setEncoding("utf8").on("data", (text: string) => { stdout += text; });
  child.stderr.setEncoding("utf8").on("data", (text: string) => { stderr += text; watch?.(stderr); });
  child.once("error", reject);
  child.once("close", (status) => accept({ stdout, stderr, status }));
});

for (const row of corpus.door) {
  test(`door: ${row.name}`, async () => {
    const result = await start(["--preload", noise, entry, ...row.args]);
    expect(result.status, result.stderr).toBe(0);
    if (row.machine) {
      expect(result.stderr, "the noise went to the diagnostic stream").toContain("[noise] console.log");
      expect(result.stdout, "no noise on the result stream").not.toContain("[noise]");
      expect(parses(result.stdout), "the whole result stream is one JSON document").toBe(true);
    } else {
      expect(result.stdout, "without the flag nothing is diverted").toContain("[noise] console.log");
    }
  }, 60_000);
}

test(`wait: ${corpus.wait.name}`, async () => {
  const root = fresh(), release = join(root, "release");
  let released = false;
  const result = await start([waiting, ...corpus.wait.args], { FIXTURE_CAPACITY_DIRECTORY: join(root, "capacity"), FIXTURE_CAPACITY_RELEASE: release }, (stderr) => {
    if (!released && stderr.includes("[capacity] waiting")) { released = true; writeFileSync(release, "go"); }
  });
  expect(released, "the fake governor forced a wait").toBe(true);
  expect(result.status, result.stderr).toBe(0);
  expect(lines(result.stdout)).toEqual(corpus.wait.stdout);
  expect(parses(result.stdout)).toBe(true);
  for (const part of corpus.wait.stderr) expect(result.stderr, part).toContain(part);
}, 60_000);
