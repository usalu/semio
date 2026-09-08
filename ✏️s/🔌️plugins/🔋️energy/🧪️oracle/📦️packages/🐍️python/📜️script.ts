#!/usr/bin/env bun
/** 🔮️ `@semio-tech/energy-oracle-py` router — provisions and drives the 🔋️energy plugin's
 * third-party physics oracle (Honeybee → OpenStudio → EnergyPlus).
 *
 * `setup` resolves this host's platform against the pinned asset table in `🔣️.json`, downloads the
 * OpenStudio archive into `.🧬semio/🦑️repo/⚡️cache/oracles/<tool>-<version>-<platform>/`, verifies
 * its sha256, extracts it and materialises the Python 3.12 venv with `uv sync`. `status` prints the
 * resolved paths and the versions the binaries actually report. `run` translates a semio
 * `Model` JSON through Honeybee and simulates it; `native` builds an ASHRAE 140 case from the
 * standard's own geometry as a translator-independent cross-check.
 *
 * @see https://github.com/NREL/OpenStudio/releases/tag/v3.11.0
 * @see ../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/ENERGY-PLUGIN-END-TO-END/📓️w2-oracle-toolchain.md
 */
import { createHash } from "node:crypto";
import { createReadStream, existsSync, mkdirSync, readFileSync, statSync } from "node:fs";
import { arch, platform } from "node:os";
import { join } from "node:path";
import { BundleScript, ScriptRouter, getRepoMetaDir, resolveTestLevel, runBundleScriptMain, runCmd, runProbe, runTestBudgeted } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

type OraclePlatformAsset = { asset: string; bytes: number; sha256: string };
type OracleToolManifest = {
  tool: string;
  version: string;
  bundles: Record<string, string>;
  downloadUrlTemplate: string;
  stripComponents: number;
  binaries: Record<string, string>;
  platforms: Record<string, OraclePlatformAsset>;
};
type OracleManifest = { tools: OracleToolManifest[]; python: { version: string } };

const PACKAGE_DIR = import.meta.dir;
const PYTHON_MODULE = "🧪️oracle.🐍️";

/** 🖥️ `<node platform>-<node arch>` key the pinned asset table is indexed by. */
function oraclePlatformKey(): string {
  return `${platform()}-${arch()}`;
}

/** 📇️ Reads the committed pin table beside this script. */
function oracleManifest(): OracleManifest {
  return JSON.parse(readFileSync(join(PACKAGE_DIR, "🔣️.json"), "utf8")) as OracleManifest;
}

/** 🗂️ `<repo>/.🧬semio/🦑️repo/⚡️cache/oracles/<tool>-<version>-<platform>`. */
function oracleToolDirectory(repoRoot: string, tool: OracleToolManifest): string {
  return join(getRepoMetaDir(repoRoot), "⚡️cache", "oracles", `${tool.tool}-${tool.version}-${oraclePlatformKey()}`);
}

/** 🔐️ Streaming sha256 of a file, hex-encoded. */
async function fileSha256(path: string): Promise<string> {
  const hash = createHash("sha256");
  for await (const chunk of createReadStream(path)) hash.update(chunk as Uint8Array);
  return hash.digest("hex");
}

/** 📥️ Downloads (once) and sha256-verifies the pinned archive, then extracts it in place. */
async function ensureOracleTool(repoRoot: string, tool: OracleToolManifest): Promise<string> {
  const key = oraclePlatformKey();
  const pin = tool.platforms[key];
  if (!pin) throw new Error(`${tool.tool} ${tool.version} has no pinned asset for ${key} (see 🔣️.json)`);
  const directory = oracleToolDirectory(repoRoot, tool);
  const archive = join(directory, pin.asset);
  const marker = join(directory, Object.values(tool.binaries)[0]!);
  mkdirSync(directory, { recursive: true });
  if (!existsSync(archive) || statSync(archive).size !== pin.bytes) {
    const url = tool.downloadUrlTemplate.replace("{asset}", encodeURIComponent(pin.asset));
    console.log(`[oracle] downloading ${pin.asset} (${(pin.bytes / 1e6).toFixed(1)} MB) from ${url}`);
    runCmd("curl", ["-fSL", "--retry", "3", "-o", archive, url], { cwd: directory });
  }
  const digest = await fileSha256(archive);
  if (digest !== pin.sha256) throw new Error(`${pin.asset} sha256 mismatch: expected ${pin.sha256}, got ${digest}`);
  console.log(`[oracle] verified ${pin.asset} sha256 ${digest}`);
  if (!existsSync(marker)) {
    console.log(`[oracle] extracting into ${directory}`);
    runCmd("tar", ["-xzf", archive, "-C", directory, "--strip-components", String(tool.stripComponents)], { cwd: directory });
  }
  if (!existsSync(marker)) throw new Error(`${tool.tool} archive extracted but ${marker} is missing`);
  return directory;
}

