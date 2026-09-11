import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
import { repoCacheDirectory } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts";

const root = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(root, "../../../../../../..");

/** @emoji 🧪️ Vitest for `@semio-tech/framework-os-shell` (inline `import.meta.vitest`). */
export default defineConfig({
  root,
  cacheDir: repoCacheDirectory(repoRoot, "vite", "os-shell"),
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
    coverage: { include: ["../../🟦️.ts", "../../🧬️schema/🟦️.ts"] },
    includeSource: ["../../🟦️.ts", "../../🧬️schema/🟦️.ts"],
    passWithNoTests: false,
  },
});
