import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

/** 📸️ Runs the committed Remodel artifact and editor example tests. */
export default defineConfig({
  root: resolve(dirname(fileURLToPath(import.meta.url)), "../../.."),
  test: {
    name: "@semio-tech/remodel-js",
    environment: "node",
    include: ["🗿️artifacts/**/📚️examples/**/🧪️tests/🟦️.ts"],
    passWithNoTests: false,
  },
});
