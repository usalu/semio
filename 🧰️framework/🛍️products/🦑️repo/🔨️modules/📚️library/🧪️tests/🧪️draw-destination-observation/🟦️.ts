import { afterAll, expect, spyOn, test } from "bun:test";
import { createHash, randomUUID } from "node:crypto";
import { appendFileSync, chmodSync, lstatSync, mkdirSync, readFileSync, readdirSync, renameSync, symlinkSync, writeFileSync } from "node:fs";
import * as fs from "node:fs";
import { dirname, isAbsolute, join, relative, resolve, sep } from "node:path";
import Ajv from "ajv";
import fg from "fast-glob";
import { parse, type ParseError } from "jsonc-parser";
import * as toml from "@iarna/toml";
import * as ts from "typescript";
import { loadCatalogTaxonomy, noFollowDirectoryAncestry, semanticOwnedInputFileSnapshot, semanticPathProjectionAuthority, type SemanticProjectionAuthorityNode } from "../../🔍️discovery/🟦️.ts";

type ObservationCase = Readonly<{ id: string; mutation: string; expected: "accepted" | "rejected"; error: string | null; contentReads: number }>;
type ObservationVector = Readonly<{ schemaVersion: number; contractId: string; authority: Readonly<{ catalog: string; catalogSha256: string; projectionIndex: number; projectionContractId: string; mappingDigest: string; authoredSource: string; authoredSourceSha256: string }>; observation: Readonly<{ files: number; directories: number; nodes: number; maxPathBytes: number }>; foldVectors: readonly Readonly<{ left: string; right: string; equal: boolean }>[]; cases: readonly ObservationCase[]; ancestorSwap: Readonly<{ boundaries: readonly ("reader-before-content" | "fact" | "names" | "content")[]; error: string; descendantObservations: number; contentReads: number }> }>;
type Projection = Readonly<{ contractId: string; sourceRoot: string; destinationRoot: string; mappingDigest: string; mappings: readonly Readonly<{ sourcePath: string; destinationPath: string }>[] }>;
type AuthoredSource = Readonly<{ members: readonly Readonly<{ path: string; content: string; format: string }>[]; oracle: Readonly<{ configuration: readonly Readonly<{ path: string; sourceEntry: string; destinationEntry: string }>[] }> }>;
type NodeFact = Readonly<{ kind: "file" | "directory" | "symlink" | "other"; mode: number; size: number; identity: string }>;
type DestinationIO = Readonly<{ fact: (path: string) => NodeFact | null; names: (path: string) => readonly string[]; content: (path: string) => string }>;
type DestinationInput = Readonly<{ repoRoot: string; destinationRoot: string; io: DestinationIO }>;
type Run = Readonly<{ root: string; repoRoot: string; started: string; report: string }>;

