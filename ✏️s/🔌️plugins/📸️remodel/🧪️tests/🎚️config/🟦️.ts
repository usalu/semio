import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const testRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../..");

/** 📸️ Runs the committed Remodel example tests and the cross-language schema fixture oracle. */
export default defineConfig({
  root: testRoot,
  test: {
    root: testRoot,
    name: "@semio-tech/remodel-js",
    environment: "node",
    include: ["🗿️artifacts/**/📚️examples/**/🧪️tests/🧩️example/🟦️.ts", "🗿️artifacts/**/🧬️schema/🧪️tests/🧩️suite/🟦️.ts"],
    passWithNoTests: false,
  },
});
