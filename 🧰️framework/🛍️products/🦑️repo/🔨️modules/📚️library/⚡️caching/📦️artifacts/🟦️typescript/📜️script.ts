#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import assert from "node:assert/strict";
import { copyFileSync, existsSync, mkdirSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve, sep } from "node:path";
import { createRequire } from "node:module";
import { BundleScript, ScriptRouter, type ScriptCommand } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { getWorkspaceRoot } from "../../../🗂️workspaces/🟦️.ts";
import { runRepositoryCommand } from "../../../🏃️process/🎛️owned-execution/🟦️.ts";

/** 🧪️ Declares artifact-owned suites and commands; suite paths resolve beneath the artifact root. */
export interface ArtifactTypeScriptPackageOptions {
  readonly suites?: readonly string[];
  readonly commands?: Readonly<Record<string, ScriptCommand>>;
}

/** 🟦️ Builds and resolves a declaration-only TypeScript artifact package from its taxonomy source. */
export async function runArtifactTypeScriptPackageMain(packageRoot: string, packageName: string, options: ArtifactTypeScriptPackageOptions = {}): Promise<void> {
  const source = resolve(packageRoot, "../../🟦️.ts"), output = resolve(packageRoot, "dist");
  const typeScript = async (entry: string, args: string[], skipLibraries = true): Promise<void> => {
    await runRepositoryCommand(process.execPath, ["x", "tsc", entry, ...args, "--allowImportingTsExtensions", "--module", "ESNext", "--moduleResolution", "Bundler", "--resolveJsonModule", "--allowSyntheticDefaultImports", "--strict", ...(skipLibraries ? ["--skipLibCheck"] : []), "--target", "ES2022"], getWorkspaceRoot(), `artifact-typescript:${packageName}:tsc`, 120_000);
  };
  const suites = (options.suites ?? []).map((suite) => {
    const path = resolve(packageRoot, "../..", suite);
    if (!existsSync(path)) throw new Error(`${packageName} registers the missing suite ${suite}`);
    return path;
  });
  const checkSuites = async (): Promise<void> => {
    for (const suite of suites) await typeScript(suite, ["--noEmit", "--allowImportingTsExtensions"]);
  };
  const typeRoot = join(output, "🧬️types");
  const copyDeclarationAssets = (): number => {
    const compiler = createRequire(import.meta.url)("typescript"), repositoryRoot = getWorkspaceRoot();
    const files: string[] = [], directories = [typeRoot];
    while (directories.length) { const directory = directories.pop()!; for (const entry of readdirSync(directory, { withFileTypes: true })) { const path = join(directory, entry.name); if (entry.isDirectory()) directories.push(path); else if (/\.d\.[cm]?ts$/.test(entry.name)) files.push(path); } }
    const visited = new Set<string>();
    for (let index = 0; index < files.length; index++) {
      const declaration = files[index]!;
      if (visited.has(declaration)) continue;
      visited.add(declaration);
      let text = readFileSync(declaration, "utf8");
      const originalDirectory = join(repositoryRoot, dirname(relative(typeRoot, declaration)));
      for (const imported of compiler.preProcessFile(text, true, true).importedFiles) {
        if (!imported.fileName.startsWith(".")) continue;
        const sourceImport = resolve(originalDirectory, imported.fileName);
        const candidates = imported.fileName.endsWith(".json") || /\.d\.[cm]?ts$/.test(imported.fileName) ? [sourceImport] : /\.[cm]?[jt]s$/.test(imported.fileName) ? [sourceImport.replace(/\.([cm])?[jt]s$/, (_: string, kind: string | undefined) => `.d.${kind ?? ""}ts`)] : [`${sourceImport}.d.ts`, join(sourceImport, "index.d.ts")];
        for (const from of candidates) {
          if (!existsSync(from)) continue;
          const local = relative(repositoryRoot, from);
          if (local === ".." || local.startsWith(`..${sep}`)) throw new Error(`Declaration asset escapes ${packageName}: ${imported.fileName}`);
          const to = join(typeRoot, local);
          if (!existsSync(to)) { mkdirSync(dirname(to), { recursive: true }); copyFileSync(from, to); }
          if (/\.d\.[cm]?ts$/.test(to)) files.push(to);
          break;
        }
      }
      const syntax = compiler.createSourceFile(declaration, text, compiler.ScriptTarget.Latest, true), edits: { start: number; end: number; value: string }[] = [];
      const visit = (node: any): void => {
        const literal = (compiler.isImportDeclaration(node) || compiler.isExportDeclaration(node)) ? node.moduleSpecifier : compiler.isImportTypeNode(node) && compiler.isLiteralTypeNode(node.argument) ? node.argument.literal : compiler.isExternalModuleReference(node) ? node.expression : undefined;
        if (literal && compiler.isStringLiteral(literal) && literal.text.startsWith(".") && !/\.d\.[cm]?ts$/.test(literal.text) && /\.[cm]?ts$/.test(literal.text)) edits.push({ start: literal.getStart(syntax), end: literal.getEnd(), value: JSON.stringify(literal.text.replace(/\.([cm])?ts$/, (_: string, kind: string | undefined) => `.${kind ?? ""}js`)) });
        compiler.forEachChild(node, visit);
      };
      visit(syntax);
      for (const edit of edits.sort((a, b) => b.start - a.start)) text = text.slice(0, edit.start) + edit.value + text.slice(edit.end);
      writeFileSync(declaration, text);
    }
    const entry = relative(output, join(typeRoot, relative(repositoryRoot, source).replace(/\.ts$/, ".js"))).split(sep).join("/");
    writeFileSync(join(output, "🟦️.d.ts"), `export * from ${JSON.stringify(`./${entry}`)};\n`);
    return visited.size;
  };
  const build = async (): Promise<void> => {
    rmSync(output, { recursive: true, force: true });
    mkdirSync(output, { recursive: true });
    const result = await Bun.build({ entrypoints: [source], outdir: output, naming: "🟦️.js", target: "bun", format: "esm", minify: false });
    if (!result.success) throw new AggregateError(result.logs, `Failed to build ${packageName}`);
    await typeScript(source, ["--declaration", "--emitDeclarationOnly", "--rootDir", getWorkspaceRoot(), "--outDir", typeRoot]);
    const assets = copyDeclarationAssets();
    console.log(`[artifact-typescript] built ${packageName} outputs=${result.outputs.length + 1 + assets}`);
  };
  class BuildScript extends BundleScript { async run(): Promise<void> { await build(); } }
  class CheckScript extends BundleScript {
    async run(): Promise<void> {
      const result = await Bun.build({ entrypoints: [source], target: "bun", format: "esm" });
      if (!result.success) throw new AggregateError(result.logs, `Failed to check ${packageName}`);
      await typeScript(source, ["--noEmit"]);
      await checkSuites();
      console.log(`[artifact-typescript] checked ${packageName} suites=${suites.length}`);
    }
  }
  class TestScript extends BundleScript {
    async run(): Promise<void> {
      await build();
      const artifact = await import(Bun.resolveSync(packageName, packageRoot));
      const probe = join(output, "🧪️consumer.ts"), typeRoots = join(output, "🧪️types");
      mkdirSync(typeRoots);
      const assertion = Object.hasOwn(artifact, "definition") ? "const definitionId: typeof artifact.definition.id = artifact.definition.id;\nvoid definitionId;" : `const artifactModule: typeof import(${JSON.stringify(packageName)}) = artifact;\nvoid artifactModule;`;
      writeFileSync(probe, `import * as artifact from ${JSON.stringify(packageName)};\n${assertion}\n`);
      try { await typeScript(probe, ["--noEmit", "--typeRoots", typeRoots], false); } finally { rmSync(probe, { force: true }); rmSync(typeRoots, { recursive: true, force: true }); }
      assert.equal(typeof artifact, "object", `${packageName} did not resolve as an ES module`);
      await checkSuites();
      for (const suite of suites) await runRepositoryCommand(process.execPath, ["test", `./${relative(getWorkspaceRoot(), suite)}`], getWorkspaceRoot(), `artifact-typescript:${packageName}:suite`, 300_000);
      console.log(`[artifact-typescript] tested ${packageName} exports=${Object.keys(artifact).length} suites=${suites.length}`);
    }
  }
  const router = new ScriptRouter(packageRoot).register("build", BuildScript).register("check", CheckScript).register("test", TestScript);
  for (const [name, Command] of Object.entries(options.commands ?? {})) router.register(name, Command);
  const segments = process.argv.slice(2);
  await receiveScriptProcessInvocation(process.env, original => (router).run(segments.length ? segments : ["test"], original));
}
