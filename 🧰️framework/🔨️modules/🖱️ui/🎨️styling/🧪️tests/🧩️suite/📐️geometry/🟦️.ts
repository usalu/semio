import { describe, it, expect } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { resolveThemeGeometry, resolveThemeSpacingPx, type UiTheme, semioTheme, patchUiThemeGeometry } from "../../../📦️packages/🟦️typescript/🟦️.ts";

describe("shared theme geometry", () => {
  it("matches neutral vectors and independent CSS calculation", async () => {
    const { calc } = await import("@csstools/css-calc");
    const { tokenize, isTokenDimension, stringify } = await import("@csstools/css-tokenizer");
    const { default: Ajv } = await import("ajv/dist/2020");
    const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧫️fixtures/📐️theme-geometry/🔣️.json"), "utf8"));
    const bindings = JSON.parse(readFileSync(resolve(import.meta.dir, "../../../🌓️theme/📐️geometry/🔣️.json"), "utf8")).bindings;
    const schema = JSON.parse(readFileSync(resolve(import.meta.dir, "../../../🌓️theme/📐️geometry/🧬️contract/🔣️.json"), "utf8"));
    const oracleSpacing = (compact: string, rootRemPx: number): number | null => {
      const tokens = tokenize({ css: compact });
      for (const token of tokens) if (isTokenDimension(token) && token[4].unit.toLowerCase() === "rem") token[1] = `${token[4].value * rootRemPx}px`;
      const value = calc(`calc(${stringify(...tokens)})`, { toCanonicalUnits: true });
      if (!/^[+-]?(?:[0-9]+(?:\.[0-9]+)?|\.[0-9]+)(?:e[+-]?[0-9]+)?px$/i.test(value) && Number(value) !== 0) return null;
      const result = Number.parseFloat(value);
      return Number.isFinite(result) && result >= 0 && result <= schema["x-semio-resolution"].maxMagnitudePx ? result : null;
    };
    const validate = new Ajv({ strict: true, keywords: ["x-semio-ui", {
      keyword: "x-semio-resolution", type: "object", validate: (_: unknown, value: { compact?: string; rootRemPx?: number }) =>
        typeof value.compact === "string" && typeof value.rootRemPx === "number" && oracleSpacing(value.compact, value.rootRemPx) !== null,
    }], formats: { "semio-css-length-expression": { type: "string", validate: (value: string) => value.length <= 512 } } }).compile(schema);
    for (const vector of fixture.cases) {
      const input = { compact: vector.compact, rootRemPx: vector.rootRemPx, metrics: vector.metrics };
      expect(validate(input), JSON.stringify(validate.errors)).toBe(true);
      const geometry = resolveThemeGeometry({ spacing: { compact: vector.compact }, metrics: { ...vector.metrics, dom: { ...vector.metrics.dom, rootRemPx: vector.rootRemPx } } });
      expect(geometry.spacingPx).toBeCloseTo(vector.expectedSpacingPx, 10);
      for (const binding of bindings) {
        const metric = vector.metrics[binding.section][binding.key];
        const cssTokens = tokenize({ css: vector.compact });
        for (const token of cssTokens) if (isTokenDimension(token) && token[4].unit.toLowerCase() === "rem") token[1] = `${token[4].value * vector.rootRemPx}px`;
        const pixels = binding.unit === "spacing" ? calc(`calc(${metric} * ${stringify(...cssTokens)})`, { toCanonicalUnits: true }) : `${metric}px`;
        expect(Number.parseFloat(pixels)).toBeCloseTo(vector.expected[binding.themeField], 10);
        expect(geometry.scalars[binding.themeField]).toBeCloseTo(vector.expected[binding.themeField], 10);
        expect(Number.parseFloat(geometry.cssVars[binding.cssVar]!)).toBeCloseTo(vector.expected[binding.themeField], 10);
      }
    }
    for (const compact of fixture.invalid) {
      expect(() => resolveThemeSpacingPx(compact, 16)).toThrow();
    }
    for (const rootRemPx of [0, -1]) {
      expect(validate({ compact: "1rem", rootRemPx, metrics: {} })).toBe(false);
      expect(() => resolveThemeSpacingPx("1rem", rootRemPx)).toThrow();
    }
  });

  it("refuses invalid metric edits without changing the retained theme", async () => {
    const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧫️fixtures/📐️theme-geometry/🔣️.json"), "utf8"));
    const { default: Ajv } = await import("ajv/dist/2020");
    const geometrySchema = JSON.parse(readFileSync(resolve(import.meta.dir, "../../../🌓️theme/📐️geometry/🧬️contract/🔣️.json"), "utf8"));
    const themeSchema = JSON.parse(readFileSync(resolve(import.meta.dir, "../../../🌓️theme/🧬️contract/🔣️.json"), "utf8"));
    const { calc } = await import("@csstools/css-calc");
    const { tokenize, isTokenDimension, stringify } = await import("@csstools/css-tokenizer");
    const bindings = JSON.parse(readFileSync(resolve(import.meta.dir, "../../../🌓️theme/📐️geometry/🔣️.json"), "utf8")).bindings;
    const canonical = JSON.parse(readFileSync(resolve(import.meta.dir, "../../../🔣️.json"), "utf8"));
    const ajv = new Ajv({ strict: true, keywords: ["x-semio-ui", "x-semio-resolution", {
      keyword: "x-semio-resolved-geometry", type: "object", validate: (_: unknown, theme: UiTheme) => {
        const root = theme.metrics.dom?.rootRemPx ?? canonical.metrics.dom.rootRemPx;
        if (typeof root !== "number" || !Number.isFinite(root) || root <= 0) return false;
        const tokens = tokenize({ css: theme.spacing.compact ?? canonical.spacing.compact });
        for (const token of tokens) if (isTokenDimension(token) && token[4].unit.toLowerCase() === "rem") token[1] = `${token[4].value * root}px`;
        const length = calc(`calc(${stringify(...tokens)})`, { toCanonicalUnits: true });
        if (!/^[+-]?(?:[0-9]+(?:\.[0-9]+)?|\.[0-9]+)(?:e[+-]?[0-9]+)?px$/i.test(length) && Number(length) !== 0) return false;
        const spacing = Number.parseFloat(length);
        const limit = geometrySchema["x-semio-resolution"].maxMagnitudePx;
        if (!Number.isFinite(spacing) || spacing < 0 || spacing > limit) return false;
        return bindings.every((binding: { section: string; key: string; unit: string }) => {
          const value = theme.metrics[binding.section]?.[binding.key] ?? canonical.metrics[binding.section][binding.key];
          const pixels = typeof value === "number" ? value * (binding.unit === "spacing" ? spacing : 1) : NaN;
          return Number.isFinite(pixels) && pixels >= 0 && pixels <= limit;
        });
      },
    }], formats: { "semio-css-length-expression": { type: "string", validate: (compact: string) => compact.length <= 512 } } });
    ajv.addSchema(geometrySchema);
    const validateTheme = ajv.compile(themeSchema);
    const base = semioTheme();
    expect(validateTheme(base), JSON.stringify(validateTheme.errors)).toBe(true);
    const before = JSON.stringify(base);
    for (const vector of fixture.invalidMetrics) {
      const invalid = structuredClone(base);
      invalid.metrics[vector.section]![vector.key] = vector.value;
      expect(validateTheme(invalid)).toBe(false);
      expect(patchUiThemeGeometry(base, next => { next.metrics[vector.section]![vector.key] = vector.value; })).toBeNull();
      expect(JSON.stringify(base)).toBe(before);
    }
    for (const compact of fixture.invalid) {
      expect(patchUiThemeGeometry(base, next => { next.spacing.compact = compact; })).toBeNull();
      expect(JSON.stringify(base)).toBe(before);
    }
    const next = patchUiThemeGeometry(base, theme => { theme.spacing.compact = "calc(1px + 0.2rem)"; });
    expect(next).not.toBeNull();
    expect(resolveThemeGeometry(next!).spacingPx).toBeCloseTo(4.2, 10);
    expect(JSON.stringify(base)).toBe(before);
  });

  it("uses projected chrome metrics in React shell placement", () => {
    const root = resolve(import.meta.dir, "../../../../🧱️elements");
    expect(readFileSync(resolve(root, "🔝️Navbar/🟦️.tsx"), "utf8")).toContain("h-[var(--navbar-height)]");
    expect(readFileSync(resolve(root, "🔚️Footer/🟦️.tsx"), "utf8")).toContain("h-[var(--footer-height)]");
    expect(readFileSync(resolve(root, "🎨️Canvas/🟦️.tsx"), "utf8")).toContain('MODE_CANVAS_INSET_CLASS = "p-[var(--panel-inset)]"');
  });
});
