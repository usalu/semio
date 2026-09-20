// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

/** 🩹️ The FRAMEWORK root, not `📦️packages/🟦️typescript`. Every in-source file this suite collects lives
 * outside that package (`🔨️modules/…`, `🧪️tests/…`), and a vitest `includeSource` glob is resolved
 * against `test.root` by a globber that cannot walk upwards — every `../../…` pattern matched nothing,
 * so `bun nx run @semio-tech/framework:test` reported "No test files found, exiting with code 1" and
 * the kernel's own `import.meta.vitest` blocks (`AppRouter`, `ActivationRegistry`,
 * `expandPluginRegistry`, `createTurnOutcomeBroadcast`, `IoEntryGraph`) had never run under the real
 * gate. Rooting here makes the same files reachable by downward globs. */
const testRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
// #endregion 🔌️Adapters

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");

/** @emoji 🧪️ Vitest for `@semio-tech/framework` (inline `import.meta.vitest`). */
export default {
  root: testRoot,
  resolve: {
    alias: {
      "@semio-tech/framework": resolve(root, "🟦️.ts"),
    },
  },
  test: {
    root: testRoot,
    name: "@semio-tech/framework",
    mode: "test",
    environment: "node",
    // 🩹️ `include` MUST stay empty: this is an in-source (`import.meta.vitest`) suite collected via
    // `includeSource`. Listing the same file in BOTH keys made vitest collect it twice and report
    // double the real test count. Add new in-source files to `includeSource`/`coverage.include` only.
    include: [],
    coverage: { include: ["📦️packages/🟦️typescript/🟦️.ts", "🔨️modules/🎠️kernel/🟦️.ts", "🔨️modules/🎭️actor/📥️cold-pair/🟦️.ts", "🔨️modules/🕹️interaction/👆️gesture/🟦️.ts", "🧪️tests/🧪️docklayoutstore/🟦️.ts"] },
    includeSource: ["🔨️modules/🎠️kernel/🟦️.ts", "🔨️modules/🎭️actor/📥️cold-pair/🟦️.ts", "🔨️modules/🕹️interaction/👆️gesture/🟦️.ts", "🧪️tests/🧪️docklayoutstore/🟦️.ts"],
    passWithNoTests: false,
  },
};
