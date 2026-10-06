/** 🧭️ Verifies current neutral decode ownership and the actual native law mount. */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import * as toml from "@iarna/toml";
import * as ts from "typescript";
import witness from "../../🧫️fixtures/🧩️ownership/🔣️.json";

const root = resolve(import.meta.dir, "../../../../../..");
const read = (path: string) => readFileSync(resolve(root, path), "utf8");

test("neutral value roots mount the owned decode laws", () => {
  expect(read(witness.packageSource)).toContain('#[path = "../../🦀️.rs"]');
  expect(read(witness.valueSource)).toContain('#[path = "🛬️decode/🦀️.rs"]');
  expect(read(witness.controlSource)).toContain(witness.nativeMount);
  const native = read(witness.nativeSource);
  expect(native).not.toContain("semio_framework_os_kernel");
  expect([...native.matchAll(/#\[test\]\s*fn ([a-z_]+)\(/gmu)].map(match => match[1])).toEqual(expect.arrayContaining(witness.nativeLaws));
});

test("independent TOML admission resolves the actual Cargo library root", () => {
  const source = read(witness.packageManifest), native = Bun.TOML.parse(source);
  expect(native).toEqual(toml.parse(source));
  const manifest = native as {package: {name: string}; lib: {name: string; path: string}};
  expect(manifest.package.name).toBe(witness.package);
  expect(manifest.lib.name).toBe("semio_framework_value");
  expect(resolve(root, dirname(witness.packageManifest), manifest.lib.path)).toBe(resolve(root, witness.packageSource));
});

test("the owner router registers the neutral test sources", () => {
  const source = read(witness.packageScript);
  const tree = ts.createSourceFile(witness.packageScript, source, ts.ScriptTarget.Latest, true);
  const paths: string[] = [];
  function collect(node: ts.Node): void {
    if (ts.isStringLiteral(node)) paths.push(node.text);
    ts.forEachChild(node, collect);
  }
  collect(tree);
  expect(paths).toContain("../../🔁️codec/🧪️tests/🛬️controlled/🟦️.ts");
  expect(paths).toContain("../../🛬️decode/🧪️tests/🟦️.ts");
});
