// #region 🔌️Adapters
import { createHash } from "node:crypto";
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineOwnedBuildConfigFactory, uiReactBuildPlugin, uiTailwindBuildPlugins, type OwnedBuildPlugin } from "../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🛠️build-tooling/🟦️.ts";
import {
  playgroundStaticSiteBuildOptions,
  semioEmojiIndexHtmlVitePlugin,
  semioHostHtmlVitePlugin,
  semioHostTitleText,
  semioReferencedAssetsVitePlugin,
  semioServeCloseVitePlugin,
  semioServeUpgradeVitePlugin,
  semioViteProductionBuild,
  type SemioHostHtmlSpec,
} from "../../../../../🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts";
import catalog from "../../🔣️.json" with { type: "json" };
import deployment from "../../🚀️deploy/🔣️.json" with { type: "json" };
// #endregion 🔌️Adapters

const siteRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const bundleRoot = resolve(siteRoot, "📦️packages/🟦️typescript");
const repoRoot = resolve(siteRoot, "../../..");
const devProctor = `http://127.0.0.1:${process.env.PROCTOR_PORT ?? String(deployment.proctor.port)}`;
const gatewayRoutes = ["/instance", "/commands", "/queries", "/actors", "/scopes"];
const siteTitles = Object.entries(catalog.title).map(([lang, text]) => ({ lang, text }));

/** 📄️ The site's document before the app runs: it assumes no language. Its title and loading text are the catalog's
 * title in every language of the catalog, in the catalog's order (English, then German); a reader without scripts is
 * told so in both. The app sets `lang` and the title of its screens once it knows its reader's language. */
export const quizHostDocument: SemioHostHtmlSpec = {
  title: siteTitles,
  entry: "./🟦️.ts",
  loading: { title: siteTitles },
  noscript: [
    { lang: "en", text: "This site needs JavaScript." },
    { lang: "de", text: "Diese Seite braucht JavaScript." },
  ],
  cnameHost: deployment.site.host,
};

const siteTitle = semioHostTitleText(siteTitles);
const proctorOrigin = process.env.PROCTOR_URL ?? `https://${deployment.proctor.host}`;

/** 🛡️ The Content-Security-Policy of a release document: everything from the site's own origin, the inline boot scripts
 * and boot style of `html` by their SHA-256 (nothing else inline runs), and connections to `proctorOrigin` alone, over
 * HTTPS and its WebSocket twin. A `meta` policy cannot carry `frame-ancestors`; `_headers` adds it for CDNs that send
 * headers. */
export function quizContentSecurityPolicy(html: string, proctorOrigin: string): string {
  const hashes = (tag: string): string[] => [...html.matchAll(new RegExp(`<${tag}(?![^>]*\\ssrc=)[^>]*>([\\s\\S]*?)</${tag}>`, "gu"))].map((block) => `'sha256-${createHash("sha256").update(block[1]!).digest("base64")}'`);
  return [
    "default-src 'self'",
    `script-src ${["'self'", ...new Set(hashes("script"))].join(" ")}`,
    `style-src ${["'self'", ...new Set(hashes("style"))].join(" ")}`,
    "style-src-attr 'unsafe-inline'",
    "img-src 'self' data:",
    "font-src 'self'",
    `connect-src ${proctorOrigin} ${proctorOrigin.replace(/^http/u, "ws")}`,
    "manifest-src 'self'",
    "object-src 'none'",
    "base-uri 'none'",
    "form-action 'self'",
  ].join("; ");
}

/** 🪪️ Vite: what only a release document carries. Like the host document it assumes no language: it describes the site
 * once per language of the catalog. It names the manifest and the colours of both appearances, and is sealed with
 * {@link quizContentSecurityPolicy} as the first thing after the charset.
 * The build also writes `robots.txt` and `manifest.webmanifest` beside it once the bundle is closed, unless the build
 * writes nothing. */
