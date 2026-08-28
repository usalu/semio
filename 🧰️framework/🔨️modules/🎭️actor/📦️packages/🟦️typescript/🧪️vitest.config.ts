// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
// #endregion 🔌️Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪️ Vitest for `@semio-tech/framework-actor` (inline `import.meta.vitest`). */
export default {
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
    // 🩹️ `include` MUST stay empty: these are in-source (`import.meta.vitest`) suites, collected via
    // `includeSource`. Listing the same files in BOTH keys makes vitest collect each one twice, which
    // reported 58 tests for 29 real ones and doubled this package's run time. Add new in-source files
    // to `includeSource`/`coverage.include` only — a file absent from `includeSource` does not run at
    // all while the suite still reports green.
    include: [],
    coverage: { include: ["🧵️shard-client.ts", "📬️mailbox.ts", "🧵️turn-scheduler.ts"] },
    includeSource: ["🧵️shard-client.ts", "📬️mailbox.ts", "🧵️turn-scheduler.ts", "../../🚪️lifetime/🟦️component.ts", "../../🚪️lifetime/🩹️patch/🟦️component.ts", "../../🪪️activation/🚪️instance/📥️output/🟦️component.ts", "../../📄️page/🟦️component.ts", "../../📤️return/🟦️component.ts", "../../📤️return/📨️response/🟦️component.ts"],
    passWithNoTests: false,
  },
};
