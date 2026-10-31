import { viteCacheDirectory } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/⚡️caching/🌐️vite/📂️paths/🟦️.ts";
import { buildModePolicy, readBuildMode } from "../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
// #region 🔌️Adapters
import { createHash } from "node:crypto";
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineOwnedBuildConfigFactory, uiReactBuildPlugin, uiTailwindBuildPlugins, type OwnedBuildPlugin } from "../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/🛠️build-tooling/🟦️.ts";
import { playgroundStaticSiteBuildOptions, semioViteBuild } from "../../../../../../🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/🏗️builder/🌐️vite/🟦️.ts";
import { semioEmojiIndexHtmlVitePlugin, semioHostHtmlVitePlugin, type SemioHostHtmlSpec } from "../../../../../../🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/🏗️builder/🌐️vite/🌐️site-html/🟦️.ts";
import { semioReferencedAssetsVitePlugin } from "../../../../../../🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/🏗️builder/🌐️vite/🗃️assets/🟦️.ts";
import { semioServeCloseVitePlugin, semioServeUpgradeVitePlugin } from "../../../../../../🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/🏗️builder/🌐️vite/🚦️status-serve/🟦️.ts";
// #endregion 🔌️Adapters

const siteRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const bundleRoot = resolve(siteRoot, "📦️packages/🟦️typescript");
const repoRoot = resolve(siteRoot, "../../../..");
const staffage = resolve(repoRoot, "🎓️teaching/🏛️architecture/🧍️staffage");

/** 🌍️ The custom domain the static build is published under. Written into `CNAME` at the build root. */
export const STAFFAGE_SITE_HOST = "staffage.semio-tech.com";

/** 📑️ CDN headers: documents revalidate, hashed assets stay immutable, and frames are refused. */
export const STAFFAGE_SITE_HEADERS = [
  "/*",
  "  X-Content-Type-Options: nosniff",
  "  X-Frame-Options: DENY",
  "  Content-Security-Policy: frame-ancestors 'none'",
  "  Referrer-Policy: no-referrer",
  "/assets/*",
  "  Cache-Control: public, max-age=31536000, immutable",
  "/",
  "  Cache-Control: no-cache",
  "/index.html",
  "  Cache-Control: no-cache",
  "/404.html",
  "  Cache-Control: no-cache",
].join("\n") + "\n";

/** 🛡️ Release Content-Security-Policy: the site's own origin, hashed inline boot code, and blob images for exports. */
export function staffageContentSecurityPolicy(html: string): string {
  const hashes = (tag: string): string[] => [...html.matchAll(new RegExp(`<${tag}(?![^>]*\\ssrc=)[^>]*>([\\s\\S]*?)</${tag}>`, "gu"))].map((block) => `'sha256-${createHash("sha256").update(block[1]!).digest("base64")}'`);
  return [
    "default-src 'self'",
    `script-src ${["'self'", ...new Set(hashes("script"))].join(" ")}`,
    `style-src ${["'self'", ...new Set(hashes("style"))].join(" ")}`,
    "style-src-attr 'unsafe-inline'",
    "img-src 'self' data: blob:",
    "font-src 'self'",
    "connect-src 'self'",
    "manifest-src 'self'",
    "object-src 'none'",
    "base-uri 'none'",
    "form-action 'self'",
  ].join("; ");
}

/** 📄️ The site's document before the app runs: it assumes no language. Its title is stated in every language, English first, then German; a reader without scripts is told so in both. The static build names {@link STAFFAGE_SITE_HOST}. */
export const staffageHostDocument: SemioHostHtmlSpec = {
  cnameHost: STAFFAGE_SITE_HOST,
  title: [
    { lang: "en", text: "Architectural Staffage Archive" },
    { lang: "de", text: "Archiv architektonischer Staffage" },
  ],
  entry: "./🟦️.ts",
  loading: {
    title: [
      { lang: "en", text: "Architectural Staffage Archive" },
      { lang: "de", text: "Archiv architektonischer Staffage" },
    ],
  },
  noscript: [
    { lang: "en", text: "This site needs JavaScript." },
    { lang: "de", text: "Diese Seite braucht JavaScript." },
  ],
};

