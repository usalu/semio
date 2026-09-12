import { existsSync, readFileSync } from "node:fs";
import { delimiter, join, resolve } from "node:path";
import { BundleScript, resolveTestLevel, runCmd, runProbe, runTestBudgeted } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { ensureOracleTool, oracleEnvironment, oracleManifest, oracleToolDirectory } from "../🛠️toolchain/🟦️.ts";

const PYTHON_OWNER = "✏️s/🔌️plugins/🔋️energy/🔮️oracles/🏃️execution";

function oraclePythonModule(packageRoot: string): string {
  const contribution = JSON.parse(readFileSync(resolve(packageRoot, "../..", "🔣️.json"), "utf8"));
  const host = contribution.oracleHostPackages?.find((entry: any) => entry.implementation === "python" && entry.path === PYTHON_OWNER);
  if (host?.module !== "🐍️") throw new Error("Energy oracle Python source/module ownership is not canonical");
  return host.module;
}

/** 🐍️ Runs the anonymous Python source through its locked uv package environment. */
export function runOraclePython(repoRoot: string, packageRoot: string, args: string[], budgetMs?: number): Promise<void> {
  const module = oraclePythonModule(packageRoot);
  const inline = `import sys; from importlib import import_module; sys.exit(import_module(${JSON.stringify(module)}).main(sys.argv[1:]))`;
  const env = oracleEnvironment(repoRoot, packageRoot);
  env.PYTHONPATH = [join(repoRoot, PYTHON_OWNER), env.PYTHONPATH ?? ""].filter(Boolean).join(delimiter);
  env.PYTHONDONTWRITEBYTECODE = "1";
  return runTestBudgeted("uv", ["run", "--locked", "--no-sync", "--project", packageRoot, "python", "-c", inline, ...args], { cwd: packageRoot, env, budgetMs });
}

export class DepsScript extends BundleScript {
  run(): void {
    runCmd("uv", ["sync", "--locked", "--project", this.root], { cwd: this.root });
  }
}

export class SetupScript extends BundleScript {
  async run(): Promise<void> {
    for (const tool of oracleManifest(this.root).tools) await ensureOracleTool(this.repoRoot, tool);
    await runOraclePython(this.repoRoot, this.root, ["status"]);
  }
}

export class StatusScript extends BundleScript {
  async run(): Promise<void> {
    const manifest = oracleManifest(this.root);
    for (const tool of manifest.tools) {
      const directory = oracleToolDirectory(this.repoRoot, tool);
      console.log(`[oracle] ${tool.tool} ${tool.version} -> ${directory}${existsSync(directory) ? "" : " (missing — run setup)"}`);
      for (const [name, relative] of Object.entries(tool.binaries)) {
        const path = join(directory, relative);
        const probe = name === "openstudio" || name === "energyplus" ? runProbe(path, ["--version"]) : null;
        console.log(`[oracle]   ${name}: ${path}${existsSync(path) ? "" : " (missing)"}${probe ? ` -> ${probe.stdout.trim() || probe.stderr.trim()}` : ""}`);
      }
    }
    await runOraclePython(this.repoRoot, this.root, ["status"]);
  }
}

export class RunScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runOraclePython(this.repoRoot, this.root, ["translate", ...segments], 4 * 60 * 60 * 1000);
  }
}

export class NativeScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runOraclePython(this.repoRoot, this.root, ["native", ...segments], 4 * 60 * 60 * 1000);
  }
}

export class EpJsonScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runOraclePython(this.repoRoot, this.root, ["epjson", ...segments], 4 * 60 * 60 * 1000);
  }
}

export class EmitScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runOraclePython(this.repoRoot, this.root, ["emit", ...segments]);
  }
}

export class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    resolveTestLevel(segments);
    await runOraclePython(this.repoRoot, this.root, ["selftest"]);
  }
}
