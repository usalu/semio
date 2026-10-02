// #region 🔌️Adapters
import react from "@vitejs/plugin-react";
import { existsSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig, type Plugin } from "vite";
import { semioEmojiIndexHtmlVitePlugin, semioServeCloseVitePlugin } from "../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts";
// #endregion 🔌️Adapters

const target = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const storiesRoot = resolve(target, "📖️stories");
const bundleRoot = resolve(target, "📦️packages/🟦️typescript");
const product = resolve(target, "../..");
const repoRoot = resolve(product, "../../..");

/** 🧫️ The menagerie the gallery shows when `PETS_MENAGERIE` names none: the sample of the schema conformance vectors. */
export const SAMPLE_MENAGERIE = "🧰️framework/🛍️products/🐾️pets/🧫️fixtures/🧬️schema-conformance/🔣️.json";

/** 🏷️ The module the gallery's document imports its menagerie from. */
export const MENAGERIE_MODULE = "pets-stories:menagerie";

/** 🎪️ Vite: serves {@link MENAGERIE_MODULE} — `source`, every export of the file at `path` (relative to the repository root; a module or a JSON document), and `origin`, that path. A path that names no file yields `source = null`, so the gallery can say what is missing instead of failing to load. The file is imported statically, so an edit to it or to anything it imports reloads the page, and so does the file appearing or going away. */
export function menagerieVitePlugin(path: string): Plugin {
  const resolved = `\0${MENAGERIE_MODULE}`;
  const file = resolve(repoRoot, path).replaceAll("\\", "/");
  return {
    name: "pets-stories-menagerie",
    resolveId(source) {
      return source === MENAGERIE_MODULE ? resolved : undefined;
    },
    load(id) {
      if (id !== resolved) return undefined;
      return `${existsSync(file) ? `export * as source from ${JSON.stringify(file)};` : "export const source = null;"}\nexport const origin = ${JSON.stringify(path)};\n`;
    },
    configureServer(server) {
      const reload = (changed: string): void => {
        if (changed.replaceAll("\\", "/") !== file) return;
        const served = server.moduleGraph.getModuleById(resolved);
        if (served) server.moduleGraph.invalidateModule(served);
        server.ws.send({ type: "full-reload" });
      };
      server.watcher.add(file);
      server.watcher.on("add", reload);
      server.watcher.on("unlink", reload);
    },
  };
}

const port = process.env.PETS_STORIES_PORT || "6069";

/** 📖️ Vite configuration of the stories gallery of `@semio-tech/pets-react`: a dev server only, never a release build.
 *
 * It serves `📖️stories/🌐️.html` for the menagerie `PETS_MENAGERIE` names (a path relative to the repository root to a
 * module exporting a `Menagerie` or to a JSON document holding one; default {@link SAMPLE_MENAGERIE}). The dependency
 * cache is kept per port, so two galleries side by side never rewrite each other's.
 * @see ../../📖️stories/🟦️.tsx — the gallery
 * @see ../../📦️packages/🟦️typescript/📜️script.ts — `dev`, which starts this server on `PETS_STORIES_PORT` */
export default defineConfig({
  root: storiesRoot,
  base: "/",
  publicDir: false,
  cacheDir: resolve(bundleRoot, `node_modules/.vite/stories-${port}`),
  plugins: [semioServeCloseVitePlugin(), semioEmojiIndexHtmlVitePlugin(storiesRoot), menagerieVitePlugin(process.env.PETS_MENAGERIE || SAMPLE_MENAGERIE), react()],
  server: { fs: { allow: [repoRoot] } },
  resolve: {
    alias: [
      { find: /^@semio-tech\/pets-react$/, replacement: resolve(bundleRoot, "🟦️.tsx") },
      { find: /^@semio-tech\/pets$/, replacement: resolve(product, "📦️packages/🟦️typescript/🟦️.ts") },
    ],
    dedupe: ["react", "react-dom"],
  },
});
