import type { OwnedBuildPlugin, OwnedBuildMiddleware } from "../../../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🛠️build-tooling/🟦️.ts";
import { resolve } from "node:path";
import { mkdirSync, writeFileSync, existsSync, readFileSync } from "node:fs";
import { SEMIO_FAVICON_HEAD_HTML, faviconVitePlugins, semioFaviconVitePlugin } from "../🔖️favicon/🟦️.ts";
import { PLAYGROUND_PLAY_BOOT_REVEAL_SCRIPT, semioHostBootHeadTags } from "../🚀️boot-html/🟦️.ts";

/** 🏷️ The host-chrome surface of a shell brand (structural subset of `framework/core/js`'s `ShellBrand`, so this styling layer never imports framework types). */
export type ShellBrandHostChrome = {
  readonly windowTitle: string;
  readonly logoSvg?: string;
  readonly faviconIcoPath?: string;
  /** 🌐️ Custom domain this brand's static build deploys to (e.g. GitHub Pages) — written verbatim into a `CNAME` file at the build root. */
  readonly cnameHost?: string;
};

/** 🚫️ Vite: writes `dist/.nojekyll` on every build (unconditionally — any static host that runs
 * Jekyll, e.g. GitHub Pages, silently drops files/dirs starting with `_` otherwise, breaking Vite's own
 * `__vite-browser-external-*.js` shim chunk) and `dist/CNAME` when a brand declares `cnameHost`. */
export function staticDeployMarkerVitePlugins(cnameHost: string | undefined): OwnedBuildPlugin[] {
  let outDir = resolve(process.cwd(), "dist");
  let writeOutput = true;
  return [
    {
      name: "static-deploy-markers",
      apply: "build",
      enforce: "pre",
      configResolved(config) {
        outDir = resolve(config.root, config.build.outDir);
        writeOutput = config.build.write !== false;
      },
      closeBundle() {
        if (!writeOutput) return;
        mkdirSync(outDir, { recursive: true });
        writeFileSync(resolve(outDir, ".nojekyll"), "");
        if (cnameHost) writeFileSync(resolve(outDir, "CNAME"), `${cnameHost}\n`);
      },
    },
  ];
}

