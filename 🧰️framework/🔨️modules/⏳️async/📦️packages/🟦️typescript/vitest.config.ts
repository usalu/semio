// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
// #endregion 🔌️Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪️ Vitest for `@semio-tech/framework-async` (inline `import.meta.vitest`). The only suite
 * suites are the `boxed_fixed_slots` budget twin in `../../🟦️.ts` — the independent, Rust-free
 * re-check of `🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json`'s `capacity × size_of` arithmetic — and the
 * continuation scheduler's fixture suite in `../../🪃️continuation/🟦️.ts`, run twice per case: once on
 * a virtual clock and once on the real event loop. */
export default {
  root,
  resolve: {
    alias: {
      "@semio-tech/framework-async": resolve(root, "../../🟦️.ts"),
    },
  },
  test: {
    name: "@semio-tech/framework-async",
    mode: "test",
    environment: "node",
    // 🩹️ `include` MUST stay empty: these are in-source (`import.meta.vitest`) suites, collected via
    // `includeSource`. Listing a file in BOTH keys makes vitest collect it twice.
    include: [],
    coverage: { include: ["../../🟦️.ts", "../../🪃️continuation/🟦️.ts"] },
    includeSource: ["../../🟦️.ts", "../../🪃️continuation/🟦️.ts"],
    passWithNoTests: false,
  },
};
