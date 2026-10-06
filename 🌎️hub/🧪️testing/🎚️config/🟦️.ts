import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../../..");

/** 🌐️ Exercises the installed application composition through its real private session worker. */
export default defineConfig({
  root,
  test: {
    name: "hub-browser-session-authority",
    environment: "node",
    include: [],
    includeSource: ["✏️s/🧑‍💻dev/🧩️service-composition/👷️worker/🟦️.ts"],
    passWithNoTests: false,
  },
});
