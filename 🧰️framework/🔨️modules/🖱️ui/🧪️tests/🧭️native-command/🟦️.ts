import { expect, mock, test } from "bun:test";
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { resolve, join } from "node:path";

import { Command } from "commander";
import { spawnSync } from "node:child_process";
import { BundleScript, findWorkspaceRoot } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { withBuildTestInvocationV1 } from "@semio-tech/process/testing/invocation";

type Call = { kind: string; args: string[]; level?: string; coverage?: boolean; budgetMs?: number; packages?: string[] };
type Case = { id: string; argv: string[]; inheritedLevel?: string; calls: Call[]; rejected: boolean; level: string; coverage: boolean };
const ui = resolve(import.meta.dir, "../.."), packageRoot = join(ui, "📦️packages/🦀️rust"), root = findWorkspaceRoot(ui);
const corpus = JSON.parse(readFileSync(join(ui, "🧪️tests/🧫️fixtures/🧭️native-command/🔣️.json"), "utf8")) as { cases: Case[] };
let calls: Call[] = [];
let script: typeof import("../../📦️packages/🦀️rust/📜️script.ts");
const cargoModule = resolve(ui, "../🏃️process/🧪️testing/🦀️cargo/🟦️.ts"), cargoActual = await import(cargoModule);
mock.module(cargoModule, () => ({ ...cargoActual, readCargoTestPolicyV1: () => ({}) }));
const providerModule = resolve(ui, "../🏃️process/🧪️testing/🦀️cargo/🔌️provider/🟦️.ts"), providerActual = await import(providerModule);
mock.module(providerModule, () => ({
  ...providerActual,
  runAdmittedCargoTestsV1: async (options: { manifestPath: string; cwd: string; packages: string[]; extraArgs: string[] }, _policy: unknown, workspace: string) => {
    expect(workspace).toBe(root);
    expect(options.manifestPath).toBe(join(packageRoot, "Cargo.toml"));
    expect(options.cwd).toBe(packageRoot);
    expect(options.packages).toEqual(options.extraArgs.includes("--lib") ? script.UI_WGPU_PACKAGES : script.UI_TEST_PACKAGES);
    calls.push({ kind: "test", args: options.extraArgs, level: process.env.SEMIO_TEST_LEVEL, coverage: process.env.SEMIO_COVERAGE === "1" });
  },
  runAdmittedCargoCommandV1: async (_invocation: unknown, _control: unknown, workspace: string, args: string[], cwd: string, _environment: unknown, budgetMs: number) => {
    expect(workspace).toBe(root);
    expect(args.slice(0, 3)).toEqual(["check", "--manifest-path", join(packageRoot, "Cargo.toml")]);
    expect(cwd).toBe(packageRoot);
    const rest = args.slice(3), packages: string[] = [];
    while (rest[0] === "-p") { rest.shift(); packages.push(rest.shift()!); }
    calls.push({ kind: "check", packages, args: rest, budgetMs });
  },
}));
class AxesCheck extends BundleScript { run(): void { calls.push({ kind: "axes", args: [] }); } }
class UnselectedAxes extends BundleScript { run(): void { throw Error("Unselected UI axes command"); } }
mock.module(join(ui, "🎚️axes/📜️script.ts"), () => ({ CheckAxesScript: AxesCheck, GenerateAxesScript: UnselectedAxes, PreviewGeneratedScript: UnselectedAxes }));
script = await import("../../📦️packages/🦀️rust/📜️script.ts");
const { createUiNativeRouter } = script;

function oracle(row: Case): { calls: Call[]; rejected: boolean; level: string; coverage: boolean } {
  const trace: Call[] = [];
  let level = row.inheritedLevel ?? "fundamental", coverage = false;
  const tui = ["semio-framework-ui-tui-core", "semio-framework-ui-tui-vt", "semio-framework-ui-tui-terminal", "semio-framework-ui-tui-widgets", "semio-framework-ui-tui-frame", "semio-framework-ui-tui-engine"];
  const project = (args: string[], head: string[]): void => {
    if (["fundamental", "quick", "long", "exhaustive"].includes(args[0] ?? "")) level = args.shift()!;
    coverage = level === "exhaustive";
    trace.push({ kind: "test", args: [...head, ...args], level, coverage });
  };
  const check = (packages: string[], ...args: string[]): void => { trace.push({ kind: "check", packages, args, budgetMs: 4242 }); };
  const command = new Command().exitOverride().configureOutput({ writeErr() {}, writeOut() {} }).enablePositionalOptions();
  const tests = command.command("test").argument("[args...]").allowUnknownOption().passThroughOptions().action((args: string[]) => project(args, ["--features", "semio-framework-ui-tui-terminal/native,semio-framework-ui-tui-unit-laws/native"]));
  tests.command("wgpu-engine").argument("[args...]").allowUnknownOption().passThroughOptions().action((args: string[]) => project(args, ["--lib"]));
  const checks = command.command("check").action(() => { trace.push({ kind: "axes", args: [] }); });
  checks.command("wasm").action(() => { check(tui, "--target", "wasm32-unknown-unknown"); check(["semio-framework-ui-tui-host"], "--target", "wasm32-unknown-unknown", "--features", "semio-framework-ui-tui-host/bindgen"); check(["semio-framework-ui-wgpu-light", "semio-framework-ui-wgpu-input"], "--target", "wasm32-wasip2"); });
  checks.command("wgpu-engine").action(() => { throw Error("Missing wasm"); }).command("wasm").action(() => check(["semio-framework-ui-wgpu-engine"], "--target", "wasm32-unknown-unknown"));
  let rejected = false;
  try { command.parse(row.argv, { from: "user" }); } catch { rejected = true; }
  return { calls: trace, rejected, level, coverage };
}

test("UI native dispatch corpus is closed and independently admitted", () => {
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
  const rejected = await withBuildTestInvocationV1(async (invocation) => { try { await createUiNativeRouter(packageRoot, root).run(row.argv, invocation); return false; } catch { return true; } });
  const actual = { calls, rejected, level: process.env.SEMIO_TEST_LEVEL, coverage: process.env.SEMIO_COVERAGE === "1" };
  expect(actual).toEqual(expected);
  mkdirSync(artifacts, { recursive: true });
  writeFileSync(join(artifacts, "ui-native-command-" + row.id + ".json"), JSON.stringify({ expected, actual }));
  console.log("[ui-native-command] " + row.id + ": " + (rejected ? "refused" : calls.map(call => call.kind).join(",")));
});
