import faviconDelivery from "../../../🌐️favicon/🔣️.json" with { type: "json" };
import { resolve } from "node:path";
import { existsSync, readFileSync, mkdirSync, writeFileSync, cpSync } from "node:fs";
import type { OwnedBuildMiddleware, OwnedBuildPlugin } from "../../../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🛠️build-tooling/🟦️.ts";
import { serveFileWithValidatorsV1 } from "../📤️file-serving/🟦️.ts";

/** @emoji 🔖️ Canonical semio emblem favicon `<link>` tags for playground and app `🌐️.html` heads. */
export const SEMIO_FAVICON_HEAD_HTML = `<link rel="icon" href="./${faviconDelivery.svg}" type="image/svg+xml" />\n    <link rel="icon" href="./${faviconDelivery.ico}" sizes="any" />`;

/** @emoji 🔖️ Repo-root paths for the round dark emblem SVG and ICO fallback (matches {@link SemioLogo}). */
export function semioFaviconSources(repoRoot: string): { readonly svg: string; readonly ico: string } {
  const logoRoot = resolve(repoRoot, "./🧰️framework/🔨️modules/🖼️assets/🪧️logos");
  return {
    svg: resolve(logoRoot, "🛡️emblem/🌘️dark-round/🖋️vector.svg"),
    ico: resolve(logoRoot, "🌐️favicon/🌘️dark-round/📏️size-32.ico"),
  };
}

const SEMIO_FAVICON_BLEED_RECT = '<rect width="350" height="350" fill="#001117"/>';

/** @emoji 🔖️ Favicon SVG with opaque bleed so ICO rasterization avoids white matte outside the round emblem. */
export function semioFaviconSvgMarkup(svgPath: string): string | undefined {
  if (!existsSync(svgPath)) {
    return undefined;
  }
  const raw = readFileSync(svgPath, "utf8");
  if (raw.includes(SEMIO_FAVICON_BLEED_RECT)) {
    return raw;
  }
  const open = raw.match(/<svg[^>]*>/)?.[0];
  if (!open) {
    return raw;
  }
  return raw.replace(open, `${open}${SEMIO_FAVICON_BLEED_RECT}`);
}

/** @emoji 🔖️ Resolved favicon content for one host: inline SVG markup plus an optional ICO fallback path. */
type FaviconContent = { readonly svgMarkup?: string; readonly icoPath?: string };

const STATIC_SITE_FAVICON_ALIASES = { svg: "favicon.svg", ico: "favicon.ico" } as const;

function createFaviconMiddleware(content: FaviconContent): OwnedBuildMiddleware {
  return (req, res, next) => {
    let url: string;
    try { url = decodeURIComponent(req.url?.split(/[?#]/, 1)[0] ?? ""); } catch { next(); return; }
    if ((url === `/${faviconDelivery.svg}` || url === `/${STATIC_SITE_FAVICON_ALIASES.svg}`) && content.svgMarkup) {
      res.setHeader("Content-Type", "image/svg+xml");
      res.end(content.svgMarkup);
      return;
    }
    if ((url === `/${faviconDelivery.ico}` || url === `/${STATIC_SITE_FAVICON_ALIASES.ico}`) && content.icoPath && existsSync(content.icoPath)) {
      serveFileWithValidatorsV1(req, res, content.icoPath, "image/x-icon");
      return;
    }
    next();
  };
}

/** @emoji 🔖️ Vite: serve and copy the given emblem SVG and bookmark ICO under their exact publication names. */
export function faviconVitePlugins(content: FaviconContent): OwnedBuildPlugin[] {
  const serveFavicon = createFaviconMiddleware(content);
  let outDir = resolve(process.cwd(), "dist");
  let writeOutput = true;
  return [
    {
      name: "semio-favicon-serve",
      enforce: "pre",
      configureServer(server) {
        server.middlewares.use(serveFavicon);
      },
      configurePreviewServer(server) {
        server.middlewares.use(serveFavicon);
      },
    },
    {
      name: "semio-favicon-build",
      apply: "build",
      enforce: "pre",
      configResolved(config) {
        outDir = resolve(config.root, config.build.outDir);
        writeOutput = config.build.write !== false;
      },
      closeBundle() {
        if (!writeOutput) return;
        const dist = outDir;
        mkdirSync(dist, { recursive: true });
        if (content.svgMarkup) {
          writeFileSync(resolve(dist, faviconDelivery.svg), content.svgMarkup);
          writeFileSync(resolve(dist, STATIC_SITE_FAVICON_ALIASES.svg), content.svgMarkup);
        }
        if (content.icoPath && existsSync(content.icoPath)) {
          cpSync(content.icoPath, resolve(dist, faviconDelivery.ico));
          cpSync(content.icoPath, resolve(dist, STATIC_SITE_FAVICON_ALIASES.ico));
        }
      },
    },
  ];
}

/** @emoji 🔖️ Vite: serve and copy semio emblem favicons at `/🛡️favicon.svg` and `/🔖️favicon.ico`. */
export function semioFaviconVitePlugin(repoRoot: string): OwnedBuildPlugin[] {
  const favicons = semioFaviconSources(repoRoot);
  return faviconVitePlugins({ svgMarkup: semioFaviconSvgMarkup(favicons.svg), icoPath: favicons.ico });
}
