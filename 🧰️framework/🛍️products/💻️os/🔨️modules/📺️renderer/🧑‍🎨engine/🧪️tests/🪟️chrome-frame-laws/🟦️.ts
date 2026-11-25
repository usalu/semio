/** 🪟️ The React OS renderer obeys the frame and glassy-chip laws: chrome carries no radius, shadow or hand-drawn border classes, free-standing chips over pictures are glass chips, and no footer item is named a gap. Source scan, one case table per rule. */
import { readdirSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";
import { describe, expect, it } from "vitest";

const elements = join(import.meta.dirname, "../../🧱️elements");
const read = (path: string): string => readFileSync(join(elements, path), "utf8");

function sources(dir: string): string[] {
  return readdirSync(dir).flatMap((name) => {
    const path = join(dir, name);
    if (name === "node_modules" || name.includes("tests") || name.includes("stories")) return [];
    if (statSync(path).isDirectory()) return sources(path);
    return /\.tsx$/u.test(path) && !/\.(test|story)\./u.test(path) ? [path] : [];
  });
}

const radius = /(?<![\w-])rounded(-(xs|sm|md|lg|xl))?(?![\w-])/u;
const shadow = /(?<![\w-])shadow-(sm|md|lg|xl|2xl)(?![\w-])/u;
const code = (path: string): string => readFileSync(path, "utf8").replace(/\/\*[\s\S]*?\*\//gu, "");

describe("os react chrome: frame and glassy-chip laws", () => {
  const all = sources(elements);

  it("no element source paints a shadow utility", () => {
    expect(all.filter((path) => shadow.test(code(path))).map((path) => path.slice(elements.length))).toEqual([]);
  });

  it("no element source paints a radius utility (round marks use rounded-full)", () => {
    expect(all.filter((path) => radius.test(code(path))).map((path) => path.slice(elements.length))).toEqual([]);
  });

  it.each([
    ["🌐️World3dHost/🟦️.tsx", 3],
    ["🌐️World3dHost/🎯️marquee-and-bridges/🟦️.tsx", 2],
    ["🌐️World3dHost/📏️modelling/🟦️.tsx", 1],
    ["🕸️NodeGraph/🟦️.tsx", 2],
    ["🏛️ShellHost/🧱️notice-bands/🟦️.tsx", 2],
    ["🖌️Paint2dHost/🟦️.tsx", 2],
  ])("%s marks its free-standing chips data-glass-chip (at least %i)", (path, count) => {
    expect(read(path).split("data-glass-chip").length - 1).toBeGreaterThanOrEqual(count);
  });

  it("the diagram node is a shared-border frame without a hand-drawn header line", () => {
    const node = read("🕸️NodeGraph/🔷️diagram-node/🟦️.tsx");
    expect(node).toContain("SharedBorderRow");
    expect(node).not.toMatch(/\bborder-b\b|\bborder border-border\b/u);
  });

  it("the diff divider and the editor completion popup draw no border classes", () => {
    expect(read("🔺️DiffViewHost/🟦️.tsx")).not.toMatch(/\bborder-l\b/u);
    expect(read("✏️TextEditor/🟦️.tsx")).not.toMatch(/\bborder border-border\b/u);
  });

  it("the footer names its empty items spacers, never gaps", () => {
    const items = read("🏛️ShellHost/🖼️canvas-footer/🟠️footer-items/🟦️.tsx");
    expect(items).not.toMatch(/footer(Leading|Trailing)Gap/u);
    expect(items).toContain("footerLeadingSpacer");
    expect(items).toContain("footerTrailingSpacer");
  });

  it("the presence label is a square chip", () => {
    expect(read("👕️canvas-presence/🟦️.tsx")).not.toMatch(/borderRadius: 3\b/u);
  });

  it("the shell notices and panes are glass, not opaque", () => {
    const bands = read("🏛️ShellHost/🧱️notice-bands/🟦️.tsx");
    expect(bands).not.toContain("bg-background");
    expect(bands).toContain("glassClass");
  });

  it.each([
    "🧭️TiledMapHost/🟦️.tsx",
    "🕸️NodeGraph/🪄️catalogue-spotlight/🟦️.tsx",
    "🏛️ShellHost/🚪️close-guard/🟦️.tsx",
  ])("%s: a floating surface is a glass chip, never frameless", (path) => {
    expect(read(path)).toContain("data-glass-chip");
  });

  it("the projection ribbon of the infinite world is a glass chip", () => {
    const ribbon = readFileSync(join(import.meta.dirname, "../../../../../../../🔨️modules/♾️infinite/🌍️world/🎨️r3f/📐️projection/🟦️.tsx"), "utf8");
    expect(ribbon.split("data-glass-chip").length - 1).toBeGreaterThanOrEqual(2);
  });

  it("the staging dialog and the close guard are glass over the workbench", () => {
    expect(read("🏛️ShellHost/📝️staging/🟦️.tsx")).not.toContain("bg-menu");
    expect(read("🏛️ShellHost/🚪️close-guard/🟦️.tsx")).not.toContain("bg-background");
  });
});
