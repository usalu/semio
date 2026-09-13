//#region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const testRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
//#endregion 🔌️Adapters

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../..");

/** @emoji 🧪️ Vitest configuration for the synchronous extension package/store boundary. */
export default {
  root: testRoot,
  test: {
    root: testRoot,
    name: "@semio-tech/plugin-extension-store",
    environment: "node",
    include: [],
    includeSource: ["📥️installation/🟦️.ts", "🟦️.ts"],
    passWithNoTests: false,
  },
};
