import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");

export default defineConfig({
  root,
  test: {
    name: "@semio-tech/trinity-jack-lsp-worker",
    environment: "node",
    include: ["🟦️.ts", "../../🟦️.ts"],
    coverage: { include: ["🟦️.ts", "../../🟦️.ts"] },
  },
});
