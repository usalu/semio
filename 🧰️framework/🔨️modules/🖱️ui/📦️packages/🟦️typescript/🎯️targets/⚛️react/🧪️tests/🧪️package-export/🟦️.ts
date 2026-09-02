import { existsSync, readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { expect, test } from "vitest";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const manifest = JSON.parse(readFileSync(resolve(root, "package.json"), "utf8")) as { exports: Record<string, string> };
const config = JSON.parse(readFileSync(resolve(root, "tsconfig.json"), "utf8")) as { compilerOptions: { paths: Record<string, string[]> } };

test("package entry and self-alias resolve to the canonical React source", () => {
  const entry = manifest.exports["."];
  expect(entry).toBe("./🟦️.tsx");
  expect(config.compilerOptions.paths["@semio-tech/ui-react"]).toEqual([entry]);
  expect(existsSync(resolve(root, entry))).toBe(true);
});
