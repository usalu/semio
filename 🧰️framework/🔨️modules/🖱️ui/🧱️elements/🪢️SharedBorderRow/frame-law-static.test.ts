import { readdirSync, readFileSync } from "node:fs";
import { relative, resolve } from "node:path";
import { describe, expect, it } from "vitest";

const ui = resolve(import.meta.dirname, "../..");
const skipDirectory = /^(node_modules|🧪️tests|📖️stories|🗑️generated|🧪️.*)$/u;
const sourceFile = /\.(?:ts|tsx)$/u;
const testFile = /\.test\.|\.cases\.|\.d\.ts$/u;
const contentShapes = ["🧱️elements/📻️TableAvatar/", "🧱️elements/🦴️Skeletons/", "🎯️targets/⚛️react/🔌️host-ports/", "🎯️targets/⚛️react/🎬️introduction-demo/", "🔨️modules/🏷️class-name-composition/"];
const decorativeUtility = /^(?:[\w\[\]:()&>*-]+:)*-?(?:rounded|shadow)(?:-(?!none$)[\w.\[\]%]+)*$/u;

function* walk(directory: string): Generator<string> {
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    if (entry.isDirectory()) {
      if (!skipDirectory.test(entry.name)) yield* walk(resolve(directory, entry.name));
    } else if (sourceFile.test(entry.name) && !testFile.test(entry.name)) yield resolve(directory, entry.name);
  }
}

function decorativeTokens(source: string): string[] {
  const found: string[] = [];
  for (const line of source.split("\n")) {
    if (/^\s*(?:\/\/|\/?\*)/u.test(line)) continue;
    for (const literal of line.matchAll(/(["'`])((?:\\.|(?!\1)[^\\\n])*)\1/gu)) {
      for (const token of literal[2]!.split(/\s+/u)) if (decorativeUtility.test(token)) found.push(token);
    }
  }
  return found;
}

describe("frame and chip law: chrome carries no radius and no shadow utilities", () => {
  const offenders = [...walk(ui)]
    .map((file) => ({ file: relative(ui, file), tokens: decorativeTokens(readFileSync(file, "utf8")) }))
    .filter(({ file, tokens }) => tokens.length > 0 && !contentShapes.some((shape) => file.startsWith(shape)));
  it("has no rounded-* or shadow-* class outside content shapes", () => {
    expect(offenders).toEqual([]);
  });
});

describe("frame and chip law: no hand-drawn border classes on shared chrome class exports", () => {
  const chromeClasses = readFileSync(resolve(ui, "🎯️targets/⚛️react/🎨️chrome-classes/🟦️.ts"), "utf8").replace(/\/\*\*[\s\S]*?\*\//gu, "");
  it("exports no border-drawing frame, edge or floating-surface class", () => {
    for (const name of ["borderNormalFrameClass", "borderEmphasizedFrameClass", "borderEmphasizedBottomClass", "borderEmphasizedTopClass", "shellChromeBorderClass"]) expect(chromeClasses).not.toContain(name);
    expect(chromeClasses).not.toMatch(/\bborder\b(?!-)/u);
  });
});
