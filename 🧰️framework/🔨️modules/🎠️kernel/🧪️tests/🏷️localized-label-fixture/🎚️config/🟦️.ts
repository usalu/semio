/** 🌐️ Executes the owned localized-label contract with the caller’s bounded Vitest policy. */
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
import { testCacheDirectoryV1 } from "../../../../🏃️process/🧪️testing/🧪️vitest/🟦️.ts";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../../..");

export default defineConfig({
  root,
  cacheDir: testCacheDirectoryV1(process.env, "kernel-localized-label-fixture"),
  test: {
    root,
    name: "@semio-tech/framework-kernel",
    environment: "node",
    include: ["🧪️tests/🏷️localized-label-fixture/🟦️.ts"],
    passWithNoTests: false,
  },
});
