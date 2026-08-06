import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    name: "@semio-tech/trinity-jack-lsp-worker",
    environment: "node",
    include: ["📦️index.ts", "../../🟦️component.ts"],
    coverage: { include: ["📦️index.ts", "../../🟦️component.ts"] },
  },
});
