import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");

/** @emoji ️ Vitest for puzzle example definition leaves. */
export default defineConfig({
  root,
  test: {
    name: "@semio-tech/puzzle-js",
    environment: "node",
    include: ["../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️concrete-forest/🧪️test/🟦️s.ts", "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏗️nakagin-capsule-tower/🧪️test/🟦️s.ts", "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️concrete-forest/🧪️test/🟦️s.ts", "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏗️nakagin-capsule-tower/🧪️test/🟦️s.ts", "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️concrete-forest/🧪️test/🟦️s.ts", "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏗️nakagin-capsule-tower/🧪️test/🟦️s.ts", "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️test/🟦️s.ts", "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️test/🟦️s.ts", "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️test/🟦️s.ts"],
  },
});
