/** 🎨️ The stylesheet tree: the root `🎨️.css` is the header and the ordered list of `@import`s of the sections under `🎨️styles`, every section folder is imported exactly once, every section is a self-contained run of rules (balanced braces, no `@import`) that names itself with its folder's emoji and stays under 1,000 lines, and `@semio-tech/quiz-react/stylesheet` is exactly those sections joined in the order the root imports them.
 *
 * @see ../../🎯️targets/⚛️react/🎨️.css
 * @see ../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🎨️stylesheet/🟦️.ts
 */

import { describe, expect, it } from "vitest";
import { quizStylesheet } from "@semio-tech/quiz-react/stylesheet";

const TARGET = "../../🎯️targets/⚛️react/";
const ROOT = Object.values(import.meta.glob<string>("../../🎯️targets/⚛️react/🎨️.css", { query: "?raw", import: "default", eager: true })).join("\n");
const SECTIONS = import.meta.glob<string>("../../🎯️targets/⚛️react/🎨️styles/*/🎨️.css", { query: "?raw", import: "default", eager: true });
const IMPORTS = [...ROOT.matchAll(/^@import "\.\/([^"]+)";$/gmu)].map((found) => found[1] ?? "");

const strip = (css: string): string => css.replaceAll(/\/\*[\s\S]*?\*\//gu, "");

describe("🎨️ stylesheet tree", () => {
  it("lists every section folder in the root exactly once, as relative imports ahead of any rule", () => {
    const folders = Object.keys(SECTIONS).map((key) => key.slice(TARGET.length));
    expect(folders.length).toBeGreaterThan(5);
    expect([...IMPORTS].sort()).toEqual([...folders].sort());
    expect(new Set(IMPORTS).size).toBe(IMPORTS.length);
    expect(strip(ROOT).trim().split("\n").every((line) => line.trim() === "" || line.startsWith("@import "))).toBe(true);
  });

  it("joins the sections in the order of the root's imports as the public stylesheet", () => {
    expect(quizStylesheet).toBe(IMPORTS.map((entry) => SECTIONS[TARGET + entry]).join("\n"));
  });

  it("makes each section a self-contained run of rules named by its folder's emoji", () => {
    for (const entry of IMPORTS) {
      const css = SECTIONS[TARGET + entry]!;
      const emoji = entry.split("/")[1]!.split("️")[0]!;
      expect(css.startsWith(`/* ${emoji} `), entry).toBe(true);
      expect(css, entry).not.toContain("@import");
      expect(strip(css).split("{").length, entry).toBe(strip(css).split("}").length);
      expect(css.split("\n").length, entry).toBeLessThanOrEqual(1000);
    }
  });
});
