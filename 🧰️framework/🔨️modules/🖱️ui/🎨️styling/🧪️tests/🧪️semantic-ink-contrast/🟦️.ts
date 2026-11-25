/** ♿️ Semantic ink (destructive, warning, success, info, muted) stays at WCAG AA 4.5:1 on chip and menu glass in both appearances; colord's a11y plugin is the third-party oracle. */
import { colord, extend } from "colord";
import a11yPlugin from "colord/plugins/a11y";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import type { Rgba8 } from "../../🌓️theme/🟦️.ts";
import * as Projection from "../../📽️projection/🟦️.ts";

extend([a11yPlugin]);

const tokens = Projection.loadTokens();
const levels = tokens.levels ?? Projection.LEVELS_DEFAULT;
const resolved = Projection.resolveAppearances(tokens);
const css = readFileSync(resolve(import.meta.dirname, "../../🖌️ui/🎨️.css"), "utf8");
const AA = 4.5;

function hex([r, g, b]: Rgba8): string {
  return `#${[r, g, b].map((c) => Math.round(c).toString(16).padStart(2, "0")).join("")}`;
}

function over(surface: Rgba8, alpha: number, backdrop: Rgba8): Rgba8 {
  return [0, 1, 2].map((i) => surface[i]! * alpha + backdrop[i]! * (1 - alpha)).concat(255) as unknown as Rgba8;
}

function mixIn(a: Rgba8, b: Rgba8, t: number): Rgba8 {
  return [0, 1, 2].map((i) => a[i]! * (1 - t) + b[i]! * t).concat(255) as unknown as Rgba8;
}

function cssInk(name: string, index: number): Rgba8 {
  const declarations = [...css.matchAll(new RegExp(`--${name}: var\\(--color-([a-z0-9-]+)\\);`, "gu"))];
  const token = declarations[index]![1]!;
  const value = tokens.colors[token]!;
  return [1, 3, 5].map((i) => parseInt(value.slice(i, i + 2), 16)).concat(255) as unknown as Rgba8;
}

describe("web and wgpu inks are the same paints", () => {
  for (const [appearance, index] of [["light", 0], ["dark", 1]] as const) {
    for (const [css, outcome] of [["destructive-foreground", "errorInk"], ["warning-ink", "warningInk"], ["success-ink", "successInk"], ["info-ink", "infoInk"]] as const) {
      it(`${appearance} --${css} is outcome.${outcome}`, () => {
        expect(cssInk(css, index)).toEqual(resolved[appearance]!.outcome![outcome]);
      });
    }
  }
});

const GLASS = [
  ["chip", 1],
  ["chip", 3],
  ["menu", 5],
] as const;

describe("semantic ink contrast on glass", () => {
  for (const [appearance, index] of [["light", 0], ["dark", 1]] as const) {
    const chrome = resolved[appearance]!.chrome!;
    const inks: Readonly<Record<string, Rgba8>> = {
      destructive: cssInk("destructive-foreground", index),
      warning: cssInk("warning-foreground", index),
      success: cssInk("success-foreground", index),
      info: cssInk("info-foreground", index),
      muted: chrome.mutedForeground!,
      warningInk: cssInk("warning-ink", index),
      successInk: cssInk("success-ink", index),
      infoInk: cssInk("info-ink", index),
      wgpuErrorInk: resolved[appearance]!.outcome!.errorInk!,
      wgpuWarningInk: resolved[appearance]!.outcome!.warningInk!,
      wgpuSuccessInk: resolved[appearance]!.outcome!.successInk!,
      wgpuInfoInk: resolved[appearance]!.outcome!.infoInk!,
    };
    for (const [role, step] of GLASS) {
      const level = levels.names[step]!;
      const key = `level${level[0]!.toUpperCase()}${level.slice(1)}`;
      const surface = chrome[key]!;
      const alpha = 1 - step * levels.glassAlphaStep;
      const backdrops = { base: chrome.base!, window: chrome.levelWindow! };
      for (const [backdropName, backdrop] of Object.entries(backdrops)) {
        const glass = over(surface, alpha, backdrop);
        const hover = mixIn(glass, chrome.foreground!, levels.hoverStepPercent / 100);
        for (const [inkName, ink] of Object.entries(inks)) {
          for (const [stateName, paint] of inkName === "muted" ? ([["rest", glass]] as const) : ([["rest", glass], ["hover", hover]] as const)) {
            it(`${appearance} ${inkName} on ${role} glass level ${level} over ${backdropName} (${stateName}) reads at 4.5:1`, () => {
              const ratio = colord(hex(ink)).contrast(hex(paint));
              expect(ratio).toBeGreaterThanOrEqual(AA);
            });
          }
        }
      }
    }
  }
});
