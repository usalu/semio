// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const testRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
// #endregion 🔌️Adapters

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../..");

/** 🧪️ Vitest for `@semio-tech/plugin-window-kits` — one suite per window kit, no shared barrel to alias: the
 * table, tree, image, mesh and document suites import their kit and register themselves; the text and media
 * kits are still collected as `import.meta.vitest` in-source suites. */
export default {
  root: testRoot,
  test: {
    root: testRoot,
    name: "@semio-tech/plugin-window-kits",
    environment: "node",
    // 🩹️ Never list one file in both `include` and `includeSource`: it would be collected twice (📌️important.md rule 18).
    include: ["📊️table/🧪️tests/🧪️rendertable/🟦️.ts", "🌳️tree/🧪️tests/🧪️rendertree/🟦️.ts", "🖼️image/🧪️tests/🧪️renderimage/🟦️.ts", "🧊️mesh/🧪️tests/🧪️rendermesh/🟦️.ts", "📃️document/🧪️tests/🧪️renderdocument/🟦️.ts"],
    coverage: {
      include: ["📝️text/🟦️.ts", "📊️table/🟦️.ts", "🌳️tree/🟦️.ts", "🖼️image/🟦️.ts", "🧊️mesh/🟦️.ts", "📃️document/🟦️.ts", "🎬️media/🟦️.ts"],
    },
    includeSource: ["📝️text/🟦️.ts", "🎬️media/🟦️.ts"],
    passWithNoTests: false,
  },
};
