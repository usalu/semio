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
    alias: [{ find: "@semio-tech/ui-react", replacement: resolve(root, "🟦️.tsx") }],
  },
  test: {
    name: "@semio-tech/ui-react",
    environment: "jsdom",
    include: [
      "../../../../🧱️elements/☑️Checkbox/🧪️tests/🧩️component/🟦️.tsx",
      "../../../../🧱️elements/🔽️Select/🧪️tests/🧩️component/🟦️.tsx",
      "../../../../🧱️elements/🕸️Diagram/🧪️tests/🧩️component/🟦️.tsx",
      "../../../../🧱️elements/↕️Collapsible/🧪️tests/🧩️component/🟦️.tsx",
      "../../../../🧱️elements/📋️MenuItem/🧪️tests/🧩️component/🟦️.tsx",
      "../../../../🧱️elements/🧾️Form/🧪️tests/🧩️component/🟦️.tsx",
      "../../../../🧱️elements/💬️Dialog/🧪️tests/🧩️component/🟦️.tsx",
      "../../../../🧱️elements/📨️UIDialog/🧪️tests/🧩️component/🟦️.tsx",
      "../../../../🧱️elements/⌨️Command/🧪️tests/🧩️component/🟦️.tsx",
      "../../../../🧱️elements/📻️TableAvatar/🧪️tests/🧩️component/🟦️.tsx",
      "../../../../🧱️elements/🗨️Popover/🧪️tests/🧩️component/🟦️.tsx",
      "../../../../🧱️elements/🎚️Slider/🧪️tests/🧩️component/🟦️.tsx",
      "../../../../🧱️elements/🔀️Toggle/🧪️tests/🧩️component/🟦️.tsx",
      "../../../../🧱️elements/🎛️ToggleGroup/🧪️tests/🧩️component/🟦️.tsx",
      "../../../../🧱️elements/🌳️Tree/🧪️tests/🧩️component/🟦️.tsx",
      "../../../../🧱️elements/📑️Tabs/🧪️tests/🧩️component/🟦️.tsx",
      "../../../../🔨️modules/🕹️control-keybinding-context/🧪️tests/🧩️component/🟦️.tsx",
      "../../../../🔨️modules/🏷️class-name-composition/🧪️tests/🧩️slot/🟦️.tsx",
      "../../../../🧪️tests/📦️react-package-export/🟦️.ts",
      resolve(root, "../../../../../../../.storybook/🧪️tests/🧹️owned-ui-react-lint/🟦️.ts"),
      resolve(root, "../../../../../../../.storybook/🧪️tests/🧭️scope-resolution/🟦️.ts"),
    ],
    includeSource: ["🟦️.tsx"],
    coverage: { include: ["🟦️.tsx"] },
    passWithNoTests: false,
    setupFiles: [resolve(root, "../../../../🧪️tests/🧹️react-environment/🟦️.ts")],
  },
});
