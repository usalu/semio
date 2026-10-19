import { resolve } from "node:path";
import type { OwnedBuildMiddleware, OwnedBuildServer } from "@semio-tech/ui-react/build-tooling";
import { readFileSync } from "node:fs";

export const repoRoot = resolve(import.meta.dir, "../../../../../../..");
/** 🧪️ A complete owned dev server whose only live seam is the middleware registration. */
export const ownedServer = (use: (middleware: OwnedBuildMiddleware) => void): OwnedBuildServer => ({
  middlewares: { use },
  ws: { send() {} },
  config: { root: repoRoot, cacheDir: resolve(repoRoot, "node_modules/.vite"), base: "/", server: {} },
  watcher: { emit: () => false },
  httpServer: null,
});
export const uiCss = readFileSync(resolve(import.meta.dir, "../../../🖌️ui/🎨️.css"), "utf8");
export const paletteCss = readFileSync(resolve(import.meta.dir, "../../../🎨️palette/🎨️.css"), "utf8");