/** 🪪️ Vite: a release document names the manifest, both appearance colours, and a sealed policy, then writes `robots.txt`, `manifest.webmanifest` and `_headers`. */
function staffageReleaseDocumentVitePlugin(): OwnedBuildPlugin {
  let outDir = resolve(bundleRoot, "dist");
  let written = true;
  const head = [
    `<meta name="description" lang="en" content="An archive of handcrafted architectural staffage with dimensions, motion and physical models." />`,
    `<meta name="description" lang="de" content="Ein Archiv handgefertigter architektonischer Staffage mit Maßen, Bewegung und physikalischen Modellen." />`,
    `<meta name="theme-color" media="(prefers-color-scheme: light)" content="#f7f3e3" />`,
    `<meta name="theme-color" media="(prefers-color-scheme: dark)" content="#001117" />`,
    `<link rel="manifest" href="/manifest.webmanifest" />`,
  ].join("\n    ");
  return {
    name: "architecture-staffage-release-document",
    apply: "build",
    enforce: "post",
    transformIndexHtml: {
      order: "post",
      handler(html) {
        const described = html.replace("</title>", `</title>\n    ${head}`);
        return described.replace(/(<meta charset="[^"]*" \/>)/u, `$1\n    <meta http-equiv="Content-Security-Policy" content="${staffageContentSecurityPolicy(described)}" />`);
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
      writeFileSync(resolve(outDir, "_headers"), STAFFAGE_SITE_HEADERS);
      writeFileSync(resolve(outDir, "manifest.webmanifest"), `${JSON.stringify({ name: "Architectural Staffage Archive", lang: "en", short_name: "Staffage", start_url: "/", scope: "/", display: "standalone", background_color: "#f7f3e3", theme_color: "#f7f3e3", icons: [{ src: "/favicon.svg", type: "image/svg+xml", sizes: "any" }] }, null, 2)}\n`);
    },
  };
}

/** 🧍️ Vite: the staffage archive site, a static React and Tailwind bundle without a backend. Assets resolve from the domain root and land in the package's `dist`; a release build is what `https://${STAFFAGE_SITE_HOST}` serves. Vite's dependency cache is the one the repository issues for the build mode, and `TEACHING_ARCHITECTURE_STAFFAGE_WATCH=off` stops the dev server watching the sources.
 * @see ../../🟦️.ts — the entry
 * @see ../../📦️packages/🟦️typescript/📜️script.ts — the `dev-site` and `build` verbs */
export default defineOwnedBuildConfigFactory(({ mode }) => ({
  root: siteRoot,
  base: "/",
  publicDir: false,
  cacheDir: viteCacheDirectory(repoRoot, { ...process.env, SEMIO_BUILD_MODE: readBuildMode(mode) }),
  plugins: [
    semioServeCloseVitePlugin(),
    semioServeUpgradeVitePlugin(),
    ...semioHostHtmlVitePlugin(repoRoot, staffageHostDocument),
    semioEmojiIndexHtmlVitePlugin(siteRoot),
    ...semioReferencedAssetsVitePlugin(repoRoot),
    ...uiTailwindBuildPlugins(),
    uiReactBuildPlugin(),
    staffageReleaseDocumentVitePlugin(),
  ],
  build: playgroundStaticSiteBuildOptions({ ...semioViteBuild(undefined, buildModePolicy(readBuildMode(mode))), outDir: resolve(bundleRoot, "dist", readBuildMode(mode)) }),
  define: { "import.meta.vitest": "undefined" },
  server: {
    fs: { allow: [repoRoot] },
    ...(process.env.TEACHING_ARCHITECTURE_STAFFAGE_WATCH === "off" ? { hmr: false, watch: null } : {}),
  },
  resolve: {
    alias: [
      { find: /^@teaching\/staffage-react\/app$/, replacement: resolve(staffage, "🎯️targets/⚛️react/📦️packages/🟦️typescript/📚️app/🟦️.tsx") },
      { find: /^@teaching\/staffage-react$/, replacement: resolve(staffage, "🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx") },
      { find: /^@teaching\/staffage$/, replacement: resolve(staffage, "📦️packages/🟦️typescript/🟦️.ts") },
      { find: /^@semio-tech\/ui-react\/introduction$/, replacement: resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/👋️introduction/🟦️.tsx") },
      { find: /^@semio-tech\/ui-react$/, replacement: resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx") },
      { find: /^@semio-tech\/ui-react\/i18n$/, replacement: resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/🌐️i18n/🟦️.ts") },
      { find: /^@semio-tech\/ui-react\/chrome$/, replacement: resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🪟️chrome/🟦️.ts") },
      { find: /^@semio-tech\/framework$/, replacement: resolve(repoRoot, "🧰️framework/📦️packages/🟦️typescript/🟦️.ts") },
    ],
    dedupe: ["react", "react-dom"],
  },
}));