const libraryRoot = resolve(import.meta.dir, "../.."), repoRoot = resolve(libraryRoot, "../../../../..");
const vectorText = readFileSync(join(import.meta.dir, "../🧪️🛟️draw-destination-observation/🔣️.json"), "utf8"), vector = JSON.parse(vectorText) as ObservationVector;
const schemaText = readFileSync(join(import.meta.dir, "../🧪️🛟️draw-destination-observation/🧬️schema/🔣️.json"), "utf8"), schema = JSON.parse(schemaText);
const catalogPath = resolve(import.meta.dir, vector.authority.catalog), catalogBytes = readFileSync(catalogPath);
const projection = JSON.parse(catalogBytes.toString()).projections[vector.authority.projectionIndex] as Projection;
const authoredPath = resolve(import.meta.dir, vector.authority.authoredSource), authoredBytes = readFileSync(authoredPath), authored = JSON.parse(authoredBytes.toString()) as AuthoredSource;
const authoredSchemaPath = join(dirname(authoredPath), "🧬️schema/🔣️.json"), authoredSchema = JSON.parse(readFileSync(authoredSchemaPath, "utf8"));
const taxonomy = loadCatalogTaxonomy();
const reportOwner = join(repoRoot, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/📓️draw-destination-observation");
const inputPaths = [import.meta.filename, join(import.meta.dir, "../🧪️🛟️draw-destination-observation/🔣️.json"), join(import.meta.dir, "../🧪️🛟️draw-destination-observation/🧬️schema/🔣️.json"), catalogPath, authoredPath, authoredSchemaPath, join(libraryRoot, "🔣️taxonomy.json"), join(libraryRoot, "🔍️discovery/🟦️.ts"), join(import.meta.dir, "../🧪️🛟️draw-destination-observation/🧪️registration/🔣️.json"), join(import.meta.dir, "../🧪️🛟️draw-destination-observation/🧪️registration/🧬️schema/🔣️.json")];
const hash = (bytes: string | Uint8Array): string => createHash("sha256").update(bytes).digest("hex");
const byteOrder = (left: string, right: string): number => Buffer.from(left).compare(Buffer.from(right));
const validateNeutral = new Ajv({ strict: true, allErrors: true }).compile(schema);
if (!validateNeutral(vector) || hash(catalogBytes) !== vector.authority.catalogSha256 || hash(authoredBytes) !== vector.authority.authoredSourceSha256 || projection.contractId !== vector.authority.projectionContractId || projection.mappingDigest !== vector.authority.mappingDigest) throw new Error("Draw observation input authority mismatch");
const identities = (): readonly Readonly<{ path: string; hash: string; size: number }>[] => inputPaths.map((path) => { const bytes = readFileSync(path); return { path, hash: hash(bytes), size: bytes.length }; });
const beforeInputs = identities();

/** 🔤️ Uses the strongest declared NFC, case and VS16 comparison without accepting alternate spelling. */
function drawDestinationKey(value: string): string {
  return value.normalize("NFC").replaceAll("\uFE0F", "").toLocaleLowerCase("und");
}

/** 🔒️ Reads only the complete exact canonical Draw destination; no source-content mode exists. */
function readDrawDestination(input: DestinationInput): readonly SemanticProjectionAuthorityNode[] {
  if (!isAbsolute(input.repoRoot) || resolve(input.repoRoot) !== input.repoRoot || input.destinationRoot !== projection.destinationRoot) throw new Error("root:expected exact canonical Draw destination");
  const io = input.io, checked = new Map<string, Readonly<{ fact: NodeFact; names: readonly string[] }>>();
  const requireDirectory = (path: string): NodeFact => {
    const fact = io.fact(path);
    if (!fact) throw new Error("missing:directory");
    if (fact.kind === "symlink") throw new Error("unsafe:directory symlink");
    if (fact.kind !== "directory") throw new Error("kind:directory required");
    return fact;
  };
  const names = (path: string): readonly string[] => {
    const fact = requireDirectory(path), values = [...io.names(path)].sort(byteOrder), previous = checked.get(path);
    if (previous && (previous.fact.identity !== fact.identity || previous.names.join("\0") !== values.join("\0"))) throw new Error("changed:directory snapshot");
    checked.set(path, { fact, names: values });
    return values;
  };
  const chain: string[] = [];
  for (let current = input.repoRoot;; current = dirname(current)) {
    chain.unshift(current);
    if (current === dirname(current)) break;
  }
  for (const path of chain) requireDirectory(path);
  let current = input.repoRoot;
  for (const segment of projection.destinationRoot.split("/")) {
    const children = names(current);
    if (children.some(child => child !== segment && drawDestinationKey(child) === drawDestinationKey(segment))) throw new Error("folded:canonical ancestor spelling");
    if (!children.includes(segment)) throw new Error("missing:canonical destination");
    current = join(current, segment);
    requireDirectory(current);
  }
  current = input.repoRoot;
  for (const segment of projection.sourceRoot.split("/")) {
    const children = names(current);
    if (children.some(child => child !== segment && drawDestinationKey(child) === drawDestinationKey(segment))) throw new Error("folded:source ancestor spelling");
    if (!children.includes(segment)) { current = ""; break; }
    current = join(current, segment);
    requireDirectory(current);
  }
  if (current) throw new Error("source-present:old Draw source root");
  const expectedFiles = new Set(projection.mappings.map(({ destinationPath }) => destinationPath));
  const expectedDirectories = new Set<string>([projection.destinationRoot]);
  for (const file of expectedFiles) for (let directory = dirname(file); directory !== dirname(projection.destinationRoot); directory = dirname(directory)) expectedDirectories.add(directory);
  const expected = new Map<string, "file" | "directory">([...expectedDirectories].map(path => [path, "directory"] as const));
  for (const path of expectedFiles) expected.set(path, "file");
  const directoryNodes: SemanticProjectionAuthorityNode[] = [], fileFacts = new Map<string, NodeFact>();
  const visit = (path: string): void => {
    const absolute = join(input.repoRoot, path), children = names(absolute), expectedChildren = [...expected.keys()].filter(candidate => dirname(candidate) === path).map(candidate => candidate.slice(path.length + 1)).sort(byteOrder);
    for (const child of children) if (expectedChildren.some(expected => child !== expected && drawDestinationKey(child) === drawDestinationKey(expected))) throw new Error("folded:canonical member spelling");
    if (children.join("\0") !== expectedChildren.join("\0")) throw new Error("membership:complete canonical child set");
    directoryNodes.push({ path, nodeKind: "directory" });
    for (const child of children) {
      const selected = path + "/" + child, fact = io.fact(join(input.repoRoot, selected));
      if (!fact) throw new Error("membership:missing canonical member");
      if (fact.kind === "symlink") throw new Error("unsafe:canonical member symlink");
      if (fact.kind !== expected.get(selected)) throw new Error("kind:canonical member kind");
      if (fact.kind === "directory") visit(selected);
      else fileFacts.set(selected, fact);
    }
  };
  visit(projection.destinationRoot);
  const files: SemanticProjectionAuthorityNode[] = [];
  for (const path of [...fileFacts.keys()].sort(byteOrder)) {
    const absolute = join(input.repoRoot, path), before = io.fact(absolute), expected = fileFacts.get(path)!;
    if (!before || before.kind !== "file" || before.identity !== expected.identity) throw new Error("changed:file before content");
    const content = io.content(absolute), after = io.fact(absolute);
    if (!after || after.kind !== "file" || after.identity !== expected.identity) throw new Error("changed:file after content");
    files.push({ path, nodeKind: "file", content });
  }
  for (const path of [...checked.keys()]) names(path);
  return [...directoryNodes, ...files].sort((left, right) => byteOrder(left.path, right.path));
}

/** 🧪️ Creates one uniquely owned, no-follow evidence root without deleting earlier runs. */
function makeRun(id: string): Run {
  const parent = join(reportOwner, "🧾️runs");
  const parts = relative(repoRoot, parent).split(sep);
  let current = repoRoot;
  for (const part of parts) {
    current = join(current, part);
    try { const stat = lstatSync(current); if (!stat.isDirectory() || stat.isSymbolicLink()) throw new Error("Unsafe retained run parent"); }
    catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error; mkdirSync(current); }
  }
  const root = join(parent, "🔖️" + randomUUID());
  mkdirSync(root);
  const fixtureRoot = join(root, "📦️fixture");
  mkdirSync(fixtureRoot);
  const started = new Date().toISOString(), report = join(root, "📝️.md");
  writeFileSync(report, "# Draw Destination Observation Run\n\nCase: " + id + "\n\nStarted: " + started + "\n\nPID: " + process.pid + "\n\nAll fixture inputs and outcomes are retained.\n", { flag: "wx" });
  return { root, repoRoot: fixtureRoot, started, report };
}

