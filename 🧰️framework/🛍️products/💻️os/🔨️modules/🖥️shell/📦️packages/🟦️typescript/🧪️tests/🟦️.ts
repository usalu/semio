import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");

/** @emoji 🧪️ Vitest for `@semio-tech/framework-os-shell` (inline `import.meta.vitest`). */
export default defineConfig({
  root,
  resolve: {
    alias: {
      "@semio-tech/framework-os-shell": resolve(root, "🟦️.ts"),
    },
  },
  test: {
    name: "@semio-tech/framework-os-shell",
    mode: "test",
    environment: "node",
    include: [],
    coverage: { include: ["../../🟦️.ts"] },
    includeSource: ["../../🟦️.ts"],
    passWithNoTests: false,
  },
});
