import { expect, test } from "bun:test";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import Ajv from "ajv";
import ts from "typescript";
import fixture from "../../🧫️fixtures/📇️bindings.json";


test("surface compiler companions keep their exact paired identity in the handpicked output owner", () => {
  
  const sourceRoot = join(import.meta.dir, "../../📦️packages/🦀️rust");
  const output = join(sourceRoot, fixture.directoryName);
  const names = [fixture.module, fixture.types, fixture.wasm, fixture.wasmTypes];
  expect(existsSync(join(sourceRoot, "pkg"))).toBe(false);
  expect(readdirSync(output).sort()).toEqual([...names, ".gitignore", "package.json"].sort());
  const js = readFileSync(join(output, fixture.module), "utf8");
  expect(js).toContain(`@ts-self-types="./${fixture.types}"`);
  expect(js).toContain(`new URL('${fixture.wasm}', import.meta.url)`);
  const resolved = ts.resolveModuleName(`./${fixture.module}`, join(output, "consumer.ts"), { moduleResolution: ts.ModuleResolutionKind.Bundler }, ts.sys).resolvedModule;
  expect(resolved?.resolvedFileName).toBe(join(output, fixture.types));
  expect(WebAssembly.validate(readFileSync(join(output, fixture.wasm)))).toBe(true);
  const manifest = JSON.parse(readFileSync(join(sourceRoot, "package.json"), "utf8"));
  expect(manifest.exports["."]).toBe(`./${fixture.directoryName}/${fixture.module}`);
  expect(manifest.exports[`./${fixture.directoryName}/${fixture.module}`]).toBe(manifest.exports["."]);
  const producer = readFileSync(join(sourceRoot, "📜️script.ts"), "utf8");
  expect(producer).toContain(`outputDirectory: "${fixture.directoryName}"`);
}, 30_000);
