import { describe, expect, test } from "vitest";
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import ts from "typescript";

const ui = resolve(dirname(fileURLToPath(import.meta.url)), "..");
function stories(directory: string): string[] {
  return readdirSync(directory, { withFileTypes: true }).flatMap(entry => entry.isDirectory() ? stories(resolve(directory, entry.name)) : entry.name.endsWith(".story.tsx") ? [resolve(directory, entry.name)] : []);
}

describe("Storybook source imports", () => {
  test("Mode stories expose their executable interaction checks", async () => {
    const mode = await import("../📖️stories/🎭️mode/🧪️.story.tsx");
    for (const story of [mode.QuadLayout, mode.MaximizeStack, mode.ContentThroughGlass]) expect(typeof story.play).toBe("function");
  });
  test("every relative UI story import resolves to an authored file", () => {
    const missing: string[] = [];
    for (const path of stories(resolve(ui, "📖️stories"))) {
      const source = ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TSX);
      for (const statement of source.statements) {
        const specifier = ts.isImportDeclaration(statement) || ts.isExportDeclaration(statement) ? statement.moduleSpecifier : undefined;
        if (!specifier || !ts.isStringLiteral(specifier) || !specifier.text.startsWith(".")) continue;
        const target = resolve(dirname(path), specifier.text);
        if (!["", ".ts", ".tsx", ".js", "/index.ts", "/index.tsx"].some(extension => existsSync(target + extension))) missing.push(`${path}: ${specifier.text}`);
      }
    }
    expect(missing).toEqual([]);
  });
});
