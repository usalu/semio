// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
// #endregion 🔌️Adapters

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");

/** 🧪️ Vitest for `@semio-tech/presentation`: the owner's canonical cases. */
export default {
  root,
  resolve: {
    alias: [{ find: /^@semio-tech\/presentation$/u, replacement: resolve(root, "🟦️.ts") }],
  },
  test: {
    root,
    name: "@semio-tech/presentation",
    mode: "test",
    environment: "node",
    include: ["../../🧪️tests/🧭️slide-glob-assembly/🟦️.ts", "../../🧪️tests/📽️presentation-core/🟦️.ts", "../../🧪️tests/🌱️intro-and-resolution/🟦️.ts", "../../🧪️tests/🪡️thought-expansion/🟦️.ts", "../../🧪️tests/🗺️arrangement-geometry-and-sequences/🟦️.ts", "../../🧪️tests/🔄️analogy-and-tile-play/🟦️.ts"],
    coverage: { include: ["🟦️.ts"] },
    passWithNoTests: false,
  },
};
