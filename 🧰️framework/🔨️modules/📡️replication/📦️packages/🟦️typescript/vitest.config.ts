// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌️Adapters

const configDir = dirname(fileURLToPath(import.meta.url));
const root = resolve(configDir, "../.."); // 📡️replication module root — owner of 🟦️component.ts

/**
 * @emoji 🧪️ Vitest for `@semio-tech/framework-replication` (inline `import.meta.vitest`).
 *
 * `includeSource`/`coverage.include` use a glob (`*.ts`, non-recursive, scoped to this module's own
 * root) rather than an explicit filename — see `@semio-tech/framework-kernel`'s config for why an
 * explicit name silently breaks (goes stale, or double-counts against `include`) the moment the
 * named file moves or a sibling `.ts` file is added beside it.
 *
 * `include` stays empty on purpose: leaving it equal to `includeSource`'s glob would make vitest
 * collect each in-source file through BOTH the normal `include` test-file path and the
 * `includeSource` in-source path, doubling every test's run count while still showing green.
 */
export default defineConfig({
  root,
  resolve: {
    alias: {
      "@semio-tech/framework-replication": resolve(root, "📦️packages/🟦️typescript/🟦️glue.ts"),
    },
  },
  test: {
    name: "@semio-tech/framework-replication",
    mode: "test",
    environment: "node",
    include: [],
    coverage: { include: ["*.ts"] },
    includeSource: ["*.ts"],
    passWithNoTests: false,
  },
});
