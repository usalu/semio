/** 🧪️ Vitest config for `@semio-tech/framework-os-dev`. `includeSource` enables in-source
 * `import.meta.vitest` blocks inside `📜️script.ts` itself (the task-router entry, not a `js/index.ts`
 * — this bundle root has no separate library entry point) for pure helper logic (marker parsing,
 * built-module scanning) that doesn't need a live cargo/vite process. */
import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    name: "@semio-tech/framework-os-dev",
    environment: "jsdom",
    include: ["📜️script.ts"],
    includeSource: ["📜️script.ts"],
    coverage: { include: ["📜️script.ts"] },
  },
});
