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

/** 🏷️ The module the gallery's document imports its menagerie from. */
export const MENAGERIE_MODULE = "pets-stories:menagerie";

/** 🎪️ Vite: serves {@link MENAGERIE_MODULE} — `source`, every export of the file at `path` (relative to the repository root; a module or a JSON document), and `origin`, that path. A path that names no file yields `source = null`, so the gallery can say what is missing instead of failing to load. The file is imported statically, so an edit to it or to anything it imports reloads the page, and so does the file appearing or going away. */
export function menagerieVitePlugin(path?: string): Plugin {
  const resolved = `\0${MENAGERIE_MODULE}`;
  const file = path ? resolve(repoRoot, path).replaceAll("\\", "/") : undefined;
  return {
    name: "pets-stories-menagerie",
    resolveId(source) {
      return source === MENAGERIE_MODULE ? resolved : undefined;
    },
    load(id) {
      if (id !== resolved) return undefined;
      return `${file && existsSync(file) ? `export * as source from ${JSON.stringify(file)};` : "export const source = null;"}\nexport const origin = ${JSON.stringify(path ?? "")};\n`;
    },
    configureServer(server) {
      if (!file) return;
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

/** 🎬️ The module the gallery's document imports the director from — and the module the pet layer imports in place of the core while this dev server serves it. */
export const DIRECTOR_MODULE = "pets-stories:director";

/** 🕹️ Vite: the gallery's seam into the stage of the pet layer, which exists only on this dev server and never in a release build.
 *
 * {@link DIRECTOR_MODULE} re-exports the whole core and a mutable `director` that holds the core's `advance` and
 * `frameOf`; its own `advance` and `frameOf` call whatever the director holds. The layer module's import of the core
 * (`@semio-tech/pets`, or the path its alias resolves to) is resolved to this module, every other import of the core
 * to the core itself. So the gallery can read every frame and stage the layer computes and fold forced changes into the
 * stage between two frames by wrapping the director's two functions, while the layer's code stays exactly what a host
 * runs: the test seam lives behind the stories, never in the product. */
export function directorVitePlugin(): Plugin {
  const resolved = `\0${DIRECTOR_MODULE}`;
  const core = resolve(product, "📦️packages/🟦️typescript/🟦️.ts").replaceAll("\\", "/");
  const layer = resolve(target, "🔨️modules/🫧️layer/🟦️.tsx").replaceAll("\\", "/");
  return {
    name: "pets-stories-director",
    enforce: "pre",
    resolveId(source, importer) {
      if (source === DIRECTOR_MODULE) return resolved;
      if (importer === undefined || importer.replaceAll("\\", "/").split("?")[0] !== layer) return undefined;
      return source === "@semio-tech/pets" || source.replaceAll("\\", "/") === core ? resolved : undefined;
    },
    load(id) {
      if (id !== resolved) return undefined;
      const from = JSON.stringify(core);
      return [
        `import { advance as advanced, frameOf as framed } from ${from};`,
        `export * from ${from};`,
        "export const director = { advance: advanced, frameOf: framed };",
        "export function advance(menagerie, stage, events) { return director.advance(menagerie, stage, events); }",
        "export function frameOf(menagerie, stage) { return director.frameOf(menagerie, stage); }",
        "",
      ].join("\n");
    },
  };
}

const port = process.env.PETS_STORIES_PORT || "6069";

/** 📖️ Vite configuration of the stories gallery of `@semio-tech/pets-react`: a dev server only, never a release build.
 *
 * It serves `📖️stories/🌐️.html` for the menagerie `PETS_MENAGERIE` names (a path relative to the repository root to a
 * module exporting a `Menagerie` or to a JSON document holding one; absent input leaves the gallery empty), with the
 * director of {@link directorVitePlugin} between the pet layer and the core. The dependency cache is kept per port, so
 * two galleries side by side never rewrite each other's.
 * @see ../../📖️stories/🟦️.tsx — the gallery
 * @see ../../📦️packages/🟦️typescript/📜️script.ts — `dev`, which starts this server on `PETS_STORIES_PORT` */
export default defineConfig({
  root: storiesRoot,
  base: "/",
  publicDir: false,
  cacheDir: resolve(bundleRoot, `node_modules/.vite/stories-${port}`),
  plugins: [semioServeCloseVitePlugin(), semioEmojiIndexHtmlVitePlugin(storiesRoot), menagerieVitePlugin(process.env.PETS_MENAGERIE), directorVitePlugin(), react()],
  server: { fs: { allow: [repoRoot] } },
  resolve: {
    alias: [
      { find: /^@semio-tech\/pets-react$/, replacement: resolve(bundleRoot, "🟦️.tsx") },
      { find: /^@semio-tech\/pets$/, replacement: resolve(product, "📦️packages/🟦️typescript/🟦️.ts") },
    ],
    dedupe: ["react", "react-dom"],
  },
});
