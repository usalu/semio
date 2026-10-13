import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { basename, dirname, join, relative, resolve } from "node:path";

/** 🧬️ Emits a flat declaration closure with rewritten local references and bounded absolute paths. */
export function emitTypeScriptDeclarations(entry: string, output: string, workspace: string): number {
  const compiler = createRequire(join(workspace, "package.json"))("typescript");
  const options = { allowImportingTsExtensions: true, module: compiler.ModuleKind.ESNext, moduleResolution: compiler.ModuleResolutionKind.Bundler, resolveJsonModule: true, allowSyntheticDefaultImports: true, strict: true, skipLibCheck: true, target: compiler.ScriptTarget.ES2022, declaration: true, emitDeclarationOnly: true, rootDir: workspace, outDir: output };
  const program = compiler.createProgram([entry], options), emitted = new Map<string, string>(), sources = new Map<string, string>();
  const result = program.emit(undefined, (file: string, text: string, _bom: boolean, _error: unknown, originals: readonly { fileName: string }[] | undefined) => {
    if (!/\.d\.[cm]?ts$/u.test(file)) return;
    const source = resolve(originals?.[0]?.fileName ?? join(workspace, relative(output, file).replace(/\.d\.([cm]?)ts$/u, ".$1ts")));
    emitted.set(source, text); sources.set(source, source);
  });
  const failures = [...compiler.getPreEmitDiagnostics(program), ...result.diagnostics].filter((diagnostic: { category: number }) => diagnostic.category === compiler.DiagnosticCategory.Error);
  if (failures.length) throw Error(compiler.formatDiagnosticsWithColorAndContext(failures, { getCurrentDirectory: () => workspace, getCanonicalFileName: (file: string) => file, getNewLine: () => "\n" }));
  const name = (source: string): string => createHash("sha256").update(relative(workspace, source).replaceAll("\\", "/")).digest("hex").slice(0, 24) + (source.endsWith(".json") ? ".json" : ".d.ts");
  const pending = [...emitted.keys()];
  mkdirSync(output, { recursive: true });
  for (let index = 0; index < pending.length; index++) {
    const origin = pending[index]!, text = emitted.get(origin)!, syntax = compiler.createSourceFile(origin, text, compiler.ScriptTarget.Latest, true), edits: { start: number; end: number; value: string }[] = [];
    const visit = (node: any): void => {
      const literal = (compiler.isImportDeclaration(node) || compiler.isExportDeclaration(node)) ? node.moduleSpecifier : compiler.isImportTypeNode(node) && compiler.isLiteralTypeNode(node.argument) ? node.argument.literal : compiler.isExternalModuleReference(node) ? node.expression : undefined;
      if (literal && compiler.isStringLiteral(literal) && literal.text.startsWith(".")) {
        const resolved = compiler.resolveModuleName(literal.text, origin, options, compiler.sys).resolvedModule;
        if (resolved) {
          const source = resolve(resolved.resolvedFileName);
          if (!sources.has(source) && !resolved.isExternalLibraryImport && existsSync(source)) {
            sources.set(source, source);
            if (source.endsWith(".json")) writeFileSync(join(output, name(source)), JSON.stringify(JSON.parse(readFileSync(source, "utf8"))) + "\n");
            else if (/\.d\.[cm]?ts$/u.test(source)) { emitted.set(source, readFileSync(source, "utf8")); pending.push(source); }
          }
          if (sources.has(source)) edits.push({ start: literal.getStart(syntax), end: literal.getEnd(), value: JSON.stringify("./" + name(source).replace(/\.d\.ts$/u, ".js")) });
        }
      }
      compiler.forEachChild(node, visit);
    };
    visit(syntax);
    let generated = text;
    for (const edit of edits.sort((left, right) => right.start - left.start)) generated = generated.slice(0, edit.start) + edit.value + generated.slice(edit.end);
    const path = join(output, name(origin));
    if (path.length > 256) throw Error("Declaration output path exceeds 256 UTF-16 units");
    writeFileSync(path, generated);
  }
  const entryDeclaration = name(resolve(entry)).replace(/\.d\.ts$/u, ".js");
  writeFileSync(join(dirname(output), "🟦️.d.ts"), `export * from ${JSON.stringify("./" + basename(output) + "/" + entryDeclaration)};\n`);
  return pending.length;
}
