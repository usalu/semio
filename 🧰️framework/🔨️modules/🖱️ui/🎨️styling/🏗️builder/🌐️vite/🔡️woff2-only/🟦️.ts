import { readFileSync } from "node:fs";
import type { OwnedBuildPlugin } from "../../../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🛠️build-tooling/🟦️.ts";

const KATEX_FALLBACK_SOURCES = /,\s*url\([^)]*\.(?:woff|ttf)\)\s*format\("(?:woff|truetype)"\)/gu;

/** 🔡️ Drops the `woff` and `ttf` fallback sources of one KaTeX stylesheet so only its `woff2` fonts are emitted. */
export function katexWoff2OnlyCssV1(css: string): string {
  return css.replace(KATEX_FALLBACK_SOURCES, "");
}

/** 🔡️ Ships only the `woff2` fonts of `katex/dist/katex.min.css`: every browser that runs the Demonstrator decodes `woff2`, so the `woff` and `ttf` copies are dead weight. */
export function semioKatexWoff2OnlyVitePlugin(): OwnedBuildPlugin {
  return {
    name: "semio-katex-woff2-only",
    enforce: "pre",
    load(id) {
      const path = id.split("?", 1)[0]!.replaceAll("\\", "/");
      return path.endsWith("/katex/dist/katex.min.css") ? katexWoff2OnlyCssV1(readFileSync(path, "utf8")) : null;
    },
  };
}
