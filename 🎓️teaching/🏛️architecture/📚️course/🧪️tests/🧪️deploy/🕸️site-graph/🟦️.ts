/** 🔗️ The site build as graph tasks: `build` is a receipt-cached product of a complete key, `publish` only verifies and stages what `build` published, and `deploy-check` reaches the compiler and the site through the graph instead of repeating them.
 * The manifest under test is the live one, or the file named by `COURSE_MANIFEST`. The key-completeness oracle is the third-party TypeScript module resolver with `picomatch` over the build inputs.
 * @see ./🧬️schema/🔣️.json — the contract of `🧫️fixtures/🕸️site-graph/🔣️.json` */
import Ajv from "ajv";
import picomatch from "picomatch";
import ts from "typescript";
import { existsSync, readFileSync } from "node:fs";
import { join, relative, resolve, sep } from "node:path";
import { expect, it } from "vitest";
import fixture from "../../../🧫️fixtures/🕸️site-graph/🔣️.json" with { type: "json" };
import deployment from "../../../🚀️deploy/🔣️.json" with { type: "json" };
import { courseSiteBuildPlan } from "../../../🚀️deploy/🌐️site/🟦️.ts";
import { deployCheckSteps, DEPLOY_CHECK_GRAPH_TASKS } from "../../../🚀️deploy/⚖️readiness/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import { repoRoot, site } from "../🧰️deploy-fixtures/🟦️.ts";

type Target = { cache?: boolean; dependsOn?: string[]; inputs?: unknown[]; outputs?: string[]; metadata?: { semio?: { cache?: { class: string }; closure?: { typescript?: string[] } } } };
const bundle = join(site, "📦️packages/🟦️typescript");
const manifest = JSON.parse(readFileSync(process.env.COURSE_MANIFEST ?? join(bundle, "📋️project.json"), "utf8")) as { namedInputs: Record<string, unknown[]>; targets: Record<string, Target> };
const project = "@teaching/architecture-course";

const flatInputs = (inputs: readonly unknown[]): unknown[] => inputs.flatMap(input => typeof input === "string" && manifest.namedInputs[input] ? flatInputs(manifest.namedInputs[input]!) : [input]);
const globs = (inputs: readonly unknown[]): { positive: string[]; negative: string[] } => {
  const strings = flatInputs(inputs).filter((input): input is string => typeof input === "string" && input !== "sharedGlobals");
  const local = (glob: string): string => glob.replaceAll("{workspaceRoot}/", "").replaceAll("{projectRoot}", relative(repoRoot, bundle));
  return { positive: strings.filter(glob => !glob.startsWith("!")).map(local), negative: strings.filter(glob => glob.startsWith("!")).map(glob => local(glob.slice(1))) };
};
const matcher = (inputs: readonly unknown[]): ((path: string) => boolean) => {
  const { positive, negative } = globs(inputs);
  return path => positive.some(glob => picomatch(glob, { dot: true })(path)) && !negative.some(glob => picomatch(glob, { dot: true })(path));
};

