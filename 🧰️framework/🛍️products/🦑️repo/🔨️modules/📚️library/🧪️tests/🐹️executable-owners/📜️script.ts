#!/usr/bin/env bun
import { mkdirSync } from "node:fs";
import { resolve } from "node:path";
import { Script, ScriptRouter, receiveScriptProcessInvocation, runScriptMain, scriptInvocationBudget } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runOwnedCommand } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";

/** 🐹️ Runs the repository-owned executable integration and its neutral ownership admission law. */
class TestScript extends Script {
  async run(segments: string[]): Promise<void> {
    if (segments.length !== 1 || !["ownership", "runtime"].includes(segments[0]!)) throw new Error("Expected test ownership|runtime");
    if (!process.env.SEMIO_TEST_ARTIFACT_DIR) throw new Error("Explicit repository executable artifact directory required");
    mkdirSync(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR), { recursive: true });
    const ownership = segments[0] === "ownership", source = resolve(import.meta.dir, ownership ? "🧭️ownership/🟦️.ts" : "🟦️.ts");
    await runOwnedCommand(process.execPath, ["test", source], this.repoRoot, ownership ? "repo:go-executable-ownership" : "repo:go-executable-acceptance", scriptInvocationBudget(this.invocation, ownership ? 15000 : 300000), { signal: this.invocation.control.signal });
  }
}

if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dir, resolve(import.meta.dir, "../../../../../../..")).register("test", TestScript), { invocation: original }));
