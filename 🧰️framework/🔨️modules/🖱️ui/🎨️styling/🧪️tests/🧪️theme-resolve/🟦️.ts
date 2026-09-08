type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { parseUiTheme, resolveThemeAppearancePalettes, resolveThemeMetrics, resolveThemePaint, serializeUiTheme } = dependencies;
  type UiTheme = any;

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
      light: { board: { edgeStroke: { token: "gray" } }, map: {}, canvas: {}, chrome: {} },
      dark: { board: { edgeStroke: { token: "primary" } }, map: {}, canvas: {}, chrome: {} },
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

export async function registerTests2(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { SPATIAL_AXIS_COLOR_REFS, STYLING_BOARD_PALETTES, blendTokenHex, clearColorResolveCache, readableForegroundHex, relativeLuminance, resolveColorHex, resolveColorRgba, resolveSemanticColorHex, resolveSpatialAxisColors, serializeCanvasThemeJson, syncSessionCanvasTheme, tokenHex, tokenVar } = dependencies;

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

export async function registerTests3(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
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
