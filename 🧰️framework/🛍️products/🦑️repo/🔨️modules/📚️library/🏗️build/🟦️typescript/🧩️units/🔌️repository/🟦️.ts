import {createHash} from "node:crypto";
import {createRequire} from "node:module";
import {existsSync,mkdirSync,readFileSync,realpathSync,renameSync,statSync,writeFileSync} from "node:fs";
import {dirname,join,relative,resolve} from "node:path";
import {withCompilerState} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗃️state/🟦️.ts";
import {compilerAdmissionBudgetV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import {typeScriptCheckEnvironmentV1,type TypeScriptCheckOptions} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🟦️typescript/🟦️.ts";
import {runOwnedCommand} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import {currentAgentSessionV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🪪️agent-sessions/🟦️.ts";
import {advanceScriptInvocation,createRequestedIssuedResourceControlV1,requireIssuedResourceControlInvocationV1,requireIssuedResourceControlV1,requireIssuedResourceControlRefinementV1,withRequestedIssuedResourceControlV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {withRepositoryCacheStoreV1} from "../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";
import {readNativeCallerAuthorityV1} from "../../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import {resolveBuildHelpLanguageV1} from "../../../⚙️operations/📖️help/🟦️.ts";
import rules from "../🔣️.json" with {type: "json"};
import {checkTypeScriptUnitsV1,deriveTypeScriptImportsV1,planTypeScriptUnitsV1,readTypeScriptUnitsDocumentV1,type TypeScriptImportsMemoV1,type TypeScriptUnitsHostV1} from "../🟦️.ts";

/** 📨️ One unit-wise check of an authored TypeScript configuration: the development gate next to the monolithic `typecheckRepositoryProjectV1` release gate. */
export type RepositoryTypeScriptUnitsOptionsV1 = Omit<TypeScriptCheckOptions, "stateRoot" | "storage" | "storageControl" | "mode" | "args"> & Readonly<{only: readonly string[]; jobs: number; replan: boolean}>;

const sha = (...parts: readonly (string | Uint8Array)[]): string => {const hash = createHash("sha256"); for (const part of parts) {hash.update(part); hash.update("\0");} return hash.digest("hex");};
const atomic = (path: string, text: string): void => {mkdirSync(dirname(path), {recursive: true}); const temporary = `${path}.${process.pid}.tmp`; writeFileSync(temporary, text); renameSync(temporary, path);};
const text = (key: keyof typeof rules.messages.texts, language: string, values: Readonly<Record<string, string | number>>): string => (rules.messages.texts[key] as Record<string, string>)[language]!.replace(/\{(\w+)\}/gu, (_, name: string) => String(values[name]));

/** 🪁️ Type-checks one authored configuration unit by unit against stored declaration outputs. Shared results live in the products store; incremental compiler state lives in the compiler-state directory of the agent session, so concurrent agents share results and never mutable state. */
export async function typecheckRepositoryUnitsV1(options: RepositoryTypeScriptUnitsOptionsV1): Promise<void> {
  const invocation = options.invocation, incoming = options.control;
  requireIssuedResourceControlInvocationV1(incoming, invocation);
  const authority = readNativeCallerAuthorityV1(invocation.capabilities), workspace = resolve(options.workspace);
  if (workspace !== authority.repositoryRoot) throw Error("Repository TypeScript workspace must match its Original");
  if (realpathSync(workspace) !== workspace) throw Error("TypeScript units need the canonical workspace path");
  const control = createRequestedIssuedResourceControlV1(incoming, options.timeoutMs);
  return withRepositoryCacheStoreV1(workspace, "dependencies", invocation, control, (_dependencies, dependencyPhase) => withRepositoryCacheStoreV1(workspace, "products", invocation, dependencyPhase, (store, phase) => runUnits({...options, workspace, mode: authority.original.buildMode, control: phase, storage: store.storage, storageControl: store.control, stateRoot: join(store.root, "typescript"), execution: {...options.execution, env: {...store.environment, ...options.execution?.env, TMPDIR: store.environment.TMPDIR, TMP: store.environment.TMP, TEMP: store.environment.TEMP}}})));
}

type UnitsRun = Omit<RepositoryTypeScriptUnitsOptionsV1, never> & Pick<TypeScriptCheckOptions, "stateRoot" | "storage" | "storageControl" | "mode">;

async function runUnits(options: UnitsRun): Promise<void> {
  const incoming = options.control, invocation = options.invocation;
  requireIssuedResourceControlInvocationV1(incoming, invocation);
  requireIssuedResourceControlV1(options.storageControl); requireIssuedResourceControlRefinementV1(options.storageControl, incoming);
  const document = readTypeScriptUnitsDocumentV1(rules), environment = options.execution?.env ?? process.env, language = resolveBuildHelpLanguageV1(document.messages.languages, [], environment);
  const say = (key: keyof typeof rules.messages.texts, values: Readonly<Record<string, string | number>>): void => {process.stderr.write(text(key, language, values) + "\n");};
  return withRequestedIssuedResourceControlV1(incoming, options.timeoutMs, async control => {
    await advanceScriptInvocation(invocation, "typecheck", "running");
    const require = createRequire(join(options.cwd, "package.json")), ts = require("typescript") as typeof import("typescript"), compiler = require.resolve("typescript/bin/tsc"), project = resolve(options.cwd, options.project);
    const family = relative(options.workspace, project), parsed = ts.getParsedCommandLineOfConfigFile(project, {}, {...ts.sys, onUnRecoverableConfigFileDiagnostic: diagnostic => {throw Error(ts.flattenDiagnosticMessageText(diagnostic.messageText, "\n"));}});
    if (!parsed) throw Error("TypeScript configuration could not be read: " + family);
    const session = currentAgentSessionV1({environment}).id, sharedRoot = join(options.stateRoot, "units");
    await withCompilerState({root: options.stateRoot, owner: options.owner, compiler: `typescript:${ts.version}`, configuration: JSON.stringify([family, "units", session]), mode: options.mode, control, storage: options.storage, storageControl: options.storageControl, files: ["units.state"], environment, capacity: {kind: "exact", parallelism: Math.max(1, Math.min(8, options.jobs))}}, async (directory, compilerEnvironment, actualControl) => {
      const memoryDigests = new Map<string, {key: string; digest: string}>(), digest = (path: string): string => {const absolute = join(options.workspace, path), info = statSync(absolute), key = `${info.mtimeMs}:${info.size}`, known = memoryDigests.get(absolute); if (known?.key === key) return known.digest; const value = createHash("sha256").update(readFileSync(absolute)).digest("hex"); memoryDigests.set(absolute, {key, digest: value}); return value;};
      const roots = parsed.fileNames.filter(file => file.startsWith(options.workspace + "/") && !file.includes("/node_modules/")).map(file => file.slice(options.workspace.length + 1)), memoPath = join(sharedRoot, "imports", sha(family, ts.version, JSON.stringify(parsed.options)) + ".json");
      let memoRows: Record<string, {d: string; t: readonly string[]; j: readonly string[]}> = {}, memoDirty = false;
      if (!options.replan && existsSync(memoPath)) try {memoRows = JSON.parse(readFileSync(memoPath, "utf8"));} catch {memoRows = {};}
      const memo: TypeScriptImportsMemoV1 = {get: (file, value) => memoRows[file]?.d === value ? {targets: memoRows[file]!.t, json: memoRows[file]!.j} : undefined, set: (file, value, row) => {memoRows[file] = {d: value, t: row.targets, j: row.json}; memoDirty = true;}};
      say("planning", {files: roots.length});
      const imports = deriveTypeScriptImportsV1(ts, {workspace: options.workspace, compilerOptions: parsed.options, roots, exists: path => existsSync(path), read: path => {try {return readFileSync(path, "utf8");} catch {return undefined;}}, digest, memo, signal: actualControl.signal});
      if (memoDirty) atomic(memoPath, JSON.stringify(memoRows));
      const plan = planTypeScriptUnitsV1(imports, document, family);
      atomic(join(sharedRoot, "plans", plan.digest + ".json"), JSON.stringify(plan));
      const typeRoots: string[] = []; for (let folder = dirname(project); folder.startsWith(options.workspace); folder = dirname(folder)) {const candidate = join(folder, "node_modules", "@types"); if (existsSync(candidate)) typeRoots.push(candidate);}
      const typeFiles = document.compiler.extraTypes.map(name => ts.resolveModuleName(name, project, parsed.options, ts.sys).resolvedModule?.resolvedFileName).filter((file): file is string => file !== undefined);
      let modules = ""; for (let folder = dirname(project); folder.startsWith(options.workspace) && !modules; folder = dirname(folder)) if (existsSync(join(folder, "node_modules"))) modules = realpathSync(join(folder, "node_modules"));
      const host: TypeScriptUnitsHostV1 = {workspace: options.workspace, typeRoots, typeFiles, compiler: `typescript@${ts.version}`, options: sha(JSON.stringify(parsed.options), JSON.stringify(document.compiler)), environment: modules, family, store: join(sharedRoot, "store"), session: "current", state: directory, digest,
        tsc: async (config, signal) => {
          const lines: string[] = []; let code = 0;
          try {await runOwnedCommand(process.execPath, [compiler, "-p", config, "--pretty", "false"], options.cwd, options.owner, compilerAdmissionBudgetV1(actualControl), {env: typeScriptCheckEnvironmentV1(options.mode, compilerEnvironment), signal, onLine: line => {lines.push(line);}});}
          catch (error) {const status = /status (\d+)/u.exec(String((error as Error).message))?.[1]; if (signal.aborted || (status !== "1" && status !== "2")) throw error; code = Number(status);}
          return {code, output: lines.join("\n")};
        }};
      const total = plan.units.length;
      say("checking", {units: total});
      const result = await checkTypeScriptUnitsV1(plan, document, host, {concurrency: Math.max(1, Math.min(8, options.jobs)), only: options.only.length ? plan.units.filter(unit => options.only.some(name => unit.label === name || unit.id === name)).map(unit => unit.id) : undefined, signal: actualControl.signal, onProgress: row => {say(row.status === "hit" ? "hit" : row.status === "ran" ? "ran" : "failed", {unit: plan.units.find(unit => unit.id === row.unit)?.label ?? row.unit, milliseconds: row.milliseconds, errors: row.errors});}});
      say("done", {errors: result.errors, units: result.units.length, reused: result.units.filter(unit => unit.status === "hit").length});
      if (result.errors) throw Error(`TypeScript units reported ${result.errors} errors`);
    });
    await advanceScriptInvocation(invocation, "typecheck", "complete");
  });
}
