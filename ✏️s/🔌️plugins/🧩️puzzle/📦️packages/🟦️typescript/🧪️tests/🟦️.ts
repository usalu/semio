import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../../..");

/** 🧩️ Vitest for the committed Puzzle example definition leaves. */
export default defineConfig({
  root,
  test: {
    name: "@semio-tech/puzzle-js",
    environment: "node",
    include: ["🗿️artifacts/**/📚️examples/**/🧪️tests/🟦️.ts"],
    passWithNoTests: false,
  },
});
