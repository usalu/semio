import { resolve } from "node:path";
import { defineConfig } from "vitest/config";

const root = resolve(import.meta.dirname, "../..");
export default defineConfig({ root, test: { root, name: "@semio-tech/hub-stdio-publication", environment: "node", include: ["🧪️tests/*/🟦️.ts"], exclude: ["🧪️tests/🎚️config/**"], testTimeout: 120_000 } });
