// #region 🔌️Adapters
import * as React from "react";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { createMemoryStoragePort } from "@semio-tech/framework";
import { currentStylingAppearanceName, resolveSemanticColorHex, setStylingAppearanceRoot } from "@semio-tech/ui-styling";
import { act, fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { ChromeControlHint } from "../../../💡️ChromeControlHint/🟦️.tsx";
import { DEFAULT_UI_DRIVER, UiDriverProvider } from "../../../🚗️UiDriver/🟦️.tsx";
import { Dialog, DialogContent, DialogTitle, DialogTrigger } from "../../../💬️Dialog/🟦️.tsx";
import { Popover, PopoverContent, PopoverTrigger } from "../../../🗨️Popover/🟦️.tsx";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "../../../🔽️Select/🟦️.tsx";
import { ShellScopeProvider, createShellScope } from "../../🟦️.tsx";

/** @emoji 🎚️ Minimal surface of the third-party `color` package used as the independent WCAG oracle.
 * Loaded through a non-literal specifier because the package ships no type declarations. */
interface ThirdPartyColor {
  contrast(other: ThirdPartyColor): number;
  luminosity(): number;
}
const thirdPartyColorSpecifier = "color";
const { default: Color } = (await import(thirdPartyColorSpecifier)) as { default: (value: string) => ThirdPartyColor };
// #endregion 🔌️Adapters

// #region 🎨️TokenOracle
/** @emoji 📂️ Absolute path of a repo file beside this test — vite serves the module under an `/@fs`
 * prefix in jsdom, which `fileURLToPath` keeps verbatim, so it is stripped back off here. */
function sourcePath(relative: string): string {
  return fileURLToPath(new URL(relative, import.meta.url)).replace(/^\/@fs(?=\/)/, "");
}

/** @emoji 📖️ Reads one of the two shipped stylesheets that own every floating-surface token. */
function styleSheet(name: "palette" | "ui"): string {
  return readFileSync(sourcePath(name === "palette" ? "../../../../🎨️styling/🎨️palette/🎨️.css" : "../../../../🎨️styling/🖌️ui/🎨️.css"), "utf8");
}

/** @emoji 🧱️ Body of the first `{ … }` block whose header matches and whose body mentions `must`. */
function cssBlock(css: string, header: string, must: string): string {
  for (let cursor = css.indexOf(header); cursor >= 0; cursor = css.indexOf(header, cursor + 1)) {
    const open = css.indexOf("{", cursor + header.length - 1);
    let depth = 0;
    for (let index = open; index < css.length; index++) {
      if (css[index] === "{") depth++;
      else if (css[index] === "}" && --depth === 0) {
        const body = css.slice(open + 1, index);
        if (body.includes(must)) return body;
        break;
      }
    }
  }
  throw new Error(`no ${header} block containing ${must}`);
}

/** @emoji 🔎️ Declared value of `property` inside a block body. */
function declaration(body: string, property: string): string {
  const match = new RegExp(`(?:^|[;{\\s])${property}:\\s*([^;]+);`).exec(body);
  if (!match) throw new Error(`no ${property} declaration`);
  return match[1]!.trim();
}

/** @emoji 🔢️ Multiplier `k` out of a `calc(… k * var(--…-step))` declaration. */
function stepMultiplier(value: string): number {
  const match = /([0-9.]+)\s*\*\s*var\(/.exec(value);
  if (!match) throw new Error(`no step multiplier in ${value}`);
  return Number(match[1]);
}

/** @emoji 🌈️ Named `--color-*` hexes of the generated palette. */
function paletteColors(): Record<string, string> {
  const colors: Record<string, string> = {};
  for (const match of styleSheet("palette").matchAll(/--color-([a-z0-9-]+):\s*(#[0-9a-fA-F]{6});/g)) colors[match[1]!] = match[2]!;
  return colors;
}

/** @emoji 🔗️ Follows a `var(--x)` chain through the palette to a literal hex. */
function resolveColor(value: string, colors: Record<string, string>, aliases: Record<string, string>): string {
  let current = value.trim();
  for (let hop = 0; hop < 8; hop++) {
    if (current.startsWith("#")) return current;
    const match = /^var\(\s*(--[a-z0-9-]+)\s*\)$/.exec(current);
    if (!match) throw new Error(`unresolvable color ${value}`);
    const name = match[1]!;
    current = aliases[name] ?? colors[name.replace("--color-", "")] ?? "";
    if (!current) throw new Error(`unknown token ${name}`);
  }
  throw new Error(`color alias cycle at ${value}`);
}

const SRGB_TO_LMS = [
  [0.4122214708, 0.5363325363, 0.0514459929],
  [0.2119034982, 0.6806995451, 0.1073969566],
  [0.0883024619, 0.2817188376, 0.6299787005],
];
const LMS_TO_OKLAB = [
  [0.2104542553, 0.793617785, -0.0040720468],
  [1.9779984951, -2.428592205, 0.4505937099],
  [0.0259040371, 0.7827717662, -0.808675766],
];
const OKLAB_TO_LMS = [
  [1, 0.3963377774, 0.2158037573],
  [1, -0.1055613458, -0.0638541728],
  [1, -0.0894841775, -1.291485548],
];
const LMS_TO_SRGB = [
  [4.0767416621, -3.3077115913, 0.2309699292],
  [-1.2684380046, 2.6097574011, -0.3413193965],
  [-0.0041960863, -0.7034186147, 1.707614701],
];

const apply = (matrix: number[][], vector: number[]): number[] => matrix.map((row) => row.reduce((sum, factor, index) => sum + factor * vector[index]!, 0));
const toLinear = (channel: number): number => (channel <= 0.04045 ? channel / 12.92 : ((channel + 0.055) / 1.055) ** 2.4);
const toGamma = (channel: number): number => (channel <= 0.0031308 ? channel * 12.92 : 1.055 * channel ** (1 / 2.4) - 0.055);
const channels = (hex: string): number[] => [1, 3, 5].map((offset) => Number.parseInt(hex.slice(offset, offset + 2), 16) / 255);
const toHex = (rgb: number[]): string => `#${rgb.map((channel) => Math.round(Math.min(1, Math.max(0, channel)) * 255).toString(16).padStart(2, "0")).join("")}`;

/** @emoji 🧪️ Owned `color-mix(in oklab, a, b p)` — the browser-side formula every level block uses. */
function oklabMix(a: string, b: string, fraction: number): string {
  const labA = apply(LMS_TO_OKLAB, apply(SRGB_TO_LMS, channels(a).map(toLinear)).map(Math.cbrt));
  const labB = apply(LMS_TO_OKLAB, apply(SRGB_TO_LMS, channels(b).map(toLinear)).map(Math.cbrt));
  const mixed = labA.map((value, index) => value * (1 - fraction) + labB[index]! * fraction);
  return toHex(apply(LMS_TO_SRGB, apply(OKLAB_TO_LMS, mixed).map((value) => value ** 3)).map(toGamma));
}

/** @emoji 🫥️ Composites a translucent fill over an opaque ground. */
function composite(fill: string, ground: string, alpha: number): string {
  const front = channels(fill);
  return toHex(channels(ground).map((back, index) => front[index]! * alpha + back * (1 - alpha)));
}

interface FloatingSurfacePaint {
  readonly appearance: "light" | "dark";
  readonly shell: string;
  readonly surface: string;
  readonly background: string;
  readonly foreground: string;
  readonly contrast: number;
}

/** @emoji 🗨️ Resolves what a `data-level="menu"` floating surface actually paints in one appearance,
 * straight out of the shipped stylesheets: the glass fill composited over the shell ground it floats
 * over, and the `--color-popover-foreground` text on top of it. */
function floatingSurfacePaint(appearance: "light" | "dark"): FloatingSurfacePaint {
  const ui = styleSheet("ui");
  const colors = paletteColors();
  const base = cssBlock(ui, appearance === "light" ? ":root {" : ".dark {", "--base:");
  const aliases: Record<string, string> = { "--base": declaration(base, "--base"), "--foreground": declaration(base, "--foreground") };
  const menu = cssBlock(ui, '[data-level="menu"] {', "--surface-bg:");
  const theme = cssBlock(ui, "@theme inline {", "--color-popover-foreground:");
  const levelStep = Number.parseFloat(declaration(theme, "--level-shade-step")) / 100;
  const elementStep = Number.parseFloat(declaration(theme, "--element-shade-step")) / 100;
  const alphaStep = Number.parseFloat(declaration(theme, "--glass-alpha-step"));
  expect(declaration(theme, "--color-popover-foreground")).toBe("var(--border-element-color)");
  expect(declaration(theme, "--color-element")).toBe("var(--border-element-color)");
  const shell = resolveColor(aliases["--base"]!, colors, aliases);
  const foregroundToken = resolveColor(aliases["--foreground"]!, colors, aliases);
  const surface = oklabMix(shell, foregroundToken, stepMultiplier(declaration(menu, "--surface-bg")) * levelStep);
  const background = composite(surface, shell, 1 - stepMultiplier(declaration(menu, "--surface-alpha")) * alphaStep);
  const foreground = oklabMix(colors["gray"]!, foregroundToken, stepMultiplier(declaration(menu, "--border-element-color")) * elementStep);
  return { appearance, shell, surface, background, foreground, contrast: Color(foreground).contrast(Color(background)) };
}
// #endregion 🎨️TokenOracle

// #region 🐚️ScopedShell
interface ScopedShell {
  readonly root: HTMLElement;
  readonly app: HTMLElement;
  readonly portal: HTMLElement;
  readonly scope: ReturnType<typeof createShellScope>;
  dispose(): void;
}

/** @emoji 🐚️ Mounts the exact DOM `FrameworkOsShell` builds: a `.semio-scope` root carrying the
 * appearance class, its app subtree, and the portal layer as the root's last child. */
function scopedShell(shellId: string, appearance: "light" | "dark"): ScopedShell {
  const root = document.createElement("div");
  const app = document.createElement("div");
  const portal = document.createElement("div");
  root.className = appearance === "dark" ? "semio-scope dark" : "semio-scope";
  root.dataset.uiAppearance = appearance;
  root.style.position = "relative";
  portal.style.position = "absolute";
  root.append(app, portal);
  document.body.append(root);
  const scope = createShellScope({ storage: createMemoryStoragePort(), initialLocale: "en", shellId });
  scope.rootRef.current = root;
  scope.portalLayerRef.current = portal;
  return { root, app, portal, scope, dispose: () => root.remove() };
}
// #endregion 🐚️ScopedShell

// #region 🪟️FloatingSurfaceAppearanceScope
describe("floating surfaces resolve the shell's appearance scope", () => {
  it("mounts every floating surface inside the shell's own portal layer, never on document.body", () => {
    const shell = scopedShell("floating-surface-scope", "dark");
    const view = render(
      <ShellScopeProvider scope={shell.scope}>
        <Select id="ui.example.picker" defaultOpen defaultValue="hex">
          <SelectTrigger>
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="hex">Hexagonal Mushroom Column</SelectItem>
            <SelectItem value="torus">Sphere Cut With Torus</SelectItem>
          </SelectContent>
        </Select>
        <Popover defaultOpen>
          <PopoverTrigger>Options</PopoverTrigger>
          <PopoverContent aria-label="Window options">Rename</PopoverContent>
        </Popover>
        <Dialog defaultOpen>
          <DialogTrigger>Command</DialogTrigger>
          <DialogContent>
            <DialogTitle>Command palette</DialogTitle>
          </DialogContent>
        </Dialog>
      </ShellScopeProvider>,
      { container: shell.app, baseElement: shell.root },
    );
    try {
      for (const slot of ["select-content", "popover-content", "dialog-portal"]) {
        const surface = shell.root.querySelector<HTMLElement>(`[data-slot="${slot}"]`);
        expect(surface, slot).not.toBeNull();
        expect(shell.portal.contains(surface!), slot).toBe(true);
        expect(surface!.closest(".semio-scope"), slot).toBe(shell.root);
        expect(surface!.parentElement === document.body, slot).toBe(false);
      }
      expect(document.body.querySelector(':scope > [data-slot="select-content"]')).toBeNull();
    } finally {
      view.unmount();
      shell.dispose();
    }
  });

  it("keeps a hover tooltip hint inside the appearance scope that owns its trigger", () => {
    vi.useFakeTimers();
    const shell = scopedShell("floating-hint-scope", "dark");
    const view = render(
      <ShellScopeProvider scope={shell.scope}>
        <UiDriverProvider driver={DEFAULT_UI_DRIVER}>
          <ChromeControlHint text="Close window" always>
            <button type="button">Close</button>
          </ChromeControlHint>
        </UiDriverProvider>
      </ShellScopeProvider>,
      { container: shell.app, baseElement: shell.root },
    );
    try {
      fireEvent.pointerEnter(shell.root.querySelector('[data-slot="chrome-control-hint"]')!);
      act(() => {
        vi.runAllTimers();
      });
      const hint = shell.root.querySelector<HTMLElement>('[data-slot="tooltip-content"]');
      expect(hint).not.toBeNull();
      expect(shell.portal.contains(hint!)).toBe(true);
    } finally {
      view.unmount();
      shell.dispose();
      vi.useRealTimers();
    }
  });

  it("gives each mounted shell its own floating surfaces so neither borrows the other's appearance", () => {
    const dark = scopedShell("floating-scope-dark", "dark");
    const light = scopedShell("floating-scope-light", "light");
    const picker = (value: string) => (
      <Select id={`ui.example.picker.${value}`} defaultOpen defaultValue={value}>
        <SelectTrigger>
          <SelectValue />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value={value}>{value}</SelectItem>
        </SelectContent>
      </Select>
    );
    const darkView = render(<ShellScopeProvider scope={dark.scope}>{picker("dark-row")}</ShellScopeProvider>, { container: dark.app, baseElement: dark.root });
    const lightView = render(<ShellScopeProvider scope={light.scope}>{picker("light-row")}</ShellScopeProvider>, { container: light.app, baseElement: light.root });
    try {
      const darkContent = dark.portal.querySelector<HTMLElement>('[data-slot="select-content"]')!;
      const lightContent = light.portal.querySelector<HTMLElement>('[data-slot="select-content"]')!;
      expect(darkContent.closest(".dark")).toBe(dark.root);
      expect(lightContent.closest(".dark")).toBeNull();
      expect(darkContent.dataset.level).toBe("menu");
      expect(lightContent.dataset.level).toBe("menu");
    } finally {
      darkView.unmount();
      lightView.unmount();
      dark.dispose();
      light.dispose();
    }
  });
});
// #endregion 🪟️FloatingSurfaceAppearanceScope

// #region 🎨️FloatingSurfaceContrast
describe("floating surface token pair stays readable in both appearances", () => {
  it("paints popover foreground on the menu glass at >= 4.5:1 in light and dark", () => {
    for (const appearance of ["light", "dark"] as const) {
      const paint = floatingSurfacePaint(appearance);
      expect(paint.contrast, `${appearance} ${paint.foreground} on ${paint.background}`).toBeGreaterThanOrEqual(4.5);
    }
  });

  it("proves the appearance scope is load-bearing: the opposite appearance's pair over the shell is unreadable", () => {
    const dark = floatingSurfacePaint("dark");
    const light = floatingSurfacePaint("light");
    const escaped = Color(light.foreground).contrast(Color(composite(light.surface, dark.shell, 0.4)));
    expect(escaped).toBeLessThan(3);
    expect(dark.contrast).toBeGreaterThan(escaped);
  });

  it("keeps the two appearances on opposite sides of their own shell ground", () => {
    expect(Color(floatingSurfacePaint("light").background).luminosity()).toBeGreaterThan(Color(floatingSurfacePaint("dark").background).luminosity());
  });
});
// #endregion 🎨️FloatingSurfaceContrast

// #region 🚧️PortalHostOwnership
describe("one owning layer resolves every floating surface host", () => {
  it("routes every portaling element through useShellFloatingSurfaceHost", () => {
    for (const element of ["🔽️Select", "🗨️Popover", "💬️Dialog", "🖱️ContextMenu", "💡️ChromeControlHint", "🌳️Tree"]) {
      const source = readFileSync(sourcePath(`../../../${element}/🟦️.tsx`), "utf8");
      expect(source, element).toContain("useShellFloatingSurfaceHost");
      expect(/\?\?\s*document\.body\s*,/.test(source), element).toBe(false);
      expect(/document\.body\s*:\s*null\)/.test(source), element).toBe(false);
    }
  });
});
// #endregion 🚧️PortalHostOwnership

// #region 🖼️CanvasAppearanceScope
describe("canvas paints resolve the shell's appearance scope, not the document root", () => {
  it("reads the registered shell root's appearance while documentElement stays light", () => {
    const dark = scopedShell("canvas-scope-dark", "dark");
    try {
      expect(document.documentElement.classList.contains("dark")).toBe(false);
      setStylingAppearanceRoot(null);
      expect(currentStylingAppearanceName()).toBe("light");
      const lightForeground = resolveSemanticColorHex("--foreground");
      setStylingAppearanceRoot(dark.root);
      expect(currentStylingAppearanceName()).toBe("dark");
      expect(resolveSemanticColorHex("--foreground")).not.toBe(lightForeground);
    } finally {
      setStylingAppearanceRoot(null);
      dark.dispose();
    }
  });

  it("falls back to documentElement once the registered root leaves the document", () => {
    const dark = scopedShell("canvas-scope-detached", "dark");
    setStylingAppearanceRoot(dark.root);
    expect(currentStylingAppearanceName()).toBe("dark");
    dark.dispose();
    expect(currentStylingAppearanceName()).toBe("light");
    setStylingAppearanceRoot(null);
  });
});
// #endregion 🖼️CanvasAppearanceScope
