import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));

export default defineConfig({
  root,
  test: {
    name: "@semio-tech/puzzle-5d-react",
    environment: "node",
    include: ["📦️index.tsx"],
    coverage: { include: ["📦️index.tsx"] },
  },
});
