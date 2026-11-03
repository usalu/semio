/** 🌪️ No infinite animation may re-resolve inherited style of the whole document: animated custom properties are registered `inherits: false` and no `:root`/`html`/`body` rule declares an animation (10 s per hop recalc storm, 2026-09-12). */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import postcss from "postcss";
import { describe, expect, it } from "vitest";

const css = readFileSync(resolve(dirname(fileURLToPath(import.meta.url)), "../../../../../../../🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css"), "utf8");
const root = postcss.parse(css);
const registered = new Map<string, string>();
root.walkAtRules("property", rule => {
  let inherits = "";
  rule.walkDecls("inherits", decl => { inherits = decl.value.trim(); });
  registered.set(rule.params.trim(), inherits);
});
const animated = new Set<string>();
root.walkAtRules("keyframes", rule => rule.walkDecls(decl => { if (decl.prop.startsWith("--")) animated.add(decl.prop); }));

describe("root animation storm", () => {
  it("finds the registered animated properties (guards against a vacuous scan)", () => {
    expect(registered.size).toBeGreaterThan(0);
    expect(animated.size).toBeGreaterThan(0);
  });

  it("registers every keyframe-animated custom property with inherits: false", () => {
    for (const prop of animated) expect(registered.get(prop), prop).toBe("false");
  });

  it("declares no animation on :root, html or body", () => {
    const offenders: string[] = [];
    root.walkRules(rule => {
      const global = rule.selectors.every(selector => /^(:root|html|body)(\[[^\]]*\])?$/u.test(selector.trim()));
      if (global) rule.walkDecls(/^animation(-|$)/u, decl => { offenders.push(`${rule.selector} { ${decl.prop} }`); });
    });
    expect(offenders).toEqual([]);
  });
});