/** 🧱️ Materializes canonical nodes solely from the surviving authored source contract. */
function canonicalFiles(): ReadonlyMap<string, string> {
  return new Map(projection.mappings.map(({ sourcePath, destinationPath }) => {
    const member = authored.members.find(({ path }) => sourcePath === projection.sourceRoot + "/" + path);
    if (!member) throw new Error("Authored Draw member is absent");
    let content = member.content.replaceAll("{{sourceRoot}}", projection.sourceRoot);
    for (const configuration of authored.oracle.configuration) if (sourcePath === projection.sourceRoot + "/" + configuration.path) content = content.replace(JSON.stringify(configuration.sourceEntry), JSON.stringify(configuration.destinationEntry));
    if (content.includes("{{")) throw new Error("Unresolved authored fixture binding");
    return [destinationPath, content] as const;
  }));
}

/** 🧾️ Supplies real filesystem observations with exact inode/read guards and an old-content tripwire. */
function physicalIO(run: Run, row: ObservationCase, reads: string[], calls: string[], beforeBoundary?: (operation: "fact" | "names" | "content", path: string) => void): DestinationIO {
  const ancestry = (path: string): void => {
    try { noFollowDirectoryAncestry(path, "Draw destination observation"); }
    catch (error) { throw new Error("unsafe:" + (error as Error).message); }
  };
  const fact = (path: string): NodeFact | null => {
    beforeBoundary?.("fact", path);
    calls.push("fact:" + path);
    ancestry(dirname(path));
    try {
      const stat = lstatSync(path);
      return { kind: stat.isSymbolicLink() ? "symlink" : stat.isDirectory() ? "directory" : stat.isFile() ? "file" : "other", mode: stat.mode & 0o7777, size: stat.size, identity: [stat.dev, stat.ino, stat.mode, stat.size, stat.mtimeMs].join(":") };
    } catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") return null; throw error; }
  };
  return {
    fact,
    names: (path) => { beforeBoundary?.("names", path); calls.push("names:" + path); ancestry(path); const values = readdirSync(path); ancestry(path); return values; },
    content: (path) => {
      beforeBoundary?.("content", path);
      const source = join(run.repoRoot, projection.sourceRoot);
      if (path === source || path.startsWith(source + sep)) throw new Error("OLD SOURCE CONTENT READ");
      ancestry(dirname(path));
      const snapshot = semanticOwnedInputFileSnapshot(run.repoRoot, relative(run.repoRoot, path).split(sep).join("/"));
      if (!snapshot) throw new Error("unsafe:missing canonical content");
      const bytes = Buffer.from(snapshot.bytes), content = bytes.toString("utf8");
      if (!Buffer.from(content).equals(bytes)) throw new Error("Invalid UTF-8");
      reads.push(path);
      if (row.mutation === "read-drift" && reads.length === 1) appendFileSync(path, "\nchanged");
      return content;
    },
  };
}

