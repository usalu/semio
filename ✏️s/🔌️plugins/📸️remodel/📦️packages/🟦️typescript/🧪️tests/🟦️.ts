import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

/** 📸️ Runs the committed Remodel example tests and the cross-language schema fixture oracle. */
export default defineConfig({
  root: resolve(dirname(fileURLToPath(import.meta.url)), "../../.."),
  test: {
    name: "@semio-tech/remodel-js",
    environment: "node",
    include: ["🗿️artifacts/**/📚️examples/**/🧪️tests/🟦️.ts", "🗿️artifacts/**/🧬️schema/🧪️tests/🟦️.ts"],
    passWithNoTests: false,
  },
});