function quizReleaseDocumentVitePlugin(proctorOrigin: string): OwnedBuildPlugin {
  let outDir = resolve(bundleRoot, "dist");
  let written = true;
  const attribute = (text: string): string => text.replace(/&/gu, "&amp;").replace(/"/gu, "&quot;").replace(/</gu, "&lt;");
  const head = [
    ...Object.entries(catalog.introduction.paragraphs[0]!).map(([language, text]) => `<meta name="description" lang="${language}" content="${attribute(text)}" />`),
    `<meta name="theme-color" media="(prefers-color-scheme: light)" content="#f7f3e3" />`,
    `<meta name="theme-color" media="(prefers-color-scheme: dark)" content="#001117" />`,
    `<link rel="manifest" href="/manifest.webmanifest" />`,
  ].join("\n    ");
  return {
    name: "architecture-quiz-release-document",
    apply: "build",
    enforce: "post",
    transformIndexHtml: {
      order: "post",
      handler(html) {
        const described = html.replace("</title>", `</title>\n    ${head}`);
        return described.replace(/(<meta charset="[^"]*" \/>)/u, `$1\n    <meta http-equiv="Content-Security-Policy" content="${quizContentSecurityPolicy(described, proctorOrigin)}" />`);
      },
    },
    configResolved(config) {
      outDir = resolve(config.root, config.build.outDir);
      written = config.build.write !== false;
    },
    closeBundle() {
      if (!written) return;
      mkdirSync(outDir, { recursive: true });
      writeFileSync(resolve(outDir, "robots.txt"), "User-agent: *\nAllow: /\n");
      writeFileSync(resolve(outDir, "manifest.webmanifest"), `${JSON.stringify({ name: siteTitle, short_name: "Quiz", start_url: "/", scope: "/", display: "standalone", background_color: "#f7f3e3", theme_color: "#f7f3e3", icons: [{ src: "/favicon.svg", type: "image/svg+xml", sizes: "any" }] }, null, 2)}\n`);
    },
  };
}

/** ❓️ Vite configuration of `@teaching/architecture-quiz`, the quiz website the CDN serves at the site host of
 * `🚀️deploy/🔣️.json`, written against the owned build contract of the UI package (no type of the build tool appears
 * here; React's and Tailwind's adapters come from the package that declares them).
 *
 * A build bakes the proctor origin into `import.meta.env.VITE_PROCTOR_URL` (`PROCTOR_URL`, else `https://` + the proctor
 * host), seals the document with a Content-Security-Policy that admits connections to that origin alone, and writes `CNAME`
 * with the site host; assets resolve from the domain root and land in the package's `dist`. The dev
 * server bakes nothing and proxies the gateway routes to the dev proctor on `PROCTOR_PORT` (default 8791), so dev and
 * tests stay same-origin; a proctor that goes away while a socket is proxied never takes the dev server along, whatever
 * the runtime. `TEACHING_ARCHITECTURE_QUIZ_CACHE` moves Vite's dependency cache, so a throw-away dev server
 * never rewrites the one a developer's own server reads, and `TEACHING_ARCHITECTURE_QUIZ_WATCH=off` stops it watching the
 * sources, so an edit never reloads the pages a test drives.
 * @see ../../🚀️deploy/🔣️.json — the site and proctor hosts
 * @see ../../🚀️deploy/🟦️.ts — `publish`, which verifies and stages this build for the CDN
 * @see ../../🧱️stack/🟦️.ts — the local stack that starts this server beside the proctor */
export default defineOwnedBuildConfigFactory(({ command }) => ({
  root: siteRoot,
  base: "/",
  publicDir: false,
  cacheDir: process.env.TEACHING_ARCHITECTURE_QUIZ_CACHE,
  plugins: [
    semioServeCloseVitePlugin(),
    semioServeUpgradeVitePlugin(),
    ...semioHostHtmlVitePlugin(repoRoot, quizHostDocument),
    semioEmojiIndexHtmlVitePlugin(siteRoot),
    ...semioReferencedAssetsVitePlugin(repoRoot),
    ...uiTailwindBuildPlugins(),
    uiReactBuildPlugin(),
    quizReleaseDocumentVitePlugin(proctorOrigin),
  ],
  build: playgroundStaticSiteBuildOptions({ ...semioViteProductionBuild(), outDir: resolve(bundleRoot, "dist") }),
  define: { "import.meta.vitest": "undefined", ...(command === "build" ? { "import.meta.env.VITE_PROCTOR_URL": JSON.stringify(proctorOrigin) } : {}) },
  server: {
    fs: { allow: [repoRoot] },
    proxy: Object.fromEntries(gatewayRoutes.map((route) => [route, { target: devProctor, ws: true }])),
    ...(process.env.TEACHING_ARCHITECTURE_QUIZ_WATCH === "off" ? { hmr: false, watch: null } : {}),
  },
  resolve: {
    alias: [
      { find: /^@semio-tech\/quiz-react$/, replacement: resolve(repoRoot, "🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx") },
      { find: /^@semio-tech\/quiz$/, replacement: resolve(repoRoot, "🧰️framework/🛍️products/❓️quiz/📦️packages/🟦️typescript/🟦️.ts") },
      { find: /^@semio-tech\/pets-react$/, replacement: resolve(repoRoot, "🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx") },
      { find: /^@semio-tech\/pets$/, replacement: resolve(repoRoot, "🧰️framework/🛍️products/🐾️pets/📦️packages/🟦️typescript/🟦️.ts") },
      { find: /^@semio-tech\/ui-react$/, replacement: resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx") },
      { find: /^@semio-tech\/ui-react\/i18n$/, replacement: resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🌐️i18n/🟦️.ts") },
      { find: /^@semio-tech\/ui-react\/chrome$/, replacement: resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🪟️chrome/🟦️.ts") },
      { find: /^@semio-tech\/framework-server$/, replacement: resolve(repoRoot, "🧰️framework/🛍️products/🖥️server/📦️packages/🟦️typescript/🟦️.ts") },
      { find: /^@semio-tech\/framework$/, replacement: resolve(repoRoot, "🧰️framework/📦️packages/🟦️typescript/🟦️.ts") },
    ],
    dedupe: ["react", "react-dom"],
  },
}));
