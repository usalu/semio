import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
import { repoCacheDirectory } from "../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts";

const testRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../..");

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const repoRoot = resolve(root, "../../../../../..");

export default defineConfig({
  root: testRoot,
  cacheDir: repoCacheDirectory(repoRoot, "vite", "plugin-registry"),
  test: {
    root: testRoot,
    name: "@semio-tech/plugin-registry",
    environment: "node",
    include: ["🧪️tests/*/🟦️.ts"],
    exclude: ["🧪️tests/📚️storybook-plugins/**"],
  },
});
