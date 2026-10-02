import geometryFixture from "../../🧫️fixtures/📐️theme-geometry/🔣️.json";
import geometryBindings from "../../🌓️theme/📐️geometry/🔣️.json";
type TestSource = { readonly directory: string; readonly url: string };
type ColorStringOracle = { readonly get: { rgb(value: string): readonly [number, number, number, number] | null } };

import canonicalThemeDocument from "../../🧫️fixtures/🎨️canonical-theme-document/🔣️.json" with { type: "json" };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: Pick<typeof import("../../🌓️theme/🟦️.ts"), "parseUiTheme" | "resolveThemeAppearancePalettes" | "resolveThemeMetrics" | "resolveThemePaint" | "serializeUiTheme">, source: TestSource): Promise<void> {
  const { parseUiTheme, resolveThemeAppearancePalettes, resolveThemeMetrics, resolveThemePaint, serializeUiTheme } = dependencies;
  type UiTheme = import("../../🌓️theme/🏛️model/🟦️.ts").UiTheme;

  const { describe, expect, it } = vitest;

  const MINIMAL_THEME: UiTheme = {
    id: "test",
    label: "Test",
    colors: { primary: "#ff0000", gray: "#808080" },
    spacing: { compact: "0.25rem" },
    fontStacks: { sans: "sans-serif" },
    canvasFonts: {},
    strokes: { edgeBase: 2 },
    radii: { chrome: 0 },
    opacities: { glassBlur: 1 },
    metrics: { dag: { ioColumnWidth: 10 } },
    appearances: {
      light: { board: { edgeStroke: { token: "gray" } }, map: {}, canvas: {}, chrome: {}, outcome: {}, diagram: {} },
      dark: { board: { edgeStroke: { token: "primary" } }, map: {}, canvas: {}, chrome: {}, outcome: {}, diagram: {} },
    },
  };

  describe("theme resolve", () => {
    it("resolveThemePaint resolves a token ref", () => {
      expect(resolveThemePaint(MINIMAL_THEME.colors, { token: "primary" })).toEqual([255, 0, 0, 255]);
    });

    it("resolveThemePaint resolves a mix ref", () => {
      const [r, g, b, a] = resolveThemePaint(MINIMAL_THEME.colors, { mix: ["primary", "gray", 0.5] });
      expect(a).toBe(255);
      expect(r).toBeGreaterThan(g);
    });

    it("resolveThemePaint resolves a literal hex ref with alpha", () => {
      expect(resolveThemePaint(MINIMAL_THEME.colors, { hex: "#00ff00", alpha: 0.5 })).toEqual([0, 255, 0, 128]);
    });

    it("resolveThemeMetrics derives dag.componentWidth", () => {
      const resolved = resolveThemeMetrics(MINIMAL_THEME.metrics);
      expect(resolved.dag!.componentWidth).toBe(20);
    });

    it("resolveThemeAppearancePalettes resolves board paints per appearance", () => {
      const light = resolveThemeAppearancePalettes(MINIMAL_THEME, "light");
      const dark = resolveThemeAppearancePalettes(MINIMAL_THEME, "dark");
      expect(light.board.edgeStroke).toEqual([128, 128, 128, 255]);
      expect(dark.board.edgeStroke).toEqual([255, 0, 0, 255]);
    });
  });

  describe("theme parse", () => {
    it("executes the canonical theme document with every concrete product removed from the dependency loader", async () => {
      const { dirname, resolve } = await import("node:path");
      const { fileURLToPath } = await import("node:url");
      const { spawnSync } = await import("node:child_process");
      const { createRequire } = await import("node:module");
      const colorString = createRequire(source.url)("color-string") as ColorStringOracle;
      const directory = dirname(fileURLToPath(source.url));
      const fixture = resolve(directory, "../🧫️fixtures/🎨️canonical-theme-document/🔣️.json");
      const products = resolve(directory, "../../../../🛍️products").replaceAll("\\", "/") + "/";
      const program = `import { parseUiTheme, serializeUiTheme, resolveThemePaint } from ${JSON.stringify(resolve(directory, "🏛️model/🟦️.ts"))}; import fixture from ${JSON.stringify(fixture)} with { type: "json" }; const theme=parseUiTheme(fixture); console.log(JSON.stringify({id:theme.id,label:theme.label,paint:resolveThemePaint(theme.colors,theme.appearances.light.chrome.accent),roundTrip:JSON.stringify(parseUiTheme(JSON.parse(serializeUiTheme(theme))))===JSON.stringify(theme)}));`;
      const standalone = String.raw`import { build } from "esbuild"; const bundle=await build({stdin:{contents:${JSON.stringify(program)},resolveDir:${JSON.stringify(directory)}},bundle:true,platform:"node",format:"esm",write:false,plugins:[{name:"neutral-theme-document",setup(builder){builder.onLoad({filter:/.*/},input=>input.path.replaceAll("\\","/").startsWith(${JSON.stringify(products)})?{errors:[{text:"General theme loads a concrete product: "+input.path}]}:undefined)}}]});await import("data:text/javascript;base64,"+Buffer.from(bundle.outputFiles[0].text).toString("base64"));`;
      const native = spawnSync("node", ["--input-type=module"], { cwd: directory, input: standalone, encoding: "utf8" });
      expect(native.status, native.stderr).toBe(0);
      const rgba = colorString.get.rgb(canonicalThemeDocument.colors.primary)!;
      const actual = JSON.parse(native.stdout);
      expect(actual).toEqual({ id: canonicalThemeDocument.id, label: canonicalThemeDocument.label, paint: [rgba[0], rgba[1], rgba[2], Math.round(rgba[3]! * 255)], roundTrip: true });
    });

    it("reads the canonical renderer-neutral theme document without translation", () => {
      const parsed = parseUiTheme(canonicalThemeDocument);
      expect(parsed.id).toBe("custom.fixture");
      expect(parsed.label).toBe("Fixture Theme");
      expect(parsed.appearances.light.chrome.accent).toEqual({ token: "primary" });
      expect(parseUiTheme(JSON.parse(serializeUiTheme(parsed)))).toEqual(parsed);
    });

    it("round-trips a valid theme through serialize/parse", () => {
      const parsed = parseUiTheme(JSON.parse(serializeUiTheme(MINIMAL_THEME)));
      expect(parsed).toEqual(MINIMAL_THEME);
    });

    it("throws on an unknown color token ref", () => {
      const broken = { ...MINIMAL_THEME, appearances: { ...MINIMAL_THEME.appearances, light: { ...MINIMAL_THEME.appearances.light, board: { edgeStroke: { token: "nope" } } } } };
      expect(() => parseUiTheme(broken)).toThrow();
    });

    it("throws when a palette group is missing", () => {
      const broken = JSON.parse(serializeUiTheme(MINIMAL_THEME));
      delete broken.appearances.light.chrome;
      expect(() => parseUiTheme(broken)).toThrow(/chrome/);
    });

    it("throws when appearances.dark is missing", () => {
      const broken = JSON.parse(serializeUiTheme(MINIMAL_THEME));
      delete broken.appearances.dark;
      expect(() => parseUiTheme(broken)).toThrow(/dark/);
    });
  });

}

