import { readdirSync, rmSync } from "node:fs";
import { delimiter, join, resolve } from "node:path";
import { BundleScript, resolveTestLevel, runCmd, runTestBudgeted } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { repoCacheDirectory } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts";

const stylingOwnerRoot = resolve(import.meta.dir, "..");
const pythonBuildRoot = (repoRoot: string): string => process.env.SEMIO_STYLING_PYTHON_BUILD_ROOT ?? repoCacheDirectory(repoRoot, "python", "ui-styling");
const pythonWheelPath = (repoRoot: string): string => {
  const wheels = readdirSync(pythonBuildRoot(repoRoot)).filter((name) => name.endsWith(".whl"));
  if (wheels.length !== 1) throw new Error(`Expected one styling wheel, got ${wheels.length}`);
  return join(pythonBuildRoot(repoRoot), wheels[0]!);
};
const dotnetState = (repoRoot: string): string => process.env.SEMIO_STYLING_DOTNET_ARTIFACTS_ROOT ?? repoCacheDirectory(repoRoot, "dotnet", "ui-styling");

/** 📥️ Synchronizes the separately locked Python package environment. */
export class StylingPythonDepsScript extends BundleScript {
  run(): void {
    runCmd("uv", ["sync", "--locked", "--project", this.root], { cwd: this.root });
  }
}

/** 🎨️ Confirms that the shared styling generator dependency completed. */
export class StylingPythonGenerateScript extends BundleScript {
  run(): void {
    console.log("[nx-generate] styling artifacts ready");
  }
}

/** 📦️ Builds the Python wheel directly from the neutral generated token source. */
export class StylingPythonBuildScript extends BundleScript {
  run(): void {
    const output = pythonBuildRoot(this.repoRoot);
    rmSync(output, { recursive: true, force: true });
    runCmd("uv", ["build", "--wheel", "--project", this.root, "--out-dir", output], { cwd: this.root });
    console.log(`Built ${pythonWheelPath(this.repoRoot)}`);
  }
}

/** 🧪️ Imports the canonical Python source and its wheel projection. */
export class StylingPythonTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    resolveTestLevel(segments);
    const sourceAssertion = "from importlib import import_module; styling = import_module('🔤️tokens.🐍️'); assert styling.BOARD_LIGHT; assert styling.STYLING_TOKENS['primary']; print('styling Python source import resolved')";
    const packageAssertion = "from importlib import import_module; styling = import_module('🎨️styling.🐍️'); assert styling.BOARD_LIGHT; assert styling.STYLING_TOKENS['primary']; print('styling Python wheel import resolved')";
    await runTestBudgeted("uv", ["run", "--locked", "--no-sync", "python", "-c", sourceAssertion], { cwd: this.root, env: { ...process.env, PYTHONPATH: [stylingOwnerRoot, process.env.PYTHONPATH ?? ""].filter(Boolean).join(delimiter) } });
    await runTestBudgeted("uv", ["run", "--locked", "--no-sync", "python", "-c", `import sys; sys.path.insert(0, sys.argv[1]); ${packageAssertion}`, pythonWheelPath(this.repoRoot)], { cwd: this.repoRoot });
  }
}

/** 📥️ Restores the dependency-free styling project into isolated native state. */
export class StylingDotnetDepsScript extends BundleScript {
  run(): void {
    runCmd("dotnet", ["restore", "🔷️.csproj", "--artifacts-path", dotnetState(this.repoRoot)], { cwd: this.root });
  }
}

/** 🔷️ Builds the .NET package directly from the neutral generated palette source. */
export class StylingDotnetBuildScript extends BundleScript {
  run(args: string[]): void {
    if (args.length) throw new Error("The .NET styling package has one Release build contract");
    runCmd("dotnet", ["build", "🔷️.csproj", "--no-restore", "--configuration", "Release", "--artifacts-path", dotnetState(this.repoRoot), `-p:PathMap=${this.repoRoot}=/_/`, "-p:ContinuousIntegrationBuild=true"], { cwd: this.root });
  }
}
