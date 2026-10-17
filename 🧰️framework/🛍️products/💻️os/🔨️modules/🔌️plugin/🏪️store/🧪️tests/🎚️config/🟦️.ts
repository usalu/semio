//#region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const testRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
//#endregion 🔌️Adapters

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../..");

/** 🧪️ Vitest configuration for the synchronous extension package/store boundary. */
export default {
  root: testRoot,
  test: {
    root: testRoot,
    name: "@semio-tech/plugin-extension-store",
    environment: "node",
    include: ["🧪️tests/🧪️authored-extension-installation-identity/🟦️.ts"],
    includeSource: ["🟦️.ts"],
    passWithNoTests: false,
  },
};
