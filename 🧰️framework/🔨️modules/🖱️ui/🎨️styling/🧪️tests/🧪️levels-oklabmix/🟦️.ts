type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { LEVELS_DEFAULT, join, linearToOklab, loadTokens, oklabMix, resolveAppearances, rgba8ToLinear, toPascalCase } = dependencies;
  type Rgba8 = any;

  const { describe, expect, it } = vitest;

  function relativeLuminance(rgba: Rgba8): number {
    const [r, g, b] = rgba8ToLinear(rgba);
    return 0.2126 * r + 0.7152 * g + 0.0722 * b;
  }

  function contrastRatio(a: Rgba8, b: Rgba8): number {
    const la = relativeLuminance(a);
    const lb = relativeLuminance(b);
    const lighter = Math.max(la, lb);
    const darker = Math.min(la, lb);
    return (lighter + 0.05) / (darker + 0.05);
  }

  function oklabL(rgba: Rgba8): number {
    const [r, g, b] = rgba8ToLinear(rgba);
    return linearToOklab(r, g, b)[0];
  }

  function toHex(rgba: Rgba8): string {
    return `#${rgba
      .slice(0, 3)
      .map((c) => c.toString(16).padStart(2, "0"))
      .join("")}`;
  }

  describe("levels: oklabMix", () => {
    it("t=0 returns a and t=1 returns b unchanged", () => {
      const a: Rgba8 = [10, 20, 30, 255];
      const b: Rgba8 = [200, 100, 50, 255];
      expect(oklabMix(a, b, 0)).toEqual(a);
      expect(oklabMix(a, b, 1)).toEqual(b);
    });
  });

  describe("levels: derived appearance paints", () => {
    const tokens = loadTokens();
    const levels = tokens.levels ?? LEVELS_DEFAULT;
    const resolved = resolveAppearances(tokens);
    const levelKeys = levels.names.map((n) => `level${toPascalCase(n)}`);
    const elementKeys = levels.names.map((n) => `element${toPascalCase(n)}`);

    it("injects exactly 6 level + 6 element paints into every appearance's chrome group", () => {
      for (const appearance of ["light", "dark"] as const) {
        const chrome = resolved[appearance]!.chrome!;
        for (const key of [...levelKeys, ...elementKeys]) {
          expect(chrome[key]).toBeDefined();
        }
      }
    });

    it("monotonic lightness per appearance across the 6 levels (light darkens, dark lightens)", () => {
      const lightL = levelKeys.map((k) => oklabL(resolved.light!.chrome![k]!));
      const darkL = levelKeys.map((k) => oklabL(resolved.dark!.chrome![k]!));
      for (let i = 1; i < lightL.length; i++) {
        expect(lightL[i]!).toBeLessThanOrEqual(lightL[i - 1]!);
      }
      for (let i = 1; i < darkL.length; i++) {
        expect(darkL[i]!).toBeGreaterThanOrEqual(darkL[i - 1]!);
      }
    });

    it("contrast against foreground stays >= 4.5:1 at every level, both appearances", () => {
      for (const appearance of ["light", "dark"] as const) {
        const chrome = resolved[appearance]!.chrome!;
        for (const key of levelKeys) {
          expect(contrastRatio(chrome[key]!, chrome.foreground!)).toBeGreaterThanOrEqual(4.5);
        }
      }
    });

    it("monotone alpha ladder: alpha(k) = 1 - k*glassAlphaStep, strictly decreasing", () => {
      const alphas = levels.names.map((_, k) => 1 - k * levels.glassAlphaStep);
      for (let i = 1; i < alphas.length; i++) {
        expect(alphas[i]!).toBeLessThan(alphas[i - 1]!);
      }
      expect(alphas[0]).toBe(1);
      expect(alphas.at(-1)!).toBeCloseTo(1 - 5 * levels.glassAlphaStep, 10);
    });

    it("monotone blur ladder: blur(k) = k*glassBlurStepPx, strictly increasing", () => {
      const blurs = levels.names.map((_, k) => k * levels.glassBlurStepPx);
      for (let i = 1; i < blurs.length; i++) {
        expect(blurs[i]!).toBeGreaterThan(blurs[i - 1]!);
      }
      expect(blurs[0]).toBe(0);
    });

    it("pinned hex snapshot: light appearance level backgrounds", () => {
      const chrome = resolved.light!.chrome!;
      expect(levelKeys.map((k) => toHex(chrome[k]!))).toEqual(["#f7f3e3", "#e9e6d7", "#dad9cc", "#cccdc1", "#bec0b5", "#b0b4aa"]);
    });

    it("pinned hex snapshot: dark appearance level backgrounds", () => {
      const chrome = resolved.dark!.chrome!;
      expect(levelKeys.map((k) => toHex(chrome[k]!))).toEqual(["#001117", "#061a1f", "#112328", "#1c2d31", "#27373a", "#324143"]);
    });
  });

}
