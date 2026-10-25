import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const here = dirname(fileURLToPath(import.meta.url));
const testRoot = resolve(here, "../../📦️packages/🟦️typescript");

export default defineConfig({
  root: testRoot,
  resolve: { alias: { "@semio-tech/trinity-jack-lsp": resolve(here, "../📡️worker/🪞️session-double/🟦️.ts") } },
  test: {
    root: testRoot,
    name: "@semio-tech/trinity-jack-lsp-worker",
    environment: "node",
    include: ["../../🧪️tests/🚦️message-guards/🟦️.ts", "../../🧪️tests/📍️position/🟦️.ts", "../../🧪️tests/🔖️protocol/🟦️.ts", "../../🧪️tests/📡️worker/🟦️.ts"],
    coverage: { include: ["../../🟦️.ts", "../../🔖️protocol/🟦️.ts"] },
  },
});
