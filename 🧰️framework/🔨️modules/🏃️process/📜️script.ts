#!/usr/bin/env bun
/** 🏃️ Runs neutral process ownership and execution contracts. */
import { resolve } from "node:path";
import { mkdirSync } from "node:fs";
import { spawn } from "node:child_process";
import { BundleScript, ScriptRouter } from "./🧭️routing/🟦️.ts";
import { runScriptMain } from "./🧭️routing/🚪️entrypoint/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length !== 1) throw Error("Expected test routing or artifact-files");
    const output = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR ?? resolve(this.root, "🗑️generated"));
    mkdirSync(output, { recursive: true });
    if (segments[0] === "artifact-files") {
      const { testArtifactFiles } = await import("./📦️artifacts/🗂️files/🧪️tests/🟦️.ts");
      await testArtifactFiles(output);
      return;
    }
    if (segments[0] === "test-budget") {
      const child = spawn(process.execPath, ["test", resolve(this.root, "🧪️testing/🎚️budget/🧪️tests/🟦️.ts")], { cwd: this.repoRoot, env: { ...process.env, SEMIO_TEST_ARTIFACT_DIR: output }, stdio: "inherit" });
      const status = await new Promise<number>((accept, reject) => { child.once("error", reject); child.once("close", (code) => accept(code ?? 1)); });
      if (status) throw Error(`test budget contract exited with status ${status}`);
      return;
    }
    if (segments[0] !== "routing") throw Error("Expected test routing or artifact-files");
    const child = spawn(process.execPath, ["test", resolve(this.root, "🧭️routing/🧪️tests/🟦️.ts")], { cwd: this.repoRoot, env: { ...process.env, SEMIO_TEST_ARTIFACT_DIR: output }, stdio: "inherit" });
    const status = await new Promise<number>((accept, reject) => { child.once("error", reject); child.once("close", (code) => accept(code ?? 1)); });
    if (status) throw Error(`routing contract exited with status ${status}`);
  }
}

if (import.meta.main) await runScriptMain(new ScriptRouter(import.meta.dir).register("test", TestScript));
