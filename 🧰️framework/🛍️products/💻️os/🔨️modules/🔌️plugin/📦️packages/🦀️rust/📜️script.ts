#!/usr/bin/env bun
/** 🦀️ Awaited plugin SDK checks and exact-filter native regression tests. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargo, runCargoTestBudgeted } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { parseArgs } from "node:util";
import Ajv from "ajv";

//#region 🧪️RunnerSelection
/** 🎯️ Selects explicit build inventory or the existing budgeted test runner without interpreting filters. */
export function pluginTestInvocation(segments: string[]): { mode: "inventory" | "budgeted"; args: string[] } {
  const { rest } = resolveTestLevel(segments);
  const boundary = rest.indexOf("--");
  const inventory = rest.slice(0, boundary < 0 ? rest.length : boundary).includes("--no-run");
  return inventory ? { mode: "inventory", args: ["test", "--manifest-path", "Cargo.toml", "--lib", ...rest] } : { mode: "budgeted", args: ["--lib", ...rest] };
}

/** 🧪️ Pins exact forwarding against the neutral fixture and Node's independent separator parser. */
export function pluginTestRunnerSelfTests(): number {
  const fixture = JSON.parse(readFileSync(new URL("../../🧪️tests/🏃️runner/🧪️fixture.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("../../🧪️tests/🏃️runner/🧬️schema.json", import.meta.url), "utf8"));
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  assert(validate(fixture), JSON.stringify(validate.errors));
  const level = process.env.SEMIO_TEST_LEVEL, coverage = process.env.SEMIO_COVERAGE;
  try {
    for (const row of fixture.cases) {
      const selected = pluginTestInvocation(row.args);
      assert.equal(selected.mode, row.mode);
      assert.deepEqual(selected.args, row.forwarded);
      const parsed = parseArgs({ args: row.args, strict: false, allowPositionals: true, options: { "no-run": { type: "boolean" } } });
      assert.equal(parsed.values["no-run"] === true ? "inventory" : "budgeted", row.mode);
      assert.equal(validate({ ...fixture, cases: fixture.cases.map((other: object) => other === row ? { ...row, mode: row.mode === "inventory" ? "budgeted" : "inventory" } : other) }), false);
    }
  } finally {
    if (level === undefined) delete process.env.SEMIO_TEST_LEVEL; else process.env.SEMIO_TEST_LEVEL = level;
    if (coverage === undefined) delete process.env.SEMIO_COVERAGE; else process.env.SEMIO_COVERAGE = coverage;
  }
  return fixture.cases.length;
}
//#endregion 🧪️RunnerSelection

//#region 🎯️Tasks
class CheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCargo(["check", "--manifest-path", "Cargo.toml", ...segments], this.root);
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    console.log(`[DEBUG] plugin-runner-oracle cases=${pluginTestRunnerSelfTests()}`);
    const invocation = pluginTestInvocation(segments);
    if (invocation.mode === "inventory") await runCargo(invocation.args, this.root);
    else await runCargoTestBudgeted([], this.root, invocation.args);
  }
}

class CodecSendSourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-codec-send-source accepts no arguments");
    const { testPluginCodecCallerSource } = await import("../../📦️codec/🧵️send/📜️script.ts");
    testPluginCodecCallerSource(this.repoRoot);
  }
}

const router = new ScriptRouter(import.meta.dir).register("check", CheckScript).register("test", TestScript).register("test-codec-send-source", CodecSendSourceScript);
if (import.meta.main) await runBundleScriptMain(router, import.meta.url, { defaultCommand: "check" });
//#endregion 🎯️Tasks