/** 🧬️ Applies only the closed matrix's isolated filesystem mutations. */
function materialize(run: Run, row: ObservationCase): Readonly<{ root: string; destinationRoot: string }> {
  const missing = row.mutation === "missing-root" || row.mutation === "missing-root-with-source", files = canonicalFiles();
  const transform = (path: string): string => row.mutation === "case-alias" ? path.replace("/🔄️fsm", "/🔄️FSM") : row.mutation === "vs16-alias" ? path.replace("/🔄️fsm", "/🔄fsm") : path;
  if (!missing) for (const [path, original] of files) {
    if (row.mutation === "missing-member" && path === projection.mappings[0]!.destinationPath) continue;
    const absolute = join(run.repoRoot, transform(path));
    mkdirSync(dirname(absolute), { recursive: true });
    if (row.mutation === "file-is-directory" && path === projection.mappings[0]!.destinationPath) mkdirSync(absolute);
    else {
      const wrong = row.mutation === "wrong-macros-entry" && path === projection.mappings[0]!.destinationPath || row.mutation === "wrong-fsm-entry" && path === projection.mappings[5]!.destinationPath;
      writeFileSync(absolute, wrong ? original.replace("📚️library/🦀️.rs", "🦀️.rs") : original, { flag: "wx", mode: 0o644 });
      chmodSync(absolute, 0o644);
    }
  }
  const destination = join(run.repoRoot, projection.destinationRoot);
  if (row.mutation === "source-present" || row.mutation === "missing-root-with-source") {
    const source = join(run.repoRoot, projection.sourceRoot);
    mkdirSync(source, { recursive: true });
    writeFileSync(join(source, "🦀️.rs"), "THIS OLD SOURCE MUST NEVER BE READ\n", { flag: "wx" });
  }
  if (row.mutation === "extra-file" || row.mutation === "nfc-extra") writeFileSync(join(destination, row.mutation === "nfc-extra" ? "e\u0301vidence" : "unknown"), "unowned\n", { flag: "wx" });
  if (row.mutation === "extra-directory") mkdirSync(join(destination, "unknown"));
  if (row.mutation === "outside-sibling") writeFileSync(join(dirname(destination), "unrelated"), "preserve\n", { flag: "wx" });
  if (row.mutation === "symlink-leaf" || row.mutation === "symlink-ancestor") {
    const selected = row.mutation === "symlink-leaf" ? join(run.repoRoot, projection.mappings[0]!.destinationPath) : join(destination, "🔄️fsm");
    const foreign = join(run.root, row.mutation === "symlink-leaf" ? "foreign-file" : "foreign-directory");
    renameSync(selected, foreign);
    symlinkSync(foreign, selected, row.mutation === "symlink-leaf" ? "file" : process.platform === "win32" ? "junction" : "dir");
  }
  let root = run.repoRoot;
  if (row.mutation === "symlink-root") { root = join(run.root, "linked-root"); symlinkSync(run.repoRoot, root, process.platform === "win32" ? "junction" : "dir"); }
  if (row.mutation === "relative-root") root = relative(repoRoot, run.repoRoot);
  return { root, destinationRoot: row.mutation === "wrong-root" ? projection.destinationRoot + "/wrong" : row.mutation === "source-root-argument" ? projection.sourceRoot : projection.destinationRoot };
}

