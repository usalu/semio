// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌️Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪️ Vitest for `@semio-tech/ui-react` and its owned React modules. */
export default defineConfig({
  root,
  resolve: {
    alias: [{ find: "@semio-tech/ui-react", replacement: resolve(root, "📦️index.tsx") }],
  },
  test: {
    name: "@semio-tech/ui-react",
    environment: "jsdom",
    include: [
      "../../../../🧱️elements/☑️Checkbox/🧪️component.test.tsx",
      "../../../../🧱️elements/☑️Select/🧪️component.test.tsx",
      "../../../../🧱️elements/📊️Diagram/🧪️component.test.tsx",
      "../../../../🧱️elements/↕️Collapsible/🧪️component.test.tsx",
      "../../../../🧱️elements/📋️MenuItem/🧪️component.test.tsx",
      "../../../../🧱️elements/🧾️Form/🧪️component.test.tsx",
      "../../../../🧱️elements/💬️Dialog/🧪️component.test.tsx",
      "../../../../🧱️elements/⌨️Command/🧪️component.test.tsx",
      "../../../../🧱️elements/📻️TableAvatar/🧪️component.test.tsx",
      "../../../../🧱️elements/🗨️Popover/🧪️component.test.tsx",
      "../../../../🧱️elements/🎚️Slider/🧪️component.test.tsx",
      "../../../../🧱️elements/🎚️Toggle/🧪️component.test.tsx",
      "../../../../🧱️elements/🎛️ToggleGroup/🧪️component.test.tsx",
      "../../../../🧱️elements/🪵️Tree/🧪️component.test.tsx",
      "../../../../🧱️elements/📑️Tabs/🧪️component.test.tsx",
      "../../../../🔨️modules/⌨️control-keybinding-context/🧪️component.test.tsx",
      "../../../../🔨️modules/🏷️class-name-composition/🧪️component.test.ts",
      "../../../../🔨️modules/🏷️class-name-composition/🧪️slot.test.tsx",
      "../../../../🔨️modules/🏷️style-variants/🧪️component.test.ts",
    ],
    includeSource: ["📦️index.tsx"],
    coverage: { include: ["📦️index.tsx"] },
    passWithNoTests: true,
    setupFiles: [resolve(root, "🟦️vitest.setup.ts")],
  },
});
