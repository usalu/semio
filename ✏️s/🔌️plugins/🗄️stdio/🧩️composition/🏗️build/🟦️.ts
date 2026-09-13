import assert from "node:assert/strict";
import { mkdirSync, rmSync, writeFileSync } from "node:fs";
import { join, relative, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { getWorkspaceRoot } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts";
import { runCmd } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { COMPOSITION_TYPESCRIPT_NAME, JsonMap } from "../../🗿️artifacts/📇️inventory/🟦️.ts";

function runTypeScriptCompiler(source: string, mode: string[]): void {
  runCmd(process.execPath, ["x", "tsc", source, ...mode, "--module", "ESNext", "--moduleResolution", "Bundler", "--resolveJsonModule", "--allowSyntheticDefaultImports", "--strict", "--skipLibCheck", "--target", "ES2022"], { cwd: getWorkspaceRoot() });
}

function compositionSource(packageRoot: string): string {
  return resolve(packageRoot, "../../🟦️.ts");
}

/** 🏗️ Builds the Stdio TypeScript composition into the selected publication directory. */
export async function buildStdioComposition(packageRoot: string, outputRoot = join(packageRoot, "dist")): Promise<void> {
  const source = compositionSource(packageRoot);
  rmSync(outputRoot, { recursive: true, force: true });
  mkdirSync(outputRoot, { recursive: true });
  const result = await Bun.build({ entrypoints: [source], outdir: outputRoot, naming: "🟦️.js", target: "bun", format: "esm", minify: false });
  if (!result.success) throw new AggregateError(result.logs, `failed to build ${COMPOSITION_TYPESCRIPT_NAME}`);
  runTypeScriptCompiler(source, ["--declaration", "--emitDeclarationOnly", "--outDir", outputRoot]);
  console.log(`[stdio-package] built ${COMPOSITION_TYPESCRIPT_NAME} outputs=${result.outputs.length}`);
}

/** 🔎️ Checks the Stdio TypeScript composition without publishing outputs. */
export async function checkStdioComposition(packageRoot: string): Promise<void> {
  const source = compositionSource(packageRoot);
  const result = await Bun.build({ entrypoints: [source], write: false, target: "bun", format: "esm" });
  if (!result.success) throw new AggregateError(result.logs, `failed to check ${COMPOSITION_TYPESCRIPT_NAME}`);
  runTypeScriptCompiler(source, ["--noEmit"]);
  console.log(`[stdio-package] checked ${COMPOSITION_TYPESCRIPT_NAME}`);
}

/** 🧪️ Builds and consumes the exact 36-export Stdio composition from the selected output root. */
export async function testStdioComposition(packageRoot: string, outputRoot = join(packageRoot, "dist")): Promise<void> {
  await buildStdioComposition(packageRoot, outputRoot);
  const declaration = join(outputRoot, "🟦️.d.ts");
  const probe = join(outputRoot, "🧪️consumer.ts");
  writeFileSync(probe, `import { binary as definition } from ${JSON.stringify(`./${relative(outputRoot, declaration).replace(/\\/gu, "/")}`)};\nvoid definition;\n`);
  try {
    runTypeScriptCompiler(probe, ["--noEmit"]);
  } finally {
    rmSync(probe, { force: true });
  }
  const facade = await import(`${pathToFileURL(join(outputRoot, "🟦️.js")).href}?stdio=${Date.now()}`);
  const entries = Object.entries(facade);
  assert.equal(entries.length, 36);
  for (const [artifact, namespace] of entries) assert.equal((namespace as JsonMap).definition?.id, `s.stdio.${artifact}`);
  console.log(`[stdio-package] tested ${COMPOSITION_TYPESCRIPT_NAME} artifacts=${entries.length}`);
}
