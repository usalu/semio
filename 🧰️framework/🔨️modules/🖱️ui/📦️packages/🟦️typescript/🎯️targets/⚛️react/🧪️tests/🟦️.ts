// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌️Adapters

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");

/** @emoji 🧪️ Vitest for `@semio-tech/ui-react` and its owned React modules. */
export default defineConfig({
  root,
  resolve: {
    alias: [{ find: "@semio-tech/ui-react", replacement: resolve(root, "🟦️.tsx") }],
  },
  test: {
    name: "@semio-tech/ui-react",
    environment: "jsdom",
    include: [
      "../../../../🧱️elements/☑️Checkbox/🧪️tests/🟦️.tsx",
      "../../../../🧱️elements/☑️Select/🧪️tests/🟦️.tsx",
      "../../../../🧱️elements/📊️Diagram/🧪️tests/🟦️.tsx",
      "../../../../🧱️elements/↕️Collapsible/🧪️tests/🟦️.tsx",
      "../../../../🧱️elements/📋️MenuItem/🧪️tests/🟦️.tsx",
      "../../../../🧱️elements/🧾️Form/🧪️tests/🟦️.tsx",
      "../../../../🧱️elements/💬️Dialog/🧪️tests/🟦️.tsx",
      "../../../../🧱️elements/⌨️Command/🧪️tests/🟦️.tsx",
      "../../../../🧱️elements/📻️TableAvatar/🧪️tests/🟦️.tsx",
      "../../../../🧱️elements/🗨️Popover/🧪️tests/🟦️.tsx",
      "../../../../🧱️elements/🎚️Slider/🧪️tests/🟦️.tsx",
      "../../../../🧱️elements/🎚️Toggle/🧪️tests/🟦️.tsx",
      "../../../../🧱️elements/🎛️ToggleGroup/🧪️tests/🟦️.tsx",
      "../../../../🧱️elements/🪵️Tree/🧪️tests/🟦️.tsx",
      "../../../../🧱️elements/📑️Tabs/🧪️tests/🟦️.tsx",
      "../../../../🔨️modules/⌨️control-keybinding-context/🧪️tests/🟦️.tsx",
      "../../../../🔨️modules/🏷️class-name-composition/🧪️slot.test.tsx",
      "🧪️tests/🧪️package-export/🟦️.ts",
    ],
    includeSource: ["🟦️.tsx", resolve(root, "../../../../../../../.storybook/🟦️lint-tooling.ts")],
    coverage: { include: ["🟦️.tsx"] },
    passWithNoTests: true,
    setupFiles: [resolve(root, "🟦️vitest.setup.ts")],
  },
});
