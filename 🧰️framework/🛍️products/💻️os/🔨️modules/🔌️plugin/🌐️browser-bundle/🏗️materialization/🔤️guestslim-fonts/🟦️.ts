/** 🔤️ Guest slim typst fonts of the plugin web materialization: the staged font seed and its placement in one plugin module root. */
import { copyFileSync, existsSync, mkdirSync, statSync } from "node:fs";
import { dirname, join } from "node:path";
import { GUESTSLIM_FONT_RELATIVE } from "../🧾️contract/🟦️.ts";

/** 🔤️ The staged typst font seed `semio-framework-os-infinite:fonts` publishes — the one source of the
 * `🪞️vendor` font asset every plugin-module root (dev and release) serves. */
export function guestSlimTypstFontSeed(repoRoot: string): string {
  return join(repoRoot, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "♾️infinite", "📦️packages", "🦀️rust", "dist", "fonts", "🔤️guestslim-typst-fonts.bin");
}

/** 🫙️ Places `🪞️vendor/🔤️guestslim-typst-fonts.bin` in one plugin-module root from the staged seed — the
 * asset plugin workers' typst text path and the wgpu serve's readiness check read, for either profile.
 * `false` when no seed is staged yet; the caller decides whether it can build one. */
export function ensureGuestSlimTypstFontsAt(moduleRoot: string, repoRoot: string): boolean {
  const out = join(moduleRoot, GUESTSLIM_FONT_RELATIVE);
  if (existsSync(out) && statSync(out).size > 0) return true;
  const seed = guestSlimTypstFontSeed(repoRoot);
  if (!existsSync(seed) || statSync(seed).size === 0) return false;
  mkdirSync(dirname(out), { recursive: true });
  copyFileSync(seed, out);
  return true;
}
