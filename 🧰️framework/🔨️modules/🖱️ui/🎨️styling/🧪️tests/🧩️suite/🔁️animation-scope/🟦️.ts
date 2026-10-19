import { describe, it, expect } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { type CssClockRule, type CssPaintRule, type CssAnimationScopeLaw, CSS_ANIMATION_SCOPE_LAWS, analyzeCssAnimationScope, cssAnimationScopeViolations, cssAnimationScopeUnclockedPaints } from "../../../📦️packages/🟦️typescript/🟦️.ts";
import { uiCss } from "../🧰️suite-build-server/🟦️.ts";

describe("🔁️ border effect animation scope", () => {
  const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧫️fixtures/🔁️animation-scope/🔣️.json"), "utf8")) as {
    laws: readonly string[];
    cases: readonly {
      name: string;
      css: string;
      expect: {
        animatedCustomProperties: readonly string[];
        keyframesByProperty: Readonly<Record<string, readonly string[]>>;
        propertyInheritance: Readonly<Record<string, boolean>>;
        rootClockSelectors: string[];
        clocks: readonly CssClockRule[];
        paints: readonly CssPaintRule[];
        violations: readonly CssAnimationScopeLaw[];
      };
    }[];
    stylesheet: { path: string; keyframesByProperty: Record<string, readonly string[]>; violations: readonly CssAnimationScopeLaw[] };
  };
  const squash = (selector: string) => selector.replace(/\s+/g, " ").trim();

  /** 🧰️ Independent oracle over the SAME questions, built on postcss's own node tree rather than this
   * repo's brace scanner — the third-party half of the law. */
  const postcssScope = async (css: string) => {
    const { parse } = await import("postcss");
    const scoping = new Set(["media", "supports", "layer", "container", "scope"]);
    const path = (node: unknown): string => {
      const parts: string[] = [];
      for (let current = node as { type: string; parent?: unknown; selector?: string; name?: string; params?: string } | undefined; current && current.type !== "root"; current = current.parent as never) {
        if (current.type === "rule") parts.unshift(squash(current.selector ?? ""));
        else if (current.type === "atrule" && !scoping.has(current.name ?? "")) parts.unshift(squash(`@${current.name} ${current.params ?? ""}`));
      }
      return parts.join(" ");
    };
    const inside = (node: unknown, name: string): boolean => {
      for (let current = node as { type: string; name?: string; parent?: unknown } | undefined; current; current = current.parent as never) if (current.type === "atrule" && current.name === name) return true;
      return false;
    };
    const root = parse(css);
    const keyframesByProperty = new Map<string, Set<string>>();
    const inheritance: Record<string, boolean> = {};
    const clocks = new Map<string, Set<string>>();
    const forwarded = new Map<string, Set<string>>();
    const painted = new Map<string, Set<string>>();
    const rootSelectors = new Set([":root", "html", ":root:root", "html:root"]);
    const rootClocks = new Set<string>();
    root.walkAtRules("property", (rule) => { inheritance[squash(rule.params)] = /inherits:\s*true/.test(rule.toString()); });
    root.walkDecls((declaration) => {
      const reads = [...declaration.value.matchAll(/var\(\s*(--[\w-]+)/g)].map((match) => match[1]!);
      if (inside(declaration, "keyframes")) {
        if (!declaration.prop.startsWith("--")) return;
        for (let current = declaration.parent as { type: string; name?: string; params?: string; parent?: unknown } | undefined; current; current = current.parent as never) {
          if (current.type === "atrule" && current.name === "keyframes") { (keyframesByProperty.get(declaration.prop) ?? keyframesByProperty.set(declaration.prop, new Set()).get(declaration.prop)!).add(squash(current.params ?? "")); break; }
        }
        return;
      }
      if (inside(declaration, "property")) return;
      const selector = path(declaration.parent);
      if (declaration.prop === "animation" || declaration.prop === "animation-name") {
        const names = declaration.value.split(",").flatMap((layer) => layer.replace(/!important/g, " ").split(/\s+/).filter((token) => /^[A-Za-z_-][\w-]*$/.test(token) && !["none", "infinite", "linear", "ease", "ease-in", "ease-out", "ease-in-out", "alternate", "alternate-reverse", "reverse", "normal", "forwards", "backwards", "both", "running", "paused", "step-start", "step-end", "important"].includes(token)));
        if (names.length > 0) {
          for (const name of names) (clocks.get(selector) ?? clocks.set(selector, new Set()).get(selector)!).add(name);
          if (selector.split(" ").every((part) => rootSelectors.has(part))) rootClocks.add(selector);
        }
      }
      if (reads.length === 0) return;
      const sink = declaration.prop.startsWith("--") ? forwarded.get(declaration.prop) ?? forwarded.set(declaration.prop, new Set()).get(declaration.prop)! : painted.get(selector) ?? painted.set(selector, new Set()).get(selector)!;
      for (const read of reads) sink.add(read);
    });
    const animated = new Set(keyframesByProperty.keys());
    const resolveRead = (name: string, seen: Set<string>): readonly string[] => {
      if (seen.has(name)) return [];
      seen.add(name);
      if (animated.has(name)) return [name];
      return [...(forwarded.get(name) ?? [])].flatMap((next) => resolveRead(next, seen));
    };
    const unclocked: string[] = [];
    for (const [selector, reads] of painted) {
      for (const property of [...new Set([...reads].flatMap((read) => resolveRead(read, new Set())))].sort()) {
        if (![...(keyframesByProperty.get(property) ?? [])].some((name) => clocks.get(selector)?.has(name) === true)) unclocked.push(`${selector} :: ${property}`);
      }
    }
    return {
      animated: [...animated].sort(),
      inheritedAnimated: [...animated].filter((name) => inheritance[name] !== false).sort(),
      rootClocks: [...rootClocks].sort(),
      unclocked: unclocked.sort(),
    };
  };

  it("reads every fixture case exactly as the fixture states it", () => {
    expect(fixture.laws).toEqual([...CSS_ANIMATION_SCOPE_LAWS]);
    for (const testCase of fixture.cases) {
      const scope = analyzeCssAnimationScope(testCase.css);
      expect(scope.animatedCustomProperties).toEqual(testCase.expect.animatedCustomProperties);
      expect(scope.keyframesByProperty).toEqual(testCase.expect.keyframesByProperty);
      expect(scope.propertyInheritance).toEqual(testCase.expect.propertyInheritance);
      expect(scope.rootClocks.map((clock) => clock.selector)).toEqual(testCase.expect.rootClockSelectors);
      expect(scope.clocks).toEqual(testCase.expect.clocks);
      expect(scope.paints).toEqual(testCase.expect.paints);
      expect(cssAnimationScopeViolations(scope)).toEqual(testCase.expect.violations);
    }
  });

  it("agrees with postcss on every fixture case and on 🖌️ui.css itself", async () => {
    for (const testCase of [...fixture.cases.map((entry) => ({ name: entry.name, css: entry.css })), { name: "🖌️ui.css", css: uiCss }]) {
      const ours = analyzeCssAnimationScope(testCase.css);
      const theirs = await postcssScope(testCase.css);
      expect(ours.animatedCustomProperties).toEqual(theirs.animated);
      expect(ours.animatedCustomProperties.filter((name) => ours.propertyInheritance[name] !== false)).toEqual(theirs.inheritedAnimated);
      expect(ours.rootClocks.map((clock) => squash(clock.selector)).sort()).toEqual(theirs.rootClocks);
      expect(cssAnimationScopeUnclockedPaints(ours).map((paint) => `${squash(paint.selector)} :: ${paint.property}`).sort()).toEqual(theirs.unclocked);
    }
  });

  it("keeps every border-effect clock on the element that paints with it, never on the document root", () => {
    const scope = analyzeCssAnimationScope(uiCss);
    expect(scope.keyframesByProperty).toEqual(fixture.stylesheet.keyframesByProperty);
    expect(scope.rootClocks).toEqual([]);
    expect(scope.animatedCustomProperties.filter((name) => scope.propertyInheritance[name] !== false)).toEqual([]);
    expect(cssAnimationScopeUnclockedPaints(scope).map((paint) => `${squash(paint.selector)} :: ${paint.property}`)).toEqual([]);
    expect(cssAnimationScopeViolations(scope)).toEqual(fixture.stylesheet.violations);
    expect(scope.paints.length).toBeGreaterThan(8);
  });
});