/** 🧭️ Rewrites Vite's SPA fallback target `/index.html` onto the constitutional emoji entry path. */
export function rewriteSpaFallbackToEmojiEntry(url: string, entryPath: string): string {
  const [pathOnly, ...rest] = url.split(/(?=[?#])/);
  const base = pathOnly ?? url;
  if (base !== "/index.html") return url;
  return `${entryPath}${rest.join("")}`;
}

function semioEmojiIndexHtmlRootRewrite(entry: string): OwnedBuildMiddleware {
  return (req, _res, next) => {
    const url = req.url ?? "";
    if (url === "/" || url.startsWith("/?")) req.url = `${entry}${url.slice(1)}`;
    next();
  };
}

function semioEmojiIndexHtmlSpaFallbackRewrite(entry: string): OwnedBuildMiddleware {
  return (req, _res, next) => {
    const url = req.url ?? "";
    const nextUrl = rewriteSpaFallbackToEmojiEntry(url, entry);
    if (nextUrl !== url) req.url = nextUrl;
    next();
  };
}

/** 📄️ Conventional static-host entry filenames emitted beside the constitutional emoji HTML entry. */
export const STATIC_SITE_HTML_ALIASES = ["index.html", "404.html"] as const;

/** 🌐️ Vite: treat hand-authored `🌐️.html` as the app index (`/` + build input). Vite's default
 * `index.html` name does not match the constitutional emoji entry filename. */
export function semioEmojiIndexHtmlVitePlugin(rootDir: string, fileName = "🌐️.html"): OwnedBuildPlugin {
  const entry = `/${fileName}`;
  let outDir = resolve(process.cwd(), "dist");
  let writeOutput = true;
  return {
    name: "semio-emoji-index-html",
    enforce: "pre",
    config() {
      return {
        build: {
          rollupOptions: {
            input: resolve(rootDir, fileName),
          },
        },
      };
    },
    configResolved(config) {
      outDir = resolve(config.root, config.build.outDir);
      writeOutput = config.build.write !== false;
    },
    closeBundle() {
      if (!writeOutput) return;
      const source = resolve(outDir, fileName);
      if (!existsSync(source)) return;
      const html = readFileSync(source);
      for (const alias of STATIC_SITE_HTML_ALIASES) {
        writeFileSync(resolve(outDir, alias), html);
      }
    },
    configureServer(server) {
      server.middlewares.use(semioEmojiIndexHtmlRootRewrite(entry));
      return () => {
        server.middlewares.use(semioEmojiIndexHtmlSpaFallbackRewrite(entry));
      };
    },
    configurePreviewServer(server) {
      server.middlewares.use(semioEmojiIndexHtmlRootRewrite(entry));
      return () => {
        server.middlewares.use(semioEmojiIndexHtmlSpaFallbackRewrite(entry));
      };
    },
  };
}

/** 🏷️ Vite: brand-aware host chrome — rewrites the `<title>` to the brand's `windowTitle`, serves/copies the brand mark at `/🛡️favicon.svg` (ICO only when the brand provides one), and writes the static-deploy markers above; no brand ⇒ canonical semio favicons (still with `.nojekyll`). */
export function semioBrandHtmlVitePlugins(repoRoot: string, brand: ShellBrandHostChrome | undefined): OwnedBuildPlugin[] {
  if (!brand) return [...semioFaviconVitePlugin(repoRoot), ...staticDeployMarkerVitePlugins(undefined)];
  return [
    ...faviconVitePlugins({ svgMarkup: brand.logoSvg, icoPath: brand.faviconIcoPath ? resolve(repoRoot, brand.faviconIcoPath) : undefined }),
    ...staticDeployMarkerVitePlugins(brand.cnameHost),
    {
      name: "semio-brand-html",
      transformIndexHtml: {
        order: "pre",
        handler: (html) => html.replace(/<title>[^<]*<\/title>/, `<title>${brand.windowTitle}</title>`),
      },
    },
  ];
}

/** 🗣️ One text of a host document in one language: a BCP 47 tag and the text in that language. */
export type SemioHostHtmlText = { readonly lang: string; readonly text: string };

/** 🗣️ A text of a host document: one string in the document's language, or — for a document that assumes no
 * language — the same text once per offered language, in the order the languages are offered. */
export type SemioHostHtmlTexts = string | readonly SemioHostHtmlText[];

/** 🧬️ Full-document spec for a semio host `🌐️.html`: title, entry module, mount point, and
 * optional CSP + pre-mount loading copy — everything an app needs beyond the shared boot scripts so it
 * stops hand-authoring its own splash screen and `<style>` blocks. A document whose `title` is given per language
 * declares no language of its own: `<html>` carries no `lang`, the title joins the languages, every other text marks
 * each part with its `lang`, and the app sets `lang` once it knows its reader's. `noscript` is what a reader without
 * scripts is told instead of the app. */
export type SemioHostHtmlSpec = {
  readonly title: SemioHostHtmlTexts;
  readonly entry: string;
  readonly rootId?: string;
  readonly bodyClass?: string;
  readonly csp?: string;
  readonly loading?: { readonly title: SemioHostHtmlTexts };
  readonly noscript?: SemioHostHtmlTexts;
  /** 🌐️ Custom domain this app's static build deploys to (e.g. GitHub Pages) — written verbatim into a
   * `CNAME` file at the build root, alongside the always-written `.nojekyll` marker. */
  readonly cnameHost?: string;
};

const SEMIO_HOST_TEXT_SEPARATOR = " · ";

function semioHostEscapedText(text: string): string {
  return text.replace(/&/gu, "&amp;").replace(/</gu, "&lt;").replace(/>/gu, "&gt;").replace(/"/gu, "&quot;");
}

/** 🏷️ The title of a host document as plain text: the one string, or every language's title joined. */
export function semioHostTitleText(title: SemioHostHtmlTexts): string {
  return typeof title === "string" ? title : title.map((part) => semioHostEscapedText(part.text)).join(SEMIO_HOST_TEXT_SEPARATOR);
}

/** 🗣️ A text as markup: the one string as it is, or one element per language carrying its `lang`. */
function semioHostTextsHtml(texts: SemioHostHtmlTexts, tag: "span" | "p", attributes = ""): string {
  if (typeof texts === "string") return tag === "span" ? texts : `<${tag}${attributes}>${texts}</${tag}>`;
  return texts.map((part) => `<${tag} lang="${semioHostEscapedText(part.lang)}"${attributes}>${semioHostEscapedText(part.text)}</${tag}>`).join(tag === "span" ? SEMIO_HOST_TEXT_SEPARATOR : "");
}

/** 🪧️ Pre-mount placeholder markup shown inside `#{rootId}` until the entry module mounts and
 * replaces it — inline-styled so it renders before any external stylesheet loads. */
function semioHostLoadingHtml(loading: SemioHostHtmlSpec["loading"]): string {
  if (!loading) {
    return "";
  }
  return `<div style="display:flex;align-items:center;justify-content:center;height:100%;font:14px system-ui,sans-serif">${semioHostTextsHtml(loading.title, "span")}</div>`;
}

/** 🚫️ What a reader without scripts gets: the boot style keeps `<body>` hidden until a script reveals it, so each
 * paragraph makes itself visible. */
function semioHostNoscriptHtml(noscript: SemioHostHtmlSpec["noscript"]): string {
  if (!noscript) {
    return "";
  }
  return `\n    <noscript>${semioHostTextsHtml(noscript, "p", ' style="visibility:visible;margin:1em;font:14px system-ui,sans-serif"')}</noscript>`;
}

/** 📄️ Generates a complete semio host `🌐️.html` document: doctype/head (title, favicon,
 * optional CSP, boot style + appearance/theme scripts) and body (`#{rootId}` mount with pre-mount loading
 * copy, the reveal script, and the entry module script) — the single source of truth
 * {@link semioHostHtmlVitePlugin} renders from, reusable as-is by non-Vite hosts such as a VS Code webview. */
export function semioHostHtmlString(spec: SemioHostHtmlSpec): string {
  const rootId = spec.rootId ?? "root";
  const cspTag = spec.csp ? `<meta http-equiv="Content-Security-Policy" content="${spec.csp}" />\n    ` : "";
  const headTags = semioHostBootHeadTags()
    .map((tag) => (tag.tag === "style" ? `<style>${tag.children}</style>` : `<script>${tag.children}</script>`))
    .join("\n    ");
  return `<!doctype html>
<html${typeof spec.title === "string" ? ' lang="en"' : ""}>
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    ${cspTag}<title>${semioHostTitleText(spec.title)}</title>
    ${SEMIO_FAVICON_HEAD_HTML}
    ${headTags}
  </head>
  <body${spec.bodyClass ? ` class="${spec.bodyClass}"` : ""}>
    <div id="${rootId}">${semioHostLoadingHtml(spec.loading)}</div>${semioHostNoscriptHtml(spec.noscript)}
    <script>${PLAYGROUND_PLAY_BOOT_REVEAL_SCRIPT}</script>
    <script type="module" src="${spec.entry}"></script>
  </body>
</html>
`;
}

/** 🎬️ Vite: renders {@link semioHostHtmlString} as the app's `🌐️.html` on every request/build
 * (full-document replace, `order: "pre"` so later plugins such as `@vitejs/plugin-react`'s HMR preamble
 * still layer on top), bundles semio favicon serving ({@link semioFaviconVitePlugin}), and writes the
 * static-deploy markers ({@link staticDeployMarkerVitePlugins} — `.nojekyll` always, `CNAME` when
 * `spec.cnameHost` is set) — one call wires an app's whole boot + deploy surface instead of a
 * hand-authored `🌐️.html` plus a separate build-output step. */
export function semioHostHtmlVitePlugin(repoRoot: string, spec: SemioHostHtmlSpec): OwnedBuildPlugin[] {
  return [
    ...semioFaviconVitePlugin(repoRoot),
    ...staticDeployMarkerVitePlugins(spec.cnameHost),
    {
      name: "semio-host-html",
      transformIndexHtml: {
        order: "pre",
        handler() {
          return semioHostHtmlString(spec);
        },
      },
    },
  ];
}