const compilerOptions = (): ts.CompilerOptions => {
  const config = ts.readConfigFile(join(bundle, "tsconfig.json"), ts.sys.readFile);
  return { ...ts.parseJsonConfigFileContent(config.config, ts.sys, bundle).options, noEmit: true };
};
const valueSpecifiers = (file: string): string[] => {
  const source = ts.createSourceFile(file, readFileSync(file, "utf8"), ts.ScriptTarget.ESNext, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const found: string[] = [];
  const typeOnly = (clause: ts.ImportClause | undefined): boolean => clause !== undefined && (clause.isTypeOnly || (clause.name === undefined && clause.namedBindings !== undefined && ts.isNamedImports(clause.namedBindings) && clause.namedBindings.elements.length > 0 && clause.namedBindings.elements.every(element => element.isTypeOnly)));
  const visit = (node: ts.Node): void => {
    if (ts.isImportDeclaration(node) && ts.isStringLiteral(node.moduleSpecifier) && !typeOnly(node.importClause)) found.push(node.moduleSpecifier.text);
    else if (ts.isExportDeclaration(node) && node.moduleSpecifier && ts.isStringLiteral(node.moduleSpecifier) && !node.isTypeOnly) found.push(node.moduleSpecifier.text);
    else if (ts.isCallExpression(node) && node.arguments.length === 1 && ts.isStringLiteral(node.arguments[0]!) && (node.expression.kind === ts.SyntaxKind.ImportKeyword || (ts.isIdentifier(node.expression) && node.expression.text === "require"))) found.push((node.arguments[0] as ts.StringLiteral).text);
    ts.forEachChild(node, visit);
  };
  visit(source);
  return found;
};
const valueClosure = (entries: readonly string[]): { files: Set<string>; packages: Set<string> } => {
  const options = compilerOptions(), files = new Set<string>(), packages = new Set<string>(), queue = [...entries];
  const host: ts.ModuleResolutionHost = { fileExists: ts.sys.fileExists, readFile: ts.sys.readFile, realpath: ts.sys.realpath, directoryExists: ts.sys.directoryExists, getCurrentDirectory: () => bundle };
  while (queue.length) {
    const file = queue.pop()!;
    if (files.has(file)) continue;
    files.add(file);
    if (!/\.(?:tsx?|mts)$/u.test(file) || !existsSync(file)) continue;
    for (const specifier of valueSpecifiers(file)) {
      const resolved = ts.resolveModuleName(specifier, file, options, host).resolvedModule?.resolvedFileName;
      if (!resolved) continue;
      if (resolved.includes(`${sep}node_modules${sep}`)) { const name = [...resolved.matchAll(/node_modules\/((?:@[^/]+\/)?[^/]+)/gu)].at(-1); if (name) packages.add(name[1]!); }
      else queue.push(resolved);
    }
  }
  return { files, packages };
};

const posix = (path: string): string => path.replaceAll("\\", "/");
const declaredProgramFiles = (): Set<string> => {
  const files = new Set<string>();
  for (const entry of manifest.targets.build!.metadata?.semio?.closure?.typescript ?? []) {
    const config = resolve(repoRoot, entry.replaceAll("{workspaceRoot}/", "").replaceAll("{projectRoot}", relative(repoRoot, bundle)));
    const parsed = ts.getParsedCommandLineOfConfigFile(config, { noEmit: true }, { ...ts.sys, onUnRecoverableConfigFileDiagnostic: diagnostic => { throw Error(ts.flattenDiagnosticMessageText(diagnostic.messageText, "\n")); } })!;
    for (const file of ts.createProgram({ rootNames: parsed.fileNames, options: parsed.options }).getSourceFiles()) files.add(posix(file.fileName));
  }
  return files;
};

/** 🕸️ Registers the graph, key and plan tests of the site build. */
export function registerSiteGraphTests(): void {
  it("holds the fixture to its schema", () => {
    const ajv = new Ajv({ strict: true });
    expect(ajv.validate(schema, fixture), JSON.stringify(ajv.errors)).toBe(true);
  });
  it("caches `build` through its store receipt with a key that names the proctor origin, the bundled packages and every source the site reads", () => {
    const build = manifest.targets.build!;
    expect(build.cache).toBe(fixture.build.cache);
    expect(build.outputs).toEqual(fixture.build.outputs);
    expect(fixture.build.forbiddenDecisionClasses).not.toContain(build.metadata?.semio?.cache?.class ?? "none");
    expect(build.metadata?.semio?.cache?.class).toBeUndefined();
    const inputs = build.inputs ?? [];
    const environment = flatInputs(inputs).flatMap(input => typeof input === "object" && input !== null && "env" in input ? [(input as { env: string }).env] : []);
    for (const name of fixture.build.inputEnvironment) expect(environment).toContain(name);
    const external = new Set(flatInputs(inputs).flatMap(input => typeof input === "object" && input !== null && "externalDependencies" in input ? (input as { externalDependencies: string[] }).externalDependencies : []));
    for (const name of fixture.build.externalDependencies) expect(external, name).toContain(name);
    const covers = matcher(inputs);
    for (const sample of fixture.build.coveredSamples) { expect(existsSync(join(repoRoot, sample)), sample).toBe(true); expect(covers(sample), sample).toBe(true); }
  });
  it("covers every workspace source the site entry and the Vite configuration value-import, and every package the site bundles, by the build inputs, the router's own closure or the declared TypeScript project closure", () => {
    const bundled = valueClosure([join(site, "🟦️.ts")]), configured = valueClosure([join(site, "🏗️builder/🌐️vite/🟦️.ts")]);
    const files = new Set([...bundled.files, ...configured.files]), packages = bundled.packages;
    const router = valueClosure([join(bundle, "📜️script.ts")]).files;
    const covers = matcher(manifest.targets.build!.inputs ?? []);
    expect(manifest.targets.build!.metadata?.semio?.closure?.typescript).toEqual(fixture.build.closureTypescript);
    const program = declaredProgramFiles();
    const uncovered = [...files].map(file => relative(repoRoot, file)).filter(path => !path.startsWith("..") && !path.includes("🤖️generated") && !covers(path) && !router.has(join(repoRoot, path)) && !program.has(posix(join(repoRoot, path))));
    expect(uncovered).toEqual([]);
    const external = new Set(flatInputs(manifest.targets.build!.inputs ?? []).flatMap(input => typeof input === "object" && input !== null && "externalDependencies" in input ? (input as { externalDependencies: string[] }).externalDependencies : []));
    for (const name of ["react", "react-dom", "tailwindcss", ...[...packages].filter(name => !name.startsWith("@types/") && !["vitest", "typescript"].includes(name))]) expect(external.has(name), name).toBe(true);
  }, 120_000);
  it("makes `publish` a cheap verification over the cached build, keyed by the same proctor origin", () => {
    const publish = manifest.targets.publish!;
    expect(publish.cache).toBe(fixture.publish.cache);
    for (const task of fixture.publish.dependsOn) expect(publish.dependsOn).toContain(task);
    const environment = flatInputs(publish.inputs ?? []).flatMap(input => typeof input === "object" && input !== null && "env" in input ? [(input as { env: string }).env] : []);
    for (const name of fixture.publish.inputEnvironment) expect(environment).toContain(name);
    expect(publish.outputs).toEqual(fixture.publish.outputs);
  });
  it("reaches the compiler and the site through the graph and keeps only the live steps in the script", () => {
    const check = manifest.targets["deploy-check"]!;
    for (const task of fixture.deployCheck.dependsOn) expect(check.dependsOn, task).toContain(task);
    expect([...DEPLOY_CHECK_GRAPH_TASKS].sort()).toEqual([...fixture.deployCheck.dependsOn].sort());
    expect(project).toBe(fixture.deployCheck.dependsOn[0]!.split(":")[0]);
    const steps = deployCheckSteps(manifest.targets);
    expect(steps).not.toContain("sources type-checked");
    expect(steps.some(step => step.startsWith("site"))).toBe(false);
  });
  for (const row of fixture.plan.cases) it(`plans the site build ${row.id} with one owner and a closed key environment`, () => {
    const plan = courseSiteBuildPlan(repoRoot, bundle, row.mode as "prototype" | "release", row.environment);
    expect(plan.owner).toBe(fixture.plan.owner);
    expect(plan.mode).toBe(row.mode);
    expect(plan.config).toBe(join(site, "🏗️builder/🌐️vite/🟦️.ts"));
    for (const [name, value] of Object.entries(row.expected)) expect(plan.environment[name], name).toBe(value === "<deployment-origin>" ? `https://${deployment.proctor.host}` : value);
    expect(Object.keys(plan.environment).sort()).toEqual(["PROCTOR_URL", "SEMIO_BUILD_MODE"]);
  });
}
