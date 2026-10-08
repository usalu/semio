import base from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts";
import { defineConfig } from "vitest/config";
import { resolve } from "node:path";
export default defineConfig({ ...base, test: { ...base.test, environment: "node", include: ["../🧩️integration/🟦️.ts", "../🎬️host-activation/🟦️.ts", "../📖️projection/🟦️.ts", "../🎮️session/🟦️.ts"].map(path => resolve(import.meta.dirname, path)), includeSource: [], testTimeout: 30000, hookTimeout: 30000 } });
