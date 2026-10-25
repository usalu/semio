/** 🧬 Contract: a TypeScript facade that only re-exports the generated wasm bindings is not type-checked from the in-tree copy, because the hermetic wasm build publishes the bindings to the artifact store and never into the source tree. */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { relative, resolve } from "node:path";

const os = resolve(import.meta.dirname, "../../../../..");
const facade = resolve(import.meta.dirname, "../../📦️packages/🦀️rust/🟦️.ts");

test("the generated-bindings facade is excluded by the os tsconfig", () => {
  const text = readFileSync(facade, "utf8");
  expect(text).toMatch(/from "(?:\.\.\/)+🕸️bindings\/flow_core\.js"/u);
  const config = JSON.parse(readFileSync(resolve(os, "tsconfig.json"), "utf8")) as { exclude: string[] };
  expect(config.exclude).toContain(relative(os, facade).replaceAll("\\", "/"));
  expect(config.exclude).toContain("**/🕸️bindings/**");
});
