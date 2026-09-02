import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    name: "@semio-tech/trinity-jack-lsp-worker",
    environment: "node",
    include: ["🟦️.ts", "../../🟦️.ts"],
    coverage: { include: ["🟦️.ts", "../../🟦️.ts"] },
  },
});
