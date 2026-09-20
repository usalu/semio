import { fileURLToPath } from "node:url";
import { dirname, resolve } from "node:path";
//#region 🔌️Adapters
import { defineConfig } from "vitest/config";

const testRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
//#endregion 🔌️Adapters

/** 🧪️ Tests for semio-tech play. */
export default defineConfig({
  root: testRoot,
  test: {
    root: testRoot,
    name: "@semio-tech/semio-tech-play",
    environment: "node",
    include: [],
    includeSource: ["./🪧️brand.ts", "./🔨️modules/📦️site/🗺️tile-serve-mode/🟦️.ts", "./🔨️modules/🧩️runtime/🟦️.ts", "./🔨️modules/🧩️runtime/♻️activation/🟦️.ts"],
    passWithNoTests: false,
  },
});
