import assert from "node:assert/strict";
import { rmSync, writeFileSync } from "node:fs";
import { join, relative } from "node:path";
import { pathToFileURL } from "node:url";
import { COMPOSITION_TYPESCRIPT_NAME, type JsonMap } from "../../🗿️artifacts/📇️inventory/🟦️.ts";
import { buildStdioComposition, runStdioTypeScriptCompiler } from "../../🧩️composition/🏗️build/🟦️.ts";

/** 🧪️ Builds and consumes the exact 36-export Stdio composition from the selected output root. */
export async function testStdioComposition(packageRoot: string, outputRoot = join(packageRoot, "dist")): Promise<void> {
  await buildStdioComposition(packageRoot, outputRoot);
  const declaration = join(outputRoot, "🟦️.d.ts");
  const probe = join(outputRoot, "🧪️consumer.ts");
  writeFileSync(probe, `import { binary as definition } from ${JSON.stringify(`./${relative(outputRoot, declaration).replace(/\\/gu, "/")}`)};\nvoid definition;\n`);
  try {
    runStdioTypeScriptCompiler(probe, ["--noEmit"]);
  } finally {
    rmSync(probe, { force: true });
  }
  const facade = await import(`${pathToFileURL(join(outputRoot, "🟦️.js")).href}?stdio=${Date.now()}`);
  const entries = Object.entries(facade);
  assert.equal(entries.length, 36);
  for (const [artifact, namespace] of entries) assert.equal((namespace as JsonMap).definition?.id, `s.stdio.${artifact}`);
  console.log(`[stdio-package] tested ${COMPOSITION_TYPESCRIPT_NAME} artifacts=${entries.length}`);
}
