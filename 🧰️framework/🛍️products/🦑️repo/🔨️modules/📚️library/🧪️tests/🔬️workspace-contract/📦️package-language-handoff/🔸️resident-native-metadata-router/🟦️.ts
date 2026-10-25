/** 🔸️ The resident native metadata router tests of the package language semantic handoff suite. */
import { buildBudgetMs } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import toml from "@iarna/toml";
import Ajv from "ajv";
import { expect, test } from "bun:test";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { basename, dirname, join, posix, relative, resolve, sep } from "node:path";
import { readSemioMarker } from "../../../../🔍️discovery/🔭️repository-scan/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { fixedFilenameContractIdsForPath } from "../../../../🔍️discovery/⚖️fixed-contract-resolution/🟦️.ts";
import { readToml } from "../../🏗️shared-fixtures/🟦️.ts";
import { contractRoot } from "../../🏗️shared-fixtures/🟦️.ts";
import { inputRoot, routerInvocation, schema } from "../🟢️harness/🟦️.ts";
import type { CargoWorkspaceManifest } from "../🟢️harness/🟦️.ts";

/** 🔹 Registers the resident native metadata router tests of the package language semantic handoff suite. */
export function registerResidentNativeMetadataRouterTests(): void {
  test("resident native metadata router preserves exact sequential bounded commands without executing Cargo", async () => {
    const ts = await import("typescript"), { spyOn } = await import("bun:test");
    const root = resolve(contractRoot, "../../../../../../.."), expected = JSON.parse(readFileSync(join(inputRoot, "💾️resident-package/🔣️.json"), "utf8"));
    const packageRoot = join(root, expected.packagePath), sourcePath = join(packageRoot, "📜️script.ts");
    expect(existsSync(sourcePath)).toBe(true);
    const source = readFileSync(sourcePath, "utf8"), syntax = ts.createSourceFile(sourcePath, source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    expect(source.split(/\r?\n/u)[0]).toBe("#!/usr/bin/env repo");
    const imports = syntax.statements.filter(ts.isImportDeclaration);
    expect(imports.map(row => ({ module: (row.moduleSpecifier as import("typescript").StringLiteral).text, names: row.importClause?.namedBindings && ts.isNamedImports(row.importClause.namedBindings) ? row.importClause.namedBindings.elements.map(node => node.name.text).sort() : [] }))).toEqual(expected.moduleImports);
    expect(expected.moduleImports.flatMap((row: { names: string[] }) => row.names).sort()).toEqual(expected.imports);
    expect(syntax.statements.every(node => ts.isImportDeclaration(node) || ts.isClassDeclaration(node) || ts.isVariableStatement(node) || ts.isIfStatement(node))).toBe(true);
    expect(syntax.statements.filter(ts.isClassDeclaration).map(node => node.name?.text)).toEqual(["TestScript", "CheckWasmScript"]);
    const transformed = ts.transform(syntax, [(context) => {
      const visit: import("typescript").Visitor = (node) => {
        if (ts.isImportDeclaration(node)) return undefined;
        if (ts.isPropertyAccessExpression(node) && ts.isMetaProperty(node.expression)) {
          if (node.name.text === "dir") return ts.factory.createStringLiteral(packageRoot);
          if (node.name.text === "url") return ts.factory.createStringLiteral("resident-source-only");
          if (node.name.text === "main") return ts.factory.createTrue();
          throw new Error("Unexpected import.meta dependency");
        }
        return ts.visitEachChild(node, visit, context);
      };
      return node => ts.visitNode(node, visit) as import("typescript").SourceFile;
    }]);
    const code = ts.createPrinter().printFile(transformed.transformed[0]!); transformed.dispose();
    const compiled = [new Bun.Transpiler({ loader: "ts" }).transformSync(code), ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext } }).outputText];
    const AsyncFunction = Object.getPrototypeOf(async () => {}).constructor;
    for (const javascript of compiled) {
      let router: ScriptRouter | undefined, active = 0, refuse = false;
      const calls: unknown[][] = [], events: string[] = [];
      const dependencies = {
        BundleScript, ScriptRouter, resolve, readCargoTestPolicyV1: (environment:unknown) => {expect(environment).toBe(process.env);return {version:1};}, buildBudgetMs: buildBudgetMs,
        runScriptMain: async (value: ScriptRouter, ...rest: unknown[]) => { expect(rest).toEqual([]); router = value; },
        runCargoTestsV1: async (...args: unknown[]) => { calls.push(args); },
        runBudgetedTestCommand: async (command: string, args: string[], options: { cwd: string; budgetMs: number; env?: NodeJS.ProcessEnv }) => {
          expect(++active).toBe(1); calls.push([command, args, options]); events.push("start:" + args.at(-1));
          await Promise.resolve(); active--; events.push("end:" + args.at(-1));
          if (refuse) throw new Error("command refused");
        },
      };
      const body = javascript.replace(/^#!\/usr\/bin\/env bun\r?\n/u, "");
      expect(body.startsWith("#!")).toBe(false);
      await new AsyncFunction(...Object.keys(dependencies), body)(...Object.values(dependencies));
      expect(router).toBeInstanceOf(ScriptRouter);
      expect(router!.usage()).toBe("repo ./📜️script.ts <test|check-wasm> [args…]");
      await router!.run(["test"], routerInvocation("workspace-contract"));
      expect(calls).toEqual([[{manifestPath:join(packageRoot,"Cargo.toml"),packages:[expected.cargo.package.name],cwd:packageRoot,extraArgs:["--lib"]},{version:1}]]);
      calls.length = 0;
      await router!.run(["check-wasm"], routerInvocation("workspace-contract"));
      expect(calls).toEqual(expected.wasmTargets.map((target: string) => ["cargo", ["check", "--manifest-path", "Cargo.toml", "--lib", "--target", target], { cwd: packageRoot, budgetMs: buildBudgetMs() }]));
      expect(events).toEqual(expected.wasmTargets.flatMap((target: string) => ["start:" + target, "end:" + target]));
      calls.length = 0;
      for (const command of ["test", "check-wasm"]) await expect(router!.run([command, "unexpected"], routerInvocation("workspace-contract"))).rejects.toThrow("accepts no arguments");
      expect(calls).toEqual([]);
      await expect(router!.run(["unknown"], routerInvocation("workspace-contract"))).rejects.toThrow('unknown command "unknown"');
      expect(calls).toEqual([]);
      refuse = true;
      await expect(router!.run(["check-wasm"], routerInvocation("workspace-contract"))).rejects.toThrow("command refused");
      expect(calls).toHaveLength(1);
      expect((calls[0]![1] as string[]).at(-1)).toBe(expected.wasmTargets[0]);
    }
  });
  test("UI-host metadata binds canonical source inputs and exact existing Cargo ownership", async () => {
    const { parse: parseJsonc } = await import("jsonc-parser"), { win32 } = await import("node:path");
    const { semanticDirectoryKindId } = await import("../../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts"), { semanticOwnedInputFileSnapshot } = await import("../../../../🔍️discovery/✅️owned-file-approvals/🟦️.ts");
    const root = resolve(contractRoot, "../../../../../../.."), fixtureRoot = join(inputRoot, "🖥️ui-host-package"), captured = new Map<string, Uint8Array>();
    const read = (path: string): Buffer => {
      const row = semanticOwnedInputFileSnapshot(root, path);
      expect(row?.nodeKind, path).toBe("file");
      const bytes = row!.bytes;
      captured.set(path, bytes);
      return Buffer.from(bytes);
    };
    const neutralPath = posix.join(relative(root, fixtureRoot).split(sep).join("/"), "🔣️.json"), expected = JSON.parse(read(neutralPath).toString("utf8"));
    const validate = new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(read("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🤝️package-language-kind-handoff/🖥️ui-host-package/🔣️.json").toString("utf8")));
    expect(validate(expected), JSON.stringify(validate.errors)).toBe(true);
    expect(parseJsonc(Buffer.from(captured.get(neutralPath)!).toString("utf8"))).toEqual(expected);
    try {
      const projectPath = expected.packagePath + "/📋️project.json", projectText = read(projectPath).toString("utf8"), project = JSON.parse(projectText);
      expect(parseJsonc(projectText)).toEqual(project);
      expect(project).toEqual(expected.project);
      const packageRoot = join(root, expected.packagePath), manifestText = read(expected.packagePath + "/Cargo.toml").toString("utf8"), manifest = toml.parse(manifestText);
      expect(Bun.TOML.parse(manifestText)).toEqual(manifest);
      expect((manifest.package as { name: string }).name).toBe(expected.cargo.name);
      expect(readSemioMarker(join(packageRoot, "Cargo.toml"), "🦀️rust", schema)).toEqual({ role: expected.cargo.role, id: expected.cargo.id });
      expect((manifest.lib as { path: string }).path).toBe(expected.cargo.libPath);
      for (const row of expected.internalDependencies) {
        let table: unknown = manifest;
        for (const key of row.table) table = (table as Record<string, unknown>)[key];
        expect((table as Record<string, unknown>)[row.name], row.name).toEqual({ path: row.path, ...(row.package ? { package: row.package } : {}) });
      }
      const workspaceInput = semanticOwnedInputFileSnapshot(root, "Cargo.toml");
      expect(workspaceInput?.nodeKind).toBe("file");
      const workspaceBefore = Buffer.from(workspaceInput!.bytes), workspace = readToml<CargoWorkspaceManifest>(workspaceBefore.toString("utf8")).workspace;
      const parsedBefore = Bun.TOML.parse(workspaceBefore.toString("utf8"));
      expect("workspace" in parsedBefore ? parsedBefore.workspace : undefined).toEqual(workspace);
      expect(workspace.members.filter((path) => path === expected.packagePath)).toEqual([expected.packagePath]);
      expect(workspace.dependencies[expected.cargo.name]).toEqual({ path: expected.packagePath });
      expect(semanticOwnedInputFileSnapshot(root, expected.packagePath + "/package.json")).toBeNull();
      expect(fixedFilenameContractIdsForPath(projectPath, schema)).toContain("build-project-manifest");
      expect(fixedFilenameContractIdsForPath(expected.packagePath + "/📜️script.ts", schema)).toContain("root-script");
      expect(semanticDirectoryKindId(basename(dirname(inputRoot)), schema, { parentKindId: "members-of-modules" })).toBe("fixtures");
      expect(readdirSync(packageRoot).sort()).toEqual([...expected.packageFiles].sort());
      for (const member of expected.members) {
        expect(semanticDirectoryKindId(member.name, schema, { parentKindId: member.ownerKindId }), member.name).toBe(member.registry);
        expect(schema.semanticDirectoryMemberKinds[member.registry].memberNames.filter((name: string) => name === member.name)).toEqual([member.name]);
      }
      expect(semanticDirectoryKindId("📦️packages", schema, { parentKindId: "members-of-members-of-modules" })).toBe("packages");
      expect(semanticDirectoryKindId("🦀️rust", schema, { parentKindId: "packages" })).toBe("rust-language");
      for (const [base, operand, target] of [[expected.packagePath, expected.cargo.libPath, expected.cargo.libSourcePath], ...expected.sourceOracles.map((oracle: { importSpecifier: string; scriptPath: string }) => [expected.packagePath, oracle.importSpecifier, oracle.scriptPath]), [expected.packagePath, expected.browserHost.script, expected.browserHost.sourcePath], [expected.packagePath, expected.project.$schema, "node_modules/nx/schemas/project-schema.json"]]) {
        expect(posix.normalize(posix.join(base, operand))).toBe(target);
        expect(win32.normalize(win32.join(base, operand)).replaceAll("\\", "/")).toBe(target);
      }
      const entry = read(expected.cargo.libSourcePath).toString("utf8");
      expect(entry).not.toContain("🌉️abi");
      for (const path of [expected.browserHost.sourcePath, expected.packagePath + "/📜️script.ts"]) read(path);
      for (const oracle of expected.sourceOracles as { ownerPath: string; scriptPath: string; fixturePath: string; schemaPath: string }[]) {
        for (const path of [oracle.scriptPath, oracle.fixturePath, oracle.schemaPath]) read(path);
        for (const path of [oracle.fixturePath, oracle.schemaPath]) expect(parseJsonc(Buffer.from(captured.get(path)!).toString("utf8"))).toEqual(JSON.parse(Buffer.from(captured.get(path)!).toString("utf8")));
        for (const old of ["📜️script.ts", "🧪️fixture/🔣️.json", "🧪️schema/🔣️.json", "🧪️test/🔣️s.json"]) expect(semanticOwnedInputFileSnapshot(root, posix.join(oracle.ownerPath, old))).toBeNull();
      }
      const toolchainText = read("rust-toolchain.toml").toString("utf8"), toolchain = toml.parse(toolchainText);
      expect(Bun.TOML.parse(toolchainText)).toEqual(toolchain);
      expect((toolchain.toolchain as { targets: string[] }).targets).toContain("wasm32-unknown-unknown");
      const workspaceAfter = Buffer.from(semanticOwnedInputFileSnapshot(root, "Cargo.toml")!.bytes), current = readToml<CargoWorkspaceManifest>(workspaceAfter.toString("utf8")).workspace;
      expect(current.members.filter((path) => path === expected.packagePath)).toEqual([expected.packagePath]);
      expect(current.dependencies[expected.cargo.name]).toEqual(workspace.dependencies[expected.cargo.name]);
    } finally {
      for (const [path, bytes] of captured) expect(semanticOwnedInputFileSnapshot(root, path)?.bytes, path + " changed during metadata proof").toEqual(bytes);
    }
  });
}
