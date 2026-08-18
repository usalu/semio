// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌️Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪️ Vitest for `@semio-tech/framework-actor` (inline `import.meta.vitest`). */
export default defineConfig({
  root,
  resolve: {
    alias: {
      "@semio-tech/framework-actor": resolve(root, "🧵️shard-client.ts"),
    },
  },
  test: {
    name: "@semio-tech/framework-actor",
    mode: "test",
    environment: "node",
    include: ["🧵️shard-client.ts"],
    coverage: { include: ["🧵️shard-client.ts"] },
    includeSource: ["🧵️shard-client.ts"],
    passWithNoTests: false,
  },
});
