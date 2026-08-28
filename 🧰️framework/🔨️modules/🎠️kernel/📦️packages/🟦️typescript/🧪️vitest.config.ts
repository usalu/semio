// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌️Adapters

const configDir = dirname(fileURLToPath(import.meta.url));
const root = resolve(configDir, "../.."); // 🎠️kernel module root — owner of 🟦️component.ts

/**
 * @emoji 🧪️ Vitest for `@semio-tech/framework-kernel` (inline `import.meta.vitest` in
 * `🟦️component.ts`: `ActivationRegistry`, runtime metrics, `IoEntryGraph` routing, …).
 *
 * `includeSource`/`coverage.include` use a glob (`*.ts`, non-recursive, scoped to this module's own
 * root) rather than an explicit filename array — the sibling `@semio-tech/framework-actor` config
 * lists `🧵️shard-client.ts`/`📬️mailbox.ts`/`🧵️turn-scheduler.ts` by name, and that style has already
 * caused a silent skip once (a new sibling test file never ran while the suite still reported green).
 * A glob picks up any future `.ts` file dropped next to `🟦️component.ts` with no config edit.
 *
 * `include` stays empty on purpose: leaving it equal to `includeSource`'s glob (as
 * `@semio-tech/framework-actor`/`framework-os`/`framework-os-mcp`/`framework-os-shell` all do) makes
 * vitest collect each in-source file through BOTH the normal `include` test-file path and the
 * `includeSource` in-source path, doubling every test's run count (verified here: 58 reported instead
 * of 29 before this was narrowed) while every test still shows green — the same "check exists but
 * silently doesn't mean what it claims" defect this packet exists to close, just on the sibling
 * configs instead of a missing suite. Not fixed there (outside this package's owned paths); see this
 * packet's report.
 */
export default defineConfig({
  root,
  test: {
    name: "@semio-tech/framework-kernel",
    mode: "test",
    environment: "jsdom",
    include: [],
    coverage: { include: ["*.ts"] },
    includeSource: ["*.ts", "📤️return/📦️content/🟦️component.ts"],
    passWithNoTests: false,
  },
});
