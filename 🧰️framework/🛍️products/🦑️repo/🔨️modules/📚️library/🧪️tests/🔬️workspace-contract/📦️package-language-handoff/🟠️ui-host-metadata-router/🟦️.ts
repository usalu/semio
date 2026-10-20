/** 🟠️ The ui host metadata router tests of the package language semantic handoff suite. */
import { buildBudgetMs } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { resolveTestLevel } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { partitionNextestExecutionFilters } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { contractRoot } from "../../🏗️shared-fixtures/🟦️.ts";
import { inputRoot, routerInvocation } from "../🟢️shared/🟦️.ts";

/** 🟡 Registers the ui host metadata router tests of the package language semantic handoff suite. */
export function registerUiHostMetadataRouterTests(): void {
  test("UI-host metadata router calls its real source oracle and preserves bounded native delegation", async () => {
    const ts = await import("typescript"), { spyOn } = await import("bun:test"), { pathToFileURL } = await import("node:url");
    const { semanticOwnedInputFileSnapshot } = await import("../../../../🔍️discovery/✅️owned-file-approvals/🟦️.ts");
    const root = resolve(contractRoot, "../../../../../../.."), expected = JSON.parse(readFileSync(join(inputRoot, "🖥️ui-host-package/🔣️.json"), "utf8")), packageRoot = join(root, expected.packagePath);
    const sourcePath = expected.packagePath + "/📜️script.ts", snapshot = semanticOwnedInputFileSnapshot(root, sourcePath);
    expect(snapshot?.nodeKind, sourcePath).toBe("file");
    const source = Buffer.from(snapshot!.bytes).toString("utf8"), syntax = ts.createSourceFile(sourcePath, source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    const oracles = expected.sourceOracles as { scope: string; scriptPath: string; importSpecifier: string; functionName: string; fixturePath: string; schemaPath: string; arguments: unknown[] }[];
    const inputs = [sourcePath, expected.packagePath + "/Cargo.toml", expected.browserHost.sourcePath, ...oracles.flatMap((oracle) => [oracle.scriptPath, oracle.fixturePath, oracle.schemaPath])];
    const before = new Map(inputs.map((path: string) => {
      const row = semanticOwnedInputFileSnapshot(root, path); expect(row?.nodeKind, path).toBe("file"); return [path, row!.bytes] as const;
    }));
    expect(source.split(/\r?\n/u)[0]).toBe("#!/usr/bin/env bun");
    const imports = syntax.statements.filter(ts.isImportDeclaration);
    expect(imports.map(row => ({ module: (row.moduleSpecifier as import("typescript").StringLiteral).text, names: row.importClause?.namedBindings && ts.isNamedImports(row.importClause.namedBindings) ? row.importClause.namedBindings.elements.map(node => node.name.text).sort() : [] }))).toEqual(expected.moduleImports);
    expect(expected.moduleImports.flatMap((row: { names: string[] }) => row.names).sort()).toEqual(expected.imports);
    expect(syntax.statements.filter(ts.isClassDeclaration).map((node) => node.name?.text)).toEqual(["TestScript", "CheckScript", "CheckWasmScript"]);
    const originalLevel = process.env.SEMIO_TEST_LEVEL, originalCoverage = process.env.SEMIO_COVERAGE;
    let dynamicImports = 0;
    const transformed = ts.transform(syntax, [(context) => {
      const visit: import("typescript").Visitor = (node) => {
        if (ts.isImportDeclaration(node)) return undefined;
        if (ts.isCallExpression(node) && node.expression.kind === ts.SyntaxKind.ImportKeyword) {
          expect(node.arguments).toHaveLength(1);
          expect(ts.isStringLiteral(node.arguments[0]!)).toBe(true);
          expect(oracles.map((oracle) => oracle.importSpecifier)).toContain((node.arguments[0] as import("typescript").StringLiteral).text);
          dynamicImports++;
          return ts.factory.createCallExpression(ts.factory.createIdentifier("loadInputFixture"), undefined, [node.arguments[0]!]);
        }
        if (ts.isPropertyAccessExpression(node) && ts.isMetaProperty(node.expression)) {
          if (node.name.text === "dir") return ts.factory.createStringLiteral(packageRoot);
          if (node.name.text === "url") return ts.factory.createStringLiteral("ui-host-source-only");
          if (node.name.text === "main") return ts.factory.createTrue();
          throw new Error("Unexpected UI-host import.meta dependency");
        }
        return ts.visitEachChild(node, visit, context);
      };
      return node => ts.visitNode(node, visit) as import("typescript").SourceFile;
    }]);
    const code = ts.createPrinter().printFile(transformed.transformed[0]!); transformed.dispose();
    expect(dynamicImports).toBe(oracles.length);
    const compiled = [new Bun.Transpiler({ loader: "ts" }).transformSync(code), ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext } }).outputText];
    const AsyncFunction = Object.getPrototypeOf(async () => {}).constructor;
    try {
      for (const javascript of compiled) {
        let router: ScriptRouter | undefined, refuse = false;
        const imported: string[] = [], called: string[] = [];
        const calls: unknown[][] = [];
        const dependencies = {
          BundleScript, ScriptRouter, resolve, readCargoTestPolicyV1: (environment:unknown) => {expect(environment).toBe(process.env);return {version:1};}, buildBudgetMs: buildBudgetMs, resolveTestLevel: resolveTestLevel,
          runScriptMain: async (value: ScriptRouter, ...rest: unknown[]) => { expect(rest).toEqual([]); router = value; },
          runCargoTestsV1: async (...args: unknown[]) => { calls.push(args); if (refuse) throw new Error("native delegation refused"); },
          runBudgetedTestCommand: async (...args: unknown[]) => { calls.push(args); if (refuse) throw new Error("native delegation refused"); },
          loadInputFixture: async (specifier: string) => {
            const oracle = oracles.find((row) => row.importSpecifier === specifier);
            expect(oracle, specifier).toBeDefined();
            imported.push(oracle!.scope);
            const absolute = resolve(packageRoot, specifier);
            expect(absolute).toBe(join(root, oracle!.scriptPath));
            const actual = await import(pathToFileURL(absolute).href) as Record<string, (...args: unknown[]) => unknown>;
            expect(typeof actual[oracle!.functionName]).toBe("function");
            return { [oracle!.functionName]: (...args: unknown[]) => { called.push(oracle!.scope); expect(args).toEqual(oracle!.arguments); return actual[oracle!.functionName]!(...args); } };
          },
        };
        await new AsyncFunction(...Object.keys(dependencies), javascript.replace(/^#!\/usr\/bin\/env bun\r?\n/u, ""))(...Object.values(dependencies));
        expect(router).toBeInstanceOf(ScriptRouter);
        expect(router!.usage()).toBe("bun ./📜️script.ts <test|check|check-wasm> [args…]");
        expect(imported).toEqual([]);
        for (const row of expected.nativeCases) {
          process.env.SEMIO_TEST_LEVEL = "fundamental"; delete process.env.SEMIO_COVERAGE;
          calls.length = 0;
          await router!.run(row.segments, routerInvocation("workspace-contract"));
          const args = ["--no-fail-fast", ...row.rest];
          expect(calls).toEqual([[process.execPath, [expected.browserHost.script], { cwd: packageRoot, budgetMs: buildBudgetMs() }], [{manifestPath:join(packageRoot,"Cargo.toml"),packages:[expected.cargo.name],cwd:packageRoot,extraArgs:args},{version:1}]]);
          expect(process.env.SEMIO_TEST_LEVEL).toBe(row.level);
          expect(partitionNextestExecutionFilters(args)).toEqual(row.partition);
          expect(imported).toEqual([]);
        }
        for (const row of expected.checks) {
          calls.length = 0;
          await router!.run([row.command], routerInvocation("workspace-contract"));
          expect(calls).toEqual([["cargo", row.args, { cwd: packageRoot, budgetMs: buildBudgetMs() }]]);
          expect(imported).toEqual([]);
        }
        for (const row of expected.sourceCases as { segments: string[]; scopes: string[] }[]) {
          calls.length = 0; imported.length = 0; called.length = 0;
          await router!.run(row.segments, routerInvocation("workspace-contract"));
          expect(imported).toEqual(row.scopes);
          expect(called).toEqual(row.scopes);
          expect(calls).toEqual([]);
        }
        imported.length = 0; called.length = 0;
        for (const args of expected.rejectedSegments) await expect(router!.run(args, routerInvocation("workspace-contract"))).rejects.toThrow();
        expect([imported, called]).toEqual([[], []]);
        expect(calls).toEqual([]);
        await expect(router!.run(["unknown"], routerInvocation("workspace-contract"))).rejects.toThrow('unknown command "unknown"');
        expect(calls).toEqual([]);
        refuse = true;
        await expect(router!.run(["test"], routerInvocation("workspace-contract"))).rejects.toThrow("native delegation refused");
        expect(calls).toHaveLength(1);
        calls.length = 0;
        await expect(router!.run(["check-wasm"], routerInvocation("workspace-contract"))).rejects.toThrow("native delegation refused");
        expect(calls).toHaveLength(1);
      }
    } finally {
      if (originalLevel === undefined) delete process.env.SEMIO_TEST_LEVEL; else process.env.SEMIO_TEST_LEVEL = originalLevel;
      if (originalCoverage === undefined) delete process.env.SEMIO_COVERAGE; else process.env.SEMIO_COVERAGE = originalCoverage;
      for (const [path, bytes] of before) expect(semanticOwnedInputFileSnapshot(root, path)?.bytes, path + " changed during router proof").toEqual(bytes);
    }
  });
}