test("Draw destination observation uses closed neutral inputs and independent JSON parsing", () => {
  const validate = validateNeutral, errors: ParseError[] = [];
  expect(validate(vector), JSON.stringify(validate.errors)).toBe(true);
  expect(parse(vectorText, errors, { disallowComments: true, allowTrailingComma: false })).toEqual(vector);
  expect(parse(schemaText, errors, { disallowComments: true, allowTrailingComma: false })).toEqual(schema);
  expect(errors).toEqual([]);
  for (const invalid of [{ ...vector, extra: true }, { ...vector, cases: vector.cases.slice(1) }, { ...vector, cases: [...vector.cases.slice(1), vector.cases[1]] }, { ...vector, observation: { ...vector.observation, sourceContentReads: 1 } }]) expect(validate(invalid)).toBe(false);
  expect(new Set(vector.cases.map(({ id }) => id)).size).toBe(vector.cases.length);
  expect(hash(catalogBytes)).toBe(vector.authority.catalogSha256);
  expect(hash(authoredBytes)).toBe(vector.authority.authoredSourceSha256);
  const validateAuthored = new Ajv({ strict: true, allErrors: true }).compile(authoredSchema);
  expect(validateAuthored(authored), JSON.stringify(validateAuthored.errors)).toBe(true);
  expect(projection.contractId).toBe(vector.authority.projectionContractId);
  expect(projection.mappingDigest).toBe(vector.authority.mappingDigest);
  expect(projection.mappings).toHaveLength(vector.observation.files);
});

test("Draw destination observation preserves declared NFC case and VS16 rejection keys", () => {
  const oracle = new Intl.Collator("und", { sensitivity: "base" });
  for (const row of vector.foldVectors) {
    expect(drawDestinationKey(row.left) === drawDestinationKey(row.right)).toBe(row.equal);
    expect(oracle.compare(row.left, row.right) === 0).toBe(row.equal);
  }
});

