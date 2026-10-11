/** 🧱️ Proves catalog execution survives complete product removal through an independent Node loader. */
import { fileURLToPath, pathToFileURL } from "node:url";
import { strict as assert } from "node:assert";
import { readFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { spawnSync } from "node:child_process";
import { build } from "esbuild";
import { generatedTargets } from "../../../🏷️entity-kinds/📋️plan/🟦️.ts";
import { readEntityCatalog } from "../../../🏷️entity-kinds/📥️source/🟦️.ts";

type OwnershipFixture = {
  readonly schemaVersion: 1;
  readonly contract: string;
  readonly generalOutputs: string[];
  readonly catalogLength: number;
  readonly emojiLength: number;
  readonly firstWins: readonly { readonly emoji: string; readonly id: string }[];
};

export async function proveEntityCatalogOwnership(repoRoot: string): Promise<void> {
  const moduleRoot = join(dirname(fileURLToPath(import.meta.url)), "..", "..", "..");
  const fixture: OwnershipFixture = JSON.parse(readFileSync(join(moduleRoot, "🧫️fixtures/🏷️entity-ownership/🔣️.json"), "utf8"));
  const failures: string[] = [];
  try {
    assert.deepEqual(generatedTargets(readEntityCatalog(repoRoot)).map(target => relative(repoRoot, target.path).replaceAll("\\", "/")), fixture.generalOutputs);
  } catch (error) { failures.push(`planned paths: ${String(error)}`); }
  try {
    const execution = join(moduleRoot, "🏷️entity-kinds/🏃️execution/🟦️.ts");
    const plan = join(moduleRoot, "🏷️entity-kinds/📋️plan/🟦️.ts");
    const source = join(moduleRoot, "🏷️entity-kinds/📥️source/🟦️.ts");
    const processInvocation = join(moduleRoot, "..", "🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts");
    const schema = join(moduleRoot, "🟦️.ts");
    const result = await build({
      stdin: { contents: `import { PreviewGeneratedScript, CheckScript } from ${JSON.stringify(execution)};
import { generatedTargets } from ${JSON.stringify(plan)};
import { readEntityCatalog } from ${JSON.stringify(source)};
import { entityKindIndexByEmoji } from ${JSON.stringify(schema)};
import { createScriptProcessEnvelope, withScriptProcessEnvelope } from ${JSON.stringify(processInvocation)};
const root = ${JSON.stringify(repoRoot)};
const catalog = readEntityCatalog(root);
if (catalog.kinds.length !== ${fixture.catalogLength}) throw Error("catalog size drift");
const index = entityKindIndexByEmoji(catalog.kinds);
if (index.size !== ${fixture.emojiLength}) throw Error("emoji key count drift");
for (const vector of ${JSON.stringify(fixture.firstWins)}) if (index.get(vector.emoji)?.id !== vector.id) throw Error("FIRST-WINS drift");
const paths = generatedTargets(catalog).map(target => target.path);
if (paths.length !== 2) throw Error("general output ownership drift");
await withScriptProcessEnvelope(createScriptProcessEnvelope({ version: 1, owner: "entity-kinds-ownership-test", maximumElapsedMilliseconds: 0 }, {}, Date.now()), async invocation => {
  new CheckScript(root, root, invocation).run([]);
  new PreviewGeneratedScript(root, root, invocation).run([]);
});
console.log("[DEBUG] independent Node product-removal loader: 58 entries, two planned projections, check and preview executed");`, resolveDir: repoRoot, sourcefile: "entity-removal-oracle.ts", loader: "ts" },
      bundle: true, write: false, platform: "node", format: "esm", logLevel: "silent",
      plugins: [{ name: "complete-product-removal", setup(api) {
        api.onResolve({ filter: /products/ }, args => ({ errors: [{ text: `product removed: ${args.path}` }] }));
        api.onLoad({ filter: /\.ts$/ }, args => ({ contents: readFileSync(args.path, "utf8").replaceAll("import.meta.url", JSON.stringify(pathToFileURL(args.path).href)), loader: "ts" }));
      } }],
    });
    const node = spawnSync("node", ["--input-type=module"], { input: result.outputFiles[0].text, cwd: repoRoot, encoding: "utf8", timeout: 30000 });
    assert.equal(node.status, 0, node.stderr || String(node.error));
    const preview = node.stdout.split("\n").find(line => line.startsWith("{\"contractId\""));
    assert(preview, node.stdout);
    const reply = JSON.parse(preview);
    assert.equal(reply.contractId, "schema-entity-catalog");
    assert.deepEqual(reply.nodes.map((node: { path: string }) => node.path).sort(), [...fixture.generalOutputs].sort());
    assert(node.stdout.includes("[DEBUG] independent Node product-removal loader"));
    console.log(node.stdout.split("\n").filter(line => line.startsWith("[DEBUG]")).join("\n"));
  } catch (error) { failures.push(`product removal loader: ${String(error)}`); }
  assert.deepEqual(failures, [], failures.join("\n"));
}
