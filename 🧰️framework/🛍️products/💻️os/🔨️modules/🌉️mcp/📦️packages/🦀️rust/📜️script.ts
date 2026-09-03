#!/usr/bin/env bun
/** 🌉️ `@semio-tech/framework-os-mcp-rs` task router: `bun ./📜️script.ts <build|check|test|dev>`. */
import { readFileSync, statSync } from "node:fs";
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargo, runCargoTestBudgeted, runCmd } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { MCP_BINARY_NAME, MCP_CARGO_PACKAGE, resolveBuiltMcpBinaryPath, resolveMcpTargetDirectory } from "../../🟦️.ts";

const binaryContract = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🧱️binary-gate.json", import.meta.url), "utf8")) as { cargoPackage: string; cargoBinary: string; profile: "debug" };
if (binaryContract.cargoPackage !== MCP_CARGO_PACKAGE || binaryContract.cargoBinary !== MCP_BINARY_NAME || binaryContract.profile !== "debug") throw new Error("semio-os-mcp binary fixture disagrees with the shared path contract");

class BuildScript extends BundleScript {
  run(): void {
    const targetDirectory = resolveMcpTargetDirectory(this.repoRoot);
    runCargo(["build", "--manifest-path", "Cargo.toml", "--package", binaryContract.cargoPackage, "--bin", binaryContract.cargoBinary, "--target-dir", targetDirectory], this.root);
    const binary = resolveBuiltMcpBinaryPath(this.repoRoot);
    if (!statSync(binary).isFile()) throw new Error(`cargo succeeded without producing ${binary}`);
    console.log(`[build] ${binary}`);
  }
}

class CheckScript extends BundleScript {
  run(): void {
    runCargo(["check", "--manifest-path", "Cargo.toml"], this.root);
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["semio-framework-os-mcp"], this.repoRoot, rest);
  }
}

/** ▶️ `bun ./📜️script.ts dev [-- stdio [flags...]]` — boots the real stdio server for local/manual
 *  smoke testing (`printf '<json-rpc line>' | bun ./📜️script.ts dev -- stdio | ...`). Defaults to
 *  `stdio` when no mode is given, matching `📦️bin.rs`'s own default-less argv contract. */
class DevScript extends BundleScript {
  run(segments: string[]): void {
    const args = segments.length > 0 ? segments : ["stdio"];
    runCmd("cargo", ["run", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-mcp", "--bin", "semio-os-mcp", "--", ...args], { cwd: this.root });
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("check", CheckScript).register("test", TestScript).register("dev", DevScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "check" });
