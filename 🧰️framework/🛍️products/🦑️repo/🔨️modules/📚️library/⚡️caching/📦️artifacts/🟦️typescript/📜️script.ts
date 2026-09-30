#!/usr/bin/env bun
import assert from "node:assert/strict";
import { copyFileSync, existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve, sep } from "node:path";
import { createRequire } from "node:module";
import { BundleScript, ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { getWorkspaceRoot } from "../../../🗂️workspaces/🟦️.ts";
import { runOwnedCommand } from "../../../🏃️process/🎛️owned-execution/🟦️.ts";

/** 🧪️ The artifact's own TypeScript suites, relative to the artifact root: `check` type-checks them and `test` also runs
 * them with `bun test`, so a suite is registered here once instead of being discovered by a filename convention. */
export interface ArtifactTypeScriptPackageOptions {
  readonly suites?: readonly string[];
}

/** 🟦️ Builds and resolves a declaration-only TypeScript artifact package from its taxonomy source. */
export async function runArtifactTypeScriptPackageMain(packageRoot: string, packageName: string, options: ArtifactTypeScriptPackageOptions = {}): Promise<void> {
  const source = resolve(packageRoot, "../../🟦️.ts"), output = resolve(packageRoot, "dist");
  const typeScript = async (entry: string, args: string[], skipLibraries = true): Promise<void> => {
    await runOwnedCommand(process.execPath, ["x", "tsc", entry, ...args, "--module", "ESNext", "--moduleResolution", "Bundler", "--resolveJsonModule", "--allowSyntheticDefaultImports", "--strict", ...(skipLibraries ? ["--skipLibCheck"] : []), "--target", "ES2022"], getWorkspaceRoot(), `artifact-typescript:${packageName}:tsc`, 120_000);
  };
  const suites = (options.suites ?? []).map((suite) => {
    const path = resolve(packageRoot, "../..", suite);
    if (!existsSync(path)) throw new Error(`${packageName} registers the missing suite ${suite}`);
    return path;
  });
  const checkSuites = async (): Promise<void> => {
    for (const suite of suites) await typeScript(suite, ["--noEmit", "--allowImportingTsExtensions"]);
  };
  const copyDeclarationAssets = (): number => {
    const declaration = join(output, "🟦️.d.ts"), compiler = createRequire(import.meta.url)("typescript");
    const copied = new Set<string>();
    for (const imported of compiler.preProcessFile(readFileSync(declaration, "utf8"), true, true).importedFiles) {
      if (!imported.fileName.startsWith(".")) continue;
      const sourceImport = resolve(dirname(source), imported.fileName);
      const candidates = imported.fileName.endsWith(".json") || /\.d\.[cm]?ts$/.test(imported.fileName) ? [[sourceImport, imported.fileName]] : [
        [sourceImport.replace(/\.(?:[cm]?[jt]s)$/, ".d.ts"), imported.fileName.replace(/\.(?:[cm]?[jt]s)$/, ".d.ts")],
        [`${sourceImport}.d.ts`, `${imported.fileName}.d.ts`],
      ];
      const selected = candidates.find(([from]) => existsSync(from));
      if (!selected) continue;
      const [from, destination] = selected, to = resolve(dirname(declaration), destination), local = relative(output, to);
      if (local === ".." || local.startsWith(`..${sep}`)) throw new Error(`Declaration asset escapes ${packageName}: ${imported.fileName}`);
      mkdirSync(dirname(to), { recursive: true });
      copyFileSync(from, to);
      copied.add(to);
    }
    return copied.size;
  };
  const build = async (): Promise<void> => {
    rmSync(output, { recursive: true, force: true });
    mkdirSync(output, { recursive: true });
    const result = await Bun.build({ entrypoints: [source], outdir: output, naming: "🟦️.js", target: "bun", format: "esm", minify: false });
    if (!result.success) throw new AggregateError(result.logs, `Failed to build ${packageName}`);
    await typeScript(source, ["--declaration", "--emitDeclarationOnly", "--outDir", output]);
    const assets = copyDeclarationAssets();
    console.log(`[artifact-typescript] built ${packageName} outputs=${result.outputs.length + 1 + assets}`);
  };
  class BuildScript extends BundleScript { async run(): Promise<void> { await build(); } }
  class CheckScript extends BundleScript {
    async run(): Promise<void> {
      const result = await Bun.build({ entrypoints: [source], target: "bun", format: "esm" });
      if (!result.success) throw new AggregateError(result.logs, `Failed to check ${packageName}`);
      await typeScript(source, ["--noEmit"]);
      await checkSuites();
      console.log(`[artifact-typescript] checked ${packageName} suites=${suites.length}`);
    }
  }
  class TestScript extends BundleScript {
    async run(): Promise<void> {
      await build();
      const artifact = await import(packageName);
      const probe = join(output, "🧪️consumer.ts"), typeRoots = join(output, "🧪️types");
      mkdirSync(typeRoots);
      const assertion = Object.hasOwn(artifact, "definition") ? "const definitionId: typeof artifact.definition.id = artifact.definition.id;\nvoid definitionId;" : `const artifactModule: typeof import(${JSON.stringify(packageName)}) = artifact;\nvoid artifactModule;`;
      writeFileSync(probe, `import * as artifact from ${JSON.stringify(packageName)};\n${assertion}\n`);
      try { await typeScript(probe, ["--noEmit", "--typeRoots", typeRoots], false); } finally { rmSync(probe, { force: true }); rmSync(typeRoots, { recursive: true, force: true }); }
      assert.equal(typeof artifact, "object", `${packageName} did not resolve as an ES module`);
      await checkSuites();
      for (const suite of suites) await runOwnedCommand(process.execPath, ["test", `./${relative(getWorkspaceRoot(), suite)}`], getWorkspaceRoot(), `artifact-typescript:${packageName}:suite`, 300_000);
      console.log(`[artifact-typescript] tested ${packageName} exports=${Object.keys(artifact).length} suites=${suites.length}`);
    }
  }
  const router = new ScriptRouter(packageRoot).register("build", BuildScript).register("check", CheckScript).register("test", TestScript);
  const segments = process.argv.slice(2);
  await router.run(segments.length ? segments : ["test"]);
}