/** 🌍️ Environment every Python entry point needs to find the extracted toolchain. */
function oracleEnvironment(repoRoot: string): NodeJS.ProcessEnv {
  const manifest = oracleManifest();
  const resolved: NodeJS.ProcessEnv = { ...process.env };
  for (const tool of manifest.tools) resolved[`SEMIO_ORACLE_${tool.tool.toUpperCase()}_ROOT`] = oracleToolDirectory(repoRoot, tool);
  return resolved;
}

/** 🐍️ Runs the oracle Python module inside the uv-managed venv. */
function runOraclePython(repoRoot: string, args: string[], budgetMs?: number): Promise<void> {
  const inline = `import sys; from importlib import import_module; sys.exit(import_module(${JSON.stringify(PYTHON_MODULE)}).main(sys.argv[1:]))`;
  return runTestBudgeted("uv", ["run", "--locked", "--no-sync", "--project", PACKAGE_DIR, "python", "-c", inline, ...args], { cwd: PACKAGE_DIR, env: oracleEnvironment(repoRoot), budgetMs });
}

/** 📥️ Synchronizes this separately locked Python environment without running application work. */
class DepsScript extends BundleScript {
  run(): void {
    runCmd("uv", ["sync", "--locked", "--project", import.meta.dir], { cwd: import.meta.dir });
  }
}

/** ⚙️ Downloads, verifies, extracts the pinned toolchain and materialises the Python 3.12 venv. */
class SetupScript extends BundleScript {
  async run(): Promise<void> {
    const manifest = oracleManifest();
    for (const tool of manifest.tools) await ensureOracleTool(this.repoRoot, tool);
    await runOraclePython(this.repoRoot, ["status"]);
  }
}

/** 🔍️ Prints the resolved oracle paths and the versions the binaries themselves report. */
class StatusScript extends BundleScript {
  async run(): Promise<void> {
    const manifest = oracleManifest();
    for (const tool of manifest.tools) {
      const directory = oracleToolDirectory(this.repoRoot, tool);
      console.log(`[oracle] ${tool.tool} ${tool.version} -> ${directory}${existsSync(directory) ? "" : " (missing — run setup)"}`);
      for (const [name, relative] of Object.entries(tool.binaries)) {
        const path = join(directory, relative);
        const probe = name === "openstudio" || name === "energyplus" ? runProbe(path, ["--version"]) : null;
        console.log(`[oracle]   ${name}: ${path}${existsSync(path) ? "" : " (missing)"}${probe ? ` -> ${probe.stdout.trim() || probe.stderr.trim()}` : ""}`);
      }
    }
    await runOraclePython(this.repoRoot, ["status"]);
  }
}

/** ▶️ `run <model.json> <weather.epw> <out.json> [--case 600] [--schedules <path>]`. */
class RunScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runOraclePython(this.repoRoot, ["translate", ...segments], 4 * 60 * 60 * 1000);
  }
}

/** 🏛️ `native <case> <weather.epw> <out.json>` — ASHRAE 140 case built directly in honeybee. */
class NativeScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runOraclePython(this.repoRoot, ["native", ...segments], 4 * 60 * 60 * 1000);
  }
}

/** ⚡️ `epjson <file.epJSON> [<weather.epw> <out.json>] [--validate-only]` — the second, honeybee-free
 * oracle route: validate a semio-written epJSON against EnergyPlus's own `Energy+.schema.epJSON`
 * with the third-party `jsonschema` validator, then run EnergyPlus 25.2.0 directly on it. */
class EpJsonScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runOraclePython(this.repoRoot, ["epjson", ...segments], 4 * 60 * 60 * 1000);
  }
}

/** 🧫️ `emit <case> <out.json>` — the oracle's own reading of an ASHRAE 140 case as a semio `Model`. */
class EmitScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runOraclePython(this.repoRoot, ["emit", ...segments]);
  }
}

/** 🧪️ Exercises the translator and the ASHRAE 140 case builder without running EnergyPlus. */
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    resolveTestLevel(segments);
    await runOraclePython(this.repoRoot, ["selftest"]);
  }
}

const router = new ScriptRouter(PACKAGE_DIR).register("deps", DepsScript).register("setup", SetupScript).register("status", StatusScript).register("run", RunScript).register("native", NativeScript).register("epjson", EpJsonScript).register("emit", EmitScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "status" });