export async function registerTests2(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: Pick<typeof import("../../🌓️theme/🟦️.ts"), "SPATIAL_AXIS_COLOR_REFS" | "STYLING_BOARD_PALETTES" | "blendTokenHex" | "clearColorResolveCache" | "contrastRatio" | "contrastRatioRgba" | "themePaintContrast" | "wcagContrastGrade" | "readableForegroundHex" | "relativeLuminance" | "resolveColorHex" | "resolveColorRgba" | "resolveSemanticColorHex" | "resolveSpatialAxisColors" | "serializeCanvasThemeJson" | "syncSessionCanvasTheme" | "tokenHex" | "tokenVar">, source: TestSource): Promise<void> {
  const { SPATIAL_AXIS_COLOR_REFS, STYLING_BOARD_PALETTES, blendTokenHex, clearColorResolveCache, contrastRatio, contrastRatioRgba, themePaintContrast, wcagContrastGrade, readableForegroundHex, relativeLuminance, resolveColorHex, resolveColorRgba, resolveSemanticColorHex, resolveSpatialAxisColors, serializeCanvasThemeJson, syncSessionCanvasTheme, tokenHex, tokenVar } = dependencies;

  const { describe, expect, it } = vitest;

  describe("styling resolve", () => {
    it("tokenVar and tokenHex read generated palette", () => {
      expect(tokenVar("primary")).toBe("var(--color-primary)");
      expect(tokenHex("primary")).toBe("#ff344f");
    });

    it("resolveColorHex resolves palette var refs headlessly", () => {
      clearColorResolveCache();
      expect(resolveColorHex("var(--color-secondary)", "gray")).toBe("#34d1bf");
    });

    it("resolveSpatialAxisColors maps X/Y/Z to primary/secondary/tertiary permanently", () => {
      clearColorResolveCache();
      expect(resolveSpatialAxisColors()).toEqual({ x: "#ff344f", y: "#34d1bf", z: "#fa9500" });
      expect(SPATIAL_AXIS_COLOR_REFS).toEqual({ x: "var(--color-primary)", y: "var(--color-secondary)", z: "var(--color-tertiary)" });
    });

    it("resolveColorHex passes through hex literals", () => {
      clearColorResolveCache();
      expect(resolveColorHex("#abc", "gray")).toBe("#aabbcc");
    });

    it("blendTokenHex mixes two palette keys", () => {
      const mixed = blendTokenHex("primary", "light", 0.28);
      expect(mixed).toMatch(/^#[0-9a-f]{6}$/u);
    });

    it("resolveColorRgba returns byte tuple", () => {
      clearColorResolveCache();
      expect(resolveColorRgba("var(--color-gray)", "gray")).toEqual([123, 130, 125, 255]);
    });

    it("relativeLuminance orders light above dark palette tokens", () => {
      expect(relativeLuminance(tokenHex("light"))).toBeGreaterThan(relativeLuminance(tokenHex("dark")));
    });

    it("contrastRatio is symmetric, bounded by 1 and 21, and matches the WCAG extremes", () => {
      expect(contrastRatio("#ffffff", "#000000")).toBeCloseTo(21, 6);
      expect(contrastRatio("#000000", "#ffffff")).toBeCloseTo(21, 6);
      expect(contrastRatio("#7b827d", "#7b827d")).toBeCloseTo(1, 10);
      // 🔬️ WebAIM publishes 4.54:1 for #767676 on white — the canonical AA boundary example.
      expect(contrastRatio("#767676", "#ffffff")).toBeGreaterThanOrEqual(4.5);
      expect(contrastRatio("#777777", "#ffffff")).toBeLessThan(4.6);
    });

    it("wcagContrastGrade bands at 3 / 4.5 / 7, inclusive at each floor", () => {
      expect(wcagContrastGrade(21)).toBe("aaa");
      expect(wcagContrastGrade(7)).toBe("aaa");
      expect(wcagContrastGrade(6.99)).toBe("aa");
      expect(wcagContrastGrade(4.5)).toBe("aa");
      expect(wcagContrastGrade(4.49)).toBe("aaLarge");
      expect(wcagContrastGrade(3)).toBe("aaLarge");
      expect(wcagContrastGrade(2.99)).toBe("fail");
      expect(wcagContrastGrade(Number.NaN)).toBe("fail");
    });

    it("contrastRatioRgba ignores alpha, and themePaintContrast rounds to the 0.01 the editor prints", () => {
      expect(contrastRatioRgba([255, 255, 255, 12], [0, 0, 0, 255])).toBeCloseTo(21, 6);
      const verdict = themePaintContrast([255, 255, 255, 255], [0, 0, 0, 255]);
      expect(verdict.ratio).toBe(21);
      expect(verdict.grade).toBe("aaa");
      expect(verdict.passesBodyText).toBe(true);
      const bad = themePaintContrast([200, 200, 200, 255], [255, 255, 255, 255]);
      expect(bad.passesBodyText).toBe(false);
      expect(bad.grade).toBe("fail");
      expect(Number.isInteger(bad.ratio * 100)).toBe(true);
    });

    it("readableForegroundHex picks light text on dark fills and dark text on light fills", () => {
      clearColorResolveCache();
      expect(readableForegroundHex("var(--color-dark)")).toBe(tokenHex("light"));
      expect(readableForegroundHex("var(--color-light)")).toBe(tokenHex("dark"));
    });

    it("resolveColorHex resolves semantic element vars to gray not foreground", () => {
      clearColorResolveCache();
      expect(resolveColorHex("var(--color-element)", "gray")).toBe("#7b827d");
      expect(resolveSemanticColorHex("border-element-color", "gray")).toBe("#7b827d");
      expect(resolveColorHex("var(--color-element)", "gray")).not.toBe(tokenHex("dark"));
    });

    it("serializeCanvasThemeJson emits token board palette fields", () => {
      const parsed = JSON.parse(serializeCanvasThemeJson("light")) as {
        rasterClear: number[];
        nodeFill: number[];
        nodeStroke: number[];
        nodeStrokeHovered: number[];
        nodeStrokeSelected: number[];
        edgeStroke: number[];
        handleStroke: number[];
        handleStrokeHovered: number[];
        handleFill: number[];
        labelFill: number[];
        labelFillHovered: number[];
        labelHalo: number[];
        gridMinorStroke: number[];
      };
      expect(parsed.rasterClear).toEqual(STYLING_BOARD_PALETTES.light.rasterClear);
      expect(parsed.nodeFill).toHaveLength(4);
      expect(parsed.labelFill).toEqual([123, 130, 125, 255]);
      expect(parsed.edgeStroke).toEqual([123, 130, 125, 255]);
      expect(parsed.handleStroke).toEqual([123, 130, 125, 255]);
      expect(parsed.handleStrokeHovered).toEqual(parsed.handleStroke);
      expect(parsed.nodeStrokeSelected).toEqual(STYLING_BOARD_PALETTES.light.nodeStrokeSelected);
      expect(parsed.handleFill[3]).toBe(0);
      expect(parsed.gridMinorStroke[3]).toBeLessThan(255);
      const dark = JSON.parse(serializeCanvasThemeJson("dark")) as { rasterClear: number[]; labelFill: number[] };
      expect(dark.rasterClear).toEqual(STYLING_BOARD_PALETTES.dark.rasterClear);
      expect(dark.rasterClear).not.toEqual(parsed.rasterClear);
      expect(dark.labelFill).toEqual(STYLING_BOARD_PALETTES.dark.labelFill);
      expect(dark.labelFill).not.toEqual(parsed.labelFill);
    });

    it("syncSessionCanvasTheme pushes serialized palette into a session", () => {
      const calls: string[] = [];
      syncSessionCanvasTheme({
        setCanvasThemeJson(json: string) {
          calls.push(json);
        },
      });
      expect(calls.length).toBe(1);
      const parsed = JSON.parse(calls[0]!) as { rasterClear: number[] };
      expect(parsed.rasterClear).toEqual(STYLING_BOARD_PALETTES.light.rasterClear);
    });
  });

}

export async function registerTests3(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: Pick<typeof import("../../🌓️theme/🟦️.ts"), "STYLING_BOARD_PALETTES" | "_activeUiTheme" | "_appliedThemeCssPropsByRoot" | "activeUiTheme" | "applyUiThemeToDocument" | "applyUiThemeToRoot" | "builtinUiThemes" | "clearUiThemeFromRoot" | "semioTheme" | "serializeCanvasThemeJson" | "setActiveUiTheme" | "subscribeActiveUiTheme">, source: TestSource): Promise<void> {
  const { STYLING_BOARD_PALETTES, _activeUiTheme, _appliedThemeCssPropsByRoot, activeUiTheme, applyUiThemeToDocument, applyUiThemeToRoot, builtinUiThemes, clearUiThemeFromRoot, semioTheme, serializeCanvasThemeJson, setActiveUiTheme, subscribeActiveUiTheme } = dependencies;

  const { afterEach, describe, expect, it } = vitest;

  afterEach(() => {
    _activeUiTheme.current = undefined;
    if (typeof document !== "undefined") {
      clearUiThemeFromRoot(document.documentElement);
    }
    for (const root of [..._appliedThemeCssPropsByRoot.keys()]) {
      clearUiThemeFromRoot(root);
    }
  });

  describe("theme registry", () => {
    it("builtinUiThemes always includes semio first", () => {
      const themes = builtinUiThemes();
      expect(themes[0]!.id).toBe("semio");
    });

    it("builtinUiThemes discovers the mono premade via import.meta.glob", () => {
      const themes = builtinUiThemes();
      expect(themes.map((t) => t.id)).toContain("mono");
    });

    it("activeUiTheme defaults to semio", () => {
      expect(activeUiTheme().id).toBe("semio");
    });

    it("serializeCanvasThemeJson matches the baked palette before any theme is set", () => {
      const parsed = JSON.parse(serializeCanvasThemeJson("light")) as { rasterClear: number[] };
      expect(parsed.rasterClear).toEqual(STYLING_BOARD_PALETTES.light.rasterClear);
    });

    it("setActiveUiTheme changes serializeCanvasThemeJson output and notifies subscribers", () => {
      const mono = builtinUiThemes().find((t) => t.id === "mono");
      if (!mono) throw new Error("mono premade not discovered by builtinUiThemes()");
      const seen: string[] = [];
      const unsubscribe = subscribeActiveUiTheme((t) => seen.push(t.id));
      setActiveUiTheme(mono);
      expect(seen).toEqual(["mono"]);
      const parsed = JSON.parse(serializeCanvasThemeJson("light")) as { rasterClear: number[] };
      expect(parsed.rasterClear).not.toEqual(STYLING_BOARD_PALETTES.light.rasterClear);
      unsubscribe();
    });

    it("applyUiThemeToDocument writes the level knob CSS vars, never the deleted per-tier ones", () => {
      const theme = semioTheme();
      applyUiThemeToDocument(theme);
      const root = document.documentElement;
      const chrome = theme.metrics.chrome;
      if (typeof chrome?.shadeStepPercent === "number") {
        expect(root.style.getPropertyValue("--level-shade-step")).toBe(`${chrome.shadeStepPercent}%`);
      }
      if (typeof chrome?.glassAlphaStep === "number") {
        expect(root.style.getPropertyValue("--glass-alpha-step")).toBe(`${chrome.glassAlphaStep}`);
      }
      for (const deleted of ["--glass-panel-blur", "--glass-panel-alpha", "--glass-menu-alpha", "--glass-window-options-blur", "--glass-window-options-alpha"]) {
        expect(root.style.getPropertyValue(deleted)).toBe("");
      }
    });

    it("retains mounted geometry and active identity for invalid geometry publication", () => {
      const root = document.createElement("div");
      const retained = semioTheme();
      setActiveUiTheme(retained);
      applyUiThemeToRoot(root, retained);
      const before = root.getAttribute("style");
      let notifications = 0;
      const unsubscribe = subscribeActiveUiTheme(() => { notifications++; });
      for (const compact of geometryFixture.invalid) {
        const invalid = structuredClone(semioTheme());
        invalid.spacing.compact = compact;
        expect(() => applyUiThemeToRoot(root, invalid)).toThrow();
        expect(root.getAttribute("style")).toBe(before);
        expect(() => setActiveUiTheme(invalid)).toThrow();
        expect(activeUiTheme()).toBe(retained);
      }
      expect(notifications).toBe(0);
      unsubscribe();
      clearUiThemeFromRoot(root);
    });

    it("applies neutral geometry vectors to isolated mounted roots", () => {
      const baseline = document.createElement("div");
      const custom = document.createElement("div");
      document.body.append(baseline, custom);
      applyUiThemeToRoot(baseline, semioTheme());
      const retainedNavbar = baseline.style.getPropertyValue("--navbar-height");
      for (const vector of geometryFixture.cases) {
        const theme = structuredClone(semioTheme());
        theme.spacing.compact = vector.compact;
        for (const [section, metrics] of Object.entries(vector.metrics)) theme.metrics[section] = { ...theme.metrics[section], ...metrics };
        theme.metrics.dom!.rootRemPx = vector.rootRemPx;
        applyUiThemeToRoot(custom, theme);
        for (const binding of geometryBindings.bindings) {
          const actual = Number.parseFloat(getComputedStyle(custom).getPropertyValue(binding.cssVar));
          expect(actual).toBeCloseTo(vector.expected[binding.themeField as keyof typeof vector.expected], 8);
        }
        expect(baseline.style.getPropertyValue("--navbar-height")).toBe(retainedNavbar);
      }
      clearUiThemeFromRoot(custom);
      for (const binding of geometryBindings.bindings) expect(custom.style.getPropertyValue(binding.cssVar)).toBe("");
      baseline.remove();
      custom.remove();
    });

    it("applyUiThemeToRoot scopes tokens per root — two co-mounted shells never clobber each other", () => {
      const mono = builtinUiThemes().find((t) => t.id === "mono");
      if (!mono) throw new Error("mono premade not discovered by builtinUiThemes()");
      const shellA = document.createElement("div");
      const shellB = document.createElement("div");
      applyUiThemeToRoot(shellA, semioTheme());
      applyUiThemeToRoot(shellB, mono);
      expect(shellA.dataset.uiTheme).toBe("semio");
      expect(shellB.dataset.uiTheme).toBe("mono");
      expect(shellA.style.getPropertyValue("--color-primary")).not.toBe("");
      expect(shellA.style.getPropertyValue("--color-primary")).not.toBe(shellB.style.getPropertyValue("--color-primary"));
      expect(document.documentElement.dataset.uiTheme).toBeUndefined();
      clearUiThemeFromRoot(shellA);
      expect(shellA.dataset.uiTheme).toBeUndefined();
      expect(shellA.style.getPropertyValue("--color-primary")).toBe("");
      expect(shellB.dataset.uiTheme).toBe("mono");
      expect(shellB.style.getPropertyValue("--color-primary")).not.toBe("");
    });
  });

}
