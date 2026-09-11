#!/usr/bin/env bun
import { readdirSync } from "node:fs";
import { join } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain, runCmd } from "../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { repoCacheDirectory } from "../../../📚️library/⚡️caching/🟦️.ts";
import { stageArtifacts } from "../../../📚️library/⚡️caching/📦️artifacts/🟦️.ts";

const project = "🧪️Semio.Repo.Test.csproj";
const nativeState = (root: string): string => repoCacheDirectory(root, "dotnet", "repo-test");

/** 📥️ Restores the locked support-library dependencies without compiling consumers. */
class DepsScript extends BundleScript {
  run(): void {
    runCmd("dotnet", ["restore", project, "--locked-mode", "--artifacts-path", nativeState(this.repoRoot)], { cwd: import.meta.dir });
  }
}

/** 🔷️ Publishes only the support-library deliverables after a successful incremental build. */
class BuildScript extends BundleScript {
  run(args: string[]): void {
    if (args.length) throw new Error("The .NET support library has one Release artifact contract");
    const output = join(nativeState(this.repoRoot), "deliverables");
    runCmd("dotnet", ["build", project, "--no-restore", "--configuration", "Release", "--artifacts-path", nativeState(this.repoRoot), "--output", output, `-p:PathMap=${this.repoRoot}=/_/`, "-p:ContinuousIntegrationBuild=true"], { cwd: import.meta.dir });
    stageArtifacts(join(import.meta.dir, "dist/build"), "@semio-tech/repo-test-dotnet:build", new Map(readdirSync(output, { withFileTypes: true }).filter((entry) => entry.isFile()).map((entry) => [entry.name, join(output, entry.name)])));
  }
}

const router = new ScriptRouter(import.meta.dir).register("deps", DepsScript).register("build", BuildScript);
if (import.meta.main) await runBundleScriptMain(router, import.meta.url);