test("Draw destination reader is strict-typed and agrees under both real compilers", () => {
  const source = readFileSync(import.meta.filename, "utf8"), syntax = ts.createSourceFile(import.meta.filename, source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const typeNames = ["Projection", "NodeFact", "DestinationIO", "DestinationInput"], functionNames = ["drawDestinationKey", "readDrawDestination"];
  const selectedTypes = typeNames.map(name => { const nodes = syntax.statements.filter(node => ts.isTypeAliasDeclaration(node) && node.name.text === name); expect(nodes, name).toHaveLength(1); return nodes[0]!.getText(syntax); }).join("\n");
  const selectedFunctions = functionNames.map(name => { const nodes = syntax.statements.filter(node => ts.isFunctionDeclaration(node) && node.name?.text === name); expect(nodes, name).toHaveLength(1); return nodes[0]!.getText(syntax); }).join("\n");
  const declarations = "type SemanticProjectionAuthorityNode = Readonly<{ path: string; nodeKind: 'file' | 'directory' | 'symlink'; content?: string }>;\ndeclare const projection: Projection;\ndeclare function byteOrder(left: string, right: string): number;\ndeclare function dirname(path: string): string;\ndeclare function isAbsolute(path: string): boolean;\ndeclare function resolve(...paths: string[]): string;\ndeclare function join(...paths: string[]): string;\n";
  const code = selectedTypes + "\n" + declarations + selectedFunctions, filename = resolve(import.meta.dir, "draw-destination-strict-proof.ts");
  const options: ts.CompilerOptions = { target: ts.ScriptTarget.ES2022, strict: true, noEmit: true, types: [], skipLibCheck: true };
  const host = ts.createCompilerHost(options), originalRead = host.readFile.bind(host), originalExists = host.fileExists.bind(host), originalSource = host.getSourceFile.bind(host);
  host.readFile = path => path === filename ? code : originalRead(path);
  host.fileExists = path => path === filename || originalExists(path);
  host.getSourceFile = (path, language, onError, fresh) => path === filename ? ts.createSourceFile(path, code, language, true, ts.ScriptKind.TS) : originalSource(path, language, onError, fresh);
  const program = ts.createProgram([filename], options, host);
  expect(ts.getPreEmitDiagnostics(program).map(diagnostic => ts.flattenDiagnosticMessageText(diagnostic.messageText, "\n"))).toEqual([]);
  const implementations = [readDrawDestination, ...[new Bun.Transpiler({ loader: "ts" }).transformSync(selectedFunctions), ts.transpileModule(selectedFunctions, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText].map(javascript => new Function("projection", "byteOrder", "dirname", "isAbsolute", "resolve", "join", javascript + "\nreturn readDrawDestination;")(projection, byteOrder, dirname, isAbsolute, resolve, join) as typeof readDrawDestination)];
  const row = vector.cases.find(row => row.id === "canonical-complete")!, run = makeRun("canonical-compiler-parity");
  let passed = false;
  try {
    materialize(run, row);
    let expected: readonly SemanticProjectionAuthorityNode[] | undefined;
    for (const implementation of implementations) {
      const reads: string[] = [], calls: string[] = [], nodes = implementation({ repoRoot: run.repoRoot, destinationRoot: projection.destinationRoot, io: physicalIO(run, row, reads, calls) });
      expected ??= nodes;
      expect(nodes).toEqual(expected);
      expect(reads).toHaveLength(vector.observation.files);
      expect(nodes.map(node => node.path)).toEqual(nodes.map(node => node.path).sort(byteOrder));
      expect(() => implementation({ repoRoot: run.repoRoot, destinationRoot: projection.sourceRoot, io: { fact: () => { throw new Error("unexpected IO"); }, names: () => { throw new Error("unexpected IO"); }, content: () => { throw new Error("unexpected IO"); } } })).toThrow("root:");
    }
    passed = true;
  } finally { appendFileSync(run.report, "\nFinished: " + new Date().toISOString() + "\n\nOutcome: " + (passed ? "passed" : "failed") + "\n\nCompiler implementations: 3; TypeScript version: " + ts.version + "\n"); }
});

for (const row of vector.cases) test("Draw destination observation: " + row.id, () => {
  const run = makeRun(row.id), reads: string[] = [], calls: string[] = [];
  let passed = false;
  try {
    const input = materialize(run, row), io = physicalIO(run, row, reads, calls);
    let nodes: readonly SemanticProjectionAuthorityNode[] | undefined, error: string | null = null;
    try {
      nodes = readDrawDestination({ repoRoot: input.root, destinationRoot: input.destinationRoot, io });
      const authority = semanticPathProjectionAuthority({ artifactRoot: projection.sourceRoot.slice(0, projection.sourceRoot.indexOf("/🏅️standards/")), contractId: projection.contractId, sourceRoot: projection.sourceRoot, layout: "destination", nodes }, taxonomy);
      if (authority.problems.length) throw new Error("authority:" + authority.problems.join(";"));
      expect(authority.mappings).toEqual(projection.mappings);
      expect(authority.mappingDigest).toBe(projection.mappingDigest);
      expect(authority.referenceEdits).toEqual([]);
    } catch (caught) { error = (caught as Error).message.split(":")[0]!; }
    expect(error, row.id).toBe(row.error);
    expect(reads).toHaveLength(row.contentReads);
    expect(reads.every(path => projection.mappings.some(({ destinationPath }) => path === join(run.repoRoot, destinationPath)))).toBe(true);
    if (row.error === "root") expect(calls).toEqual([]);
    if (row.expected === "accepted") {
      expect(nodes).toHaveLength(vector.observation.nodes);
      expect(Math.max(...nodes!.map(node => Buffer.byteLength(node.path, "utf8")))).toBe(vector.observation.maxPathBytes);
      expect(nodes!.filter(({ nodeKind }) => nodeKind === "directory")).toHaveLength(vector.observation.directories);
      const observedFiles = nodes!.filter(({ nodeKind }) => nodeKind === "file").map(({ path }) => path).sort(byteOrder);
      const oracleFiles = fg.sync("**/*", { cwd: join(run.repoRoot, projection.destinationRoot), dot: true, onlyFiles: true, followSymbolicLinks: false }).map(path => projection.destinationRoot + "/" + path).sort(byteOrder);
      expect(observedFiles).toEqual(oracleFiles);
      const oracleDirectories = [projection.destinationRoot, ...fg.sync("**/*", { cwd: join(run.repoRoot, projection.destinationRoot), dot: true, onlyDirectories: true, followSymbolicLinks: false }).map(path => projection.destinationRoot + "/" + path)].sort(byteOrder);
      expect(nodes!.filter(({ nodeKind }) => nodeKind === "directory").map(({ path }) => path).sort(byteOrder)).toEqual(oracleDirectories);
      for (const configuration of authored.oracle.configuration) {
        const mapping = projection.mappings.find(({ sourcePath }) => sourcePath === projection.sourceRoot + "/" + configuration.path)!;
        const actual = nodes!.find(({ path }) => path === mapping.destinationPath)!;
        expect((toml.parse(actual.content!) as { lib: { path: string } }).lib.path).toBe(configuration.destinationEntry);
      }
      if (row.mutation === "outside-sibling") expect(readFileSync(join(run.repoRoot, dirname(projection.destinationRoot), "unrelated"), "utf8")).toBe("preserve\n");
    }
    passed = true;
  } finally { appendFileSync(run.report, "\nFinished: " + new Date().toISOString() + "\n\nOutcome: " + (passed ? "passed" : "failed") + "\n\nContent read count: " + reads.length + "\n"); }
});

for (const boundary of vector.ancestorSwap.boundaries) test("Draw destination ancestor swap: " + boundary, () => {
  const row = vector.cases.find(row => row.id === "canonical-complete")!, run = makeRun("ancestor-swap-" + boundary), reads: string[] = [], calls: string[] = [];
  const selected = join(run.repoRoot, projection.destinationRoot), firstFile = join(run.repoRoot, [...projection.mappings].map(row => row.destinationPath).sort(byteOrder)[0]!);
  const original = { lstatSync: fs.lstatSync, readdirSync: fs.readdirSync, openSync: fs.openSync, readFileSync: fs.readFileSync };
  const observations: { operation: string; path: string; afterSwap: boolean }[] = [], descriptors = new Map<number, string>();
  let swapped = false, armed = false, firstFileFacts = 0, completeMembership = false, passed = false, error: string | null = null;
  materialize(run, row);
  const trace = (operation: string, value: fs.PathLike | number): void => {
    const path = typeof value === "number" ? descriptors.get(value) : typeof value === "string" ? value : undefined;
    if (path && (path.startsWith(selected + sep) || operation === "readdir" && path === selected)) observations.push({ operation, path, afterSwap: swapped });
  };
  const spies = [
    spyOn(fs, "lstatSync").mockImplementation(((path: fs.PathLike, ...options: unknown[]) => { trace("lstat", path); return Reflect.apply(original.lstatSync, fs, [path, ...options]); }) as typeof fs.lstatSync),
    spyOn(fs, "readdirSync").mockImplementation(((path: fs.PathLike, ...options: unknown[]) => { trace("readdir", path); return Reflect.apply(original.readdirSync, fs, [path, ...options]); }) as typeof fs.readdirSync),
    spyOn(fs, "openSync").mockImplementation(((path: fs.PathLike, ...options: unknown[]) => { trace("open", path); const descriptor = Reflect.apply(original.openSync, fs, [path, ...options]) as number; if (typeof path === "string") descriptors.set(descriptor, path); return descriptor; }) as typeof fs.openSync),
    spyOn(fs, "readFileSync").mockImplementation(((path: fs.PathLike | number, ...options: unknown[]) => { trace("read", path); return Reflect.apply(original.readFileSync, fs, [path, ...options]); }) as typeof fs.readFileSync),
  ];
  const io = physicalIO(run, row, reads, calls, (operation, path) => {
    if (!armed || swapped) return;
    const trigger = boundary === "reader-before-content" ? operation === "fact" && path === firstFile && ++firstFileFacts === 2 : operation === boundary;
    if (!trigger) return;
    completeMembership = projection.mappings.every(mapping => calls.includes("fact:" + join(run.repoRoot, mapping.destinationPath)));
    const retained = join(run.root, "📦️substituted-directory");
    renameSync(selected, retained);
    symlinkSync(retained, selected, process.platform === "win32" ? "junction" : "dir");
    swapped = true;
  });
  try {
    expect(io.fact(firstFile)?.kind).toBe("file");
    expect(observations.some(row => row.operation === "lstat" && row.path === firstFile)).toBe(true);
    observations.length = 0;
    calls.length = 0;
    armed = true;
    try {
      if (boundary === "reader-before-content") readDrawDestination({ repoRoot: run.repoRoot, destinationRoot: projection.destinationRoot, io });
      else if (boundary === "fact") io.fact(firstFile);
      else if (boundary === "names") io.names(dirname(firstFile));
      else io.content(firstFile);
    } catch (caught) { error = (caught as Error).message.split(":")[0]!; }
    expect(swapped).toBe(true);
    if (boundary === "reader-before-content") expect(completeMembership).toBe(true);
    expect(observations.filter(row => row.afterSwap), JSON.stringify(observations.filter(row => row.afterSwap))).toHaveLength(vector.ancestorSwap.descendantObservations);
    expect(observations.filter(row => row.afterSwap && row.operation === "read")).toHaveLength(vector.ancestorSwap.contentReads);
    expect(reads).toHaveLength(vector.ancestorSwap.contentReads);
    expect(error).toBe(vector.ancestorSwap.error);
    passed = true;
  } finally {
    for (const spy of spies.reverse()) spy.mockRestore();
    appendFileSync(run.report, "\nFinished: " + new Date().toISOString() + "\n\nOutcome: " + (passed ? "passed" : "failed") + "\n\nSwapped: " + swapped + "\n\nComplete membership before swap: " + completeMembership + "\n\nReturned error: " + error + "\n\nDescendant observations after swap: " + observations.filter(row => row.afterSwap).length + "\n\nContent reads after swap: " + observations.filter(row => row.afterSwap && row.operation === "read").length + "\n\n```json\n" + JSON.stringify(observations.filter(row => row.afterSwap), null, 2) + "\n```\n");
  }
});

test("Draw destination observation has the closed default-budget canonical registration", async () => {
  const directory = join(import.meta.dir, "../🧪️🛟️draw-destination-observation/🧪️registration"), bytes = readFileSync(join(directory, "../🧪️🛟️draw-destination-observation/🔣️.json"), "utf8"), registration = JSON.parse(bytes);
  const validate = new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(directory, "../🧪️🛟️draw-destination-observation/🧬️schema/🔣️.json"), "utf8")));
  expect(validate(registration), JSON.stringify(validate.errors)).toBe(true);
  for (const invalid of [{ ...registration, source: "../🧪️🛟️draw-destination-observation/🟦️.ts" }, { ...registration, budget: 120000 }, { ...registration, budgetMs: 120000 }, { ...registration, filter: "selected" }, { ...registration, runner: "other" }, { ...registration, launchOrder: 410.210 }]) expect(validate(invalid)).toBe(false);
  const errors: ParseError[] = [];
  expect(parse(bytes, errors, { disallowComments: true, allowTrailingComma: false })).toEqual(registration);
  expect(errors).toEqual([]);
  const packageRelative = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript", packageRoot = join(repoRoot, packageRelative);
  const paths = [join(packageRoot, "📋️project.json"), join(packageRoot, "package.json"), join(packageRoot, "📜️script.ts"), join(repoRoot, ".vscode/🧩️launch.seed.jsonc"), join(repoRoot, ".vscode/launch.json")];
  const capture = () => paths.map(path => { const bytes = readFileSync(path); return { path, hash: hash(bytes), size: bytes.length }; }), before = capture();
  try {
    expect(join(repoRoot, registration.source)).toBe(import.meta.filename);
    const project = JSON.parse(readFileSync(paths[0]!, "utf8"));
    expect(project.targets[registration.target]).toBeDefined();
    expect(project.targets[registration.target]).toEqual({ executor: "nx:run-commands", options: { cwd: packageRelative, command: "bun ./📜️script.ts test " + registration.command } });
    expect(JSON.parse(readFileSync(paths[1]!, "utf8")).scripts[registration.target]).toBe("nx run @semio-tech/repo-lib:" + registration.target);
    const syntax = ts.createSourceFile(paths[2]!, readFileSync(paths[2]!, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    const declarations = syntax.statements.filter(node => ts.isClassDeclaration(node) && node.name?.text === "TestScript");
    expect(declarations).toHaveLength(1);
    const source = declarations[0]!.getText(syntax) + "\nreturn new TestScript();";
    for (const javascript of [new Bun.Transpiler({ loader: "ts" }).transformSync(source), ts.transpileModule(source, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText]) {
      const invocations: { executable: string; args: string[]; options: { cwd: string } }[] = [];
      class FixtureBundle { root = packageRoot; repoRoot = repoRoot; }
      const router = new Function("BundleScript", "join", "runTestBudgeted", "resolveTestLevel", javascript)(FixtureBundle, join, async (executable: string, args: string[], options: { cwd: string }) => { invocations.push({ executable, args, options }); }, () => { throw new Error("Draw observation fell through to generic routing"); });
      await router.run([registration.command]);
      expect(invocations).toEqual([{ executable: process.execPath, args: ["test", import.meta.filename], options: { cwd: repoRoot } }]);
    }
    for (const path of paths.slice(3)) {
      const errors: ParseError[] = [], document = parse(readFileSync(path, "utf8"), errors);
      expect(errors).toEqual([]);
      expect(document.configurations.filter((row: { name: string }) => row.name === registration.launchName)).toEqual([{ name: registration.launchName, type: "node-terminal", request: "launch", command: "bun nx run @semio-tech/repo-lib:" + registration.target + " --skip-nx-cache", cwd: "${workspaceFolder}", presentation: { group: registration.launchGroup, order: registration.launchOrder } }]);
      expect(document.configurations.filter((row: { presentation?: { group: string; order: number } }) => row.presentation?.group === registration.launchGroup && row.presentation?.order === registration.launchOrder)).toHaveLength(1);
    }
  } finally {
    const after = capture();
    expect(after).toEqual(before);
    console.log("[DEBUG] Draw destination registration input closure " + JSON.stringify({ pid: process.pid, inputs: after }));
  }
});

afterAll(() => {
  const afterInputs = identities();
  expect(afterInputs).toEqual(beforeInputs);
  console.log("[DEBUG] Draw destination observation input closure " + JSON.stringify({ pid: process.pid, inputs: afterInputs }));
});
