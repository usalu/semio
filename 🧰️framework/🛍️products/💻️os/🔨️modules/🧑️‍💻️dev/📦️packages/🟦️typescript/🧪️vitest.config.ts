/** 🧪️ Vitest config for `@semio-tech/framework-os-dev`. `includeSource` enables in-source
 * `import.meta.vitest` blocks inside `📜️script.ts` itself (the task-router entry, not a `js/index.ts`
 * — this bundle root has no separate library entry point) for pure helper logic (marker parsing,
 * built-module scanning) that doesn't need a live cargo/vite process. */
import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    name: "@semio-tech/framework-os-dev",
    environment: "jsdom",
    // 🩹️ `include` MUST stay empty: this is an in-source (`import.meta.vitest`) suite collected via
    // `includeSource`. Listing the same file in BOTH keys made vitest collect it twice and report
    // double the real test count. Add new in-source files to `includeSource`/`coverage.include` only.
    include: [],
    includeSource: ["📜️script.ts", "../../../🔌️plugin/📤️return/🟦️component.ts", "../../../🔌️plugin/📥️poll/🏘️composition/🟦️component.ts"],
    coverage: { include: ["📜️script.ts", "../../../🔌️plugin/📤️return/🟦️component.ts", "../../../🔌️plugin/📥️poll/🏘️composition/🟦️component.ts"] },
  },
});
