import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const testRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");

export default defineConfig({
  root: testRoot,
  test: {
    root: testRoot,
    name: "@semio-tech/trinity-jack-lsp-worker",
    environment: "node",
    include: ["../../🧪️tests/🔖️protocol/🟦️.ts", "../../🧪️tests/📡️worker/🟦️.ts"],
    coverage: { include: ["../../🟦️.ts", "../../🔖️protocol/🟦️.ts"] },
  },
});
