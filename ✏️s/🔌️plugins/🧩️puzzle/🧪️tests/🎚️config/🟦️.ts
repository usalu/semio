import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const testRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../..");

const root = resolve(resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript"), "../..");

/** 🧩️ Vitest for the committed Puzzle example definition leaves. */
export default defineConfig({
  root: testRoot,
  test: {
    root: testRoot,
    name: "@semio-tech/puzzle-js",
    environment: "node",
    include: ["🗿️artifacts/**/📚️examples/**/🧪️tests/🧩️example/🟦️.ts"],
    passWithNoTests: false,
  },
});
