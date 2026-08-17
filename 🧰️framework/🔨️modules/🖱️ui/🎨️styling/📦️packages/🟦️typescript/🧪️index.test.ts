import { describe, expect, it } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import {
  clearColorResolveCache,
  resolveColorHex,
  resolveColorRgba,
  resolveSemanticColorHex,
  resolveSpatialAxisColors,
  serializeCanvasThemeJson,
  syncSessionCanvasTheme,
  SPATIAL_AXIS_COLOR_REFS,
  STYLING_BOARD_PALETTES,
  STYLING_TOKENS,
  elementStateAttributes,
  elementStateHidden,
  resolveElementFillKind,
  resolveElementState,
} from "./📦️index.ts";
import { meshCollectionVitePlugin, resolveSemioAssetRoot, SEMIO_ASSET_ROOT, type PlaygroundAssetSpec } from "../../📦️packages/🦀️rust/🟦️vite-elements-assets.ts";

const repoRoot = resolve(import.meta.dir, "../../../../../..");
const uiCss = readFileSync(resolve(import.meta.dir, "🎨️ui.css"), "utf8");
const paletteCss = readFileSync(resolve(import.meta.dir, "../../🎨️palette.css"), "utf8");

describe("palette asset urls", () => {
  it("every @font-face url in palette.css resolves under SEMIO_ASSET_ROOT", () => {
    const assetRoot = resolveSemioAssetRoot(repoRoot);
    const urls = [...paletteCss.matchAll(/url\("(\/asset\/[^"]+)"\)/g)].map((m) => m[1]!);
    expect(urls.length).toBeGreaterThan(0);
    for (const url of urls) {
      const rel = url.slice("/asset/".length);
      expect(existsSync(resolve(assetRoot, rel))).toBe(true);
    }
  });
});

describe("styling resolve", () => {
  it("selection fill uses accent with emphasized text color so muted gray stays readable", () => {
    expect(uiCss).toMatch(/::selection\s*\{\s*background-color:\s*var\(--accent\);\s*color:\s*var\(--border-emphasized-color\);/);
    expect(uiCss).toMatch(/::-moz-selection\s*\{\s*background-color:\s*var\(--accent\);\s*color:\s*var\(--border-emphasized-color\);/);
  });

  it("keeps panel-tab toggle dividers normal even when the active fill recolors other borders", () => {
    expect(uiCss).toMatch(/\[data-slot="panel-tabs"\] > \[data-slot="panel-tab-button"\]\s*\{\s*border-inline-end-color:\s*var\(--border-normal-color\) !important;/);
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
      /\.window-silhouette-content-plane:has\(\s*\[data-window-content-layout="edgeless"\],\s*\[data-slot="window-dead-line-scroll"\]\s*\)\s*\{\s*padding-block-start: 0;\s*padding-block-end: 0;/,
    );
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
    const light = JSON.parse(serializeCanvasThemeJson("light")) as { labelFill: number[] };
    const dark = JSON.parse(serializeCanvasThemeJson("dark")) as { labelFill: number[] };
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

  it("syncSessionCanvasTheme pushes serialized palette into a session", () => {
    const calls: string[] = [];
    syncSessionCanvasTheme({
      setCanvasThemeJson(json: string) {
        calls.push(json);
      },
    });
    expect(calls).toHaveLength(1);
    const parsed = JSON.parse(calls[0]!) as { labelFill: number[] };
    expect(parsed.labelFill).toEqual(STYLING_BOARD_PALETTES.light.labelFill);
  });
});

describe("puzzle3d mesh-collection asset spec", () => {
  const puzzle3dMeshSpec: Extract<PlaygroundAssetSpec, { kind: "mesh-collection" }> = {
    kind: "mesh-collection",
    route: "/mesh",
    roots: [
      "./🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🎨️representation",
      "./♻️mit-bestand/🖼️asset/🏚️abbau-aufbau",
    ],
    placeholder: "./🧰️framework/🔨️modules/🖼️assets/🥽️mesh/🧊️placeholder.glb",
    filterFromExamples: true,
  };

  it("resolves kit glb roots and shared placeholder", () => {
    expect(existsSync(resolve(repoRoot, puzzle3dMeshSpec.roots[0]!, "🧊️capsule_J.glb"))).toBe(true);
    expect(existsSync(resolve(repoRoot, puzzle3dMeshSpec.placeholder))).toBe(true);
  });

  it("registers a generic mesh-collection serve/build program pair", () => {
    const plugins = meshCollectionVitePlugin(repoRoot, puzzle3dMeshSpec);
    expect(plugins.map((plugin) => plugin.name)).toEqual(["mesh-collection-serve/mesh", "mesh-collection-build/mesh"]);
  });

  it("includes 🧊️base.glb for shooting's default fixture", () => {
    expect(existsSync(resolve(repoRoot, puzzle3dMeshSpec.roots[0]!, "🧊️base.glb"))).toBe(true);
  });
});

describe("elementState", () => {
  it("resolveElementState defaults every axis to inert", () => {
    expect(resolveElementState()).toEqual({ state: "normal", status: "idle", hover: false, selected: false });
    expect(resolveElementState({ selected: true })).toEqual({ state: "normal", status: "idle", hover: false, selected: true });
  });

  it("elementStateHidden is true only for state === hidden", () => {
    expect(elementStateHidden({ state: "hidden" })).toBe(true);
    for (const state of ["introducing", "celebrating", "previewed", "normal", "disabled"] as const) {
      expect(elementStateHidden({ state })).toBe(false);
    }
  });

  it("elementStateAttributes omits every axis at default", () => {
    expect(elementStateAttributes(resolveElementState())).toEqual({});
  });

  it("elementStateAttributes returns {} for hidden regardless of other axes", () => {
    expect(elementStateAttributes({ state: "hidden", status: "loading", hover: true, selected: true })).toEqual({});
  });

  it("elementStateAttributes stamps data-ui-state plus data-introduced for introducing", () => {
    expect(elementStateAttributes(resolveElementState({ state: "introducing" }))).toEqual({
      "data-ui-state": "introducing",
      "data-introduced": "true",
    });
  });

  it("elementStateAttributes stamps data-ui-state plus data-celebrated for celebrating", () => {
    expect(elementStateAttributes(resolveElementState({ state: "celebrating" }))).toEqual({
      "data-ui-state": "celebrating",
      "data-celebrated": "true",
    });
  });

  it("elementStateAttributes stamps data-ui-state for previewed/disabled without data-introduced", () => {
    expect(elementStateAttributes(resolveElementState({ state: "previewed" }))).toEqual({ "data-ui-state": "previewed" });
    expect(elementStateAttributes(resolveElementState({ state: "disabled" }))).toEqual({ "data-ui-state": "disabled" });
  });

  it("elementStateAttributes stamps data-ui-status for non-idle status", () => {
    expect(elementStateAttributes(resolveElementState({ status: "loading" }))).toEqual({ "data-ui-status": "loading" });
    expect(elementStateAttributes(resolveElementState({ status: "waiting" }))).toEqual({ "data-ui-status": "waiting" });
    expect(elementStateAttributes(resolveElementState({ status: "finished" }))).toEqual({ "data-ui-status": "finished" });
  });

  it("elementStateAttributes stamps data-ui-hover and data-ui-selected independently", () => {
    expect(elementStateAttributes(resolveElementState({ hover: true }))).toEqual({ "data-ui-hover": "true" });
    expect(elementStateAttributes(resolveElementState({ selected: true }))).toEqual({ "data-ui-selected": "true" });
  });

  it("elementStateAttributes composes all four axes simultaneously", () => {
    expect(elementStateAttributes(resolveElementState({ state: "previewed", status: "waiting", hover: true, selected: true }))).toEqual({
      "data-ui-state": "previewed",
      "data-ui-status": "waiting",
      "data-ui-hover": "true",
      "data-ui-selected": "true",
    });
  });

  it("resolveElementFillKind follows disabled > celebrated > selected > previewed > hovered > neutral precedence", () => {
    expect(resolveElementFillKind(resolveElementState())).toBe("neutral");
    expect(resolveElementFillKind(resolveElementState({ hover: true }))).toBe("hovered");
    expect(resolveElementFillKind(resolveElementState({ state: "previewed" }))).toBe("previewed");
    expect(resolveElementFillKind(resolveElementState({ state: "previewed", hover: true }))).toBe("previewed");
    expect(resolveElementFillKind(resolveElementState({ selected: true, hover: true }))).toBe("selected");
    expect(resolveElementFillKind(resolveElementState({ selected: true, state: "previewed" }))).toBe("selected");
    expect(resolveElementFillKind(resolveElementState({ state: "celebrating", selected: true }))).toBe("celebrated");
    expect(resolveElementFillKind(resolveElementState({ state: "disabled", selected: true }))).toBe("disabled");
  });

  it("resolveElementFillKind returns null for hidden", () => {
    expect(resolveElementFillKind({ state: "hidden", status: "idle", hover: true, selected: true })).toBeNull();
  });
});
