import { describe, it, expect } from "bun:test";
import { clearColorResolveCache, resolveColorHex, resolveSemanticColorHex, resolveSpatialAxisColors, SPATIAL_AXIS_COLOR_REFS, resolveColorRgba, serializeCanvasThemeJson, STYLING_BOARD_PALETTES, STYLING_TOKENS, syncSessionCanvasTheme } from "../../../📦️packages/🟦️typescript/🟦️.ts";
import { uiCss } from "../🧰️shared/🟦️.ts";

describe("styling resolve", () => {
  it("selection fill uses accent with emphasized text color so muted gray stays readable", () => {
    expect(uiCss).toMatch(/::selection\s*\{\s*background-color:\s*var\(--accent\);\s*color:\s*var\(--border-emphasized-color\);/);
    expect(uiCss).toMatch(/::-moz-selection\s*\{\s*background-color:\s*var\(--accent\);\s*color:\s*var\(--border-emphasized-color\);/);
  });

  it("scoped roots resolve their own authored emphasis palette", async () => {
    const { parse } = await import("postcss");
    const rules = parse(uiCss);
    for (const appearance of ["light", "dark"]) {
      const values: string[] = [];
      rules.walkRules(rule => {
        if (rule.selector.includes(appearance === "light" ? ":root" : ".dark")) rule.walkDecls("--border-emphasized-color", declaration => { values.push(declaration.value); });
      });
      expect(values.some(value => value.includes(`--theme-${appearance}-chrome-border-emphasized`))).toBe(true);
    }
    expect(uiCss).toMatch(/:root,\s*\.dark\s*\{[^}]*--color-emphasized:\s*var\(--foreground\)/);
  });

  it("keeps panel-tab toggle dividers normal even when the active fill recolors other borders", () => {
    expect(uiCss).toMatch(/\[data-slot="panel-tabs"\] > \[data-slot="panel-tab-button"\]:not\(:last-child\)\s*\{\s*box-shadow:\s*inset calc\(-1 \* var\(--stroke-hairline\)\) 0 0 0 var\(--border-normal-color\);/);
  });

  it("leaves flowing chips borderless while their silhouette owns the continuous outline", () => {
    expect(uiCss).toMatch(
      /\[data-window-silhouette-chip\],\s*\[data-window-silhouette-chip\] > \*\s*\{\s*border-width:\s*0 !important;\s*border-style:\s*none !important;\s*box-shadow:\s*none;/,
    );
    expect(uiCss).toMatch(
      /\[data-window-silhouette-chip\] > :is\([\s\S]*?\[data-slot="button-group"\][\s\S]*?\)\s*\{\s*-webkit-backdrop-filter:\s*none !important;\s*backdrop-filter:\s*none !important;\s*background-color:\s*transparent !important;/,
    );
  });

  it("keeps accessibility glass fallbacks scoped to painted regions while gaps stay cut out", () => {
    expect(uiCss).toMatch(
      /@supports not \(\(-webkit-backdrop-filter: blur\(1px\)\) or \(backdrop-filter: blur\(1px\)\)\) \{[\s\S]*?:is\(\.ui-glass, \.ui-veil\) \{\s*background-color: var\(--surface-bg\);/,
    );
    expect(uiCss).toMatch(
      /@media \(prefers-reduced-transparency: reduce\) \{[\s\S]*?:is\(\.ui-glass, \.ui-veil\) \{[\s\S]*?backdrop-filter: none;[\s\S]*?background-color: var\(--surface-bg\);[\s\S]*?\[data-window-silhouette-gap\] \{\s*background: transparent !important;\s*background-color: transparent !important;/,
    );
    expect(uiCss).toMatch(
      /@media \(forced-colors: active\) \{[\s\S]*?:is\(\.ui-glass, \.ui-veil\) \{[\s\S]*?background-color: Canvas;\s*color: CanvasText;[\s\S]*?\[data-window-silhouette-border\] path \{\s*stroke: CanvasText !important;[\s\S]*?\[data-window-silhouette-gap\] \{\s*background: transparent !important;\s*background-color: transparent !important;\s*forced-color-adjust: none;/,
    );
  });

  it("expands the clipped content plane without changing document or auto-size clearances", () => {
    expect(uiCss).toMatch(
      /\.window-silhouette-content-plane\s*\{\s*margin-block-start: calc\(-1 \* var\(--window-silhouette-top-clearance, 0px\)\);\s*margin-block-end: calc\(-1 \* var\(--window-silhouette-bottom-clearance, 0px\)\);\s*padding-block-start: var\(--window-silhouette-top-clearance, 0px\);\s*padding-block-end: var\(--window-silhouette-bottom-clearance, 0px\);/,
    );
    expect(uiCss).toMatch(
      /\.window-silhouette-content-plane:has\(\s*\[data-window-content-layout="edgeless"\],\s*\[data-slot="window-dead-line-scroll"\]\s*\):not\(:has\(\[data-slot="window"\]\)\)\s*\{\s*padding-block-start: 0;\s*padding-block-end: 0;/,
    );
  });

  it("never lets a content plane that hosts a whole window drop its chrome clearance", () => {
    const optOut = uiCss.slice(uiCss.indexOf(".window-silhouette-content-plane:has("));
    const selector = optOut.slice(0, optOut.indexOf("{")).replace(/\s+/g, " ").trim();
    expect(selector).toContain(':not(:has([data-slot="window"]))');
    expect(selector.indexOf(':not(:has([data-slot="window"]))')).toBeGreaterThan(selector.indexOf('[data-slot="window-dead-line-scroll"]'));
    expect(optOut.slice(optOut.indexOf("{"), optOut.indexOf("}"))).toMatch(/padding-block-start: 0;\s*padding-block-end: 0;/);
  });

  it("resolveColorHex resolves palette var refs headlessly", () => {
    clearColorResolveCache();
    expect(resolveColorHex("var(--color-secondary)", "gray")).toBe("#34d1bf");
    expect(resolveSemanticColorHex("border-element-color", "gray")).toBe("#7b827d");
  });

  it("resolveSpatialAxisColors maps X/Y/Z to primary/secondary/tertiary permanently", () => {
    clearColorResolveCache();
    expect(resolveSpatialAxisColors()).toEqual({ x: "#ff344f", y: "#34d1bf", z: "#fa9500" });
    expect(SPATIAL_AXIS_COLOR_REFS).toEqual({ x: "var(--color-primary)", y: "var(--color-secondary)", z: "var(--color-tertiary)" });
  });

  it("resolveColorRgba returns byte tuple", () => {
    clearColorResolveCache();
    expect(resolveColorRgba("var(--color-gray)", "gray")).toEqual([123, 130, 125, 255]);
  });

  it("serializeCanvasThemeJson dark labelFill differs from light", () => {
    const light = JSON.parse(serializeCanvasThemeJson("light")) as { labelFill: readonly number[] };
    const dark = JSON.parse(serializeCanvasThemeJson("dark")) as { labelFill: readonly number[] };
    expect(light.labelFill).toEqual(STYLING_BOARD_PALETTES.light.labelFill);
    expect(dark.labelFill).toEqual(STYLING_BOARD_PALETTES.dark.labelFill);
    expect(dark.labelFill).not.toEqual(light.labelFill);
  });

  it("resolveColorHex foreground flips with html.dark appearance", () => {
    clearColorResolveCache();
    const previousDocument = globalThis.document;
    const classSet = new Set<string>();
    globalThis.document = {
      documentElement: {
        get className() {
          return [...classSet].join(" ");
        },
        set className(value: string) {
          classSet.clear();
          for (const part of value.split(/\s+/u).filter(Boolean)) classSet.add(part);
        },
        classList: {
          contains: (name: string) => classSet.has(name),
          add: (...names: string[]) => {
            for (const name of names) classSet.add(name);
          },
          remove: (...names: string[]) => {
            for (const name of names) classSet.delete(name);
          },
        },
      },
      createElement: () => {
        throw new Error("css probe unavailable in this test");
      },
    } as unknown as Document;
    try {
      document.documentElement.classList.remove("dark");
      clearColorResolveCache();
      const lightFg = resolveColorHex("var(--color-foreground)", "dark");
      document.documentElement.classList.add("dark");
      clearColorResolveCache();
      const darkFg = resolveColorHex("var(--color-foreground)", "light");
      expect(lightFg).toBe(STYLING_TOKENS.dark);
      expect(darkFg).toBe(STYLING_TOKENS.light);
      expect(darkFg).not.toBe(lightFg);
    } finally {
      globalThis.document = previousDocument;
      clearColorResolveCache();
    }
  });

  it("resolveColorHex border-emphasized flips with html.dark appearance", () => {
    clearColorResolveCache();
    const previousDocument = globalThis.document;
    const classSet = new Set<string>();
    globalThis.document = {
      documentElement: {
        classList: {
          contains: (name: string) => classSet.has(name),
          add: (...names: string[]) => {
            for (const name of names) classSet.add(name);
          },
          remove: (...names: string[]) => {
            for (const name of names) classSet.delete(name);
          },
        },
      },
      createElement: () => {
        throw new Error("css probe unavailable in this test");
      },
    } as unknown as Document;
    try {
      document.documentElement.classList.remove("dark");
      clearColorResolveCache();
      const lightEmphasis = resolveColorHex("var(--border-emphasized-color)", "gray");
      document.documentElement.classList.add("dark");
      clearColorResolveCache();
      const darkEmphasis = resolveColorHex("var(--border-emphasized-color)", "gray");
      expect(lightEmphasis).toBe(STYLING_TOKENS.dark);
      expect(darkEmphasis).toBe(STYLING_TOKENS.light);
      expect(darkEmphasis).not.toBe(lightEmphasis);
    } finally {
      globalThis.document = previousDocument;
      clearColorResolveCache();
    }
  });

  it("syncSessionCanvasTheme pushes serialized palette into a session", () => {
    const calls: string[] = [];
    syncSessionCanvasTheme({
      setCanvasThemeJson(json: string) {
        calls.push(json);
      },
    });
    expect(calls).toHaveLength(1);
    const parsed = JSON.parse(calls[0]!) as { labelFill: readonly number[] };
    expect(parsed.labelFill).toEqual(STYLING_BOARD_PALETTES.light.labelFill);
  });
});
