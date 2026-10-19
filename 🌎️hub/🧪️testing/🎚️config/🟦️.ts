import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const repoRoot = resolve(root, "..");

/** 🌐️ Exercises the installed application composition through its real private session worker: the shared worker-law suites, narrowed by the caller's name pattern. */
export default defineConfig({
  root,
  test: {
    root,
    name: "hub-browser-session-authority",
    environment: "node",
    include: [resolve(repoRoot, "🧰️framework/🛍️products/💻️os/🧪️tests/📚️worker-law-suites/🟦️.ts")],
    passWithNoTests: false,
  },
});
