//#region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
//#endregion 🔌️Adapters

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");

/** @emoji 🧪️ Vitest configuration for the synchronous extension package/store boundary. */
export default {
  root,
  test: {
    name: "@semio-tech/plugin-extension-store",
    environment: "node",
    include: [],
    includeSource: ["📥️store.ts", "🟦️.ts"],
    passWithNoTests: false,
  },
};
