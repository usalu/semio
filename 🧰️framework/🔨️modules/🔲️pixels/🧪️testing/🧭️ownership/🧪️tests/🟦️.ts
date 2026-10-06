/** 🧭️ Task ownership vectors checked by Ajv and observation of the actual task router. */
import { expect, test } from "bun:test";

import ts from "typescript";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { findWorkspaceRoot } from "../../../../🏃️process/🧭️routing/🟦️.ts";
import descriptor from "../🔣️.json";
import fixture from "../🧫️fixtures/🔣️.json";
import { readPixelsTaskDescriptorV1 } from "../🟦️.ts";

const repoRoot = findWorkspaceRoot(import.meta.dir);
const pixelRoot = join(repoRoot, "🧰️framework/🔨️modules/🔲️pixels");
const strictFlags = ["--noEmit", "--strict", "--noUncheckedIndexedAccess", "--skipLibCheck", "--target", "ES2022", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions"];

for (const row of fixture.cases) test(row.name, () => {
  const candidate = structuredClone(descriptor);
  if (row.field) (candidate[row.field as "strictRoots" | "testRoots" | "inputs"]).push(row.value!);
  if (row.admit) expect<unknown>(readPixelsTaskDescriptorV1(candidate)).toEqual(candidate);
  else expect(() => readPixelsTaskDescriptorV1(candidate)).toThrow();
});

test("default task preserves exact neutral compiler and runtime argv", async () => {
  const execution = join(repoRoot, "🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts");
  const entrypoint = join(repoRoot, "🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts");
  const source = `import {mock} from "bun:test"; const calls=[]; const execution=await import(${JSON.stringify(execution)}); mock.module(${JSON.stringify(execution)},()=>({...execution,runOwnedCommand:async(executable,argv,cwd,policy,budget)=>{calls.push({executable,argv,cwd,policy,budget});}})); mock.module(${JSON.stringify(entrypoint)},()=>({runScriptMain:async(router)=>router.run(["test","typescript"])})); await import(${JSON.stringify(join(pixelRoot, "📜️script.ts"))}); console.log(JSON.stringify(calls));`;
  const child = Bun.spawn([process.execPath, "-e", source], { cwd: repoRoot, stdout: "pipe", stderr: "pipe", env: { ...process.env, SEMIO_CMD_BUDGET_MS: "137" } });
  const [stdout, stderr, code] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
  expect(code, stderr).toBe(0);
  const calls = JSON.parse(stdout) as { executable: string; argv: string[]; cwd: string; policy: string; budget: number }[];
  expect(calls.length).toBe(2);
  expect(calls[0]!.argv).toEqual([join(repoRoot, "node_modules/typescript/bin/tsc"), ...strictFlags, ...descriptor.strictRoots.map(path => join(repoRoot, path))]);
  expect(calls[1]!.argv).toEqual(["test", ...descriptor.testRoots.map(path => join(repoRoot, path))]);
  for (const call of calls) {
    expect(call.executable).toBe(process.execPath);
    expect(call.cwd).toBe(repoRoot);
    expect(call.policy).toBe("tool:owner");
    expect(call.budget).toBe(137);
  }
});

test("registered neutral Nx inputs match the language neutral task descriptor", () => {
  const project = JSON.parse(readFileSync(join(pixelRoot, "📋️project.json"), "utf8"));
  expect(project.targets["test-typescript"].inputs).toEqual(descriptor.inputs);
  expect(project.targets["test-typescript"].options.command).toBe("bun ./📜️script.ts test typescript");
});

test("the neutral source owner has no Cargo registration or runtime imports", () => {
  const project = JSON.parse(readFileSync(join(pixelRoot,"📋️project.json"),"utf8"));
  const tree = ts.createSourceFile("📜️script.ts",readFileSync(join(pixelRoot,"📜️script.ts"),"utf8"),ts.ScriptTarget.Latest,true);
  const imports = tree.statements.filter(ts.isImportDeclaration).map(node => (node.moduleSpecifier as ts.StringLiteral).text);
  expect(project.metadata?.nativeRoot).toBeUndefined();
  expect(Object.keys(project.targets)).toEqual(["test-typescript"]);
  expect(imports.some(path=>path.includes("/🦀️cargo/"))).toBe(false);
});
