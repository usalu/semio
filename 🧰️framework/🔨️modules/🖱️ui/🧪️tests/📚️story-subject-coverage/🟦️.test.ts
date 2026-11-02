import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import ts from "typescript";
import { loadCsf } from "storybook/internal/csf-tools";
import fixture from "../🧫️fixtures/📚️story-subjects/🔣️.json";

function inspectStory(source: string) {
  const file = ts.createSourceFile("subject.story.tsx", source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TSX);
  const components: string[] = [];
  const variants: string[] = [];
  const imports: string[] = [];
  const rendered: string[] = [];
  let defaults = 0;
  function visit(node: ts.Node) {
    if (ts.isImportDeclaration(node) && ts.isStringLiteral(node.moduleSpecifier)) {
      if (node.moduleSpecifier.text === "@semio-tech/ui-react" || node.moduleSpecifier.text.startsWith("@semio-tech/ui/elements/")) {
        const bindings = node.importClause?.namedBindings;
        if (bindings && ts.isNamedImports(bindings)) imports.push(...bindings.elements.map((entry) => entry.name.text));
      }
    }
    if (ts.isJsxOpeningElement(node) || ts.isJsxSelfClosingElement(node)) rendered.push(node.tagName.getText(file));
    if (ts.isExportAssignment(node) && !node.isExportEquals) defaults++;
    if (ts.isPropertyAssignment(node) && node.name.getText(file) === "component") components.push(node.initializer.getText(file));
    if (ts.isVariableStatement(node) && node.modifiers?.some((modifier) => modifier.kind === ts.SyntaxKind.ExportKeyword)) {
      variants.push(...node.declarationList.declarations.map((entry) => entry.name.getText(file)));
    }
    ts.forEachChild(node, visit);
  }
  visit(file);
  return { components, variants, imports, rendered, defaults, diagnostics: (file as ts.SourceFile & { parseDiagnostics: readonly ts.Diagnostic[] }).parseDiagnostics };
}

describe("explicit public Storybook subjects", () => {
  test("the language-neutral contract has seventeen unique subjects", () => {
    expect(fixture.version).toBe(1);
    expect(new Set(fixture.subjects.map((subject) => subject.component)).size).toBe(17);
    expect(new Set(fixture.subjects.map((subject) => subject.directory)).size).toBe(17);
  });
  for (const subject of fixture.subjects) {
    test(`${subject.component} has its own live-source CSF meta and required states`, () => {
      const path = new URL(`../../📖️stories/${subject.directory}/🧪️.story.tsx`, import.meta.url);
      const source = readFileSync(path, "utf8");
      const actual = inspectStory(source);
      expect(actual.diagnostics).toHaveLength(0);
      expect(actual.defaults).toBe(1);
      expect(actual.components).toEqual([subject.component]);
      expect(actual.imports).toContain(subject.component);
      expect(actual.rendered).toContain(subject.component);
      for (const companion of subject.companions ?? []) {
        expect(actual.imports).toContain(companion);
        expect(actual.rendered).toContain(companion);
      }
      const csf = loadCsf(source, { makeTitle: (title) => title }).parse();
      expect(csf.meta.component).toBe(subject.component);
      expect(csf.meta.title).toBe(`🖱️ui⚛️react/${subject.component}`);
      expect(csf.stories).toHaveLength(actual.variants.length);
      for (const variant of subject.variants) {
        expect(actual.variants).toContain(variant);
        expect(csf.stories.map((story) => story.id)).toContain(`🖱️ui⚛️react-${subject.component.toLowerCase()}--${variant.replace(/([a-z])([A-Z])/g, "$1-$2").toLowerCase()}`);
      }
      expect(source).not.toMatch(/\b(?:mock|mockImplementation|mockReturnValue)\s*\(/);
    });
  }
  test("the parser rejects incidental JSX without subject metadata", () => {
    const incidental = inspectStory('import { Badge } from "@semio-tech/ui-react"; export const Default = { render: () => <Badge text="Ready" /> };');
    expect(incidental.components).toEqual([]);
    expect(incidental.defaults).toBe(0);
  });
});
