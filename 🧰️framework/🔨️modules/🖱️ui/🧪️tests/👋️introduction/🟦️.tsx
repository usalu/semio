import { readFileSync, existsSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import ts from "typescript";
import { describe, expect, it, vi } from "vitest";
import * as React from "react";
import { fireEvent, render } from "@testing-library/react";
import { type IntroductionDefinition } from "@semio-tech/framework";
import { initUiLocaleSync } from "@semio-tech/ui-react/i18n";
import { registerIntroductionSurfaceResolver as publicRegisterIntroductionSurfaceResolver } from "../../🎯️targets/⚛️react/📦️packages/🟦️typescript/👋️introduction.ts";
import { Vector2 } from "three";
import fixture from "../🧫️fixtures/👋️introduction/🔣️.json";
import { UIIntroduction, ndcToViewportPoint, registerIntroductionSurfaceResolver, resolveIntroductionPoint, splitIntroductionBodyParagraphs } from "@semio-tech/ui-react/introduction";

describe("independent introduction contract", () => {
  it("keeps its complete UI source graph independent of scene, diagram and reference renderers", () => {
    const uiRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
    const seen = new Set<string>();
    const visit = (path: string): void => {
      if (seen.has(path)) return;
      seen.add(path);
      expect(path).not.toBe(resolve(uiRoot, "🎯️targets/⚛️react/🟦️.tsx"));
      expect(path).not.toBe(resolve(uiRoot, "🧱️elements/🔌️Ports/🟦️.tsx"));
      const source = ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true);
      const specifiers: string[] = [];
      const dynamicImports = (node: ts.Node): void => {
        if (ts.isCallExpression(node) && node.expression.kind === ts.SyntaxKind.ImportKeyword && node.arguments[0] && ts.isStringLiteral(node.arguments[0])) specifiers.push(node.arguments[0].text);
        ts.forEachChild(node, dynamicImports);
      };
      dynamicImports(source);
      for (const statement of source.statements) {
        if (!ts.isImportDeclaration(statement) && !ts.isExportDeclaration(statement)) continue;
        if (!statement.moduleSpecifier || !ts.isStringLiteral(statement.moduleSpecifier)) continue;
        if (ts.isImportDeclaration(statement) && statement.importClause?.isTypeOnly) continue;
        const bindings = ts.isImportDeclaration(statement) ? statement.importClause?.namedBindings : statement.exportClause;
        if (bindings && (ts.isNamedImports(bindings) || ts.isNamedExports(bindings)) && bindings.elements.every(binding => binding.isTypeOnly)) continue;
        if (ts.isExportDeclaration(statement) && statement.isTypeOnly) continue;
        specifiers.push(statement.moduleSpecifier.text);
      }
      for (const specifier of specifiers) {
        expect(specifier).not.toMatch(/^(?:@semio-tech\/ui-react$|three(?:\/|$)|@react-three\/|@xyflow\/|pdfjs-dist(?:\/|$))/);
        if (!specifier.startsWith(".")) continue;
        const target = resolve(dirname(path), specifier);
        const resolved = [target, `${target}.ts`, `${target}.tsx`].find(candidate => existsSync(candidate));
        expect(resolved, `${path}: ${specifier}`).toBeDefined();
        if (resolved) visit(resolved);
      }
    };
    visit(resolve(uiRoot, "🎯️targets/⚛️react/👋️introduction/🟦️.tsx"));
    expect(seen.size).toBeGreaterThan(1);
  });

  it.each(["en", "de"] as const)("renders and completes the tour using %s chrome independently", (locale) => {
    initUiLocaleSync(locale);
    const onDismiss = vi.fn();
    const { container } = render(<UIIntroduction introduction={fixture.introduction as IntroductionDefinition} stepIndex={0} onStepIndexChange={vi.fn()} onDismiss={onDismiss} />);
    expect(container.querySelectorAll('[data-slot="introduction-body-paragraph"]')).toHaveLength(fixture.paragraphs.expected.length);
    const next = container.querySelector<HTMLButtonElement>('[id="ui.introduction.next"]');
    expect(next).not.toBeNull();
    expect(next?.textContent).toContain(fixture.chrome[locale].done);
    fireEvent.click(next!);
    expect(onDismiss).toHaveBeenCalledWith(true);
  });

  it("projects shared fixtures with the independent Three vector oracle", () => {
    for (const { ndc, expected } of fixture.projection) {
      const { left, top, width, height } = fixture.viewport;
      const oracle = new Vector2(ndc.x, -ndc.y).addScalar(1).multiply(new Vector2(width / 2, height / 2)).add(new Vector2(left, top));
      expect(ndcToViewportPoint(ndc, fixture.viewport)).toEqual(expected);
      expect({ x: oracle.x, y: oracle.y }).toEqual(expected);
    }
  });

  it("resolves scene coordinates through the shared registry without a scene runtime", () => {
    expect(publicRegisterIntroductionSurfaceResolver).toBe(registerIntroductionSurfaceResolver);
    const dispose = publicRegisterIntroductionSurfaceResolver(fixture.surface.windowId, {
      scenePoint: () => ({ ...fixture.surface.expected, visible: true }),
    });
    try {
      expect(resolveIntroductionPoint({ kind: "scene", id: fixture.surface.windowId, position: fixture.surface.worldPoint as [number, number, number] })).toEqual(fixture.surface.expected);
    } finally {
      dispose();
    }
    expect(resolveIntroductionPoint({ kind: "scene", id: fixture.surface.windowId, position: fixture.surface.worldPoint as [number, number, number] })).toBeNull();
  });

  it("retains paragraph layout from the language neutral fixture", () => {
    expect(splitIntroductionBodyParagraphs(fixture.paragraphs.input)).toEqual(fixture.paragraphs.expected);
  });
});
