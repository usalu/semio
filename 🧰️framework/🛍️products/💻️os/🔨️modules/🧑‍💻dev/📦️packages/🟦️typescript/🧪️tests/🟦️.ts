/** 🧪️ Vitest config for `@semio-tech/framework-os-dev`. `includeSource` enables in-source
 * `import.meta.vitest` blocks inside `📜️script.ts` itself (the task-router entry, not a `js/index.ts`
 * — this bundle root has no separate library entry point) for pure helper logic (marker parsing,
 * built-module scanning) that doesn't need a live cargo/vite process. */
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
import { stripExecutableShebang } from "./🧹️executable-source/🟦️.ts";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");

export default defineConfig({
  root,
  plugins: [
    {
      name: "semio-strip-executable-shebang",
      enforce: "pre",
      transform(source, id) {
        if (!/(?:^|[\\/])📜️script\.ts(?:[?#].*)?$/u.test(id) || !source.startsWith("#!")) return null;
        return { code: stripExecutableShebang(source), map: null };
      },
    },
  ],
  test: {
    name: "@semio-tech/framework-os-dev",
    environment: "jsdom",
    // 🩹️ In-source files belong only in `includeSource`; listing them in BOTH keys made Vitest
    // collect them twice. Dedicated regression files remain ordinary `include` entries.
    include: ["🧹️config.test.ts"],
    includeSource: ["📜️script.ts", "../../../🔌️plugin/📤️return/🟦️.ts", "../../../🔌️plugin/📥️poll/🏘️composition/🟦️.ts"],
    coverage: { include: ["📜️script.ts", "../../../🔌️plugin/📤️return/🟦️.ts", "../../../🔌️plugin/📥️poll/🏘️composition/🟦️.ts"] },
  },
});
