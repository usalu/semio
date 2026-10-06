/** 🧭️ Checks the actual framework command router's general source owners. */
import { expect, test } from "bun:test";

import ts from "typescript";
import { build } from "esbuild";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import fixture from "../🧫️fixtures/🔣️.json";

const framework = resolve(import.meta.dir, "../..");
const entry = join(framework, fixture.entry);



test("the boundary law compiles with strict owned types", () => {
  const program = ts.createProgram([join(import.meta.dir, "🟦️.ts")], { noEmit: true, strict: true, noUncheckedIndexedAccess: true, target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, allowImportingTsExtensions: true, resolveJsonModule: true, esModuleInterop: true, skipLibCheck: true, types: ["bun", "node"] });
  expect(ts.getPreEmitDiagnostics(program).map((row) => ts.flattenDiagnosticMessageText(row.messageText, "\n"))).toEqual([]);
});

test("every owned command is explicitly registered in the actual router", () => {
  const source = ts.createSourceFile(entry, readFileSync(entry, "utf8"), ts.ScriptTarget.Latest, true);
  const commands: string[] = [];
  const visit = (node: ts.Node): void => {
    if (ts.isCallExpression(node) && ts.isPropertyAccessExpression(node.expression) && node.expression.name.text === "register") {
      const name = node.arguments[0];
      expect(name !== undefined && ts.isStringLiteral(name)).toBe(true);
      if (name && ts.isStringLiteral(name)) commands.push(name.text);
    }
    ts.forEachChild(node, visit);
  };
  visit(source);
  expect(commands.sort()).toEqual(fixture.commands);
});

test("the actual router bundles with every specific source owner unavailable", async () => {
  const workspace = resolve(framework, "..");
  const refused = fixture.refusedOwners.map((path) => resolve(workspace, path).replaceAll("\\", "/") + "/");
  const result = await build({ entryPoints: [entry], bundle: true, platform: "node", format: "esm", write: false, metafile: true, logLevel: "silent", plugins: [{ name: "removed-framework-owner", setup(builder) { builder.onLoad({ filter: /.*/ }, (input) => { const path = input.path.replaceAll("\\", "/"); return refused.some((prefix) => path.startsWith(prefix)) ? { errors: [{ text: "Framework command imports an unavailable owner: " + path }] } : undefined; }); } }] });
  expect(result.outputFiles?.length).toBe(1);
  const inputs = Object.keys(result.metafile!.inputs);
  expect(inputs.length).toBeGreaterThan(3);
  expect(inputs.some((path) => fixture.refusedOwners.some((owner) => path.replaceAll("\\", "/").startsWith(owner + "/")))).toBe(false);
  console.log("[DEBUG] framework-script-boundary " + JSON.stringify({ sourceOwners: inputs.length, specificOwners: 0 }));
}, 15_000);
