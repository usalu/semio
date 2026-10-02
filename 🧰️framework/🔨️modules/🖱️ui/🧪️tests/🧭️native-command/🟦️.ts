import { expect, mock, test } from "bun:test";
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { resolve, join } from "node:path";
import Ajv from "ajv";
import { Command } from "commander";
import { spawnSync } from "node:child_process";
import { BundleScript, findWorkspaceRoot } from "../../../🏃️process/🧭️routing/🟦️.ts";

type Call = { kind: string; args: string[]; level?: string; coverage?: boolean; budgetMs?: number };
type Case = { id: string; argv: string[]; inheritedLevel?: string; calls: Call[]; rejected: boolean; level: string; coverage: boolean };
const ui = resolve(import.meta.dir, "../.."), packageRoot = join(ui, "📦️packages/🦀️rust"), root = findWorkspaceRoot(ui);
const corpus = JSON.parse(readFileSync(join(ui, "🧫️fixtures/🧭️native-command/🔣️.json"), "utf8")) as { cases: Case[] };
const validate = new Ajv({ strict: true }).compile(JSON.parse(readFileSync(join(ui, "🧬️schema/🧭️native-command/🔣️.json"), "utf8")));
let calls: Call[] = [];
mock.module(resolve(ui, "../🏃️process/🧪️testing/🦀️cargo/🟦️.ts"), () => ({
  readCargoTestPolicyV1: () => ({}),
  runCargoTestsV1: async (options: { manifestPath: string; cwd: string; packages: string[]; extraArgs: string[] }) => {
    expect(options.manifestPath).toBe(join(packageRoot, "Cargo.toml"));
    expect(options.cwd).toBe(packageRoot);
    expect(options.packages).toEqual([]);
    calls.push({ kind: "test", args: options.extraArgs, level: process.env.SEMIO_TEST_LEVEL, coverage: process.env.SEMIO_COVERAGE === "1" });
  },
}));
mock.module(resolve(ui, "../🏃️process/🎛️owned-execution/🟦️.ts"), () => ({
  runOwnedCommand: async (command: string, args: string[], cwd: string, owner: string, budgetMs: number) => {
    expect(command).toBe("cargo");
    expect(args.slice(0, 5)).toEqual(["check", "--manifest-path", join(packageRoot, "Cargo.toml"), "-p", "semio-framework-ui"]);
    expect(cwd).toBe(packageRoot);
    expect(owner).toBe("tool:owner");
    calls.push({ kind: "check", args: args.slice(5), budgetMs });
  },
}));
class AxesCheck extends BundleScript { run(): void { calls.push({ kind: "axes", args: [] }); } }
class UnselectedAxes extends BundleScript { run(): void { throw Error("Unselected UI axes command"); } }
mock.module(join(ui, "🎚️axes/🏃️execution/🟦️.ts"), () => ({ CheckAxesScript: AxesCheck, GenerateAxesScript: UnselectedAxes, PreviewGeneratedScript: UnselectedAxes }));
const { createUiNativeRouter } = await import("../../📦️packages/🦀️rust/📜️script.ts");

function oracle(row: Case): { calls: Call[]; rejected: boolean; level: string; coverage: boolean } {
  const trace: Call[] = [];
  let level = row.inheritedLevel ?? "fundamental", coverage = false;
  const project = (args: string[], features: string, library = false): void => {
    if (["fundamental", "quick", "long", "exhaustive"].includes(args[0] ?? "")) level = args.shift()!;
    coverage = level === "exhaustive";
    trace.push({ kind: "test", args: ["--features", features, ...(library ? ["--lib"] : []), ...args], level, coverage });
  };
  const check = (target: string, features: string): void => { trace.push({ kind: "check", args: ["--target", target, "--features", features], budgetMs: 4242 }); };
  const command = new Command().exitOverride().configureOutput({ writeErr() {}, writeOut() {} }).enablePositionalOptions();
  const tests = command.command("test").argument("[args...]").allowUnknownOption().passThroughOptions().action((args: string[]) => project(args, "tui-terminal,wgpu"));
  tests.command("wgpu-engine").argument("[args...]").allowUnknownOption().passThroughOptions().action((args: string[]) => project(args, "wgpu-engine", true));
  const checks = command.command("check").action(() => { trace.push({ kind: "axes", args: [] }); });
  checks.command("wasm").action(() => { check("wasm32-unknown-unknown", "tui"); check("wasm32-unknown-unknown", "tui-bindgen"); check("wasm32-wasip2", "wgpu"); });
  checks.command("wgpu-engine").action(() => { throw Error("Missing wasm"); }).command("wasm").action(() => check("wasm32-unknown-unknown", "wgpu-engine"));
  let rejected = false;
  try { command.parse(row.argv, { from: "user" }); } catch { rejected = true; }
  return { calls: trace, rejected, level, coverage };
}

test("UI native dispatch corpus is closed and independently admitted", () => {
  expect(validate(corpus), JSON.stringify(validate.errors)).toBe(true);
  expect(new Set(corpus.cases.map(row => row.id)).size).toBe(corpus.cases.length);
});

test("independent Node Commander emits the portable command receipts", () => {
  const artifacts = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifacts) throw Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned ticket output");
  const source = "import { Command } from 'commander'; const oracle = " + oracle.toString() + "; console.log(JSON.stringify(" + JSON.stringify(corpus.cases) + ".map(oracle)));";
  const child = spawnSync("node", ["--input-type=module", "--eval", source], { cwd: root, encoding: "utf8" });
  expect(child.status, child.stderr).toBe(0);
  expect(JSON.parse(child.stdout)).toEqual(corpus.cases.map(row => ({ calls: row.calls, rejected: row.rejected, level: row.level, coverage: row.coverage })));
  mkdirSync(artifacts, { recursive: true });
  writeFileSync(join(artifacts, "ui-native-command-node-oracle.json"), child.stdout);
});

for (const row of corpus.cases) test(row.id, async () => {
  const artifacts = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifacts) throw Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned ticket output");
  const expected = { calls: row.calls, rejected: row.rejected, level: row.level, coverage: row.coverage };
  expect(oracle(row)).toEqual(expected);
  calls = [];
  process.env.SEMIO_TEST_LEVEL = row.inheritedLevel ?? "fundamental";
  delete process.env.SEMIO_COVERAGE;
  process.env.SEMIO_BUILD_BUDGET_MS = "4242";
  let rejected = false;
  try { await createUiNativeRouter(packageRoot, root).run(row.argv); } catch { rejected = true; }
  const actual = { calls, rejected, level: process.env.SEMIO_TEST_LEVEL, coverage: process.env.SEMIO_COVERAGE === "1" };
  expect(actual).toEqual(expected);
  mkdirSync(artifacts, { recursive: true });
  writeFileSync(join(artifacts, "ui-native-command-" + row.id + ".json"), JSON.stringify({ expected, actual }));
  console.log("[ui-native-command] " + row.id + ": " + (rejected ? "refused" : calls.map(call => call.kind).join(",")));
});
