import { mkdirSync, rmSync } from "node:fs";
import { join, resolve } from "node:path";
import { getWorkspaceRoot } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts";
import { runCmd } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { COMPOSITION_TYPESCRIPT_NAME } from "../../🗿️artifacts/📇️inventory/🟦️.ts";

/** 🧾️ Runs the TypeScript compiler over one composition source with the flags every publication and consumer check shares. */
export function runStdioTypeScriptCompiler(source: string, mode: string[]): void {
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
  runStdioTypeScriptCompiler(source, ["--declaration", "--emitDeclarationOnly", "--outDir", outputRoot]);
  console.log(`[stdio-package] built ${COMPOSITION_TYPESCRIPT_NAME} outputs=${result.outputs.length}`);
}

/** 🔎️ Checks the Stdio TypeScript composition without publishing outputs. */
export async function checkStdioComposition(packageRoot: string): Promise<void> {
  const source = compositionSource(packageRoot);
  const result = await Bun.build({ entrypoints: [source], write: false, target: "bun", format: "esm" });
  if (!result.success) throw new AggregateError(result.logs, `failed to check ${COMPOSITION_TYPESCRIPT_NAME}`);
  runStdioTypeScriptCompiler(source, ["--noEmit"]);
  console.log(`[stdio-package] checked ${COMPOSITION_TYPESCRIPT_NAME}`);
}
