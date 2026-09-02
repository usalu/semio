// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
// #endregion 🔌️Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪️ Vitest for `@semio-tech/plugin-window-kits` — seven independent `import.meta.vitest`
 * in-source suites, one per window kit, no shared barrel to alias. */
export default {
  root,
  test: {
    name: "@semio-tech/plugin-window-kits",
    environment: "node",
    // 🩹️ `include` MUST stay empty: these are in-source (`import.meta.vitest`) suites collected via
    // `includeSource` — listing a file in both keys double-collects it (📌️important.md rule 18).
    include: [],
    coverage: {
      include: ["📝️text/🟦️.ts", "📊️table/🟦️.ts", "🌳️tree/🟦️.ts", "🖼️image/🟦️.ts", "🧊️mesh/🟦️.ts", "📄️document/🟦️.ts", "🎬️media/🟦️.ts"],
    },
    includeSource: ["📝️text/🟦️.ts", "📊️table/🟦️.ts", "🌳️tree/🟦️.ts", "🖼️image/🟦️.ts", "🧊️mesh/🟦️.ts", "📄️document/🟦️.ts", "🎬️media/🟦️.ts"],
    passWithNoTests: false,
  },
};
